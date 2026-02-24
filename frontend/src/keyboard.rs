use std::collections::HashMap;

use leptos::prelude::*;

/// Centralized keyboard shortcut context.
///
/// Components register handlers for specific keys. The global `keydown` listener
/// in `app.rs` dispatches events to the appropriate handler.
///
/// Key format:
/// - Simple keys: `"f"`, `"q"`, `"ArrowLeft"`, `"ArrowRight"`
/// - Ctrl/Meta combinations: `"ctrl+s"`, `"ctrl+enter"`
#[derive(Copy, Clone)]
pub struct KeyboardShortcuts {
    handlers: RwSignal<HashMap<String, Callback<()>>>,
}

impl Default for KeyboardShortcuts {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardShortcuts {
    pub fn new() -> Self {
        Self {
            handlers: RwSignal::new(HashMap::new()),
        }
    }

    /// Register a keyboard shortcut handler.
    pub fn register(&self, key: &str, handler: Callback<()>) {
        self.handlers.update(|map| {
            map.insert(key.to_string(), handler);
        });
    }

    /// Unregister a keyboard shortcut handler.
    pub fn unregister(&self, key: &str) {
        self.handlers.update(|map| {
            map.remove(key);
        });
    }

    /// Dispatch a key event. Returns `true` if a handler was found and called.
    pub fn dispatch(&self, key: &str) -> bool {
        self.handlers.with_untracked(|map| {
            if let Some(handler) = map.get(key) {
                handler.run(());
                true
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn key_format_examples() {
        // Just verify the key format convention
        let keys = vec!["f", "q", "ArrowLeft", "ArrowRight", "ctrl+s"];
        for key in keys {
            assert!(!key.is_empty());
        }
    }
}
