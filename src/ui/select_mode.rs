use crate::ui::window::AppMsg;
use crate::utils::{
    ask_password, check_file_ownership, check_service_active, hide_sensitive_string,
    return_pfx_files_in_folder, show_alert_dialog, show_remove_file_alert_dialog,
    tasks_filename_filters,
};
use e_imzo::EIMZO;
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    component::{AsyncComponent, AsyncComponentParts, AsyncComponentSender},
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
use tracing::warn;

#[derive(Debug)]
pub struct SelectModePage {
    is_path_empty: bool,
    is_file_loaded: bool,
    open_dialog: Controller<OpenDialog>,
    file_list_factory: FactoryVecDeque<CertificateRow>,
}

#[derive(Debug)]
pub enum SelectModeMsg {
    // Open file
    OpenFile,
    OpenFileConfirmed,
    OpenFileResponse(PathBuf),
    // Alerts
    ShowMessage(String),
    ShowRemoveFileMsg(DynamicIndex, String),
    // File CRUD
    RefreshCertificates,
    SetFileLoadedState(bool),
    RemoveCertificates(DynamicIndex, String),
    // AddCertificates(CertificateRow),
    None,
}

#[relm4::component(pub, async)]
impl AsyncComponent for SelectModePage {
    type Init = ();
    type Input = SelectModeMsg;
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::ScrolledWindow {
              set_vexpand: true,
              set_hexpand: true,
              set_hscrollbar_policy: gtk::PolicyType::Never,
              set_vscrollbar_policy: gtk::PolicyType::Automatic,

              if model.is_file_loaded {
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
                                      set_label: &gettext("Load .pfx"),
                                  },
                                  connect_clicked => SelectModeMsg::OpenFile,
                              },
                          }
                      } else {
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
                  }
              } else {
                  gtk::Box {
                      set_vexpand: true,
                      set_hexpand: true,
                      set_valign: gtk::Align::Center,
                      set_halign: gtk::Align::Center,

                      adw::Spinner {
                          set_width_request: 32,
                          set_height_request: 32,
                      }
                  }
              },
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
                cancel_label: "Cancel".into(),
                accept_label: "Open".into(),
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
                        SelectModeMsg::ShowRemoveFileMsg(index.clone(), file.clone())
                    }
                });

        let model = SelectModePage {
            is_path_empty: return_pfx_files_in_folder().is_empty(),
            is_file_loaded: false,
            open_dialog,
            file_list_factory,
        };

        let allbox = model.file_list_factory.widget();
        let widgets = view_output!();

        // when app started prevent this
        if check_service_active("e-imzo.service") {
            sender.input(SelectModeMsg::RefreshCertificates);
        }

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
                if Path::new("/media/DSKEYS").exists() && check_file_ownership().unwrap() == 1000 {
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
                    sender.input(SelectModeMsg::ShowMessage(
                        gettext("File already exists. You can use it").to_string(),
                    ));
                } else {
                    let _ = fs::copy(&path, format!("/media/DSKEYS/{}", copied_file));
                    sender.input(SelectModeMsg::SetFileLoadedState(false));
                    // implement adding feature by updating e_imzo crate
                    // self.file_list_factory.guard().push_back(data);
                    sender.input(SelectModeMsg::RefreshCertificates);
                }
            }
            // Alerts
            SelectModeMsg::ShowMessage(text) => show_alert_dialog(&text),
            SelectModeMsg::ShowRemoveFileMsg(index, file_name) => {
                show_remove_file_alert_dialog(index, file_name, sender)
            }
            SelectModeMsg::RefreshCertificates => {
                // todo: create getting spesific file from e_imzo
                // instead of list_all_certificates. It saves much time
                // creates new list of PreferenceGroup elements when new file added
                self.file_list_factory.guard().clear();

                let pfx_files_in_folder = return_pfx_files_in_folder();
                
                // wait enough to wait e-imzo.service activation
                tokio::time::sleep(Duration::from_millis(1600)).await;

                // Hmm..., When service active and user launches app then toggle button
                // changes from gray to green. If user press toggle button in
                // grey color stage which is NOT YET connected to e_imzo sdk then EIMZO
                // returns unnessary error saying “Connection refused”. Why press grey 
                // button before 1600 mileseconds because user wants deactivate service 
                // very fast when app launched
                EIMZO::new().map_or_else(
                    |e| warn!("No connection because service is stopped: {e:?}"),
                    |mut res| {
                        res.list_all_certificates().map_or_else(
                            |e| warn!("Connection not yet established: {e:?}"),
                            |cer| {
                                cer.iter()
                                    .map(|c| (c, c.get_alias()))
                                    .for_each(|(c, alias)| {
                                        let file_name = c.name.clone();

                                        // convert string "2027.07.23 17:44:06" into "23.07.2027"
                                        let validfrom: Vec<_> =
                                            alias.get("validfrom").unwrap().split(" ").collect();
                                        let mut validfrom_dmy: Vec<_> =
                                            validfrom[0].split(".").collect();
                                        validfrom_dmy.reverse();

                                        let validto: Vec<_> =
                                            alias.get("validto").unwrap().split(" ").collect();
                                        let mut validto_dmy: Vec<_> =
                                            validto[0].split(".").collect();
                                        validto_dmy.reverse();

                                        let full_name = format!(
                                            "Full name: {}",
                                            alias
                                                .get("cn")
                                                .expect("Full name not found")
                                                .to_uppercase()
                                        );
                                        let serial_number = format!(
                                            "Seriya raqami: {}",
                                            alias
                                                .get("serialnumber")
                                                .expect("Serial number not found")
                                        );
                                        let name: String = hide_sensitive_string(
                                            alias.get("name").unwrap().clone(),
                                            '*',
                                            2,
                                        );
                                        let surname = hide_sensitive_string(
                                            alias.get("surname").unwrap().clone(),
                                            '*',
                                            2,
                                        );
                                        let title = format!("{} {}", name, surname).to_uppercase();
                                        let validity = format!(
                                            "Sertifikat amal qilish muddati: {} - {}",
                                            validfrom_dmy.join("."),
                                            validto_dmy.join(".")
                                        );

                                        let data = CertificateRow {
                                            title: title.to_owned(),
                                            file_name: file_name.to_owned(),
                                            full_name_line: full_name.to_owned(),
                                            serial_number_line: serial_number.to_owned(),
                                            validity_line: validity,
                                        };
                                        self.file_list_factory.guard().push_back(data);
                                    });
                                // after successfully  load, remove spinner
                                self.is_file_loaded = true;
                            },
                        )
                    },
                );
                // after removeing spinner check files in /media/DSKEYS exists or empty
                self.is_path_empty = pfx_files_in_folder.is_empty();
            }
            SelectModeMsg::SetFileLoadedState(is_loaded) => {
                self.is_file_loaded = is_loaded;
            }

            SelectModeMsg::RemoveCertificates(index, file_name) => {
                let full_path = Path::new("/media/DSKEYS/").join(format!("{}.pfx", file_name));
                if let Err(e) = fs::remove_file(&full_path) {
                    eprintln!("failed {}: {}", full_path.display(), e);
                } else {
                    self.file_list_factory.guard().remove(index.current_index());
                    if self.file_list_factory.is_empty() && return_pfx_files_in_folder().is_empty()
                    {
                        self.is_path_empty = true;
                    }
                    println!("deleted: {}", full_path.display());
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
    // type ParentInput = SelectModeMsg;

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
                add_suffix = &gtk::Button {
                    set_icon_name: "user-trash-symbolic",
                    add_css_class: "destructive-action",
                    set_valign: gtk::Align::Center,

                    connect_clicked[sender, index, file_name = self.file_name.clone()] => move |_| {
                        sender.output(CertificateRowOutput::RemoveRequested(index.clone(), file_name.clone())).unwrap()
                    },
                },
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }
}
