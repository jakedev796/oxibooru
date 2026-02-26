use leptos::prelude::*;
use oxibooru_shared::enums::Rating;

#[component]
pub fn ScoreWidget(score: RwSignal<i64>, own_score: RwSignal<Rating>, on_vote: Callback<Rating>) -> impl IntoView {
    view! {
        <div class="score-widget" role="group" aria-label="Score">
            <span class="score-widget__cell">
                <button
                    type="button"
                    class="vote-up"
                    class:active=move || own_score.get() == Rating::Like
                    aria-label="Like"
                    on:click=move |_| {
                        let current = own_score.get_untracked();
                        let effective = if current == Rating::Like { Rating::None } else { Rating::Like };
                        let delta = (effective as i64) - (current as i64);
                        score.update(|s| *s += delta);
                        own_score.set(effective);
                        on_vote.run(effective);
                    }
                >
                    <i class=move || if own_score.get() == Rating::Like { "fa fa-thumbs-up" } else { "fa fa-thumbs-o-up" } />
                </button>
            </span>
            <span class="score-widget__cell score-widget__value">
                <span class="score-value">{move || score.get()}</span>
            </span>
            <span class="score-widget__cell">
                <button
                    type="button"
                    class="vote-down"
                    class:active=move || own_score.get() == Rating::Dislike
                    aria-label="Dislike"
                    on:click=move |_| {
                        let current = own_score.get_untracked();
                        let effective = if current == Rating::Dislike { Rating::None } else { Rating::Dislike };
                        let delta = (effective as i64) - (current as i64);
                        score.update(|s| *s += delta);
                        own_score.set(effective);
                        on_vote.run(effective);
                    }
                >
                    <i class=move || if own_score.get() == Rating::Dislike { "fa fa-thumbs-down" } else { "fa fa-thumbs-o-down" } />
                </button>
            </span>
        </div>
    }
}
