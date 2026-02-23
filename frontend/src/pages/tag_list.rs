use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use crate::api::ApiClient;
use crate::components::pagination::Pagination;
use crate::components::search_bar::SearchBar;
use crate::utils::{build_list_url, format_time_short, use_list_query_params};

const FIELDS: &str = "names,suggestions,implications,creationTime,usages,category";

#[component]
pub fn TagListPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_list_query_params(50);
    let navigate = use_navigate();

    let tags = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        async move { client.get_tags(&p.query, p.offset, p.limit, FIELDS).await.ok() }
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

    view! {
        <Title text="Tags" />
        <div class="tag-list-page">
            <SearchBar query=query_signal on_submit=on_search />
            <Suspense fallback=|| view! { <p>"Loading tags..."</p> }>
                {move || Suspend::new(async move {
                    match tags.await {
                        Some(data) => {
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
                                            let primary_name = tag.names.as_ref()
                                                .and_then(|n| n.first().cloned())
                                                .unwrap_or_default();
                                            let category = tag.category.clone().unwrap_or_default();
                                            let usages = tag.usages.unwrap_or(0);
                                            let implications = tag.implications.as_ref()
                                                .map(|v| v.iter().map(|t| t.names.first().cloned().unwrap_or_default()).collect::<Vec<_>>().join(", "))
                                                .unwrap_or_default();
                                            let suggestions = tag.suggestions.as_ref()
                                                .map(|v| v.iter().map(|t| t.names.first().cloned().unwrap_or_default()).collect::<Vec<_>>().join(", "))
                                                .unwrap_or_default();
                                            let created = tag.creation_time.as_deref()
                                                .map(format_time_short)
                                                .unwrap_or_default();
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
                        None => view! {
                            <p class="error">"Failed to load tags."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
