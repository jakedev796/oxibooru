use leptos::prelude::*;
use leptos_meta::Title;
use oxibooru_shared::snapshot::SnapshotInfo;

use crate::api::ApiClient;
use crate::components::pagination::Pagination;
use crate::settings::SettingsState;
use crate::utils::{build_list_url, format_time_short, setup_scroll_listener, use_list_query_params};

#[component]
pub fn HistoryPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let settings = expect_context::<SettingsState>();
    let params = use_list_query_params(25);
    let endless = settings.inner.get_untracked().endless_scroll;

    let snapshots = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        async move { client.get_snapshots(p.offset, p.limit).await.ok() }
    });

    // Endless scroll state
    let accumulated = RwSignal::new(Vec::<SnapshotInfo>::new());
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
        let offset = loaded_up_to.get_untracked();
        let limit = params.get_untracked().limit;

        leptos::task::spawn_local(async move {
            match client.get_snapshots(offset, limit).await {
                Ok(data) => {
                    let new_count = data.results.len() as i64;
                    accumulated.update(|v| v.extend(data.results));
                    loaded_up_to.set(offset + new_count);
                    total_results.set(data.total);
                }
                Err(_) => {}
            }
            loading_more.set(false);
        });
    };

    if endless {
        setup_scroll_listener(loading_more, has_more, move || load_more());
    }

    view! {
        <Title text="History" />
        <div class="history-page">
            <h1>"History"</h1>
            <Suspense fallback=|| view! { <p>"Loading history..."</p> }>
                {move || Suspend::new(async move {
                    match snapshots.await {
                        Some(data) => {
                            if endless {
                                accumulated.set(data.results);
                                loaded_up_to.set(data.offset + accumulated.get_untracked().len() as i64);
                                total_results.set(data.total);

                                // Wrap accumulated items with stable indices for keying
                                let indexed = Signal::derive(move || {
                                    accumulated.get().into_iter().enumerate().collect::<Vec<_>>()
                                });

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
                                            <For
                                                each=move || indexed.get()
                                                key=|(idx, _)| *idx
                                                children=move |(_, snap)| {
                                                    render_snapshot_row(snap)
                                                }
                                            />
                                        </tbody>
                                    </table>
                                    <div class="scroll-sentinel">
                                        {move || loading_more.get().then(|| view! {
                                            <p class="loading-more">"Loading more history..."</p>
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
                                                render_snapshot_row(snap)
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

fn render_snapshot_row(snap: SnapshotInfo) -> impl IntoView {
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
