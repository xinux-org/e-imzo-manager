use relm4::{
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

use crate::app::AppMsg;
use crate::app::Page;

pub struct SelectModePage {}

#[derive(Debug)]
pub enum SelectModeMsg {
    SetMethod(SelectModeMethod),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectModeMethod {
    Local,
    USB,
}

#[relm4::component(pub)]
impl SimpleComponent for SelectModePage {
    type Init = ();
    type Input = SelectModeMsg;
    type Output = AppMsg;
    type Widgets = AppWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            set_hexpand: true,

               gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_spacing: 10,
                    set_margin_all: 10,

                    gtk::Button {
                        set_label: "Local",
                        connect_clicked[sender] => move |_| {
                            sender.input(SelectModeMsg::SetMethod(SelectModeMethod::Local))
                        }
                    },

                    gtk::Button {
                        set_label: "USB",
                        connect_clicked[sender] => move |_| {
                            sender.input(SelectModeMsg::SetMethod(SelectModeMethod::USB))
                        }
                    }
                },

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SelectModePage {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: SelectModeMsg, sender: ComponentSender<Self>) {
        match msg {
            SelectModeMsg::SetMethod(SelectModeMethod::Local) => {
                let _ = sender.output(AppMsg::SetPage(Page::Local));
            }
            SelectModeMsg::SetMethod(SelectModeMethod::USB) => {
                let _ = sender.output(AppMsg::SetPage(Page::USB));
            }
        }
    }
}
