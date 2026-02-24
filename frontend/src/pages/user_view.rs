use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::api::ApiClient;
use crate::components::api_error::ApiErrorMessage;
use crate::components::loading_bar::LoadingState;
use crate::utils::format_time_short;

#[component]
pub fn UserViewPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let loading = expect_context::<LoadingState>();
    let params = use_params_map();

    let user_name = Memo::new(move |_| {
        params.get().get("name").unwrap_or_default()
    });

    let user = LocalResource::new(move || {
        let client = api.get();
        let name = user_name.get();
        async move {
            loading.start();
            let result = client.get_user(&name).await;
            loading.finish();
            result
        }
    });

    view! {
        <Title text=move || format!("User: {}", user_name.get()) />
        <div class="user-view-page">
            <Suspense fallback=|| view! { <p>"Loading user..."</p> }>
                {move || Suspend::new(async move {
                    match user.await {
                        Ok(user) => {
                            let name = user.name.unwrap_or_default();
                            let avatar_url = user.avatar_url.unwrap_or_default();
                            let rank = user.rank
                                .map(|r| format!("{r:?}").to_lowercase())
                                .unwrap_or_default();
                            let created = user.creation_time.as_deref()
                                .map(format_time_short)
                                .unwrap_or_default();
                            let last_login = user.last_login_time.as_deref()
                                .map(format_time_short)
                                .unwrap_or_else(|| "never".to_string());
                            let uploaded = user.uploaded_post_count.unwrap_or(0);
                            let comments = user.comment_count.unwrap_or(0);
                            let favorites = user.favorite_post_count.unwrap_or(0);
                            let liked = user.liked_post_count.as_ref()
                                .and_then(|p| p.value().copied());
                            let disliked = user.disliked_post_count.as_ref()
                                .and_then(|p| p.value().copied());
                            let rank_class = format!("rank-{rank}");

                            view! {
                                <div class="user-view">
                                    <div class="user-profile">
                                        <img class="user-avatar-large" src=avatar_url alt=name.clone() />
                                        <div class="user-details">
                                            <h1>{name}</h1>
                                            <span class=rank_class>{rank}</span>
                                        </div>
                                    </div>

                                    <section class="user-stats">
                                        <h2>"Stats"</h2>
                                        <dl>
                                            <dt>"Joined"</dt><dd>{created}</dd>
                                            <dt>"Last login"</dt><dd>{last_login}</dd>
                                            <dt>"Uploaded posts"</dt><dd>{uploaded}</dd>
                                            <dt>"Comments"</dt><dd>{comments}</dd>
                                            <dt>"Favorites"</dt><dd>{favorites}</dd>
                                            {liked.map(|n| view! {
                                                <dt>"Liked posts"</dt><dd>{n}</dd>
                                            })}
                                            {disliked.map(|n| view! {
                                                <dt>"Disliked posts"</dt><dd>{n}</dd>
                                            })}
                                        </dl>
                                    </section>
                                </div>
                            }.into_any()
                        }
                        Err(e) => view! {
                            <ApiErrorMessage error=e />
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
