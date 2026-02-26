use std::collections::HashMap;

use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use oxibooru_shared::enums::PostSafety;

use crate::api::posts::{CreatePostBody, ReverseSearchBody};
use crate::api::{ApiClient, ApiError};
use crate::components::file_dropper::FileDropper;
use crate::components::tag_input::TagInput;

/// Status of an individual uploadable item.
#[derive(Clone, PartialEq)]
enum UploadStatus {
    Pending,
    Uploading,
    Done,
    Error(String),
    Skipped,
}

/// A single item in the upload queue (file or URL).
#[derive(Clone)]
struct Uploadable {
    key: String,
    name: String,
    file: Option<web_sys::File>,
    url: Option<String>,
    preview_url: Option<String>,
    safety: PostSafety,
    anonymous: bool,
    status: UploadStatus,
}

#[component]
pub fn PostUploadPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let navigate = use_navigate();

    // Upload queue
    let uploadables: RwSignal<Vec<Uploadable>> = RwSignal::new(Vec::new());

    // Per-file tag signals, keyed by uploadable key. Stored separately to avoid
    // reactive loops (writing to uploadables from inside its own render closure).
    let per_file_tags: RwSignal<HashMap<String, RwSignal<Vec<String>>>> =
        RwSignal::new(HashMap::new());

    // Auto-detect mode: bulk when >1 item
    let is_bulk = Memo::new(move |_| uploadables.get().len() > 1);

    // Shared settings
    let tags = RwSignal::new(Vec::<String>::new());
    let (relate_all, set_relate_all) = signal(false);

    // Single-mode signals
    let (single_safety, set_single_safety) = signal(PostSafety::Safe);
    let (single_anonymous, set_single_anonymous) = signal(false);

    // Options
    let (skip_duplicate, set_skip_duplicate) = signal(false);
    let (force_upload_similar, set_force_upload_similar) = signal(false);
    let (pause_on_error, set_pause_on_error) = signal(true);

    // Upload state
    let (uploading, set_uploading) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);

    // Add files to queue
    let on_files = Callback::new(move |files: Vec<web_sys::File>| {
        uploadables.update(|list| {
            for file in files {
                let key = format!("{}_{}", file.name(), file.size());
                if list.iter().any(|u| u.key == key) {
                    continue;
                }
                let preview_url = web_sys::Url::create_object_url_with_blob(&file).ok();
                let k = key.clone();
                list.push(Uploadable {
                    key,
                    name: file.name(),
                    file: Some(file),
                    url: None,
                    preview_url,
                    safety: PostSafety::Safe,
                    anonymous: false,
                    status: UploadStatus::Pending,
                });
                // Create per-file tag signal
                per_file_tags.update(|map| {
                    map.entry(k).or_insert_with(|| RwSignal::new(Vec::new()));
                });
            }
        });
    });

    // Add URL to queue
    let on_url = Callback::new(move |url: String| {
        uploadables.update(|list| {
            if list.iter().any(|u| u.key == url) {
                return;
            }
            let name = url.rsplit('/').next().unwrap_or(&url).to_string();
            let k = url.clone();
            list.push(Uploadable {
                key: url.clone(),
                name,
                file: None,
                url: Some(url),
                preview_url: None,
                safety: PostSafety::Safe,
                anonymous: false,
                status: UploadStatus::Pending,
            });
            per_file_tags.update(|map| {
                map.entry(k).or_insert_with(|| RwSignal::new(Vec::new()));
            });
        });
    });

    // Remove item from queue
    let remove_item = move |key: String| {
        uploadables.update(|list| {
            if let Some(item) = list.iter().find(|u| u.key == key) {
                if let Some(url) = &item.preview_url {
                    let _ = web_sys::Url::revoke_object_url(url);
                }
            }
            list.retain(|u| u.key != key);
        });
        per_file_tags.update(|map| {
            map.remove(&key);
        });
    };

    // Move item up/down
    let move_item = move |key: String, direction: i32| {
        uploadables.update(|list| {
            if let Some(idx) = list.iter().position(|u| u.key == key) {
                let new_idx = (idx as i32 + direction) as usize;
                if new_idx < list.len() {
                    list.swap(idx, new_idx);
                }
            }
        });
    };

    // Set safety for an item
    let set_item_safety = move |key: String, safety: PostSafety| {
        uploadables.update(|list| {
            if let Some(item) = list.iter_mut().find(|u| u.key == key) {
                item.safety = safety;
            }
        });
    };

    // Set anonymous for an item
    let set_item_anonymous = move |key: String, anon: bool| {
        uploadables.update(|list| {
            if let Some(item) = list.iter_mut().find(|u| u.key == key) {
                item.anonymous = anon;
            }
        });
    };

    // Upload all items
    let nav = navigate.clone();
    let on_upload_all = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let items = uploadables.get_untracked();
        let pending: Vec<_> = items
            .iter()
            .enumerate()
            .filter(|(_, u)| u.status == UploadStatus::Pending)
            .map(|(i, _)| i)
            .collect();

        if pending.is_empty() {
            set_error_msg.set(Some("No files to upload.".into()));
            return;
        }

        let shared_tags = tags.get_untracked();
        let bulk = is_bulk.get_untracked();
        let skip_dup = skip_duplicate.get_untracked();
        let force_sim = force_upload_similar.get_untracked();
        let pause_err = pause_on_error.get_untracked();
        let s_safety = single_safety.get_untracked();
        let s_anon = single_anonymous.get_untracked();
        let relate = relate_all.get_untracked();

        // Snapshot per-file tags before starting upload
        let file_tags_snapshot: HashMap<String, Vec<String>> = per_file_tags
            .get_untracked()
            .iter()
            .map(|(k, sig)| (k.clone(), sig.get_untracked()))
            .collect();

        set_uploading.set(true);
        set_error_msg.set(None);
        set_success_msg.set(None);

        let client = api.get_untracked();
        let nav = nav.clone();

        leptos::task::spawn_local(async move {
            let mut created_post_ids: Vec<i64> = Vec::new();

            for idx in pending {
                let item = uploadables.get_untracked()[idx].clone();

                // Update status to uploading
                uploadables.update(|list| {
                    if let Some(u) = list.get_mut(idx) {
                        u.status = UploadStatus::Uploading;
                    }
                });

                // Step 1: Upload file to get token (if file-based)
                let (content_token, content_url) = if let Some(file) = &item.file {
                    let form = web_sys::FormData::new().unwrap();
                    let _ = form.append_with_blob("content", file);
                    match client.upload_file(&form).await {
                        Ok(resp) => (Some(resp.token), None),
                        Err(e) => {
                            let msg = format!("{}: Upload failed — {e}", item.name);
                            uploadables.update(|list| {
                                if let Some(u) = list.get_mut(idx) {
                                    u.status = UploadStatus::Error(msg.clone());
                                }
                            });
                            if pause_err {
                                set_error_msg.set(Some(msg));
                                set_uploading.set(false);
                                return;
                            }
                            continue;
                        }
                    }
                } else {
                    (None, item.url.clone())
                };

                // Step 2: Duplicate check via reverse search
                if skip_dup || !force_sim {
                    let search_body = ReverseSearchBody {
                        content_token: content_token.clone(),
                        content_url: content_url.clone(),
                    };
                    if let Ok(search_result) = client.reverse_search_json(&search_body).await {
                        if search_result.exact_post.is_some() && skip_dup {
                            uploadables.update(|list| {
                                if let Some(u) = list.get_mut(idx) {
                                    u.status = UploadStatus::Skipped;
                                }
                            });
                            continue;
                        }
                        if !search_result.similar_posts.is_empty() && !force_sim {
                            let msg = format!("{}: Similar posts found. Check 'Force upload similar' to override.", item.name);
                            uploadables.update(|list| {
                                if let Some(u) = list.get_mut(idx) {
                                    u.status = UploadStatus::Error(msg.clone());
                                }
                            });
                            if pause_err {
                                set_error_msg.set(Some(msg));
                                set_uploading.set(false);
                                return;
                            }
                            continue;
                        }
                    }
                }

                // Step 3: Create post
                let safety = if bulk { item.safety } else { s_safety };
                let anon = if bulk { item.anonymous } else { s_anon };

                // Merge shared tags + per-file tags
                let mut all_tags = shared_tags.clone();
                if let Some(file_tags) = file_tags_snapshot.get(&item.key) {
                    for t in file_tags {
                        if !all_tags.iter().any(|existing| existing.eq_ignore_ascii_case(t)) {
                            all_tags.push(t.clone());
                        }
                    }
                }

                // Relations: if "relate all" is checked, include all previously created post IDs
                let relations = if relate && !created_post_ids.is_empty() {
                    Some(created_post_ids.clone())
                } else {
                    None
                };

                let body = CreatePostBody {
                    safety,
                    content_token,
                    content_url,
                    thumbnail_token: None,
                    source: None,
                    description: None,
                    relations,
                    anonymous: if anon { Some(true) } else { None },
                    tags: if all_tags.is_empty() { None } else { Some(all_tags) },
                    notes: None,
                    flags: None,
                };

                match client.create_post_json(&body).await {
                    Ok(post) => {
                        if let Some(id) = post.id {
                            created_post_ids.push(id);
                        }
                        uploadables.update(|list| {
                            if let Some(u) = list.get_mut(idx) {
                                u.status = UploadStatus::Done;
                            }
                        });
                    }
                    Err(e) => {
                        let msg = format!("{}: {}", item.name, format_api_error(&e));
                        uploadables.update(|list| {
                            if let Some(u) = list.get_mut(idx) {
                                u.status = UploadStatus::Error(msg.clone());
                            }
                        });
                        if pause_err {
                            set_error_msg.set(Some(msg));
                            set_uploading.set(false);
                            return;
                        }
                    }
                }
            }

            set_uploading.set(false);

            let items = uploadables.get_untracked();
            let all_done = items.iter().all(|u| matches!(u.status, UploadStatus::Done | UploadStatus::Skipped));
            if all_done {
                if created_post_ids.len() == 1 {
                    let id = created_post_ids[0];
                    nav(&format!("/post/{id}"), Default::default());
                } else if !created_post_ids.is_empty() {
                    set_success_msg.set(Some(format!("All {} posts uploaded.", created_post_ids.len())));
                }
            }
        });
    };

    let has_items = Memo::new(move |_| !uploadables.get().is_empty());

    view! {
        <Title text="Upload" />
        <div id="post-upload">
            <form on:submit=on_upload_all>
                // File dropper
                <FileDropper
                    on_files=on_files
                    on_url=on_url
                    label="Drop files here!\nOr just click on this box."
                    sublabel="Allowed extensions: .jpg, .png, .gif, .webm, .mp4, .swf, .avif, .heif, .heic"
                />

                // Control strip
                <div class="control-strip">
                    <button type="submit" class="btn-primary" disabled=move || uploading.get() || !has_items.get()>
                        {move || if uploading.get() { "Uploading..." } else { "Upload all" }}
                    </button>
                    <ul class="input inline">
                        <li>
                            <label>
                                <input
                                    type="checkbox"
                                    prop:checked=move || skip_duplicate.get()
                                    on:change=move |ev| set_skip_duplicate.set(event_target_checked(&ev))
                                    disabled=move || uploading.get()
                                />
                                <span class="checkbox">"Skip duplicate"</span>
                            </label>
                        </li>
                        <li>
                            <label>
                                <input
                                    type="checkbox"
                                    prop:checked=move || force_upload_similar.get()
                                    on:change=move |ev| set_force_upload_similar.set(event_target_checked(&ev))
                                    disabled=move || uploading.get()
                                />
                                <span class="checkbox">"Force upload similar"</span>
                            </label>
                        </li>
                        <li>
                            <label>
                                <input
                                    type="checkbox"
                                    prop:checked=move || pause_on_error.get()
                                    on:change=move |ev| set_pause_on_error.set(event_target_checked(&ev))
                                    disabled=move || uploading.get()
                                />
                                <span class="checkbox">"Pause on error"</span>
                            </label>
                        </li>
                    </ul>
                </div>

                // Messages
                {move || error_msg.get().map(|msg| view! { <p class="error messages">{msg}</p> })}
                {move || success_msg.get().map(|msg| view! { <p class="success messages">{msg}</p> })}

                // Shared settings (visible when files are queued)
                <div class="shared-settings" style:display=move || if has_items.get() { "" } else { "none" }>
                    <div class="form-row">
                        <TagInput tags=tags label="Tags (all posts)" />
                    </div>
                    <ul class="input">
                        <li>
                            <label>
                                <input
                                    type="checkbox"
                                    prop:checked=move || relate_all.get()
                                    on:change=move |ev| set_relate_all.set(event_target_checked(&ev))
                                    disabled=move || uploading.get()
                                />
                                <span class="checkbox">"Relate all uploads to each other"</span>
                            </label>
                        </li>
                    </ul>
                </div>

                // Single mode: safety + anonymous (only when exactly 1 item)
                <div
                    class="single-settings"
                    style:display=move || if !is_bulk.get() && has_items.get() { "" } else { "none" }
                >
                    <div class="form-row">
                        <label>"Safety"</label>
                        <div class="safety-radios">
                            <label class:selected=move || single_safety.get() == PostSafety::Safe>
                                <input
                                    type="radio"
                                    name="single-safety"
                                    prop:checked=move || single_safety.get() == PostSafety::Safe
                                    on:change=move |_| set_single_safety.set(PostSafety::Safe)
                                />
                                " Safe"
                            </label>
                            <label class:selected=move || single_safety.get() == PostSafety::Sketchy>
                                <input
                                    type="radio"
                                    name="single-safety"
                                    prop:checked=move || single_safety.get() == PostSafety::Sketchy
                                    on:change=move |_| set_single_safety.set(PostSafety::Sketchy)
                                />
                                " Sketchy"
                            </label>
                            <label class:selected=move || single_safety.get() == PostSafety::Unsafe>
                                <input
                                    type="radio"
                                    name="single-safety"
                                    prop:checked=move || single_safety.get() == PostSafety::Unsafe
                                    on:change=move |_| set_single_safety.set(PostSafety::Unsafe)
                                />
                                " Unsafe"
                            </label>
                        </div>
                    </div>
                    <ul class="input">
                        <li>
                            <label>
                                <input
                                    type="checkbox"
                                    prop:checked=move || single_anonymous.get()
                                    on:change=move |ev| set_single_anonymous.set(event_target_checked(&ev))
                                />
                                <span class="checkbox">"Upload anonymously"</span>
                            </label>
                        </li>
                    </ul>
                </div>

                // Uploadables list
                <ul class="uploadables-container">
                    {move || uploadables.get().into_iter().map(|item| {
                        let key = item.key.clone();
                        let key_remove = key.clone();
                        let key_up = key.clone();
                        let key_down = key.clone();
                        let bulk = is_bulk.get();

                        let status_class = match &item.status {
                            UploadStatus::Pending => "",
                            UploadStatus::Uploading => "uploading",
                            UploadStatus::Done => "done",
                            UploadStatus::Error(_) => "error",
                            UploadStatus::Skipped => "skipped",
                        };
                        let status_msg = match &item.status {
                            UploadStatus::Error(msg) => Some(msg.clone()),
                            UploadStatus::Skipped => Some("Skipped (duplicate)".into()),
                            UploadStatus::Done => Some("Uploaded".into()),
                            UploadStatus::Uploading => Some("Uploading...".into()),
                            _ => None,
                        };

                        let name = item.name.clone();
                        let safety = item.safety;
                        let anon = item.anonymous;
                        let preview = item.preview_url.clone();

                        let container_class = format!("uploadable-container {status_class}");

                        // Look up the per-file tag signal for this item
                        let item_tags = per_file_tags
                            .get_untracked()
                            .get(&key)
                            .copied()
                            .unwrap_or_else(|| RwSignal::new(Vec::new()));

                        view! {
                            <li class=container_class>
                                {preview.map(|url| view! {
                                    <div class="thumbnail-wrapper">
                                        <img src=url />
                                    </div>
                                })}
                                <div class="uploadable">
                                    <header>
                                        <span class="filename">{name}</span>
                                        <nav>
                                            {bulk.then(|| view! {
                                                <button type="button" class="btn-icon" title="Move up"
                                                    on:click=move |_| move_item(key_up.clone(), -1)>
                                                    <i class="fa fa-chevron-up" />
                                                </button>
                                                <button type="button" class="btn-icon" title="Move down"
                                                    on:click=move |_| move_item(key_down.clone(), 1)>
                                                    <i class="fa fa-chevron-down" />
                                                </button>
                                            })}
                                            <button type="button" class="btn-icon remove" title="Remove"
                                                on:click=move |_| remove_item(key_remove.clone())>
                                                <i class="fa fa-times" />
                                            </button>
                                        </nav>
                                    </header>
                                    {bulk.then(|| {
                                        let key_safe = key.clone();
                                        let key_sketchy = key.clone();
                                        let key_unsafe = key.clone();
                                        let key_anon = key.clone();
                                        view! {
                                            <div class="body">
                                                <div class="safety-radios">
                                                    <label class:selected=move || safety == PostSafety::Safe>
                                                        <input
                                                            type="radio"
                                                            prop:checked=safety == PostSafety::Safe
                                                            on:change=move |_| set_item_safety(key_safe.clone(), PostSafety::Safe)
                                                        />
                                                        " Safe"
                                                    </label>
                                                    <label class:selected=move || safety == PostSafety::Sketchy>
                                                        <input
                                                            type="radio"
                                                            prop:checked=safety == PostSafety::Sketchy
                                                            on:change=move |_| set_item_safety(key_sketchy.clone(), PostSafety::Sketchy)
                                                        />
                                                        " Sketchy"
                                                    </label>
                                                    <label class:selected=move || safety == PostSafety::Unsafe>
                                                        <input
                                                            type="radio"
                                                            prop:checked=safety == PostSafety::Unsafe
                                                            on:change=move |_| set_item_safety(key_unsafe.clone(), PostSafety::Unsafe)
                                                        />
                                                        " Unsafe"
                                                    </label>
                                                </div>
                                                <ul class="input">
                                                    <li>
                                                        <label>
                                                            <input
                                                                type="checkbox"
                                                                prop:checked=anon
                                                                on:change=move |ev| set_item_anonymous(key_anon.clone(), event_target_checked(&ev))
                                                            />
                                                            <span class="checkbox">"Upload anonymously"</span>
                                                        </label>
                                                    </li>
                                                </ul>
                                                <div class="per-file-tags">
                                                    <TagInput tags=item_tags label="Tags (this post)" />
                                                </div>
                                            </div>
                                        }
                                    })}
                                    {status_msg.map(|msg| view! {
                                        <div class="status-msg">{msg}</div>
                                    })}
                                </div>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </form>
        </div>
    }
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
