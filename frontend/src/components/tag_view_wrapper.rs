use leptos::prelude::*;

use crate::auth::AuthState;
use crate::settings::SettingsState;

/// Shared wrapper for all tag view pages (Summary, Edit, Merge, Delete).
/// Renders the tag name heading and tab navigation with the active tab highlighted.
#[component]
pub fn TagViewWrapper(
    #[prop(into)] tag_name: String,
    #[prop(into)] active_tab: String,
    children: Children,
) -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let settings = expect_context::<SettingsState>();

    let display_name = settings.display_name(&tag_name);

    let can_edit = Memo::new(move |_| {
        auth.has_privilege("tag_edit_name")
            || auth.has_privilege("tag_edit_category")
            || auth.has_privilege("tag_edit_description")
            || auth.has_privilege("tag_edit_implication")
            || auth.has_privilege("tag_edit_suggestion")
    });
    let can_merge = Memo::new(move |_| auth.has_privilege("tag_merge"));
    let can_delete = Memo::new(move |_| auth.has_privilege("tag_delete"));

    let summary_href = format!("/tag/{tag_name}");
    let edit_href = format!("/tag/{tag_name}/edit");
    let merge_href = format!("/tag/{tag_name}/merge");
    let delete_href = format!("/tag/{tag_name}/delete");

    let summary_active = active_tab == "summary";
    let edit_active = active_tab == "edit";
    let merge_active = active_tab == "merge";
    let delete_active = active_tab == "delete";

    view! {
        <div class="tag-view">
            <h1>{display_name}</h1>

            <nav class="buttons">
                <ul>
                    <li class:active=summary_active><a href=summary_href>"Summary"</a></li>
                    {move || can_edit.get().then(|| view! {
                        <li class:active=edit_active><a href=edit_href.clone()>"Edit"</a></li>
                    })}
                    {move || can_merge.get().then(|| view! {
                        <li class:active=merge_active><a href=merge_href.clone()>"Merge with\u{2026}"</a></li>
                    })}
                    {move || can_delete.get().then(|| view! {
                        <li class:active=delete_active><a href=delete_href.clone()>"Delete"</a></li>
                    })}
                </ul>
            </nav>

            {children()}
        </div>
    }
}
