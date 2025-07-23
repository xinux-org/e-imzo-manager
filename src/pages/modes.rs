use relm4::{
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt},
    },
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

use crate::app::AppMsg;
use crate::pages::select_mode::SelectModeMsg;


pub struct LocalModeModel {
}


#[relm4::component(pub)]
impl SimpleComponent for LocalModeModel {
    type Init = ();
    type Input = ();
    type Output = SelectModeMsg;
    type Widgets = AppWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            set_hexpand: true,
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
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = LocalModeModel { };
        
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    // fn update(&mut self, msg: SelectModeMsg, _sender: ComponentSender<Self>) {
    //     match msg {
    //         SelectModeMsg::SetMethod(method) => {
    //             self.method = method;
    //         }
    //     }
    // }
}
