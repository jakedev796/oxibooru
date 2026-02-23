use leptos::prelude::*;
use leptos_meta::Title;

use crate::api::ApiClient;

#[component]
pub fn HomePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();

    let info = LocalResource::new(move || {
        let client = api.get();
        async move { client.get_info().await.ok() }
    });

    view! {
        <Title text="Home" />
        <div class="home-page">
            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                {move || Suspend::new(async move {
                    match info.await {
                        Some(info) => view! {
                            <div class="home-info">
                                <h1>{info.config.name.clone()}</h1>
                                <dl>
                                    <dt>"Posts"</dt>
                                    <dd>{info.post_count}</dd>
                                    <dt>"Disk usage"</dt>
                                    <dd>{format_disk_usage(info.disk_usage)}</dd>
                                    <dt>"Server time"</dt>
                                    <dd>{info.server_time.clone()}</dd>
                                </dl>
                                {info.featured_post.as_ref().map(|post| {
                                    view! {
                                        <div class="featured-post">
                                            <h2>"Featured post"</h2>
                                            <a href=format!("/post/{}", post.id.unwrap_or(0))>
                                                <img
                                                    src=post.thumbnail_url.clone().unwrap_or_default()
                                                    alt="Featured post"
                                                />
                                            </a>
                                        </div>
                                    }
                                })}
                            </div>
                        }.into_any(),
                        None => view! {
                            <p class="error">"Failed to load server info."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

fn format_disk_usage(bytes: i64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let bytes = bytes as f64;
    if bytes >= TB {
        format!("{:.1} TB", bytes / TB)
    } else if bytes >= GB {
        format!("{:.1} GB", bytes / GB)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes / KB)
    } else {
        format!("{bytes} B")
    }
}
