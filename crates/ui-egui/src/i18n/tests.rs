use super::*;

fn catalog_text(l: Language) -> Option<&'static str> {
    match l {
        Language::En => None,
        Language::Ru => Some(include_str!("ru.tsv")),
    }
}

#[test]
fn catalogs_are_well_formed() {
    for l in Language::ALL {
        let Some(text) = catalog_text(l) else { continue };
        let code = l.code();
        let (entries, errors) = catalog::parse_entries(text);
        assert!(errors.is_empty(), "{code}: {errors:?}");
        for (i, (ctx, en, tr)) in entries.iter().enumerate() {
            assert!(entries.iter().take(i).all(|(c, e, _)| (c, e) != (ctx, en)), "{code}: duplicate entry {ctx:?} {en:?}");
            let (mut a, mut b) = (catalog::placeholders(en), catalog::placeholders(tr));
            a.sort_unstable();
            b.sort_unstable();
            assert_eq!(a, b, "{code}: placeholders differ in {en:?} → {tr:?}");
            assert_eq!(en.ends_with('…'), tr.ends_with('…'), "{code}: ellipsis differs in {en:?} → {tr:?}");
        }
    }
}

#[test]
fn the_current_language_is_per_thread() {
    // any entry the Russian catalog has
    let (entries, _) = catalog::parse_entries(include_str!("ru.tsv"));
    let Some((_, en, es)) = entries.into_iter().find(|(ctx, en, es)| ctx.is_empty() && en != es) else { return };
    set_current(Language::Ru);
    assert_eq!((current(), t(&en)), (Language::Ru, es.as_str()));
    // another thread (another test) still draws in English
    let source = en.clone();
    assert_eq!(std::thread::spawn(move || t(&source).to_string()).join().ok(), Some(en.clone()));
    set_current(Language::En);
    assert_eq!(t(&en), en);
}

#[test]
fn template_values_are_not_interpreted_as_placeholders() {
    assert_eq!(fmt("{name}: {n}", &[("name", "Vídeo {n}.mp4"), ("n", "3")]), "Vídeo {n}.mp4: 3");
    assert_eq!(fmt("{n} {name} {n}", &[("name", "{n}"), ("n", "日本語")]), "日本語 {n} 日本語");
    assert_eq!(fmt("{missing} y {unfinished", &[]), "{missing} y {unfinished");
}

#[test]
fn searches_match_translated_labels_and_english_names() {
    for l in Language::ALL {
        set_current(l);
        let blur = l.tr("Gaussian Blur").to_lowercase();
        let selection = l.tr("Selection Tool").to_lowercase();
        assert!(matches_query("Gaussian Blur", &blur), "{}: {blur}", l.code());
        assert!(matches_query("Gaussian Blur", "gaussian"), "{}", l.code());
        assert!(matches_query("Selection Tool", &selection), "{}: {selection}", l.code());
        assert!(!matches_query("Gaussian Blur", &selection), "{}: {selection}", l.code());
    }
    set_current(Language::En);
}

#[test]
fn tl_literals_are_plain_strings() {
    for (path, text) in sources() {
        assert!(!text.contains("tl!(r") && !text.contains("tlf!(r"), "{path}: use a plain string literal in tl! and tlf!");
    }
}

fn interface_strings() -> Vec<String> {
    let app = crate::FilmcraftApp::new(filmcraft_engine::Session::default());
    let mut out = Vec::new();
    for top in crate::menus::MENUS.into_iter().chain(["Audio"]).chain(crate::menus::NATIVE_APP_MENU) {
        push(&mut out, top);
    }
    for it in crate::menus::menu_items(&app) {
        push(&mut out, &it.label);
        it.path.iter().for_each(|p| push(&mut out, p));
    }
    for (_, text) in sources() {
        tl_literals(&text).iter().for_each(|lit| push(&mut out, lit));
    }
    dynamic_names().iter().for_each(|n| push(&mut out, n));
    out
}

