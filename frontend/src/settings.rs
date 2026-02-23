use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "oxibooru-settings";

/// Client-side user preferences, persisted in localStorage.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fit_mode: String,
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
            fit_mode: "fit-both".to_string(),
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
#[derive(Clone)]
pub struct SettingsState {
    pub inner: RwSignal<Settings>,
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
}
