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
            .application_name("E-IMZO Manager")
            .application_icon(APP_ID)
            .license_type(gtk::License::Apache20)
            .website("https://xinux.uz/")
            .issue_url("https://github.com/xinux-org/e-imzo/issues")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright("© 2025 Xinux Developers")
            .developers(vec!["Baxrom Raxmatov (bahrom04)", "Bemeritus", "let-rec"])
            .release_notes_version("0.1.0")
            .release_notes(release_notes())
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

fn release_notes() -> &'static str {
    r#"<p>This release contains new features and fixes:</p>
    <ul>
      <li>New area selection UI</li>
      <li>Added option to about section of UI</li>
    </ul>"#
}
