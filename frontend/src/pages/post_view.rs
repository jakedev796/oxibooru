use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use oxibooru_shared::comment::CommentInfo;
use oxibooru_shared::enums::{PostSafety, PostType, Rating};

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::comment_form::CommentForm;
use crate::components::comment_list::CommentList;
use crate::components::favorite_widget::FavoriteWidget;
use crate::components::markdown::Markdown;
use crate::components::post_content::PostContent;
use crate::components::score_widget::ScoreWidget;
use crate::settings::SettingsState;
use crate::utils::{format_file_size, format_time_short};

#[component]
pub fn PostViewPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let settings = expect_context::<SettingsState>();
    let params = use_params_map();

    let post_id = Memo::new(move |_| {
        params.get().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0)
    });

    let post = LocalResource::new(move || {
        let client = api.get();
        let id = post_id.get();
        async move { client.get_post(id).await.ok() }
    });

    let neighbors = LocalResource::new(move || {
        let client = api.get();
        let id = post_id.get();
        async move { client.get_post_around(id, "", "id").await.ok() }
    });

    view! {
        <Title text=move || format!("Post {}", post_id.get()) />
        <div class="post-view-page">
            <Suspense fallback=|| view! { <p>"Loading post..."</p> }>
                {move || Suspend::new(async move {
                    match post.await {
                        Some(post) => {
                            let id = post.id.unwrap_or(0);
                            let safety = post.safety.unwrap_or(PostSafety::Safe);
                            let post_type = post.type_.unwrap_or(PostType::Image);
                            let content_url = post.content_url.clone().unwrap_or_default();
                            let fit_mode = settings.inner.get_untracked().fit_mode.clone();
                            let flags = post.flags.clone().unwrap_or_default();

                            // Static sidebar info
                            let file_size = post.file_size.map(format_file_size).unwrap_or_default();
                            let dimensions = match (post.canvas_width, post.canvas_height) {
                                (Some(w), Some(h)) => format!("{w}x{h}"),
                                _ => String::new(),
                            };
                            let uploader = post.user.flatten()
                                .map(|u| u.name)
                                .unwrap_or_else(|| "Anonymous".to_string());
                            let created = post.creation_time.as_deref()
                                .map(format_time_short)
                                .unwrap_or_default();
                            let safety_str = format!("{safety:?}").to_lowercase();
                            let source = post.source.clone().unwrap_or_default();
                            let description = post.description.clone().unwrap_or_default();

                            // Interactive signals
                            let score_signal = RwSignal::new(post.score.unwrap_or(0));
                            let own_score_signal = RwSignal::new(post.own_score.unwrap_or(Rating::None));
                            let favorited_signal = RwSignal::new(post.own_favorite.unwrap_or(false));
                            let fav_count_signal = RwSignal::new(post.favorite_count.unwrap_or(0));
                            let comments_signal = RwSignal::new(post.comments.unwrap_or_default());

                            // Post score callback
                            let on_post_vote = Callback::new(move |rating: Rating| {
                                let client = api.get_untracked();
                                leptos::task::spawn_local(async move {
                                    if let Ok(p) = client.score_post(id, rating).await {
                                        score_signal.set(p.score.unwrap_or(0));
                                        own_score_signal.set(p.own_score.unwrap_or(Rating::None));
                                    }
                                });
                            });

                            // Favorite toggle callback
                            let on_toggle_fav = Callback::new(move |want_fav: bool| {
                                let client = api.get_untracked();
                                leptos::task::spawn_local(async move {
                                    let result = if want_fav {
                                        client.add_favorite(id).await
                                    } else {
                                        client.remove_favorite(id).await
                                    };
                                    if let Ok(p) = result {
                                        favorited_signal.set(p.own_favorite.unwrap_or(false));
                                        fav_count_signal.set(p.favorite_count.unwrap_or(0));
                                    }
                                });
                            });

                            // New comment callback
                            let on_comment_created = Callback::new(move |comment: CommentInfo| {
                                comments_signal.update(|list| list.push(comment));
                            });

                            // Tags grouped by category
                            let tags = post.tags.unwrap_or_default();
                            let pools = post.pools.unwrap_or_default();
                            let relations = post.relations.unwrap_or_default();

                            let uploader_href = format!("/user/{uploader}");
                            let safety_class = format!("safety-{safety_str}");

                            // Privilege checks for action links
                            let can_edit = auth.has_privilege("posts:edit:own") || auth.has_privilege("posts:edit:any");
                            let can_merge = auth.has_privilege("posts:merge");
                            let can_comment = auth.has_privilege("comments:create");
                            let show_actions = can_edit || can_merge;

                            view! {
                                <div class="post-view">
                                    <div class="post-content-area">
                                        <PostContent
                                            content_url=content_url
                                            post_type=post_type
                                            fit_mode=fit_mode
                                            flags=flags
                                        />
                                    </div>
                                    <aside class="post-sidebar">
                                        <section class="post-info">
                                            <h2>"Info"</h2>
                                            <dl>
                                                <dt>"ID"</dt><dd>{id}</dd>
                                                <dt>"Safety"</dt><dd class=safety_class>{safety_str}</dd>
                                                <dt>"Type"</dt><dd>{format!("{post_type:?}").to_lowercase()}</dd>
                                                <dt>"Size"</dt><dd>{file_size}</dd>
                                                {(!dimensions.is_empty()).then(|| view! {
                                                    <dt>"Dimensions"</dt><dd>{dimensions}</dd>
                                                })}
                                                <dt>"Uploader"</dt>
                                                <dd><a href=uploader_href>{uploader}</a></dd>
                                                <dt>"Uploaded"</dt><dd>{created}</dd>
                                                <dt>"Score"</dt>
                                                <dd><ScoreWidget score=score_signal own_score=own_score_signal on_vote=on_post_vote /></dd>
                                                <dt>"Favorites"</dt>
                                                <dd><FavoriteWidget favorited=favorited_signal count=fav_count_signal on_toggle=on_toggle_fav /></dd>
                                                <dt>"Comments"</dt>
                                                <dd>{move || comments_signal.get().len()}</dd>
                                            </dl>
                                            {(!source.is_empty()).then(|| {
                                                let source_href = source.clone();
                                                view! {
                                                    <div class="post-source">
                                                        <h3>"Source"</h3>
                                                        <a href=source_href rel="noopener noreferrer">{source}</a>
                                                    </div>
                                                }
                                            })}
                                        </section>

                                        {show_actions.then(|| {
                                            let edit_href = format!("/post/{id}/edit");
                                            let merge_href = format!("/post/{id}/merge");
                                            view! {
                                                <section class="post-actions">
                                                    <h2>"Actions"</h2>
                                                    <ul>
                                                        {can_edit.then(|| view! {
                                                            <li><a href=edit_href>"Edit post"</a></li>
                                                        })}
                                                        {can_merge.then(|| view! {
                                                            <li><a href=merge_href>"Merge post"</a></li>
                                                        })}
                                                    </ul>
                                                </section>
                                            }
                                        })}

                                        {(!tags.is_empty()).then(|| view! {
                                            <section class="post-tags">
                                                <h2>"Tags"</h2>
                                                <ul class="tag-list-inline">
                                                    {tags.into_iter().map(|tag| {
                                                        let name = tag.names.first().cloned().unwrap_or_default();
                                                        let category = tag.category.clone();
                                                        let href = format!("/tag/{name}");
                                                        let class = format!("tag-category-{category}");
                                                        view! {
                                                            <li class=class>
                                                                <a href=href>{name}</a>
                                                                <span class="tag-usages">{tag.usages}</span>
                                                            </li>
                                                        }
                                                    }).collect_view()}
                                                </ul>
                                            </section>
                                        })}

                                        {(!pools.is_empty()).then(|| view! {
                                            <section class="post-pools">
                                                <h2>"Pools"</h2>
                                                <ul>
                                                    {pools.into_iter().map(|pool| {
                                                        let name = pool.names.first().cloned().unwrap_or_default();
                                                        let href = format!("/pool/{}", pool.id);
                                                        view! {
                                                            <li><a href=href>{name}</a></li>
                                                        }
                                                    }).collect_view()}
                                                </ul>
                                            </section>
                                        })}

                                        {(!relations.is_empty()).then(|| view! {
                                            <section class="post-relations">
                                                <h2>"Relations"</h2>
                                                <div class="relation-thumbs">
                                                    {relations.into_iter().map(|rel| {
                                                        let href = format!("/post/{}", rel.id);
                                                        view! {
                                                            <a href=href>
                                                                <img src=rel.thumbnail_url.clone() loading="lazy" alt=format!("Post {}", rel.id) />
                                                            </a>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </section>
                                        })}
                                    </aside>
                                </div>

                                {(!description.is_empty()).then(|| view! {
                                    <section class="post-description">
                                        <h2>"Description"</h2>
                                        <Markdown text=description />
                                    </section>
                                })}

                                <section class="post-comments">
                                    <h2>"Comments"</h2>
                                    <CommentList post_id=id comments=comments_signal />
                                    {can_comment.then(|| view! {
                                        <CommentForm post_id=id on_submit=on_comment_created />
                                    })}
                                </section>

                                // Prev/Next navigation
                                <Suspense fallback=|| ()>
                                    {move || Suspend::new(async move {
                                        neighbors.await.map(|n| {
                                            let prev_href = n.prev.and_then(|p| p.id).map(|id| format!("/post/{id}"));
                                            let next_href = n.next.and_then(|p| p.id).map(|id| format!("/post/{id}"));
                                            view! {
                                                <nav class="post-neighbors">
                                                    {prev_href.map(|href| view! {
                                                        <a class="prev" href=href>"\u{00AB} Previous"</a>
                                                    })}
                                                    {next_href.map(|href| view! {
                                                        <a class="next" href=href>"Next \u{00BB}"</a>
                                                    })}
                                                </nav>
                                            }
                                        })
                                    })}
                                </Suspense>
                            }.into_any()
                        }
                        None => view! {
                            <p class="error">"Failed to load post."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
