use relm4::{
    adw,
    gtk::{
        self,
        gdk::Texture,
        gdk_pixbuf::Pixbuf,
        gio::{prelude::ApplicationExt, Cancellable, MemoryInputStream},
        glib,
        prelude::{GtkWindowExt, OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

fn embedded_logo() -> Texture {
    let bytes = include_bytes!("../.github/assets/Relm_logo.png");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}

pub struct WelcomeModel;

#[derive(Debug)]
pub enum WelcomeModelMgs {
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for WelcomeModel {
    type Init = ();
    type Input = WelcomeModelMgs;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_About E-IMZO Manager" => AboutAction,
            }
        }
    }

    view! {
        #[root]
        adw::ApplicationWindow{
            set_visible: true,
            // width and height below
            set_size_request: (800, 800),
            set_default_size: (900, 900),
            set_title: Some("Simple app"),

            connect_close_request[sender] => move |_| {
                sender.input(WelcomeModelMgs::Quit);
                glib::Propagation::Stop
            },

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    // set_spacing: 5,
                    // set_margin_all: 5,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                        gtk::Image {
                            set_pixel_size: 100,
                            set_paintable: Some(&embedded_logo()),
                        },
                        gtk::Label {
                            add_css_class: relm4::css::TITLE_1,

                            #[watch]
                            set_label: &format!("Welcom to los pollos hermanos"),
                            set_margin_all: 1,
                        },
                        gtk::Label {
                            // add_css_class: relm4::css::TITLE_1,
                            #[watch]
                            set_label: &format!("It seems you don't have e-imzo installed. Please download \nand relaunch app again"),
                            set_margin_all: 6,
                            set_justify: gtk::Justification::Center,
                        }
                }
            },
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = WelcomeModel {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            WelcomeModelMgs::Quit => relm4::main_application().quit(),
        }
    }
}
