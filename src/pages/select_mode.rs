use e_imzo_rs::list_all_certificates;
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self},
    *,
};

use crate::utils::{
    add_file_row_to_list, check_file_ownership, get_pfx_files_in_folder,
    return_pfx_files_in_folder, show_alert_dialog, tasks_filename_filters,
};

use relm4_components::open_dialog::*;
use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitStatus,
};

use crate::{app::AppMsg, config::LIBEXECDIR};

pub struct SelectModePage {
    is_path_empty: bool,
    file_list_parent: gtk::Box,
    file_list: adw::PreferencesGroup,
    open_dialog: Controller<OpenDialog>,
}

#[derive(Debug)]
pub enum SelectModeMsg {
    OpenFile,
    OpenFileConfirmed,
    OpenFileResponse(PathBuf),
    ShowMessage(String),
    RefreshCertificates,
    None,
}

#[relm4::component(pub)]
impl SimpleComponent for SelectModePage {
    type Init = ();
    type Input = SelectModeMsg;
    type Output = AppMsg;
    type Widgets = AppWidgets;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            if model.is_path_empty {
                adw::StatusPage {
                    set_vexpand: true,
                    set_hexpand: true,
                    set_icon_name: Some("checkbox-checked-symbolic"),
                    set_title: &gettext("No certificates"),
                    set_description: Some(&gettext("Load some certificates to start using the app.")),
                    gtk::Button {
                        set_halign: gtk::Align::Center,
                        set_focus_on_click: true,
                        set_css_classes: &["pill", "suggested-action"],
                        adw::ButtonContent {
                            set_icon_name: "folder-documents-symbolic",
                            #[watch]
                            set_label: "Load .pfx",
                        },
                        connect_clicked => SelectModeMsg::OpenFile,
                    },
                }
            } else {
                gtk::Box {
                    gtk::Label {
                        add_css_class: relm4::css::TITLE_2,
                        #[watch]
                        set_label: &gettext("Loaded certificates"),
                        set_margin_all: 1,
                    },
                    set_spacing: 20,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_margin_top: 20,
                    set_margin_bottom: 10,
                    set_orientation: gtk::Orientation::Vertical,

                    adw::Clamp {
                        #[name(file_list_parent)]
                        gtk::Box {

                        }
                    }
                }
            },
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // let sender_clone = sender.input_sender().clone();

        // glib::timeout_add_seconds_local(3, move || {
        //     sender_clone
        //         .send(SelectModeMsg::RefreshCertificates)
        //         .unwrap();
        //     glib::ControlFlow::Continue
        // });

        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(OpenDialogSettings {
                create_folders: false,
                folder_mode: false,
                cancel_label: "Cancel".into(),
                accept_label: "Open".into(),
                is_modal: true,
                filters: tasks_filename_filters(),
            })
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => SelectModeMsg::OpenFileResponse(path),
                OpenDialogResponse::Cancel => SelectModeMsg::None,
            });

        let mut model = SelectModePage {
            is_path_empty: return_pfx_files_in_folder().is_empty(),
            file_list_parent: gtk::Box::new(gtk::Orientation::Vertical, 1),
            file_list: adw::PreferencesGroup::new(),
            open_dialog,
        };
        let widgets = view_output!();
        let file_list_parent = widgets.file_list_parent.clone();
        model.file_list_parent = file_list_parent;

        match list_all_certificates() {
            Ok(pfx) => {
                pfx.iter().map(|c| c.get_alias()).for_each(|alias| {
                    add_file_row_to_list(alias, &model.file_list);
                });
                model.file_list_parent.append(&model.file_list);
            }
            _ => {
                println!("asdasd");
            }
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: SelectModeMsg, sender: ComponentSender<Self>) {
        match msg {
            SelectModeMsg::OpenFile => {
                if Path::new("/media/DSKEYS").exists() && check_file_ownership().unwrap() == 1000 {
                    self.open_dialog.emit(OpenDialogMsg::Open);
                } else {
                    relm4::spawn(async move {
                        let output = tokio::process::Command::new("pkexec")
                            .arg(format!("{}/e-helper", LIBEXECDIR))
                            .output()
                            .await;
                        match output {
                            Ok(output) => {
                                if !ExitStatus::success(&output.status) {
                                    // do nothing if user canceled entering password
                                    return;
                                }
                                sender.input(SelectModeMsg::OpenFileConfirmed);
                            }
                            Err(e) => {
                                eprintln!("Failed to execute pkexec: {}", e);
                            }
                        }
                    });
                }
            }
            SelectModeMsg::OpenFileConfirmed => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            SelectModeMsg::OpenFileResponse(path) => {
                let copied_file = &path.file_name().unwrap().to_str().unwrap();

                if return_pfx_files_in_folder().contains(&copied_file.to_string()) {
                    let _ = sender.input(SelectModeMsg::ShowMessage(
                        gettext("File already exists. You can use it").to_string(),
                    ));
                } else {
                    let _ = fs::copy(&path, format!("/media/DSKEYS/{}", copied_file));
                    let _ = sender.input(SelectModeMsg::RefreshCertificates);
                }
            }

            SelectModeMsg::ShowMessage(text) => {
                show_alert_dialog(&text);
            }
            SelectModeMsg::RefreshCertificates => {
                self.file_list_parent.remove_all();
                let new_group = adw::PreferencesGroup::new();

                // Save reference so it can be updated later
                self.file_list = new_group;

                if Path::new("/media/DSKEYS").exists() {
                    match list_all_certificates() {
                        Ok(pfx) => {
                            pfx.iter().map(|c| (c.get_alias())).for_each(|alias| {
                                add_file_row_to_list(alias, &self.file_list);
                            });
                        }
                        Err(e) => {
                            eprintln!("list_all_certificates:::::::::::::::: {}", e);
                        }
                    };
                    self.file_list_parent.append(&self.file_list);
                    self.is_path_empty = return_pfx_files_in_folder().is_empty();
                }
            }
            SelectModeMsg::None => {}
        }
    }
}
