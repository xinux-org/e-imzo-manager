use crate::{
    config::MEDIA_DSKEYS,
    ui::{
        alert::{RemoveCertificateDialog, RemoveCertificateDialogInit},
        window::AppMsg,
    },
    utils::{ask_password, check_keys_ownership, is_service_active, get_pfx_files_in_folder},
};
use e_imzo::{EIMZO, prelude::Certificate};
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
use tracing::debug;

#[derive(Debug)]
pub struct SelectModePage {
    open_dialog: Controller<OpenDialog>,
    file_list_factory: FactoryVecDeque<CertificateRow>,
    stack: SelectModeStack,
}

impl SelectModePage {
    // file selection filter .pfx file
    pub fn tasks_filename_filters() -> Vec<gtk::FileFilter> {
        let filename_filter = gtk::FileFilter::default();
        filename_filter.set_name(Some("PFX (.pfx)"));
        filename_filter.add_suffix("pfx");

        vec![filename_filter]
    }
    pub fn certificate_rows(&self, certs: Vec<Certificate>) -> Vec<CertificateRow> {
        certs
            .into_iter()
            .filter_map(|c| {
                let mut alias = c.get_alias();
                let full_name_line = format!(
                    "{}: {}",
                    gettext("Full name"),
                    alias.get("cn")?.to_uppercase()
                );
                let serial_number = format!(
                    "{}: {}",
                    gettext("Certificate number"),
                    alias.get("serialnumber")?
                );
                // check time output yourself if you arenʻt sure
                // from "23.07.2027 11:11:11" to this "23.07.2027"
                let validity = format!(
                    "{}: {} - {}",
                    gettext("Certificate validity period"),
                    c.valid_from?.format("%d.%m.%Y"),
                    c.valid_to?.format("%d.%m.%Y")
                );
                Some(CertificateRow {
                    name: alias.remove("name"),
                    surname: alias.remove("surname"),
                    file_name: c.name,
                    full_name_line,
                    serial_number_line: serial_number,
                    validity_line: validity,
                    is_expired: c.is_expired?,
                })
            })
            .collect::<Vec<CertificateRow>>()
    }
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
    CertificatesLoaded(Vec<Certificate>),
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
impl AsyncComponent for SelectModePage {
    type Init = ();
    type Input = SelectModeMsg;
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
      gtk::ScrolledWindow {
          set_vexpand: true,
          set_hexpand: true,
          set_hscrollbar_policy: gtk::PolicyType::Never,
          set_vscrollbar_policy: gtk::PolicyType::Automatic,
          gtk::Stack {
              set_transition_type: gtk::StackTransitionType::Crossfade,
              #[watch]
              set_visible_child_name: match model.stack {
                  SelectModeStack::Empty => "empty",
                  SelectModeStack::NotEmpty => "not_empty",
                  SelectModeStack::Loading => "loading",
              },

              add_child = &adw::StatusPage {
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
              } -> { set_name: "empty", },

              add_child = &gtk::Box {
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
                  #[local_ref]
                  allbox -> adw::PreferencesGroup {},
                  
              } -> { set_name: "not_empty", },

              add_child = &gtk::Box {
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
              } -> { set_name: "loading", }
          }
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
                filters: Self::tasks_filename_filters(),
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
            stack: if get_pfx_files_in_folder().is_ok_and(|cers| cers.is_empty()) {
                SelectModeStack::Empty
            } else {
                SelectModeStack::NotEmpty
            },
        };

        let allbox = model.file_list_factory.widget();
        // when app started prevent this
        if is_service_active() {
            sender.input(SelectModeMsg::SetFileLoadedState(SelectModeStack::Loading));
            sender.input(SelectModeMsg::RefreshCertificates);
        }

        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: SelectModeMsg,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            SelectModeMsg::OpenFile => {
                if Path::new(MEDIA_DSKEYS).exists() && check_keys_ownership().unwrap() == 1000 {
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

                if get_pfx_files_in_folder()
                    .is_ok_and(|certs| certs.contains(&copied_file.to_string()))
                {
                    sender.output_sender().emit(AppMsg::ShowMessage(gettext(
                        "File already exists. You can use it",
                    )));
                } else {
                    // Copy lesected file to e-imzo path with fileʻs name
                    fs::copy(&path, format!("{}/{}", MEDIA_DSKEYS, copied_file));
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
                tokio::time::sleep(Duration::from_millis(2000)).await;

                // Hmm..., When service active and user launches app then toggle button
                // changes from gray to green. If user press toggle button in
                // grey color stage which is NOT YET connected to e_imzo sdk then EIMZO
                // returns unnessary error saying “Connection refused”. Why press grey
                // button before 1600 mileseconds because user wants deactivate service
                // very fast when app launched
                relm4::spawn(async move {
                    let mut eimzo = match EIMZO::new() {
                        Ok(eimzo) => eimzo,
                        Err(error) => {
                            sender
                                .output(AppMsg::ShowMessage(format!("EIMZO connection: {error}")));
                            return;
                        }
                    };
                    let certs = match eimzo.list_all_certificates() {
                        Ok(certs) => certs,
                        Err(error) => {
                            sender.output(AppMsg::ShowMessage(format!(
                                "list_all_certificates: {error}"
                            )));
                            vec![]
                        }
                    };
                    sender.input(SelectModeMsg::CertificatesLoaded(certs));
                });
            }
            SelectModeMsg::CertificatesLoaded(certs) => {
                let rows = self.certificate_rows(certs);

                for row in rows {
                    self.file_list_factory.guard().push_back(row);
                }
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

                if fs::remove_file(&full_path).is_ok() {
                    self.file_list_factory.guard().remove(index.current_index());
                    debug!("deleted: {}", full_path.display());
                    if self.file_list_factory.is_empty() {
                        sender
                            .input_sender()
                            .emit(SelectModeMsg::SetFileLoadedState(SelectModeStack::Empty));
                    }
                } else {
                    eprintln!("failed {}", full_path.display());
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
    pub name: Option<String>,
    pub surname: Option<String>,
    pub file_name: String,
    pub full_name_line: String,
    pub serial_number_line: String,
    pub validity_line: String,
    pub is_expired: bool,
}

impl CertificateRow {
    // Example full name: Jo**** Do****
    fn hidden_full_name(&self) -> String {
        match (self.name.clone(), self.surname.clone()) {
            (Some(name), Some(surname)) => {
                let hidden_name = self.hide_sensitive_string(name.to_owned(), '*', 2);
                let hidden_surname = self.hide_sensitive_string(surname.to_owned(), '*', 2);
                (hidden_name + &hidden_surname).to_uppercase()
            }
            _ => gettext("Name not found").to_owned(),
        }
    }
    fn hide_sensitive_string(&self, name: String, symbol: char, range: usize) -> String {
        name.chars()
            .enumerate()
            .map(|(i, c)| if i <= range { c } else { symbol })
            .collect()
    }
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
            set_title: &self.hidden_full_name(),
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
                        sender.output_sender().emit(CertificateRowOutput::RemoveRequested(index.to_owned(), file_name.to_owned()))
                    },
                },

            },
        }
    }
    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            name: init.name,
            surname: init.surname,
            file_name: init.file_name,
            full_name_line: init.full_name_line,
            serial_number_line: init.serial_number_line,
            validity_line: init.validity_line,
            is_expired: init.is_expired,
        }
    }
}
