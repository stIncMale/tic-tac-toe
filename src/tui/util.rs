use std::collections::HashMap;

use cursive::{
    menu::{
        Item,
        Item::{Delimiter, Leaf, Subtree},
        Tree,
    },
    views::Menubar,
};
use xxhash_rust::xxh3::Xxh3Builder;

use crate::tui::{
    menu,
    util::MenuItemSwitchState::{Disabled, Enabled},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MenuItemSwitchState {
    Enabled,
    Disabled,
}

impl From<bool> for MenuItemSwitchState {
    fn from(value: bool) -> Self {
        if value {
            Enabled
        } else {
            Disabled
        }
    }
}

impl From<MenuItemSwitchState> for bool {
    fn from(value: MenuItemSwitchState) -> Self {
        value == Enabled
    }
}

#[derive(Debug)]
struct MenuItemsState(HashMap<String, MenuItemSwitchState, Xxh3Builder>);

#[derive(Debug)]
struct MenuItemsStateBuilder(HashMap<String, MenuItemSwitchState, Xxh3Builder>);

impl MenuItemsStateBuilder {
    fn new() -> Self {
        Self(HashMap::with_hasher(Xxh3Builder::new()))
    }

    fn add(&mut self, label: &str, enabled: MenuItemSwitchState) {
        assert!(
            self.0.insert(label.to_owned(), enabled).is_none(),
            "duplicate menu item label: {label:?}"
        );
    }

    fn build(self) -> MenuItemsState {
        MenuItemsState(self.0)
    }
}

#[derive(Debug)]
pub struct MenuItemsStateSwitcher {
    orig_state: Option<MenuItemsState>,
}

impl MenuItemsStateSwitcher {
    pub fn new() -> Self {
        Self { orig_state: None }
    }

    pub fn with_all_disabled(menu: &mut Menubar) -> Self {
        let mut new = MenuItemsStateSwitcher::new();
        new.disable_all(menu);
        new
    }

    pub fn disable_all(&mut self, menu: &mut Menubar) {
        self.switch(menu, |_| Some(Disabled));
    }

    pub fn switch(
        &mut self,
        menu: &mut Menubar,
        state_computer: impl Fn(&str) -> Option<MenuItemSwitchState> + Clone,
    ) {
        assert!(self.orig_state.is_none());
        self.orig_state = MenuItemsStateSwitcher::set_menu_items_state(menu, true, |label| {
            if label == menu::EXIT_LABEL {
                None
            } else {
                state_computer(label)
            }
        });
    }

    pub fn restore(&self, menu: &mut Menubar) {
        let orig_state = self
            .orig_state
            .as_ref()
            .expect("`restore` should be called after `disable`");
        MenuItemsStateSwitcher::set_menu_items_state(menu, false, |label| {
            orig_state.0.get(label).copied()
        });
    }

    /// Returns the original state of the menu items iff `record_orig_state`.
    fn set_menu_items_state(
        menu: &mut Menubar,
        record_orig_state: bool,
        state_computer: impl Fn(&str) -> Option<MenuItemSwitchState> + Clone,
    ) -> Option<MenuItemsState> {
        let mut orig_state_builder = if record_orig_state {
            Some(MenuItemsStateBuilder::new())
        } else {
            None
        };
        for i in 0..menu.len() {
            if let Some(tree) = menu.get_subtree(i) {
                MenuItemsStateSwitcher::set_tree_items_state(
                    tree,
                    &mut orig_state_builder,
                    state_computer.clone(),
                );
            }
        }
        orig_state_builder.map(MenuItemsStateBuilder::build)
    }

    fn set_tree_items_state(
        tree: &mut Tree,
        orig_state_builder: &mut Option<MenuItemsStateBuilder>,
        state_computer: impl Fn(&str) -> Option<MenuItemSwitchState> + Clone,
    ) {
        tree.children.iter_mut().for_each(|item| {
            if let Some(tree) = item.as_subtree() {
                MenuItemsStateSwitcher::set_tree_items_state(
                    tree,
                    orig_state_builder,
                    state_computer.clone(),
                );
            }
            if let Some(orig_state) =
                MenuItemsStateSwitcher::set_item_state(item, state_computer.clone())
            {
                if let Some(orig_state_builder) = orig_state_builder {
                    orig_state_builder.add(item.label(), orig_state);
                }
            }
        });
    }

    /// Returns the original `item` state.
    fn set_item_state(
        item: &mut Item,
        state_computer: impl FnOnce(&str) -> Option<MenuItemSwitchState>,
    ) -> Option<MenuItemSwitchState> {
        match item {
            Leaf {
                ref mut enabled,
                ref label,
                ..
            }
            | Subtree {
                ref mut enabled,
                ref label,
                ..
            } => {
                let orig_state: MenuItemSwitchState = (*enabled).into();
                if let Some(computed_state) = state_computer(label.source()) {
                    *enabled = computed_state.into();
                }
                Some(orig_state)
            }
            Delimiter => None,
        }
    }
}
