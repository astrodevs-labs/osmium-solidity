use std::io;
use std::path::PathBuf;
use std::fs::{read_dir, DirEntry};
use std::time::SystemTime;
use crate::SolcJsonFile;
use crate::error::SolcWrapperError;
use serde_json;
use crate::utils::join_path;

pub fn get_files_from_solc_output(base_path: &str) -> Result<Vec<SolcJsonFile>, SolcWrapperError> {
    let mut files = Vec::new();

    let init_time = SystemTime::now();
    let output = std::fs::read_to_string(get_last_build_info(base_path)?)?;
    let json: serde_json::Value = serde_json::from_str(&output)?;
    for (file, json) in json["output"]["sources"].as_object().ok_or(SolcWrapperError::NoBuildInfo)? {
        if file.contains("safeconsole.sol") {
            continue;
        }
        files.push(SolcJsonFile {
            json: json["ast"].clone(),
            file: join_path(base_path, &file),
        });
    };


    let current_time = SystemTime::now().duration_since(init_time).unwrap();
    eprintln!("Finished retreiving json ast in: {:?} seconds", current_time.as_secs());

    Ok(files)
}

fn get_last_build_info(base_path: &str) -> Result<PathBuf, SolcWrapperError> {
    let out = read_dir(base_path.to_string() + "/out/build-info")?;
 
    let mut entries: Vec<DirEntry> = out.flatten().collect();
    entries.sort_by(|a, b| {
        sort_latest(a, b).unwrap_or(std::cmp::Ordering::Equal)
    });
    let last_build_info = entries.first().ok_or(SolcWrapperError::NoBuildInfo)?;
    Ok(last_build_info.path())
}

fn sort_latest(a: &DirEntry, b: &DirEntry) -> Result<std::cmp::Ordering, io::Error>  {
    if let Ok(met_a) = a.metadata() {
        if let Ok(met_b) = b.metadata() {
            if met_a.created()? > met_b.created()? {
                return Ok(std::cmp::Ordering::Greater);
            } else {
                return Ok(std::cmp::Ordering::Less);
            }
        }
    }
    Ok(std::cmp::Ordering::Equal)
}