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
            set_title: &gettext("List of used e-imzo websites"),
            set_follows_content_size: true,
            set_presentation_mode: adw::DialogPresentationMode::Floating,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar,

                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    adw::PreferencesGroup {
                        set_title: "Websites",

                        adw::ActionRow {
                            set_title: "ahost.uz",
                            add_suffix = &gtk::LinkButton::with_label(
                                "https://clients.ahost.uz/login",
                                "Open"
                            ),
                        },

                        adw::ActionRow {
                            set_title: "id.egov.uz",
                            add_suffix = &gtk::LinkButton::with_label(
                                "https://id.egov.uz/oz",
                                "Open"
                            ),
                        },

                        adw::ActionRow {
                            set_title: "didox.uz",
                            add_suffix = &gtk::LinkButton::with_label(
                                "https://didox.uz/login_with_signature",
                                "Open"
                            ),
                        },

                        adw::ActionRow {
                            set_title: "birdarcha.uz",
                            add_suffix = &gtk::LinkButton::with_label(
                                "https://new.birdarcha.uz/login",
                                "Open"
                            ),
                        },

                        adw::ActionRow {
                            set_title: "e-invoice.uz",
                            add_suffix = &gtk::LinkButton::with_label(
                                "https://e-invoice.uz/register/",
                                "Open"
                            ),
                        },

                        adw::ActionRow {
                            set_title: "my.mehnat.uz",
                            add_suffix = &gtk::LinkButton::with_label(
                                "https://my.mehnat.uz/login#",
                                "Open"
                            ),
                        },
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
