use leptos::prelude::*;
use leptos_meta::Title;
use oxibooru_shared::comment::CommentInfo;
use oxibooru_shared::enums::Rating;
use oxibooru_shared::post::PostInfo;
use oxibooru_shared::request::DeleteBody;

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::api_error::ApiErrorMessage;
use crate::components::comment_form::CommentForm;
use crate::components::loading_bar::LoadingState;
use crate::components::markdown::Markdown;
use crate::components::pagination::Pagination;
use crate::components::score_widget::ScoreWidget;
use crate::settings::SettingsState;
use crate::utils::{build_list_url, format_relative_time, setup_scroll_listener, use_list_query_params};

/// Fields needed to display posts with their comments.
const FIELDS: &str = "id,thumbnailUrl,comments,commentCount";

/// The comments page shows recent comments grouped by post.
/// It fetches posts sorted by latest comment date.
#[component]
pub fn CommentsPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let loading = expect_context::<LoadingState>();
    let settings = expect_context::<SettingsState>();
    let params = use_list_query_params(10);
    let endless = settings.inner.get_untracked().endless_scroll;
    let refetch_trigger = RwSignal::new(0u32);
    let editing_id = RwSignal::new(Option::<i64>::None);

    let posts = LocalResource::new(move || {
        let _ = refetch_trigger.get();
        let client = api.get();
        let p = params.get();
        // Combine user query with comment sorting
        let mut query = "sort:comment-date comment-count:1..".to_string();
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
        let mut query = "sort:comment-date comment-count:1..".to_string();
        if !p.query.is_empty() {
            query = format!("{} {query}", p.query);
        }
        let offset = loaded_up_to.get_untracked();
        let limit = p.limit;

        leptos::task::spawn_local(async move {
            if let Ok(data) = client.get_posts(&query, offset, limit, FIELDS).await {
                let new_count = data.results.len() as i64;
                accumulated.update(|v| v.extend(data.results));
                loaded_up_to.set(offset + new_count);
                total_results.set(data.total);
            }
            loading_more.set(false);
        });
    };

    if endless {
        setup_scroll_listener(loading_more, has_more, load_more);
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
                                    {move || {
                                        let total = total_results.get();
                                        let limit = params.get().limit;
                                        let total_pages = if limit > 0 && total > 0 {
                                            (total + limit - 1) / limit
                                        } else {
                                            0
                                        };
                                        (total_pages > 0).then(|| view! {
                                            <p class="page-header">
                                                <span>{format!("Page 1 of {}", total_pages)}</span>
                                            </p>
                                        })
                                    }}
                                    <div class="comments-list">
                                        <For
                                            each=move || accumulated.get()
                                            key=|post| post.id.unwrap_or(0)
                                            children=move |post| {
                                                render_post_comments(
                                                    post,
                                                    api,
                                                    auth,
                                                    refetch_trigger,
                                                    editing_id,
                                                )
                                            }
                                        />
                                    </div>
                                    <div class="scroll-sentinel">
                                        {move || loading_more.get().then(|| view! {
                                            <p class="loading-more">"Loading more comments..."</p>
                                        })}
                                    </div>
                                }.into_any()
                            } else {
                                let total = data.total;
                                let offset = data.offset;
                                let limit = data.limit;
                                let total_pages = if limit > 0 { (total + limit - 1) / limit } else { 0 };
                                let current_page = if limit > 0 { offset / limit + 1 } else { 1 };
                                view! {
                                    {(total_pages > 0).then(|| view! {
                                        <p class="page-header">
                                            <span>{format!("Page {} of {}", current_page, total_pages)}</span>
                                        </p>
                                    })}
                                    <div class="comments-list">
                                        {data.results.into_iter().map(|post| {
                                            render_post_comments(
                                                post,
                                                api,
                                                auth,
                                                refetch_trigger,
                                                editing_id,
                                            )
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

fn render_post_comments(
    post: PostInfo,
    api: RwSignal<ApiClient>,
    auth: AuthState,
    refetch_trigger: RwSignal<u32>,
    editing_id: RwSignal<Option<i64>>,
) -> impl IntoView {
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
                    let comment_id = comment.id.unwrap_or(0);
                    let version = comment.version.clone().unwrap_or_default();
                    let micro = comment.user.flatten();
                    let author = micro.as_ref()
                        .map(|u| u.name.clone())
                        .unwrap_or_else(|| "Anonymous".to_string());
                    let avatar_url = micro.as_ref()
                        .map(|u| u.avatar_url.clone())
                        .unwrap_or_default();
                    let time = comment.creation_time.as_deref()
                        .map(format_relative_time)
                        .unwrap_or_default();
                    let comment_text = comment.text.clone().unwrap_or_default();
                    let current_username = auth.current_user.get_untracked()
                        .and_then(|u| u.name.clone());
                    let can_edit_own = auth.has_privilege("comment_edit_own");
                    let can_edit_any = auth.has_privilege("comment_edit_any");
                    let can_delete_own = auth.has_privilege("comment_delete_own");
                    let can_delete_any = auth.has_privilege("comment_delete_any");
                    let is_own = current_username.as_deref() == Some(author.as_str());
                    let can_edit = (is_own && can_edit_own) || can_edit_any;
                    let can_delete = (is_own && can_delete_own) || can_delete_any;
                    let score_signal = RwSignal::new(comment.score.unwrap_or(0));
                    let own_score_signal = RwSignal::new(comment.own_score.unwrap_or(Rating::None));
                    let on_vote = Callback::new(move |rating: Rating| {
                        let client = api.get_untracked();
                        leptos::task::spawn_local(async move {
                            if client.score_comment(comment_id, rating).await.is_ok() {
                                refetch_trigger.update(|n| *n += 1);
                            }
                        });
                    });
                    let version_del = version.clone();
                    let on_delete = move |_: leptos::ev::MouseEvent| {
                        let client = api.get_untracked();
                        let v = version_del.clone();
                        leptos::task::spawn_local(async move {
                            if client.delete_comment(comment_id, &DeleteBody { version: v }).await.is_ok() {
                                refetch_trigger.update(|n| *n += 1);
                            }
                        });
                    };
                    let version_edit = version.clone();
                    let text_edit = comment_text.clone();
                    let on_edit_done = Callback::new(move |_: CommentInfo| {
                        editing_id.set(None);
                        refetch_trigger.update(|n| *n += 1);
                    });
                    let on_cancel_edit = Callback::new(move |_: ()| editing_id.set(None));
                    let text_display = comment_text.clone();
                    view! {
                        <div class="comment-container">
                            <div class="comment-avatar">
                                <a href=format!("/user/{}", author)>
                                    {if avatar_url.is_empty() {
                                        view! {
                                            <span class="thumbnail empty" aria-hidden="true"></span>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <img
                                                class="thumbnail"
                                                src=avatar_url
                                                alt=""
                                                width="40"
                                                height="40"
                                                loading="lazy"
                                            />
                                        }.into_any()
                                    }}
                                </a>
                            </div>
                            <div class="comment">
                                <header>
                                    <span class="nickname">
                                        <a href=format!("/user/{}", author)>{author.clone()}</a>
                                    </span>
                                    " "
                                    <span class="date">"commented " {time}</span>
                                    <ScoreWidget score=score_signal own_score=own_score_signal on_vote=on_vote />
                                    {(can_edit || can_delete).then(|| view! {
                                        <span class="action-container">
                                            {can_edit.then(|| view! {
                                                <a href="javascript:void(0)" class="edit" on:click=move |_| editing_id.set(Some(comment_id))>
                                                    <i class="fa fa-pencil" />
                                                    " edit"
                                                </a>
                                            })}
                                            {can_delete.then(|| view! {
                                                <a href="javascript:void(0)" class="delete" on:click=on_delete>
                                                    <i class="fa fa-remove" />
                                                    " delete"
                                                </a>
                                            })}
                                        </span>
                                    })}
                                </header>
                                <div class="comment-body">
                                    {
                                        let td = text_display;
                                        move || {
                                            if editing_id.get() == Some(comment_id) {
                                                view! {
                                                    <CommentForm
                                                        post_id=post_id
                                                        on_submit=on_edit_done
                                                        edit_comment_id=comment_id
                                                        edit_comment_version=version_edit.clone()
                                                        initial_text=text_edit.clone()
                                                        on_cancel=on_cancel_edit
                                                    />
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <Markdown text=td.clone() />
                                                }.into_any()
                                            }
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </article>
    }
}
