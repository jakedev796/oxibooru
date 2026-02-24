use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::request::MergeBody;

use crate::api::{ApiClient, ApiError};

#[component]
pub fn TagMergePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
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
    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    // Load source tag version
    Effect::new(move || {
        let client = api.get_untracked();
        let name = source_name();
        leptos::task::spawn_local(async move {
            match client.get_tag(&name).await {
                Ok(tag) => {
                    set_source_version.set(tag.version.unwrap_or_default());
                    set_loading.set(false);
                }
                Err(_) => {
                    set_load_error.set(true);
                    set_loading.set(false);
                }
            }
        });
    });

    let load_target = move || {
        let name = target_name.get_untracked();
        if !name.trim().is_empty() {
            let client = api.get_untracked();
            leptos::task::spawn_local(async move {
                if let Ok(tag) = client.get_tag(&name).await {
                    set_target_version.set(tag.version.unwrap_or_default());
                }
            });
        }
    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let src = source_name();
        let tgt = target_name.get_untracked();
        if tgt.trim().is_empty() {
            set_error_msg.set(Some("Target tag name is required.".into()));
            return;
        }
        if tgt.trim() == src {
            set_error_msg.set(Some("Cannot merge a tag with itself.".into()));
            return;
        }

        let tv = target_version.get_untracked();
        if tv.is_empty() {
            set_error_msg.set(Some("Loading target tag version...".into()));
            load_target();
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
                    let name = merged
                        .names
                        .and_then(|n| n.into_iter().next())
                        .unwrap_or_default();
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

    view! {
        <Title text=move || format!("Merge Tag — {}", source_name()) />
        <div class="content-wrapper">
            <h1>{move || format!("Merge Tag \u{2014} {}", source_name())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Tag not found."</p> })}
            <form
                class="form-grid"
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}

                <div class="form-row">
                    <p>"This will merge tag \""<strong>{source_name()}</strong>"\" into the target tag. The source tag will be deleted."</p>
                </div>

                <div class="form-row">
                    <label for="target-tag">"Target tag name"</label>
                    <input
                        id="target-tag"
                        type="text"
                        prop:value=move || target_name.get()
                        on:input=move |ev| set_target_name.set(event_target_value(&ev))
                        on:blur=move |_| load_target()
                        disabled=move || submitting.get()
                        placeholder="Enter target tag name"
                    />
                </div>

                <div class="form-row buttons">
                    <button type="submit" disabled=move || submitting.get()>
                        {move || if submitting.get() { "Merging..." } else { "Merge" }}
                    </button>
                    {move || {
                        let name = source_name();
                        let encoded = js_sys::encode_uri_component(&name).as_string().unwrap_or_default();
                        view! { <a href=format!("/tag/{encoded}")>"Cancel"</a> }
                    }}
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
