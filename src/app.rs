use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw::{
        self,
        prelude::{AdwApplicationWindowExt, IsA, OrientableExt, ToValue},
    },
    gtk::{
        self, gio, glib,
        prelude::{ApplicationExt, ApplicationWindowExt, BoxExt, GtkWindowExt, SettingsExt, WidgetExt}, StackPage,
    },
    main_application, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use std::convert::identity;

use crate::{
    config::{APP_ID, PROFILE},
    modals::about::AboutDialog,
    pages::{dashboard::DashboardModel, select_mode::{SelectModePage}, welcome::WelcomeModel},
};
// use crate::welcome::AppWidgets;

#[derive(Clone)]
enum Page {
    Welcome,
    Main,
    SelectModeEnum
}

pub struct App {
    _welcome: Controller<WelcomeModel>,
    _main_page: Controller<DashboardModel>,
    _select_mode: Controller<SelectModePage>,
    _current_page: Page,
    // page: StackPage
}

#[derive(Debug)]
pub enum AppMsg {
    SetStackPage(StackPage),
    Quit,
}

relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = bool;
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About E-IMZO Manager" => AboutAction,
            }
        }
    }

    view! {
        #[root]
        main_window = adw::ApplicationWindow::new(&main_application()) {

            set_visible: true,
            // width and height below
            set_size_request: (800, 800),
            set_default_size: (900, 900),

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/com/belmoussaoui/GtkRustTemplate/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
            } else {
                None
            },

            add_breakpoint = bp_with_setters(
                adw::Breakpoint::new(
                    adw::BreakpointCondition::new_length(
                        adw::BreakpointConditionLengthType::MaxWidth,
                        400.0,
                        adw::LengthUnit::Sp,
                    )
                ),
                &[(&main_page.model().split_view, "collapsed", true)]
            ),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },
                append: &stack,
            },

        },
        stack = &gtk::Stack {
            add_named: (main_page.widget(), Some("Main")),
            add_named: (welcomepage.widget(), Some("Welcome")),
            add_named: (select_mode.widget(), Some("SelectMode")),
            set_vhomogeneous: false,
        }

    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let welcomepage = WelcomeModel::builder()
            .launch(false)
            .forward(sender.input_sender(), identity);

        let main_page = DashboardModel::builder()
            .launch((0, true))
            .forward(sender.input_sender(), identity);

        let select_mode = SelectModePage::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let current_page = if init { Page::SelectModeEnum } else { Page::Welcome };

        let widgets = view_output!();

        let model = Self {
            _welcome: welcomepage,
            _main_page: main_page,
            _select_mode: select_mode,
            _current_page: current_page.clone(),
            // page: 
        };

        widgets.load_window_size();

        match current_page {
            Page::Main => widgets.stack.set_visible_child_name("Main"),
            Page::Welcome => widgets.stack.set_visible_child_name("Welcome"),
            Page::SelectModeEnum => widgets.stack.set_visible_child_name("SelectMode"),
        };

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                AboutDialog::builder().launch(()).detach();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
            AppMsg::SetStackPage(_) => todo!(),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}
fn bp_with_setters(
    bp: adw::Breakpoint,
    additions: &[(&impl IsA<glib::Object>, &str, impl ToValue)],
) -> adw::Breakpoint {
    bp.add_setters(additions);
    bp
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
