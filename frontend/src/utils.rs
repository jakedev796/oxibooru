use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use wasm_bindgen::prelude::*;

/// Parsed list page query parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct ListQueryParams {
    pub query: String,
    pub offset: i64,
    pub limit: i64,
}

/// Reactively read list query params from the current URL.
pub fn use_list_query_params(default_limit: i64) -> Memo<ListQueryParams> {
    let query_map = use_query_map();
    Memo::new(move |_| {
        let map = query_map.get();
        ListQueryParams {
            query: map.get("query").unwrap_or_default(),
            offset: map.get("offset").and_then(|s| s.parse().ok()).unwrap_or(0),
            limit: map
                .get("limit")
                .and_then(|s| s.parse().ok())
                .unwrap_or(default_limit),
        }
    })
}

/// Build a URL path with query params for a list page.
pub fn build_list_url(base: &str, query: &str, offset: i64, limit: i64) -> String {
    let mut parts = vec![];
    if !query.is_empty() {
        parts.push(format!("query={}", url_encode(query)));
    }
    if offset > 0 {
        parts.push(format!("offset={offset}"));
    }
    parts.push(format!("limit={limit}"));
    if parts.is_empty() {
        base.to_string()
    } else {
        format!("{base}?{}", parts.join("&"))
    }
}

/// Set up a scroll event listener that fires `on_threshold` when the user
/// scrolls near the bottom of the page (within 300px). Automatically
/// guards against duplicate calls via `loading_more` and `has_more`.
/// Registers cleanup to remove the listener when the component is unmounted.
pub fn setup_scroll_listener(
    loading_more: RwSignal<bool>,
    has_more: Signal<bool>,
    on_threshold: impl Fn() + 'static,
) {
    let handler = Closure::wrap(Box::new(move || {
        if loading_more.get_untracked() || !has_more.get_untracked() {
            return;
        }
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(el) = doc.document_element() {
                    let scroll_top = el.scroll_top();
                    let scroll_height = el.scroll_height();
                    let client_height = el.client_height();
                    if scroll_height - scroll_top - client_height < 300 {
                        on_threshold();
                    }
                }
            }
        }
    }) as Box<dyn FnMut()>);

    let js_fn: js_sys::Function = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
    handler.forget();
    let js_fn_cleanup = js_fn.clone();

    if let Some(win) = web_sys::window() {
        let _ = win.add_event_listener_with_callback("scroll", &js_fn);
    }

    on_cleanup(move || {
        if let Some(win) = web_sys::window() {
            let _ = win.remove_event_listener_with_callback("scroll", &js_fn_cleanup);
        }
    });
}

/// URL-encode a string.
fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}

/// Format a file size in bytes to a human-readable string.
pub fn format_file_size(bytes: i64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}

/// Format an ISO timestamp for display (e.g., "2024-01-15T10:30:45Z" → "2024-01-15").
pub fn format_time_short(timestamp: &str) -> String {
    timestamp
        .split('T')
        .next()
        .unwrap_or(timestamp)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_file_size_bytes() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1023), "1023 B");
    }

    #[test]
    fn format_file_size_kilobytes() {
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(10240), "10.0 KB");
    }

    #[test]
    fn format_file_size_megabytes() {
        assert_eq!(format_file_size(1_048_576), "1.0 MB");
        assert_eq!(format_file_size(5_242_880), "5.0 MB");
    }

    #[test]
    fn format_file_size_gigabytes() {
        assert_eq!(format_file_size(1_073_741_824), "1.0 GB");
        assert_eq!(format_file_size(2_147_483_648), "2.0 GB");
    }

    #[test]
    fn format_time_short_iso_timestamp() {
        assert_eq!(format_time_short("2024-01-15T10:30:45Z"), "2024-01-15");
    }

    #[test]
    fn format_time_short_date_only() {
        assert_eq!(format_time_short("2024-01-15"), "2024-01-15");
    }

    #[test]
    fn format_time_short_empty() {
        assert_eq!(format_time_short(""), "");
    }

    #[test]
    fn format_time_short_no_t_separator() {
        assert_eq!(format_time_short("just-a-string"), "just-a-string");
    }
}
