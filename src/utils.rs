use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{
        self,
        prelude::{BoxExt, WidgetExt},
    },
    RelmWidgetExt,
};
use std::{collections::HashMap, fs, io, os::unix::fs::MetadataExt, path::Path, process::Command};

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

// bunch of C code in Rust
pub fn add_file_row_to_list(
    alias: HashMap<String, String>,
    file_list: &adw::PreferencesGroup,
) {
    // all data from certificate
    let validfrom = alias.get("validfrom").unwrap();
    let validto = alias.get("validto").unwrap();
    let full_name = alias.get("cn").unwrap();
    let serial_number = alias.get("serialnumber").unwrap();
    let name = alias.get("name").unwrap();
    let surname = alias.get("surname").unwrap();

    let full_name_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    full_name_box.set_margin_all(12);
    full_name_box.set_hexpand(true);
    full_name_box.append(
        &gtk::Label::builder()
            .label("F.I.O")
            .css_classes(["dim-label"])
            .build(),
    );
    full_name_box.append(&gtk::Label::new(Some(&full_name.to_uppercase())));

    let serial_number_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    serial_number_box.set_margin_all(12);
    serial_number_box.set_hexpand(true);
    serial_number_box.append(
        &gtk::Label::builder()
            .label("Sertifikat raqami:")
            .css_classes(["dim-label"])
            .build(),
    );
    serial_number_box.append(&gtk::Label::new(Some(&serial_number)));

    let valid_date_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    valid_date_box.set_margin_all(12);
    valid_date_box.set_hexpand(true);
    valid_date_box.append(
        &gtk::Label::builder()
            .label("Sertifikatning amal qilish muddati:")
            .css_classes(["dim-label"])
            .build(),
    );
    valid_date_box.append(&gtk::Label::new(Some(&format!(
        "{} - {}",
        validfrom, validto
    ))));

    let expander = adw::ExpanderRow::builder()
        .title(format!(
            "<b>{} {}</b>",
            name.to_uppercase(),
            surname.to_uppercase()
        ))
        .use_markup(true)
        .build();
    expander.add_row(&adw::ActionRow::builder().child(&full_name_box).build());
    expander.add_row(&adw::ActionRow::builder().child(&serial_number_box).build());
    expander.add_row(&adw::ActionRow::builder().child(&valid_date_box).build());

    file_list.add(&expander);
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
