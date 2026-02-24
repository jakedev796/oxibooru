use leptos::prelude::*;
use leptos_router::components::A;

use crate::auth::AuthState;

#[component]
pub fn Navigation() -> impl IntoView {
    let auth = expect_context::<AuthState>();

    let is_logged_in = move || auth.current_user.get().is_some();

    let username = move || -> String { auth.current_user.get().and_then(|u| u.name).unwrap_or_default() };

    view! {
        <nav class="top-navigation">
            <ul>
                <li><A href="/">"Home"</A></li>
                <li class=move || if !auth.has_privilege("post_list") { "hidden" } else { "" }>
                    <A href="/posts">"Posts"</A>
                </li>
                <li class=move || if !auth.has_privilege("post_create_identified") && !auth.has_privilege("post_create_anonymous") { "hidden" } else { "" }>
                    <A href="/upload">"Upload"</A>
                </li>
                <li class=move || if !auth.has_privilege("comment_list") { "hidden" } else { "" }>
                    <A href="/comments">"Comments"</A>
                </li>
                <li class=move || if !auth.has_privilege("tag_list") { "hidden" } else { "" }>
                    <A href="/tags">"Tags"</A>
                </li>
                <li class=move || if !auth.has_privilege("pool_list") { "hidden" } else { "" }>
                    <A href="/pools">"Pools"</A>
                </li>
                <li class=move || if !auth.has_privilege("user_list") { "hidden" } else { "" }>
                    <A href="/users">"Users"</A>
                </li>
                <li class=move || if !is_logged_in() { "hidden" } else { "" }>
                    <A href=move || format!("/user/{}", username())>"Account"</A>
                </li>
                <li class=move || if is_logged_in() { "hidden" } else { "" }>
                    <A href="/register">"Register"</A>
                </li>
                <li class=move || if is_logged_in() { "hidden" } else { "" }>
                    <A href="/login">"Log in"</A>
                </li>
                <li class=move || if !is_logged_in() { "hidden" } else { "" }>
                    <A href="/logout">"Logout"</A>
                </li>
                <li><A href="/help">"Help"</A></li>
            </ul>
        </nav>
    }
}
