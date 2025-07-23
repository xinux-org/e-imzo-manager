use relm4::{
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
    }, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent
};

use crate::app::AppMsg;
use crate::pages::dashboard::DashboardModel;
use std::convert::identity;

pub struct SelectModePage {
    method: SelectModeMethod,
    local_mode: Controller<DashboardModel>,
}

#[derive(Debug)]
pub enum SelectModeMsg {
    SetMethod(SelectModeMethod),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectModeMethod {
    Initial,
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
            

            #[transition(SlideUpDown)]
            match model.method {
                SelectModeMethod::Initial => gtk::Box {
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

                SelectModeMethod::Local => gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,
                    set_hexpand: true,

                    gtk::Label {
                        set_label: "Local content selected.",
                    },

                    gtk::Label {
                        set_label: "You can now pick a local option.",
                    },
                },

                SelectModeMethod::USB => 
                    gtk::Box { 
                        set_orientation: gtk::Orientation::Vertical,
                        set_vexpand: true,
                        set_hexpand: true,
                        append: model.local_mode.widget()
                    },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        
        let local_mode = DashboardModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
    
        let model = SelectModePage {
            method: SelectModeMethod::Initial,
            local_mode: local_mode
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: SelectModeMsg, _sender: ComponentSender<Self>) {
        match msg {
            SelectModeMsg::SetMethod(method) => {
                self.method = method;
            }
        }
    }
}
