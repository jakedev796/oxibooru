use leptos::prelude::*;

/// A search bar with text input and submit button.
#[component]
pub fn SearchBar(
    /// Current query string.
    #[prop(into)]
    query: Signal<String>,
    /// Called when user submits a new query.
    #[prop(into)]
    on_submit: Callback<String>,
) -> impl IntoView {
    let (input, set_input) = signal(String::new());

    // Sync input with query prop when it changes
    Effect::new(move || {
        set_input.set(query.get());
    });

    let on_form_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        on_submit.run(input.get());
    };

    view! {
        <form class="search-bar" on:submit=on_form_submit>
            <input
                type="text"
                id="search-input"
                placeholder="Search…"
                prop:value=move || input.get()
                on:input=move |ev| set_input.set(event_target_value(&ev))
            />
            <button type="submit">"Search"</button>
        </form>
    }
}
