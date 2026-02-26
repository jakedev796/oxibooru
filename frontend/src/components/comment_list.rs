use leptos::prelude::*;
use oxibooru_shared::comment::CommentInfo;
use oxibooru_shared::enums::Rating;
use oxibooru_shared::request::DeleteBody;

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::comment_form::CommentForm;
use crate::components::markdown::Markdown;
use crate::components::score_widget::ScoreWidget;
use crate::utils::format_relative_time;

#[component]
pub fn CommentList(post_id: i64, comments: RwSignal<Vec<CommentInfo>>) -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let editing_id = RwSignal::new(Option::<i64>::None);

    view! {
        <div class="comments-list">
            {move || {
                let current_username = auth.current_user.get()
                    .and_then(|u| u.name.clone());
                let can_edit_own = auth.has_privilege("comment_edit_own");
                let can_edit_any = auth.has_privilege("comment_edit_any");
                let can_delete_own = auth.has_privilege("comment_delete_own");
                let can_delete_any = auth.has_privilege("comment_delete_any");

                if comments.get().is_empty() {
                    return view! { <p class="no-comments">"No comments yet."</p> }.into_any();
                }

                comments.get().into_iter().map(|comment| {
                    let comment_id = comment.id.unwrap_or(0);
                    let version = comment.version.clone().unwrap_or_default();
                    let micro_user = comment.user.flatten();
                    let author = micro_user.as_ref()
                        .map(|u| u.name.clone())
                        .unwrap_or_else(|| "Anonymous".to_string());
                    let avatar_url = micro_user.as_ref()
                        .map(|u| u.avatar_url.clone())
                        .filter(|url| !url.is_empty());
                    let relative_time = comment.creation_time.as_deref()
                        .map(format_relative_time)
                        .unwrap_or_default();
                    let comment_text = comment.text.clone().unwrap_or_default();

                    let is_own = current_username.as_deref() == Some(author.as_str());
                    let can_edit = (is_own && can_edit_own) || can_edit_any;
                    let can_delete = (is_own && can_delete_own) || can_delete_any;

                    let author_href = format!("/user/{author}");
                    let avatar_href = format!("/user/{author}");
                    let score_signal = RwSignal::new(comment.score.unwrap_or(0));
                    let own_score_signal = RwSignal::new(comment.own_score.unwrap_or(Rating::None));

                    let on_vote = Callback::new(move |rating: Rating| {
                        let client = api.get_untracked();
                        leptos::task::spawn_local(async move {
                            if let Ok(updated) = client.score_comment(comment_id, rating).await {
                                comments.update(|list| {
                                    if let Some(c) = list.iter_mut().find(|c| c.id == Some(comment_id)) {
                                        *c = updated;
                                    }
                                });
                            }
                        });
                    });

                    let version_del = version.clone();
                    let on_delete = move |_: leptos::ev::MouseEvent| {
                        let client = api.get_untracked();
                        let v = version_del.clone();
                        leptos::task::spawn_local(async move {
                            if client.delete_comment(comment_id, &DeleteBody { version: v }).await.is_ok() {
                                comments.update(|list| list.retain(|c| c.id != Some(comment_id)));
                            }
                        });
                    };

                    let version_edit = version.clone();
                    let text_edit = comment_text.clone();
                    let on_edit_done = Callback::new(move |updated: CommentInfo| {
                        comments.update(|list| {
                            if let Some(c) = list.iter_mut().find(|c| c.id == Some(comment_id)) {
                                *c = updated;
                            }
                        });
                        editing_id.set(None);
                    });

                    let on_cancel_edit = Callback::new(move |_: ()| editing_id.set(None));

                    let text_display = comment_text.clone();
                    let ve = version_edit;
                    let te = text_edit;

                    view! {
                        <div class="comment-container">
                            <div class="comment-avatar">
                                <a href=avatar_href>
                                    {match avatar_url {
                                        Some(url) => view! {
                                            <img class="thumbnail" src=url alt="avatar" />
                                        }.into_any(),
                                        None => view! {
                                            <div class="thumbnail empty">
                                                <i class="fa fa-user" />
                                            </div>
                                        }.into_any(),
                                    }}
                                </a>
                            </div>
                            <div class="comment">
                                <header>
                                    <span class="nickname">
                                        <a href=author_href>{author}</a>
                                    </span>
                                    " "
                                    <span class="date">"commented " {relative_time}</span>
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
                                                        edit_comment_version=ve.clone()
                                                        initial_text=te.clone()
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
                }).collect_view().into_any()
            }}
        </div>
    }
}
