use relm4::{gtk};
use relm4::gtk::prelude::{OrientableExt, BoxExt};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};
use relm4::gtk::prelude::GtkWindowExt;

pub struct Welcome;

#[relm4::component(pub)]
impl SimpleComponent for Welcome {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        #[root]

        gtk::Window {
            set_title: Some("Simple app"),
            set_default_size: (800, 800),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter:"),
                    set_margin_all: 5,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Welcome { };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
