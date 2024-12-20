use crate::completions::auto_complete_provider::AutoCompleteProvider;
use crate::error::CodeActionError;
use crate::references::reference_provider::ReferenceProvider;
use crate::types::{CompletionItem, Location, Position};
use osmium_libs_solidity_ast_extractor::extract::extract_ast_from_foundry;
use osmium_libs_solidity_ast_extractor::types::SolidityAstFile;
use std::sync::RwLock;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct CodeActionsProvider {
    pub files: Arc<Mutex<Vec<SolidityAstFile>>>,
    pub base_path: RwLock<String>,
}

impl Default for CodeActionsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeActionsProvider {
    pub fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(vec![])),
            base_path: RwLock::new(String::new()),
        }
    }

    pub fn set_base_path(&self, base_path: String) {
        let mut r = self.base_path.write().unwrap();
        *r = base_path;
    }

    pub fn update_file_content(&self) -> Result<(), CodeActionError> {
        let new_files = extract_ast_from_foundry(&self.base_path.read().unwrap())?; // will always find the root foundry project
        let mut files = self.files.lock().unwrap();
        *files = new_files;
        Ok(())
    }

    pub fn get_references(&self, uri: &str, position: Position) -> Vec<Location> {
        let files = self.files.lock().unwrap();
        let provider = ReferenceProvider::new();
        provider.get_references(uri, position, &files)
    }

    pub fn get_definition(&self, uri: &str, position: Position) -> Option<Location> {
        let files = self.files.lock().unwrap();
        let provider = ReferenceProvider::new();
        provider.get_definition(
            uri,
            position,
            &files,
            self.base_path.read().unwrap().as_str(),
        )
    }

    pub fn get_completions(&self, uri: &str, position: Position) -> Vec<CompletionItem> {
        let files = self.files.lock().unwrap();
        let provider = AutoCompleteProvider::new();
        provider.get_suggestions(uri, position, &files)
    }

    pub fn refactor(&self, uri: &str, position: Position) -> Vec<Location> {
        let mut refactors: Vec<Location> = vec![];
        if let Some(def) = self.get_definition(uri, position) {
            let refs = self.get_references(def.uri.as_str(), def.clone().start);
            refactors.push(def);
            for reference in refs {
                refactors.push(reference);
            }
        }
        refactors
    }
}
