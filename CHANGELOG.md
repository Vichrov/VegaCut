# Changelog

## 0.4.0

- Fix VegaCut U: remove both event grouping and the audio/video link in one undoable command; G can regroup selected events. The old U mapping only removed ordinary groups.
- Plain clicks on clip bodies or empty timeline space now seek to the clicked frame in the VegaCut profile. Drags and modified selection clicks retain their editing behavior.
- Add red A/V synchronization badges with signed frame offsets, including live move/trim/slip previews and unlinked overlapping clips from the same source.
- A separate speed-mismatch badge avoids presenting unequal playback speeds as one constant offset. Holds and animated time remapping are excluded from numeric sync calculations.
- Add engine and pointer/keyboard regressions plus English/Russian rendered checks.

## 0.3.0

- Add the VegaCut keyboard/mouse profile: S split, G/U group/ungroup, F8 snapping, Q loop, Alt+arrows frame stepping and Up/Down timeline zoom.
- Wheel zooms about the pointer, Ctrl+wheel scrolls tracks, Shift+wheel scrolls time. Ctrl-drag copies clips without ripple insertion; Ctrl-click toggles selection.
- New installations start with VegaCut controls; existing saved profiles are preserved. Choose VegaCut in Edit > Keyboard Shortcuts when upgrading.
- Mouse behavior survives profile save/export/import, undo/redo, Cancel and restart. Other profiles retain their previous gestures.
- Add headless interaction regressions and complete the v0.1/v0.2 milestone audit.

## 0.2.0

- Keep only English and Russian; remove other interface catalogs, language commands and settings choices. Old language preferences migrate to English.

- Default VegaCut workspace: media/effects upper left, preview/meters upper right, wide timeline below.
- Compact neutral-gray panels with cyan accents and amber playhead.
- Audio track headers expose volume and stereo pan through existing mixer commands; Mute/Solo remain available.
- Wider track headers and taller audio tracks; compact rows keep controls in the full mixer.
- Named workspace save/reset/reopen remains available; previous saved layouts are preserved.
- Full Vegas key bindings and automatic overlap crossfades remain later milestones.

## 0.1.0

- Establish VegaCut from FilmCraft commit `32749e34f611735460e9773335345db026b7b037`.
- Independent name, original icon, project links and isolated preferences.
- Remove restricted ArtCraft marks; preserve upstream legal notices and credits.
- Disable automatic replacement; manual updates open VegaCut releases.
- Windows x64 portable packaging; retain internal Cargo names and `.fcproj` compatibility.

Validation and limitations: docs/VEGACUT_VALIDATION.md and KNOWN_ISSUES.md.
