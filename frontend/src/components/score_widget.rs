use leptos::prelude::*;
use oxibooru_shared::enums::Rating;

#[component]
pub fn ScoreWidget(
    score: RwSignal<i64>,
    own_score: RwSignal<Rating>,
    on_vote: Callback<Rating>,
) -> impl IntoView {
    view! {
        <div class="score-widget">
            <button
                class="vote-down"
                class:active=move || own_score.get() == Rating::Dislike
                on:click=move |_| {
                    let current = own_score.get_untracked();
                    let effective = if current == Rating::Dislike { Rating::None } else { Rating::Dislike };
                    let delta = (effective as i64) - (current as i64);
                    score.update(|s| *s += delta);
                    own_score.set(effective);
                    on_vote.run(effective);
                }
            >
                "\u{25BC}"
            </button>
            <span class="score-value">{move || score.get()}</span>
            <button
                class="vote-up"
                class:active=move || own_score.get() == Rating::Like
                on:click=move |_| {
                    let current = own_score.get_untracked();
                    let effective = if current == Rating::Like { Rating::None } else { Rating::Like };
                    let delta = (effective as i64) - (current as i64);
                    score.update(|s| *s += delta);
                    own_score.set(effective);
                    on_vote.run(effective);
                }
            >
                "\u{25B2}"
            </button>
        </div>
    }
}
