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
            limit: map.get("limit").and_then(|s| s.parse().ok()).unwrap_or(default_limit),
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
///
/// Also listens on `resize` and checks after each load completes,
/// so pages that don't fill the viewport will keep loading until they do.
pub fn setup_scroll_listener(loading_more: RwSignal<bool>, has_more: Signal<bool>, on_threshold: impl Fn() + 'static) {
    let on_threshold = std::rc::Rc::new(on_threshold);
    let on_threshold_check = on_threshold.clone();

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
    let js_fn_scroll = js_fn.clone();
    let js_fn_resize = js_fn.clone();
    let js_fn_cleanup_scroll = js_fn.clone();
    let js_fn_cleanup_resize = js_fn;

    if let Some(win) = web_sys::window() {
        let _ = win.add_event_listener_with_callback("scroll", &js_fn_scroll);
        let _ = win.add_event_listener_with_callback("resize", &js_fn_resize);
    }

    // Re-check viewport fill whenever loading finishes OR when has_more becomes true
    Effect::new(move || {
        let is_loading = loading_more.get();
        let more = has_more.get();
        if !is_loading && more {
            let check = on_threshold_check.clone();
            // Defer to next microtask so DOM has updated with new content
            leptos::task::spawn_local(async move {
                if let Some(win) = web_sys::window() {
                    if let Some(doc) = win.document() {
                        if let Some(el) = doc.document_element() {
                            let scroll_height = el.scroll_height();
                            let client_height = el.client_height();
                            if scroll_height <= client_height + 300 {
                                check();
                            }
                        }
                    }
                }
            });
        }
    });

    on_cleanup(move || {
        if let Some(win) = web_sys::window() {
            let _ = win.remove_event_listener_with_callback("scroll", &js_fn_cleanup_scroll);
            let _ = win.remove_event_listener_with_callback("resize", &js_fn_cleanup_resize);
        }
    });
}

/// URL-encode a string.
fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}

/// Format a file size in bytes to a human-readable string.
pub fn format_file_size(bytes: i64) -> String {
    let units = ["B", "K", "M", "G"];
    let mut value = bytes as f64;
    let mut unit_idx = 0;
    while value >= 1024.0 && unit_idx < units.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }
    let suffix = units[unit_idx];
    if suffix == "B" || value >= 20.0 {
        format!("{:.0}{suffix}", value)
    } else {
        format!("{:.1}{suffix}", value)
    }
}

/// Format an ISO timestamp for display
pub fn format_time_short(timestamp: &str) -> String {
    timestamp.split('T').next().unwrap_or(timestamp).to_string()
}

/// Format an ISO timestamp as a relative time string
pub fn format_relative_time(timestamp: &str) -> String {
    let now_ms = js_sys::Date::now();
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(timestamp));
    let then_ms = date.get_time();
    if then_ms.is_nan() {
        return timestamp.to_string();
    }
    let diff_secs = ((now_ms - then_ms) / 1000.0) as i64;
    if diff_secs < 0 {
        return "just now".to_string();
    }
    let (value, unit) = if diff_secs < 60 {
        (diff_secs, "second")
    } else if diff_secs < 3600 {
        (diff_secs / 60, "minute")
    } else if diff_secs < 86400 {
        (diff_secs / 3600, "hour")
    } else if diff_secs < 2_592_000 {
        (diff_secs / 86400, "day")
    } else if diff_secs < 31_536_000 {
        (diff_secs / 2_592_000, "month")
    } else {
        (diff_secs / 31_536_000, "year")
    };
    if value == 1 {
        format!("{value} {unit} ago")
    } else {
        format!("{value} {unit}s ago")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_file_size_bytes() {
        assert_eq!(format_file_size(0), "0B");
        assert_eq!(format_file_size(512), "512B");
        assert_eq!(format_file_size(1023), "1023B");
    }

    #[test]
    fn format_file_size_kilobytes() {
        assert_eq!(format_file_size(1024), "1.0K");
        assert_eq!(format_file_size(1536), "1.5K");
        assert_eq!(format_file_size(10240), "10.0K");
    }

    #[test]
    fn format_file_size_megabytes() {
        assert_eq!(format_file_size(1_048_576), "1.0M");
        assert_eq!(format_file_size(5_242_880), "5.0M");
    }

    #[test]
    fn format_file_size_gigabytes() {
        assert_eq!(format_file_size(1_073_741_824), "1.0G");
        assert_eq!(format_file_size(324_281_999_360_i64), "302G");
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
