use super::{TagGroupInjection, TagSize};
use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn InteractionTag(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Whether the interaction tag is disabled.
    #[prop(optional, into)]
    disabled: Option<Signal<bool>>,
    /// Size of the tag.
    #[prop(optional, into)]
    size: Option<Signal<TagSize>>,
    children: Children,
) -> impl IntoView {
    mount_style("interaction-tag", include_str!("./interaction-tag.css"));
    let tag_group = TagGroupInjection::use_context();

    let disabled = {
        if let Some(disabled) = disabled {
            Some(disabled)
        } else {
            tag_group.as_ref().map(|tag_group| tag_group.disabled)
        }
    };

    let size_class = {
        if let Some(size) = size {
            Some(size)
        } else {
            tag_group.as_ref().map(|tag_group| tag_group.size)
        }
    };

    view! {
        <div class=class_list![
            "thaw-interaction-tag",
            ("thaw-interaction-tag--disabled", move || disabled.is_some_and(|d| d.get())),
                size_class.map(|size| move || format!("thaw-interaction-tag--{}", size.get().as_str())),
                class
        ]>{children()}</div>
    }
}

#[component]
pub fn InteractionTagPrimary(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <button class=class_list!["thaw-interaction-tag-primary", class]>
            <span class="thaw-interaction-tag-primary__primary-text">{children()}</span>
        </button>
    }
}
