use leptos::prelude::*;

#[component]
pub fn FavoriteWidget(favorited: RwSignal<bool>, count: RwSignal<i64>, on_toggle: Callback<bool>) -> impl IntoView {
    view! {
        <button
            class="favorite-widget"
            class:active=move || favorited.get()
            on:click=move |_| {
                let was_fav = favorited.get_untracked();
                favorited.set(!was_fav);
                count.update(|c| *c += if was_fav { -1 } else { 1 });
                on_toggle.run(!was_fav);
            }
        >
            <span class="heart">{move || if favorited.get() { "\u{2665}" } else { "\u{2661}" }}</span>
            " "
            <span class="fav-count">{move || count.get()}</span>
        </button>
    }
}
