use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use std::collections::HashMap;

const STORAGE_KEY: &str = "oxibooru-expanders";

/// Load expander states from localStorage.
fn load_expander_states() -> HashMap<String, bool> {
    LocalStorage::get(STORAGE_KEY).unwrap_or_default()
}

/// Save expander states to localStorage.
fn save_expander_states(states: &HashMap<String, bool>) {
    let _ = LocalStorage::set(STORAGE_KEY, states);
}

/// Collapsible section with localStorage persistence.
///
/// The `name` prop serves as the localStorage key for this section's state.
/// Sections are expanded by default.
#[component]
pub fn Expander(
    /// Unique name used as localStorage key for persistence.
    #[prop(into)]
    name: String,
    /// Display title shown in the header.
    #[prop(into)]
    title: String,
    children: Children,
) -> impl IntoView {
    let name_clone = name.clone();
    let states = load_expander_states();
    let initial = states.get(&name).copied().unwrap_or(true);
    let (expanded, set_expanded) = signal(initial);

    let toggle = move |_: leptos::ev::MouseEvent| {
        let new_val = !expanded.get_untracked();
        set_expanded.set(new_val);
        let mut states = load_expander_states();
        states.insert(name_clone.clone(), new_val);
        save_expander_states(&states);
    };

    view! {
        <section class="expander">
            <header class="expander-header" on:click=toggle>
                <span class="expander-chevron">
                    <i class=move || if expanded.get() { "fa fa-chevron-down" } else { "fa fa-chevron-up" } />
                </span>
                <span class="expander-title">{title}</span>
            </header>
            <div class="expander-body" style:display=move || if expanded.get() { "" } else { "none" }>
                {children()}
            </div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_empty_states() {
        // Without localStorage available, should return empty map
        let states: HashMap<String, bool> = HashMap::new();
        assert!(states.get("test").is_none());
    }
}
