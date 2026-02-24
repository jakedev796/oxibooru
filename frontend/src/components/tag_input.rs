use leptos::prelude::*;

use crate::components::tag_autocomplete::TagAutocomplete;

/// Chip-based multi-tag input with autocomplete for adding tags.
#[component]
pub fn TagInput(
    tags: RwSignal<Vec<String>>,
    #[prop(optional, into)]
    label: String,
) -> impl IntoView {
    let on_select = Callback::new(move |name: String| {
        tags.update(|list| {
            if !list.iter().any(|t| t.eq_ignore_ascii_case(&name)) {
                list.push(name);
            }
        });
    });

    view! {
        <div class="tag-input">
            {(!label.is_empty()).then(|| view! { <label>{label}</label> })}
            <div class="tag-chips">
                {move || tags.get().into_iter().map(|tag| {
                    let tag_name = tag.clone();
                    view! {
                        <span class="tag-chip">
                            {tag}
                            <button type="button" class="chip-remove" on:click=move |_| {
                                let name = tag_name.clone();
                                tags.update(|list| list.retain(|t| t != &name));
                            }>
                                "\u{00D7}"
                            </button>
                        </span>
                    }
                }).collect_view()}
            </div>
            <TagAutocomplete on_select=on_select placeholder="Add tag..." />
        </div>
    }
}
