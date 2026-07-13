use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::gtk::{self, gio::{self, AppLaunchContext}};
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
                #[name = "toast_overlay"]
                set_content = &adw::ToastOverlay {
                    #[wrap(Some)]
                    set_child = &adw::PreferencesPage {
                        adw::PreferencesGroup {
                            adw::ActionRow {
                                set_title: "my.gov.uz",
                                add_suffix = &gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    append = &gtk::Button {
                                        set_icon_name: "edit-copy-symbolic",
                                        set_tooltip_text: Some("Copy URL"),
                                        add_css_class: "flat",
                                        connect_clicked[toast_overlay] => move |_| {
                                            copy_to_clipboard("https://my.gov.uz/uz", &toast_overlay);
                                        },
                                    },
                                    append = &gtk::Label {
                                        set_label: "|",
                                        add_css_class: "dim-label",
                                        set_margin_start: 4,
                                        set_margin_end: 4,
                                    },
                                    append = &gtk::Button {
                                        set_icon_name: "external-link-symbolic",
                                        set_tooltip_text: Some("Open in Browser"),
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,
                                        connect_clicked[toast_overlay] => move |_| {
                                            open_uri("https://my.gov.uz/uz", &toast_overlay);
                                        }
                                    }
                                }
                            },

                            adw::ActionRow {
                                set_title: "ahost.uz",
                                add_suffix = &gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    append = &gtk::Button {
                                        set_icon_name: "edit-copy-symbolic",
                                        set_tooltip_text: Some("Copy URL"),
                                        add_css_class: "flat",
                                        connect_clicked[toast_overlay] => move |_| {
                                            copy_to_clipboard("https://clients.ahost.uz/login", &toast_overlay);
                                        },
                                    },
                                    append = &gtk::Label {
                                        set_label: "|",
                                        add_css_class: "dim-label",
                                        set_margin_start: 4,
                                         set_margin_end: 4,
                                    },
                                    append = &gtk::Button {
                                        set_icon_name: "external-link-symbolic",
                                        set_tooltip_text: Some("Open in Browser"),
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,
                                        connect_clicked[toast_overlay] => move |_| {
                                            open_uri("https://clients.ahost.uz/login", &toast_overlay);
                                        }
                                    }
                                }
                            },

                            adw::ActionRow {
                                set_title: "id.egov.uz",
                                add_suffix = &gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    append = &gtk::Button {
                                        set_icon_name: "edit-copy-symbolic",
                                        set_tooltip_text: Some("Copy URL"),
                                        add_css_class: "flat",
                                        connect_clicked[toast_overlay] => move |_| {
                                            copy_to_clipboard("https://id.egov.uz/oz", &toast_overlay);
                                        },
                                    },
                                    append = &gtk::Label {
                                        set_label: "|",
                                        add_css_class: "dim-label",
                                        set_margin_start: 4,
                                        set_margin_end: 4,
                                    },
                                    append = &gtk::Button {
                                        set_icon_name: "external-link-symbolic",
                                        set_tooltip_text: Some("Open in Browser"),
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,
                                        connect_clicked[toast_overlay] => move |_| {
                                            open_uri("https://id.egov.uz/oz", &toast_overlay);
                                        }
                                    }
                                }
                            },

                            adw::ActionRow {
                                set_title: "didox.uz",
                                add_suffix = &gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    append = &gtk::Button {
                                        set_icon_name: "edit-copy-symbolic",
                                        set_tooltip_text: Some("Copy URL"),
                                        add_css_class: "flat",
                                        connect_clicked[toast_overlay] => move |_| {
                                            copy_to_clipboard("https://didox.uz/login_with_signature", &toast_overlay);
                                        },
                                    },
                                    append = &gtk::Label {
                                        set_label: "|",
                                        add_css_class: "dim-label",
                                        set_margin_start: 4,
                                        set_margin_end: 4,
                                    },
                                    append = &gtk::Button {
                                        set_icon_name: "external-link-symbolic",
                                        set_tooltip_text: Some("Open in Browser"),
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,
                                        connect_clicked[toast_overlay] => move |_| {
                                            open_uri("https://didox.uz/login_with_signature", &toast_overlay);
                                        }
                                    }
                                }
                            },

                            adw::ActionRow {
                                set_title: "birdarcha.uz",
                                add_suffix = &gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    append = &gtk::Button {
                                        set_icon_name: "edit-copy-symbolic",
                                        set_tooltip_text: Some("Copy URL"),
                                        add_css_class: "flat",
                                        connect_clicked[toast_overlay] => move |_| {
                                            copy_to_clipboard("https://new.birdarcha.uz/login", &toast_overlay);
                                        },
                                    },
                                    append = &gtk::Label {
                                        set_label: "|",
                                        add_css_class: "dim-label",
                                        set_margin_start: 4,
                                        set_margin_end: 4,
                                    },
                                    append = &gtk::Button {
                                        set_icon_name: "external-link-symbolic",
                                        set_tooltip_text: Some("Open in Browser"),
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,
                                        connect_clicked[toast_overlay] => move |_| {
                                            open_uri("https://new.birdarcha.uz/login", &toast_overlay);
                                        }
                                    }
                                }
                            },

                            adw::ActionRow {
                                set_title: "e-invoice.uz",
                                add_suffix = &gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    append = &gtk::Button {
                                        set_icon_name: "edit-copy-symbolic",
                                        set_tooltip_text: Some("Copy URL"),
                                        add_css_class: "flat",
                                        connect_clicked[toast_overlay] => move |_| {
                                            copy_to_clipboard("https://e-invoice.uz/register/", &toast_overlay);
                                        },
                                    },
                                    append = &gtk::Label {
                                        set_label: "|",
                                        add_css_class: "dim-label",
                                        set_margin_start: 4,
                                        set_margin_end: 4,
                                    },
                                    append = &gtk::Button {
                                        set_icon_name: "external-link-symbolic",
                                        set_tooltip_text: Some("Open in Browser"),
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,
                                        connect_clicked[toast_overlay] => move |_| {
                                            open_uri("https://e-invoice.uz/register/", &toast_overlay);
                                        }
                                    }
                                }
                            },

                            adw::ActionRow {
                                set_title: "my.mehnat.uz",
                                add_suffix = &gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    append = &gtk::Button {
                                        set_icon_name: "edit-copy-symbolic",
                                        set_tooltip_text: Some("Copy URL"),
                                        add_css_class: "flat",
                                        connect_clicked[toast_overlay] => move |_| {
                                            copy_to_clipboard("https://my.mehnat.uz/login#", &toast_overlay);
                                        },
                                    },
                                    append = &gtk::Label {
                                        set_label: "|",
                                        add_css_class: "dim-label",
                                        set_margin_start: 4,
                                        set_margin_end: 4,
                                    },
                                    append = &gtk::Button {
                                        set_icon_name: "external-link-symbolic",
                                        set_tooltip_text: Some("Open in Browser"),
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,
                                        connect_clicked[toast_overlay] => move |_| {
                                            open_uri("https://my.mehnat.uz/login#", &toast_overlay);
                                        }
                                    }
                                }
                            },

                            adw::ActionRow {
                                set_title: "esi.uz",
                                add_suffix = &gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    append = &gtk::Button {
                                        set_icon_name: "edit-copy-symbolic",
                                        set_tooltip_text: Some("Copy URL"),
                                        add_css_class: "flat",
                                        connect_clicked[toast_overlay] => move |_| {
                                            copy_to_clipboard("https://esi.uz/", &toast_overlay);
                                        },
                                    },
                                    append = &gtk::Label {
                                        set_label: "|",
                                        add_css_class: "dim-label",
                                        set_margin_start: 4,
                                        set_margin_end: 4,
                                    },
                                    append = &gtk::Button {
                                        set_icon_name: "external-link-symbolic",
                                        set_tooltip_text: Some("Open in Browser"),
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,
                                        connect_clicked[toast_overlay] => move |_| {
                                            open_uri("https://esi.uz/", &toast_overlay);
                                        }
                                    }
                                }
                            },
                        }
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
        let model: AwesomeModel = Self;
        let widgets: AwesomeModelWidgets = view_output!();
        let window: Option<gtk::Window> = relm4::main_application().active_window();
        root.present(window.as_ref());

        ComponentParts { model, widgets }
    }
}

