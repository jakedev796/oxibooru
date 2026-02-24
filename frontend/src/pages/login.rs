use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use crate::auth::AuthState;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let navigate = use_navigate();

    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error_msg.set(None);

        let user = username.get_untracked();
        let pass = password.get_untracked();
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            match auth.login(user, pass).await {
                Ok(_) => navigate("/", Default::default()),
                Err(e) => {
                    set_error_msg.set(Some(e.to_string()));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text="Log in" />
        <div class="content-wrapper login">
            <h1>"Log in"</h1>
            <form on:submit=on_submit>
                <ul class="input">
                    <li>
                        <label for="user-name">"User name"</label>
                        <input
                            type="text"
                            id="user-name"
                            autocomplete="username"
                            prop:value=move || username.get()
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            disabled=move || submitting.get()
                        />
                    </li>
                    <li>
                        <label for="user-password">"Password"</label>
                        <input
                            type="password"
                            id="user-password"
                            autocomplete="current-password"
                            prop:value=move || password.get()
                            on:input=move |ev| set_password.set(event_target_value(&ev))
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
                    <input type="submit" value="Log in" disabled=move || submitting.get() />
                    <a href="/password-reset">"Forgot password?"</a>
                </div>
            </form>
        </div>
    }
}