#[test]
fn missing_translations_report() {
    let strings = interface_strings();
    println!("Missing translations ({} interface strings; each shows in English):", strings.len());
    for l in Language::ALL {
        let Some(text) = catalog_text(l) else { continue };
        let code = l.code();
        let mut missing: Vec<&str> = strings.iter().map(String::as_str).filter(|s| !l.has(s)).collect();
        missing.sort_unstable();
        println!("{code}: {} missing", missing.len());
        for s in missing {
            println!("  {code}: {s:?}");
        }
        let (entries, _) = catalog::parse_entries(text);
        let unused: Vec<_> = entries.iter().filter(|(_, en, _)| translatable(en) && !strings.contains(en)).collect();
        if !unused.is_empty() {
            println!("{code}: {} entries without a source", unused.len());
            for (ctx, en, _) in unused {
                println!("  {code}: {ctx:?} {en:?}");
            }
        }
    }
}

fn sources() -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut dirs = vec![std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"))];
    while let Some(d) = dirs.pop() {
        for entry in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = entry.path();
            if p.is_dir() {
                dirs.push(p);
            } else if p.extension().is_some_and(|e| e == "rs") && p.file_name().is_some_and(|n| n != "tests.rs") {
                // a Windows checkout with core.autocrlf has CRLF, which the `\n` below would not match
                let text = std::fs::read_to_string(&p).unwrap_or_default().replace("\r\n", "\n");
                let cut = text.find("#[cfg(test)]\nmod tests").unwrap_or(text.len());
                out.push((p.display().to_string(), text[..cut].to_string()));
            }
        }
    }
    out
}

fn tl_literals(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    loop {
        let next = ["tl!(\"", "tlf!(\""].into_iter().filter_map(|p| Some((rest.find(p)?, p.len()))).min();
        let Some((at, len)) = next else { break };
        rest = &rest[at + len..];
        let mut lit = String::new();
        let mut chars = rest.char_indices();
        let mut end = rest.len();
        while let Some((i, c)) = chars.next() {
            match c {
                '"' => {
                    end = i + 1;
                    break;
                }
                '\\' => match chars.next().map(|(_, e)| e) {
                    Some('n') => lit.push('\n'),
                    Some('t') => lit.push('\t'),
                    Some('u') => {
                        // \u{201c}
                        let hex: String = chars.by_ref().map(|(_, h)| h).take_while(|h| *h != '}').filter(|h| *h != '{').collect();
                        lit.extend(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32));
                    }
                    Some(e) => lit.push(e),
                    None => {}
                },
                c => lit.push(c),
            }
        }
        rest = &rest[end..];
        out.push(lit);
    }
    out
}

fn translatable(s: &str) -> bool {
    const UNITS: [&str; 14] = ["Hz", "kHz", "dB", "oct", "fps", "IRE", "mm", "px", "ms", "s", "x", "Bit", "HLG", "PQ"];
    !Language::ALL.iter().any(|l| l.name() == s) && s.split(|c: char| !c.is_alphabetic()).any(|w| !w.is_empty() && !UNITS.contains(&w))
}

fn push(out: &mut Vec<String>, s: &str) {
    if translatable(s) && !out.iter().any(|o| o == s) {
        out.push(s.to_string());
    }
}

fn settings_rows(list: &[filmcraft_engine::settings::Row], out: &mut Vec<String>) {
    use filmcraft_engine::settings::{Kind, Row};
    for r in list {
        match r {
            Row::Field(f) => {
                push(out, f.label);
                match f.kind {
                    Kind::Int { unit, .. } | Kind::Float { unit, .. } => push(out, unit),
                    Kind::Choice(opts) => opts.iter().for_each(|(_, l)| push(out, l)),
                    _ => {}
                }
            }
            Row::Group(title, inner) => {
                push(out, title);
                settings_rows(inner, out);
            }
            Row::Note(text) => push(out, text),
            Row::Button { label, .. } => push(out, label),
            Row::Custom(_) => {}
        }
    }
}

