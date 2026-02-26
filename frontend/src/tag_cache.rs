use leptos::prelude::*;
use oxibooru_shared::tag::TagInfo;

/// Simple in-memory cache for the last fetched tag, shared across tag view tabs.
/// Prevents re-fetching when navigating between Summary/Edit/Merge/Delete.
#[derive(Copy, Clone)]
pub struct TagCache {
    name: RwSignal<String>,
    tag: RwSignal<Option<TagInfo>>,
}

impl TagCache {
    pub fn new() -> Self {
        Self {
            name: RwSignal::new(String::new()),
            tag: RwSignal::new(None),
        }
    }

    /// Get cached tag if it matches the given name (non-tracking read).
    pub fn get(&self, name: &str) -> Option<TagInfo> {
        if !name.is_empty() && self.name.get_untracked() == name {
            self.tag.get_untracked()
        } else {
            None
        }
    }

    /// Store a tag in the cache.
    pub fn set(&self, name: &str, tag: TagInfo) {
        self.name.set(name.to_string());
        self.tag.set(Some(tag));
    }
}
