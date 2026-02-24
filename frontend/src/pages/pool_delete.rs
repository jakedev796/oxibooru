use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::request::DeleteBody;

use crate::api::{ApiClient, ApiError};

#[component]
pub fn PoolDeletePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_params_map();
    let navigate = use_navigate();

    let pool_id = move || {
        params
            .get()
            .get("id")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };

    let (version, set_version) = signal(String::new());
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);
    let (pool_name, set_pool_name) = signal(String::new());

    Effect::new(move || {
        let client = api.get_untracked();
        let id = pool_id();
        leptos::task::spawn_local(async move {
            match client.get_pool(id).await {
                Ok(pool) => {
                    set_version.set(pool.version.unwrap_or_default());
                    set_pool_name.set(
                        pool.names
                            .and_then(|n| n.into_iter().next())
                            .unwrap_or_else(|| format!("#{id}")),
                    );
                    set_loading.set(false);
                }
                Err(_) => {
                    set_load_error.set(true);
                    set_loading.set(false);
                }
            }
        });
    });

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error_msg.set(None);

        let id = pool_id();
        let body = DeleteBody {
            version: version.get_untracked(),
        };
        let client = api.get_untracked();
        let nav = navigate.clone();

        leptos::task::spawn_local(async move {
            match client.delete_pool(id, &body).await {
                Ok(()) => {
                    nav("/pools", Default::default());
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text=move || format!("Delete Pool {}", pool_id()) />
        <div class="content-wrapper">
            <h1>{move || format!("Delete Pool \u{2014} {}", pool_name.get())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Pool not found."</p> })}
            <form
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                <p>
                    "Are you sure you want to delete pool "
                    <strong>{move || pool_name.get()}</strong>
                    "? This cannot be undone."
                </p>
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}
                <div class="buttons">
                    <button type="submit" class="btn-danger" disabled=move || submitting.get()>
                        "Delete pool"
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
