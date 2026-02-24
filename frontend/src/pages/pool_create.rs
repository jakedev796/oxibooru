use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use oxibooru_shared::category::PoolCategoryInfo;

use crate::api::pools::CreatePoolBody;
use crate::api::{ApiClient, ApiError};

#[component]
pub fn PoolCreatePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let navigate = use_navigate();

    let (loading, set_loading) = signal(true);
    let categories = RwSignal::new(Vec::<PoolCategoryInfo>::new());

    // Form signals
    let (names_str, set_names_str) = signal(String::new());
    let (category, set_category) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (posts_str, set_posts_str) = signal(String::new());

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    // Load categories
    Effect::new(move || {
        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            if let Ok(resp) = client.get_pool_categories().await {
                categories.set(resp.results);
            }
            set_loading.set(false);
        });
    });

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let parsed_names: Vec<String> = names_str
            .get_untracked()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if parsed_names.is_empty() {
            set_error_msg.set(Some("At least one name is required.".into()));
            return;
        }

        let cat = category.get_untracked();
        let desc = description.get_untracked();
        let posts: Option<Vec<i64>> = {
            let s = posts_str.get_untracked();
            if s.trim().is_empty() {
                None
            } else {
                match s
                    .split(',')
                    .map(|p| p.trim().parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()
                {
                    Ok(ids) => Some(ids),
                    Err(_) => {
                        set_error_msg.set(Some("Invalid post IDs.".into()));
                        return;
                    }
                }
            }
        };

        let body = CreatePoolBody {
            names: parsed_names,
            category: if cat.is_empty() { "default".into() } else { cat },
            description: if desc.is_empty() { None } else { Some(desc) },
            posts,
        };

        set_submitting.set(true);
        set_error_msg.set(None);
        let client = api.get_untracked();
        let nav = navigate.clone();

        leptos::task::spawn_local(async move {
            match client.create_pool(&body).await {
                Ok(pool) => {
                    let id = pool.id.unwrap_or(0);
                    nav(&format!("/pool/{id}"), Default::default());
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text="Create Pool" />
        <div class="content-wrapper">
            <h1>"Create Pool"</h1>
            <form
                class="form-grid"
                on:submit=on_submit
                style:display=move || if loading.get() { "none" } else { "" }
            >
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}

                <div class="form-row">
                    <label for="pool-names">"Names (comma-separated)"</label>
                    <input
                        id="pool-names"
                        type="text"
                        prop:value=move || names_str.get()
                        on:input=move |ev| set_names_str.set(event_target_value(&ev))
                        disabled=move || submitting.get()
                        placeholder="e.g. my_pool"
                    />
                </div>

                <div class="form-row">
                    <label for="pool-category">"Category"</label>
                    <select
                        id="pool-category"
                        prop:value=move || category.get()
                        on:change=move |ev| set_category.set(event_target_value(&ev))
                        disabled=move || submitting.get()
                    >
                        {move || categories.get().into_iter().map(|cat| {
                            let name = cat.name.clone().unwrap_or_default();
                            let val = name.clone();
                            view! { <option value=val>{name}</option> }
                        }).collect_view()}
                    </select>
                </div>

                <div class="form-row">
                    <label for="pool-desc">"Description"</label>
                    <textarea
                        id="pool-desc"
                        prop:value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        rows=4
                        disabled=move || submitting.get()
                    />
                </div>

                <div class="form-row">
                    <label for="pool-posts">"Posts (comma-separated IDs)"</label>
                    <input
                        id="pool-posts"
                        type="text"
                        prop:value=move || posts_str.get()
                        on:input=move |ev| set_posts_str.set(event_target_value(&ev))
                        disabled=move || submitting.get()
                        placeholder="e.g. 1, 5, 12"
                    />
                </div>

                <div class="form-row buttons">
                    <button type="submit" disabled=move || submitting.get()>
                        {move || if submitting.get() { "Creating..." } else { "Create pool" }}
                    </button>
                </div>
            </form>
        </div>
    }
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
