use crate::{
    action_register,
    config::{APP_ID, PROFILE},
    shortcut_register, shortcut_register_ws,
    ui::{
        about::AboutDialog,
        awesome::AwesomeModel,
        localhost::Localhost,
        select_mode::{SelectModeMsg, SelectModePage},
        shortcuts::{Shortcut, ShortcutsDialog, ShortcutsDialogInit},
        welcome::WelcomeModel,
    },
    utils::{check_service_active, check_service_installed, show_alert_dialog},
};
use gettextrs::gettext;
use relm4::{actions::AccelsPlus, component::AsyncComponent};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw::{self, prelude::*},
    gtk::{self, gio, glib},
    main_application,
    prelude::AsyncComponentController,
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
};
use std::{convert::identity, time::Duration};

#[derive(Debug, Clone)]
pub enum Page {
    Welcome,
    SelectMode,
}

pub struct App {
    page: Page,
    welcome_page: Controller<WelcomeModel>,
    select_mode_page: relm4::prelude::AsyncController<SelectModePage>,
    service_active: bool,
    service_installed: bool,
    service: gtk::Button,
    service_limiter: bool,
    tooltip_text: String,
}

#[derive(Debug)]
pub enum AppMsg {
    Quit,
    SelectMode(SelectModeMsg),
    StartAndStopService,
    RefreshService(bool),
    ShowMessage(String),
    ServiceLimiter(bool),
}

relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(AwesomeAction, WindowActionGroup, "awesome");
relm4::new_stateless_action!(pub ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(LocalhostAction, WindowActionGroup, "localhost");
relm4::new_stateless_action!(
    StartAndStopServiceAction,
    WindowActionGroup,
    "start-and-stop-service"
);
relm4::new_stateless_action!(QuitAction, WindowActionGroup, "quit");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                &gettext("Awesome E-IMZO") => AwesomeAction,
                &gettext("Open https://127.0.0.1:64443/") => LocalhostAction,
                &gettext("Keyboard") => ShortcutsAction,
                &gettext("About E-IMZO Manager") => AboutAction,
            }
        }
    }
    view! {
        #[root]
        main_window = adw::ApplicationWindow::new(&main_application()) {
            // set_visible: true,
            set_size_request: (360, 550),
            set_default_size: (400, 600),

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
            } else {
                None
            },
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand: true,
                set_hexpand: true,

                adw::HeaderBar {
                    pack_start = &gtk::Button {
                        set_tooltip_text: Some(&gettext("Add key")),
                        set_icon_name: "list-add-symbolic",
                        add_css_class: "flat",
                        connect_clicked => AppMsg::SelectMode(SelectModeMsg::OpenFile),
                        #[watch]
                        set_visible: matches!(model.page, Page::SelectMode),
                    },

                    #[name(service)]
                    pack_start = &gtk::Button {
                        #[watch]
                        set_tooltip_text: Some(&model.tooltip_text),
                        #[watch]
                        set_visible: model.service_installed,
                        add_css_class: "service-button",
                        connect_clicked => AppMsg::StartAndStopService,
                        #[watch]
                        set_sensitive: !model.service_limiter,
                    },

                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                #[transition(SlideLeftRight)]
                match model.page {
                    Page::Welcome => gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_vexpand: true,
                        set_hexpand: true,
                        append: model.welcome_page.widget()
                    },
                    Page::SelectMode => gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_vexpand: true,
                        set_hexpand: true,
                        append: model.select_mode_page.widget()
                    },
                },
            },
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let welcome_page = WelcomeModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let select_mode_page = SelectModePage::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let service_active = check_service_active("e-imzo.service");
        let page: Page = if service_active {
            Page::SelectMode
        } else {
            Page::Welcome
        };

        let tooltip_text = if service_active {
            gettext("Service is ON - Click to stop")
        } else {
            gettext("Service is OFF - Click to start")
        };

        let mut model = Self {
            page,
            welcome_page,
            select_mode_page,
            service_active,
            service_installed: check_service_installed("/etc/systemd/user/e-imzo.service"),
            service: gtk::Button::new(),
            service_limiter: false,
            tooltip_text,
        };

        let widgets = view_output!();
        let service = widgets.service.clone();
        model.service = service;
        widgets.load_window_size();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();
        let app = root.application().unwrap();
        let mut shortcuts = vec![];


        shortcut_register_ws!(
          (app, shortcuts, actions, sender),
          gettext("Quit") => "<Control>q",
          QuitAction => AppMsg::Quit
        );

        shortcut_register_ws!(
            (app, shortcuts, actions, sender),
            gettext("Start/Stop Service") => "<Control>r",
            StartAndStopServiceAction => AppMsg::StartAndStopService
        );
        shortcut_register!(
            (app, shortcuts, actions),
            gettext("Awesome E-IMZO") => "<Control>a",
            AwesomeAction => { AwesomeModel::builder().launch(()).detach(); }
        );

        action_register!(actions, ShortcutsAction => {
          ShortcutsDialog::builder()
            .launch(ShortcutsDialogInit(shortcuts.clone()))
            .detach();
        });
        action_register!(actions, AboutAction => { AboutDialog::builder().launch(()).detach(); });
        action_register!(actions, LocalhostAction => { Localhost::builder().launch(()).detach(); });

        actions.register_for_widget(&widgets.main_window);

        let sender_clone = sender.clone().input_sender().clone();
        glib::timeout_add_seconds_local(1, move || {
            if check_service_installed("/etc/systemd/user/e-imzo.service") {
                let active = check_service_active("e-imzo.service");
                sender_clone.send(AppMsg::RefreshService(active)).ok();
            }
            glib::ControlFlow::Continue
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => {
                main_application().quit();
            }
            AppMsg::SelectMode(msg) => {
                self.select_mode_page.emit(msg);
            }
            AppMsg::StartAndStopService => {
                if self.service_limiter {
                    return;
                }
                self.service_limiter = true;

                if self.service_active {
                    let _ = std::process::Command::new("systemctl")
                        .arg("stop")
                        .arg("--user")
                        .arg("e-imzo.service")
                        .status();
                    sender.input(AppMsg::ShowMessage(
                        gettext("E-IMZO service stopped").to_string(),
                    ));
                } else {
                    let _ = std::process::Command::new("systemctl")
                        .arg("start")
                        .arg("--user")
                        .arg("e-imzo.service")
                        .status();

                    sender.input(AppMsg::ShowMessage(
                        gettext("E-IMZO service started").to_string(),
                    ));

                    self.select_mode_page
                        .emit(SelectModeMsg::SetFileLoadedState(false));

                    self.select_mode_page
                        .emit(SelectModeMsg::RefreshCertificates);
                }
                glib::timeout_add_local_once(Duration::from_millis(1650), move || {
                    sender.input(AppMsg::ServiceLimiter(false));
                });
            }
            AppMsg::RefreshService(active) => {
                self.service_active = active;
                if active {
                    self.service.remove_css_class("off");
                    self.service.add_css_class("on");
                    self.page = Page::SelectMode;
                    self.tooltip_text = gettext("Service is ON - Click to stop");
                } else {
                    self.service.remove_css_class("on");
                    self.service.add_css_class("off");
                    self.page = Page::Welcome;
                    self.tooltip_text = gettext("Service is OFF - Click to start");
                }
            }
            AppMsg::ShowMessage(text) => {
                show_alert_dialog(&text);
            }
            AppMsg::ServiceLimiter(is_clicable) => self.service_limiter = is_clicable,
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();
        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;
        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;
        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);
        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");
        self.main_window.set_default_size(width, height);
        if is_maximized {
            self.main_window.maximize();
        }
    }
}
