use std::{ops::Deref, rc::Rc};

use gettextrs::gettext;
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    adw::{self, prelude::*},
    gtk::{self},
    prelude::DynamicIndex,
};
use tracing::debug;

use crate::ui::{select_mode::SelectModeMsg, window::AppMsg};

pub struct ToggleServiceDialog {
    heading: String,
}

#[derive(Debug)]
pub enum ToggleServiceDialogMsg {
    SetHeading(String),
}

#[relm4::component(pub)]
impl SimpleComponent for ToggleServiceDialog {
    type Init = gtk::Window;
    type Input = ToggleServiceDialogMsg;
    type Output = AppMsg;

    view! {
        adw::AlertDialog {
            #[watch]
            set_heading: Some(&model.heading),
            add_response: ("ok", &gettext("Ok")),

            connect_response: (None, move |_dialog, response| {
                if response == "ok" {
                    debug!("User clicked ToggleServiceDialog Ok button")
                }
            })
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ToggleServiceDialog {
            heading: String::new(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ToggleServiceDialogMsg::SetHeading(heading) => self.heading = heading,
        }
    }
}

// --------------------------------------------------
pub struct RemoveCertificateDialogInit {
    pub index: DynamicIndex,
    pub file_name: String,
}

pub struct RemoveCertificateDialog;

#[relm4::component(pub)]
impl SimpleComponent for RemoveCertificateDialog {
    type Init = RemoveCertificateDialogInit;
    type Input = ();
    type Output = SelectModeMsg;

    view! {
        dialog = adw::AlertDialog {
            set_heading: Some(&gettext("Are you sure?")),
            set_body: &gettext("Do you really want to delete this certificate?"),
            add_response: ("yes", &gettext("Yes")),
            add_response: ("no", &gettext("No")),
            set_response_appearance: ("yes", adw::ResponseAppearance::Destructive),
            set_response_appearance: ("no", adw::ResponseAppearance::Suggested),
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = RemoveCertificateDialog {};
        let widgets = view_output!();
        let init = Rc::new(init);

        widgets
            .dialog
            .connect_response(None, move |_dialog, response| match response {
                "yes" => {
                    debug!("User clicked yes in SelectModeMsg::RemoveCertificates");
                    let _ = sender.output(SelectModeMsg::RemoveCertificates(
                        init.deref().index.clone(),
                        init.deref().file_name.clone(),
                    ));
                }
                "no" => {
                    debug!("User pressed no in SelectModeMsg::RemoveCertificates");
                }
                _ => (),
            });

        ComponentParts { model, widgets }
    }
}
