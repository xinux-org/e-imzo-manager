use relm4::adw::prelude::*;
use relm4::gtk::prelude::*;
use relm4::{css, prelude::*};

#[derive(Debug, Clone, Copy)]
pub struct AwesomeModel;

#[relm4::component(pub)]
impl SimpleComponent for AwesomeModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
            dialog = adw::Dialog {
                // set_content_width: 325,
                set_title: "asdasdasda",
                set_can_close: true,
                
                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_margin_all: 24,
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 24,

                        gtk::Label {
                            add_css_class: css::classes::ERROR,
                            set_label: "asdasdas",
                        },

                        gtk::Label {
                            add_css_class: relm4::css::TITLE_2,
                            #[watch]
                            set_label: &format!("Welcome to E-imzo"),
                            set_margin_all: 1,
                        },

                        gtk::Label {
                            add_css_class: relm4::css::TITLE_4,
                            #[watch]
                            set_markup: "It seems you <a href=\"appstream://org.gnome.Calculator.desktop\">don't have e-imzo installed</a>.",
                            set_use_markup: true,
                            set_margin_all: 5,
                            set_justify: gtk::Justification::Center,
                        },

                        gtk::Label {
                            add_css_class: relm4::css::TITLE_4,
                            #[watch]
                            set_markup: "Please download and relaunch the app again.",
                            set_use_markup: true,
                            set_margin_all: 5,
                            set_justify: gtk::Justification::Center,
                        },
                    }

                }
            }
        }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        tracing::info!("Initializing about dialog");

        let model = Self;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
