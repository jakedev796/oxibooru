use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use oxibooru_shared::user::UserInfo;

use crate::api::ApiClient;
use crate::components::api_error::ApiErrorMessage;
use crate::components::loading_bar::LoadingState;
use crate::components::pagination::Pagination;
use crate::components::search_bar::SearchBar;
use crate::settings::SettingsState;
use crate::utils::{build_list_url, format_time_short, setup_scroll_listener, use_list_query_params};

#[component]
pub fn UserListPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let loading = expect_context::<LoadingState>();
    let settings = expect_context::<SettingsState>();
    let params = use_list_query_params(30);
    let navigate = use_navigate();
    let endless = settings.inner.get_untracked().endless_scroll;

    let users = LocalResource::new(move || {
        let client = api.get();
        let p = params.get();
        async move {
            loading.start();
            let result = client.get_users(&p.query, p.offset, p.limit).await;
            loading.finish();
            result
        }
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

    // Endless scroll state
    let accumulated = RwSignal::new(Vec::<UserInfo>::new());
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
            if let Ok(data) = client.get_users(&query, offset, limit).await {
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
        <Title text="Users" />
        <div class="user-list-page">
            <SearchBar query=query_signal on_submit=on_search />
            <Suspense fallback=|| view! { <p>"Loading users..."</p> }>
                {move || Suspend::new(async move {
                    match users.await {
                        Ok(data) => {
                            if endless {
                                accumulated.set(data.results);
                                loaded_up_to.set(data.offset + accumulated.get_untracked().len() as i64);
                                total_results.set(data.total);

                                view! {
                                    <div class="user-grid">
                                        <For
                                            each=move || accumulated.get()
                                            key=|user| user.name.clone().unwrap_or_default()
                                            children=move |user| {
                                                render_user_card(user)
                                            }
                                        />
                                    </div>
                                    <div class="scroll-sentinel">
                                        {move || loading_more.get().then(|| view! {
                                            <p class="loading-more">"Loading more users..."</p>
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
                                    <div class="user-grid">
                                        {data.results.into_iter().map(|user| {
                                            render_user_card(user)
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

fn render_user_card(user: UserInfo) -> impl IntoView {
    let name = user.name.clone().unwrap_or_default();
    let avatar_url = user.avatar_url.clone().unwrap_or_default();
    let rank = user.rank.map(|r| format!("{r:?}").to_lowercase()).unwrap_or_default();
    let created = user.creation_time.as_deref().map(format_time_short).unwrap_or_default();
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
}
