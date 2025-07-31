use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AwesomeModel;

#[relm4::component(pub)]
impl SimpleComponent for AwesomeModel {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::Dialog {
            set_content_width: 325,
            set_title: "List of used e-imzo websites",

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar,

                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 12,

                    gtk::Label {
                        set_markup: "1. <a href=\"https://clients.ahost.uz/login\">ahost.uz</a>",
                        set_use_markup: true,
                        set_margin_all: 5,
                        set_justify: gtk::Justification::Left,
                    },

                    gtk::Label {
                        set_markup: "2. <a href=\"https://id.egov.uz/oz\">id.egov.uz</a>",
                        set_use_markup: true,
                        set_margin_all: 5,
                        set_justify: gtk::Justification::Left,
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();
        let window = relm4::main_application().active_window();
        root.present(window.as_ref());

        ComponentParts { model, widgets }
    }
}
