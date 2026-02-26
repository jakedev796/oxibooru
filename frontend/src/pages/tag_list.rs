use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use oxibooru_shared::tag::TagInfo;

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::api_error::ApiErrorMessage;
use crate::components::loading_bar::LoadingState;
use crate::components::pagination::Pagination;
use crate::components::search_bar::SearchBar;
use crate::settings::SettingsState;
use crate::utils::{build_list_url, format_relative_time, setup_scroll_listener, use_list_query_params};

const FIELDS: &str = "names,suggestions,implications,creationTime,usages,category";

/// Sort tokens for tag list (must match server search Token serialization).
const SORT_NAME: &str = "sort:name";
const SORT_CATEGORY: &str = "sort:category";
const SORT_USAGES: &str = "sort:usages";
const SORT_IMPLICATION_COUNT: &str = "sort:implication-count";
const SORT_SUGGESTION_COUNT: &str = "sort:suggestion-count";
const SORT_CREATION_TIME: &str = "sort:creation-time";

/// Returns the query to use when clicking a sort header: toggles asc/desc when current sort matches.
fn sort_href_query(current: &str, sort_asc: &str) -> String {
    let sort_desc = format!("-{sort_asc}");
    if current == sort_asc {
        sort_desc
    } else if current == sort_desc {
        sort_asc.to_string()
    } else {
        sort_asc.to_string()
    }
}

