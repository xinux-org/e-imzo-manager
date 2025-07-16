use relm4::{
    actions::{RelmActionGroup},
    adw::{
        self,
        prelude::{AdwApplicationWindowExt, IsA, NavigationPageExt, ToValue},
    },
    gtk::{
        self, glib,
        prelude::{
            WidgetExt,
        },
    },
    Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use std::convert::identity;

use crate::{
    modals::{content::CounterModel, toggler::TogglerModel}
};
use crate::app::AppMsg;
use crate::app::WindowActionGroup;

pub struct DashboardModel {
    _counter: Controller<CounterModel>,
    _toggler: Controller<TogglerModel>,
}

#[derive(Debug)]
pub enum DashboardModelMsg {
    Quit,
}

#[relm4::component(pub)]
impl SimpleComponent for DashboardModel {
    type Init = (u8, bool);
    type Input = DashboardModelMsg;
    type Output = AppMsg;

    view! {
        #[root]
        gtk::Box {
            #[name(split_view)]
            adw::NavigationSplitView {
                set_vexpand: true,
                set_hexpand: true,

                #[wrap(Some)]
                set_sidebar = &adw::NavigationPage {
                    set_title: "Sidebar",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        // add_top_bar = &adw::HeaderBar {},

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
                        // add_top_bar = &adw::HeaderBar {},
                        set_content: Some(&stack),

                    },
                },
            },
            
            // add_breakpoint = bp_with_setters(
            //     adw::Breakpoint::new(
            //         adw::BreakpointCondition::new_length(
            //             adw::BreakpointConditionLengthType::MaxWidth,
            //             400.0,
            //             adw::LengthUnit::Sp,
            //         )
            //     ),
            //     &[(&split_view, "collapsed", true)]
            // ),
        },
        stack = &gtk::Stack {
            add_titled: (counter.widget(), None, "Counter"),
            add_titled: (toggler.widget(), None, "Toggle"),
            set_vhomogeneous: false,
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let actions = RelmActionGroup::<WindowActionGroup>::new();

        let counter = CounterModel::builder()
            .launch(init.0)
            .forward(sender.input_sender(), identity);

        let toggler = TogglerModel::builder()
            .launch(init.1)
            .forward(sender.input_sender(), identity);

        let widgets = view_output!();

        let model = Self {
            _counter: counter,
            _toggler: toggler,
        };

        widgets.stack.connect_visible_child_notify({
            let split_view = widgets.split_view.clone();
            move |_| {
                split_view.set_show_content(true);
            }
        });
        
        ComponentParts { model, widgets }
    }
}
fn bp_with_setters(
    bp: adw::Breakpoint,
    additions: &[(&impl IsA<glib::Object>, &str, impl ToValue)],
) -> adw::Breakpoint {
    bp.add_setters(additions);
    bp
}
