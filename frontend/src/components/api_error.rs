use leptos::prelude::*;

use crate::api::ApiError;

#[component]
pub fn ApiErrorMessage(error: ApiError) -> impl IntoView {
    let message = error.user_message();
    let is_auth = error.is_auth_error();
    view! {
        <div class="messages">
            <div class="message error">
                <p>{message}</p>
                {is_auth.then(|| view! {
                    <p><a href="/login">"Log in to continue"</a></p>
                })}
            </div>
        </div>
    }
}
