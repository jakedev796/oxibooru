use std::cell::RefCell;
use std::rc::Rc;

use gloo_timers::callback::Timeout;
use leptos::prelude::*;
use oxibooru_shared::tag::TagInfo;
use crate::api::ApiClient;
use crate::settings::SettingsState;

/// A search bar with text input, submit button, and optional tag autocomplete.
#[component]
pub fn SearchBar(
    /// Current query string.
    #[prop(into)]
    query: Signal<String>,
    /// Called when user submits a new query.
    #[prop(into)]
    on_submit: Callback<String>,
    /// Enable tag autocomplete on the search input.
    #[prop(default = false)]
    tag_autocomplete: bool,
    /// Placeholder text for the input.
    #[prop(default = "Search\u{2026}")]
    placeholder: &'static str,
    /// Extra CSS class for the form element.
    #[prop(optional, into)]
    form_class: Option<String>,
) -> impl IntoView {
    let settings = expect_context::<SettingsState>();
    let (input, set_input) = signal(String::new());

    // Sync input with query prop when it changes
    Effect::new(move || {
        set_input.set(query.get());
    });

    let on_form_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        on_submit.run(input.get());
    };

    // Autocomplete state
    let (suggestions, set_suggestions) = signal(Vec::<TagInfo>::new());
    let (show_dropdown, set_show_dropdown) = signal(false);
    let (selected_index, set_selected_index) = signal(Option::<usize>::None);
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let debounce_handle: Rc<RefCell<Option<Timeout>>> = Rc::new(RefCell::new(None));

    // Extract the word at the cursor position
    let get_current_word = move || -> String {
        let Some(el) = input_ref.get() else {
            return String::new();
        };
        let value = el.value();
        let cursor = el.selection_start().ok().flatten().unwrap_or(0) as usize;
        let before_cursor = &value[..cursor.min(value.len())];
        let word = before_cursor.rsplit_once(' ').map(|(_, w)| w).unwrap_or(before_cursor);
        word.to_string()
    };

    // Replace the word at the cursor position with the selected tag
    let replace_current_word = move |replacement: &str| {
        let Some(el) = input_ref.get() else { return };
        let value = el.value();
        let cursor = el.selection_start().ok().flatten().unwrap_or(0) as usize;
        let cursor = cursor.min(value.len());
        let before_cursor = &value[..cursor];
        let after_cursor = &value[cursor..];

        // Find word boundaries
        let word_start = before_cursor.rfind(' ').map(|i| i + 1).unwrap_or(0);
        let word_end_in_after = after_cursor.find(' ').unwrap_or(after_cursor.len());

        let prefix = &value[..word_start];
        let suffix = &value[cursor + word_end_in_after..];

        let new_value = format!("{prefix}{replacement} {}", suffix.trim_start());
        let new_cursor = prefix.len() + replacement.len() + 1;
        set_input.set(new_value.clone());
        el.set_value(&new_value);
        let _ = el.set_selection_range(new_cursor as u32, new_cursor as u32);
        let _ = el.focus();
    };

    let api = expect_context::<RwSignal<ApiClient>>();
    let dh = debounce_handle.clone();

    let on_input_handler = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        set_input.set(val);

        if !tag_autocomplete {
            return;
        }

        // Cancel previous debounce
        dh.borrow_mut().take();
        set_selected_index.set(None);

        let word = get_current_word();
        if word.is_empty() {
            set_suggestions.set(vec![]);
            set_show_dropdown.set(false);
            return;
        }

        let dh2 = dh.clone();
        let timeout = Timeout::new(200, move || {
            dh2.borrow_mut().take();
            let client = api.get_untracked();
            // 3+ chars = *word*, otherwise word*
            let search_term = if word.len() >= 3 {
                format!("*{word}* sort:usages")
            } else {
                format!("{word}* sort:usages")
            };
            leptos::task::spawn_local(async move {
                if let Ok(page) = client.get_tags(&search_term, 0, 15, "names,category,usages").await {
                    set_suggestions.set(page.results);
                    set_show_dropdown.set(true);
                }
            });
        });
        *dh.borrow_mut() = Some(timeout);
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if !tag_autocomplete || !show_dropdown.get_untracked() {
            return;
        }
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let len = suggestions.get_untracked().len();
                if len > 0 {
                    set_selected_index.update(|idx| {
                        *idx = Some(match *idx {
                            Some(i) => (i + 1).min(len - 1),
                            None => 0,
                        });
                    });
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|idx| {
                    *idx = match *idx {
                        Some(0) | None => None,
                        Some(i) => Some(i - 1),
                    };
                });
            }
            "Tab" | "Enter" => {
                if let Some(i) = selected_index.get_untracked() {
                    let items = suggestions.get_untracked();
                    if let Some(tag) = items.get(i) {
                        if let Some(name) = tag.names.as_ref().and_then(|n| n.first()) {
                            ev.prevent_default();
                            replace_current_word(name);
                            set_suggestions.set(vec![]);
                            set_show_dropdown.set(false);
                        }
                    }
                }
            }
            "Escape" => {
                set_show_dropdown.set(false);
            }
            _ => {}
        }
    };

    let on_blur = move |_| {
        Timeout::new(200, move || {
            set_show_dropdown.set(false);
        })
        .forget();
    };

    let combined_class = format!("search-bar{}", form_class.map(|c| format!(" {c}")).unwrap_or_default());

    view! {
        <form class=combined_class on:submit=on_form_submit>
            <div class="search-input-wrapper">
                <input
                    type="text"
                    name="search-text"
                    placeholder=placeholder
                    prop:value=move || input.get()
                    on:input=on_input_handler
                    on:keydown=on_keydown
                    on:blur=on_blur
                    node_ref=input_ref
                    autocomplete="off"
                />
                {tag_autocomplete.then(|| view! {
                    <Show when=move || show_dropdown.get() && !suggestions.get().is_empty()>
                        <div class="autocomplete-dropdown">
                            {move || suggestions.get().into_iter().enumerate().map(|(i, tag)| {
                                let name = tag.names.as_ref()
                                    .and_then(|n| n.first())
                                    .cloned()
                                    .unwrap_or_default();
                                let category = tag.category.clone().unwrap_or_default();
                                let usages = tag.usages.unwrap_or(0);
                                let is_highlighted = move || selected_index.get() == Some(i);
                                let name_for_click = name.clone();
                                let display = settings.display_name(&name);
                                let class = format!("autocomplete-item tag-category-{category}");
                                view! {
                                    <div
                                        class=class
                                        class:highlighted=is_highlighted
                                        on:mousedown=move |ev| {
                                            ev.prevent_default();
                                            replace_current_word(&name_for_click);
                                            set_suggestions.set(vec![]);
                                            set_show_dropdown.set(false);
                                        }
                                    >
                                        <span class="tag-name">{display}</span>
                                        <span class="tag-usages">{usages}</span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </Show>
                })}
            </div>
            <input type="submit" value="Search" />
        </form>
    }
}
