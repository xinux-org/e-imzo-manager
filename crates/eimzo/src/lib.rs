use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::{fs, io, process::Command};


pub fn is_service_active(service_name: &str) -> Result<bool, String> {
    let output = Command::new("systemctl")
        .args(&["--user", "is-active", service_name])
        .output()
        .map_err(|e| format!("Failed to run systemctl: {}", e))?;

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

    match status.as_str() {
        "active" => Ok(true),
        "inactive" | "failed" | "activating" | "deactivating" | "unknown" => Ok(false),
        _ => Err(format!("Unexpected status: {}", status)),
    }
}

pub fn check_service_active(service: &str) -> bool {
    match is_service_active(service) {
        Ok(active) => active,
        Err(_e) => {
            // eprintln!("Error checking service: {}", e);
            false
        }
    }
}

pub fn get_pfx_files_in_folder() -> io::Result<Vec<String>> {
    let path = Path::new("/media/DSKEYS");
    let entries = fs::read_dir(path)?;

    let pfx_files: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension()?.to_str()? == "pfx" {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(pfx_files)
}

pub fn check_file_ownership() -> Result<u32, Box<dyn std::error::Error>> {
    let path = Path::new("/media/DSKEYS");
    let metadata = fs::metadata(path)?;
    let uid = metadata.uid();
    return Ok(uid);
}
