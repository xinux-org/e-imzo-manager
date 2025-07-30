use adw::prelude::AdwDialogExt;
use gtk::prelude::GtkApplicationExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config::{APP_ID, VERSION};

pub struct AboutDialog {}

impl SimpleComponent for AboutDialog {
    type Init = ();
    type Widgets = adw::AboutDialog;
    type Input = ();
    type Output = ();
    type Root = adw::AboutDialog;

    fn init_root() -> Self::Root {
        adw::AboutDialog::builder()
            .application_icon(APP_ID)
            // Insert your license of choice here
            // .license_type(gtk::License::MitX11)
            .website("https://e-imzo.soliq.uz/")
            .issue_url("https://github.com/xinux-org/e-imzo/issues")
            .application_name("E-IMZO Manager")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright("© 2025 Xinux Developers")
            .developers(vec![
				"Baxrom Raxmatov bahrom04",
                "Bemeritus",
                "let-rec"
			])
            .build()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();
        widgets.present(Some(&relm4::main_application().windows()[0]));

        ComponentParts { model, widgets }
    }

    fn update_view(&self, _dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
