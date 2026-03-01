// if we just want to add button action
#[macro_export]
macro_rules! action_register {
    ( $actions:ident, $action:ty => $msg:block ) => {{
        $actions.add_action(RelmAction::<$action>::new_stateless(move |_| $msg));
    }};
}

// if we want to add action button with shortcuts
#[macro_export]
macro_rules! shortcut_register {
    ( ($app:ident, $shortcuts:ident, $actions:ident), $label:expr => $accelerator:expr, $action:ty => $msg:block ) => {{
        $shortcuts.push(Shortcut {
            label: $label.to_string(),
            accelerator: $accelerator.to_string(),
        });

        $app.set_accelerators_for_action::<$action>(&[$accelerator]);

        $actions.add_action(RelmAction::<$action>::new_stateless(move |_| $msg));
    }};
}

// if we want to add action button with shortcuts by triggering Enums
#[macro_export]
macro_rules! shortcut_register_ws {
    ( ($app:ident, $shortcuts:ident, $actions:ident, $sender:ident), $label:expr => $accelerator:expr, $action:ty => $msg:expr ) => {{
        $shortcuts.push(Shortcut {
            label: $label.to_string(),
            accelerator: $accelerator.to_string(),
        });

        $app.set_accelerators_for_action::<$action>(&[$accelerator]);

        $actions.add_action(RelmAction::<$action>::new_stateless({
            let sender = $sender.to_owned();
            move |_| {
                sender.input($msg);
            }
        }));
    }};
}
