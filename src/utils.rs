use crate::{
    config::MEDIA_DSKEYS,
    ui::select_mode::{SelectModeMsg, SelectModePage},
};
use anyhow::{Result, bail};
use relm4::AsyncComponentSender;
use std::{
    fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::Command,
};

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

pub fn get_pfx_files_in_folder() -> Result<Vec<String>> {
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

pub fn check_service_installed(service: &str) -> bool {
    let path = Path::new(service);

    if path.exists() {
        return true;
    }

    false
}

pub fn set_folder_permission() -> bool {
    let real_uid = uzers::get_current_uid();
    let cmd = format!("mkdir -p {0} && chown {1}:{1} {0}", MEDIA_DSKEYS, real_uid);
    let status = Command::new("pkexec")
        .args(["sh", "-c", &cmd])
        .status()
        .map(|s| s.success());

    status.unwrap_or(false)
}

pub fn ask_password(sender: AsyncComponentSender<SelectModePage>) {
    relm4::spawn(async move {
        if set_folder_permission() {
            sender.input(SelectModeMsg::OpenFileConfirmed);
        }
    });
}

pub fn copy_pfx_file_to_folder(path: PathBuf) -> Result<()> {
    if path.try_exists()?
        && path.is_file()
        && let Some(filename) = path.file_name().and_then(|s| s.to_str())
        && let Ok(certs) = get_pfx_files_in_folder()
        && !certs.contains(&filename.to_string())
    {
        fs::copy(&path, format!("{}/{}", MEDIA_DSKEYS, filename))?;
        Ok(())
    } else {
        bail!("Can not copy .pfx file to keys folder")
    }
}

pub fn check_keys_ownership() -> Result<u32> {
    let path = Path::new(MEDIA_DSKEYS);
    let metadata = fs::metadata(path)?;
    let uid = metadata.uid();
    Ok(uid)
}
