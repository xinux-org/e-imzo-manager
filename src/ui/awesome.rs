use gettextrs::gettext;
use relm4::{
    adw::prelude::*,
    gtk::{
        self,
        gio::{self, AppLaunchContext},
    },
};
use relm4::{prelude::*, view};
use std::collections::HashMap;

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
                #[name(toast_overlay)]
                set_content = &adw::ToastOverlay {
                    #[wrap(Some)]
                    set_child = &adw::PreferencesPage {
                        #[name(awesome_list)]
                        adw::PreferencesGroup { }
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
        let toast_overlay = widgets.toast_overlay.clone();
        let awesome_list = widgets.awesome_list.clone();
        model.setup_awesome_list(awesome_list, &toast_overlay);

        let window: Option<gtk::Window> = relm4::main_application().active_window();
        root.present(window.as_ref());
        ComponentParts { model, widgets }
    }
}

impl AwesomeModel {
    fn setup_awesome_list(
        self,
        awesome_list: adw::PreferencesGroup,
        toast_overlay: &adw::ToastOverlay,
    ) {
        let sites: HashMap<&str, &str> = HashMap::from([
            ("my.gov.uz", "https://my.gov.uz/uz"),
            ("ahost.uz", "https://clients.ahost.uz/login"),
            ("id.egov.uz", "https://id.egov.uz/oz"),
            ("didox.uz", "https://didox.uz/login_with_signature"),
            ("birdarcha.uz", "https://new.birdarcha.uz/login"),
            ("e-invoice.uz", "https://e-invoice.uz/register/"),
            ("my.mehnat.uz", "https://my.mehnat.uz/login#"),
            ("esi.uz", "https://esi.uz/"),
        ]);
        for (site, url) in sites {
            view! {
                row = adw::ActionRow {
                    set_title: site,
                    add_suffix = &gtk::Box {
                        set_valign: gtk::Align::Center,
                        append = &gtk::Button {
                            set_icon_name: "edit-copy-symbolic",
                            set_tooltip_text: Some("Copy URL"),
                            add_css_class: "flat",
                            connect_clicked[toast_overlay] => move |_| {
                                self.copy_to_clipboard(url, &toast_overlay);
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
                                self.open_uri(url, &toast_overlay);
                            }
                        }
                    }
                },
            };
            awesome_list.add(&row);
        }
    }
    fn open_uri(&self, uri: &str, toast_overlay: &adw::ToastOverlay) {
        if gio::AppInfo::launch_default_for_uri(uri, None::<&AppLaunchContext>).is_ok() {
        } else {
            let toast: adw::Toast = adw::Toast::new("Cannot open website in Browser");
            toast.set_timeout(2);
            toast_overlay.add_toast(toast);
        }
    }
    fn copy_to_clipboard(&self, url: &str, toast_overlay: &adw::ToastOverlay) {
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
