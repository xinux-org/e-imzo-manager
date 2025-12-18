use relm4::{
    adw::{self, prelude::*},
    prelude::*,
};

use crate::ui::window::AppMsg;

#[derive(Debug)]
pub struct ShortcutsDialog;

#[derive(Debug, Clone)]
pub struct Shortcut {
    pub label: String,
    pub accelerator: String,
}

#[derive(Debug)]
pub struct ShortcutsDialogInit(pub Vec<Shortcut>);

impl SimpleComponent for ShortcutsDialog {
    type Root = adw::ShortcutsDialog;
    type Widgets = adw::ShortcutsDialog;
    type Init = ShortcutsDialogInit;
    type Input = ();
    type Output = AppMsg;

    fn init_root() -> Self::Root {
        adw::ShortcutsDialog::builder().build()
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = root.clone();
        let section = adw::ShortcutsSection::new(None);

        for Shortcut { label, accelerator } in &init.0 {
            section.add(adw::ShortcutsItem::new(label, accelerator))
        }

        widgets.add(section);
        widgets.present(Some(&relm4::main_application().windows()[0]));

        ComponentParts { model, widgets }
    }
}
