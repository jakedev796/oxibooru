use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::navigation::Navigation;
use crate::pages::comments_page::CommentsPage;
use crate::pages::help::HelpPage;
use crate::pages::history::HistoryPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::pool_list::PoolListPage;
use crate::pages::pool_view::PoolViewPage;
use crate::pages::post_list::PostListPage;
use crate::pages::post_view::PostViewPage;
use crate::pages::tag_list::TagListPage;
use crate::pages::tag_view::TagViewPage;
use crate::pages::user_list::UserListPage;
use crate::pages::user_view::UserViewPage;
use crate::settings::SettingsState;

/// Placeholder component for routes not yet implemented.
#[component]
fn Todo(#[prop(into)] name: String) -> impl IntoView {
    view! {
        <Title text=name.clone() />
        <div class="placeholder-page">
            <h1>{name.clone()}</h1>
            <p>"This page is not yet implemented."</p>
        </div>
    }
}

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

    // Fetch initial server info to populate privileges
    let initial_info = LocalResource::new(move || {
        let client = api.get();
        async move { client.get_info().await.ok() }
    });

    // When info loads, update auth state with privileges
    Effect::new(move || {
        if let Some(Some(info)) = initial_info.get() {
            auth.privileges
                .set(Some(info.config.privileges.clone()));
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
                    <Route path=path!("/post/:id/edit") view=|| view! { <Todo name="Edit Post" /> } />
                    <Route path=path!("/post/:id/merge") view=|| view! { <Todo name="Merge Post" /> } />
                    <Route path=path!("/upload") view=|| view! { <Todo name="Upload" /> } />

                    // Tags
                    <Route path=path!("/tags") view=TagListPage />
                    <Route path=path!("/tag/:name") view=TagViewPage />
                    <Route path=path!("/tag/:name/edit") view=|| view! { <Todo name="Edit Tag" /> } />
                    <Route path=path!("/tag/:name/merge") view=|| view! { <Todo name="Merge Tag" /> } />
                    <Route path=path!("/tag/:name/delete") view=|| view! { <Todo name="Delete Tag" /> } />
                    <Route path=path!("/tag-categories") view=|| view! { <Todo name="Tag Categories" /> } />

                    // Pools
                    <Route path=path!("/pools") view=PoolListPage />
                    <Route path=path!("/pool/create") view=|| view! { <Todo name="Create Pool" /> } />
                    <Route path=path!("/pool/:id") view=PoolViewPage />
                    <Route path=path!("/pool/:id/edit") view=|| view! { <Todo name="Edit Pool" /> } />
                    <Route path=path!("/pool/:id/merge") view=|| view! { <Todo name="Merge Pool" /> } />
                    <Route path=path!("/pool/:id/delete") view=|| view! { <Todo name="Delete Pool" /> } />
                    <Route path=path!("/pool-categories") view=|| view! { <Todo name="Pool Categories" /> } />

                    // Users
                    <Route path=path!("/users") view=UserListPage />
                    <Route path=path!("/user/:name") view=UserViewPage />
                    <Route path=path!("/user/:name/edit") view=|| view! { <Todo name="Edit User" /> } />
                    <Route path=path!("/user/:name/list-tokens") view=|| view! { <Todo name="User Tokens" /> } />
                    <Route path=path!("/user/:name/delete") view=|| view! { <Todo name="Delete User" /> } />

                    // Auth
                    <Route path=path!("/login") view=|| view! { <Todo name="Log In" /> } />
                    <Route path=path!("/logout") view=|| view! { <Todo name="Log Out" /> } />
                    <Route path=path!("/register") view=|| view! { <Todo name="Register" /> } />
                    <Route path=path!("/password-reset") view=|| view! { <Todo name="Password Reset" /> } />
                    <Route path=path!("/password-reset/:token") view=|| view! { <Todo name="Password Reset" /> } />

                    // Comments
                    <Route path=path!("/comments") view=CommentsPage />

                    // History
                    <Route path=path!("/history") view=HistoryPage />

                    // Settings
                    <Route path=path!("/settings") view=|| view! { <Todo name="Settings" /> } />

                    // Help
                    <Route path=path!("/help") view=HelpPage />
                    <Route path=path!("/help/:section") view=HelpPage />
                    <Route path=path!("/help/:section/:subsection") view=HelpPage />
                </Routes>
            </main>
        </Router>
    }
}
