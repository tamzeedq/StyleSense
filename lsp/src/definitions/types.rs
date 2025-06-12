use tree_sitter::*;
extern crate tree_sitter_c;
extern crate tree_sitter_cpp;

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
            SupportedLanguageE::LangC => tree_sitter_c::LANGUAGE.into(),
            SupportedLanguageE::LangCPP => tree_sitter_cpp::LANGUAGE.into(),
        }
    }
}
