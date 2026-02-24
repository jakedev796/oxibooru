use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::api::password_reset::PasswordResetTokenBody;
use crate::api::{ApiClient, ApiError};

/// Password reset request page — enter username/email to receive a reset link.
#[component]
pub fn PasswordResetPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();

    let (identifier, set_identifier) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error_msg.set(None);
        set_success_msg.set(None);

        let ident = identifier.get_untracked();
        let client = api.get_untracked();

        leptos::task::spawn_local(async move {
            match client.request_password_reset(&ident).await {
                Ok(()) => {
                    set_success_msg.set(Some("Password reset email sent. Check your inbox.".to_string()));
                    set_submitting.set(false);
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text="Password reset" />
        <div class="content-wrapper password-reset">
            <h1>"Password reset"</h1>
            <form on:submit=on_submit>
                <ul class="input">
                    <li>
                        <label for="user-name">"User name or email"</label>
                        <input
                            type="text"
                            id="user-name"
                            required
                            prop:value=move || identifier.get()
                            on:input=move |ev| set_identifier.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        />
                    </li>
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
                    <input type="submit" value="Reset password" disabled=move || submitting.get() />
                </div>
            </form>
        </div>
    }
}

/// Password reset confirmation page — receives token from URL, shows new password.
#[component]
pub fn PasswordResetConfirmPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_params_map();

    let (new_password, set_new_password) = signal(Option::<String>::None);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(true);

    // Parse the token param which is "username:token" (URL-encoded as "username%3Atoken")
    Effect::new(move || {
        let token_param = params.get().get("token");
        let Some(raw) = token_param else {
            set_error_msg.set(Some("Missing reset token.".to_string()));
            set_loading.set(false);
            return;
        };

        // Decode URI component
        let decoded = js_sys::decode_uri_component(&raw)
            .map(String::from)
            .unwrap_or(raw.clone());

        // Split on ":" to get (identifier, token)
        let Some((identifier, token)) = decoded.split_once(':') else {
            set_error_msg.set(Some("Invalid reset token format.".to_string()));
            set_loading.set(false);
            return;
        };

        let identifier = identifier.to_string();
        let token = token.to_string();
        let client = api.get_untracked();

        leptos::task::spawn_local(async move {
            let body = PasswordResetTokenBody { token };
            match client.confirm_password_reset(&identifier, &body).await {
                Ok(resp) => {
                    set_new_password.set(Some(resp.password));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <Title text="Password reset" />
        <div class="content-wrapper password-reset">
            <h1>"Password reset"</h1>
            {move || {
                if loading.get() {
                    view! { <p>"Processing reset token…"</p> }.into_any()
                } else if let Some(pass) = new_password.get() {
                    view! {
                        <div class="messages">
                            <div class="message success">
                                "Your new password is: "
                                <code>{pass}</code>
                                <br />
                                "Please log in and change it."
                            </div>
                        </div>
                        <div class="buttons">
                            <a href="/login">"Log in"</a>
                        </div>
                    }.into_any()
                } else if let Some(msg) = error_msg.get() {
                    view! {
                        <div class="messages">
                            <div class="message error">{msg}</div>
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
