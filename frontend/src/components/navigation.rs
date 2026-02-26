use leptos::prelude::*;
use leptos_router::components::A;
use wasm_bindgen::JsCast;

use crate::auth::AuthState;
use oxibooru_shared::info::PublicConfig;

#[component]
pub fn Navigation() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let server_config = expect_context::<RwSignal<Option<PublicConfig>>>();

    let is_logged_in = move || auth.current_user.get().is_some();

    let username =
        move || -> String { auth.current_user.get().and_then(|u| u.name).unwrap_or_default() };

    let avatar_url =
        move || -> Option<String> { auth.current_user.get().and_then(|u| u.avatar_url) };

    let site_name = move || -> String {
        server_config
            .get()
            .map(|c| c.name)
            .unwrap_or_else(|| "oxibooru".to_string())
    };

    let (menu_open, set_menu_open) = signal(false);
    let (user_dropdown_open, set_user_dropdown_open) = signal(false);

    let toggle_menu = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        set_menu_open.update(|v| *v = !*v);
    };

    let toggle_user_dropdown = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        set_user_dropdown_open.update(|v| *v = !*v);
    };

    // Close menu when any nav link is clicked
    let on_nav_click = move |ev: web_sys::MouseEvent| {
        if let Some(target) = ev.target() {
            let el: &web_sys::EventTarget = &target;
            if let Ok(el) = el.clone().dyn_into::<web_sys::HtmlElement>() {
                if el.tag_name() == "A" {
                    set_menu_open.set(false);
                    set_user_dropdown_open.set(false);
                }
            }
        }
    };

    // Close user dropdown when clicking elsewhere
    let on_body_click = move |_: web_sys::MouseEvent| {
        set_user_dropdown_open.set(false);
    };

    view! {
        <nav class="top-navigation" on:click=on_body_click>
            <button class="mobile-nav-toggle" on:click=toggle_menu>
                <span class="site-name">{move || site_name()}</span>
                <span class="toggle-icon"><i class="fa fa-bars" /></span>
            </button>
            <div class="nav-body" class:opened=move || menu_open.get() on:click=on_nav_click>
                <ul class="nav-links">
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
                    <li><A href="/help">"Help"</A></li>
                </ul>
                <div class="nav-right">
                    <span class="nav-settings"><A href="/settings"><i class="fa fa-cog" /></A></span>

                    // Logged in: avatar dropdown (desktop only), inline items on mobile
                    <Show when=move || is_logged_in()>
                        <div class="user-dropdown">
                            <button class="user-dropdown-toggle" on:click=toggle_user_dropdown>
                                {move || avatar_url().map(|url| view! {
                                    <span
                                        class="nav-avatar"
                                        style=format!("background-image: url('{url}')")
                                    />
                                })}
                                <span class="username">{move || username()}</span>
                                <i class="fa fa-caret-down" />
                            </button>
                            <ul
                                class="user-dropdown-menu"
                                style:display=move || if user_dropdown_open.get() { "" } else { "none" }
                            >
                                <li><A href=move || format!("/user/{}", username())>"Account"</A></li>
                                <li><A href="/register">"Register"</A></li>
                                <li><A href="/logout">"Logout"</A></li>
                            </ul>
                        </div>
                    </Show>

                    // Not logged in: Register + Login links
                    <Show when=move || !is_logged_in()>
                        <span class="nav-auth-link"><A href="/register">"Register"</A></span>
                        <span class="nav-auth-link"><A href="/login">"Log in"</A></span>
                    </Show>
                </div>
            </div>
        </nav>
    }
}
