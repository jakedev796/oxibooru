use leptos::prelude::*;

/// Drag-and-drop zone + click-to-browse + URL paste for file upload.
#[component]
pub fn FileDropper(
    on_file: Callback<web_sys::File>,
    #[prop(optional)] on_url: Option<Callback<String>>,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] accept: String,
) -> impl IntoView {
    let (dragging, set_dragging) = signal(false);
    let (file_name, set_file_name) = signal(Option::<String>::None);
    let (url_mode, set_url_mode) = signal(false);
    let (url_value, set_url_value) = signal(String::new());

    let input_ref = NodeRef::<leptos::html::Input>::new();

    let on_click = move |_| {
        if let Some(input) = input_ref.get() {
            input.click();
        }
    };

    let on_file_change = move |_| {
        if let Some(input) = input_ref.get() {
            let input_el: &web_sys::HtmlInputElement = &input;
            if let Some(files) = input_el.files() {
                if let Some(file) = files.get(0) {
                    set_file_name.set(Some(file.name()));
                    on_file.run(file);
                }
            }
        }
    };

    let on_dragover = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        set_dragging.set(true);
    };

    let on_dragleave = move |_: web_sys::DragEvent| {
        set_dragging.set(false);
    };

    let on_drop = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        set_dragging.set(false);
        if let Some(dt) = ev.data_transfer() {
            if let Some(files) = dt.files() {
                if let Some(file) = files.get(0) {
                    set_file_name.set(Some(file.name()));
                    on_file.run(file);
                }
            }
        }
    };

    let on_paste = move |ev: web_sys::ClipboardEvent| {
        if let Some(dt) = ev.clipboard_data() {
            // Check for pasted files
            if let Some(files) = dt.files() {
                if let Some(file) = files.get(0) {
                    ev.prevent_default();
                    set_file_name.set(Some(file.name()));
                    on_file.run(file);
                    return;
                }
            }
            // Check for pasted URL text
            if let Some(on_url) = on_url {
                if let Ok(text) = dt.get_data("text/plain") {
                    let trimmed = text.trim().to_string();
                    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                        ev.prevent_default();
                        set_file_name.set(Some(trimmed.clone()));
                        on_url.run(trimmed);
                    }
                }
            }
        }
    };

    let has_url_support = on_url.is_some();
    let display_label = StoredValue::new(if label.is_empty() {
        "Drop file here or click to browse".to_string()
    } else {
        label
    });

    let accept_attr = accept.clone();

    view! {
        <div
            class="file-dropper"
            class:dragging=move || dragging.get()
            class:has-file=move || file_name.get().is_some()
            on:dragover=on_dragover
            on:dragleave=on_dragleave
            on:drop=on_drop
            on:paste=on_paste
        >
            <input
                type="file"
                node_ref=input_ref
                on:change=on_file_change
                accept=accept_attr
                style="display: none"
            />
            <Show
                when=move || !url_mode.get()
                fallback=move || {
                    view! {
                        <div class="url-input-area">
                            <input
                                type="text"
                                placeholder="Paste URL here..."
                                prop:value=move || url_value.get()
                                on:input=move |ev| set_url_value.set(event_target_value(&ev))
                                on:keydown=move |ev: web_sys::KeyboardEvent| {
                                    if ev.key() == "Enter" {
                                        ev.prevent_default();
                                        let val = url_value.get_untracked();
                                        if !val.trim().is_empty() {
                                            if let Some(on_url) = on_url {
                                                set_file_name.set(Some(val.clone()));
                                                on_url.run(val);
                                            }
                                        }
                                    }
                                }
                            />
                            <button type="button" on:click=move |_| set_url_mode.set(false)>
                                "Back to file"
                            </button>
                        </div>
                    }
                }
            >
                <div class="drop-area" on:click=on_click>
                    {move || match file_name.get() {
                        Some(name) => view! {
                            <p class="file-name">{name}</p>
                        }.into_any(),
                        None => view! {
                            <p class="drop-label">{display_label.get_value()}</p>
                        }.into_any(),
                    }}
                </div>
                {has_url_support.then(|| view! {
                    <button type="button" class="url-toggle" on:click=move |_| set_url_mode.set(true)>
                        "Or enter URL"
                    </button>
                })}
            </Show>
        </div>
    }
}
