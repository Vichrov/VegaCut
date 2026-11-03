# VegaCut

An independent Rust video editor based on [FilmCraft](https://github.com/storytold/filmcraft),
with a Vegas-inspired editing workflow for Windows x64. Not affiliated with VEGAS or ArtCraft.

## Status

Early development. Version 0.1 establishes the fork; 0.2 introduces its editing workspace.
A version number is not a claim of production readiness. See [known issues](KNOWN_ISSUES.md)
and [validation](docs/VEGACUT_VALIDATION.md).

## Build

Use Rust 1.95 or later and the platform dependencies in [contributing](docs/contributing.md).
Internal Cargo package names remain unchanged to keep upstream merges and automation compatible:

```sh
cargo build --release --locked -p filmcraft -p filmcraft-cli
cargo run -p filmcraft -- --demo
```

On Windows the portable packaging script produces `VegaCut.exe` and `VegaCut-cli.exe`.
Settings use a separate VegaCut data directory (or `VEGACUT_DATA_DIR` / `--data-dir`).
Projects retain the `.fcproj` format. No `.veg` import is claimed.
Automatic application replacement is disabled; Help > Check for Updates opens this fork's releases.

## Project

- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)
- [Vegas workflow mapping](docs/VEGAS_PARITY.md)
- [Fork architecture](docs/VEGACUT_ARCHITECTURE.md)

## Credits and licence

Based on FilmCraft by the ArtCraft team and contributors. Upstream copyright, licences,
contributor credits and NOTICE are retained. Code: MIT OR Apache-2.0.
See [ATTRIBUTION.md](ATTRIBUTION.md) for assets. VegaCut has its own original mark.
