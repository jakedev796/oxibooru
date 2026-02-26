use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use leptos_router::NavigateOptions;
use oxibooru_shared::comment::CommentInfo;
use oxibooru_shared::enums::{MimeType, PostSafety, PostType, Rating};

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::api_error::ApiErrorMessage;
use crate::components::comment_form::CommentForm;
use crate::components::comment_list::CommentList;
use crate::components::favorite_widget::FavoriteWidget;
use crate::components::loading_bar::LoadingState;
use crate::components::markdown::Markdown;
use crate::components::post_content::PostContent;
use crate::components::post_notes::PostNotesOverlay;
use crate::components::score_widget::ScoreWidget;
use crate::keyboard::KeyboardShortcuts;
use crate::settings::SettingsState;
use crate::utils::{format_file_size, format_relative_time};

/// Map MimeType to a short display string.
fn mime_display(mime: MimeType) -> &'static str {
    match mime {
        MimeType::Bmp => "BMP",
        MimeType::Gif => "GIF",
        MimeType::Jpeg => "JPEG",
        MimeType::Png => "PNG",
        MimeType::Webp => "WEBP",
        MimeType::Mp4 => "MPEG-4",
        MimeType::Mov => "MOV",
        MimeType::Webm => "WEBM",
        MimeType::Swf => "SWF",
        MimeType::Avif => "AVIF",
    }
}

/// Extract hostname from a URL string.
fn extract_domain(url: &str) -> String {
    if let Ok(parsed) = web_sys::Url::new(url) {
        let host = parsed.hostname();
        if !host.is_empty() {
            return host;
        }
    }
    url.to_string()
}

