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
