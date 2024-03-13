use glob::{glob, PatternError};
use osmium_libs_lsp_server_wrapper::{
    lsp_types::{InitializeParams, MessageType, WorkspaceFolder},
    Client,
};

pub fn get_closest_config_filepath(
    connection: &Client,
    params: InitializeParams,
) -> Result<Option<String>, PatternError> {
    let root_path_url = params.root_uri.unwrap();
    let root_path = root_path_url.path();
    connection.log_message(MessageType::INFO, format!("root_path: {:?}", root_path));

    if let Some(folders) = params.workspace_folders {
        connection.log_message(MessageType::INFO, format!("folders: {:?}", folders));
        return get_closest_workspace_config_filepath(connection, folders);
    }

    // Return the path to the closest .solidhunter.json file
    let paths = glob(&format!("{}/**/.solidhunter.json", root_path))?;
    let mut all_configs = vec![];
    for path in paths.flatten() {
        all_configs.push(path.to_str().unwrap().to_string());
    }
    all_configs.sort_by_key(|a| a.len());
    if all_configs.is_empty() {
        return Ok(None);
    }
    Ok(Some(all_configs[0].clone()))
}

fn get_closest_workspace_config_filepath(
    connection: &Client,
    folders: Vec<WorkspaceFolder>,
) -> Result<Option<String>, PatternError> {
    let mut paths: Vec<String> = Vec::new();
    for folder in folders {
        let workspace_path = folder.uri.path();

        let file_content =
            match std::fs::read_to_string(format!("{}/.solidhunter.json", workspace_path)) {
                Ok(content) => content,
                Err(err) => {
                    connection.log_message(
                        MessageType::ERROR,
                        format!(
                            "error, cannot read file: {:?}, error: {:?}",
                            format!("{}/.solidhunter.json", workspace_path),
                            err
                        ),
                    );
                    continue;
                }
            };
        connection.log_message(
            MessageType::INFO,
            format!("file_content: {:?}", file_content),
        );

        let pattern = format!("{}/**/.solidhunter.json", workspace_path);
        connection.log_message(MessageType::INFO, format!("pattern: {:?}", pattern));
        let workspaces_paths = glob(&pattern).map_err(|err| {
            connection.log_message(MessageType::ERROR, format!("error: {:?}", err));
            err
        })?;
        let mut all_configs = vec![];
        for path in workspaces_paths {
            match path {
                Ok(path) => {
                    connection.log_message(MessageType::INFO, format!("pushing path: {:?}", path));
                    all_configs.push(path.to_str().unwrap().to_string());
                }
                Err(err) => {
                    connection.log_message(MessageType::ERROR, format!("error: {:?}", err));
                }
            }
        }
        connection.log_message(MessageType::INFO, format!("all_configs: {:?}", all_configs));
        all_configs.sort_by_key(|a| a.len());
        // Push the shortest path , if any exist
        if !all_configs.is_empty() {
            connection.log_message(
                MessageType::INFO,
                format!("pushing workspace_path: {:?}", workspace_path),
            );
            paths.push(all_configs[0].clone());
        }
    }
    paths.sort_by_key(|a| a.len());
    connection.log_message(MessageType::INFO, format!("paths: {:?}", paths));
    if paths.is_empty() {
        return Ok(None);
    }
    Ok(Some(paths[0].clone()))
}
