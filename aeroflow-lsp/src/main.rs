use zero::LanguageServer;
use zero::LspService;
use zero::Server;
use serde_json::Value; // JSON-RPC value

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer as TowerLanguageServer, LspService as TowerLspService, Server as TowerServer};
use aerolang::parser; // Our existing parser

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl TowerLanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), "(".to_string()]),
                    all_commit_characters: None,
                    work_done_progress_options: Default::default(),
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                // ...
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "AeroFlow Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = &params.content_changes[0].text;
        
        // Use our real parser to check for errors
        // aerolang::parser::parse_program panics on error currently.
        // We need a non-panicking version or handle the panic.
        // For MVP, if it panics, we just catch unwind (bad practice in async) or update parser to return Result.
        // Let's assume parser is robust for now or just do basic checks.
        
        // Actually, let's catch panic for demo purposes in a spawn_blocking if needed,
        // but robust parser should return Result.
        // For now, let's just claim it parses if no panic.
        
        let diagnostics = match std::panic::catch_unwind(|| parser::parse_program(text)) {
            Ok(_) => vec![],
            Err(_) => {
                vec![Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 1 },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("afls".to_string()),
                    message: "Syntax Error (Detailed position requires parser update)".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                }]
            }
        };

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
    
    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem {
                label: "fn".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Define a function".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "actor".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Define an actor".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "send".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Send a message to an actor".to_string()),
                documentation: Some(Documentation::String("send(target, message)".to_string())),
                ..Default::default()
            },
             CompletionItem {
                label: "recv".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Receive a message".to_string()),
                ..Default::default()
            },
        ])))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("AeroFlow Syntax".to_string())),
            range: None,
        }))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = TowerLspService::new(|client| Backend { client });
    TowerServer::new(stdin, stdout, socket).serve(service).await;
}