fn open_uri(uri: &str, toast_overlay: &adw::ToastOverlay) {
    if let _ = gio::AppInfo::launch_default_for_uri(uri, None::<&AppLaunchContext>) {
    } else {
        let toast: adw::Toast = adw::Toast::new("Cannot open website in Browser");
        toast.set_timeout(2);
        toast_overlay.add_toast(toast);
    }
}

fn copy_to_clipboard(url: &str, toast_overlay: &adw::ToastOverlay) {
    if let Some(display) = gtk::gdk::Display::default() {
        display.clipboard().set_text(url);
        let toast: adw::Toast = adw::Toast::new("Copied to clipboard");
        toast.set_timeout(2);
        toast_overlay.add_toast(toast);
    } else {
        let toast: adw::Toast = adw::Toast::new("Cannot copy URL to your clipboard");
        toast.set_timeout(2);   
        toast_overlay.add_toast(toast);
    }
}

// #[gtk::test]
// fn atest_copy_to_clipboard() {
//     let display = gtk::gdk::Display::default();

//     // assert_eq!(
//     //     display,
//     //     None
//     // );
    
//     let toast: adw::Toast = adw::Toast::new("Cannot copy URL to your clipboard");
//     toast.set_timeout(2);
//     // assert_eq!(toast.title(), Some("Cannot copy URL to your clipboard".into()));
    
//     let toast_overlay = adw::ToastOverlay::default();
//     toast_overlay.add_toast(toast);


//     // let toogle_group = adw::ToggleGroup::default();

//     // let toggle1 = adw::Toggle::builder()
//     //     .name("toggle1")
//     //     .child(&gtk::Box::default())
//     //     .build();
//     // let toggle2 = adw::Toggle::default();
//     // let toggle3 = adw::Toggle::default();

//     // let toogle_group_item1 = toogle_group.factory_append(&toggle1, &());
//     // let toogle_group_item2 = toogle_group.factory_append(&toggle2, &());
//     // let toogle_group_item3 = toogle_group.factory_insert_after(&toggle3, &(), &toogle_group_item1);

//     // assert_eq!(
//     //     adw::ToggleGroup::returned_widget_to_child(&toogle_group_item1),
//     //     toggle1
//     // );
//     // assert_eq!(
//     //     adw::ToggleGroup::returned_widget_to_child(&toogle_group_item2),
//     //     toggle2
//     // );
//     // assert_eq!(
//     //     adw::ToggleGroup::returned_widget_to_child(&toogle_group_item3),
//     //     toggle3
//     // );

//     // assert_eq!(toogle_group.factory_remove(&toogle_group_item3), ());
//     // assert!(toogle_group.toggle_by_name("toggle1") == Some(toogle_group_item1));
// }