#[component]
pub fn PostViewPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let loading = expect_context::<LoadingState>();
    let auth = expect_context::<AuthState>();
    let settings = expect_context::<SettingsState>();
    let params = use_params_map();

    let post_id = Memo::new(move |_| params.get().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0));

    let post = LocalResource::new(move || {
        let client = api.get();
        let id = post_id.get();
        async move {
            loading.start();
            let result = client.get_post(id).await;
            loading.finish();
            result
        }
    });

    let neighbors = LocalResource::new(move || {
        let client = api.get();
        let id = post_id.get();
        async move { client.get_post_around(id, "", "id,contentUrl,type").await.ok() }
    });

    // Fit mode
    let fit_mode_signal = Signal::derive(move || settings.inner.with(|s| s.fit_mode.clone()));
    let upscale = settings.inner.get_untracked().upscale_small_posts;

    // Signals to hold neighbor IDs (populated when neighbors load)
    let prev_id: RwSignal<Option<i64>> = RwSignal::new(None);
    let next_id: RwSignal<Option<i64>> = RwSignal::new(None);

    // Keyboard shortcuts
    let shortcuts = expect_context::<KeyboardShortcuts>();
    let navigate = use_navigate();

    // E — go to edit page
    let nav_edit = navigate.clone();
    shortcuts.register(
        "e",
        Callback::new(move |()| {
            let id = post_id.get_untracked();
            if id > 0 {
                nav_edit(&format!("/post/{id}/edit"), NavigateOptions::default());
            }
        }),
    );

    // A / ArrowLeft — previous post
    let nav_prev = navigate.clone();
    let go_prev = Callback::new(move |()| {
        if let Some(id) = prev_id.get_untracked() {
            nav_prev(&format!("/post/{id}"), NavigateOptions::default());
        }
    });
    shortcuts.register("a", go_prev);
    shortcuts.register("ArrowLeft", go_prev);

    // D / ArrowRight — next post
    let nav_next = navigate.clone();
    let go_next = Callback::new(move |()| {
        if let Some(id) = next_id.get_untracked() {
            nav_next(&format!("/post/{id}"), NavigateOptions::default());
        }
    });
    shortcuts.register("d", go_next);
    shortcuts.register("ArrowRight", go_next);

    on_cleanup(move || {
        shortcuts.unregister("f");
        shortcuts.unregister("e");
        shortcuts.unregister("a");
        shortcuts.unregister("ArrowLeft");
        shortcuts.unregister("d");
        shortcuts.unregister("ArrowRight");
    });

    view! {
        <Title text=move || format!("Post {}", post_id.get()) />
        <div class="post-view-page">
            <Suspense fallback=|| view! { <p>"Loading post..."</p> }>
                {move || Suspend::new(async move {
                    match post.await {
                        Ok(post) => {
                            let id = post.id.unwrap_or(0);
                            let safety = post.safety.unwrap_or(PostSafety::Safe);
                            let post_type = post.type_.unwrap_or(PostType::Image);
                            let content_url = post.content_url.clone().unwrap_or_default();
                            let flags = post.flags.clone().unwrap_or_default();
                            let mime = post.mime_type;

                            // Sidebar info
                            let file_size = post.file_size.map(format_file_size).unwrap_or_default();
                            let mime_str = mime.map(mime_display).unwrap_or("unknown");
                            let dimensions = match (post.canvas_width, post.canvas_height) {
                                (Some(w), Some(h)) => format!("({w}x{h})"),
                                _ => String::new(),
                            };
                            let download_text = format!("{file_size} {mime_str} {dimensions}");
                            let download_url = content_url.clone();

                            let micro_user = post.user.flatten();
                            let uploader = micro_user.as_ref()
                                .map(|u| u.name.clone())
                                .unwrap_or_else(|| "Anonymous".to_string());
                            let avatar_url = micro_user.as_ref()
                                .map(|u| u.avatar_url.clone())
                                .filter(|url| !url.is_empty());
                            let uploader_href = format!("/user/{uploader}");
                            let created_relative = post.creation_time.as_deref()
                                .map(format_relative_time)
                                .unwrap_or_default();

                            let safety_str = format!("{safety:?}");
                            let safety_class = format!("safety-{}", safety_str.to_lowercase());

                            // Source
                            let source_raw = post.source.clone().unwrap_or_default();
                            let source_urls: Vec<(String, String)> = source_raw
                                .split_whitespace()
                                .filter(|s| !s.is_empty())
                                .map(|url| {
                                    let domain = extract_domain(url);
                                    (url.to_string(), domain)
                                })
                                .collect();
                            let has_source = !source_urls.is_empty();

                            // Flags
                            let has_loop = flags.iter().any(|f| f == "loop");
                            let has_sound = flags.iter().any(|f| f == "sound");

                            let description = post.description.clone().unwrap_or_default();
                            let checksum_md5 = post.checksum_md5.clone().unwrap_or_default();

                            // Reverse search links
                            let abs_content_url = web_sys::window()
                                .and_then(|w| w.location().origin().ok())
                                .map(|origin| format!("{origin}{content_url}"))
                                .unwrap_or_default();
                            let iqdb_url = if !abs_content_url.is_empty() {
                                Some(format!("https://iqdb.org/?url={}", js_sys::encode_uri_component(&abs_content_url)))
                            } else { None };
                            let danbooru_url = if !checksum_md5.is_empty() {
                                Some(format!("https://danbooru.donmai.us/posts?tags=md5:{checksum_md5}"))
                            } else { None };
                            let google_lens_url = if !abs_content_url.is_empty() {
                                Some(format!("https://lens.google.com/uploadbyurl?url={}", js_sys::encode_uri_component(&abs_content_url)))
                            } else { None };
                            let has_reverse_search = iqdb_url.is_some() || danbooru_url.is_some() || google_lens_url.is_some();

                            // Interactive signals
                            let score_signal = RwSignal::new(post.score.unwrap_or(0));
                            let own_score_signal = RwSignal::new(post.own_score.unwrap_or(Rating::None));
                            let favorited_signal = RwSignal::new(post.own_favorite.unwrap_or(false));
                            let fav_count_signal = RwSignal::new(post.favorite_count.unwrap_or(0));
                            let comments_signal = RwSignal::new(post.comments.unwrap_or_default());

                            let on_post_vote = Callback::new(move |rating: Rating| {
                                let client = api.get_untracked();
                                leptos::task::spawn_local(async move {
                                    if let Ok(p) = client.score_post(id, rating).await {
                                        score_signal.set(p.score.unwrap_or(0));
                                        own_score_signal.set(p.own_score.unwrap_or(Rating::None));
                                    }
                                });
                            });

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

                            let on_comment_created = Callback::new(move |comment: CommentInfo| {
                                comments_signal.update(|list| list.push(comment));
                            });

                            let tags = post.tags.unwrap_or_default();
                            let tag_count = tags.len();
                            let pools = post.pools.unwrap_or_default();
                            let relations = post.relations.unwrap_or_default();
                            let relation_count = relations.len();
                            let notes = post.notes.unwrap_or_default();

                            let can_edit = auth.has_privilege("post_edit_tag");
                            let can_comment = auth.has_privilege("comment_create");
                            let edit_href = format!("/post/{id}/edit");

                            view! {
                                <div class="post-view">
                                    // Sidebar
                                    <aside class="post-sidebar">
                                        // Navigation buttons
                                        <nav class="sidebar-nav">
                                            <a
                                                class="nav-btn"
                                                class:inactive=move || prev_id.get().is_none()
                                                href=move || prev_id.get().map(|pid| format!("/post/{pid}")).unwrap_or_default()
                                            >
                                                <i class="fa fa-chevron-left" />
                                            </a>
                                            <a
                                                class="nav-btn"
                                                class:inactive=move || next_id.get().is_none()
                                                href=move || next_id.get().map(|pid| format!("/post/{pid}")).unwrap_or_default()
                                            >
                                                <i class="fa fa-chevron-right" />
                                            </a>
                                            {can_edit.then(|| view! {
                                                <a class="nav-btn" href=edit_href>
                                                    <i class="fa fa-pencil" />
                                                </a>
                                            })}
                                        </nav>

                                        // Details section
                                        <div class="sidebar-details">
                                            // Download link
                                            <div class="detail-row">
                                                <i class="fa fa-download" />
                                                " "
                                                <a href=download_url rel="external" download="">{download_text}</a>
                                                {has_loop.then(|| view! { " " <i class="fa fa-repeat" title="Loop" /> })}
                                                {has_sound.then(|| view! { " " <i class="fa fa-volume-up" title="Sound" /> })}
                                            </div>

                                            // Uploader
                                            <div class="detail-row">
                                                {match avatar_url {
                                                    Some(url) => view! {
                                                        <img class="uploader-avatar" src=url alt="avatar" />
                                                    }.into_any(),
                                                    None => view! {
                                                        <i class="fa fa-user" />
                                                        " "
                                                    }.into_any(),
                                                }}
                                                <a href=uploader_href>{uploader}</a>
                                                ", "
                                                {created_relative}
                                            </div>

                                            // Safety
                                            <div class="detail-row">
                                                <i class=format!("fa fa-circle {safety_class}") />
                                                " "
                                                {safety_str}
                                            </div>

                                            // Fit modes
                                            <div class="detail-row zoom-row">
                                                <a
                                                    href="javascript:void(0)"
                                                    class="fit-link"
                                                    class:active=move || fit_mode_signal.get() == "fit-original"
                                                    on:click=move |_| settings.update(|s| s.fit_mode = "fit-original".to_string())
                                                >"Original zoom"</a>
                                                " \u{00B7} "
                                                <a
                                                    href="javascript:void(0)"
                                                    class="fit-link"
                                                    class:active=move || fit_mode_signal.get() == "fit-width"
                                                    on:click=move |_| settings.update(|s| s.fit_mode = "fit-width".to_string())
                                                >"fit width"</a>
                                                " \u{00B7} "
                                                <a
                                                    href="javascript:void(0)"
                                                    class="fit-link"
                                                    class:active=move || fit_mode_signal.get() == "fit-height"
                                                    on:click=move |_| settings.update(|s| s.fit_mode = "fit-height".to_string())
                                                >"height"</a>
                                                " \u{00B7} "
                                                <a
                                                    href="javascript:void(0)"
                                                    class="fit-link"
                                                    class:active=move || fit_mode_signal.get() == "fit-both"
                                                    on:click=move |_| settings.update(|s| s.fit_mode = "fit-both".to_string())
                                                >"both"</a>
                                            </div>

                                            // Source
                                            {has_source.then(|| {
                                                view! {
                                                    <div class="detail-row">
                                                        "Source: "
                                                        {source_urls.into_iter().enumerate().map(|(i, (url, domain))| {
                                                            let url_clone = url.clone();
                                                            view! {
                                                                {(i > 0).then(|| " \u{00B7} ")}
                                                                <a href=url target="_blank" rel="noopener noreferrer" title=url_clone>{domain}</a>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }
                                            })}

                                            // Reverse search
                                            {has_reverse_search.then(|| view! {
                                                <div class="detail-row">
                                                    "Search on "
                                                    {iqdb_url.map(|url| view! {
                                                        <a href=url target="_blank" rel="noopener noreferrer">"IQDB"</a>
                                                    })}
                                                    {danbooru_url.map(|url| view! {
                                                        " \u{00B7} "
                                                        <a href=url target="_blank" rel="noopener noreferrer">"Danbooru"</a>
                                                    })}
                                                    {google_lens_url.map(|url| view! {
                                                        " \u{00B7} "
                                                        <a href=url target="_blank" rel="noopener noreferrer">"Google Images"</a>
                                                    })}
                                                </div>
                                            })}

                                            // Score + Favorites
                                            <div class="detail-row social-row">
                                                <ScoreWidget score=score_signal own_score=own_score_signal on_vote=on_post_vote />
                                                <FavoriteWidget favorited=favorited_signal count=fav_count_signal on_toggle=on_toggle_fav />
                                            </div>
                                        </div>

                                        // Relations (if any)
                                        {(!relations.is_empty()).then(|| view! {
                                            <div class="sidebar-section">
                                                <h1>{format!("Relations ({relation_count})")}</h1>
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
                                            </div>
                                        })}

                                        // Tags (if any)
                                        <div class="sidebar-section">
                                            <h1>{format!("Tags ({tag_count})")}</h1>
                                            {if tags.is_empty() {
                                                view! { <p class="no-tags">"No tags yet!"</p> }.into_any()
                                            } else {
                                                view! {
                                                    <ul class="compact-tags">
                                                        {tags.into_iter().map(|tag| {
                                                            let name = tag.names.first().cloned().unwrap_or_default();
                                                            let category = tag.category.clone();
                                                            let view_href = format!("/tag/{name}");
                                                            let search_href = format!("/posts?query={name}");
                                                            let display = settings.display_name(&name);
                                                            let cat_class = format!("tag-category-{category}");
                                                            let icon_class = format!("{cat_class} tag");
                                                            let name_class = format!("{cat_class} tag");
                                                            view! {
                                                                <li>
                                                                    <a class=icon_class href=view_href>
                                                                        <i class="fa fa-tag" />
                                                                    </a>
                                                                    " "
                                                                    <a class=name_class href=search_href>
                                                                        {display}
                                                                        " "
                                                                        <span class="tag-usages">{tag.usages}</span>
                                                                    </a>
                                                                </li>
                                                            }
                                                        }).collect_view()}
                                                    </ul>
                                                }.into_any()
                                            }}
                                        </div>

                                        // Pools (if any)
                                        {(!pools.is_empty()).then(|| view! {
                                            <div class="sidebar-section">
                                                <h1>{format!("Pools ({})", pools.len())}</h1>
                                                <ul>
                                                    {pools.into_iter().map(|pool| {
                                                        let name = pool.names.first().cloned().unwrap_or_default();
                                                        let display = settings.display_name(&name);
                                                        let href = format!("/pool/{}", pool.id);
                                                        view! {
                                                            <li><a href=href>{display}</a></li>
                                                        }
                                                    }).collect_view()}
                                                </ul>
                                            </div>
                                        })}
                                    </aside>

                                    // Content area
                                    <div class="post-content-area">
                                        <div class="post-content-wrapper">
                                            <PostContent
                                                content_url=content_url
                                                post_type=post_type
                                                fit_mode=fit_mode_signal
                                                upscale=upscale
                                                flags=flags
                                            />
                                            {(!notes.is_empty()).then(|| view! {
                                                <PostNotesOverlay notes=notes />
                                            })}
                                        </div>

                                        // Description
                                        {(!description.is_empty()).then(|| view! {
                                            <section class="post-description">
                                                <details open>
                                                    <summary>"Description"</summary>
                                                    <Markdown text=description />
                                                </details>
                                            </section>
                                        })}

                                        // Comments
                                        <section class="post-view-comments">
                                            {can_comment.then(|| view! {
                                                <h1>"Add comment"</h1>
                                                <CommentForm post_id=id on_submit=on_comment_created />
                                            })}
                                            <CommentList post_id=id comments=comments_signal />
                                        </section>
                                    </div>
                                </div>

                                // Neighbor preloading
                                <Suspense fallback=|| ()>
                                    {move || Suspend::new(async move {
                                        neighbors.await.map(|n| {
                                            let prev_post_id = n.prev.as_ref().and_then(|p| p.id);
                                            let next_post_id = n.next.as_ref().and_then(|p| p.id);
                                            prev_id.set(prev_post_id);
                                            next_id.set(next_post_id);

                                            preload_neighbor(&n.prev);
                                            preload_neighbor(&n.next);
                                        })
                                    })}
                                </Suspense>
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

/// Preload a neighboring post's content image.
fn preload_neighbor(neighbor: &Option<oxibooru_shared::post::PostInfo>) {
    if let Some(post) = neighbor {
        let post_type = post.type_.unwrap_or(PostType::Image);
        if matches!(post_type, PostType::Image | PostType::Animation) {
            if let Some(url) = &post.content_url {
                if let Ok(img) = web_sys::HtmlImageElement::new() {
                    let _ = img.set_attribute("fetchpriority", "low");
                    img.set_src(url);
                }
            }
        }
    }
}
