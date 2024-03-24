use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tower_lsp::lsp_types::{
    CodeAction, CodeActionOrCommand, CodeActionResponse, CompletionItem, CompletionResponse,
    Diagnostic, DiagnosticSeverity, Documentation, GotoDefinitionResponse, Hover, HoverContents,
    Location, MarkedString, Position, Range, TextEdit, Url, WorkspaceEdit,
};

#[derive(Debug, Default)]
pub struct State {
    documents: Arc<Mutex<HashMap<Url, String>>>,
}

impl State {
    pub fn open_document(&self, uri: Url, text: String) -> Vec<Diagnostic> {
        let mut lock = self
            .documents
            .lock()
            .expect("Failed to lock state documents");
        lock.insert(uri, text.clone());
        Self::get_diagnostics_for_file(text)
    }

    pub fn update_document(&self, uri: Url, text: String) -> Vec<Diagnostic> {
        let mut lock = self
            .documents
            .lock()
            .expect("Failed to lock state documents");
        lock.insert(uri, text.clone());
        Self::get_diagnostics_for_file(text)
    }

    pub fn hover(&self, uri: &Url, _position: Position) -> Hover {
        let lock = self
            .documents
            .lock()
            .expect("Failed to lock state documents");
        let document = lock.get(uri).expect("Failed to find document: {uri}");
        Hover {
            contents: HoverContents::Scalar(MarkedString::String(format!(
                "File: {uri}, Characters: {}",
                document.len()
            ))),
            range: Default::default(),
        }
    }

    pub fn definition(&self, uri: &Url, position: Position) -> GotoDefinitionResponse {
        return GotoDefinitionResponse::Scalar(Location {
            uri: uri.clone(),
            range: tower_lsp::lsp_types::Range {
                start: Position {
                    line: position.line - 1,
                    character: 0,
                },
                end: Position {
                    line: position.line - 1,
                    character: 0,
                },
            },
        });
    }

    pub fn code_action(&self, uri: &Url) -> CodeActionResponse {
        let lock = self
            .documents
            .lock()
            .expect("Failed to lock state documents");
        let mut actions = vec![];
        let text = lock.get(uri).expect("Failed to find document: {uri}");
        for (row, line) in text.split("\n").enumerate() {
            let find_idx = line.find("VS Code");
            if let Some(idx) = find_idx {
                let idx_32: u32 = idx.try_into().expect("Failed to convert usize to u32");
                let row_32: u32 = row.try_into().expect("Failed to convert usize to u32");

                let idx_len: u32 = "VS Code"
                    .len()
                    .try_into()
                    .expect("Failed to convert usize to u32");
                let replace_change = TextEdit {
                    range: Self::line_range(row_32, idx_32, idx_32 + idx_len),
                    new_text: "Neovim".into(),
                };
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "Replace VS C*de with a superior editor".into(),
                    edit: WorkspaceEdit {
                        changes: HashMap::from([(uri.clone(), vec![replace_change])]).into(),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                }));

                let censor_change = TextEdit {
                    range: Self::line_range(row_32, idx_32, idx_32 + idx_len),
                    new_text: "VS C*de".into(),
                };
                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "Censor to VS C*de".into(),
                    edit: WorkspaceEdit {
                        changes: HashMap::from([(uri.clone(), vec![censor_change])]).into(),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                }));
            }
        }
        actions
    }

    pub fn completion(&self, _uri: &Url) -> CompletionResponse {
        let items = vec![CompletionItem {
            label: "Neovim (BTW)".into(),
            detail: "Very cool editor".to_owned().into(),
            documentation: Documentation::String("Some example documentation".into()).into(),
            ..Default::default()
        }];

        CompletionResponse::Array(items)
    }

    fn line_range(line: u32, start: u32, end: u32) -> Range {
        Range {
            start: Position {
                line,
                character: start,
            },
            end: Position {
                line,
                character: end,
            },
        }
    }

    fn get_diagnostics_for_file(text: String) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        for (row, line) in text.split("\n").enumerate() {
            let row_32: u32 = row.try_into().expect("Failed to convert usize to u32");
            let find_idx: Option<u32> = line
                .find("VS Code")
                .map(|idx| idx.try_into().expect("Failed to convert usize to u32"));
            if let Some(idx) = find_idx {
                let idx_len: u32 = "VS Code"
                    .len()
                    .try_into()
                    .expect("Failed to convert usize to u32");
                diagnostics.push(Diagnostic {
                    range: Self::line_range(row_32, idx, idx + idx_len),
                    severity: DiagnosticSeverity::ERROR.into(),
                    source: "Common sense".to_owned().into(),
                    message: "Please make sure we use good language".to_owned().into(),
                    ..Default::default()
                });
            }
            let find_idx: Option<u32> = line
                .find("Neovim")
                .map(|idx| idx.try_into().expect("Failed to convert usize to u32"));
            if let Some(idx) = find_idx {
                let idx_len: u32 = "Neovim"
                    .len()
                    .try_into()
                    .expect("Failed to convert usize to u32");
                diagnostics.push(Diagnostic {
                    range: Self::line_range(row_32, idx, idx + idx_len),
                    severity: DiagnosticSeverity::HINT.into(),
                    source: "Common sense".to_owned().into(),
                    message: "Great choice :)".to_owned().into(),
                    ..Default::default()
                });
            }
        }

        diagnostics
    }
}
