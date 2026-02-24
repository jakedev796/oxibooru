use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::api::ApiClient;
use crate::components::api_error::ApiErrorMessage;
use crate::components::loading_bar::LoadingState;
use crate::components::markdown::Markdown;
use crate::utils::format_time_short;

#[component]
pub fn TagViewPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let loading = expect_context::<LoadingState>();
    let params = use_params_map();

    let tag_name = Memo::new(move |_| {
        params.get().get("name").unwrap_or_default()
    });

    let tag = LocalResource::new(move || {
        let client = api.get();
        let name = tag_name.get();
        async move {
            loading.start();
            let result = client.get_tag(&name).await;
            loading.finish();
            result
        }
    });

    let siblings = LocalResource::new(move || {
        let client = api.get();
        let name = tag_name.get();
        async move { client.get_tag_siblings(&name).await.ok() }
    });

    view! {
        <Title text=move || format!("Tag: {}", tag_name.get()) />
        <div class="tag-view-page">
            <Suspense fallback=|| view! { <p>"Loading tag..."</p> }>
                {move || Suspend::new(async move {
                    match tag.await {
                        Ok(tag) => {
                            let names = tag.names.unwrap_or_default();
                            let primary_name = names.first().cloned().unwrap_or_default();
                            let aliases = if names.len() > 1 {
                                names[1..].to_vec()
                            } else {
                                vec![]
                            };
                            let category = tag.category.unwrap_or_default();
                            let usages = tag.usages.unwrap_or(0);
                            let created = tag.creation_time.as_deref()
                                .map(format_time_short)
                                .unwrap_or_default();
                            let description = tag.description.unwrap_or_default();
                            let implications = tag.implications.unwrap_or_default();
                            let suggestions = tag.suggestions.unwrap_or_default();
                            let category_class = format!("tag-category-{category}");

                            view! {
                                <div class="tag-view">
                                    <header class=category_class>
                                        <h1>{primary_name.clone()}</h1>
                                    </header>

                                    <section class="tag-summary">
                                        <dl>
                                            <dt>"Category"</dt><dd>{category}</dd>
                                            <dt>"Usages"</dt>
                                            <dd>
                                                <a href=format!("/posts?query={primary_name}")>
                                                    {usages}" post"{if usages == 1 { "" } else { "s" }}
                                                </a>
                                            </dd>
                                            <dt>"Created"</dt><dd>{created}</dd>
                                        </dl>

                                        {(!aliases.is_empty()).then(|| view! {
                                            <div class="tag-aliases">
                                                <h3>"Aliases"</h3>
                                                <ul>
                                                    {aliases.into_iter().map(|alias| {
                                                        view! { <li>{alias}</li> }
                                                    }).collect_view()}
                                                </ul>
                                            </div>
                                        })}
                                    </section>

                                    {(!description.is_empty()).then(|| view! {
                                        <section class="tag-description">
                                            <h2>"Description"</h2>
                                            <Markdown text=description />
                                        </section>
                                    })}

                                    {(!implications.is_empty()).then(|| view! {
                                        <section class="tag-implications">
                                            <h2>"Implications"</h2>
                                            <ul class="tag-list-inline">
                                                {implications.into_iter().map(|t| {
                                                    let name = t.names.first().cloned().unwrap_or_default();
                                                    let href = format!("/tag/{name}");
                                                    let class = format!("tag-category-{}", t.category);
                                                    view! {
                                                        <li class=class><a href=href>{name}</a></li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </section>
                                    })}

                                    {(!suggestions.is_empty()).then(|| view! {
                                        <section class="tag-suggestions">
                                            <h2>"Suggestions"</h2>
                                            <ul class="tag-list-inline">
                                                {suggestions.into_iter().map(|t| {
                                                    let name = t.names.first().cloned().unwrap_or_default();
                                                    let href = format!("/tag/{name}");
                                                    let class = format!("tag-category-{}", t.category);
                                                    view! {
                                                        <li class=class><a href=href>{name}</a></li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </section>
                                    })}

                                    // Siblings loaded separately
                                    <Suspense fallback=|| ()>
                                        {move || Suspend::new(async move {
                                            siblings.await.map(|data| {
                                                let sibs = data.results;
                                                (!sibs.is_empty()).then(|| view! {
                                                    <section class="tag-siblings">
                                                        <h2>"Siblings"</h2>
                                                        <ul class="tag-list-inline">
                                                            {sibs.into_iter().map(|s| {
                                                                let name = s.tag.names.as_ref()
                                                                    .and_then(|n| n.first().cloned())
                                                                    .unwrap_or_default();
                                                                let cat = s.tag.category.unwrap_or_default();
                                                                let href = format!("/tag/{name}");
                                                                let class = format!("tag-category-{cat}");
                                                                view! {
                                                                    <li class=class>
                                                                        <a href=href>{name}</a>
                                                                        <span class="sibling-count">{s.occurrences}</span>
                                                                    </li>
                                                                }
                                                            }).collect_view()}
                                                        </ul>
                                                    </section>
                                                })
                                            })
                                        })}
                                    </Suspense>
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
