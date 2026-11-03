# VegaCut interface languages

Only English (`en`) and Russian (`ru`) are supported. English is the source language;
the Russian catalog is `crates/ui-egui/src/i18n/ru.tsv`.

Edit > Language and Settings > General expose the same two choices. System Language
selects a supported operating-system locale, otherwise English. Saved preferences for
removed languages migrate to English while retaining other preferences and UI state.

Missing Russian entries fall back to English. `cargo xtask i18n-report` lists them.
Commands, project schema, CLI and MCP retain stable English identifiers.
