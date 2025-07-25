use relm4::{
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
use std::{convert::identity, path::PathBuf};

use crate::app::AppMsg;
use crate::modals::document::{Document, DocumentInput};

pub struct SelectModePage {
    document: Controller<Document>,
    open_dialog: Controller<OpenDialog>,
}

#[derive(Debug)]
pub enum SelectModeMsg {
    OpenFile,
    OpenFileResponse(PathBuf),
    None,
}

#[relm4::component(pub)]
impl SimpleComponent for SelectModePage {
    type Init = ();
    type Input = SelectModeMsg;
    type Output = AppMsg;
    type Widgets = AppWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            set_hexpand: true,

               gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_spacing: 10,
                    set_margin_all: 10,

                    gtk::Button {
                        set_label: "Open .pfx",
                        connect_clicked => SelectModeMsg::OpenFile,
                    }
                },
        }
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

        let model = SelectModePage {
            document,
            open_dialog,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: SelectModeMsg, _sender: ComponentSender<Self>) {
        match msg {
            SelectModeMsg::OpenFile => {
                self.open_dialog.emit(OpenDialogMsg::Open);
            }
            SelectModeMsg::OpenFileResponse(path) => {
                self.document.emit(DocumentInput::Open(path));
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
