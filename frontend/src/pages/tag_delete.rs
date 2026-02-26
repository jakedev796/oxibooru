use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::request::DeleteBody;

use crate::api::{ApiClient, ApiError};
use crate::auth::AuthState;
use crate::settings::SettingsState;
use crate::tag_cache::TagCache;

#[component]
pub fn TagDeletePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let settings = expect_context::<SettingsState>();
    let tag_cache = expect_context::<TagCache>();
    let params = use_params_map();
    let navigate = use_navigate();

    let tag_name = move || params.get().get("name").unwrap_or_default();

    let (version, set_version) = signal(String::new());
    let (usages, set_usages) = signal(0i64);
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);
    let (confirmed, set_confirmed) = signal(false);

    // Load tag data
    Effect::new(move || {
        let name = tag_name();

        // Use cached tag data if available
        if let Some(cached) = tag_cache.get(&name) {
            set_version.set(cached.version.unwrap_or_default());
            set_usages.set(cached.usages.unwrap_or(0));
            set_loading.set(false);
        }

        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            match client.get_tag(&name).await {
                Ok(tag) => {
                    tag_cache.set(&name, tag.clone());
                    set_version.set(tag.version.unwrap_or_default());
                    set_usages.set(tag.usages.unwrap_or(0));
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

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if !confirmed.get_untracked() {
            set_error_msg.set(Some("Please confirm that you want to delete this tag.".into()));
            return;
        }

        set_submitting.set(true);
        set_error_msg.set(None);

        let name = tag_name();
        let body = DeleteBody {
            version: version.get_untracked(),
        };
        let client = api.get_untracked();
        let nav = navigate.clone();

        leptos::task::spawn_local(async move {
            match client.delete_tag(&name, &body).await {
                Ok(()) => {
                    nav("/tags", Default::default());
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
        <Title text=move || format!("Delete Tag — {}", tag_name()) />
        <div class="tag-view-page">
        <div class="tag-view">
            {move || {
                let name = tag_name();
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
                                <li><a href=merge_href.clone()>"Merge with\u{2026}"</a></li>
                            })}
                            {move || can_delete.get().then(|| view! {
                                <li class="active"><a href=delete_href.clone()>"Delete"</a></li>
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

                <p style="margin-bottom: 1.5em;">
                    "This tag has "
                    <a href=move || format!("/posts?query={}", tag_name())>
                        {move || usages.get()}
                        " usage(s)"
                    </a>
                    "."
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
                            <span class="checkbox">"I confirm that I want to delete this tag."</span>
                        </label>
                    </li>
                </ul>

                <div class="form-row buttons">
                    <button
                        type="submit"
                        disabled=move || submitting.get() || !confirmed.get()
                    >
                        {move || if submitting.get() { "Deleting..." } else { "Delete tag" }}
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
