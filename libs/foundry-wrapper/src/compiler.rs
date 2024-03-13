use crate::{
    error::Error,
    types::ProjectCompileOutput,
    utils::{
        check_executable_argument, find_forge_executable, find_projects_paths, normalize_path,
    },
};
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
        let filepath = normalize_path(file_path);
        self.inner
            .workspaces
            .iter()
            .filter(|path| filepath.starts_with(path.as_str()))
            .max_by_key(|path| path.len())
            .map(|path| path.to_string())
    }

    pub fn load_workspace(&mut self, root_folder: String) -> Result<(), Error> {
        let paths = find_projects_paths(&root_folder)?;
        for path in paths {
            if let Some(path) = path.to_str() {
                self.inner.workspaces.push(normalize_path(path));
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
}
