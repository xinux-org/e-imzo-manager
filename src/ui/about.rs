use adw::prelude::AdwDialogExt;
use gettextrs::gettext;
use gtk::prelude::GtkApplicationExt;
use relm4::{ComponentParts, ComponentSender, SimpleComponent, adw, gtk};

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
            .application_name(gettext("E-IMZO Manager"))
            .application_icon(APP_ID)
            .license_type(gtk::License::Apache20)
            .website("https://xinux.uz/")
            .issue_url("https://github.com/xinux-org/e-imzo/issues")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright(gettext("© 2026 Xinux Developers"))
            .developers(vec![
                gettext("Contributors https://git.oss.uzinfocom.uz/xinux/e-imzo-manager/activity/contributors"),
            ])
            .release_notes_version(VERSION)
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

fn release_notes() -> String {
    gettext(
        r#"<p>This release contains following changes</p>
    <ul>
        <li>Updated GNOME runtime to 50</li>
        <li>Display error on dialog when occured</li>
        <li>Added transition animation with loading spinner</li>
        <li>Support opening pkcs12 files from file manager: #10</li>
    </ul>"#,
    )
}
