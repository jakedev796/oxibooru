use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use oxibooru_shared::user_token::UserTokenInfo;

use crate::api::user_tokens::CreateUserTokenBody;
use crate::api::{ApiClient, ApiError};

#[component]
pub fn UserTokensPage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let params = use_params_map();

    let username = move || {
        params
            .get()
            .get("name")
            .unwrap_or_default()
    };

    // Tokens list
    let (tokens, set_tokens) = signal(Vec::<UserTokenInfo>::new());
    let (loading, set_loading) = signal(true);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    // Load tokens on mount
    Effect::new(move || {
        let client = api.get_untracked();
        let name = username();
        set_loading.set(true);
        leptos::task::spawn_local(async move {
            match client.get_user_tokens(&name).await {
                Ok(resp) => {
                    set_tokens.set(resp.results);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_loading.set(false);
                }
            }
        });
    });

    // Create token form
    let (new_note, set_new_note) = signal(String::new());
    let (creating, set_creating) = signal(false);

    let on_create = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_creating.set(true);
        set_error_msg.set(None);

        let name = username();
        let body = CreateUserTokenBody {
            enabled: Some(true),
            note: {
                let n = new_note.get_untracked();
                if n.is_empty() { None } else { Some(n) }
            },
            expiration_time: None,
        };

        let client = api.get_untracked();
        leptos::task::spawn_local(async move {
            match client.create_user_token(&name, &body).await {
                Ok(token) => {
                    set_tokens.update(|list| list.push(token));
                    set_new_note.set(String::new());
                    set_creating.set(false);
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                    set_creating.set(false);
                }
            }
        });
    };

    view! {
        <Title text=move || format!("Tokens — {}", username()) />
        <div class="content-wrapper user-tokens">
            <h1>{move || format!("API tokens — {}", username())}</h1>

            {move || error_msg.get().map(|msg| view! {
                <div class="messages">
                    <div class="message error">{msg}</div>
                </div>
            })}

            {move || {
                if loading.get() {
                    view! { <p>"Loading…"</p> }.into_any()
                } else {
                    view! {
                        <table class="token-list">
                            <thead>
                                <tr>
                                    <th>"Token"</th>
                                    <th>"Note"</th>
                                    <th>"Enabled"</th>
                                    <th>"Created"</th>
                                    <th>"Expires"</th>
                                    <th>"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <For
                                    each=move || tokens.get()
                                    key=|t| t.token.clone().unwrap_or_default()
                                    let:token
                                >
                                    <TokenRow token=token username=username() api=api set_tokens=set_tokens set_error_msg=set_error_msg />
                                </For>
                            </tbody>
                        </table>
                    }.into_any()
                }
            }}

            <h2>"Create new token"</h2>
            <form on:submit=on_create>
                <ul class="input">
                    <li>
                        <label for="token-note">"Note (optional)"</label>
                        <input
                            type="text"
                            id="token-note"
                            prop:value=move || new_note.get()
                            on:input=move |ev| set_new_note.set(event_target_value(&ev))
                            disabled=move || creating.get()
                        />
                    </li>
                </ul>
                <div class="buttons">
                    <input type="submit" value="Create token" disabled=move || creating.get() />
                </div>
            </form>
        </div>
    }
}

#[component]
fn TokenRow(
    token: UserTokenInfo,
    username: String,
    api: RwSignal<ApiClient>,
    set_tokens: WriteSignal<Vec<UserTokenInfo>>,
    set_error_msg: WriteSignal<Option<String>>,
) -> impl IntoView {
    let token_value = token.token.clone().unwrap_or_default();
    let note = token.note.clone().unwrap_or_default();
    let enabled = token.enabled.unwrap_or(false);
    let creation_time = token.creation_time.clone().unwrap_or_default();
    let expiration_time = token
        .expiration_time
        .flatten()
        .unwrap_or_else(|| "Never".to_string());
    let version = token.version.clone().unwrap_or_default();

    let token_id = token_value.clone();
    let token_id_for_delete = token_value.clone();
    let username_for_toggle = username.clone();
    let username_for_delete = username.clone();

    let on_toggle = move |_: ev::MouseEvent| {
        let client = api.get_untracked();
        let name = username_for_toggle.clone();
        let tid = token_id.clone();
        let ver = version.clone();
        let new_enabled = !enabled;
        let body = crate::api::user_tokens::UpdateUserTokenBody {
            version: ver,
            enabled: Some(new_enabled),
            note: None,
            expiration_time: None,
        };
        leptos::task::spawn_local(async move {
            match client.update_user_token(&name, &tid, &body).await {
                Ok(updated) => {
                    set_tokens.update(|list| {
                        if let Some(t) = list.iter_mut().find(|t| t.token == updated.token) {
                            *t = updated;
                        }
                    });
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                }
            }
        });
    };

    let on_delete = move |_: ev::MouseEvent| {
        let client = api.get_untracked();
        let name = username_for_delete.clone();
        let tid = token_id_for_delete.clone();
        leptos::task::spawn_local(async move {
            match client.delete_user_token(&name, &tid).await {
                Ok(()) => {
                    set_tokens.update(|list| {
                        list.retain(|t| t.token.as_deref() != Some(&tid));
                    });
                }
                Err(e) => {
                    set_error_msg.set(Some(format_api_error(&e)));
                }
            }
        });
    };

    view! {
        <tr>
            <td class="token-value"><code>{token_value.clone()}</code></td>
            <td>{note}</td>
            <td>
                <button type="button" on:click=on_toggle>
                    {if enabled { "Enabled" } else { "Disabled" }}
                </button>
            </td>
            <td>{creation_time}</td>
            <td>{expiration_time}</td>
            <td>
                <button type="button" class="dangerous" on:click=on_delete>"Delete"</button>
            </td>
        </tr>
    }
}

fn format_api_error(e: &ApiError) -> String {
    match e {
        ApiError::Server(resp) => resp.description.clone(),
        ApiError::Network(msg) => msg.clone(),
    }
}
