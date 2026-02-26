use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "oxibooru-settings";

/// Client-side user preferences, persisted in localStorage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub posts_per_page: u32,
    pub tag_suggestions_enabled: bool,
    pub dark_theme: bool,
    pub list_posts_safe: bool,
    pub list_posts_sketchy: bool,
    pub list_posts_unsafe: bool,
    pub upscale_small_posts: bool,
    pub endless_scroll: bool,
    pub tag_underscores: bool,
    pub keyboard_shortcuts: bool,
    pub fit_mode: String,
    pub post_flow: bool,
    pub transparency_grid: bool,
    pub autoplay_videos: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            posts_per_page: 42,
            tag_suggestions_enabled: true,
            dark_theme: false,
            list_posts_safe: true,
            list_posts_sketchy: true,
            list_posts_unsafe: false,
            upscale_small_posts: false,
            endless_scroll: false,
            tag_underscores: false,
            keyboard_shortcuts: true,
            fit_mode: "fit-both".to_string(),
            post_flow: false,
            transparency_grid: true,
            autoplay_videos: false,
        }
    }
}

impl Settings {
    /// Load settings from localStorage, falling back to defaults.
    pub fn load() -> Self {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    /// Save settings to localStorage.
    pub fn save(&self) {
        let _ = LocalStorage::set(STORAGE_KEY, self);
    }
}

/// Settings state provided as Leptos context.
#[derive(Copy, Clone)]
pub struct SettingsState {
    pub inner: RwSignal<Settings>,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            inner: RwSignal::new(Settings::load()),
        }
    }

    /// Update a setting and persist to localStorage.
    pub fn update(&self, f: impl FnOnce(&mut Settings)) {
        self.inner.update(|s| {
            f(s);
            s.save();
        });
    }

    /// Format a name for display, replacing underscores with spaces if the setting is enabled.
    pub fn display_name(&self, name: &str) -> String {
        if self.inner.with(|s| s.tag_underscores) {
            name.replace('_', " ")
        } else {
            name.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_values() {
        let s = Settings::default();
        assert_eq!(s.posts_per_page, 42);
        assert!(s.tag_suggestions_enabled);
        assert!(!s.dark_theme);
        assert!(s.list_posts_safe);
        assert!(s.list_posts_sketchy);
        assert!(!s.list_posts_unsafe);
        assert!(!s.upscale_small_posts);
        assert!(!s.endless_scroll);
        assert!(!s.tag_underscores);
        assert!(s.keyboard_shortcuts);
        assert_eq!(s.fit_mode, "fit-both");
        assert!(!s.post_flow);
        assert!(s.transparency_grid);
        assert!(!s.autoplay_videos);
    }

    #[test]
    fn settings_serialize_roundtrip() {
        let original = Settings {
            posts_per_page: 100,
            dark_theme: true,
            fit_mode: "fit-width".to_string(),
            ..Settings::default()
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.posts_per_page, 100);
        assert!(restored.dark_theme);
        assert_eq!(restored.fit_mode, "fit-width");
        assert!(restored.tag_suggestions_enabled); // unchanged default
    }
}
