use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use crate::api::ApiClient;
use crate::components::pagination::Pagination;
use crate::components::search_bar::SearchBar;
use crate::utils::{build_list_url, format_time_short, use_list_query_params};

const FIELDS: &str = "id,names,creationTime,postCount,category";

#[component]
pub fn PoolListPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_list_query_params(50);
    let navigate = use_navigate();

    let pools = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        async move { client.get_pools(&p.query, p.offset, p.limit, FIELDS).await.ok() }
    });

    let query_signal = Signal::derive(move || params.get().query);

    let on_search = {
        let navigate = navigate.clone();
        Callback::new(move |query: String| {
            let limit = params.get().limit;
            let url = build_list_url("/pools", &query, 0, limit);
            navigate(&url, Default::default());
        })
    };

    view! {
        <Title text="Pools" />
        <div class="pool-list-page">
            <SearchBar query=query_signal on_submit=on_search />
            <Suspense fallback=|| view! { <p>"Loading pools..."</p> }>
                {move || Suspend::new(async move {
                    match pools.await {
                        Some(data) => {
                            let total = data.total;
                            let offset = data.offset;
                            let limit = data.limit;
                            let query_for_page = params.get().query.clone();
                            view! {
                                <table class="pool-list">
                                    <thead>
                                        <tr>
                                            <th>"Pool"</th>
                                            <th>"Category"</th>
                                            <th>"Posts"</th>
                                            <th>"Created"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {data.results.into_iter().map(|pool| {
                                            let id = pool.id.unwrap_or(0);
                                            let primary_name = pool.names.as_ref()
                                                .and_then(|n| n.first().cloned())
                                                .unwrap_or_default();
                                            let category = pool.category.clone().unwrap_or_default();
                                            let post_count = pool.post_count.unwrap_or(0);
                                            let created = pool.creation_time.as_deref()
                                                .map(format_time_short)
                                                .unwrap_or_default();
                                            let href = format!("/pool/{id}");
                                            let row_class = format!("pool-category-{category}");
                                            view! {
                                                <tr class=row_class>
                                                    <td><a href=href>{primary_name}</a></td>
                                                    <td>{category}</td>
                                                    <td>{post_count}</td>
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
                                        build_list_url("/pools", &query_for_page, offset, limit)
                                    })
                                />
                            }.into_any()
                        }
                        None => view! {
                            <p class="error">"Failed to load pools."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
