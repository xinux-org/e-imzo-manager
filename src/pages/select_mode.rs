use relm4::{
    adw,
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
    },
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent,
};
use relm4_components::open_dialog::{
    OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings,
};
use std::{collections::HashMap, convert::identity, path::PathBuf};

use crate::app::AppMsg;
use crate::modals::document::{Document, DocumentInput};
use eimzo::get_pfx_files_in_folder;

pub struct SelectModePage {
    is_path_empty: bool,
    certificate: Vec<String>,
    file_list: gtk::ListBox, 
    document: Controller<Document>,
    open_dialog: Controller<OpenDialog>,
}

#[derive(Debug)]
pub enum SelectModeMsg {
    OpenFile,
    OpenFileResponse(PathBuf),
    SaveFile(String),
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
                        connect_clicked => SelectModeMsg::OpenFile,
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
        let document = Document::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

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

        match get_pfx_files_in_folder("/media/DSKEYS") {
            Ok(file_names) => {
                for file_name in file_names {
                    println!(
                        "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
                    );
                    println!("{:?}", file_name);
                    println!(
                        "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
                    );
                    certificate.push(file_name.clone());
                }
            }
            Err(e) => println!("Error in function eimzo::get_pfx_files_in_folder: {}", e),
        }

        let mut model = SelectModePage {
            is_path_empty: std::path::PathBuf::from("/media/DSKEYS")
                .read_dir()
                .expect("REASON")
                .next()
                .is_none(),
            certificate: certificate.clone(),
            file_list: gtk::ListBox::new(), 
            document,
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

    fn update(&mut self, msg: SelectModeMsg, _sender: ComponentSender<Self>) {
        match msg {
            SelectModeMsg::OpenFile => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            SelectModeMsg::OpenFileResponse(path) => {
                self.document.emit(DocumentInput::Save(path.clone()));
            }
            SelectModeMsg::SaveFile(path) => {
                self.certificate.push(path.to_string());
                self.is_path_empty = false;
                add_file_row_to_list(&path.clone(), &self.file_list);
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

