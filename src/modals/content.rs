// The Counter page
use relm4::{gtk};
use crate::app::AppMsg;
// use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

pub struct CounterModel {
    counter: u8,
}

#[derive(Debug)]
pub enum CounterMsg {
    Increment,
    Decrement,
}

#[relm4::component(pub)]
impl SimpleComponent for CounterModel {
    type Init = u8;
    type Input = CounterMsg;
    type Output = AppMsg;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Button {
                set_label: "Increment",
                connect_clicked => CounterMsg::Increment
            },

            gtk::Button::with_label("Decrement") {
                connect_clicked => CounterMsg::Decrement
            },

            gtk::Label {
                #[watch]
                set_label: &format!("Counter: {}", model.counter),
                set_margin_all: 5,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CounterModel { counter: init };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            CounterMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            CounterMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}
