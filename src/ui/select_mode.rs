use crate::ui::alert::{RemoveCertificateDialog, RemoveCertificateDialogInit};
use crate::ui::window::AppMsg;
use crate::utils::{
    ask_password, check_file_ownership, check_service_active, hide_sensitive_string,
    return_pfx_files_in_folder, tasks_filename_filters,
};
use e_imzo::EIMZO;
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    component::{AsyncComponentParts, AsyncComponentSender},
    factory::*,
    gtk::{self},
    prelude::*,
    *,
};
use relm4_components::open_dialog::*;
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};
use tracing::{debug, warn};

const MEDIA_DSKEYS: &str = "/media/DSKEYS";

#[derive(Debug)]
pub struct SelectModePage {
    open_dialog: Controller<OpenDialog>,
    file_list_factory: FactoryVecDeque<CertificateRow>,
    stack: SelectModeStack,
}

#[derive(Debug)]
pub enum SelectModeMsg {
    // Open file
    OpenFile,
    OpenFileConfirmed,
    OpenFileResponse(PathBuf),
    // Alerts
    ShowRemoveFileMsg(DynamicIndex, String),
    // File CRUD
    RefreshCertificates,
    SetFileLoadedState(SelectModeStack),
    RemoveCertificates(DynamicIndex, String),
    // AddCertificates(CertificateRow),
    None,
}

