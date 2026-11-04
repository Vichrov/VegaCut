# Known issues

- Early fork: inherited codec/export limitations are not assumed fixed.
- No `.veg` project import, VST3 or OpenFX compatibility promise.
- No automatic self-update in 0.1–0.4; download releases manually.
- Only Windows x64 portable packaging is targeted. Other upstream packaging is not a VegaCut release path.
- Legacy internal package names, automation IDs and `.fcproj` extension are intentionally retained.
- User-visible translations inherited from upstream may still use upstream wording.
- The VegaCut profile implements the documented v0.3 subset, not every Vegas gesture. Alt-drag slip, Vegas edge-trim modifiers, Shift range selection, numpad modes and stop/return transport semantics are not implemented by this profile. Existing trim tools remain available. Automatic overlap crossfades are deferred beyond the 0.4 feedback fixes.

See docs/VEGACUT_VALIDATION.md for tests actually run and outstanding checks.

- Sync badges compare events of the same project source. They do not analyze separate audio recordings; animated time remapping and frame holds are excluded from numeric offsets.
