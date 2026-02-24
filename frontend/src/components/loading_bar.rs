use leptos::prelude::*;

/// Global loading state tracked by a reference count.
/// Pages call `start()` before fetching and `finish()` when done.
#[derive(Copy, Clone)]
pub struct LoadingState {
    count: RwSignal<usize>,
    finishing: RwSignal<bool>,
}

impl LoadingState {
    pub fn new() -> Self {
        Self {
            count: RwSignal::new(0),
            finishing: RwSignal::new(false),
        }
    }

    pub fn start(&self) {
        self.finishing.set(false);
        self.count.update(|c| *c += 1);
    }

    pub fn finish(&self) {
        self.count.update(|c| *c = c.saturating_sub(1));
    }

    pub fn is_loading(&self) -> bool {
        self.count.get() > 0
    }

    pub fn is_finishing(&self) -> bool {
        self.finishing.get()
    }
}

#[component]
pub fn LoadingBar() -> impl IntoView {
    let state = expect_context::<LoadingState>();

    // Watch for transitions from loading to not-loading
    let prev_loading = RwSignal::new(false);
    Effect::new(move || {
        let loading = state.is_loading();
        let was_loading = prev_loading.get_untracked();
        if was_loading && !loading {
            // Transition: loading -> done. Show finishing animation.
            state.finishing.set(true);
            // Clear finishing state after animation completes
            gloo_timers::callback::Timeout::new(500, move || {
                state.finishing.set(false);
            })
            .forget();
        }
        prev_loading.set(loading);
    });

    view! {
        <div
            class="loading-bar"
            class:active=move || state.is_loading()
            class:finishing=move || state.is_finishing()
        />
    }
}
