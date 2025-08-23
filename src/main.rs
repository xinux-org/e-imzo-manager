#[rustfmt::skip]
mod config;
mod app;
mod modals;
mod pages;
mod utils;

use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};
use gettextrs::LocaleCategory;
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk::{self, gio, glib, prelude::*},
    main_application, RelmApp,
};

use crate::config::RESOURCES_FILE;
use app::App;
use gtk::gdk;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn main() {
    gtk::init().unwrap();
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    // setup gettext
    setup_gettext();

    glib::set_application_name("E-IMZO Manager");
    gtk::Window::set_default_icon_name(APP_ID);

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = main_application();
    app.set_resource_base_path(Some("/uz/xinux/EIMZOManager/"));

    let mut actions = RelmActionGroup::<AppActionGroup>::new();

    let quit_action = {
        let app = app.clone();
        RelmAction::<QuitAction>::new_stateless(move |_| {
            app.quit();
        })
    };
    actions.add_action(quit_action);
    actions.register_for_main_application();

    app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    let provider = gtk::CssProvider::new();
    provider.load_from_path("/data/resources/style.css");
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let app = RelmApp::from_app(app);

    let data = res
        .lookup_data(
            "/uz/xinux/EIMZOManager/style.css",
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
    relm4::set_global_css(&glib::GString::from_utf8_checked(data.to_vec()).unwrap());

    app.visible_on_activate(false).run::<App>(());
}

fn setup_gettext() {
    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to bind the text domain codeset to UTF-8");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
}
