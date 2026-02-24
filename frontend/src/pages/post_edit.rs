use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_params_map};
use oxibooru_shared::enums::PostSafety;
use oxibooru_shared::request::DeleteBody;
use wasm_bindgen::JsCast;

use crate::api::posts::{FeatureBody, UpdatePostBody};
use crate::api::{ApiClient, ApiError};
use crate::auth::AuthState;
use crate::components::expander::Expander;
use crate::components::file_dropper::FileDropper;
use crate::components::tag_input::TagInput;
use crate::keyboard::KeyboardShortcuts;
use oxibooru_shared::post::Note;

#[component]
pub fn PostEditPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let params = use_params_map();
    let navigate = use_navigate();

    let post_id = move || {
        params
            .get()
            .get("id")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };

    // Loading state
    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);

    // Form signals
    let (safety, set_safety) = signal(String::new());
    let tags = RwSignal::new(Vec::<String>::new());
    let (source, set_source) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (relations_str, set_relations_str) = signal(String::new());
    let (flag_loop, set_flag_loop) = signal(false);
    let (flag_sound, set_flag_sound) = signal(false);
    let (version, set_version) = signal(String::new());

    let notes_signal: RwSignal<Vec<Note>> = RwSignal::new(Vec::new());

    let (content_token, set_content_token) = signal(Option::<String>::None);
    let (thumbnail_token, set_thumbnail_token) = signal(Option::<String>::None);
    let (uploading, set_uploading) = signal(false);

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);

    // Load post data
    Effect::new(move || {
        let client = api.get_untracked();
        let id = post_id();
        leptos::task::spawn_local(async move {
            match client.get_post(id).await {
                Ok(post) => {
                    set_safety.set(
                        post.safety
                            .map(|s| format!("{s:?}").to_lowercase())
                            .unwrap_or_else(|| "safe".into()),
                    );
                    let tag_names: Vec<String> = post
                        .tags
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|t| t.names.into_iter().next())
                        .collect();
                    tags.set(tag_names);
                    set_source.set(post.source.unwrap_or_default());
                    set_description.set(post.description.unwrap_or_default());
                    let rels: Vec<String> = post
                        .relations
                        .unwrap_or_default()
                        .iter()
                        .map(|r| r.id.to_string())
                        .collect();
                    set_relations_str.set(rels.join(", "));
                    let flags = post.flags.unwrap_or_default();
                    set_flag_loop.set(flags.iter().any(|f| f == "loop"));
                    set_flag_sound.set(flags.iter().any(|f| f == "sound"));
                    set_version.set(post.version.unwrap_or_default());
                    notes_signal.set(post.notes.unwrap_or_default());
                    set_loading.set(false);
                }
                Err(_) => {
                    set_load_error.set(true);
                    set_loading.set(false);
                }
            }
        });
    });

    // Upload replacement content
    let on_content_file = Callback::new(move |file: web_sys::File| {
        let client = api.get_untracked();
        set_uploading.set(true);
        set_error_msg.set(None);
        leptos::task::spawn_local(async move {
            let form = web_sys::FormData::new().unwrap();
            let _ = form.append_with_blob("content", &file);
            match client.upload_file(&form).await {
                Ok(resp) => set_content_token.set(Some(resp.token)),
                Err(e) => set_error_msg.set(Some(format!("Upload failed: {e}"))),
            }
            set_uploading.set(false);
        });
    });

    // Upload replacement thumbnail
    let on_thumb_file = Callback::new(move |file: web_sys::File| {
        let client = api.get_untracked();
        set_uploading.set(true);
        set_error_msg.set(None);
        leptos::task::spawn_local(async move {
            let form = web_sys::FormData::new().unwrap();
            let _ = form.append_with_blob("content", &file);
            match client.upload_file(&form).await {
                Ok(resp) => set_thumbnail_token.set(Some(resp.token)),
                Err(e) => set_error_msg.set(Some(format!("Thumbnail upload failed: {e}"))),
            }
            set_uploading.set(false);
        });
    });

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let safety_val = match safety.get_untracked().as_str() {
            "sketchy" => Some(PostSafety::Sketchy),
            "unsafe" => Some(PostSafety::Unsafe),
            "safe" => Some(PostSafety::Safe),
            _ => None,
        };

        let tag_list = tags.get_untracked();
        let src = source.get_untracked();
        let desc = description.get_untracked();
        let rels: Option<Vec<i64>> = {
            let s = relations_str.get_untracked();
            if s.trim().is_empty() {
                Some(vec![])
            } else {
                let parsed: Result<Vec<i64>, _> = s
                    .split(',')
                    .map(|p| p.trim().parse::<i64>())
                    .collect();
                match parsed {
                    Ok(ids) => Some(ids),
                    Err(_) => {
                        set_error_msg.set(Some("Invalid relation IDs.".into()));
                        return;
                    }
                }
            }
        };

        let mut flags = Vec::new();
        if flag_loop.get_untracked() {
            flags.push("loop".to_string());
        }
        if flag_sound.get_untracked() {
            flags.push("sound".to_string());
        }

        let body = UpdatePostBody {
            version: version.get_untracked(),
            safety: safety_val,
            source: Some(src),
            description: Some(desc),
            relations: rels,
            tags: Some(tag_list),
            notes: None,
            flags: Some(flags),
            content_token: content_token.get_untracked(),
            content_url: None,
            thumbnail_token: thumbnail_token.get_untracked(),
        };

        set_submitting.set(true);
        set_error_msg.set(None);
        set_success_msg.set(None);
        let client = api.get_untracked();
        let id = post_id();

        leptos::task::spawn_local(async move {
            match client.update_post_json(id, &body).await {
                Ok(updated) => {
                    set_version.set(updated.version.unwrap_or_default());
                    set_success_msg.set(Some("Post updated.".into()));
                    set_submitting.set(false);
                    set_content_token.set(None);
                    set_thumbnail_token.set(None);
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    // Delete handler
    let nav_delete = navigate.clone();
    let on_delete = move |_| {
        let confirmed = web_sys::window()
            .and_then(|w| w.confirm_with_message("Delete this post?").ok())
            .unwrap_or(false);
        if !confirmed {
            return;
        }
        let client = api.get_untracked();
        let id = post_id();
        let ver = version.get_untracked();
        let nav = nav_delete.clone();
        leptos::task::spawn_local(async move {
            if client
                .delete_post(id, &DeleteBody { version: ver })
                .await
                .is_ok()
            {
                nav("/posts", Default::default());
            }
        });
    };

    // Feature handler
    let on_feature = move |_| {
        let client = api.get_untracked();
        let id = post_id();
        leptos::task::spawn_local(async move {
            match client.feature_post(&FeatureBody { id }).await {
                Ok(_) => {
                    set_success_msg.set(Some("Post featured!".into()));
                }
                Err(e) => set_error_msg.set(Some(format_api_error(&e))),
            }
        });
    };

    let can_delete =
        auth.has_privilege("posts:delete:own") || auth.has_privilege("posts:delete:any");
    let can_feature = auth.has_privilege("posts:feature");

    // Keyboard shortcuts: Ctrl+S to save
    let shortcuts = expect_context::<KeyboardShortcuts>();

    let save_form_ref: NodeRef<leptos::html::Form> = NodeRef::new();
    shortcuts.register("ctrl+s", Callback::new(move |()| {
        if let Some(form) = save_form_ref.get() {
            let _ = form.request_submit();
        }
    }));

    // T to focus tag input
    shortcuts.register("t", Callback::new(move |()| {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.get_element_by_id("tag-input") {
                if let Ok(input) = el.dyn_into::<web_sys::HtmlElement>() {
                    let _ = input.focus();
                }
            }
        }
    }));

    // Delete — trigger post deletion (with confirmation dialog)
    if can_delete {
        let nav_kb_delete = navigate.clone();
        shortcuts.register("Delete", Callback::new(move |()| {
            let confirmed = web_sys::window()
                .and_then(|w| w.confirm_with_message("Delete this post?").ok())
                .unwrap_or(false);
            if !confirmed {
                return;
            }
            let client = api.get_untracked();
            let id = post_id();
            let ver = version.get_untracked();
            let nav = nav_kb_delete.clone();
            leptos::task::spawn_local(async move {
                if client
                    .delete_post(id, &DeleteBody { version: ver })
                    .await
                    .is_ok()
                {
                    nav("/posts", Default::default());
                }
            });
        }));
    }

    on_cleanup(move || {
        shortcuts.unregister("ctrl+s");
        shortcuts.unregister("t");
        shortcuts.unregister("Delete");
    });

    view! {
        <Title text=move || format!("Edit Post {}", post_id()) />
        <div class="content-wrapper">
            <h1>{move || format!("Edit Post #{}", post_id())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Post not found."</p> })}
            <form
                class="form-grid"
                node_ref=save_form_ref
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}
                {move || success_msg.get().map(|msg| view! { <p class="success">{msg}</p> })}

                <div class="form-row">
                    <label for="safety">"Safety"</label>
                    <select
                        id="safety"
                        prop:value=move || safety.get()
                        on:change=move |ev| set_safety.set(event_target_value(&ev))
                        disabled=move || submitting.get()
                    >
                        <option value="safe">"Safe"</option>
                        <option value="sketchy">"Sketchy"</option>
                        <option value="unsafe">"Unsafe"</option>
                    </select>
                </div>

                <div class="form-row">
                    <TagInput tags=tags label="Tags" />
                </div>

                <div class="form-row">
                    <label for="source">"Source"</label>
                    <input
                        id="source"
                        type="text"
                        prop:value=move || source.get()
                        on:input=move |ev| set_source.set(event_target_value(&ev))
                        disabled=move || submitting.get()
                    />
                </div>

                <div class="form-row">
                    <label for="description">"Description"</label>
                    <textarea
                        id="description"
                        prop:value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        rows=4
                        disabled=move || submitting.get()
                    />
                </div>

                <div class="form-row">
                    <label for="relations">"Relations (comma-separated IDs)"</label>
                    <input
                        id="relations"
                        type="text"
                        prop:value=move || relations_str.get()
                        on:input=move |ev| set_relations_str.set(event_target_value(&ev))
                        disabled=move || submitting.get()
                        placeholder="e.g. 1, 5, 12"
                    />
                </div>

                <div class="form-row">
                    <label>"Flags"</label>
                    <label>
                        <input
                            type="checkbox"
                            prop:checked=move || flag_loop.get()
                            on:change=move |ev| set_flag_loop.set(event_target_checked(&ev))
                        />
                        " Loop"
                    </label>
                    <label>
                        <input
                            type="checkbox"
                            prop:checked=move || flag_sound.get()
                            on:change=move |ev| set_flag_sound.set(event_target_checked(&ev))
                        />
                        " Sound"
                    </label>
                </div>

                <Expander name="post-edit-files".to_string() title="Replace Files".to_string()>
                    <div class="form-row">
                        <label>"Replace content (optional)"</label>
                        <FileDropper on_file=on_content_file />
                        {move || content_token.get().map(|_| view! { <p class="success">"New content uploaded."</p> })}
                    </div>
                    <div class="form-row">
                        <label>"Replace thumbnail (optional)"</label>
                        <FileDropper on_file=on_thumb_file accept="image/*" />
                        {move || thumbnail_token.get().map(|_| view! { <p class="success">"New thumbnail uploaded."</p> })}
                    </div>
                </Expander>

                {move || {
                    let notes = notes_signal.get();
                    (!notes.is_empty()).then(|| view! {
                        <div class="form-row">
                            <label>{format!("Notes ({})", notes.len())}</label>
                            <ul class="notes-list-readonly">
                                {notes.iter().map(|note| {
                                    let html = crate::components::markdown::render_markdown(&note.text);
                                    view! { <li inner_html=html /> }
                                }).collect_view()}
                            </ul>
                        </div>
                    })
                }}

                <div class="form-row buttons">
                    <button type="submit" disabled=move || submitting.get() || uploading.get()>
                        {move || if submitting.get() { "Saving..." } else { "Save changes" }}
                    </button>
                    {can_feature.then(|| view! {
                        <button type="button" on:click=on_feature>"Feature this post"</button>
                    })}
                    {can_delete.then(|| view! {
                        <button type="button" class="btn-danger" on:click=on_delete>"Delete post"</button>
                    })}
                    <a href=move || format!("/post/{}", post_id())>"Back to post"</a>
                </div>
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
