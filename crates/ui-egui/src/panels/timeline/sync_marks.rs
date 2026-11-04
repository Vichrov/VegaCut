//! A/V source alignment, including live move/trim previews. No edit or project mutation.
use super::{ClipId, Color32, FilmcraftApp, HashMap, ItemId, Rect, Sequence, Stroke, StrokeKind, Tick, Tokens, TrackId, TrackItem, TrackKind, pos2, vec2};

struct Event<'a> {
    item: &'a TrackItem,
    kind: TrackKind,
    start: Tick,
    duration: Tick,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Offset {
    Frames(f64),
    Speed,
}

/// Match the opposite media type from the same source. Groups/links constrain the search;
/// ungrouped events need an overlap. Prefer the most overlap, then nearest, then stable id.
pub(super) fn offsets(
    seq: &Sequence,
    previews: &HashMap<ClipId, (Tick, Tick, Option<TrackId>)>,
    edited: &HashMap<ClipId, TrackItem>,
    unlinked: bool,
) -> HashMap<ClipId, Offset> {
    let mut sources: HashMap<ItemId, Vec<Event<'_>>> = HashMap::new();
    for tr in seq.all_tracks() {
        for original in &tr.items {
            let item = edited.get(&original.id).unwrap_or(original);
            let (start, duration, _) = previews.get(&item.id).copied().unwrap_or((item.start, item.duration, None));
            if duration > Tick::ZERO {
                sources.entry(item.item).or_default().push(Event { item, kind: tr.kind, start, duration });
            }
        }
    }
    let mut out = HashMap::new();
    for events in sources.values() {
        for a in events {
            let grouped = a.item.link.is_some() || a.item.group.is_some();
            if !grouped && !unlinked {
                continue;
            }
            let partner = events
                .iter()
                .filter(|b| {
                    if a.kind == b.kind {
                        return false;
                    }
                    let related = (a.item.link.is_some() && a.item.link == b.item.link) || (a.item.group.is_some() && a.item.group == b.item.group);
                    (!grouped || related) && (related || overlap(a, b) > Tick::ZERO)
                })
                .max_by_key(|b| (overlap(a, b), std::cmp::Reverse((a.start - b.start).0.unsigned_abs()), std::cmp::Reverse(b.item.id.0)));
            let Some(b) = partner else { continue };
            // Holds and animated retiming have no single constant offset to report.
            let complex = |i: &TrackItem| {
                i.frame_hold.is_some()
                    || i.effect("time_remap").is_some_and(|e| {
                        e.enabled
                            && e.params
                                .get("speed")
                                .is_some_and(|p| p.is_animated() || !matches!(p.value, filmcraft_project::ParamValue::Float(v) if v == 100.0))
                    })
            };
            if complex(a.item) || complex(b.item) || !a.item.speed.is_finite() || !b.item.speed.is_finite() || a.item.speed == 0.0 || b.item.speed == 0.0 {
                continue;
            }
            if a.item.reverse != b.item.reverse || (a.item.speed.abs() - b.item.speed.abs()).abs() > 1e-9 {
                out.insert(a.item.id, Offset::Speed);
                continue;
            }
            let at = a.start.max(b.start);
            let source = |e: &Event<'_>| e.item.source_time_at(at - e.start + e.item.start);
            let delta = source(b) - source(a);
            if delta != Tick::ZERO {
                let direction = if a.item.reverse { -1.0 } else { 1.0 };
                let frames = delta.0 as f64 / a.item.speed.abs() / seq.settings.frame_rate.frame_duration().0.max(1) as f64 * direction;
                if frames.is_finite() {
                    out.insert(a.item.id, Offset::Frames(frames));
                }
            }
        }
    }
    out
}

fn overlap(a: &Event<'_>, b: &Event<'_>) -> Tick {
    ((a.start + a.duration).min(b.start + b.duration) - a.start.max(b.start)).max(Tick::ZERO)
}

