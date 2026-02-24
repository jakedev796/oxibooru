use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::category_styles::CategoryStyles;
use crate::components::loading_bar::{LoadingBar, LoadingState};
use crate::components::navigation::Navigation;
use crate::keyboard::KeyboardShortcuts;
use oxibooru_shared::category::{PoolCategoryInfo, TagCategoryInfo};
use oxibooru_shared::info::PublicConfig;
use crate::pages::comments_page::CommentsPage;
use crate::pages::help::HelpPage;
use crate::pages::history::HistoryPage;
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::logout::LogoutPage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::password_reset::{PasswordResetPage, PasswordResetConfirmPage};
use crate::pages::pool_categories::PoolCategoriesPage;
use crate::pages::pool_create::PoolCreatePage;
use crate::pages::pool_delete::PoolDeletePage;
use crate::pages::pool_edit::PoolEditPage;
use crate::pages::pool_list::PoolListPage;
use crate::pages::pool_merge::PoolMergePage;
use crate::pages::pool_view::PoolViewPage;
use crate::pages::post_edit::PostEditPage;
use crate::pages::post_list::PostListPage;
use crate::pages::post_merge::PostMergePage;
use crate::pages::post_upload::PostUploadPage;
use crate::pages::post_view::PostViewPage;
use crate::pages::register::RegisterPage;
use crate::pages::tag_categories::TagCategoriesPage;
use crate::pages::tag_delete::TagDeletePage;
use crate::pages::tag_edit::TagEditPage;
use crate::pages::tag_list::TagListPage;
use crate::pages::tag_merge::TagMergePage;
use crate::pages::tag_view::TagViewPage;
use crate::pages::settings::SettingsPage;
use crate::pages::user_delete::UserDeletePage;
use crate::pages::user_edit::UserEditPage;
use crate::pages::user_list::UserListPage;
use crate::pages::user_tokens::UserTokensPage;
use crate::pages::user_view::UserViewPage;
use crate::settings::SettingsState;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Create the API client (relative URL — Trunk proxy handles /api/)
    let api = RwSignal::new(ApiClient::new("/api"));
    provide_context(api);

    // Create auth state
    let auth = AuthState::new(api);
    provide_context(auth);

    // Create settings state
    let settings = SettingsState::new();
    provide_context(settings);

    // Loading state
    let loading = LoadingState::new();
    provide_context(loading);

    // Dark theme: toggle body class based on settings
    Effect::new(move || {
        let dark = settings.inner.with(|s| s.dark_theme);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = doc.body() {
                let class_list = body.class_list();
                if dark {
                    let _ = class_list.add_1("darktheme");
                } else {
                    let _ = class_list.remove_1("darktheme");
                }
            }
        }
    });

    // Keyboard shortcuts
    let shortcuts = KeyboardShortcuts::new();
    provide_context(shortcuts);

    // Register global shortcut: Q to focus search input
    shortcuts.register("q", Callback::new(move |()| {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.get_element_by_id("search-input") {
                if let Ok(input) = el.dyn_into::<web_sys::HtmlElement>() {
                    let _ = input.focus();
                }
            }
        }
    }));

    // Global keydown listener
    if let Some(window) = web_sys::window() {
        let handler = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            // Check if keyboard shortcuts are enabled
            if !settings.inner.with_untracked(|s| s.keyboard_shortcuts) {
                return;
            }

            // Skip when focus is on an input element
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(active) = doc.active_element() {
                    let tag = active.tag_name().to_uppercase();
                    if tag == "INPUT" || tag == "TEXTAREA" || tag == "SELECT" {
                        return;
                    }
                }
            }

            // Build key string
            let key = ev.key();
            let lookup = if ev.ctrl_key() || ev.meta_key() {
                format!("ctrl+{}", key.to_lowercase())
            } else {
                key
            };

            if shortcuts.dispatch(&lookup) {
                ev.prevent_default();
            }
        });
        let _ = window.add_event_listener_with_callback(
            "keydown",
            handler.as_ref().unchecked_ref(),
        );
        handler.forget();
    }

    // Server config context — used by registration, password reset, etc.
    let server_config: RwSignal<Option<PublicConfig>> = RwSignal::new(None);
    provide_context(server_config);

    // Category color contexts — populated on startup for dynamic CSS
    let tag_categories: RwSignal<Vec<TagCategoryInfo>> = RwSignal::new(Vec::new());
    provide_context(tag_categories);
    let pool_categories: RwSignal<Vec<PoolCategoryInfo>> = RwSignal::new(Vec::new());
    provide_context(pool_categories);

    // Fetch initial server info to populate privileges and config
    let initial_info = LocalResource::new(move || {
        let client = api.get();
        async move { client.get_info().await.ok() }
    });

    // When info loads, update auth state with privileges and server config
    Effect::new(move || {
        if let Some(Some(info)) = initial_info.get() {
            auth.privileges
                .set(Some(info.config.privileges.clone()));
            server_config.set(Some(info.config.clone()));
            // Verify stored credentials and populate current_user
            leptos::task::spawn_local(async move {
                auth.verify_session().await;
            });
        }
    });

    // Fetch tag and pool categories for dynamic CSS colors
    leptos::task::spawn_local(async move {
        let client = api.get_untracked();
        if let Ok(data) = client.get_tag_categories().await {
            tag_categories.set(data.results);
        }
        if let Ok(data) = client.get_pool_categories().await {
            pool_categories.set(data.results);
        }
    });

    view! {
        <Title formatter=|text| format!("{text} — oxibooru") />
        <Router>
            <CategoryStyles />
            <LoadingBar />
            <Navigation />
            <main>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    // Home
                    <Route path=path!("/") view=HomePage />

                    // Posts
                    <Route path=path!("/posts") view=PostListPage />
                    <Route path=path!("/post/:id") view=PostViewPage />
                    <Route path=path!("/post/:id/edit") view=PostEditPage />
                    <Route path=path!("/post/:id/merge") view=PostMergePage />
                    <Route path=path!("/upload") view=PostUploadPage />

                    // Tags
                    <Route path=path!("/tags") view=TagListPage />
                    <Route path=path!("/tag/:name") view=TagViewPage />
                    <Route path=path!("/tag/:name/edit") view=TagEditPage />
                    <Route path=path!("/tag/:name/merge") view=TagMergePage />
                    <Route path=path!("/tag/:name/delete") view=TagDeletePage />
                    <Route path=path!("/tag-categories") view=TagCategoriesPage />

                    // Pools
                    <Route path=path!("/pools") view=PoolListPage />
                    <Route path=path!("/pool/create") view=PoolCreatePage />
                    <Route path=path!("/pool/:id") view=PoolViewPage />
                    <Route path=path!("/pool/:id/edit") view=PoolEditPage />
                    <Route path=path!("/pool/:id/merge") view=PoolMergePage />
                    <Route path=path!("/pool/:id/delete") view=PoolDeletePage />
                    <Route path=path!("/pool-categories") view=PoolCategoriesPage />

                    // Users
                    <Route path=path!("/users") view=UserListPage />
                    <Route path=path!("/user/:name") view=UserViewPage />
                    <Route path=path!("/user/:name/edit") view=UserEditPage />
                    <Route path=path!("/user/:name/list-tokens") view=UserTokensPage />
                    <Route path=path!("/user/:name/delete") view=UserDeletePage />

                    // Auth
                    <Route path=path!("/login") view=LoginPage />
                    <Route path=path!("/logout") view=LogoutPage />
                    <Route path=path!("/register") view=RegisterPage />
                    <Route path=path!("/password-reset") view=PasswordResetPage />
                    <Route path=path!("/password-reset/:token") view=PasswordResetConfirmPage />

                    // Comments
                    <Route path=path!("/comments") view=CommentsPage />

                    // History
                    <Route path=path!("/history") view=HistoryPage />

                    // Settings
                    <Route path=path!("/settings") view=SettingsPage />

                    // Help
                    <Route path=path!("/help") view=HelpPage />
                    <Route path=path!("/help/:section") view=HelpPage />
                    <Route path=path!("/help/:section/:subsection") view=HelpPage />
                </Routes>
            </main>
        </Router>
    }
}
