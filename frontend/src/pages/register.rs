use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use oxibooru_shared::info::PublicConfig;

use crate::api::users::CreateUserBody;
use crate::api::{ApiClient, ApiError};
use crate::auth::AuthState;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let server_config = expect_context::<RwSignal<Option<PublicConfig>>>();
    let navigate = use_navigate();

    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_error_msg.set(None);

        let user = username.get_untracked();
        let pass = password.get_untracked();
        let mail = email.get_untracked();

        // Client-side regex validation
        if let Some(config) = server_config.get_untracked() {
            if let Some(err) = validate_regex(&user, &config.username_regex, "User name") {
                set_error_msg.set(Some(err));
                return;
            }
            if let Some(err) = validate_regex(&pass, &config.password_regex, "Password") {
                set_error_msg.set(Some(err));
                return;
            }
        }

        set_submitting.set(true);

        let body = CreateUserBody {
            name: user.clone(),
            password: pass.clone(),
            email: if mail.is_empty() { None } else { Some(mail) },
            rank: None,
            avatar_style: None,
        };

        let navigate = navigate.clone();
        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            match client.create_user(&body).await {
                Ok(_) => {
                    // Auto-login after registration
                    match auth.login(user, pass).await {
                        Ok(_) => navigate("/", Default::default()),
                        Err(_) => navigate("/login", Default::default()),
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
        <Title text="Register" />
        <div class="content-wrapper register">
            <h1>"Registration"</h1>
            <form on:submit=on_submit>
                <ul class="input">
                    <li>
                        <label for="user-name">"User name"</label>
                        <input
                            type="text"
                            id="user-name"
                            autocomplete="username"
                            required
                            prop:value=move || username.get()
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        />
                        {move || server_config.get().map(|c| view! {
                            <span class="hint">{format!("Regex: {}", c.username_regex)}</span>
                        })}
                    </li>
                    <li>
                        <label for="user-password">"Password"</label>
                        <input
                            type="password"
                            id="user-password"
                            autocomplete="new-password"
                            required
                            prop:value=move || password.get()
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        />
                        {move || server_config.get().map(|c| view! {
                            <span class="hint">{format!("Regex: {}", c.password_regex)}</span>
                        })}
                    </li>
                    <li>
                        <label for="user-email">"Email (optional)"</label>
                        <input
                            type="email"
                            id="user-email"
                            autocomplete="email"
                            prop:value=move || email.get()
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        />
                    </li>
                </ul>
                {move || error_msg.get().map(|msg| view! {
                    <div class="messages">
                        <div class="message error">{msg}</div>
                    </div>
                })}
                <div class="buttons">
                    <input type="submit" value="Create account" disabled=move || submitting.get() />
                </div>
            </form>
        </div>
    }
}

/// Validate a value against a regex using the browser's native RegExp.
fn validate_regex(value: &str, pattern: &str, field_name: &str) -> Option<String> {
    let re = js_sys::RegExp::new(pattern, "");
    if !re.test(value) {
        Some(format!("{field_name} doesn't satisfy regex: {pattern}"))
    } else {
        None
    }
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
