use crate::{
    error::Error,
    output::{get_files_from_foundry_output, remove_previous_outputs},
    types::ProjectCompileOutput,
    utils::{check_executable_argument, find_forge_executable, find_projects_paths},
    FoundryJsonFile,
};

use osmium_libs_solidity_path_utils::slashify_path;
use std::process::Command;

#[derive(Debug)]
struct CompilerInner {
    root_path: String,
    workspaces: Vec<String>,
    executable_path: String,
}

#[derive(Debug)]
pub struct Compiler {
    inner: CompilerInner,
}

impl Compiler {
    pub fn new_with_executable_check() -> Result<Self, Error> {
        let executable_path = find_forge_executable()?;
        check_executable_argument(executable_path.to_str().unwrap_or_default())?;
        Ok(Self {
            inner: CompilerInner {
                root_path: String::new(),
                workspaces: Vec::new(),
                executable_path: executable_path.to_str().unwrap_or_default().to_string(),
            },
        })
    }

    fn find_closest_workspace(&self, file_path: &str) -> Option<String> {
        self.inner
            .workspaces
            .iter()
            .filter(|path| file_path.starts_with(path.as_str()))
            .max_by_key(|path| path.len())
            .map(|path| path.to_string())
    }

    pub fn load_workspace(&mut self, root_folder: String) -> Result<(), Error> {
        let paths = find_projects_paths(&root_folder)?;
        for path in paths {
            if let Some(path) = path.to_str() {
                self.inner.workspaces.push(slashify_path(path));
            }
        }
        self.inner.root_path = root_folder;
        Ok(())
    }

    pub fn reload_project_for_file(&mut self, _: &str) -> Result<(), Error> {
        Ok(())
    }

    pub fn compile(&mut self, file_path: &str) -> Result<(String, ProjectCompileOutput), Error> {
        let workspace_path = self
            .find_closest_workspace(file_path)
            .ok_or_else(|| Error::InvalidFilePath(file_path.to_string()))?;
        let json = Command::new(&self.inner.executable_path)
            .current_dir(&workspace_path)
            .arg("compile")
            .arg("--format-json")
            .output()
            .map_err(Error::ExecutableError)?;
        let output_str = String::from_utf8_lossy(&json.stdout);
        let compile_output: ProjectCompileOutput = serde_json::from_str(&output_str)?;
        Ok((workspace_path, compile_output))
    }

    pub fn compile_ast(
        &mut self,
        file_path: &str,
    ) -> Result<(String, Vec<FoundryJsonFile>), Error> {
        let workspace_path = self
            .find_closest_workspace(file_path)
            .ok_or_else(|| Error::InvalidFilePath(file_path.to_string()))?;

        remove_previous_outputs(&workspace_path)?;
        //info!("Workspace to compile: {}", workspace_path);
        let _ = Command::new(&self.inner.executable_path)
            .current_dir(&workspace_path)
            .arg("compile")
            .arg("--build-info")
            .arg("--no-cache")
            .output()
            .map_err(Error::ExecutableError)?;

        let out = get_files_from_foundry_output(&workspace_path)?;
        Ok((workspace_path, out))
    }
}
