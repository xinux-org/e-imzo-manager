use gettextrs::gettext;
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
            // set_content_width: 325,
            set_title: &gettext("List of used e-imzo websites"),
            set_follows_content_size: true,
            set_presentation_mode: adw::DialogPresentationMode::Floating,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar,

                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 6,
                    set_spacing: 5,

                    gtk::LinkButton {
                        set_label: "ahost.uz",
                        set_uri: "https://clients.ahost.uz/login",
                    },

                    gtk::LinkButton {
                        set_label: "id.egov.uz",
                        set_uri: "https://id.egov.uz/oz",
                    },

                    gtk::LinkButton {
                        set_label: "didox.uz",
                        set_uri: "https://didox.uz/login_with_signature",
                    },

                    gtk::LinkButton {
                        set_label: "birdarcha.uz",
                        set_uri: "https://new.birdarcha.uz/login",
                    },

                    gtk::LinkButton {
                        set_label: "e-invoice.uz",
                        set_uri: "https://e-invoice.uz/register/",
                    },

                    gtk::LinkButton {
                        set_label: "my.mehnat.uz",
                        set_uri: "https://my.mehnat.uz/login#",
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
        let model = Self;
        let widgets = view_output!();
        let window = relm4::main_application().active_window();
        root.present(window.as_ref());

        ComponentParts { model, widgets }
    }
}
