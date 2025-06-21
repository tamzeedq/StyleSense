use tower_lsp::{LspService, Server};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, 
    DidOpenTextDocumentParams, InitializeParams, InitializeResult, InitializedParams,
    Position, Range, ServerCapabilities, TextDocumentSyncCapability, 
    TextDocumentSyncKind, Url,
};
use tower_lsp::Client;
use tower_lsp::LanguageServer;

// Import our parser modules
mod parser;
mod definitions;
use parser::tree::get_syntax_tree;
use definitions::types::SupportedLanguageE;

#[derive(Debug)]
struct Backend {
    client: Client,
}

impl Backend {    /// Analyzes a document and publishes diagnostics
    async fn analyze_document(&self, uri: Url, text: &str, language_id: &str) {
        // Determine the language
        let language = match language_id {
            "c" => SupportedLanguageE::LangC,
            "cpp" => SupportedLanguageE::LangCPP,
            _ => return, // Unsupported language
        };
        
        // Parse the file using tree-sitter
        match get_syntax_tree(&text, language) {
            Ok(_tree) => {
                // Successfully parsed - proceed with style checking
            }
            Err(_e) => {
                // Failed to parse - skip style checking
                return;
            }
        }
        
        // Perform style checking
        let mut diagnostics = Vec::new();
        
        for (line_idx, line) in text.lines().enumerate() {
            if let Some(col_idx) = line.find("=") {
                // Check for space before equals sign
                if col_idx > 0 && !line.chars().nth(col_idx - 1).unwrap().is_whitespace() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_idx as u32, character: col_idx as u32 - 1 },
                            end: Position { line: line_idx as u32, character: col_idx as u32 + 1 },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("stylesense".to_string()),
                        message: "Missing space before '='".to_string(),
                        ..Default::default()
                    });
                }
                
                // Check for space after equals sign
                if col_idx < line.len() - 1 && !line.chars().nth(col_idx + 1).unwrap().is_whitespace() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_idx as u32, character: col_idx as u32 },
                            end: Position { line: line_idx as u32, character: col_idx as u32 + 2 },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("stylesense".to_string()),
                        message: "Missing space after '='".to_string(),
                        ..Default::default()
                    });
                }
            }
        }
        
        // Publish diagnostics
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {    
    
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL, // will need to change document sync from full to incremental later
                )),
                // We'll add more capabilities as needed later
                ..Default::default()
            },
            ..Default::default()
        })
    }    async fn initialized(&self, _: InitializedParams) {
        // Server initialized successfully
    }
    
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let language_id = &params.text_document.language_id;
        
        self.analyze_document(uri, &text, language_id).await;
    }    
      async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        
        // Get the language ID from the URI extension
        let language_id = if uri.path().ends_with(".cpp") || uri.path().ends_with(".cxx") || uri.path().ends_with(".cc") {
            "cpp"
        } else if uri.path().ends_with(".c") || uri.path().ends_with(".h") {
            "c"
        } else {
            return; // Unsupported file type
        };
        
        // With FULL sync, we should always get the complete document content
        if let Some(change) = params.content_changes.first() {
            self.analyze_document(uri, &change.text, language_id).await;
        }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}