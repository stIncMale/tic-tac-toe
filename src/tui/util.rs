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

use crate::tui::EXIT_MENU_ITEM_LABEL;

struct MenuLeafItemsState(HashMap<String, bool, Xxh3Builder>);

struct MenuLeafItemsStateBuilder(HashMap<String, bool, Xxh3Builder>);

impl MenuLeafItemsStateBuilder {
    fn new() -> Self {
        Self(HashMap::with_hasher(Xxh3Builder::new()))
    }

    fn add(&mut self, label: &str, enabled: bool) {
        assert!(
            self.0.insert(label.to_owned(), enabled).is_none(),
            "Duplicate menu item label: {label:?}"
        );
    }

    fn build(self) -> MenuLeafItemsState {
        MenuLeafItemsState(self.0)
    }
}

pub struct MenuLeafItemsStateSwitcher {
    orig_state: Option<MenuLeafItemsState>,
}

impl MenuLeafItemsStateSwitcher {
    pub fn new() -> Self {
        Self { orig_state: None }
    }

    pub fn disable_all(&mut self, menu: &mut Menubar) {
        self.disable(menu, |_| Some(false));
    }

    pub fn disable(
        &mut self,
        menu: &mut Menubar,
        state_computer: impl Fn(&str) -> Option<bool> + Clone,
    ) {
        assert!(self.orig_state.is_none());
        self.orig_state = MenuLeafItemsStateSwitcher::set_menu_state(menu, true, |label| {
            if label == EXIT_MENU_ITEM_LABEL {
                None
            } else {
                state_computer(label)
            }
        });
    }

    pub fn restore(&self, menu: &mut Menubar) {
        let orig_state = self.orig_state.as_ref().expect("Should be `Some`.");
        MenuLeafItemsStateSwitcher::set_menu_state(menu, false, |label| {
            orig_state.0.get(label).copied()
        });
    }

    /// Returns the original state of [`Leaf`] menu items iff `record_orig_state`.
    fn set_menu_state(
        menu: &mut Menubar,
        record_orig_state: bool,
        state_computer: impl Fn(&str) -> Option<bool> + Clone,
    ) -> Option<MenuLeafItemsState> {
        let mut orig_state_builder = if record_orig_state {
            Some(MenuLeafItemsStateBuilder::new())
        } else {
            None
        };
        for i in 0..menu.len() {
            if let Some(tree) = menu.get_subtree(i) {
                MenuLeafItemsStateSwitcher::set_leaf_items_state(
                    tree,
                    &mut orig_state_builder,
                    state_computer.clone(),
                );
            }
        }
        orig_state_builder.map(MenuLeafItemsStateBuilder::build)
    }

    fn set_leaf_items_state(
        tree: &mut Tree,
        orig_state_builder: &mut Option<MenuLeafItemsStateBuilder>,
        state_computer: impl Fn(&str) -> Option<bool> + Clone,
    ) {
        tree.children.iter_mut().for_each(|item| {
            if let Some(tree) = item.as_subtree() {
                MenuLeafItemsStateSwitcher::set_leaf_items_state(
                    tree,
                    orig_state_builder,
                    state_computer.clone(),
                );
            } else if let Some(orig_state) =
                MenuLeafItemsStateSwitcher::set_leaf_item_state(item, state_computer.clone())
            {
                if let Some(orig_state_builder) = orig_state_builder {
                    orig_state_builder.add(item.label(), orig_state);
                }
            }
        });
    }

    /// Returns the original `item` state iff the `item` is [`Leaf`].
    fn set_leaf_item_state(
        item: &mut Item,
        state_computer: impl FnOnce(&str) -> Option<bool>,
    ) -> Option<bool> {
        match item {
            Leaf {
                ref mut enabled,
                ref label,
                ..
            } => {
                let orig_state = *enabled;
                if let Some(computed_state) = state_computer(label.source()) {
                    *enabled = computed_state;
                }
                Some(orig_state)
            }
            Subtree { .. } | Delimiter => None,
        }
    }
}
