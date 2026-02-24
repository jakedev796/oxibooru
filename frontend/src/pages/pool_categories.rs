use leptos::prelude::*;
use leptos_meta::Title;
use oxibooru_shared::category::PoolCategoryInfo;
use oxibooru_shared::request::DeleteBody;

use crate::api::categories::{CreatePoolCategoryBody, UpdatePoolCategoryBody};
use crate::api::{ApiClient, ApiError};

#[component]
pub fn PoolCategoriesPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();

    let categories = RwSignal::new(Vec::<PoolCategoryInfo>::new());
    let (loading, set_loading) = signal(true);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let (new_name, set_new_name) = signal(String::new());
    let (new_color, set_new_color) = signal("#aaaaaa".to_string());

    // Initial load
    Effect::new(move || {
        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            match client.get_pool_categories().await {
                Ok(resp) => {
                    categories.set(resp.results);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_loading.set(false);
                }
            }
        });
    });

    let on_create = move |_| {
        let name = new_name.get_untracked();
        if name.trim().is_empty() {
            return;
        }
        let body = CreatePoolCategoryBody {
            name: name.clone(),
            color: new_color.get_untracked(),
        };
        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            match client.create_pool_category(&body).await {
                Ok(_) => {
                    set_new_name.set(String::new());
                    set_error_msg.set(None);
                    if let Ok(resp) = client.get_pool_categories().await {
                        categories.set(resp.results);
                    }
                }
                Err(e) => set_error_msg.set(Some(format_api_error(&e))),
            }
        });
    };

    view! {
        <Title text="Pool Categories" />
        <div class="content-wrapper">
            <h1>"Pool Categories"</h1>
            {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}
            {move || loading.get().then(|| view! { <p>"Loading\u{2026}"</p> })}

            <table class="category-table" style:display=move || if loading.get() { "none" } else { "" }>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Color"</th>
                        <th>"Default"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || categories.get().into_iter().map(|cat| {
                        let name = cat.name.clone().unwrap_or_default();
                        let version = cat.version.clone().unwrap_or_default();
                        let color = RwSignal::new(cat.color.clone().unwrap_or_else(|| "#aaaaaa".into()));
                        let is_default = cat.default.unwrap_or(false);

                        let name_save = name.clone();
                        let ver_save = version.clone();
                        let save = move |_| {
                            let client = api.get_untracked();
                            let n = name_save.clone();
                            let v = ver_save.clone();
                            let c = color.get_untracked();
                            leptos::task::spawn_local(async move {
                                let body = UpdatePoolCategoryBody {
                                    version: v,
                                    name: None,
                                    color: Some(c),
                                };
                                match client.update_pool_category(&n, &body).await {
                                    Ok(_) => {
                                        if let Ok(resp) = client.get_pool_categories().await {
                                            categories.set(resp.results);
                                        }
                                    }
                                    Err(e) => set_error_msg.set(Some(format_api_error(&e))),
                                }
                            });
                        };

                        let name_del = name.clone();
                        let ver_del = version.clone();
                        let name_def = name.clone();
                        let swatch = cat.color.clone().unwrap_or_else(|| "#aaaaaa".into());

                        view! {
                            <tr>
                                <td>
                                    <span class="color-swatch" style=format!("background: {swatch}")></span>
                                    {name.clone()}
                                </td>
                                <td>
                                    <input
                                        type="color"
                                        prop:value=move || color.get()
                                        on:input=move |ev| color.set(event_target_value(&ev))
                                    />
                                </td>
                                <td>{if is_default { "\u{2713}" } else { "" }}</td>
                                <td>
                                    <button type="button" on:click=save>"Save"</button>
                                    {(!is_default).then(|| {
                                        let nd = name_def.clone();
                                        view! {
                                            <button type="button" on:click=move |_| {
                                                let client = api.get_untracked();
                                                let n = nd.clone();
                                                leptos::task::spawn_local(async move {
                                                    if client.set_default_pool_category(&n).await.is_ok() {
                                                        if let Ok(resp) = client.get_pool_categories().await {
                                                            categories.set(resp.results);
                                                        }
                                                    }
                                                });
                                            }>"Set default"</button>
                                        }
                                    })}
                                    {(!is_default).then(|| view! {
                                        <button type="button" class="btn-danger" on:click=move |_| {
                                            let client = api.get_untracked();
                                            let n = name_del.clone();
                                            let v = ver_del.clone();
                                            leptos::task::spawn_local(async move {
                                                if client.delete_pool_category(&n, &DeleteBody { version: v }).await.is_ok() {
                                                    if let Ok(resp) = client.get_pool_categories().await {
                                                        categories.set(resp.results);
                                                    }
                                                }
                                            });
                                        }>"Delete"</button>
                                    })}
                                </td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>

            <h2>"Add Category"</h2>
            <div class="form-grid">
                <div class="form-row">
                    <label>"Name"</label>
                    <input
                        type="text"
                        prop:value=move || new_name.get()
                        on:input=move |ev| set_new_name.set(event_target_value(&ev))
                        placeholder="category name"
                    />
                </div>
                <div class="form-row">
                    <label>"Color"</label>
                    <input
                        type="color"
                        prop:value=move || new_color.get()
                        on:input=move |ev| set_new_color.set(event_target_value(&ev))
                    />
                </div>
                <div class="form-row">
                    <button type="button" on:click=on_create>"Add category"</button>
                </div>
            </div>
        </div>
    }
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
