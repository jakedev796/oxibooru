use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::request::DeleteBody;

use crate::api::{ApiClient, ApiError};

#[component]
pub fn TagDeletePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_params_map();
    let navigate = use_navigate();

    let tag_name = move || params.get().get("name").unwrap_or_default();

    let (version, set_version) = signal(String::new());
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);

    Effect::new(move || {
        let client = api.get_untracked();
        let name = tag_name();
        leptos::task::spawn_local(async move {
            match client.get_tag(&name).await {
                Ok(tag) => {
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

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
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

    view! {
        <Title text=move || format!("Delete Tag — {}", tag_name()) />
        <div class="content-wrapper">
            <h1>{move || format!("Delete Tag \u{2014} {}", tag_name())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Tag not found."</p> })}
            <form
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                <p>
                    "Are you sure you want to delete tag "
                    <strong>{tag_name()}</strong>
                    "? This cannot be undone."
                </p>
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}
                <div class="buttons">
                    <button type="submit" class="btn-danger" disabled=move || submitting.get()>
                        "Delete tag"
                    </button>
                    {move || {
                        let name = tag_name();
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
