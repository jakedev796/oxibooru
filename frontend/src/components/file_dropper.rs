use leptos::prelude::*;

/// Drag-and-drop zone + click-to-browse + optional URL input for file upload.
///
/// Two modes:
/// - **Single file** (default): provide `on_file` callback. Shows file name when selected.
/// - **Multi file**: provide `on_files` callback. Fires with all selected files at once.
///
/// URL support: provide `on_url` callback to show an always-visible URL input row below the drop zone.
#[component]
pub fn FileDropper(
    /// Called when a single file is selected (single-file mode).
    #[prop(optional)]
    on_file: Option<Callback<web_sys::File>>,
    /// Called when one or more files are selected (multi-file mode).
    #[prop(optional)]
    on_files: Option<Callback<Vec<web_sys::File>>>,
    /// Called when a URL is submitted.
    #[prop(optional)]
    on_url: Option<Callback<String>>,
    /// Text shown inside the drop zone.
    #[prop(optional, into)]
    label: String,
    /// Subtext shown below the main label (e.g. allowed extensions).
    #[prop(optional, into)]
    sublabel: String,
    /// HTML accept attribute for the file input.
    #[prop(optional, into)]
    accept: String,
) -> impl IntoView {
    let (dragging, set_dragging) = signal(false);
    let (file_name, set_file_name) = signal(Option::<String>::None);
    let (url_value, set_url_value) = signal(String::new());

    let multi = on_files.is_some();
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let emit_files = move |file_list: web_sys::FileList| {
        let mut files = Vec::new();
        for i in 0..file_list.length() {
            if let Some(f) = file_list.get(i) {
                files.push(f);
            }
        }
        if files.is_empty() {
            return;
        }
        if let Some(on_files) = on_files {
            on_files.run(files);
        } else if let Some(on_file) = on_file {
            // Single-file mode: take first file
            if let Some(f) = files.into_iter().next() {
                set_file_name.set(Some(f.name()));
                on_file.run(f);
            }
        }
    };

    let on_click = move |_| {
        if let Some(input) = input_ref.get() {
            input.click();
        }
    };

    let on_file_change = move |_| {
        if let Some(input) = input_ref.get() {
            let input_el: &web_sys::HtmlInputElement = &input;
            if let Some(files) = input_el.files() {
                emit_files(files);
            }
            // Reset input so the same file(s) can be selected again
            input_el.set_value("");
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
                emit_files(files);
            }
        }
    };

    let on_paste = move |ev: web_sys::ClipboardEvent| {
        if let Some(dt) = ev.clipboard_data() {
            if let Some(files) = dt.files() {
                if files.length() > 0 {
                    ev.prevent_default();
                    emit_files(files);
                    return;
                }
            }
            if let Some(on_url) = on_url {
                if let Ok(text) = dt.get_data("text/plain") {
                    let trimmed = text.trim().to_string();
                    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                        ev.prevent_default();
                        on_url.run(trimmed);
                    }
                }
            }
        }
    };

    let submit_url = move || {
        let val = url_value.get_untracked();
        if !val.trim().is_empty() {
            if let Some(on_url) = on_url {
                on_url.run(val.trim().to_string());
                set_url_value.set(String::new());
            }
        }
    };

    let display_label = StoredValue::new(if label.is_empty() {
        "Drop files here!\nOr just click on this box.".to_string()
    } else {
        label
    });
    let display_sublabel = StoredValue::new(sublabel);
    let has_url_support = on_url.is_some();
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
                multiple=multi
                style="display: none"
            />
            <div class="drop-area" on:click=on_click>
                {move || {
                    if !multi {
                        // Single-file mode: show file name when selected
                        if let Some(name) = file_name.get() {
                            return view! { <p class="file-name">{name}</p> }.into_any();
                        }
                    }
                    let label_text = display_label.get_value();
                    let sub_text = display_sublabel.get_value();
                    view! {
                        <div class="drop-text">
                            {label_text.split('\n').map(|line| {
                                let l = line.to_string();
                                view! { <p class="drop-label">{l}</p> }
                            }).collect_view()}
                            {(!sub_text.is_empty()).then(|| {
                                view! { <p class="drop-sublabel">{sub_text}</p> }
                            })}
                        </div>
                    }.into_any()
                }}
            </div>
            {has_url_support.then(|| view! {
                <div class="url-input-row">
                    <input
                        type="text"
                        placeholder="Alternatively, paste a URL here."
                        prop:value=move || url_value.get()
                        on:input=move |ev| set_url_value.set(event_target_value(&ev))
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                ev.prevent_default();
                                submit_url();
                            }
                        }
                    />
                    <button type="button" class="url-add-btn" on:click=move |_| submit_url()>
                        "Add URL"
                    </button>
                </div>
            })}
        </div>
    }
}
