use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::api::ApiClient;
use crate::components::api_error::ApiErrorMessage;
use crate::components::markdown::Markdown;
use crate::components::tag_view_wrapper::TagViewWrapper;
use crate::settings::SettingsState;
use crate::tag_cache::TagCache;

#[component]
pub fn TagViewPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let settings = expect_context::<SettingsState>();
    let tag_cache = expect_context::<TagCache>();
    let params = use_params_map();

    let tag_name = Memo::new(move |_| params.get().get("name").unwrap_or_default());

    let (tag_data, set_tag_data) = signal(None);
    let (error, set_error) = signal(None);

    // Use cached data if available, then fetch fresh in background
    Effect::new(move || {
        let name = tag_name.get();
        if let Some(cached) = tag_cache.get(&name) {
            set_tag_data.set(Some(cached));
        }
        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            match client.get_tag(&name).await {
                Ok(tag) => {
                    tag_cache.set(&name, tag.clone());
                    set_tag_data.set(Some(tag));
                }
                Err(e) => {
                    if tag_data.get_untracked().is_none() {
                        set_error.set(Some(e));
                    }
                }
            }
        });
    });

    view! {
        <Title text=move || format!("Tag: {}", tag_name.get()) />
        <div class="tag-view-page">
            {move || {
                if let Some(e) = error.get() {
                    return view! { <ApiErrorMessage error=e /> }.into_any();
                }
                match tag_data.get() {
                    None => view! {}.into_any(),
                    Some(tag) => {
                        let names = tag.names.unwrap_or_default();
                        let primary_name = names.first().cloned().unwrap_or_default();
                        let aliases: Vec<String> = if names.len() > 1 {
                            names[1..].to_vec()
                        } else {
                            vec![]
                        };
                        let category = tag.category.unwrap_or_default();
                        let usages = tag.usages.unwrap_or(0);
                        let description = tag.description.unwrap_or_default();
                        let implications = tag.implications.unwrap_or_default();
                        let suggestions = tag.suggestions.unwrap_or_default();

                        let category_class = format!("tag-category-{category}");
                        let search_href = format!("/posts?query={primary_name}");

                        view! {
                            <TagViewWrapper tag_name=primary_name active_tab="summary">
                                <div class="tag-summary">
                                    <section class="details">
                                        <section>
                                            "Category: "
                                            <span class=category_class>{category}</span>
                                        </section>

                                        <section>
                                            "Aliases:"<br />
                                            {if aliases.is_empty() {
                                                view! { <span class="none">"(none)"</span> }.into_any()
                                            } else {
                                                view! {
                                                    <ul>
                                                        {aliases.into_iter().enumerate().map(|(i, alias)| {
                                                            let display = settings.display_name(&alias);
                                                            let href = format!("/tag/{alias}");
                                                            view! {
                                                                <li>
                                                                    {(i > 0).then(|| ", ")}
                                                                    <a href=href>{display}</a>
                                                                </li>
                                                            }
                                                        }).collect_view()}
                                                    </ul>
                                                }.into_any()
                                            }}
                                        </section>

                                        <section>
                                            "Implications:"<br />
                                            {if implications.is_empty() {
                                                view! { <span class="none">"(none)"</span> }.into_any()
                                            } else {
                                                view! {
                                                    <ul>
                                                        {implications.into_iter().enumerate().map(|(i, t)| {
                                                            let name = t.names.first().cloned().unwrap_or_default();
                                                            let href = format!("/tag/{name}");
                                                            let display = settings.display_name(&name);
                                                            let class = format!("tag-category-{}", t.category);
                                                            view! {
                                                                <li>
                                                                    {(i > 0).then(|| ", ")}
                                                                    <a class=class href=href>{display}</a>
                                                                </li>
                                                            }
                                                        }).collect_view()}
                                                    </ul>
                                                }.into_any()
                                            }}
                                        </section>

                                        <section>
                                            "Suggestions:"<br />
                                            {if suggestions.is_empty() {
                                                view! { <span class="none">"(none)"</span> }.into_any()
                                            } else {
                                                view! {
                                                    <ul>
                                                        {suggestions.into_iter().enumerate().map(|(i, t)| {
                                                            let name = t.names.first().cloned().unwrap_or_default();
                                                            let href = format!("/tag/{name}");
                                                            let display = settings.display_name(&name);
                                                            let class = format!("tag-category-{}", t.category);
                                                            view! {
                                                                <li>
                                                                    {(i > 0).then(|| ", ")}
                                                                    <a class=class href=href>{display}</a>
                                                                </li>
                                                            }
                                                        }).collect_view()}
                                                    </ul>
                                                }.into_any()
                                            }}
                                        </section>
                                    </section>

                                    <section class="description">
                                        <hr />
                                        {if description.is_empty() {
                                            view! { <p>"This tag has no description yet."</p> }.into_any()
                                        } else {
                                            view! { <Markdown text=description /> }.into_any()
                                        }}
                                        <p>
                                            "This tag has "
                                            <a href=search_href>{usages}" usage(s)"</a>
                                            "."
                                        </p>
                                    </section>
                                </div>
                            </TagViewWrapper>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
