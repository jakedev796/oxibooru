use leptos::prelude::*;
use leptos_meta::Title;

use crate::api::ApiClient;
use crate::components::pagination::Pagination;
use crate::utils::{build_list_url, format_time_short, use_list_query_params};

#[component]
pub fn HistoryPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_list_query_params(25);

    let snapshots = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        async move { client.get_snapshots(p.offset, p.limit).await.ok() }
    });

    view! {
        <Title text="History" />
        <div class="history-page">
            <h1>"History"</h1>
            <Suspense fallback=|| view! { <p>"Loading history..."</p> }>
                {move || Suspend::new(async move {
                    match snapshots.await {
                        Some(data) => {
                            let total = data.total;
                            let offset = data.offset;
                            let limit = data.limit;
                            view! {
                                <table class="history-list">
                                    <thead>
                                        <tr>
                                            <th>"Time"</th>
                                            <th>"User"</th>
                                            <th>"Operation"</th>
                                            <th>"Type"</th>
                                            <th>"ID"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {data.results.into_iter().map(|snap| {
                                            let time = snap.time.as_deref()
                                                .map(format_time_short)
                                                .unwrap_or_default();
                                            let user_name = snap.user.flatten()
                                                .map(|u| u.name)
                                                .unwrap_or_else(|| "Anonymous".to_string());
                                            let operation = snap.operation
                                                .map(|o| format!("{o:?}"))
                                                .unwrap_or_default();
                                            let resource_type = snap.resource_type
                                                .map(|t| format!("{t:?}"))
                                                .unwrap_or_default();
                                            let resource_id = snap.resource_id.unwrap_or_default();
                                            let link = resource_link(&resource_type, &resource_id);
                                            let user_href = format!("/user/{user_name}");
                                            let op_class = format!("operation-{}", operation.to_lowercase());
                                            view! {
                                                <tr>
                                                    <td><time>{time}</time></td>
                                                    <td><a href=user_href>{user_name}</a></td>
                                                    <td class=op_class>{operation}</td>
                                                    <td>{resource_type}</td>
                                                    <td>{
                                                        if let Some(href) = link {
                                                            view! { <a href=href>{resource_id.clone()}</a> }.into_any()
                                                        } else {
                                                            view! { <span>{resource_id.clone()}</span> }.into_any()
                                                        }
                                                    }</td>
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
                                        build_list_url("/history", "", offset, limit)
                                    })
                                />
                            }.into_any()
                        }
                        None => view! {
                            <p class="error">"Failed to load history."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

/// Generate a link to the resource based on its type and ID.
fn resource_link(resource_type: &str, resource_id: &str) -> Option<String> {
    match resource_type {
        "Post" => resource_id.parse::<i64>().ok().map(|id| format!("/post/{id}")),
        "Tag" | "TagCategory" | "TagImplication" | "TagSuggestion" => Some(format!("/tag/{resource_id}")),
        "Pool" | "PoolCategory" => resource_id.parse::<i64>().ok().map(|id| format!("/pool/{id}")),
        "User" | "UserToken" => Some(format!("/user/{resource_id}")),
        "Comment" => None,
        _ => None,
    }
}
