# VegaCut v0.3.0

Core Vegas-style editing controls: S split, G/U grouping, F8 snapping, Q loop, frame stepping and timeline zoom. Wheel zooms around the pointer; Ctrl+wheel scrolls tracks and Shift+wheel scrolls time. Ctrl-drag duplicates clips without ripple insertion, with one Undo.

Новый профиль управления VegaCut: S — разрезание, G/U — группировка, F8 — привязка, Q — цикл. Колесо — масштаб; Ctrl+колесо — дорожки; Shift+колесо — время; Ctrl+перетаскивание — копирование.

New installations use the profile automatically. Existing profiles are preserved: select **VegaCut** in **Edit → Keyboard Shortcuts** after upgrading. See `docs/VEGACUT_CONTROLS.md` for the exact subset and limitations.

Extract the Windows x64 portable ZIP into a writable folder and launch `VegaCut.exe`. Preferences, autosaves and recovery live in the adjacent `data` folder. Preserve that folder when upgrading. CLI: `VegaCut-cli.exe`; project format: `.fcproj`. English/Russian interface only.

The release is published only after Linux validation and Windows tests/build/packaging. Manual Windows GUI testing remains unverified. This is an early-development release, not full Vegas parity; no `.veg`, VST3 or OpenFX compatibility. Automatic overlap crossfades remain v0.4 work. Download updates manually.
