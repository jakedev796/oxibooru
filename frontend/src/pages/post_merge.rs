use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::api::posts::PostMergeBody;
use crate::api::{ApiClient, ApiError};

#[component]
pub fn PostMergePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_params_map();
    let navigate = use_navigate();

    let post_id = move || {
        params
            .get()
            .get("id")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };

    // Loading state for source post version
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);
    let (source_version, set_source_version) = signal(String::new());

    // Form signals
    let (target_id, set_target_id) = signal(String::new());
    let (target_version, set_target_version) = signal(String::new());
    let (replace_content, set_replace_content) = signal(false);
    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    // Load source post to get version
    Effect::new(move || {
        let client = api.get_untracked();
        let id = post_id();
        leptos::task::spawn_local(async move {
            match client.get_post(id).await {
                Ok(post) => {
                    set_source_version.set(post.version.unwrap_or_default());
                    set_loading.set(false);
                }
                Err(_) => {
                    set_load_error.set(true);
                    set_loading.set(false);
                }
            }
        });
    });

    // Load target post version when target_id changes
    let load_target = move || {
        let id_str = target_id.get_untracked();
        let id = id_str.trim().parse::<i64>().ok();
        if let Some(id) = id {
            let client = api.get_untracked();
            leptos::task::spawn_local(async move {
                if let Ok(post) = client.get_post(id).await {
                    set_target_version.set(post.version.unwrap_or_default());
                }
            });
        }
    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let source_id = post_id();
        let target = target_id.get_untracked().trim().parse::<i64>();
        let Ok(target_post_id) = target else {
            set_error_msg.set(Some("Invalid target post ID.".into()));
            return;
        };
        if target_post_id == source_id {
            set_error_msg.set(Some("Cannot merge a post with itself.".into()));
            return;
        }

        // Load target version if not already loaded
        let tv = target_version.get_untracked();
        if tv.is_empty() {
            set_error_msg.set(Some("Loading target post version...".into()));
            load_target();
            return;
        }

        let body = PostMergeBody {
            remove: source_id,
            merge_to: target_post_id,
            remove_version: source_version.get_untracked(),
            merge_to_version: tv,
            replace_content: replace_content.get_untracked(),
        };

        set_submitting.set(true);
        set_error_msg.set(None);
        let client = api.get_untracked();
        let nav = navigate.clone();

        leptos::task::spawn_local(async move {
            match client.merge_posts(&body).await {
                Ok(merged) => {
                    let id = merged.id.unwrap_or(target_post_id);
                    nav(&format!("/post/{id}"), Default::default());
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text=move || format!("Merge Post {}", post_id()) />
        <div class="content-wrapper">
            <h1>{move || format!("Merge Post #{}", post_id())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Post not found."</p> })}
            <form
                class="form-grid"
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}

                <div class="form-row">
                    <p>"This will merge post #"{move || post_id()}" into the target post. The source post will be deleted."</p>
                </div>

                <div class="form-row">
                    <label for="target-id">"Target post ID"</label>
                    <input
                        id="target-id"
                        type="text"
                        prop:value=move || target_id.get()
                        on:input=move |ev| set_target_id.set(event_target_value(&ev))
                        on:blur=move |_| load_target()
                        disabled=move || submitting.get()
                        placeholder="Enter target post ID"
                    />
                </div>

                <div class="form-row">
                    <label>
                        <input
                            type="checkbox"
                            prop:checked=move || replace_content.get()
                            on:change=move |ev| set_replace_content.set(event_target_checked(&ev))
                        />
                        " Replace target content with source content"
                    </label>
                </div>

                <div class="form-row buttons">
                    <button type="submit" disabled=move || submitting.get()>
                        {move || if submitting.get() { "Merging..." } else { "Merge" }}
                    </button>
                    <a href=move || format!("/post/{}", post_id())>"Cancel"</a>
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
