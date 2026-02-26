use leptos::prelude::*;
use oxibooru_shared::enums::{PostSafety, PostType};

/// A post thumbnail with safety badge and type indicator.
#[component]
pub fn PostThumbnail(
    id: i64,
    #[prop(into)] thumbnail_url: String,
    safety: PostSafety,
    post_type: PostType,
    score: Option<i64>,
    favorite_count: Option<i64>,
    comment_count: Option<i64>,
) -> impl IntoView {
    let safety_class = match safety {
        PostSafety::Safe => "safety-safe",
        PostSafety::Sketchy => "safety-sketchy",
        PostSafety::Unsafe => "safety-unsafe",
    };

    let type_badge_class = match post_type {
        PostType::Video | PostType::Animation => Some("type-badge type-video"),
        PostType::Flash => Some("type-badge type-flash"),
        PostType::Image => None,
    };

    let bg_style = format!("background-image: url('{thumbnail_url}')");

    view! {
        <article class="post-thumbnail">
            <a href=format!("/post/{id}") style=bg_style>
                <img src=thumbnail_url.clone() loading="lazy" alt=format!("Post {id}") />
                <span class=format!("safety-badge {safety_class}") />
                {type_badge_class.map(|cls| view! {
                    <span class=cls />
                })}
            </a>
            <div class="post-stats">
                {score.map(|s| view! { <span class="stat" title="Score">{s}</span> })}
                {favorite_count.map(|f| view! { <span class="stat" title="Favorites">{f}</span> })}
                {comment_count.map(|c| view! { <span class="stat" title="Comments">{c}</span> })}
            </div>
        </article>
    }
}
