//! English source strings and the Russian interface catalog.
mod catalog;
use catalog::Catalog;
use std::cell::Cell;
#[cfg(test)]
use std::sync::Arc;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Ru,
    #[default]
    #[serde(other)]
    En,
}
static RUSSIAN: OnceLock<Catalog> = OnceLock::new();
impl Language {
    pub const ALL: [Self; 2] = [Self::En, Self::Ru];
    pub fn bcp47(self) -> &'static str {
        self.code()
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ru => "Русский",
        }
    }
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "en" => Some(Self::En),
            "ru" => Some(Self::Ru),
            _ => None,
        }
    }
    pub fn from_locales(tags: &[String]) -> Self {
        tags.iter().find_map(|tag| Self::parse(&tag.split(['-', '_', '.', '@']).next().unwrap_or_default().to_ascii_lowercase())).unwrap_or_default()
    }
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ru => "ru",
        }
    }
    fn catalog(self) -> Option<&'static Catalog> {
        match self {
            Self::En => None,
            Self::Ru => Some(RUSSIAN.get_or_init(|| Catalog::parse(include_str!("i18n/ru.tsv")))),
        }
    }
    pub fn tr(self, text: &str) -> &str {
        self.catalog().and_then(|c| c.plain(text)).unwrap_or(text)
    }
    pub fn has(self, text: &str) -> bool {
        self.catalog().and_then(|c| c.plain(text)).is_some()
    }
    pub fn to_uppercase(self, text: &str) -> String {
        text.to_uppercase()
    }
    pub fn to_lowercase(self, text: &str) -> String {
        text.to_lowercase()
    }
}

thread_local! {
    /// The language the UI is drawn in. Per thread: the UI is drawn on one thread, and tests that
    /// switch language don't leak it into tests running on other threads.
    static CURRENT: Cell<Language> = const { Cell::new(Language::En) };
}

/// Set the language for drawing. `FilmcraftApp::frame` calls this at the start of every frame from
/// the UI state, so widgets translate without every call site carrying the language around.
pub fn set_current(lang: Language) {
    CURRENT.with(|c| c.set(lang));
}

/// The language the UI is drawn in.
pub fn current() -> Language {
    CURRENT.with(Cell::get)
}

/// Translate an English UI string into the current language (the `tl!` macro, for literals).
pub fn t(text: &str) -> &str {
    current().tr(text)
}

/// Uppercase `text` in the current interface language (with Turkish-aware casing when the
/// language is Turkish).
pub fn to_uppercase(text: &str) -> String {
    current().to_uppercase(text)
}

/// Lowercase `text` in the current interface language (with Turkish-aware casing when the
/// language is Turkish).
pub fn to_lowercase(text: &str) -> String {
    current().to_lowercase(text)
}

/// Match English source and Russian labels without case sensitivity.
pub fn matches_query(text: &str, query: &str) -> bool {
    let q = query.to_lowercase();
    text.to_lowercase().contains(&q) || t(text).to_lowercase().contains(&q)
}

/// Like [`t`], for an English string that needs a disambiguating `context` (a catalog row whose
/// first column is that context); falls back to the context-free translation.
pub fn tr_ctx<'a>(context: &str, text: &'a str) -> &'a str {
    let lang = current();
    lang.catalog().and_then(|c| c.contextual(context, text)).unwrap_or_else(|| lang.tr(text))
}

/// Fill `{name}` placeholders. Unknown placeholders are left as written.
pub fn fmt(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = String::new();
    let mut rest = template;
    while let Some((prefix, after)) = rest.split_once('{') {
        out.push_str(prefix);
        let Some((name, tail)) = after.split_once('}') else {
            out.push('{');
            out.push_str(after);
            return out;
        };
        if let Some((_, value)) = args.iter().find(|(key, _)| *key == name) {
            out.push_str(value);
        } else {
            out.push('{');
            out.push_str(name);
            out.push('}');
        }
        rest = tail;
    }
    out.push_str(rest);
    out
}

/// Text every Japanese interface font must cover (menus use kanji, hiragana and katakana).
#[cfg(test)]
const JAPANESE_SAMPLE: &str = "日本語ファイル編集あア";

