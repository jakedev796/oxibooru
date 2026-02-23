use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::enums::{AvatarStyle, UserRank};

use crate::api::users::UpdateUserBody;
use crate::api::{ApiClient, ApiError};
use crate::auth::AuthState;

#[component]
pub fn UserEditPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let params = use_params_map();
    let navigate = use_navigate();

    let username = move || {
        params
            .get()
            .get("name")
            .unwrap_or_default()
    };

    // Loading state
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);

    // Form field signals
    let (new_name, set_new_name) = signal(String::new());
    let (new_password, set_new_password) = signal(String::new());
    let (new_email, set_new_email) = signal(String::new());
    let (new_rank, set_new_rank) = signal(String::new());
    let (new_avatar_style, set_new_avatar_style) = signal(String::new());
    let (new_avatar_url, set_new_avatar_url) = signal(String::new());
    let (version, set_version) = signal(String::new());

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);

    // Load user data on mount
    Effect::new(move || {
        let client = api.get_untracked();
        let name = username();
        leptos::task::spawn_local(async move {
            match client.get_user(&name).await {
                Ok(user) => {
                    set_new_name.set(user.name.clone().unwrap_or_default());
                    set_new_email.set(
                        user.email
                            .as_ref()
                            .and_then(|e| e.value())
                            .and_then(|e| e.clone())
                            .unwrap_or_default(),
                    );
                    set_new_rank.set(
                        user.rank
                            .map(|r| format!("{:?}", r).to_lowercase())
                            .unwrap_or_default(),
                    );
                    set_new_avatar_style.set(
                        user.avatar_style
                            .map(|a| format!("{:?}", a).to_lowercase())
                            .unwrap_or_default(),
                    );
                    set_version.set(user.version.clone().unwrap_or_default());
                    set_loading.set(false);
                }
                Err(_) => {
                    set_load_error.set(true);
                    set_loading.set(false);
                }
            }
        });
    });

    let is_self = move || {
        let current = auth.current_user.get();
        let target = username();
        current.and_then(|u| u.name).as_deref() == Some(&target)
    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error_msg.set(None);
        set_success_msg.set(None);

        let name = username();
        let body = UpdateUserBody {
            version: version.get_untracked(),
            name: {
                let n = new_name.get_untracked();
                if n != name { Some(n) } else { None }
            },
            password: {
                let p = new_password.get_untracked();
                if p.is_empty() { None } else { Some(p) }
            },
            email: {
                let e = new_email.get_untracked();
                if e.is_empty() { Some(None) } else { Some(Some(e)) }
            },
            rank: parse_rank(&new_rank.get_untracked()),
            avatar_style: parse_avatar_style(&new_avatar_style.get_untracked()),
            avatar_url: {
                let url = new_avatar_url.get_untracked();
                if url.is_empty() { None } else { Some(url) }
            },
        };

        let client = api.get_untracked();
        let navigate = navigate.clone();
        let renamed = body.name.clone();
        leptos::task::spawn_local(async move {
            match client.update_user(&name, &body).await {
                Ok(updated) => {
                    set_version.set(updated.version.unwrap_or_default());
                    set_success_msg.set(Some("User updated.".to_string()));
                    set_submitting.set(false);
                    set_new_password.set(String::new());

                    if let Some(new_n) = renamed {
                        navigate(
                            &format!("/user/{}/edit", url_encode(&new_n)),
                            Default::default(),
                        );
                    }
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text=move || format!("Edit {}", username()) />
        <div class="content-wrapper user-edit">
            <h1>{move || format!("Edit user — {}", username())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading…"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"User not found."</p> })}
            <form
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                <ul class="input">
                    <li>
                        <label for="user-name">"User name"</label>
                        <input
                            type="text"
                            id="user-name"
                            prop:value=move || new_name.get()
                            on:input=move |ev| set_new_name.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        />
                    </li>
                    <li>
                        <label for="user-password">"New password (leave blank to keep)"</label>
                        <input
                            type="password"
                            id="user-password"
                            autocomplete="new-password"
                            prop:value=move || new_password.get()
                            on:input=move |ev| set_new_password.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        />
                    </li>
                    <li>
                        <label for="user-email">"Email"</label>
                        <input
                            type="email"
                            id="user-email"
                            prop:value=move || new_email.get()
                            on:input=move |ev| set_new_email.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        />
                    </li>
                    <li>
                        <label for="user-rank">"Rank"</label>
                        <select
                            id="user-rank"
                            prop:value=move || new_rank.get()
                            on:change=move |ev| set_new_rank.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        >
                            <option value="restricted">"Restricted"</option>
                            <option value="regular">"Regular"</option>
                            <option value="power">"Power"</option>
                            <option value="moderator">"Moderator"</option>
                            <option value="administrator">"Administrator"</option>
                        </select>
                    </li>
                    <li>
                        <label for="user-avatar-style">"Avatar style"</label>
                        <select
                            id="user-avatar-style"
                            prop:value=move || new_avatar_style.get()
                            on:change=move |ev| set_new_avatar_style.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        >
                            <option value="gravatar">"Gravatar"</option>
                            <option value="manual">"Manual"</option>
                        </select>
                    </li>
                    {move || (new_avatar_style.get() == "manual").then(|| view! {
                        <li>
                            <label for="user-avatar-url">"Avatar URL"</label>
                            <input
                                type="text"
                                id="user-avatar-url"
                                prop:value=move || new_avatar_url.get()
                                on:input=move |ev| set_new_avatar_url.set(event_target_value(&ev))
                                disabled=move || submitting.get()
                            />
                        </li>
                    })}
                </ul>
                {move || error_msg.get().map(|msg| view! {
                    <div class="messages">
                        <div class="message error">{msg}</div>
                    </div>
                })}
                {move || success_msg.get().map(|msg| view! {
                    <div class="messages">
                        <div class="message success">{msg}</div>
                    </div>
                })}
                <div class="buttons">
                    <input type="submit" value="Save changes" disabled=move || submitting.get() />
                    {move || is_self().then(|| view! {
                        <a href=format!("/user/{}/delete", url_encode(&username()))>"Delete account"</a>
                    })}
                </div>
            </form>
        </div>
    }
}

fn parse_rank(s: &str) -> Option<UserRank> {
    match s {
        "restricted" => Some(UserRank::Restricted),
        "regular" => Some(UserRank::Regular),
        "power" => Some(UserRank::Power),
        "moderator" => Some(UserRank::Moderator),
        "administrator" => Some(UserRank::Administrator),
        _ => None,
    }
}

fn parse_avatar_style(s: &str) -> Option<AvatarStyle> {
    match s {
        "gravatar" => Some(AvatarStyle::Gravatar),
        "manual" => Some(AvatarStyle::Manual),
        _ => None,
    }
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
