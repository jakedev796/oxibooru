use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use oxibooru_shared::enums::PostSafety;

use crate::api::posts::CreatePostBody;
use crate::api::ApiClient;
use crate::components::file_dropper::FileDropper;
use crate::components::tag_input::TagInput;

#[component]
pub fn PostUploadPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let navigate = use_navigate();

    // Form signals
    let (safety, set_safety) = signal("safe".to_string());
    let tags = RwSignal::new(Vec::<String>::new());
    let (source, set_source) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (anonymous, set_anonymous) = signal(false);
    let (content_token, set_content_token) = signal(Option::<String>::None);
    let (content_url, set_content_url) = signal(Option::<String>::None);
    let (thumbnail_token, set_thumbnail_token) = signal(Option::<String>::None);

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (uploading_content, set_uploading_content) = signal(false);
    let (uploading_thumb, set_uploading_thumb) = signal(false);

    // Upload content file → get token
    let on_content_file = Callback::new(move |file: web_sys::File| {
        let client = api.get_untracked();
        set_uploading_content.set(true);
        set_error_msg.set(None);
        set_content_url.set(None);
        leptos::task::spawn_local(async move {
            let form = web_sys::FormData::new().unwrap();
            let _ = form.append_with_blob("content", &file);
            match client.upload_file(&form).await {
                Ok(resp) => set_content_token.set(Some(resp.token)),
                Err(e) => set_error_msg.set(Some(format!("Upload failed: {e}"))),
            }
            set_uploading_content.set(false);
        });
    });

    // Content from URL (no upload needed)
    let on_content_url = Callback::new(move |url: String| {
        set_content_token.set(None);
        set_content_url.set(Some(url));
    });

    // Upload thumbnail file → get token
    let on_thumb_file = Callback::new(move |file: web_sys::File| {
        let client = api.get_untracked();
        set_uploading_thumb.set(true);
        set_error_msg.set(None);
        leptos::task::spawn_local(async move {
            let form = web_sys::FormData::new().unwrap();
            let _ = form.append_with_blob("content", &file);
            match client.upload_file(&form).await {
                Ok(resp) => set_thumbnail_token.set(Some(resp.token)),
                Err(e) => set_error_msg.set(Some(format!("Thumbnail upload failed: {e}"))),
            }
            set_uploading_thumb.set(false);
        });
    });

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let ct = content_token.get_untracked();
        let cu = content_url.get_untracked();
        if ct.is_none() && cu.is_none() {
            set_error_msg.set(Some("Please select a file or enter a URL.".into()));
            return;
        }

        let safety_val = match safety.get_untracked().as_str() {
            "sketchy" => PostSafety::Sketchy,
            "unsafe" => PostSafety::Unsafe,
            _ => PostSafety::Safe,
        };

        let tag_list = tags.get_untracked();
        let src = source.get_untracked();
        let desc = description.get_untracked();
        let anon = anonymous.get_untracked();
        let tt = thumbnail_token.get_untracked();

        let body = CreatePostBody {
            safety: safety_val,
            content_token: ct,
            content_url: cu,
            thumbnail_token: tt,
            source: if src.is_empty() { None } else { Some(src) },
            description: if desc.is_empty() { None } else { Some(desc) },
            relations: None,
            anonymous: if anon { Some(true) } else { None },
            tags: if tag_list.is_empty() { None } else { Some(tag_list) },
            notes: None,
            flags: None,
        };

        set_submitting.set(true);
        set_error_msg.set(None);
        let client = api.get_untracked();
        let nav = navigate.clone();

        leptos::task::spawn_local(async move {
            let metadata = serde_json::to_string(&body).unwrap_or_default();
            let form = web_sys::FormData::new().unwrap();
            let _ = form.append_with_str("metadata", &metadata);
            match client.create_post(&form).await {
                Ok(post) => {
                    let id = post.id.unwrap_or(0);
                    nav(&format!("/post/{id}"), Default::default());
                }
                Err(e) => {
                    set_submitting.set(false);
                    set_error_msg.set(Some(e.to_string()));
                }
            }
        });
    };

    view! {
        <Title text="Upload" />
        <div class="content-wrapper">
            <h1>"Upload Post"</h1>
            <form class="form-grid" on:submit=on_submit>
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}

                <div class="form-row">
                    <label>"Content"</label>
                    <FileDropper
                        on_file=on_content_file
                        on_url=on_content_url
                        label="Drop content file here or click to browse"
                    />
                    {move || {
                        if uploading_content.get() {
                            Some(view! { <p class="info">"Uploading..."</p> })
                        } else if content_token.get().is_some() {
                            Some(view! { <p class="success">"File uploaded."</p> })
                        } else if content_url.get().is_some() {
                            Some(view! { <p class="success">"URL set."</p> })
                        } else {
                            None
                        }
                    }}
                </div>

                <div class="form-row">
                    <label for="safety">"Safety"</label>
                    <select
                        id="safety"
                        prop:value=move || safety.get()
                        on:change=move |ev| set_safety.set(event_target_value(&ev))
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
                        placeholder="https://..."
                    />
                </div>

                <div class="form-row">
                    <label for="description">"Description"</label>
                    <textarea
                        id="description"
                        prop:value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        rows=4
                        placeholder="Optional description (markdown supported)"
                    />
                </div>

                <div class="form-row">
                    <label>"Custom Thumbnail"</label>
                    <FileDropper
                        on_file=on_thumb_file
                        label="Drop thumbnail here (optional)"
                        accept="image/*"
                    />
                    {move || {
                        if uploading_thumb.get() {
                            Some(view! { <p class="info">"Uploading thumbnail..."</p> })
                        } else if thumbnail_token.get().is_some() {
                            Some(view! { <p class="success">"Thumbnail uploaded."</p> })
                        } else {
                            None
                        }
                    }}
                </div>

                <div class="form-row">
                    <label>
                        <input
                            type="checkbox"
                            prop:checked=move || anonymous.get()
                            on:change=move |ev| set_anonymous.set(event_target_checked(&ev))
                        />
                        " Upload anonymously"
                    </label>
                </div>

                <div class="form-row">
                    <button type="submit" disabled=move || submitting.get() || uploading_content.get()>
                        {move || if submitting.get() { "Creating post..." } else { "Upload" }}
                    </button>
                </div>
            </form>
        </div>
    }
}
