use tower_lsp::{LspService, Server};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOptions, CodeActionOrCommand, CodeActionParams, CodeActionResponse,
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, 
    DidOpenTextDocumentParams, InitializeParams, InitializeResult, InitializedParams,
    Position, Range, ServerCapabilities, TextDocumentSyncCapability, 
    TextDocumentSyncKind, TextEdit, Url, WorkspaceEdit,
};
use std::collections::HashMap;
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

impl Backend {
    /// Creates a code action to fix a specific diagnostic
    async fn create_fix_action(&self, uri: &Url, diagnostic: &Diagnostic) -> Option<CodeAction> {
        let title = match diagnostic.message.as_str() {
            "Missing space before '='" => "Add space before '='",
            "Missing space after '='" => "Add space after '='",
            _ => return None,
        };

        let edit = self.create_text_edit_for_diagnostic(diagnostic)?;
        
        let mut changes = HashMap::new();
        changes.insert(uri.clone(), vec![edit]);
        
        let workspace_edit = WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        };

        Some(CodeAction {
            title: title.to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            edit: Some(workspace_edit),
            is_preferred: Some(true),
            ..Default::default()
        })
    }    /// Creates the appropriate text edit for a diagnostic
    fn create_text_edit_for_diagnostic(&self, diagnostic: &Diagnostic) -> Option<TextEdit> {
        match diagnostic.message.as_str() {
            "Missing space before '='" => {
                // Insert a space right before the '=' character
                // diagnostic.range.start points to the character before '=', so '=' is at start + 1
                Some(TextEdit {
                    range: Range {
                        start: Position {
                            line: diagnostic.range.start.line,
                            character: diagnostic.range.start.character + 1, // Position of the '=' character
                        },
                        end: Position {
                            line: diagnostic.range.start.line,
                            character: diagnostic.range.start.character + 1, // Same position for insertion
                        },
                    },
                    new_text: " ".to_string(),
                })
            }
            "Missing space after '='" => {
                // Insert a space after the '=' character
                Some(TextEdit {
                    range: Range {
                        start: Position {
                            line: diagnostic.range.start.line,
                            character: diagnostic.range.start.character + 1,
                        },
                        end: Position {
                            line: diagnostic.range.start.line,
                            character: diagnostic.range.start.character + 1,
                        },
                    },
                    new_text: " ".to_string(),
                })
            }
            _ => None,
        }
    }

    /// Analyzes a document and publishes diagnostics
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
                code_action_provider: Some(tower_lsp::lsp_types::CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![CodeActionKind::QUICKFIX]),
                        work_done_progress_options: Default::default(),
                        resolve_provider: Some(false),
                    }
                )),
                // We'll add more capabilities as needed later
                ..Default::default()
            },
            ..Default::default()
        })
    }async fn initialized(&self, _: InitializedParams) {
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
    }    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let mut actions = Vec::new();
        
        // Get the document URI and range
        let uri = params.text_document.uri;
        let range = params.range;
        
        // Check each diagnostic in the context
        for diagnostic in params.context.diagnostics {
            if diagnostic.source.as_deref() == Some("stylesense") {
                if let Some(action) = self.create_fix_action(&uri, &diagnostic).await {
                    actions.push(CodeActionOrCommand::CodeAction(action));
                }
            }
        }
        
        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}