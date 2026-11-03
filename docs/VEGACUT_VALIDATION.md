# VegaCut validation

Environment: Linux x86_64, Rust 1.95.0, Mesa lavapipe Vulkan. FFmpeg 8.1.3 is an external test oracle only; it is not linked or packaged. Noto CJK system fonts were installed in the test environment.

## v0.1.0

- Desktop and CLI `cargo check`: passed.
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo xtask layers`: passed.
- `cargo xtask assets`: passed (61 attributed assets).
- `cargo xtask wasm`: passed (49 crates).
- Workspace tests completed: 4713 passed after the two environment-dependent targets were rerun, 52 upstream ignored tests. UI integration: 754 passed, 5 ignored.
- GPU checks: 49 passed, 4 upstream ignored tests.
- Saved workspace switching, reset, rename, delete and reopen: passed; offscreen screenshot inspected.

Two environment failures in the workspace run were rerun successfully: the desktop tests require WGPU_BACKEND to be unset (92 passed); the TIFF oracle needed a fresh fixture generated with the current FFmpeg (media integration target: 9 passed). Earlier test-environment failures from FFmpeg 6 and missing CJK fonts were resolved. The SVT-AV1 fixture height was corrected to the encoder's minimum 64 pixels.

Windows x64 compilation, PE resource embedding and portable ZIP packaging are checked by `.github/workflows/vegacut-windows.yml`. A Windows GUI session has not been tested in this Linux environment. The workflow publishes an alpha release only after its tests and packaging succeed.

## v0.2.0 and prerequisite audit for v0.3

Release commit: `950e8ea75f5ef82b4b3a99d1afd77f71d089a137`.
[Successful Linux and Windows release gate](https://github.com/Vichrov/VegaCut/actions/runs/41624650108).

- Complete workspace run: 4684 passed, 52 upstream skipped. Formatting, Clippy (all targets, warnings denied), layer checks, attributed assets (61) and WASM checks (49 crates) passed.
- `workspaces_ui::vegacut_workspace_controls_and_persistence` passed: desktop defaults, fixed panels, header volume/pan through actual pointer input, Undo, precision preservation, named workspace save/switch/reopen. Rendered English and Russian screenshots were inspected.
- Windows engine/edit/project-format tests, desktop compilation and portable packaging passed. The v0.2.0 ZIP and SHA-256 file were published successfully.
- v0.1 identity, original icon, separate data directory, manual updates and Windows portable distribution remain present. Its [Windows release gate](https://github.com/Vichrov/VegaCut/actions/runs/41592422458) passed.
- v0.2 layout, neutral-gray styling, audio header controls and saved workspaces are implemented. Interface catalogs and language choices are limited to English/Russian; unsupported old settings fall back to English. Unicode media names and subtitle contents remain supported.

This closes the scoped 0.1/0.2 roadmap prerequisites, not full Vegas parity. A manual Windows GUI session remains unverified; no claim of that check is made.

## v0.3.0 release gate

The candidate branch is `vegacut-v030`. The release workflow must pass the full Linux gate and Windows tests/build/package before fast-forwarding main and publishing v0.3.0. A candidate commit is not evidence of a successful release; inspect its linked Actions run.

Added regressions cover actual key-driven split/group/ungroup/Undo, snapping and loop toggles, text-field protection, modifier wheel navigation, Ctrl-drag copy preview/commit/Undo, and profile persistence/import/export/Cancel/undo/redo. Existing workspace persistence and EN/RU rendering checks run again. No local whole-workspace rebuild is claimed for this change.
