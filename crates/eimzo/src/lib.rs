use std::{
    process::Command,
    {fs, io},
};

pub fn is_service_active(service_name: &str) -> bool {
    let output = Command::new("systemctl")
        .args(&["--user", "is-active", service_name])
        .output()
        .map_err(|e| format!("Failed to run systemctl: {}", e));

    let status = String::from_utf8_lossy(&output.unwrap().stdout).trim().to_string();

    match status.as_str() {
        "active" => true,
        "inactive" | "failed" | "activating" | "deactivating" | "unknown" => false,
        _ => false,
    }
}

pub fn get_pfx_files_in_folder(path: &str) -> io::Result<Vec<String>> {
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

pub fn check_path_and_perm() -> () {
    let user = Command::new("whoami")
        .output()
        .map_err(|e| format!("Failed to run systemctl: {}", e));

    let username = String::from_utf8_lossy(&user.unwrap().stdout)
        .trim()
        .to_string();

    let output = Command::new("pkexec")
        .args([
            "sh",
            "-c",
            &format!(
                "mkdir -p /media/DSKEYS && chown {} /media/DSKEYS",
                username.as_str()
            ),
        ])
        .output()
        .map_err(|e| format!("Failed to run check_path_and_perm: {}", e));
    
    let status = String::from_utf8_lossy(&output.unwrap().stdout).trim().to_string();

    println!("We somehow made it to here! {}", status);
}
