// The Toggler page
use relm4::{gtk};
use crate::app::AppMsg;
// use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

pub struct TogglerModel {
    toggle: bool,
}

#[derive(Debug)]
pub enum ToggleMsg {
    Toggle,
}

#[relm4::component(pub)]
impl SimpleComponent for TogglerModel {
    type Init = bool;
    type Input = ToggleMsg;
    type Output = AppMsg;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::ToggleButton {
                set_label: "Toggle",
                connect_clicked => ToggleMsg::Toggle,
            },

            gtk::Label {
                #[watch]
                set_label: &format!("Toggle: {}", model.toggle),
                set_margin_all: 5,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = TogglerModel { toggle: init };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ToggleMsg::Toggle => {
                self.toggle = !self.toggle;
            }
        }
    }
}

