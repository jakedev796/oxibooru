use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::navigation::Navigation;
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

    // Server config context — used by registration, password reset, etc.
    let server_config: RwSignal<Option<PublicConfig>> = RwSignal::new(None);
    provide_context(server_config);

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

    view! {
        <Title formatter=|text| format!("{text} — oxibooru") />
        <Router>
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
