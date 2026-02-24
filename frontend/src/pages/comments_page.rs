use leptos::prelude::*;
use leptos_meta::Title;
use oxibooru_shared::post::PostInfo;

use crate::api::ApiClient;
use crate::components::api_error::ApiErrorMessage;
use crate::components::loading_bar::LoadingState;
use crate::components::markdown::Markdown;
use crate::components::pagination::Pagination;
use crate::settings::SettingsState;
use crate::utils::{build_list_url, format_time_short, setup_scroll_listener, use_list_query_params};

/// Fields needed to display posts with their comments.
const FIELDS: &str = "id,thumbnailUrl,comments,commentCount";

/// The comments page shows recent comments grouped by post.
/// It fetches posts sorted by latest comment date.
#[component]
pub fn CommentsPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let loading = expect_context::<LoadingState>();
    let settings = expect_context::<SettingsState>();
    let params = use_list_query_params(10);
    let endless = settings.inner.get_untracked().endless_scroll;

    let posts = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        // Combine user query with comment sorting
        let mut query = "sort:comment-date comment-count-min:1".to_string();
        if !p.query.is_empty() {
            query = format!("{} {query}", p.query);
        }
        async move {
            loading.start();
            let result = client.get_posts(&query, p.offset, p.limit, FIELDS).await;
            loading.finish();
            result
        }
    });

    // Endless scroll state
    let accumulated = RwSignal::new(Vec::<PostInfo>::new());
    let loaded_up_to = RwSignal::new(0i64);
    let total_results = RwSignal::new(0i64);
    let loading_more = RwSignal::new(false);
    let has_more = Signal::derive(move || loaded_up_to.get() < total_results.get());

    let load_more = move || {
        if loading_more.get_untracked() || !has_more.get_untracked() {
            return;
        }
        loading_more.set(true);
        let client = api.get_untracked();
        let p = params.get_untracked();
        let mut query = "sort:comment-date comment-count-min:1".to_string();
        if !p.query.is_empty() {
            query = format!("{} {query}", p.query);
        }
        let offset = loaded_up_to.get_untracked();
        let limit = p.limit;

        leptos::task::spawn_local(async move {
            match client.get_posts(&query, offset, limit, FIELDS).await {
                Ok(data) => {
                    let new_count = data.results.len() as i64;
                    accumulated.update(|v| v.extend(data.results));
                    loaded_up_to.set(offset + new_count);
                    total_results.set(data.total);
                }
                Err(_) => {}
            }
            loading_more.set(false);
        });
    };

    if endless {
        setup_scroll_listener(loading_more, has_more, move || load_more());
    }

    view! {
        <Title text="Comments" />
        <div class="comments-page">
            <h1>"Comments"</h1>
            <Suspense fallback=|| view! { <p>"Loading comments..."</p> }>
                {move || Suspend::new(async move {
                    match posts.await {
                        Ok(data) => {
                            if endless {
                                accumulated.set(data.results);
                                loaded_up_to.set(data.offset + accumulated.get_untracked().len() as i64);
                                total_results.set(data.total);

                                view! {
                                    <div class="comments-list">
                                        <For
                                            each=move || accumulated.get()
                                            key=|post| post.id.unwrap_or(0)
                                            children=move |post| {
                                                render_post_comments(post)
                                            }
                                        />
                                    </div>
                                    <div class="scroll-sentinel">
                                        {move || loading_more.get().then(|| view! {
                                            <p class="loading-more">"Loading more comments..."</p>
                                        })}
                                        {move || {
                                            let total = total_results.get();
                                            let loaded = loaded_up_to.get();
                                            (!loading_more.get() && loaded >= total && total > 0).then(|| {
                                                view! { <p class="end-of-results">{format!("{total} results")}</p> }
                                            })
                                        }}
                                    </div>
                                }.into_any()
                            } else {
                                let total = data.total;
                                let offset = data.offset;
                                let limit = data.limit;
                                view! {
                                    <div class="comments-list">
                                        {data.results.into_iter().map(|post| {
                                            render_post_comments(post)
                                        }).collect_view()}
                                    </div>
                                    <Pagination
                                        offset=offset
                                        limit=limit
                                        total=total
                                        href_for_page=Callback::new(move |(offset, limit)| {
                                            build_list_url("/comments", "", offset, limit)
                                        })
                                    />
                                }.into_any()
                            }
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

fn render_post_comments(post: PostInfo) -> impl IntoView {
    let post_id = post.id.unwrap_or(0);
    let thumb = post.thumbnail_url.clone().unwrap_or_default();
    let comments = post.comments.unwrap_or_default();
    view! {
        <article class="post-comments">
            <div class="post-comments-thumb">
                <a href=format!("/post/{post_id}")>
                    <img src=thumb loading="lazy" alt=format!("Post {post_id}") />
                </a>
            </div>
            <div class="post-comments-body">
                {comments.into_iter().map(|comment| {
                    let author = comment.user.flatten()
                        .map(|u| u.name)
                        .unwrap_or_else(|| "Anonymous".to_string());
                    let time = comment.creation_time.as_deref()
                        .map(format_time_short)
                        .unwrap_or_default();
                    let text = comment.text.unwrap_or_default();
                    view! {
                        <div class="comment">
                            <div class="comment-header">
                                <a class="comment-author" href=format!("/user/{author}")>{author.clone()}</a>
                                <time>{time}</time>
                            </div>
                            <Markdown text=text />
                        </div>
                    }
                }).collect_view()}
            </div>
        </article>
    }
}
