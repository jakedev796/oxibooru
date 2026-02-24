use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use oxibooru_shared::category::PoolCategoryInfo;

use crate::api::pools::UpdatePoolBody;
use crate::api::{ApiClient, ApiError};

#[component]
pub fn PoolEditPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_params_map();
    let pool_id = move || params.get().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);

    let (loading, set_loading) = signal(true);
    let (load_error, set_load_error) = signal(false);
    let categories = RwSignal::new(Vec::<PoolCategoryInfo>::new());

    // Form signals
    let (names_str, set_names_str) = signal(String::new());
    let (category, set_category) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (posts_str, set_posts_str) = signal(String::new());
    let (version, set_version) = signal(String::new());

    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);

    // Load pool + categories
    Effect::new(move || {
        let client = api.get_untracked();
        let id = pool_id();
        leptos::task::spawn_local(async move {
            if let Ok(resp) = client.get_pool_categories().await {
                categories.set(resp.results);
            }
            match client.get_pool(id).await {
                Ok(pool) => {
                    set_names_str.set(pool.names.unwrap_or_default().join(", "));
                    set_category.set(pool.category.unwrap_or_default());
                    set_description.set(pool.description.unwrap_or_default());
                    let post_ids: Vec<String> = pool
                        .posts
                        .unwrap_or_default()
                        .iter()
                        .map(|p| p.id.to_string())
                        .collect();
                    set_posts_str.set(post_ids.join(", "));
                    set_version.set(pool.version.unwrap_or_default());
                    set_loading.set(false);
                }
                Err(_) => {
                    set_load_error.set(true);
                    set_loading.set(false);
                }
            }
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

        let posts: Option<Vec<i64>> = {
            let s = posts_str.get_untracked();
            if s.trim().is_empty() {
                Some(vec![])
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

        let body = UpdatePoolBody {
            version: version.get_untracked(),
            category: Some(category.get_untracked()),
            names: Some(parsed_names),
            description: Some(description.get_untracked()),
            posts,
        };

        set_submitting.set(true);
        set_error_msg.set(None);
        set_success_msg.set(None);
        let client = api.get_untracked();
        let id = pool_id();

        leptos::task::spawn_local(async move {
            match client.update_pool(id, &body).await {
                Ok(updated) => {
                    set_version.set(updated.version.unwrap_or_default());
                    set_success_msg.set(Some("Pool updated.".into()));
                    set_submitting.set(false);
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <Title text=move || format!("Edit Pool {}", pool_id()) />
        <div class="content-wrapper">
            <h1>{move || format!("Edit Pool #{}", pool_id())}</h1>
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}
            {move || load_error.get().then(|| view! { <p class="error">"Pool not found."</p> })}
            <form
                class="form-grid"
                on:submit=on_submit
                style:display=move || if loading.get() || load_error.get() { "none" } else { "" }
            >
                {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}
                {move || success_msg.get().map(|msg| view! { <p class="success">{msg}</p> })}

                <div class="form-row">
                    <label for="pool-names">"Names (comma-separated)"</label>
                    <input
                        id="pool-names"
                        type="text"
                        prop:value=move || names_str.get()
                        on:input=move |ev| set_names_str.set(event_target_value(&ev))
                        disabled=move || submitting.get()
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
                    />
                </div>

                <div class="form-row buttons">
                    <button type="submit" disabled=move || submitting.get()>
                        {move || if submitting.get() { "Saving..." } else { "Save changes" }}
                    </button>
                    <a href=move || format!("/pool/{}", pool_id())>"Back to pool"</a>
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
