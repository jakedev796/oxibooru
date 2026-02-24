use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use oxibooru_shared::tag::TagInfo;

use crate::api::ApiClient;
use crate::components::api_error::ApiErrorMessage;
use crate::components::loading_bar::LoadingState;
use crate::components::pagination::Pagination;
use crate::components::search_bar::SearchBar;
use crate::settings::SettingsState;
use crate::utils::{build_list_url, format_time_short, setup_scroll_listener, use_list_query_params};

const FIELDS: &str = "names,suggestions,implications,creationTime,usages,category";

#[component]
pub fn TagListPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let loading = expect_context::<LoadingState>();
    let settings = expect_context::<SettingsState>();
    let params = use_list_query_params(50);
    let navigate = use_navigate();
    let endless = settings.inner.get_untracked().endless_scroll;

    let tags = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        async move {
            loading.start();
            let result = client.get_tags(&p.query, p.offset, p.limit, FIELDS).await;
            loading.finish();
            result
        }
    });

    let query_signal = Signal::derive(move || params.get().query);

    let on_search = {
        let navigate = navigate.clone();
        Callback::new(move |query: String| {
            let limit = params.get().limit;
            let url = build_list_url("/tags", &query, 0, limit);
            navigate(&url, Default::default());
        })
    };

    // Endless scroll state
    let accumulated = RwSignal::new(Vec::<TagInfo>::new());
    let loaded_up_to = RwSignal::new(0i64);
    let total_results = RwSignal::new(0i64);
    let loading_more = RwSignal::new(false);
    let has_more = Signal::derive(move || loaded_up_to.get() < total_results.get());

    let load_more = move || {
        if loading_more.get_untracked() || !has_more.get_untracked() {
            return;
        }
        loading_more.set(true);
        let client = api.get_untracked();
        let query = params.get_untracked().query.clone();
        let offset = loaded_up_to.get_untracked();
        let limit = params.get_untracked().limit;

        leptos::task::spawn_local(async move {
            if let Ok(data) = client.get_tags(&query, offset, limit, FIELDS).await {
                let new_count = data.results.len() as i64;
                accumulated.update(|v| v.extend(data.results));
                loaded_up_to.set(offset + new_count);
                total_results.set(data.total);
            }
            loading_more.set(false);
        });
    };

    if endless {
        setup_scroll_listener(loading_more, has_more, load_more);
    }

    view! {
        <Title text="Tags" />
        <div class="tag-list-page">
            <SearchBar query=query_signal on_submit=on_search />
            <Suspense fallback=|| view! { <p>"Loading tags..."</p> }>
                {move || Suspend::new(async move {
                    match tags.await {
                        Ok(data) => {
                            if endless {
                                accumulated.set(data.results);
                                loaded_up_to.set(data.offset + accumulated.get_untracked().len() as i64);
                                total_results.set(data.total);

                                view! {
                                    <table class="tag-list">
                                        <thead>
                                            <tr>
                                                <th>"Tag"</th>
                                                <th>"Category"</th>
                                                <th>"Usages"</th>
                                                <th>"Implications"</th>
                                                <th>"Suggestions"</th>
                                                <th>"Created"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            <For
                                                each=move || accumulated.get()
                                                key=|tag| tag.names.as_ref().and_then(|n| n.first().cloned()).unwrap_or_default()
                                                children=move |tag| {
                                                    render_tag_row(tag)
                                                }
                                            />
                                        </tbody>
                                    </table>
                                    <div class="scroll-sentinel">
                                        {move || loading_more.get().then(|| view! {
                                            <p class="loading-more">"Loading more tags..."</p>
                                        })}
                                        {move || {
                                            let total = total_results.get();
                                            let loaded = loaded_up_to.get();
                                            (!loading_more.get() && loaded >= total && total > 0).then(|| {
                                                view! { <p class="end-of-results">{format!("{total} results")}</p> }
                                            })
                                        }}
                                    </div>
                                }.into_any()
                            } else {
                                let total = data.total;
                                let offset = data.offset;
                                let limit = data.limit;
                                let query_for_page = params.get().query.clone();
                                view! {
                                    <table class="tag-list">
                                        <thead>
                                            <tr>
                                                <th>"Tag"</th>
                                                <th>"Category"</th>
                                                <th>"Usages"</th>
                                                <th>"Implications"</th>
                                                <th>"Suggestions"</th>
                                                <th>"Created"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {data.results.into_iter().map(|tag| {
                                                render_tag_row(tag)
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                    <Pagination
                                        offset=offset
                                        limit=limit
                                        total=total
                                        href_for_page=Callback::new(move |(offset, limit)| {
                                            build_list_url("/tags", &query_for_page, offset, limit)
                                        })
                                    />
                                }.into_any()
                            }
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

fn render_tag_row(tag: TagInfo) -> impl IntoView {
    let primary_name = tag.names.as_ref().and_then(|n| n.first().cloned()).unwrap_or_default();
    let category = tag.category.clone().unwrap_or_default();
    let usages = tag.usages.unwrap_or(0);
    let implications = tag
        .implications
        .as_ref()
        .map(|v| {
            v.iter()
                .map(|t| t.names.first().cloned().unwrap_or_default())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    let suggestions = tag
        .suggestions
        .as_ref()
        .map(|v| {
            v.iter()
                .map(|t| t.names.first().cloned().unwrap_or_default())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    let created = tag.creation_time.as_deref().map(format_time_short).unwrap_or_default();
    let href = format!("/tag/{primary_name}");
    let row_class = format!("tag-category-{category}");
    view! {
        <tr class=row_class>
            <td><a href=href>{primary_name}</a></td>
            <td>{category}</td>
            <td>{usages}</td>
            <td class="tag-relations">{implications}</td>
            <td class="tag-relations">{suggestions}</td>
            <td>{created}</td>
        </tr>
    }
}
