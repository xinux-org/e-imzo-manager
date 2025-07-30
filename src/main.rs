#[rustfmt::skip]
mod config;
mod app;
mod modals;
mod pages;

use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};
use gettextrs::{gettext, LocaleCategory};
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk::{self, gio, glib, prelude::*},
    main_application, RelmApp,
};

use app::App;
use std::process::Command;

use crate::config::RESOURCES_FILE;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn is_service_active(service_name: &str) -> Result<bool, String> {
    let output = Command::new("systemctl")
        .args(&["--user", "is-active", service_name])
        .output()
        .map_err(|e| format!("Failed to run systemctl: {}", e))?;

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

    match status.as_str() {
        "active" => Ok(true),
        "inactive" | "failed" | "activating" | "deactivating" | "unknown" => Ok(false),
        _ => Err(format!("Unexpected status: {}", status)),
    }
}

fn main() {
    gtk::init().unwrap();
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    // setup gettext
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    gtk::Window::set_default_icon_name(APP_ID);

    let app = main_application();
    app.set_resource_base_path(Some("/com/belmoussaoui/GtkRustTemplate/"));

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
    // app.set_accelerators_for_action::<AwesomeAction>(&["<Control>q"]);

    let app = RelmApp::from_app(app);

    let data = res
        .lookup_data(
            "/com/belmoussaoui/GtkRustTemplate/style.css",
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
    relm4::set_global_css(&glib::GString::from_utf8_checked(data.to_vec()).unwrap());

    app.visible_on_activate(false).run::<App>(());
}

