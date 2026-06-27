use crate::ui::window::AppMsg;
use crate::utils::is_service_installed;
use gettextrs::gettext;
use relm4::{
    gtk::{self, gdk::Texture, glib, prelude::*},
    prelude::{AsyncComponent, AsyncComponentParts},
    *,
};

pub struct WelcomeModel;

impl WelcomeModel {
    async fn embedded_logo(&self) -> Texture {
        let bytes = include_bytes!("../../.forgejo/assets/logo.png");
        let g_bytes = glib::Bytes::from(&bytes.to_vec());
        Texture::from_bytes(&g_bytes).unwrap()
    }
}

#[relm4::component(pub, async)]
impl AsyncComponent for WelcomeModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_vexpand: true,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Image {
                set_pixel_size: 320,
                set_paintable: Some(&model.embedded_logo().await),
            },

            gtk::Label {
                add_css_class: relm4::css::TITLE_2,
                #[watch]
                set_label: &gettext("Welcome to E-imzo"),
                set_margin_all: 1,
            },

            if is_service_installed() {
                gtk::Label {
                    #[watch]
                    set_label: &gettext("Please click the red button to start e-imzo service"),
                    set_margin_all: 1,
                }
            } else {
                gtk::LinkButton {
                    set_label: &gettext("Please download e-imzo service and relaunch the app again."),
                    set_uri: "https://search.nixos.org/packages?channel=unstable&query=e-imzo&show=e-imzo",
                }
            },
        }
    }
    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = WelcomeModel;
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
