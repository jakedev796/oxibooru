use std::cell::RefCell;
use std::rc::Rc;

use gloo_timers::callback::Timeout;
use leptos::prelude::*;
use oxibooru_shared::pool::PoolInfo;

use crate::api::ApiClient;

/// Chip-based multi-pool input with debounced autocomplete.
/// Each pool is represented as (id, display_name).
#[component]
pub fn PoolInput(pools: RwSignal<Vec<(i64, String)>>, #[prop(optional, into)] label: String) -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let (query, set_query) = signal(String::new());
    let (results, set_results) = signal(Vec::<PoolInfo>::new());
    let (selected_index, set_selected_index) = signal(Option::<usize>::None);
    let (show_dropdown, set_show_dropdown) = signal(false);

    let debounce_handle: Rc<RefCell<Option<Timeout>>> = Rc::new(RefCell::new(None));

    let dh = debounce_handle.clone();
    let on_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        set_query.set(val.clone());
        set_selected_index.set(None);

        dh.borrow_mut().take();

        if val.trim().is_empty() {
            set_results.set(vec![]);
            set_show_dropdown.set(false);
            return;
        }

        let dh2 = dh.clone();
        let timeout = Timeout::new(300, move || {
            dh2.borrow_mut().take();
            let client = api.get_untracked();
            let search_query = format!("*{val}*");
            leptos::task::spawn_local(async move {
                if let Ok(page) = client.get_pools(&search_query, 0, 15, "id,names,category").await {
                    set_results.set(page.results);
                    set_show_dropdown.set(true);
                }
            });
        });
        *dh.borrow_mut() = Some(timeout);
    };

    let do_select = move |pool: &PoolInfo| {
        let pool_id = pool.id.unwrap_or(0);
        let name = pool.names.as_ref().and_then(|n| n.first()).cloned().unwrap_or_default();
        pools.update(|list| {
            if !list.iter().any(|(id, _)| *id == pool_id) {
                list.push((pool_id, name));
            }
        });
        set_query.set(String::new());
        set_results.set(vec![]);
        set_show_dropdown.set(false);
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| match ev.key().as_str() {
        "ArrowDown" => {
            ev.prevent_default();
            let len = results.get_untracked().len();
            if len > 0 {
                set_selected_index.update(|idx| {
                    *idx = Some(match *idx {
                        Some(i) => (i + 1).min(len - 1),
                        None => 0,
                    });
                });
            }
        }
        "ArrowUp" => {
            ev.prevent_default();
            set_selected_index.update(|idx| {
                *idx = match *idx {
                    Some(0) | None => None,
                    Some(i) => Some(i - 1),
                };
            });
        }
        "Enter" => {
            ev.prevent_default();
            if let Some(i) = selected_index.get_untracked() {
                let items = results.get_untracked();
                if let Some(pool) = items.get(i) {
                    do_select(pool);
                }
            }
        }
        "Escape" => {
            set_show_dropdown.set(false);
        }
        _ => {}
    };

    let on_blur = move |_| {
        Timeout::new(200, move || {
            set_show_dropdown.set(false);
        })
        .forget();
    };

    let on_focus = move |_| {
        if !results.get_untracked().is_empty() {
            set_show_dropdown.set(true);
        }
    };

    view! {
        <div class="pool-input">
            {(!label.is_empty()).then(|| view! { <label>{label}</label> })}
            <div class="tag-chips">
                {move || pools.get().into_iter().map(|(id, name)| {
                    view! {
                        <span class="tag-chip">
                            {name}
                            <button type="button" class="chip-remove" on:click=move |_| {
                                pools.update(|list| list.retain(|(pid, _)| *pid != id));
                            }>
                                "\u{00D7}"
                            </button>
                        </span>
                    }
                }).collect_view()}
            </div>
            <div class="tag-autocomplete">
                <input
                    type="text"
                    prop:value=move || query.get()
                    on:input=on_input
                    on:keydown=on_keydown
                    on:blur=on_blur
                    on:focus=on_focus
                    placeholder="Add pool..."
                    autocomplete="off"
                />
                <Show when=move || show_dropdown.get() && !results.get().is_empty()>
                    <ul class="autocomplete-dropdown">
                        {move || results.get().into_iter().enumerate().map(|(i, pool)| {
                            let pool_id = pool.id.unwrap_or(0);
                            let name = pool.names.as_ref()
                                .and_then(|n| n.first())
                                .cloned()
                                .unwrap_or_default();
                            let category = pool.category.clone().unwrap_or_default();
                            let is_selected = move || selected_index.get() == Some(i);
                            let name_click = name.clone();
                            let class = format!("autocomplete-item pool-category-{category}");
                            view! {
                                <li
                                    class=class
                                    class:selected=is_selected
                                    on:mousedown=move |_| {
                                        pools.update(|list| {
                                            if !list.iter().any(|(id, _)| *id == pool_id) {
                                                list.push((pool_id, name_click.clone()));
                                            }
                                        });
                                        set_query.set(String::new());
                                        set_results.set(vec![]);
                                        set_show_dropdown.set(false);
                                    }
                                >
                                    <span class="pool-name">{name}</span>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                </Show>
            </div>
        </div>
    }
}
