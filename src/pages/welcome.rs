use crate::app::AppMsg;
use relm4::{
    gtk::{
        self,
        gdk::Texture,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
        glib,
        prelude::{BoxExt, OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

fn embedded_logo() -> Texture {
    let bytes = include_bytes!("../../.github/assets/e_imzo.png");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}

pub struct WelcomeModel;

#[relm4::component(pub)]
impl SimpleComponent for WelcomeModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;
    type Widgets = AppWidgets;
    
    
    view! {
        
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_vexpand: true,
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
                    
            gtk::Image {
                set_pixel_size: 320,
                set_paintable: Some(&embedded_logo()),
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
            }
        }    
        
        
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
