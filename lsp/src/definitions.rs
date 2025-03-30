use tree_sitter::*;

unsafe extern "C" {
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
}

#[non_exhaustive]
pub enum SupportedLanguageE {
    LangC,
    LangCPP,
}

/*
 * @brief Maps a given SupportedLanguageE to a Tree-sitter language object.
 * 
 * @param[in]   self
 * 
 * @return      Ok if mapping was successful.
 * @return      Err if mapping was not successful.
 */
impl SupportedLanguageE {
    pub fn to_parser_language(self) -> Result<Language, String> {
        match self {
            SupportedLanguageE::LangC => Ok(unsafe { tree_sitter_c() }),
            SupportedLanguageE::LangCPP => Ok(unsafe { tree_sitter_cpp() }),
            _ => Err("Unsupported language".to_string()),
        }
    }
}

