use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use crate::auth::AuthState;

#[component]
pub fn LogoutPage() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let navigate = use_navigate();

    // Logout immediately on mount
    auth.logout();
    navigate("/", Default::default());

    view! {
        <Title text="Log out" />
        <div class="content-wrapper">
            <p>"Logging out…"</p>
        </div>
    }
}