#[derive(Debug)]
pub enum SelectModeStack {
    Empty,
    NotEmpty,
    Loading,
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for SelectModePage {
    type Init = ();
    type Input = SelectModeMsg;
    type Output = AppMsg;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_hexpand: true,
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_vscrollbar_policy: gtk::PolicyType::Automatic,

                #[transition(Crossfade)]
                match model.stack {
                    SelectModeStack::Empty => {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
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
                                        set_label: &gettext("Load .pfx"),
                                    },
                                    connect_clicked => SelectModeMsg::OpenFile,
                                },
                            }
                        }
                    },
                    SelectModeStack::NotEmpty => {
                        gtk::Box {
                            gtk::Label {
                                add_css_class: relm4::css::TITLE_2,
                                #[watch]
                                set_label: &gettext("Loaded keys"),
                                set_margin_all: 1,
                            },
                            set_spacing: 20,
                            set_margin_start: 10,
                            set_margin_end: 10,
                            set_margin_top: 20,
                            set_margin_bottom: 10,
                            set_orientation: gtk::Orientation::Vertical,
                            set_halign: gtk::Align::Center,
                            adw::Clamp {
                                #[local_ref]
                                allbox -> adw::PreferencesGroup {}
                            }
                        }
                    },
                    SelectModeStack::Loading => {
                        gtk::Box {
                            set_vexpand: true,
                            set_hexpand: true,
                            set_valign: gtk::Align::Center,
                            set_halign: gtk::Align::Center,
                            set_orientation: gtk::Orientation::Vertical,

                            adw::Spinner {
                                set_width_request: 40,
                                set_height_request: 40,
                                set_margin_bottom: 25,
                            },

                            gtk::Label {
                                set_label: &gettext("Loading keys"),
                                add_css_class: relm4::css::TITLE_2,
                            },
                        }
                    }
                }
            },
        },
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(OpenDialogSettings {
                create_folders: false,
                folder_mode: false,
                cancel_label: gettext("Cancel"),
                accept_label: gettext("Open"),
                is_modal: true,
                filters: tasks_filename_filters(),
            })
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => SelectModeMsg::OpenFileResponse(path),
                OpenDialogResponse::Cancel => SelectModeMsg::None,
            });

        let file_list_factory =
            FactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), |msg| match msg {
                    CertificateRowOutput::RemoveRequested(index, file) => {
                        SelectModeMsg::ShowRemoveFileMsg(index, file)
                    }
                });

        let model = SelectModePage {
            open_dialog,
            file_list_factory,
            stack: if return_pfx_files_in_folder().is_empty() {
                SelectModeStack::Empty
            } else {
                SelectModeStack::NotEmpty
            },
        };

        let allbox = model.file_list_factory.widget();
        // when app started prevent this
        if check_service_active("e-imzo.service") {
            sender.input(SelectModeMsg::SetFileLoadedState(SelectModeStack::Loading));
            sender.input(SelectModeMsg::RefreshCertificates);
        }

        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: SelectModeMsg, sender: AsyncComponentSender<Self>) {
        match msg {
            SelectModeMsg::OpenFile => {
                if Path::new(MEDIA_DSKEYS).exists() && check_file_ownership().unwrap() == 1000 {
                    self.open_dialog.emit(OpenDialogMsg::Open);
                } else {
                    ask_password(sender);
                }
            }
            SelectModeMsg::OpenFileConfirmed => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            SelectModeMsg::OpenFileResponse(path) => {
                let copied_file = &path.file_name().unwrap().to_str().unwrap();

                if return_pfx_files_in_folder().contains(&copied_file.to_string()) {
                    let _ = sender.output(AppMsg::ShowMessage(gettext(
                        "File already exists. You can use it",
                    )));
                } else {
                    // Copy lesected file to e-imzo path with fileʻs name
                    let _ = fs::copy(&path, format!("{}/{}", MEDIA_DSKEYS, copied_file));
                    sender.input(SelectModeMsg::SetFileLoadedState(SelectModeStack::Loading));
                    // implement adding feature by updating e_imzo crate
                    // self.file_list_factory.guard().push_back(data);
                    sender.input(SelectModeMsg::RefreshCertificates);
                }
            }
            // Alerts
            SelectModeMsg::ShowRemoveFileMsg(index, file_name) => {
                let dialog = RemoveCertificateDialog::builder()
                    .launch(RemoveCertificateDialogInit { index, file_name })
                    .forward(sender.input_sender(), |msg| msg);

                dialog
                    .widget()
                    .present(relm4::main_application().active_window().as_ref());
            }
            SelectModeMsg::RefreshCertificates => {
                // todo: create getting spesific file from e_imzo
                // instead of list_all_certificates. It saves much time
                // creates new list of PreferenceGroup elements when new file added
                self.file_list_factory.guard().clear();

                // wait enough to wait e-imzo.service activation
                tokio::time::sleep(Duration::from_millis(1800)).await;

                // Hmm..., When service active and user launches app then toggle button
                // changes from gray to green. If user press toggle button in
                // grey color stage which is NOT YET connected to e_imzo sdk then EIMZO
                // returns unnessary error saying “Connection refused”. Why press grey
                // button before 1600 mileseconds because user wants deactivate service
                // very fast when app launched
                let mut eimzo = match EIMZO::new() {
                    Ok(eimzo) => eimzo,
                    Err(e) => {
                        warn!("No connection because service is stopped: {e:?}");
                        return;
                    }
                };
                if let Ok(certs) = eimzo.list_all_certificates() {
                    let row = certs.iter().filter_map(|c| {
                        let file_name = &c.name;
                        let alias = c.get_alias();

                        // coming time format from certificate: "23.07.2027"
                        let validfrom = c.valid_from?;
                        let validto = c.valid_to?;
                        let is_expired = c.is_expired?;

                        let full_name = format!(
                            "{}: {}",
                            gettext("Full name"),
                            alias.get("cn")?.to_uppercase()
                        );
                        let serial_number = format!(
                            "{}: {}",
                            gettext("Certificate number"),
                            alias.get("serialnumber")?
                        );
                        let name: String =
                            hide_sensitive_string(alias.get("name")?.to_owned(), '*', 2);
                        let surname =
                            hide_sensitive_string(alias.get("surname")?.to_owned(), '*', 2);
                        let title = format!("{} {}", name, surname).to_uppercase();
                        let validity = format!(
                            "{}: {} - {}",
                            gettext("Certificate validity period"),
                            validfrom.format("%d.%m.%Y"),
                            validto.format("%d.%m.%Y")
                        );

                        Some(CertificateRow {
                            title,
                            file_name: file_name.to_owned(),
                            full_name_line: full_name,
                            serial_number_line: serial_number,
                            validity_line: validity,
                            is_expired,
                        })
                    });
                    self.file_list_factory.extend(row);
                }
                // after removing spinner check files in /media/DSKEYS exists or empty
                if self.file_list_factory.is_empty() {
                    sender
                        .input_sender()
                        .emit(SelectModeMsg::SetFileLoadedState(SelectModeStack::Empty));
                } else {
                    sender
                        .input_sender()
                        .emit(SelectModeMsg::SetFileLoadedState(SelectModeStack::NotEmpty));
                }
            }
            SelectModeMsg::SetFileLoadedState(stack) => {
                self.stack = stack;
            }
            SelectModeMsg::RemoveCertificates(index, file_name) => {
                debug!("REMOVE CESTSRSTSRTRSTRS");
                let full_path = Path::new(MEDIA_DSKEYS).join(format!("{}.pfx", file_name));

                match fs::remove_file(&full_path) {
                    Ok(()) => {
                        self.file_list_factory.guard().remove(index.current_index());
                        debug!("deleted: {}", full_path.display());
                        if self.file_list_factory.is_empty() {
                            sender
                                .input_sender()
                                .emit(SelectModeMsg::SetFileLoadedState(SelectModeStack::Empty));
                        }
                    }
                    Err(e) => {
                        eprintln!("failed {}: {}", full_path.display(), e);
                    }
                }
            }
            // todo.
            // SelectMode::AddCertificates(file) => {
            //   // self.file_list_factory.guard().
            //     // (self.file_list_factory.guard().push_back(file))
            // }

            // when user cancels file selection or deletion do nothing
            SelectModeMsg::None => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct CertificateRow {
    pub title: String,
    pub file_name: String,
    pub full_name_line: String,
    pub serial_number_line: String,
    pub validity_line: String,
    pub is_expired: bool,
}

#[derive(Debug)]
pub enum CertificateRowOutput {
    RemoveRequested(DynamicIndex, String),
}

#[relm4::factory(pub)]
impl FactoryComponent for CertificateRow {
    type Init = CertificateRow;
    type Input = ();
    type Output = CertificateRowOutput;
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    #[root]
    view! {
        adw::ExpanderRow {
            set_use_markup: true,
            set_title: &self.title,


            add_row = &adw::ActionRow {
                set_title: &self.full_name_line,
            },

            add_row = &adw::ActionRow {
                set_title: &self.serial_number_line,
            },

            add_row = &adw::ActionRow {
                set_title: &self.validity_line,
            },

            add_row = &adw::ActionRow {
                add_prefix = &gtk::Label{
                    add_css_class: if self.is_expired {"warning-badge"} else {"success-badge"},
                    set_label: &if self.is_expired { gettext("Expired")} else { gettext("Active")},
                    set_valign: gtk::Align::Center,
                },

                add_suffix = &gtk::Button {
                    set_icon_name: "user-trash-symbolic",
                    add_css_class: "destructive-action",
                    set_valign: gtk::Align::Center,

                    connect_clicked[sender, index, file_name = self.file_name.to_owned()] => move |_| {
                        sender.output(CertificateRowOutput::RemoveRequested(index.to_owned(), file_name.to_owned())).unwrap()
                    },
                },

            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }
}
