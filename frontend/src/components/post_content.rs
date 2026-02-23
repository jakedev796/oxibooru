use leptos::prelude::*;
use oxibooru_shared::enums::PostType;

/// Displays post content (image, video, or animation) with CSS fit modes.
#[component]
pub fn PostContent(
    #[prop(into)] content_url: String,
    post_type: PostType,
    #[prop(into)] fit_mode: String,
    #[prop(default = vec![])] flags: Vec<String>,
) -> impl IntoView {
    let has_loop = flags.iter().any(|f| f == "loop");
    let container_class = format!("post-content {fit_mode}");

    match post_type {
        PostType::Image => view! {
            <div class=container_class>
                <img src=content_url alt="Post content" />
            </div>
        }
        .into_any(),
        PostType::Animation => view! {
            <div class=container_class>
                <img src=content_url alt="Post content" />
            </div>
        }
        .into_any(),
        PostType::Video => view! {
            <div class=container_class>
                <video
                    src=content_url
                    controls=true
                    loop=has_loop
                    autoplay=false
                    preload="metadata"
                />
            </div>
        }
        .into_any(),
        PostType::Flash => view! {
            <div class=container_class>
                <p class="flash-notice">"Flash content is not supported."</p>
            </div>
        }
        .into_any(),
    }
}
