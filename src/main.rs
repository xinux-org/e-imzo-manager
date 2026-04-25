#[rustfmt::skip]
mod config;
mod ui;
mod utils;

use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};
use gettextrs::{LocaleCategory, gettext};
use relm4::{
    RelmApp,
    gtk::{self, gio, glib, prelude::*},
    main_application,
};
use crate::config::RESOURCES_FILE;
use ui::window::App;

fn main() {
    gtk::init().unwrap();
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    // setup gettext
    setup_gettext();

    glib::set_application_name(&gettext("e-imzo-manager"));
    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    gtk::Window::set_default_icon_name(APP_ID);

    let app = main_application();
    app.set_resource_base_path(Some("/uz/xinux/EIMZOManager/"));

    let data = res
        .lookup_data(
            "/uz/xinux/EIMZOManager/style.css",
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
    relm4::set_global_css(&glib::GString::from_utf8_checked(data.to_vec()).unwrap());

    let app = RelmApp::from_app(app);
    app.visible_on_activate(true).run::<App>(());
}

fn setup_gettext() {
    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to bind the text domain codeset to UTF-8");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
}
