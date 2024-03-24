use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tower_lsp::lsp_types::{
    GotoDefinitionResponse, Hover, HoverContents, Location, MarkedString, Position, Url,
};

#[derive(Debug, Default)]
pub struct State {
    documents: Arc<Mutex<HashMap<Url, String>>>,
}

impl State {
    pub fn open_document(&self, uri: Url, text: String) {
        let mut lock = self
            .documents
            .lock()
            .expect("Failed to lock state documents");
        lock.insert(uri, text);
    }

    pub fn update_document(&self, uri: Url, text: String) {
        let mut lock = self
            .documents
            .lock()
            .expect("Failed to lock state documents");
        lock.insert(uri, text);
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
}
