use leptos::ev;
use leptos::prelude::*;
use oxibooru_shared::comment::CommentInfo;

use crate::api::comments::{CreateCommentBody, UpdateCommentBody};
use crate::api::ApiClient;

#[component]
pub fn CommentForm(
    post_id: i64,
    on_submit: Callback<CommentInfo>,
    #[prop(optional)]
    edit_comment_id: Option<i64>,
    #[prop(optional)]
    edit_comment_version: Option<String>,
    #[prop(optional, into)]
    initial_text: String,
    #[prop(optional)]
    on_cancel: Option<Callback<()>>,
) -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let (text, set_text) = signal(initial_text);
    let (submitting, set_submitting) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let body_text = text.get_untracked();
        if body_text.trim().is_empty() {
            return;
        }
        set_submitting.set(true);
        set_error_msg.set(None);
        let client = api.get_untracked();
        let edit_id = edit_comment_id;
        let edit_ver = edit_comment_version.clone();

        leptos::task::spawn_local(async move {
            let result = if let (Some(cid), Some(ver)) = (edit_id, edit_ver) {
                client
                    .update_comment(
                        cid,
                        &UpdateCommentBody {
                            version: ver,
                            text: body_text,
                        },
                    )
                    .await
            } else {
                client
                    .create_comment(&CreateCommentBody {
                        post_id,
                        text: body_text,
                    })
                    .await
            };
            set_submitting.set(false);
            match result {
                Ok(comment) => {
                    set_text.set(String::new());
                    on_submit.run(comment);
                }
                Err(e) => set_error_msg.set(Some(e.to_string())),
            }
        });
    };

    view! {
        <form class="comment-form" on:submit=handle_submit>
            {move || error_msg.get().map(|msg| view! { <p class="error">{msg}</p> })}
            <textarea
                prop:value=move || text.get()
                on:input=move |ev| set_text.set(event_target_value(&ev))
                placeholder="Write a comment..."
                rows=4
            />
            <div class="comment-form-actions">
                <button type="submit" disabled=move || submitting.get()>
                    {if edit_comment_id.is_some() { "Save" } else { "Submit" }}
                </button>
                {on_cancel.map(|cancel| view! {
                    <button type="button" on:click=move |_| cancel.run(())>"Cancel"</button>
                })}
            </div>
        </form>
    }
}
