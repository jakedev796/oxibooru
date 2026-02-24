use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="404 — Not Found" />
        <div class="not-found-page">
            <h1>"404"</h1>
            <p>"The requested page was not found."</p>
            <p><a href="/">"Return to home page"</a></p>
        </div>
    }
}
