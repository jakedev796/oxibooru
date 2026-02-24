use std::collections::HashSet;

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use oxibooru_shared::enums::PostSafety;
use oxibooru_shared::post::PostInfo;
use oxibooru_shared::request::DeleteBody;

use wasm_bindgen::JsCast;

use crate::api::posts::UpdatePostBody;
use crate::api::{ApiClient, ApiError};
use crate::auth::AuthState;
use crate::components::pagination::Pagination;
use crate::components::post_thumbnail::PostThumbnail;
use crate::components::search_bar::SearchBar;
use crate::keyboard::KeyboardShortcuts;
use crate::settings::SettingsState;
use crate::utils::{build_list_url, setup_scroll_listener, use_list_query_params};

const FIELDS: &str = "id,thumbnailUrl,type,safety,score,favoriteCount,commentCount,tags,version";

#[component]
pub fn PostListPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let settings = expect_context::<SettingsState>();
    let default_limit = settings.inner.get_untracked().posts_per_page as i64;
    let params = use_list_query_params(default_limit);
    let navigate = use_navigate();
    let endless = settings.inner.get_untracked().endless_scroll;

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

    // Bulk selection state
    let selecting = RwSignal::new(false);
    let selected = RwSignal::new(HashSet::<i64>::new());
    let post_data = RwSignal::new(Vec::<PostInfo>::new());

    let (bulk_error, set_bulk_error) = signal(Option::<String>::None);
    let (bulk_working, set_bulk_working) = signal(false);
    let (show_tag_dialog, set_show_tag_dialog) = signal(false);
    let (bulk_tags_str, set_bulk_tags_str) = signal(String::new());
    let (show_safety_dialog, set_show_safety_dialog) = signal(false);
    let (bulk_safety, set_bulk_safety) = signal("safe".to_string());

    let toggle_select_mode = move |_| {
        let new_val = !selecting.get_untracked();
        selecting.set(new_val);
        if !new_val {
            selected.set(HashSet::new());
        }
    };

    let toggle_post = Callback::new(move |id: i64| {
        selected.update(|set| {
            if set.contains(&id) {
                set.remove(&id);
            } else {
                set.insert(id);
            }
        });
    });

    let select_all = move |_| {
        let all_ids: HashSet<i64> = post_data
            .get_untracked()
            .iter()
            .filter_map(|p| p.id)
            .collect();
        selected.set(all_ids);
    };

    let deselect_all = move |_| {
        selected.set(HashSet::new());
    };

    // Bulk add tags
    let do_bulk_tag = move |_| {
        let tags_to_add: Vec<String> = bulk_tags_str
            .get_untracked()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if tags_to_add.is_empty() {
            return;
        }

        let sel = selected.get_untracked();
        let data = post_data.get_untracked();
        let client = api.get_untracked();
        set_bulk_working.set(true);
        set_bulk_error.set(None);

        leptos::task::spawn_local(async move {
            let mut errors = Vec::new();
            for post in data.iter().filter(|p| p.id.map_or(false, |id| sel.contains(&id))) {
                let id = post.id.unwrap();
                let version = post.version.clone().unwrap_or_default();
                let mut existing: Vec<String> = post
                    .tags
                    .as_ref()
                    .map(|tags| {
                        tags.iter()
                            .flat_map(|t| t.names.first().cloned())
                            .collect()
                    })
                    .unwrap_or_default();
                for tag in &tags_to_add {
                    if !existing.iter().any(|t| t.eq_ignore_ascii_case(tag)) {
                        existing.push(tag.clone());
                    }
                }
                let body = UpdatePostBody {
                    version,
                    safety: None,
                    source: None,
                    description: None,
                    relations: None,
                    tags: Some(existing),
                    notes: None,
                    flags: None,
                    content_token: None,
                    content_url: None,
                    thumbnail_token: None,
                };
                if let Err(e) = client.update_post_json(id, &body).await {
                    errors.push(format!("Post {id}: {}", format_api_error(&e)));
                }
            }
            set_bulk_working.set(false);
            if !errors.is_empty() {
                set_bulk_error.set(Some(errors.join("; ")));
            } else {
                set_show_tag_dialog.set(false);
                set_bulk_tags_str.set(String::new());
                selected.set(HashSet::new());
                selecting.set(false);
            }
        });
    };

    // Bulk change safety
    let do_bulk_safety = move |_| {
        let safety = match bulk_safety.get_untracked().as_str() {
            "sketchy" => PostSafety::Sketchy,
            "unsafe" => PostSafety::Unsafe,
            _ => PostSafety::Safe,
        };
        let sel = selected.get_untracked();
        let data = post_data.get_untracked();
        let client = api.get_untracked();
        set_bulk_working.set(true);
        set_bulk_error.set(None);

        leptos::task::spawn_local(async move {
            let mut errors = Vec::new();
            for post in data.iter().filter(|p| p.id.map_or(false, |id| sel.contains(&id))) {
                let id = post.id.unwrap();
                let version = post.version.clone().unwrap_or_default();
                let body = UpdatePostBody {
                    version,
                    safety: Some(safety),
                    source: None,
                    description: None,
                    relations: None,
                    tags: None,
                    notes: None,
                    flags: None,
                    content_token: None,
                    content_url: None,
                    thumbnail_token: None,
                };
                if let Err(e) = client.update_post_json(id, &body).await {
                    errors.push(format!("Post {id}: {}", format_api_error(&e)));
                }
            }
            set_bulk_working.set(false);
            if !errors.is_empty() {
                set_bulk_error.set(Some(errors.join("; ")));
            } else {
                set_show_safety_dialog.set(false);
                selected.set(HashSet::new());
                selecting.set(false);
            }
        });
    };

    // Bulk delete
    let do_bulk_delete = move |_| {
        let sel = selected.get_untracked();
        let data = post_data.get_untracked();
        let client = api.get_untracked();
        set_bulk_working.set(true);
        set_bulk_error.set(None);

        leptos::task::spawn_local(async move {
            let mut errors = Vec::new();
            for post in data.iter().filter(|p| p.id.map_or(false, |id| sel.contains(&id))) {
                let id = post.id.unwrap();
                let version = post.version.clone().unwrap_or_default();
                let body = DeleteBody { version };
                if let Err(e) = client.delete_post(id, &body).await {
                    errors.push(format!("Post {id}: {}", format_api_error(&e)));
                }
            }
            set_bulk_working.set(false);
            if !errors.is_empty() {
                set_bulk_error.set(Some(errors.join("; ")));
            } else {
                selected.set(HashSet::new());
                selecting.set(false);
            }
        });
    };

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
        let query = params.get_untracked().query.clone();
        let offset = loaded_up_to.get_untracked();
        let limit = params.get_untracked().limit;

        leptos::task::spawn_local(async move {
            match client.get_posts(&query, offset, limit, FIELDS).await {
                Ok(data) => {
                    let new_count = data.results.len() as i64;
                    accumulated.update(|v| v.extend(data.results.clone()));
                    post_data.update(|v| v.extend(data.results));
                    loaded_up_to.set(offset + new_count);
                    total_results.set(data.total);
                }
                Err(_) => {}
            }
            loading_more.set(false);
        });
    };

    // Set up scroll listener for endless scroll mode
    if endless {
        setup_scroll_listener(loading_more, has_more, move || load_more());
    }

    // P — focus first post thumbnail
    let shortcuts = expect_context::<KeyboardShortcuts>();
    shortcuts.register("p", Callback::new(move |()| {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.query_selector(".post-thumbnail a").ok().flatten() {
                if let Ok(link) = el.dyn_into::<web_sys::HtmlElement>() {
                    let _ = link.focus();
                }
            }
        }
    }));
    on_cleanup(move || {
        shortcuts.unregister("p");
    });

    // Check bulk privileges
    let can_bulk_tag = auth.has_privilege("post_bulk_edit_tag");
    let can_bulk_safety = auth.has_privilege("post_bulk_edit_safety");
    let can_bulk_delete = auth.has_privilege("post_bulk_edit_delete");
    let can_bulk = can_bulk_tag || can_bulk_safety || can_bulk_delete;

    view! {
        <Title text="Posts" />
        <div class="post-list-page">
            <SearchBar query=query_signal on_submit=on_search />

            // Bulk select toolbar
            {can_bulk.then(|| view! {
                <div class="bulk-toolbar">
                    <button type="button" class="btn-select" on:click=toggle_select_mode>
                        {move || if selecting.get() { "Cancel selection" } else { "Select" }}
                    </button>
                    {move || selecting.get().then(|| {
                        let count = selected.get().len();
                        view! {
                            <span class="bulk-count">{count}" selected"</span>
                            <button type="button" on:click=select_all>"All"</button>
                            <button type="button" on:click=deselect_all>"None"</button>
                            {can_bulk_tag.then(|| view! {
                                <button type="button"
                                    disabled=move || selected.get().is_empty() || bulk_working.get()
                                    on:click=move |_| set_show_tag_dialog.set(true)
                                >"Tag"</button>
                            })}
                            {can_bulk_safety.then(|| view! {
                                <button type="button"
                                    disabled=move || selected.get().is_empty() || bulk_working.get()
                                    on:click=move |_| set_show_safety_dialog.set(true)
                                >"Safety"</button>
                            })}
                            {can_bulk_delete.then(|| view! {
                                <button type="button" class="btn-danger"
                                    disabled=move || selected.get().is_empty() || bulk_working.get()
                                    on:click=move |_| {
                                        let count = selected.get_untracked().len();
                                        let win = web_sys::window().unwrap();
                                        if win.confirm_with_message(&format!("Delete {count} post(s)?")).unwrap_or(false) {
                                            do_bulk_delete(());
                                        }
                                    }
                                >"Delete"</button>
                            })}
                        }
                    })}
                </div>
            })}

            // Bulk tag dialog
            {move || show_tag_dialog.get().then(|| view! {
                <div class="bulk-dialog">
                    <h3>"Add tags to selected posts"</h3>
                    <input
                        type="text"
                        placeholder="tag1, tag2, ..."
                        prop:value=move || bulk_tags_str.get()
                        on:input=move |ev| set_bulk_tags_str.set(event_target_value(&ev))
                        disabled=move || bulk_working.get()
                    />
                    <div class="bulk-dialog-actions">
                        <button type="button" on:click=do_bulk_tag disabled=move || bulk_working.get()>
                            {move || if bulk_working.get() { "Applying..." } else { "Apply" }}
                        </button>
                        <button type="button" on:click=move |_| set_show_tag_dialog.set(false)>"Cancel"</button>
                    </div>
                </div>
            })}

            // Bulk safety dialog
            {move || show_safety_dialog.get().then(|| view! {
                <div class="bulk-dialog">
                    <h3>"Change safety of selected posts"</h3>
                    <select
                        prop:value=move || bulk_safety.get()
                        on:change=move |ev| set_bulk_safety.set(event_target_value(&ev))
                        disabled=move || bulk_working.get()
                    >
                        <option value="safe">"Safe"</option>
                        <option value="sketchy">"Sketchy"</option>
                        <option value="unsafe">"Unsafe"</option>
                    </select>
                    <div class="bulk-dialog-actions">
                        <button type="button" on:click=do_bulk_safety disabled=move || bulk_working.get()>
                            {move || if bulk_working.get() { "Applying..." } else { "Apply" }}
                        </button>
                        <button type="button" on:click=move |_| set_show_safety_dialog.set(false)>"Cancel"</button>
                    </div>
                </div>
            })}

            // Bulk error
            {move || bulk_error.get().map(|msg| view! { <p class="error">{msg}</p> })}

            <Suspense fallback=|| view! { <p>"Loading posts..."</p> }>
                {move || Suspend::new(async move {
                    match posts.await {
                        Some(data) => {
                            if endless {
                                // Reset accumulated state from initial/new page
                                accumulated.set(data.results.clone());
                                post_data.set(data.results.clone());
                                loaded_up_to.set(data.offset + data.results.len() as i64);
                                total_results.set(data.total);

                                view! {
                                    <div class="post-grid">
                                        <For
                                            each=move || accumulated.get()
                                            key=|post| post.id.unwrap_or(0)
                                            children=move |post| {
                                                let id = post.id.unwrap_or(0);
                                                let thumbnail_url = post.thumbnail_url.clone().unwrap_or_default();
                                                let safety = post.safety.unwrap_or(PostSafety::Safe);
                                                let post_type = post.type_.unwrap_or(oxibooru_shared::enums::PostType::Image);
                                                let score = post.score;
                                                let favorite_count = post.favorite_count;
                                                let comment_count = post.comment_count;
                                                let is_selecting = selecting;
                                                let is_selected = Signal::derive(move || selected.get().contains(&id));
                                                let on_toggle = toggle_post;
                                                view! {
                                                    <div
                                                        class="post-thumbnail-wrapper"
                                                        class:selected=is_selected
                                                        on:click=move |_| {
                                                            if is_selecting.get_untracked() {
                                                                on_toggle.run(id);
                                                            }
                                                        }
                                                    >
                                                        {move || is_selecting.get().then(|| view! {
                                                            <div class="select-overlay">
                                                                <input
                                                                    type="checkbox"
                                                                    prop:checked=is_selected
                                                                    on:click=move |ev| {
                                                                        ev.stop_propagation();
                                                                        on_toggle.run(id);
                                                                    }
                                                                />
                                                            </div>
                                                        })}
                                                        <PostThumbnail
                                                            id=id
                                                            thumbnail_url=thumbnail_url
                                                            safety=safety
                                                            post_type=post_type
                                                            score=score
                                                            favorite_count=favorite_count
                                                            comment_count=comment_count
                                                        />
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                    <div class="scroll-sentinel">
                                        {move || loading_more.get().then(|| view! {
                                            <p class="loading-more">"Loading more posts..."</p>
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
                                // Pagination mode
                                post_data.set(data.results.clone());
                                let total = data.total;
                                let offset = data.offset;
                                let limit = data.limit;
                                let query_for_page = params.get().query.clone();
                                view! {
                                    <div class="post-grid">
                                        {data.results.into_iter().map(|post| {
                                            let id = post.id.unwrap_or(0);
                                            let thumbnail_url = post.thumbnail_url.clone().unwrap_or_default();
                                            let safety = post.safety.unwrap_or(PostSafety::Safe);
                                            let post_type = post.type_.unwrap_or(oxibooru_shared::enums::PostType::Image);
                                            let score = post.score;
                                            let favorite_count = post.favorite_count;
                                            let comment_count = post.comment_count;
                                            let is_selecting = selecting;
                                            let is_selected = Signal::derive(move || selected.get().contains(&id));
                                            let on_toggle = toggle_post;
                                            view! {
                                                <div
                                                    class="post-thumbnail-wrapper"
                                                    class:selected=is_selected
                                                    on:click=move |_| {
                                                        if is_selecting.get_untracked() {
                                                            on_toggle.run(id);
                                                        }
                                                    }
                                                >
                                                    {move || is_selecting.get().then(|| view! {
                                                        <div class="select-overlay">
                                                            <input
                                                                type="checkbox"
                                                                prop:checked=is_selected
                                                                on:click=move |ev| {
                                                                    ev.stop_propagation();
                                                                    on_toggle.run(id);
                                                                }
                                                            />
                                                        </div>
                                                    })}
                                                    <PostThumbnail
                                                        id=id
                                                        thumbnail_url=thumbnail_url
                                                        safety=safety
                                                        post_type=post_type
                                                        score=score
                                                        favorite_count=favorite_count
                                                        comment_count=comment_count
                                                    />
                                                </div>
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

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
