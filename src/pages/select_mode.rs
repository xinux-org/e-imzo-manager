use relm4::{
    adw,
    gtk::{
        self, glib,
        prelude::*,
    },
    Component, ComponentController, ComponentParts, ComponentSender, Controller, JoinHandle,
    RelmIterChildrenExt, RelmWidgetExt, SimpleComponent,
};
use relm4_components::open_dialog::{
    OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings,
};

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::app::AppMsg;
use eimzo::{check_path_and_perm, get_pfx_files_in_folder};

pub struct SelectModePage {
    is_path_empty: bool,
    certificate: Vec<String>,
    file_list: gtk::ListBox,
    open_dialog: Controller<OpenDialog>,
}

#[derive(Debug)]
pub enum SelectModeMsg {
    OpenFile,
    OpenFileResponse(PathBuf),
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
                    set_icon_name: Some("checkbox-checked-symbolic"),
                    set_title: "No certificates",
                    set_description: Some("Load some certificates to start using the app."),
                    gtk::Button {
                        set_halign: gtk::Align::Center,
                        set_focus_on_click: true,
                        set_css_classes: &["pill", "suggested-action"],
                        adw::ButtonContent {
                            set_icon_name: "drive-multidisk-symbolic",
                            #[watch]
                            set_label: "Load .pfx",
                        },
                        connect_clicked => SelectModeMsg::OpenFile,
                    },
                }
            } else {
                gtk::Box {
                    set_spacing: 20,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_margin_top: 20,
                    set_margin_bottom: 10,
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Button {
                        set_halign: gtk::Align::Center,
                        set_focus_on_click: true,
                        adw::ButtonContent {
                            set_icon_name: "drive-multidisk-symbolic",
                            #[watch]
                            set_label: "Load .pfx",
                        },
                        connect_clicked => SelectModeMsg::OpenFile
                    },

                    adw::Clamp {
                        #[name(file_list)]
                        gtk::ListBox {
                            #[watch]
                            set_selection_mode: gtk::SelectionMode::None,
                            add_css_class: "boxed-list",
                        },
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
        let sender_clone = sender.input_sender().clone();

        glib::timeout_add_seconds_local(2, move || {
            sender_clone
                .send(SelectModeMsg::RefreshCertificates)
                .unwrap();
            glib::ControlFlow::Continue
        });

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

        let mut certificate = Vec::<String>::new();

        let path = Path::new("/media/DSKEYS");
        if path.exists() {
            match get_pfx_files_in_folder("/media/DSKEYS") {
                Ok(file_names) => {
                    for file_name in file_names {
                        certificate.push(file_name.clone());
                    }
                }
                Err(e) => println!("Error in Init function eimzo::get_pfx_files_in_folder: {}", e),
            }
        }

        let mut model = SelectModePage {
            is_path_empty: certificate.is_empty(),
            certificate: certificate.clone(),
            file_list: gtk::ListBox::new(),
            open_dialog,
        };
        let widgets = view_output!();
        let file_list = widgets.file_list.clone();
        model.file_list = file_list;

        for file_name in &model.certificate {
            add_file_row_to_list(file_name, &model.file_list);
        }
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: SelectModeMsg, sender: ComponentSender<Self>) {
        match msg {
            SelectModeMsg::OpenFile => {
                // std::thread::spawn(check_path_and_perm)
                //     .join()
                //     .expect("Fucked up");

                // check_path_and_perm();
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            SelectModeMsg::OpenFileResponse(path) => {
                let copied_file = &path.file_name().unwrap().to_str().unwrap();

                match get_pfx_files_in_folder("/media/DSKEYS") {
                    Ok(file_names) => {
                        if file_names.contains(&copied_file.to_string()) {
                            // todo show dialog message that file already exists
                            ()
                        } else {
                            let _ = fs::copy(&path, format!("/media/DSKEYS/{}", copied_file));
                            let _ = sender.input(SelectModeMsg::RefreshCertificates);
                        }
                    }
                    Err(e) => println!(
                        "Error OpenFileResponse in function eimzo::get_pfx_files_in_folder: {}",
                        e
                    ),
                }
            }
            SelectModeMsg::RefreshCertificates => {
                // Clear current list
                for row in self.file_list.iter_children() {
                    self.file_list.remove(&row);
                }

                self.certificate.clear();

                match get_pfx_files_in_folder("/media/DSKEYS") {
                    Ok(file_names) => {
                        for file_name in file_names {
                            self.certificate.push(file_name.clone());
                            add_file_row_to_list(&file_name.clone(), &self.file_list);
                        }
                        self.is_path_empty = self.certificate.is_empty();
                    }
                    Err(e) => println!("Error in RefreshCertificates eimzo::get_pfx_files_in_folder: {}", e),
                }
            }
            SelectModeMsg::None => {}
        }
    }
}

fn tasks_filename_filters() -> Vec<gtk::FileFilter> {
    let filename_filter = gtk::FileFilter::default();
    filename_filter.set_name(Some("PFX (.pfx)"));
    filename_filter.add_suffix("pfx");

    vec![filename_filter]
}

fn add_file_row_to_list(file_name: &str, file_list: &gtk::ListBox) {
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    hbox.set_margin_all(12);
    hbox.set_hexpand(true);

    let icon = gtk::Image::from_icon_name("folder-symbolic");
    hbox.append(&icon);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);

    let title = gtk::Label::new(Some(file_name));
    title.set_xalign(0.0);
    title.add_css_class("title-3");

    let subtitle = gtk::Label::new(Some("/media/DSKEYS"));
    subtitle.set_xalign(0.0);
    subtitle.add_css_class("dim-label");

    vbox.append(&title);
    vbox.append(&subtitle);

    hbox.append(&vbox);
    file_list.append(&hbox);
}
