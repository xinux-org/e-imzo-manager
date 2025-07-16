use crate::app::AppMsg;
use relm4::{
    gtk::{
        self,
        gdk::Texture,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
        glib,
        prelude::{OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

fn embedded_logo() -> Texture {
    let bytes = include_bytes!("../../.github/assets/Relm_logo.png");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}

pub struct WelcomeModel;

#[relm4::component(pub)]
impl SimpleComponent for WelcomeModel {
    type Init = bool;
    type Input = ();
    type Output = AppMsg;
    type Widgets = AppWidgets;

    view! {
        gtk::Box{

            set_orientation: gtk::Orientation::Vertical,
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
                        set_markup: "It seems you <a href=\"appstream://org.gnome.Calculator.desktop\">don't have e-imzo installed</a>. Please download and relaunch the app again.",
                        set_use_markup: true,
                        set_margin_all: 6,
                        set_justify: gtk::Justification::Center,
                    }
                }
            },
        }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = WelcomeModel { };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