#[component]
pub fn TagListPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let auth = expect_context::<AuthState>();
    let loading = expect_context::<LoadingState>();
    let settings = expect_context::<SettingsState>();
    let params = use_list_query_params(50);
    let navigate = use_navigate();
    let endless = settings.inner.get_untracked().endless_scroll;
    let show_tag_categories_link = auth.has_privilege("tag_category_list");

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
            <div class="tag-list-header">
                <SearchBar
                    query=query_signal
                    on_submit=on_search
                    tag_autocomplete=true
                    placeholder="Search\u{2026}"
                />
                <a class="search-append" href="/help/search/tags">"Syntax help"</a>
                {move || show_tag_categories_link.then(|| view! {
                    <a class="search-append" href="/tag-categories">"Tag categories"</a>
                })}
            </div>
            <Suspense fallback=|| view! { <p>"Loading tags..."</p> }>
                {move || Suspend::new(async move {
                    match tags.await {
                        Ok(data) => {
                            if endless {
                                accumulated.set(data.results);
                                loaded_up_to.set(data.offset + accumulated.get_untracked().len() as i64);
                                total_results.set(data.total);
                                view! {
                                    {move || {
                                        let total = total_results.get();
                                        let limit = params.get().limit;
                                        let total_pages = if limit > 0 && total > 0 {
                                            (total + limit - 1) / limit
                                        } else {
                                            0
                                        };
                                        (total_pages > 0).then(|| view! {
                                            <p class="page-header">
                                                <span>{format!("Page 1 of {}", total_pages)}</span>
                                            </p>
                                        })
                                    }}
                                    <table class="tag-list">
                                        <thead>
                                            {move || {
                                                let q = params.get().query.clone();
                                                view! {
                                                    <tr>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_NAME), 0, params.get().limit)>
                                                                "Tag name(s)"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_CATEGORY), 0, params.get().limit)>
                                                                "Category"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_USAGES), 0, params.get().limit)>
                                                                "Usages"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_IMPLICATION_COUNT), 0, params.get().limit)>
                                                                "Implications"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_SUGGESTION_COUNT), 0, params.get().limit)>
                                                                "Suggestions"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_CREATION_TIME), 0, params.get().limit)>
                                                                "Created on"
                                                            </a>
                                                        </th>
                                                    </tr>
                                                }
                                            }}
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
                                    </div>
                                }.into_any()
                            } else {
                                let total = data.total;
                                let offset = data.offset;
                                let limit = data.limit;
                                let query_for_page = params.get().query.clone();
                                let total_pages = if limit > 0 { (total + limit - 1) / limit } else { 0 };
                                let current_page = if limit > 0 { offset / limit + 1 } else { 1 };
                                view! {
                                    {(total_pages > 0).then(|| view! {
                                        <p class="page-header">
                                            <span>{format!("Page {} of {}", current_page, total_pages)}</span>
                                        </p>
                                    })}
                                    <table class="tag-list">
                                        <thead>
                                            {move || {
                                                let q = params.get().query.clone();
                                                view! {
                                                    <tr>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_NAME), 0, params.get().limit)>
                                                                "Tag name(s)"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_CATEGORY), 0, params.get().limit)>
                                                                "Category"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_USAGES), 0, params.get().limit)>
                                                                "Usages"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_IMPLICATION_COUNT), 0, params.get().limit)>
                                                                "Implications"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_SUGGESTION_COUNT), 0, params.get().limit)>
                                                                "Suggestions"
                                                            </a>
                                                        </th>
                                                        <th class="sortable">
                                                            <a href=build_list_url("/tags", &sort_href_query(&q, SORT_CREATION_TIME), 0, params.get().limit)>
                                                                "Created on"
                                                            </a>
                                                        </th>
                                                    </tr>
                                                }
                                            }}
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
    let settings = expect_context::<SettingsState>();
    let primary_name = tag.names.as_ref().and_then(|n| n.first().cloned()).unwrap_or_default();
    let category = tag.category.clone().unwrap_or_default();
    let usages = tag.usages.unwrap_or(0);
    let created = tag.creation_time.as_deref().map(format_relative_time).unwrap_or_default();
    let tag_href = format!("/tag/{primary_name}");
    let display_name = settings.display_name(&primary_name);
    let tag_cell_class = format!("tag-category-{category}");
    let category_cell_class = tag_cell_class.clone();
    let posts_query = primary_name.clone();
    view! {
        <tr>
            <td class=format!("tag-name-cell {tag_cell_class}")>
                <a href=tag_href>{display_name}</a>
            </td>
            <td class=category_cell_class>{category}</td>
            <td class="tag-usages-cell">
                {if usages == 0 {
                    view! { <span>"0"</span> }.into_any()
                } else {
                    view! {
                        <a href=format!("/posts?query={}", url_encode(&posts_query))>{usages}</a>
                    }.into_any()
                }}
            </td>
            <td class="tag-relations">
                {match &tag.implications {
                    &None => view! { <span class="tag-relation-empty">"-"</span> }.into_any(),
                    &Some(ref list) if list.is_empty() => view! { <span class="tag-relation-empty">"-"</span> }.into_any(),
                    &Some(ref list) => view! {
                        <ul class="tag-relation-list">
                            {list.iter().map(|t| {
                                let name = t.names.first().cloned().unwrap_or_default();
                                let cat = t.category.clone();
                                let class = format!("tag-category-{cat}");
                                let display = settings.display_name(&name);
                                let href = format!("/tag/{name}");
                                view! {
                                    <li class=class>
                                        <a href=href>{display}</a>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any(),
                }}
            </td>
            <td class="tag-relations">
                {match &tag.suggestions {
                    &None => view! { <span class="tag-relation-empty">"-"</span> }.into_any(),
                    &Some(ref list) if list.is_empty() => view! { <span class="tag-relation-empty">"-"</span> }.into_any(),
                    &Some(ref list) => view! {
                        <ul class="tag-relation-list">
                            {list.iter().map(|t| {
                                let name = t.names.first().cloned().unwrap_or_default();
                                let cat = t.category.clone();
                                let class = format!("tag-category-{cat}");
                                let display = settings.display_name(&name);
                                let href = format!("/tag/{name}");
                                view! {
                                    <li class=class>
                                        <a href=href>{display}</a>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any(),
                }}
            </td>
            <td class="tag-created-cell">{created}</td>
        </tr>
    }
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}
