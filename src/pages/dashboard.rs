use relm4::{
    adw::{self, prelude::NavigationPageExt},
    gtk::{
        self,
        prelude::{BoxExt, WidgetExt},
    },
    ComponentParts, ComponentSender, SimpleComponent,
};
use std::collections::HashMap;

use crate::app::AppMsg;

struct Certificate {
    name: String,
    data: Vec<String>,
}

impl Certificate {
    fn new<T>(name: T, data: Vec<String>) -> Self
    where
        T: ToString,
    {
        Certificate {
            name: name.to_string(),
            data,
        }
    }
}

pub struct DashboardModel {
    pub split_view: adw::NavigationSplitView,
    // certificate: Certificate
}

#[relm4::component(pub)]
impl SimpleComponent for DashboardModel {
    type Init = (u8, bool);
    type Input = ();
    type Output = AppMsg;

    view! {
        #[root]
        gtk::Box {
            set_vexpand: true,
            set_hexpand: true,

            #[name(split_view)]
            adw::NavigationSplitView {
                set_vexpand: true,
                set_hexpand: true,

                #[wrap(Some)]
                set_sidebar = &adw::NavigationPage {
                    set_title: "Sidebar",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {

                        #[wrap(Some)]
                        set_content = &gtk::StackSidebar {
                            set_stack: &stack,
                        },
                    },
                },

                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    set_title: "Content",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        set_content: Some(&stack),

                    },
                },
            },
        },
        #[name(stack)]
        gtk::Stack {
            // set_transition_type: gtk::StackTransitionType::SlideUp,
            // set_transition_duration: 500,
            set_vhomogeneous: false,
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut certificate = HashMap::new();

        certificate.insert(
            "test.txt111",
            vec!["asd1111".to_string(), "asdas111".to_string()],
        );
        certificate.insert(
            "tesasdasdasdt.txt",
            vec!["asd22222".to_string(), "asdas2222".to_string()],
        );
        certificate.insert(
            "tesasdasdasdt3333.txt",
            vec!["asd3333".to_string(), "333333".to_string()],
        );

        let widgets = view_output!();
        let stack = widgets.stack.clone();

        for (file_name, data) in &certificate {
            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

            for item in data {
                let label = gtk::Label::new(Some(item));
                vbox.append(&label);
            }
            stack.add_titled(&vbox, None, file_name);
        }

        // Force sidebar to be visible
        widgets.stack.connect_visible_child_notify({
            let split_view = widgets.split_view.clone();
            move |_| {
                split_view.set_show_content(true);
            }
        });

        let model = Self {
            split_view: widgets.split_view.clone(),
            // certificate: certificate
        };

        ComponentParts { model, widgets }
    }
}
