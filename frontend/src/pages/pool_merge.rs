use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::request::MergeBody;

use crate::api::{ApiClient, ApiError};

#[component]
pub fn PoolMergePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_params_map();
    let navigate = use_navigate();

    let pool_id = move || params.get().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);

    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);
    let (source_version, set_source_version) = signal(String::new());

    let (target_id, set_target_id) = signal(String::new());
    let (target_version, set_target_version) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    Effect::new(move || {
        let client = api.get_untracked();
        let id = pool_id();
        leptos::task::spawn_local(async move {
            match client.get_pool(id).await {
                Ok(pool) => {
                    set_source_version.set(pool.version.unwrap_or_default());
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
        if let Ok(id) = target_id.get_untracked().trim().parse::<i64>() {
            let client = api.get_untracked();
            leptos::task::spawn_local(async move {
                if let Ok(pool) = client.get_pool(id).await {
                    set_target_version.set(pool.version.unwrap_or_default());
                }
            });
        }
    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let source = pool_id();
        let Ok(target) = target_id.get_untracked().trim().parse::<i64>() else {
            set_error_msg.set(Some("Invalid target pool ID.".into()));
            return;
        };
        if target == source {
            set_error_msg.set(Some("Cannot merge a pool with itself.".into()));
            return;
        }
        let tv = target_version.get_untracked();
        if tv.is_empty() {
            set_error_msg.set(Some("Loading target pool version...".into()));
            load_target();
            return;
        }

        let body = MergeBody {
            remove: source,
            merge_to: target,
            remove_version: source_version.get_untracked(),
            merge_to_version: tv,
        };

        set_submitting.set(true);
        set_error_msg.set(None);
        let client = api.get_untracked();
        let nav = navigate.clone();

        leptos::task::spawn_local(async move {
            match client.merge_pools(&body).await {
                Ok(merged) => {
                    let id = merged.id.unwrap_or(target);
                    nav(&format!("/pool/{id}"), Default::default());
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text=move || format!("Merge Pool {}", pool_id()) />
        <div class="content-wrapper">
            <h1>{move || format!("Merge Pool #{}", pool_id())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Pool not found."</p> })}
            <form
                class="form-grid"
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}

                <div class="form-row">
                    <p>"This will merge pool #"{move || pool_id()}" into the target pool. The source pool will be deleted."</p>
                </div>

                <div class="form-row">
                    <label for="target-pool">"Target pool ID"</label>
                    <input
                        id="target-pool"
                        type="text"
                        prop:value=move || target_id.get()
                        on:input=move |ev| set_target_id.set(event_target_value(&ev))
                        on:blur=move |_| load_target()
                        disabled=move || submitting.get()
                        placeholder="Enter target pool ID"
                    />
                </div>

                <div class="form-row buttons">
                    <button type="submit" disabled=move || submitting.get()>
                        {move || if submitting.get() { "Merging..." } else { "Merge" }}
                    </button>
                    <a href=move || format!("/pool/{}", pool_id())>"Cancel"</a>
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
