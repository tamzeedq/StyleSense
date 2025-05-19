use tower_lsp::{LspService, Server};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, 
    DidOpenTextDocumentParams, InitializeParams, InitializeResult, InitializedParams,
    MessageType, Position, Range, ServerCapabilities, TextDocumentSyncCapability, 
    TextDocumentSyncKind,
};
use tower_lsp::Client;
use tower_lsp::LanguageServer;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                // We'll add more capabilities as needed later
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "StyleSense server initialized!")
            .await;
    }
    
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        
        // Basic style checking for '=' without spaces
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
        
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
      async fn did_change(&self, _params: DidChangeTextDocumentParams) {
        // Re-analyze when document changes (we'll implement this later)
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

// Include our simplified style rules
mod rules;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}