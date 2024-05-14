use crate::{error::SlitherError, types::SlitherResult};
use osmium_libs_solidity_lsp_utils::log::{error, trace};
use std::process::Stdio;
use tokio::{io::AsyncReadExt, process::Command};
use tower_lsp::lsp_types::Diagnostic;
use regex::Regex;


pub async fn parse_slither_out(
    uri: &str,
    workspace: &str,
) -> Result<Vec<Diagnostic>, SlitherError> {
    let mut results: Vec<Diagnostic> = Vec::new();

    let mut output = exec_slither(uri, workspace)?;
    let out = match output.stdout.take() {
        Some(out) => out,
        None => {
            return Err(SlitherError::Unknown(
                "Failed to get slither output pipe".to_string(),
            ))
        }
    };

    let mut buffer = tokio::io::BufReader::new(out);
    let mut dst = String::new();


    output.wait().await?;
    buffer.read_to_string(&mut dst).await?;

    let json: Result<SlitherResult, serde_json::Error> = serde_json::from_str(&dst);
    match json {
        Ok(json) => {
            for detector in json.results.detectors {
                results.append(&mut crate::types::diag_from_json(detector.clone()));
            }
        }
        Err(e) => {
            let slither_err = get_slither_error(uri, workspace).await;
            if slither_err.is_err() {
                return Err(slither_err.err().unwrap());
            }
            error!("Error parsing slither output: {}", e);
            trace!("Slither stdout: {}", dst);
            return Err(SlitherError::ParsingFailed(e));
        }
    }

    Ok(results)
}

async fn get_slither_error( uri: &str, workspace: &str ) ->Result<(), SlitherError> {

    let mut output = exec_slither_err(uri, workspace)?;

    let errout = match output.stderr.take() {
        Some(out) => out,
        None => {
            return Err(SlitherError::Unknown(
                "Failed to get slither stderr pipe".to_string(),
            ))
        }
    };

    let mut errbuffer = tokio::io::BufReader::new(errout);
    let mut errdst = String::new();

    output.wait().await?;
    errbuffer.read_to_string(&mut errdst).await?;

    if errdst.len() > 0 && errdst.contains("Error: Source file requires different compiler version") {
        let regex = Regex::new(r"(?m)(?:current compiler is.+\))").unwrap();
        let match_ = regex.find(&errdst).unwrap().as_str();
        let match_ = &match_[..match_.len()-1];
        return Err(SlitherError::Unknown(format!("Slither needs a different version from the one specified in file: {}", match_)));
    } 
    else if errdst.len() > 0 && errdst.contains("Invalid option for --evm-version:") {
        return Err(SlitherError::Unknown("Please explicitly specify the evm version in the foundry.toml file to a compatible version of your solc compiler version".to_string()));
    }
    Ok(())
}

fn exec_slither(uri: &str, workspace: &str) -> Result<tokio::process::Child, std::io::Error> {
    Command::new("slither")
        .current_dir(workspace)
        .arg(uri)
        .arg("--exclude")
        .arg("naming-convention")
        .arg("--json")
        .arg("-")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
}

fn exec_slither_err(uri: &str, workspace: &str) -> Result<tokio::process::Child, std::io::Error> {
    Command::new("slither")
        .current_dir(workspace)
        .arg(uri)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
}
