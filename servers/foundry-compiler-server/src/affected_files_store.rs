use std::collections::HashMap;

#[derive(Debug)]
pub struct AffectedFilesStore {
    projects_files: HashMap<String, Vec<String>>,
}

impl AffectedFilesStore {
    pub fn new() -> Self {
        Self {
            projects_files: HashMap::new(),
        }
    }

    pub fn add_project_file(&mut self, project_path: String, file: String) {
        if let Some(files) = self.projects_files.get_mut(&project_path) {
            if !files.contains(&file) {
                files.push(file);
            }
        } else {
            self.projects_files.insert(project_path, vec![file]);
        }
    }

    /**
     * This function returns the list of files that previously raised an error and are not raising it anymore.
     * It also updates the list of files that are raising an error.
     * @param {Vec<String>} raised_files List of files that are raising an error
     * @param {String} project_path Project path
     * @returns {Vec<String>} List of files that are not raising an error anymore
     */
    pub fn fill_affected_files(
        &mut self,
        raised_files: Vec<String>,
        project_path: &str,
    ) -> Vec<String> {
        let mut affected_files = Vec::new();
        if let Some(project_files) = self.projects_files.get_mut(project_path) {
            project_files.retain(|file| !raised_files.contains(file));
            affected_files = project_files.clone();
            project_files.extend(raised_files);
        } else {
            self.projects_files
                .insert(project_path.to_string(), raised_files);
        }
        affected_files
    }
}
