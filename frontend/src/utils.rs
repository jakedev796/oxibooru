use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

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