fn dynamic_names() -> Vec<String> {
    let mut out = Vec::new();
    for c in filmcraft_engine::settings::categories() {
        push(&mut out, c.title);
        settings_rows(c.rows, &mut out);
    }
    filmcraft_engine::settings::DURATION_UNITS.iter().for_each(|(_, l)| push(&mut out, l));
    for top in filmcraft_project::vtransition::EFFECT_TOP_FOLDERS {
        push(&mut out, top);
    }
    for d in filmcraft_project::effect_defs() {
        push(&mut out, d.name);
        d.category.iter().for_each(|c| push(&mut out, c));
        for p in &d.params {
            push(&mut out, p.label);
            p.group.iter().for_each(|g| push(&mut out, g));
            if let filmcraft_project::ParamKind::Choice(opts) = p.kind {
                opts.iter().for_each(|o| push(&mut out, o));
            }
        }
    }
    crate::panels::timeline::CLIP_MENU.iter().flat_map(|g| g.iter()).for_each(|(l, _)| push(&mut out, l));
    crate::panels::timeline::EDIT_POINT_TYPES.iter().for_each(|(l, ..)| push(&mut out, l));
    crate::panels::project::NEW_ITEMS.iter().for_each(|(l, _)| push(&mut out, l));
    // section headers keyed by their English name (collapsed state), translated when drawn
    let sections = [
        // Lumetri Color
        "Basic Correction",
        "Creative",
        "Curves",
        "Color Wheels & Match",
        "HSL Secondary",
        "Vignette",
        // Properties
        "Transform",
        "Crop",
        // Essential Graphics
        "Layers",
        "Template Properties",
        "Responsive Design - Time",
        "Responsive Design - Position",
        "Align and Transform",
        "Text",
        "Shape",
        "Appearance",
    ];
    sections.iter().for_each(|s| push(&mut out, s));
    for p in filmcraft_render::lumetri_presets::presets() {
        push(&mut out, p.name);
        push(&mut out, p.folder);
        push(&mut out, p.description);
    }
    crate::dock::PanelKind::ALL.iter().for_each(|p| push(&mut out, p.title()));
    crate::dock::WORKSPACES.iter().for_each(|w| push(&mut out, w));
    filmcraft_project::Label::ALL.iter().for_each(|l| push(&mut out, l.name()));
    crate::panels::panel_state::TcMode::ALL.iter().for_each(|m| push(&mut out, m.label()));
    crate::panels::panel_state::TcSource::ALL.iter().for_each(|s| push(&mut out, s.label()));
    crate::panels::panel_state::SCOPE_PRESETS.iter().for_each(|p| push(&mut out, p.0));
    filmcraft_time::TimeDisplay::ALL.iter().for_each(|d| push(&mut out, d.label()));
    use filmcraft_engine::panels::Level;
    [Level::Info, Level::Warning, Level::Error].iter().for_each(|l| push(&mut out, l.label()));
    use filmcraft_scopes as sc;
    sc::ScopeKind::ALL.iter().for_each(|k| push(&mut out, k.label()));
    sc::WaveformType::ALL.iter().for_each(|k| push(&mut out, k.label()));
    sc::ParadeType::ALL.iter().for_each(|k| push(&mut out, k.label()));
    sc::ColorSpace::ALL.iter().for_each(|k| push(&mut out, k.label()));
    sc::Scale::ALL.iter().for_each(|k| push(&mut out, k.label()));
    sc::Brightness::ALL.iter().for_each(|k| push(&mut out, k.label()));
    [sc::Targets::Percent75, sc::Targets::Percent100].iter().for_each(|k| push(&mut out, k.label()));
    crate::state::PlaybackRes::ALL.iter().for_each(|r| push(&mut out, r.label()));
    crate::state::Tool::ALL.iter().for_each(|t| push(&mut out, t.label()));
    crate::credits::NameMode::ALL.iter().for_each(|m| push(&mut out, m.label()));
    crate::credits::SortKey::ALL.iter().for_each(|k| push(&mut out, k.label().0));
    filmcraft_project::TimeInterpolation::ALL.iter().for_each(|m| push(&mut out, m.label()));
    filmcraft_project::CaptionFormat::ALL.iter().for_each(|f| push(&mut out, f.label()));
    filmcraft_project::MaskMode::ALL.iter().for_each(|m| push(&mut out, m.label()));
    use filmcraft_project::essential as es;
    for kind in es::AudioType::ALL {
        push(&mut out, kind.label());
        kind.sections().iter().for_each(|s| push(&mut out, s.label()));
    }
    es::EQ_PRESETS.iter().for_each(|p| push(&mut out, p.name));
    es::REVERB_PRESETS.iter().for_each(|p| push(&mut out, p.name));
    filmcraft_engine::essential_sound::presets(&filmcraft_engine::Session::default()).iter().for_each(|p| push(&mut out, &p.name));
    filmcraft_project::TrackMethod::ALL.iter().for_each(|m| push(&mut out, m.label()));
    filmcraft_project::AutomationMode::ALL.iter().for_each(|m| push(&mut out, m.label()));
    filmcraft_project::graphic_design::RollMode::ALL.iter().for_each(|m| push(&mut out, m.label()));
    filmcraft_project::InputMap::ALL.iter().for_each(|m| push(&mut out, m.label()));
    filmcraft_engine::panels::LOG_FIELDS.iter().for_each(|f| push(&mut out, f));
    filmcraft_engine::project_panel::BUILTIN_COLUMNS.iter().for_each(|c| push(&mut out, c));
    filmcraft_project::find::COLUMNS.iter().for_each(|c| push(&mut out, c));
    filmcraft_project::FindOp::ALL.iter().for_each(|o| push(&mut out, o.label()));
    filmcraft_engine::media_browser::ALL_COLUMNS.iter().for_each(|c| push(&mut out, c));
    // export: presets, settings choices, ranges and pixel aspects
    use filmcraft_engine::export as ex;
    for p in ex::builtin_presets() {
        push(&mut out, &p.name);
        push(&mut out, &p.category);
        push(&mut out, &p.description);
    }
    ex::Format::ALL.iter().for_each(|f| push(&mut out, f.label()));
    ex::MxfVideoCodec::ALL.iter().for_each(|c| push(&mut out, c.label()));
    ex::Placement::ALL.iter().for_each(|p| push(&mut out, p.label()));
    [ex::FieldOrder::Progressive, ex::FieldOrder::UpperFirst, ex::FieldOrder::LowerFirst].iter().for_each(|f| push(&mut out, f.label()));
    [ex::BitrateMode::Cbr, ex::BitrateMode::Vbr1Pass, ex::BitrateMode::Vbr2Pass, ex::BitrateMode::Crf].iter().for_each(|m| push(&mut out, m.label()));
    [ex::H264Profile::Baseline, ex::H264Profile::Main, ex::H264Profile::High].iter().for_each(|p| push(&mut out, p.label()));
    crate::panels::export_mode::RANGES.iter().for_each(|(_, l)| push(&mut out, l));
    crate::panels::export_mode::PARS.iter().for_each(|(l, _)| push(&mut out, l));
    for template in filmcraft_project::gtemplate::builtin_templates() {
        push(&mut out, &template.name);
        push(&mut out, &template.category);
        push(&mut out, &template.description);
        template.controls.iter().for_each(|c| push(&mut out, &c.name));
    }
    // effect presets (Effects panel ▸ Presets)
    for p in filmcraft_engine::presets::builtin_presets() {
        push(&mut out, &p.name);
        push(&mut out, &p.description);
    }
    // every command (Keyboard Shortcuts dialog, History panel) and the shortcut contexts
    let app = crate::FilmcraftApp::new(filmcraft_engine::Session::default());
    for c in app.session.shortcuts.commands() {
        push(&mut out, &c.label);
        push(&mut out, &c.category);
    }
    push(&mut out, filmcraft_engine::shortcuts::APPLICATION);
    filmcraft_engine::shortcuts::PANELS.iter().for_each(|p| push(&mut out, p));
    // metadata rows, item types and history entries of the demo project
    let mut s = filmcraft_engine::Session::default();
    let _ = s.execute("file.openDemoProject", serde_json::json!({}));
    for it in s.project.items.values() {
        push(&mut out, it.type_label());
        for f in filmcraft_engine::panels::metadata_fields(&s.project, it.id) {
            push(&mut out, f.section);
            push(&mut out, &f.name);
        }
    }
    out
}

