use crate::error::Error;
use crate::types::FoundryJsonFile;
use osmium_libs_solidity_path_utils::join_path;

use std::fs::{read_dir, remove_dir_all, DirEntry};
use std::io;
use std::path::PathBuf;

pub fn remove_previous_outputs(base_path: &str) -> Result<(), Error> {
    let build_info_path = format!("{}/out/build-info", base_path);

    let res = remove_dir_all(&build_info_path);
    if let Err(e) = res {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(Error::FileSystemError(e));
        }
    }
    Ok(())
}

pub fn get_files_from_foundry_output(base_path: &str) -> Result<Vec<FoundryJsonFile>, Error> {
    let mut files = Vec::new();

    // let init_time = SystemTime::now();
    let output = std::fs::read_to_string(get_last_build_info(base_path)?)?;
    let json: serde_json::Value = serde_json::from_str(&output)?;
    for (file, json) in json["output"]["sources"]
        .as_object()
        .ok_or(Error::NoBuildInfo)?
    {
        if file.contains("safeconsole.sol") {
            // excluded file since it is very large and not useful for references
            continue;
        }
        files.push(FoundryJsonFile {
            json: json["ast"].clone(),
            file: join_path(base_path, file),
        });
    }

    /*  let current_time = SystemTime::now().duration_since(init_time).unwrap();
    info!(
        "Finished retreiving json ast in: {:?} seconds",
        current_time.as_secs()
    );
    */
    Ok(files)
}

fn get_last_build_info(base_path: &str) -> Result<PathBuf, Error> {
    let out = read_dir(base_path.to_string() + "/out/build-info")?;

    let mut entries: Vec<DirEntry> = out.flatten().collect();
    entries.sort_by(|a, b| sort_latest(a, b).unwrap_or(std::cmp::Ordering::Equal));
    let last_build_info = entries.first().ok_or(Error::NoBuildInfo)?;
    Ok(last_build_info.path())
}

fn sort_latest(a: &DirEntry, b: &DirEntry) -> Result<std::cmp::Ordering, io::Error> {
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