/// Installed families preferred for Japanese interface text, best first (Gothic / sans-serif faces
/// read best at menu sizes). Any other installed face that covers `JAPANESE_SAMPLE` is used if
/// none of these is present.
#[cfg(test)]
const PREFERRED_JAPANESE: &[&str] = &[
    "Hiragino Sans",
    "Hiragino Kaku Gothic ProN",
    "Hiragino Kaku Gothic Pro",
    "Yu Gothic UI",
    "Yu Gothic",
    "Meiryo UI",
    "Meiryo",
    "Noto Sans CJK JP",
    "Noto Sans JP",
    "Source Han Sans JP",
    "Source Han Sans",
    "IPAexGothic",
    "IPAGothic",
    "TakaoGothic",
    "VL Gothic",
];

#[cfg(test)]
const JAPANESE_FONT: &str = "system-japanese";

/// A Japanese font already installed on this system, for the interface (none is bundled). Looked up
/// once per process: the system font folders are scanned on first use (name tables only), then the
/// chosen face's file is read. `None` on the web and on systems without a Japanese font.
#[cfg(test)]
pub fn system_japanese_font() -> Option<Arc<egui::FontData>> {
    static FONT: OnceLock<Option<Arc<egui::FontData>>> = OnceLock::new();
    FONT.get_or_init(|| {
        filmcraft_text::fonts::scan_system();
        let faces: Vec<_> = filmcraft_text::fonts::all_faces().into_iter().filter(|f| f.info.origin == "system" && !f.info.italic).collect();
        let covers = |f: &filmcraft_text::fonts::Face| f.covers_text(JAPANESE_SAMPLE);
        // within a family, the face closest to regular weight
        let by_weight = |f: &&Arc<filmcraft_text::fonts::Face>| f.info.weight.abs_diff(400);
        let preferred =
            PREFERRED_JAPANESE.iter().find_map(|name| faces.iter().filter(|f| f.info.family.eq_ignore_ascii_case(name) && covers(f)).min_by_key(by_weight));
        let face = preferred.or_else(|| faces.iter().filter(|f| covers(f)).min_by_key(by_weight))?;
        // kept for the life of the process (one font, read once) so installing it again after a
        // theme change shares the bytes instead of copying the whole file
        let bytes: &'static [u8] = Box::leak(face.data()?.into_boxed_slice());
        Some(Arc::new(egui::FontData::from_blob(Arc::new(bytes), face.info.index)))
    })
    .clone()
}

/// Whether the craft-fonts build input (empty unless built with `CRAFT_FONTS_DIR`) supplies a
/// Japanese interface font: some craft-fonts face covers `JAPANESE_SAMPLE`. `theme::install` adds
/// these faces to every font family, so nothing else needs installing (and no system scan runs).
#[cfg(test)]
pub fn craft_japanese_font() -> bool {
    filmcraft_text::fonts::craft_japanese().next().is_some()
        && filmcraft_text::fonts::all_faces()
            .iter()
            .any(|f| f.info.origin == filmcraft_text::fonts::CRAFT_ORIGIN && JAPANESE_SAMPLE.chars().all(|c| f.has_char(c)))
}

/// Japanese for the interface: true when built with the craft-fonts (already installed by
/// `theme::install`). Otherwise add the system's Japanese font as the last fallback of every theme font family, from the next
/// pass on. Returns false (and changes nothing) when no Japanese font is installed. Call it again
/// after `theme::install`, which replaces the font definitions.
#[cfg(test)]
pub fn install_japanese_font(ctx: &egui::Context) -> bool {
    if craft_japanese_font() {
        return true;
    }
    let Some(font) = system_japanese_font() else { return false };
    let families = crate::theme::font_families()
        .into_iter()
        .map(|family| egui::epaint::text::InsertFontFamily { family, priority: egui::epaint::text::FontPriority::Lowest })
        .collect();
    // queued for the next pass (works before the first frame); a no-op when already installed
    ctx.add_font(egui::epaint::text::FontInsert { name: JAPANESE_FONT.into(), data: (*font).clone(), families });
    true
}

#[cfg(test)]
mod tests;