#[test]
fn tl_literals_are_found_and_unescaped() {
    let src = "a(tl!(\"Save\")); b(tl!(\"Say \\\"hi\\\"\\n\")); tl!(x); tlf!(\"{n} file(s)\", n);";
    assert_eq!(tl_literals(src), ["Save", "Say \"hi\"\n", "{n} file(s)"]);
}

fn assert_bundled_fonts_cover(text: impl Iterator<Item = char>) {
    let ctx = egui::Context::default();
    crate::theme::install(&ctx, &crate::theme::Tokens::for_kind(crate::theme::ThemeKind::default()));
    let mut output = ctx.run_ui(egui::RawInput::default(), |_| {});
    output.textures_delta.clear();
    let chars: std::collections::BTreeSet<_> = text.filter(|ch| !ch.is_ascii()).collect();
    ctx.fonts_mut(|fonts| {
        for family in crate::theme::font_families() {
            let font = egui::FontId::new(13.0, family);
            // has_glyph compares font faces, giving false negatives when Inter also supplies
            // the replacement character (the named medium/semibold stacks). Compare the
            // actual glyph texture regions instead; the noncharacter U+10FFFF is missing.
            let text: String = chars.iter().copied().chain(['\u{10ffff}']).collect();
            let galley = fonts.layout_no_wrap(text, font.clone(), egui::Color32::WHITE);
            let glyphs = &galley.rows[0].glyphs;
            assert_eq!(glyphs.len(), chars.len() + 1);
            let replacement = glyphs.last().unwrap().uv_rect;
            for glyph in glyphs.iter().take(chars.len()) {
                assert_ne!(glyph.uv_rect, replacement, "missing {} in {font:?}", glyph.chr);
            }
        }
    });
}

