use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use crate::api::ApiClient;
use crate::components::pagination::Pagination;
use crate::components::post_thumbnail::PostThumbnail;
use crate::components::search_bar::SearchBar;
use crate::settings::SettingsState;
use crate::utils::{build_list_url, use_list_query_params};

const FIELDS: &str = "id,thumbnailUrl,type,safety,score,favoriteCount,commentCount,tags,version";

#[component]
pub fn PostListPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let settings = expect_context::<SettingsState>();
    let default_limit = settings.inner.get_untracked().posts_per_page as i64;
    let params = use_list_query_params(default_limit);
    let navigate = use_navigate();

    let posts = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        async move { client.get_posts(&p.query, p.offset, p.limit, FIELDS).await.ok() }
    });

    let query_signal = Signal::derive(move || params.get().query);

    let on_search = {
        let navigate = navigate.clone();
        Callback::new(move |query: String| {
            let limit = params.get().limit;
            let url = build_list_url("/posts", &query, 0, limit);
            navigate(&url, Default::default());
        })
    };

    view! {
        <Title text="Posts" />
        <div class="post-list-page">
            <SearchBar query=query_signal on_submit=on_search />
            <Suspense fallback=|| view! { <p>"Loading posts..."</p> }>
                {move || Suspend::new(async move {
                    match posts.await {
                        Some(data) => {
                            let total = data.total;
                            let offset = data.offset;
                            let limit = data.limit;
                            let query_for_page = params.get().query.clone();
                            view! {
                                <div class="post-grid">
                                    {data.results.into_iter().map(|post| {
                                        let id = post.id.unwrap_or(0);
                                        let thumbnail_url = post.thumbnail_url.clone().unwrap_or_default();
                                        let safety = post.safety.unwrap_or(oxibooru_shared::enums::PostSafety::Safe);
                                        let post_type = post.type_.unwrap_or(oxibooru_shared::enums::PostType::Image);
                                        let score = post.score;
                                        let favorite_count = post.favorite_count;
                                        let comment_count = post.comment_count;
                                        view! {
                                            <PostThumbnail
                                                id=id
                                                thumbnail_url=thumbnail_url
                                                safety=safety
                                                post_type=post_type
                                                score=score
                                                favorite_count=favorite_count
                                                comment_count=comment_count
                                            />
                                        }
                                    }).collect_view()}
                                </div>
                                <Pagination
                                    offset=offset
                                    limit=limit
                                    total=total
                                    href_for_page=Callback::new(move |(offset, limit)| {
                                        build_list_url("/posts", &query_for_page, offset, limit)
                                    })
                                />
                            }.into_any()
                        }
                        None => view! {
                            <p class="error">"Failed to load posts."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
