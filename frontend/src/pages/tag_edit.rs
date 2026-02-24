use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::category::TagCategoryInfo;

use crate::api::tags::{CreateTagBody, UpdateTagBody};
use crate::api::{ApiClient, ApiError};
use crate::components::tag_input::TagInput;

/// Used for both creating and editing tags. When `name` URL param is present, it's edit mode.
#[component]
pub fn TagEditPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_params_map();
    let navigate = use_navigate();

    let tag_name = move || params.get().get("name").unwrap_or_default();
    let is_create = move || tag_name().is_empty();

    // Loading state
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);

    // Form signals
    let (names_str, set_names_str) = signal(String::new());
    let (category, set_category) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let implications = RwSignal::new(Vec::<String>::new());
    let suggestions = RwSignal::new(Vec::<String>::new());
    let (version, set_version) = signal(String::new());
    let categories = RwSignal::new(Vec::<TagCategoryInfo>::new());

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);

    // Load tag data + categories
    Effect::new(move || {
        let client = api.get_untracked();
        let name = tag_name();
        leptos::task::spawn_local(async move {
            // Load categories
            if let Ok(cats) = client.get_tag_categories().await {
                categories.set(cats.results);
            }

            if name.is_empty() {
                // Create mode — no tag to load
                set_loading.set(false);
                return;
            }

            match client.get_tag(&name).await {
                Ok(tag) => {
                    let tag_names = tag.names.unwrap_or_default();
                    set_names_str.set(tag_names.join(", "));
                    set_category.set(tag.category.unwrap_or_default());
                    set_description.set(tag.description.unwrap_or_default());
                    let impl_names: Vec<String> = tag
                        .implications
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|t| t.names.into_iter().next())
                        .collect();
                    implications.set(impl_names);
                    let sugg_names: Vec<String> = tag
                        .suggestions
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|t| t.names.into_iter().next())
                        .collect();
                    suggestions.set(sugg_names);
                    set_version.set(tag.version.unwrap_or_default());
                    set_loading.set(false);
                }
                Err(_) => {
                    set_load_error.set(true);
                    set_loading.set(false);
                }
            }
        });
    });

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let parsed_names: Vec<String> = names_str
            .get_untracked()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if parsed_names.is_empty() {
            set_error_msg.set(Some("At least one name is required.".into()));
            return;
        }

        let cat = category.get_untracked();
        let desc = description.get_untracked();
        let impls = implications.get_untracked();
        let suggs = suggestions.get_untracked();

        set_submitting.set(true);
        set_error_msg.set(None);
        set_success_msg.set(None);
        let client = api.get_untracked();
        let name = tag_name();
        let nav = navigate.clone();

        if name.is_empty() {
            // Create
            let body = CreateTagBody {
                category: if cat.is_empty() { "default".into() } else { cat },
                names: parsed_names,
                description: if desc.is_empty() { None } else { Some(desc) },
                implications: if impls.is_empty() { None } else { Some(impls) },
                suggestions: if suggs.is_empty() { None } else { Some(suggs) },
            };
            leptos::task::spawn_local(async move {
                match client.create_tag(&body).await {
                    Ok(tag) => {
                        let new_name = tag.names.and_then(|n| n.into_iter().next()).unwrap_or_default();
                        let encoded = js_sys::encode_uri_component(&new_name);
                        nav(&format!("/tag/{encoded}"), Default::default());
                    }
                    Err(e) => {
                        set_error_msg.set(Some(format_api_error(&e)));
                        set_submitting.set(false);
                    }
                }
            });
        } else {
            // Update
            let body = UpdateTagBody {
                version: version.get_untracked(),
                category: Some(cat),
                names: Some(parsed_names),
                description: Some(desc),
                implications: Some(impls),
                suggestions: Some(suggs),
            };
            leptos::task::spawn_local(async move {
                match client.update_tag(&name, &body).await {
                    Ok(updated) => {
                        set_version.set(updated.version.unwrap_or_default());
                        set_success_msg.set(Some("Tag updated.".into()));
                        set_submitting.set(false);
                        // If name changed, navigate
                        let new_name = updated.names.and_then(|n| n.into_iter().next()).unwrap_or(name.clone());
                        if new_name != name {
                            let encoded = js_sys::encode_uri_component(&new_name);
                            nav(&format!("/tag/{encoded}/edit"), Default::default());
                        }
                    }
                    Err(e) => {
                        set_error_msg.set(Some(format_api_error(&e)));
                        set_submitting.set(false);
                    }
                }
            });
        }
    };

    view! {
        <Title text=move || if is_create() { "Create Tag".to_string() } else { format!("Edit Tag — {}", tag_name()) } />
        <div class="content-wrapper">
            <h1>{move || if is_create() { "Create Tag".to_string() } else { format!("Edit Tag \u{2014} {}", tag_name()) }}</h1>
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Tag not found."</p> })}
            <form
                class="form-grid"
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}
                {move || success_msg.get().map(|msg| view! { <p class="success">{msg}</p> })}

                <div class="form-row">
                    <label for="tag-names">"Names (comma-separated)"</label>
                    <input
                        id="tag-names"
                        type="text"
                        prop:value=move || names_str.get()
                        on:input=move |ev| set_names_str.set(event_target_value(&ev))
                        disabled=move || submitting.get()
                        placeholder="e.g. landscape, scenery"
                    />
                </div>

                <div class="form-row">
                    <label for="tag-category">"Category"</label>
                    <select
                        id="tag-category"
                        prop:value=move || category.get()
                        on:change=move |ev| set_category.set(event_target_value(&ev))
                        disabled=move || submitting.get()
                    >
                        {move || categories.get().into_iter().map(|cat| {
                            let name = cat.name.clone();
                            let val = name.clone();
                            view! { <option value=val>{name}</option> }
                        }).collect_view()}
                    </select>
                </div>

                <div class="form-row">
                    <label for="tag-desc">"Description"</label>
                    <textarea
                        id="tag-desc"
                        prop:value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        rows=4
                        disabled=move || submitting.get()
                    />
                </div>

                <div class="form-row">
                    <TagInput tags=implications label="Implications" />
                </div>

                <div class="form-row">
                    <TagInput tags=suggestions label="Suggestions" />
                </div>

                <div class="form-row buttons">
                    <button type="submit" disabled=move || submitting.get()>
                        {move || {
                            if submitting.get() {
                                "Saving..."
                            } else if is_create() {
                                "Create tag"
                            } else {
                                "Save changes"
                            }
                        }}
                    </button>
                    {move || (!is_create()).then(|| {
                        let name = tag_name();
                        let encoded = js_sys::encode_uri_component(&name).as_string().unwrap_or_default();
                        view! { <a href=format!("/tag/{encoded}")>"Back to tag"</a> }
                    })}
                </div>
            </form>
        </div>
    }
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
