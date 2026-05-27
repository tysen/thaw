use super::ComboboxRuleTrigger;
use leptos::prelude::*;
use std::collections::HashMap;
use thaw_utils::{Model, VecModel};

/// (value, text, disabled)
pub(super) type ComboboxOption = (String, String, Signal<bool>);

#[derive(Clone, Copy)]
pub(crate) struct ComboboxInjection {
    pub(super) value: Model<String>,
    pub(super) selected_options: VecModel<String>,
    pub(super) options: StoredValue<HashMap<String, ComboboxOption>>,
    pub(super) is_show_listbox: RwSignal<bool>,
    pub(super) validate: Callback<Option<ComboboxRuleTrigger>, bool>,
    pub multiselect: bool,
}

impl ComboboxInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    /// value: (value, text, disabled)
    pub fn insert_option(&self, id: String, value: ComboboxOption) {
        self.options.update_value(|options| {
            options.insert(id, value);
        });
    }

    pub fn remove_option(&self, id: &String) {
        self.options.update_value(|options| {
            options.remove(id);
        });
    }

    pub fn is_selected(&self, value: &String) -> bool {
        self.selected_options.contains(value)
    }

    pub fn select_option(&self, value: &String, text: &str) {
        self.selected_options.update(|options| match options {
            (None, None, Some(v)) => {
                if let Some(index) = v.iter().position(|v| v == value) {
                    v.remove(index);
                    return;
                }
                v.push(value.clone());
            }
            (None, Some(v), None) => {
                *v = Some(value.clone());
                self.value.set(text.to_owned());
                self.is_show_listbox.set(false);
            }
            (Some(v), None, None) => {
                *v = value.clone();
                self.value.set(text.to_owned());
                self.is_show_listbox.set(false);
            }
            _ => unreachable!(),
        });
        self.validate.run(Some(ComboboxRuleTrigger::Change));
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ComboboxSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ComboboxSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