pub(super) fn paint(app: &mut FilmcraftApp, painter: &egui::Painter, body: Rect, clip: ClipId, offset: Option<&Offset>) {
    let Some(offset) = offset else { return };
    let visible = body.intersect(painter.clip_rect());
    if visible.width() < 28.0 || visible.height() < 18.0 {
        return;
    }
    let (text, label) = match offset {
        Offset::Frames(f) => {
            let number = if f.abs() < 0.005 { format!("{f:+.4}") } else { format!("{f:+.2}") };
            (format!("{number} f"), format!("{}: {number} f", crate::i18n::t("A/V offset")))
        }
        Offset::Speed => ("Δ speed".to_string(), crate::i18n::t("A/V speed mismatch").to_string()),
    };
    let font = Tokens::ui(11.0);
    let galley = painter.layout_no_wrap(text, font, Color32::WHITE);
    let size = vec2((galley.size().x + 8.0).min(visible.width()), 16.0);
    let badge = Rect::from_min_size(pos2(visible.right() - size.x, visible.bottom() - size.y), size);
    let p = painter.with_clip_rect(visible);
    p.rect_stroke(body, 0.0, Stroke::new(1.5, Color32::from_rgb(240, 92, 88)), StrokeKind::Inside);
    p.rect_filled(badge, 2.0, Color32::from_rgb(153, 38, 38));
    p.with_clip_rect(badge).galley(badge.min + vec2(4.0, 1.0), galley, Color32::WHITE);
    app.auto.add(&format!("timeline.clip.{}.syncOffset", clip.0), badge, &label);
}

#[cfg(test)]
mod tests {
    use super::{ClipId, HashMap, Offset, Sequence, offsets};
    use filmcraft_engine::Session;
    use serde_json::json;

    fn pair() -> (Sequence, ClipId, ClipId) {
        let mut s = Session::default();
        s.execute("file.openDemoProject", json!({})).unwrap();
        s.execute("file.newSequence", json!({"name": "Sync", "fps": 24})).unwrap();
        let media = s.project.items.values().find(|i| i.has_video() && i.has_audio() && i.as_media().is_some()).unwrap().id;
        s.execute("timeline.place", json!({"item": media.0, "time": 0, "duration": s.sequence_rate().tick_of(96).0})).unwrap();
        let q = s.active_sequence().unwrap().clone();
        let (v, a) = (q.video_tracks[0].items[0].id, q.audio_tracks[0].items[0].id);
        (q, v, a)
    }

    #[test]
    fn move_slip_trim_and_live_preview_alignment() {
        let (mut q, v, a) = pair();
        let empty = HashMap::new();
        let frame = q.settings.frame_rate.tick_of(1);
        assert!(offsets(&q, &empty, &HashMap::new(), true).is_empty());
        q.find_item_mut(a).unwrap().1.start = q.settings.frame_rate.tick_of(12);
        let marks = offsets(&q, &empty, &HashMap::new(), true);
        assert!(matches!(marks.get(&a), Some(Offset::Frames(f)) if (*f - 12.0).abs() < 1e-6));
        assert!(matches!(marks.get(&v), Some(Offset::Frames(f)) if (*f + 12.0).abs() < 1e-6));
        // A trim advances source In by the same amount: no out-of-sync marker.
        q.find_item_mut(a).unwrap().1.source_in = q.settings.frame_rate.tick_of(12);
        assert!(offsets(&q, &empty, &HashMap::new(), true).is_empty());
        let mut edit = q.find_item(a).unwrap().1.clone();
        edit.source_in += frame;
        let edited = HashMap::from([(a, edit)]);
        assert!(matches!(offsets(&q, &empty, &edited, true).get(&a), Some(Offset::Frames(f)) if (*f + 1.0).abs() < 1e-6));
        let it = q.find_item(a).unwrap().1;
        let preview = HashMap::from([(a, (it.start + frame, it.duration, None))]);
        assert!(matches!(offsets(&q, &preview, &HashMap::new(), true).get(&a), Some(Offset::Frames(f)) if (*f - 1.0).abs() < 1e-6));
    }

    #[test]
    fn detached_pair_overlap_and_rate_limits() {
        let (mut q, v, a) = pair();
        for id in [v, a] {
            q.find_item_mut(id).unwrap().1.link = None;
        }
        q.find_item_mut(a).unwrap().1.start = q.settings.frame_rate.tick_of(1);
        assert_eq!(offsets(&q, &HashMap::new(), &HashMap::new(), true).len(), 2);
        assert!(offsets(&q, &HashMap::new(), &HashMap::new(), false).is_empty());
        q.find_item_mut(a).unwrap().1.speed = 2.0;
        assert!(matches!(offsets(&q, &HashMap::new(), &HashMap::new(), true).get(&a), Some(Offset::Speed)));
        q.find_item_mut(a).unwrap().1.speed = f64::NAN;
        assert!(offsets(&q, &HashMap::new(), &HashMap::new(), true).is_empty());
        q.find_item_mut(a).unwrap().1.speed = 1.0;
        q.find_item_mut(a).unwrap().1.start = q.settings.frame_rate.tick_of(200);
        assert!(offsets(&q, &HashMap::new(), &HashMap::new(), true).is_empty(), "unrelated time ranges do not imply sync partners");
    }
}
