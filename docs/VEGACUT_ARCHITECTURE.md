# VegaCut fork notes

Base: FilmCraft `32749e34f611735460e9773335345db026b7b037` (workspace 0.17.0).
VegaCut versions restart at 0.1.0; they are independent of upstream releases.

Keep internal filmcraft crate names, command IDs, integer Tick math and project schema intact.
Presentation is in crates/ui-egui; desktop hosting is apps/filmcraft. Docking comes from
pinned craft-ui. Workspace persistence is already implemented through workspaces.json.
Preferences are isolated by default_data_dir in crates/engine/src/autosave.rs.
Never point the updater at upstream or reuse upstream installer product identities.

Windows release path: packaging/windows/vegacut-portable.ps1. Other inherited packaging
is reference-only until independently adapted. Original ArtCraft restricted marks were removed.

Application versions are explicit in apps/filmcraft, apps/filmcraft-cli and crates/ui-egui.
Core workspace crates retain upstream 0.17.0: the H.264 encoder includes its package version
in SEI, whose bit count influences CBR/VBR decisions. Renumbering every core crate changes
encoded slices even without an encoder source change. Packaging reads the desktop app manifest.
