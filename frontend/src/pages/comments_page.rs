use leptos::prelude::*;
use leptos_meta::Title;

use crate::api::ApiClient;
use crate::components::markdown::Markdown;
use crate::components::pagination::Pagination;
use crate::utils::{build_list_url, format_time_short, use_list_query_params};

/// Fields needed to display posts with their comments.
const FIELDS: &str = "id,thumbnailUrl,comments,commentCount";

/// The comments page shows recent comments grouped by post.
/// It fetches posts sorted by latest comment date.
#[component]
pub fn CommentsPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_list_query_params(10);

    let posts = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        // Combine user query with comment sorting
        let mut query = "sort:comment-date comment-count-min:1".to_string();
        if !p.query.is_empty() {
            query = format!("{} {query}", p.query);
        }
        async move { client.get_posts(&query, p.offset, p.limit, FIELDS).await.ok() }
    });

    view! {
        <Title text="Comments" />
        <div class="comments-page">
            <h1>"Comments"</h1>
            <Suspense fallback=|| view! { <p>"Loading comments..."</p> }>
                {move || Suspend::new(async move {
                    match posts.await {
                        Some(data) => {
                            let total = data.total;
                            let offset = data.offset;
                            let limit = data.limit;
                            view! {
                                <div class="comments-list">
                                    {data.results.into_iter().map(|post| {
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
                        None => view! {
                            <p class="error">"Failed to load comments."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
