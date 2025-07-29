use std::{
    error::Error,
    fs, io,
    process::{Command, exit},
};

#[cfg(not(feature = "development"))]
static PATH: &str = "/media/DSKEYS";

#[cfg(feature = "development")]
static PATH: &str = "/home/bahrom/DSKEYS";

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

pub fn check_path_and_perm() -> Result<(), Box<dyn Error + Send>> {
    // let user = Command::new("whoami")
    //     .output()
    //     .map_err(|e| format!("Failed to run systemctl: {}", e));

    // let username = String::from_utf8_lossy(&user.unwrap().stdout)
    //     .trim()
    //     .to_string();

    let foo = std::env::var("PATH");
    println!("{:?}", foo);

    let output = Command::new("/usr/bin/env") // "/run/wrappers/bin/pkexec"
        .args([
            "pkexec",
            "mkdir -p /media/DSKEYS/e-helper",
            // "mkdir",
            // "-p",
            // "/media/DSKEYS",
        ])
        .output()
        .map_err(|e| format!("Failed to run check_path_and_perm: {}", e));

    let output = match output {
        Ok(o) => {
            println!("{}", String::from_utf8_lossy(&o.stdout));
            o
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("We somehow made it to here with this path! {}", PATH);

    Ok(())
}
