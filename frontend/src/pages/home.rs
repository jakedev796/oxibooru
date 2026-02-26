use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;

use crate::api::ApiClient;
use crate::auth::AuthState;
use crate::components::api_error::ApiErrorMessage;
use crate::components::loading_bar::LoadingState;
use crate::components::post_content::PostContent;
use crate::components::search_bar::SearchBar;
use crate::utils::{format_file_size, format_relative_time};

#[component]
pub fn HomePage() -> impl IntoView {
    let api = expect_context::<RwSignal<ApiClient>>();
    let loading = expect_context::<LoadingState>();
    let auth = expect_context::<AuthState>();

    let info = LocalResource::new(move || {
        let client = api.get();
        async move {
            loading.start();
            let result = client.get_info().await;
            loading.finish();
            result
        }
    });

    let query_signal = Signal::derive(|| String::new());

    let on_search = Callback::new(move |query: String| {
        let navigate = use_navigate();
        if query.is_empty() {
            navigate("/posts", Default::default());
        } else {
            let encoded = js_sys::encode_uri_component(&query);
            navigate(&format!("/posts?query={encoded}&limit=42"), Default::default());
        }
    });

    view! {
        <Title text="Home" />
        <div class="content-wrapper transparent-container" id="home">
            <header>
                <Suspense fallback=|| ()>
                    {move || Suspend::new(async move {
                        info.await.ok().map(|i| view! { <h1>{i.config.name.clone()}</h1> })
                    })}
                </Suspense>
            </header>

            <Show when=move || auth.has_privilege("post_list")>
                <div class="home-search">
                    <SearchBar
                        query=query_signal
                        on_submit=on_search
                        tag_autocomplete=true
                        placeholder="enter some tags"
                        form_class="horizontal".to_string()
                    />
                    <span class="sep">"or"</span>
                    <a href="/posts">"browse all posts"</a>
                </div>
            </Show>

            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                {move || Suspend::new(async move {
                    match info.await {
                        Ok(info) => {
                            let post_count = info.post_count;
                            let disk_usage = format_file_size(info.disk_usage);
                            let featured_post = info.featured_post.clone();

                            view! {
                                <div class="post-info-container">
                                    {featured_post.as_ref().map(|post| {
                                        let content_url = post.content_url.clone().unwrap_or_default();
                                        let post_type = post.type_.unwrap_or(oxibooru_shared::enums::PostType::Image);
                                        let post_id = post.id.unwrap_or(0);
                                        let creation_time = post.creation_time.clone().unwrap_or_default();
                                        let user = post.user.clone().flatten();
                                        let flags = post.flags.clone().unwrap_or_default();

                                        view! {
                                            <div class="post-container">
                                                <a href=format!("/post/{post_id}")>
                                                    <PostContent
                                                        content_url=content_url
                                                        post_type=post_type
                                                        fit_mode=Signal::derive(|| "fit-both".to_string())
                                                        flags=flags
                                                    />
                                                </a>
                                            </div>
                                            <aside>
                                                "Featured post: "
                                                <a href=format!("/post/{post_id}")>
                                                    {format!("#{post_id}")}
                                                </a>
                                                ", posted "
                                                {format_relative_time(&creation_time)}
                                                {user.map(|u| {
                                                    let user_name = u.name.clone();
                                                    let user_href = format!("/user/{user_name}");
                                                    view! {
                                                        " by "
                                                        <a href=user_href>{user_name}</a>
                                                    }
                                                })}
                                            </aside>
                                        }
                                    })}
                                </div>

                                <footer class="footer-container">
                                    <ul>
                                        <li>{post_count}" posts"</li>
                                        <span class="sep">" "</span>
                                        <li>{disk_usage}</li>
                                        <span class="sep">" "</span>
                                        <li>"Build "
                                            <a class="version" href="https://github.com/liamw1/oxibooru/commits/master">
                                                {env!("GIT_VERSION")}
                                            </a>
                                            " from "
                                            {format_relative_time(env!("BUILD_DATE"))}
                                        </li>
                                        {auth.has_privilege("snapshot_list").then(|| view! {
                                            <span class="sep">" "</span>
                                            <li><a href="/history">"History"</a></li>
                                        })}
                                    </ul>
                                </footer>
                            }.into_any()
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
