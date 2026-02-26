use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::api::ApiClient;
use crate::components::api_error::ApiErrorMessage;
use crate::components::loading_bar::LoadingState;
use crate::components::markdown::Markdown;
use crate::settings::SettingsState;
use crate::utils::format_time_short;

#[component]
pub fn PoolViewPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let loading = expect_context::<LoadingState>();
    let settings = expect_context::<SettingsState>();
    let params = use_params_map();

    let pool_id = Memo::new(move |_| params.get().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0));

    let pool = LocalResource::new(move || {
        let client = api.get();
        let id = pool_id.get();
        async move {
            loading.start();
            let result = client.get_pool(id).await;
            loading.finish();
            result
        }
    });

    view! {
        <Title text=move || format!("Pool {}", pool_id.get()) />
        <div class="pool-view-page">
            <Suspense fallback=|| view! { <p>"Loading pool..."</p> }>
                {move || Suspend::new(async move {
                    match pool.await {
                        Ok(pool) => {
                            let names = pool.names.unwrap_or_default();
                            let primary_name = names.first().cloned().unwrap_or_default();
                            let aliases = if names.len() > 1 {
                                names[1..].to_vec()
                            } else {
                                vec![]
                            };
                            let category = pool.category.unwrap_or_default();
                            let post_count = pool.post_count.unwrap_or(0);
                            let created = pool.creation_time.as_deref()
                                .map(format_time_short)
                                .unwrap_or_default();
                            let description = pool.description.unwrap_or_default();
                            let posts = pool.posts.unwrap_or_default();
                            let display_name = settings.display_name(&primary_name);
                            let category_class = format!("pool-category-{category}");

                            view! {
                                <div class="pool-view">
                                    <header class=category_class>
                                        <h1>{display_name}</h1>
                                    </header>

                                    <section class="pool-summary">
                                        <dl>
                                            <dt>"Category"</dt><dd>{category}</dd>
                                            <dt>"Posts"</dt><dd>{post_count}</dd>
                                            <dt>"Created"</dt><dd>{created}</dd>
                                        </dl>

                                        {(!aliases.is_empty()).then(|| view! {
                                            <div class="pool-aliases">
                                                <h3>"Aliases"</h3>
                                                <ul>
                                                    {aliases.into_iter().map(|alias| {
                                                        let display = settings.display_name(&alias);
                                                        view! { <li>{display}</li> }
                                                    }).collect_view()}
                                                </ul>
                                            </div>
                                        })}
                                    </section>

                                    {(!description.is_empty()).then(|| view! {
                                        <section class="pool-description">
                                            <h2>"Description"</h2>
                                            <Markdown text=description />
                                        </section>
                                    })}

                                    {(!posts.is_empty()).then(|| view! {
                                        <section class="pool-posts">
                                            <h2>"Posts"</h2>
                                            <div class="post-grid">
                                                {posts.into_iter().map(|post| {
                                                    let href = format!("/post/{}", post.id);
                                                    view! {
                                                        <a class="pool-post-thumb" href=href>
                                                            <img src=post.thumbnail_url.clone() loading="lazy" alt=format!("Post {}", post.id) />
                                                        </a>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </section>
                                    })}
                                </div>
                            }.into_any()
                        }
                        Err(e) => view! {
                            <ApiErrorMessage error=e />
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
