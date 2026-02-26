use std::collections::HashMap;

use leptos::prelude::*;

use crate::components::tag_autocomplete::TagAutocomplete;
use crate::settings::SettingsState;

/// Chip-based multi-tag input with autocomplete for adding tags.
/// Tag chips are colored according to their category.
#[component]
pub fn TagInput(tags: RwSignal<Vec<String>>, #[prop(optional, into)] label: String) -> impl IntoView {
    let settings = expect_context::<SettingsState>();

    // Map tag name → category for coloring chips
    let category_map: RwSignal<HashMap<String, String>> = RwSignal::new(HashMap::new());

    let on_select = Callback::new(move |(name, category): (String, String)| {
        tags.update(|list| {
            if !list.iter().any(|t| t.eq_ignore_ascii_case(&name)) {
                list.push(name.clone());
            }
        });
        if !category.is_empty() {
            category_map.update(|map| {
                map.insert(name, category);
            });
        }
    });

    view! {
        <div class="tag-input">
            {(!label.is_empty()).then(|| view! { <label>{label}</label> })}
            <div class="tag-chips">
                {move || {
                    let cat_map = category_map.get();
                    tags.get().into_iter().map(|tag| {
                        let tag_name = tag.clone();
                        let display = settings.display_name(&tag);
                        let cat = cat_map.get(&tag).cloned().unwrap_or_default();
                        let chip_class = if cat.is_empty() {
                            "tag-chip".to_string()
                        } else {
                            format!("tag-chip tag-category-{cat}")
                        };
                        view! {
                            <span class=chip_class>
                                {display}
                                <button type="button" class="chip-remove" on:click=move |_| {
                                    let name = tag_name.clone();
                                    tags.update(|list| list.retain(|t| t != &name));
                                }>
                                    <i class="fa fa-remove" />
                                </button>
                            </span>
                        }
                    }).collect_view()
                }}
            </div>
            <TagAutocomplete on_select=on_select placeholder="Add tag..." />
        </div>
    }
}
