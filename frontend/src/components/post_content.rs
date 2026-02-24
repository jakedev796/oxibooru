use leptos::prelude::*;
use oxibooru_shared::enums::PostType;

/// Displays post content (image, video, or animation) with reactive CSS fit modes.
#[component]
pub fn PostContent(
    #[prop(into)] content_url: String,
    post_type: PostType,
    fit_mode: Signal<String>,
    #[prop(default = false)] upscale: bool,
    #[prop(default = vec![])] flags: Vec<String>,
) -> impl IntoView {
    let has_loop = flags.iter().any(|f| f == "loop");
    let container_class = move || {
        let mode = fit_mode.get();
        if upscale {
            format!("post-content {mode} upscale")
        } else {
            format!("post-content {mode}")
        }
    };

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
