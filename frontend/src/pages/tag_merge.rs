use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::request::MergeBody;

use crate::api::{ApiClient, ApiError};
use crate::auth::AuthState;
use crate::components::tag_autocomplete::TagAutocomplete;
use crate::settings::SettingsState;
use crate::tag_cache::TagCache;

#[component]
pub fn TagMergePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let settings = expect_context::<SettingsState>();
    let tag_cache = expect_context::<TagCache>();
    let params = use_params_map();
    let navigate = use_navigate();

    let source_name = move || params.get().get("name").unwrap_or_default();

    // Loading state
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);
    let (source_version, set_source_version) = signal(String::new());

    // Form signals
    let (target_name, set_target_name) = signal(String::new());
    let (target_version, set_target_version) = signal(String::new());
    let (confirmed, set_confirmed) = signal(false);
    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    // Load source tag version
    Effect::new(move || {
        let name = source_name();

        // Use cached tag data if available
        if let Some(cached) = tag_cache.get(&name) {
            set_source_version.set(cached.version.unwrap_or_default());
            set_loading.set(false);
        }

        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            match client.get_tag(&name).await {
                Ok(tag) => {
                    tag_cache.set(&name, tag.clone());
                    set_source_version.set(tag.version.unwrap_or_default());
                    set_loading.set(false);
                }
                Err(_) => {
                    if loading.get_untracked() {
                        set_load_error.set(true);
                        set_loading.set(false);
                    }
                }
            }
        });
    });

    // When a tag is selected via autocomplete, load its version from the API
    let on_tag_select = Callback::new(move |(name, _category): (String, String)| {
        set_target_name.set(name.clone());
        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            if let Ok(tag) = client.get_tag(&name).await {
                set_target_version.set(tag.version.unwrap_or_default());
            }
        });
    });

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let src = source_name();
        let tgt = target_name.get_untracked();
        if tgt.trim().is_empty() {
            set_error_msg.set(Some("Target tag is required.".into()));
            return;
        }
        if tgt.trim() == src {
            set_error_msg.set(Some("Cannot merge a tag with itself.".into()));
            return;
        }
        if !confirmed.get_untracked() {
            set_error_msg.set(Some("Please confirm that you want to merge this tag.".into()));
            return;
        }

        let tv = target_version.get_untracked();
        if tv.is_empty() {
            set_error_msg.set(Some("Loading target tag version, please try again...".into()));
            // Try to load target version
            let name = tgt.clone();
            let client = api.get_untracked();
            leptos::task::spawn_local(async move {
                if let Ok(tag) = client.get_tag(&name).await {
                    set_target_version.set(tag.version.unwrap_or_default());
                }
            });
            return;
        }

        let body = MergeBody {
            remove: src.clone(),
            merge_to: tgt.trim().to_string(),
            remove_version: source_version.get_untracked(),
            merge_to_version: tv,
        };

        set_submitting.set(true);
        set_error_msg.set(None);
        let client = api.get_untracked();
        let nav = navigate.clone();

        leptos::task::spawn_local(async move {
            match client.merge_tags(&body).await {
                Ok(merged) => {
                    let name = merged.names.and_then(|n| n.into_iter().next()).unwrap_or_default();
                    let encoded = js_sys::encode_uri_component(&name);
                    nav(&format!("/tag/{encoded}"), Default::default());
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    // Tab navigation privileges
    let can_edit = Memo::new(move |_| {
        auth.has_privilege("tag_edit_name")
            || auth.has_privilege("tag_edit_category")
            || auth.has_privilege("tag_edit_description")
            || auth.has_privilege("tag_edit_implication")
            || auth.has_privilege("tag_edit_suggestion")
    });
    let can_merge = Memo::new(move |_| auth.has_privilege("tag_merge"));
    let can_delete = Memo::new(move |_| auth.has_privilege("tag_delete"));

    view! {
        <Title text=move || format!("Merge Tag — {}", source_name()) />
        <div class="tag-view-page">
        <div class="tag-view">
            {move || {
                let name = source_name();
                let display = settings.display_name(&name);
                let summary_href = format!("/tag/{name}");
                let edit_href = format!("/tag/{name}/edit");
                let merge_href = format!("/tag/{name}/merge");
                let delete_href = format!("/tag/{name}/delete");
                view! {
                    <h1>{display}</h1>
                    <nav class="buttons">
                        <ul>
                            <li><a href=summary_href>"Summary"</a></li>
                            {move || can_edit.get().then(|| view! {
                                <li><a href=edit_href.clone()>"Edit"</a></li>
                            })}
                            {move || can_merge.get().then(|| view! {
                                <li class="active"><a href=merge_href.clone()>"Merge with\u{2026}"</a></li>
                            })}
                            {move || can_delete.get().then(|| view! {
                                <li><a href=delete_href.clone()>"Delete"</a></li>
                            })}
                        </ul>
                    </nav>
                }
            }}

            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Tag not found."</p> })}

            <form
                class="tag-edit"
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}

                <div class="form-row">
                    <label>"Target tag"</label>
                    <TagAutocomplete on_select=on_tag_select placeholder="type to add..." />
                    {move || {
                        let name = target_name.get();
                        (!name.is_empty()).then(|| {
                            let display = settings.display_name(&name);
                            view! {
                                <div class="tag-chips">
                                    <span class="tag-chip">
                                        {display}
                                        <button type="button" class="chip-remove" on:click=move |_| {
                                            set_target_name.set(String::new());
                                            set_target_version.set(String::new());
                                        }>
                                            <i class="fa fa-remove" />
                                        </button>
                                    </span>
                                </div>
                            }
                        })
                    }}
                </div>

                <p class="merge-info" style="margin-top: 1.5em; margin-bottom: 1.5em;">
                    "Usages in posts, suggestions and implications will be merged. Category needs to be handled manually."
                </p>

                <ul class="input">
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || confirmed.get()
                                on:change=move |ev| set_confirmed.set(event_target_checked(&ev))
                                disabled=move || submitting.get()
                            />
                            <span class="checkbox">"I confirm that I want to merge this tag."</span>
                        </label>
                    </li>
                </ul>

                <div class="form-row buttons">
                    <button
                        type="submit"
                        disabled=move || submitting.get() || !confirmed.get()
                    >
                        {move || if submitting.get() { "Merging..." } else { "Merge tag" }}
                    </button>
                </div>
            </form>
        </div>
        </div>
    }
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
