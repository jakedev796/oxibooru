use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::request::DeleteBody;

use crate::api::{ApiClient, ApiError};
use crate::auth::AuthState;

#[component]
pub fn UserDeletePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let params = use_params_map();
    let navigate = use_navigate();

    let username = move || params.get().get("name").unwrap_or_default();

    // Load user to get version for optimistic locking
    let (version, set_version) = signal(String::new());
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);

    Effect::new(move || {
        let client = api.get_untracked();
        let name = username();
        leptos::task::spawn_local(async move {
            match client.get_user(&name).await {
                Ok(user) => {
                    set_version.set(user.version.unwrap_or_default());
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

        let name = username();
        let body = DeleteBody {
            version: version.get_untracked(),
        };
        let client = api.get_untracked();
        let navigate = navigate.clone();
        let is_self = auth.current_user.get_untracked().and_then(|u| u.name).as_deref() == Some(&name);

        leptos::task::spawn_local(async move {
            match client.delete_user(&name, &body).await {
                Ok(()) => {
                    if is_self {
                        auth.logout();
                    }
                    navigate("/", Default::default());
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text=move || format!("Delete {}", username()) />
        <div class="content-wrapper user-delete">
            <h1>{move || format!("Delete user — {}", username())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading…"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"User not found."</p> })}
            <form
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                <p>
                    "Are you sure you want to delete user "
                    <strong>{username()}</strong>
                    "? This cannot be undone."
                </p>
                {move || error_msg.get().map(|msg| view! {
                    <div class="messages">
                        <div class="message error">{msg}</div>
                    </div>
                })}
                <div class="buttons">
                    <input
                        type="submit"
                        value="Delete account"
                        class="dangerous"
                        disabled=move || submitting.get()
                    />
                    <a href=format!("/user/{}", js_sys::encode_uri_component(&username()))>
                        "Cancel"
                    </a>
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