#[test]
fn language_menu_and_settings_choices_are_in_the_same_order() {
    use filmcraft_engine::settings::{Kind, Row};
    fn choices(rows: &[Row], out: &mut Vec<&'static str>) {
        for r in rows {
            match r {
                Row::Field(f) if f.key == "general.interfaceLanguage" => {
                    if let Kind::Choice(opts) = f.kind {
                        out.extend(opts.iter().filter(|(code, _)| *code != "system").map(|(_, label)| *label));
                    }
                }
                Row::Group(_, inner) => choices(inner, out),
                _ => {}
            }
        }
    }
    let mut settings = Vec::new();
    for c in filmcraft_engine::settings::categories() {
        choices(c.rows, &mut settings);
    }
    let app = crate::FilmcraftApp::new(filmcraft_engine::Session::default());
    let menu: Vec<String> = crate::menus::menu_items(&app).into_iter().filter(|it| it.id.starts_with("app.language.")).map(|it| it.label).collect();
    assert!(!settings.is_empty());
    assert_eq!(menu, settings);
}

#[test]
fn russian_renders_with_bundled_fonts() {
    let (entries, _) = catalog::parse_entries(include_str!("ru.tsv"));
    // the whole alphabet including «ё», even letters no entry uses yet, and every letter, quote,
    // dash and ellipsis the entries use
    let alphabet = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдеёжзийклмнопрстуфхцчшщъыьэюя«»";
    let letters = entries.iter().flat_map(|(_, _, ru)| ru.chars()).filter(|ch| ch.is_alphabetic() || "«»“”‘’–—…".contains(*ch));
    assert_bundled_fonts_cover(alphabet.chars().chain(Language::Ru.name().chars()).chain(letters));
}

#[test]
fn supported_languages_and_old_preferences() {
    assert_eq!(Language::ALL, [Language::En, Language::Ru]);
    for lang in Language::ALL {
        assert_eq!(Language::parse(lang.code()), Some(lang));
        assert_eq!(serde_json::from_str::<Language>(&serde_json::to_string(&lang).unwrap()).unwrap(), lang);
    }
    for code in ["ja", "es", "de", "tr", "zh-cn"] {
        assert_eq!(Language::parse(code), None);
        assert_eq!(serde_json::from_value::<Language>(serde_json::json!(code)).unwrap(), Language::En);
    }
    assert_eq!(Language::from_locales(&["de-DE".into(), "ru_RU.UTF-8".into()]), Language::Ru);
    assert_eq!(Language::from_locales(&["de-DE".into()]), Language::En);
}

#[test]
fn language_commands_switch_and_persist() {
    let mut app = crate::FilmcraftApp::new(filmcraft_engine::Session::default());
    let ctx = egui::Context::default();
    for (name, lang) in [("russian", Language::Ru), ("english", Language::En)] {
        crate::menus::invoke(&mut app, &ctx, &format!("app.language.{name}"), serde_json::json!({})).unwrap();
        assert_eq!(app.ui.language, lang);
        assert_eq!(app.session.prefs.general.interface_language, lang.code());
        let mut restarted = filmcraft_engine::Session::default();
        restarted.prefs = app.session.prefs.clone();
        assert_eq!(crate::FilmcraftApp::new(restarted).ui.language, lang);
    }
    for name in ["japanese", "spanish", "german", "turkish", "chinese"] {
        assert!(crate::menus::invoke(&mut app, &ctx, &format!("app.language.{name}"), serde_json::json!({})).is_err());
    }
    set_current(Language::En);
}
