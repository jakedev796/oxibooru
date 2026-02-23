use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use crate::api::ApiClient;
use crate::components::pagination::Pagination;
use crate::components::search_bar::SearchBar;
use crate::utils::{build_list_url, format_time_short, use_list_query_params};

#[component]
pub fn UserListPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_list_query_params(30);
    let navigate = use_navigate();

    let users = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        async move { client.get_users(&p.query, p.offset, p.limit).await.ok() }
    });

    let query_signal = Signal::derive(move || params.get().query);

    let on_search = {
        let navigate = navigate.clone();
        Callback::new(move |query: String| {
            let limit = params.get().limit;
            let url = build_list_url("/users", &query, 0, limit);
            navigate(&url, Default::default());
        })
    };

    view! {
        <Title text="Users" />
        <div class="user-list-page">
            <SearchBar query=query_signal on_submit=on_search />
            <Suspense fallback=|| view! { <p>"Loading users..."</p> }>
                {move || Suspend::new(async move {
                    match users.await {
                        Some(data) => {
                            let total = data.total;
                            let offset = data.offset;
                            let limit = data.limit;
                            let query_for_page = params.get().query.clone();
                            view! {
                                <div class="user-grid">
                                    {data.results.into_iter().map(|user| {
                                        let name = user.name.clone().unwrap_or_default();
                                        let avatar_url = user.avatar_url.clone().unwrap_or_default();
                                        let rank = user.rank.map(|r| format!("{r:?}").to_lowercase()).unwrap_or_default();
                                        let created = user.creation_time.as_deref()
                                            .map(format_time_short)
                                            .unwrap_or_default();
                                        let href = format!("/user/{name}");
                                        view! {
                                            <article class="user-card">
                                                <a href=href>
                                                    <img class="user-avatar" src=avatar_url loading="lazy" alt=name.clone() />
                                                    <div class="user-info">
                                                        <span class="user-name">{name}</span>
                                                        <span class=format!("user-rank rank-{rank}")>{rank.clone()}</span>
                                                        <span class="user-date">{created}</span>
                                                    </div>
                                                </a>
                                            </article>
                                        }
                                    }).collect_view()}
                                </div>
                                <Pagination
                                    offset=offset
                                    limit=limit
                                    total=total
                                    href_for_page=Callback::new(move |(offset, limit)| {
                                        build_list_url("/users", &query_for_page, offset, limit)
                                    })
                                />
                            }.into_any()
                        }
                        None => view! {
                            <p class="error">"Failed to load users."</p>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
