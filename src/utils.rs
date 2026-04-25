use relm4::{
    AsyncComponentSender,
    gtk::{self},
};
use std::{fs, io, os::unix::fs::MetadataExt, path::Path, process::Command};

use crate::{
    config::MEDIA_DSKEYS,
    ui::select_mode::{SelectModeMsg, SelectModePage},
};

pub fn hide_sensitive_string(name: String, symbol: char, range: usize) -> String {
    name.chars()
        .enumerate()
        .map(|(i, c)| if i <= range { c } else { symbol })
        .collect()
}

pub fn is_service_active(service_name: &str) -> Result<bool, String> {
    let output = Command::new("systemctl")
        .args(["--user", "is-active", service_name])
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
    is_service_active(service).unwrap_or_default()
}

pub fn get_pfx_files_in_folder() -> io::Result<Vec<String>> {
    let path = Path::new(MEDIA_DSKEYS);
    let entries = fs::read_dir(path)?;

    let pfx_files: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension()?.to_str()? == "pfx" {
                path.file_name()?.to_str().map(str::to_string)
            } else {
                None
            }
        })
        .collect();

    Ok(pfx_files)
}

pub fn return_pfx_files_in_folder() -> Vec<String> {
    let mut certificate = Vec::<String>::new();

    let path = Path::new(MEDIA_DSKEYS);
    if path.exists() {
        match get_pfx_files_in_folder() {
            Ok(file_names) => {
                for file_name in file_names {
                    certificate.push(file_name);
                }
            }
            Err(e) => tracing::error!(
                "Error in Init function eimzo::get_pfx_files_in_folder: {}",
                e
            ),
        }
    }
    certificate
}

pub fn check_file_ownership() -> Result<u32, Box<dyn std::error::Error>> {
    let path = Path::new(MEDIA_DSKEYS);
    let metadata = fs::metadata(path)?;
    let uid = metadata.uid();
    Ok(uid)
}

pub fn check_service_installed(service: &str) -> bool {
    let path = Path::new(service);

    if path.exists() {
        return true;
    }

    false
}

// file selection filter .pfx file
pub fn tasks_filename_filters() -> Vec<gtk::FileFilter> {
    let filename_filter = gtk::FileFilter::default();
    filename_filter.set_name(Some("PFX (.pfx)"));
    filename_filter.add_suffix("pfx");

    vec![filename_filter]
}

pub fn ask_password(sender: AsyncComponentSender<SelectModePage>) {
    relm4::spawn(async move {
        let path = MEDIA_DSKEYS;
        let real_uid = users::get_current_uid();

        let success = {
            let cmd = format!("mkdir -p {0} && chown {1}:{1} {0}", path, real_uid);
            tokio::process::Command::new("pkexec")
                .args(["sh", "-c", &cmd])
                .status()
                .await
                .map(|s| s.success())
                .unwrap_or(false)
        };

        if success {
            sender.input(SelectModeMsg::OpenFileConfirmed);
        }
    });
}
