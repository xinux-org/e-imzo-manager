// use gettextrs::gettext;
use gio::AppInfo;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Localhost;

#[relm4::component(pub)]
impl SimpleComponent for Localhost {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Box{}
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();
        // let window = relm4::main_application().active_window();
        // root.present(window.as_ref());

        if let Err(err) = AppInfo::launch_default_for_uri(
            "https://127.0.0.1:64443/",
            None::<&gio::AppLaunchContext>,
        ) {
            eprintln!("Failed to open URL: {}", err);
        }
        ComponentParts { model, widgets }
    }
}
