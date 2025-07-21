use crate::app::AppMsg;
use relm4::{
    gtk::{
        self,
        prelude::{ButtonExt, OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

pub struct SelectModePage {
    method: SelectModeMethod,
}

#[derive(Debug)]
pub enum SelectModeMsg {
    SetMethod(SelectModeMethod),
    Refresh,
}

#[derive(Debug)]
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


            gtk::Button {
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,

                gtk::Label {
                        #[watch]
                        set_label: "Select a local",
                },

                connect_clicked[sender] => move |_| {
                    sender.input(SelectModeMsg::SetMethod(SelectModeMethod::Local))
                }
            },

            gtk::Button {
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,

                gtk::Label {
                        #[watch]
                        set_label: "Select a USB",
                },

                connect_clicked[sender] => move |_| {
                    sender.input(SelectModeMsg::SetMethod(SelectModeMethod::USB))
                }
            },


            #[name(liststack)]
            match model.method {
                SelectModeMethod::Local => gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        #[watch]
                        set_label: "Select a localAAAAAAAAAAAAAAAA",
                    },
                },
                SelectModeMethod::USB => gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        // set_spacing: 5,
                        // set_margin_all: 5,
                        set_hexpand: true,
                        set_vexpand: true,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::Center,
                            gtk::Label {
                                add_css_class: relm4::css::TITLE_1,

                                #[watch]
                                set_label: &format!("Welcome to e-imzo manager"),
                                set_margin_all: 1,
                            },
                            gtk::Label {
                                add_css_class: relm4::css::TITLE_2,
                                #[watch]
                                set_markup: "It seems you <a href=\"appstream://org.gnome.Calculator.desktop\">don't have e-imzo installed</a>.",
                                set_use_markup: true,
                                set_margin_all: 10,
                                set_justify: gtk::Justification::Center,
                            },
                            gtk::Label {
                                add_css_class: relm4::css::TITLE_3,
                                #[watch]
                                set_markup: "Please download and relaunch the app again.",
                                set_use_markup: true,
                                set_margin_all: 5,
                                set_justify: gtk::Justification::Center,
                            }
                        }
                    },
            },

            gtk::Label {
                set_label: "hi welcome"
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SelectModePage {
            method: SelectModeMethod::Local,
        };

        let widgets = view_output!();
        // widgets.liststack.set_vhomogeneous(false);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            // SelectModeMethod::Local => gtk::Box {
            //         set_orientation: gtk::Orientation::Vertical,
            //         gtk::Label {
            //             #[watch]
            //             set_label: "Select a local",
            //         },
            // },
            SelectModeMsg::SetMethod(method) => {
                self.method = method;
                sender.input(SelectModeMsg::Refresh);
            }
            SelectModeMsg::Refresh => todo!(),
        }
    }
}
