use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{
        self,
        prelude::{BoxExt, WidgetExt},
    },
    RelmWidgetExt,
};
use std::{fs, io, os::unix::fs::MetadataExt, path::Path, process::Command};
use e_imzo_rs::list_all_certificates;

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

// file selection filter .pfx file
pub fn tasks_filename_filters() -> Vec<gtk::FileFilter> {
    let filename_filter = gtk::FileFilter::default();
    filename_filter.set_name(Some("PFX (.pfx)"));
    filename_filter.add_suffix("pfx");

    vec![filename_filter]
}

// list of added certificates displayed on Listbox
pub fn add_file_row_to_list(file_name: &str, file_list: &gtk::ListBox) {
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    hbox.set_margin_all(12);
    hbox.set_hexpand(true);

    let icon = gtk::Image::from_icon_name("folder-documents-symbolic");
    hbox.append(&icon);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);

    let title = gtk::Label::new(Some(file_name));

    title.set_xalign(0.0);
    title.add_css_class("title-3");


    let pfx = list_all_certificates().expect("not found");
    let alias: Vec<_> = pfx.iter().map(|c| (c.get_alias())).collect();
    let surname = alias[0].get("surname").cloned();

    let subtitle = gtk::Label::new(surname);
    subtitle.set_xalign(0.0);
    subtitle.add_css_class("dim-label");

    vbox.append(&title);
    vbox.append(&subtitle);

    hbox.append(&vbox);
    file_list.append(&hbox);
}

pub fn show_alert_dialog(text: &str) {
    let dialog = adw::AlertDialog::builder()
        .heading(text)
        .default_response("ok")
        .follows_content_size(true)
        .build();

    dialog.add_responses(&[("ok", &gettext("OK"))]);

    dialog.connect_response(None, |dialog, response| {
        println!("Dialog response: {}", response);
        dialog.close();
    });

    if let Some(win) = relm4::main_application().active_window() {
        dialog.present(Some(&win));
    }
}

pub fn check_service_installed(service: &str) -> bool {
    let path = Path::new(service);

    if path.exists() {
        return true;
    }

    false
}
