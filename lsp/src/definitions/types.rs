use tree_sitter::*;

// The tree sitter grammar's for the supported languages.
unsafe extern "C" {
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
}

/// An enum describing the languages supported by StyleSense.
pub enum SupportedLanguageE {
    LangC,
    LangCPP,
}

/// Maps a given SupportedLanguageE to a Tree-sitter language object.
/// 
/// # Arguments
/// 
/// * 'self'
impl SupportedLanguageE {
    pub fn to_parser_language(self) -> Language {
        match self {
            SupportedLanguageE::LangC => unsafe { tree_sitter_c() },
            SupportedLanguageE::LangCPP => unsafe { tree_sitter_cpp() },
        }
    }
}
