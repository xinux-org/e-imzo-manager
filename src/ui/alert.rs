use gettextrs::gettext;
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    adw::{self, prelude::*},
    gtk::{self},
};

use crate::ui::window::AppMsg;

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
                    tracing::info!("User clicked ToggleServiceDialog Ok button")
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

pub struct RemoveCertificateDialog {
    heading: String,
    body: String,
}

#[derive(Debug)]
pub enum RemoveCertificateDialogMsg {
    SetHeading(String),
    SetBody(String),
}

#[relm4::component(pub)]
impl SimpleComponent for RemoveCertificateDialog {
    type Init = gtk::Window;
    type Input = ToggleServiceDialogMsg;
    type Output = AppMsg;

    view! {
        adw::AlertDialog {
            #[watch]
            set_heading: Some(&model.heading),
            #[watch]
            set_body: &model.body,
            add_response: ("ok", &gettext("Ok")),

            connect_response: (None, move |_dialog, response| {
                if response == "ok" {
                    tracing::info!("User clicked ToggleServiceDialog Ok button")
                }
                // } else if response "" {

                // }
            })
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = RemoveCertificateDialog {
            heading: String::new(),
            body: String::new(),
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
// pub fn show_remove_file_alert_dialog(
//     index: DynamicIndex,
//     file_name: String,
//     sender: AsyncComponentSender<SelectModePage>,
// ) {
//     let dialog = adw::AlertDialog::builder()
//         .heading(gettext("Are you sure?"))
//         .body(gettext("Do you really want to delete this certificate?"))
//         .build();

//     dialog.add_responses(&[("yes", &gettext("Yes")), ("no", &gettext("No"))]);
//     dialog.set_default_response(Some("no"));

//     dialog.set_response_appearance("yes", adw::ResponseAppearance::Destructive);
//     dialog.set_response_appearance("no", adw::ResponseAppearance::Suggested);

//     dialog.connect_response(None, {
//         let sender = sender.clone();
//         let file_name = file_name.clone();
//         move |dialog, response| {
//             match response {
//                 "yes" => {
//                     sender.input(SelectModeMsg::RemoveCertificates(
//                         index.clone(),
//                         file_name.clone(),
//                     ));
//                 }
//                 "no" => {
//                     // sender.input(SelectModeMsg::SetFileLoadedState(false));
//                     // sender.input(SelectModeMsg::RefreshCertificates);
//                     info!("User pressed no");
//                     sender.input(SelectModeMsg::None);
//                 }
//                 _ => {}
//             }
//             dialog.close();
//         }
//     });
//     if let Some(win) = relm4::main_application().active_window() {
//         dialog.present(Some(&win));
//     }
// }
