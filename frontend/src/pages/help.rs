use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

#[component]
pub fn HelpPage() -> impl IntoView {
    let params = use_params_map();

    let section = Memo::new(move |_| params.get().get("section").unwrap_or_else(|| "about".to_string()));
    let subsection = Memo::new(move |_| params.get().get("subsection").unwrap_or_default());

    view! {
        <Title text="Help" />
        <div class="help-page">
            <nav class="help-nav">
                <ul>
                    <li class=move || if section.get() == "about" { "active" } else { "" }>
                        <a href="/help/about">"About"</a>
                    </li>
                    <li class=move || if section.get() == "keyboard" { "active" } else { "" }>
                        <a href="/help/keyboard">"Keyboard"</a>
                    </li>
                    <li class=move || if section.get() == "search" { "active" } else { "" }>
                        <a href="/help/search">"Search syntax"</a>
                    </li>
                    <li class=move || if section.get() == "comments" { "active" } else { "" }>
                        <a href="/help/comments">"Comments"</a>
                    </li>
                    <li class=move || if section.get() == "tos" { "active" } else { "" }>
                        <a href="/help/tos">"Terms of Service"</a>
                    </li>
                </ul>
            </nav>
            <div class="help-content">
                {move || {
                    let sec = section.get();
                    let sub = subsection.get();
                    match sec.as_str() {
                        "keyboard" => help_keyboard().into_any(),
                        "search" => help_search(&sub).into_any(),
                        "comments" => help_comments().into_any(),
                        "tos" => help_tos().into_any(),
                        _ => help_about().into_any(),
                    }
                }}
            </div>
        </div>
    }
}

fn help_about() -> impl IntoView {
    view! {
        <section>
            <h1>"About"</h1>
            <p>"Oxibooru is an image board engine, forked from szurubooru and rewritten in Rust."</p>
            <h2>"Registration"</h2>
            <p>"New users can register by clicking the 'Register' link. After registration, you can browse, upload posts, add tags, and comment."</p>
            <h2>"User ranks"</h2>
            <dl>
                <dt>"Anonymous"</dt><dd>"Can only view safe posts."</dd>
                <dt>"Restricted"</dt><dd>"Can view and upload posts."</dd>
                <dt>"Regular"</dt><dd>"Standard user with full browsing and basic editing privileges."</dd>
                <dt>"Power"</dt><dd>"Can edit tags, pools, and moderate comments."</dd>
                <dt>"Moderator"</dt><dd>"Can delete posts and manage users."</dd>
                <dt>"Administrator"</dt><dd>"Full control over the site."</dd>
            </dl>
        </section>
    }
}

fn help_keyboard() -> impl IntoView {
    view! {
        <section>
            <h1>"Keyboard shortcuts"</h1>
            <table>
                <thead>
                    <tr><th>"Key"</th><th>"Action"</th></tr>
                </thead>
                <tbody>
                    <tr><td><kbd>"Q"</kbd></td><td>"Focus search field"</td></tr>
                    <tr><td><kbd>"A"</kbd></td><td>"Go to previous post"</td></tr>
                    <tr><td><kbd>"D"</kbd></td><td>"Go to next post"</td></tr>
                    <tr><td><kbd>"F"</kbd></td><td>"Toggle favorite on current post"</td></tr>
                    <tr><td><kbd>"E"</kbd></td><td>"Edit current post"</td></tr>
                    <tr><td><kbd>"P"</kbd></td><td>"Cycle post fit mode"</td></tr>
                    <tr><td><kbd>"T"</kbd></td><td>"Focus tag input (on edit page)"</td></tr>
                    <tr><td><kbd>"Ctrl+S"</kbd></td><td>"Save changes (on edit page)"</td></tr>
                    <tr><td><kbd>"Delete"</kbd></td><td>"Delete post (on post page, if privileged)"</td></tr>
                </tbody>
            </table>
        </section>
    }
}

fn help_search(subsection: &str) -> impl IntoView {
    let content = match subsection {
        "posts" => help_search_posts().into_any(),
        "tags" => help_search_tags().into_any(),
        "users" => help_search_users().into_any(),
        "pools" => help_search_pools().into_any(),
        _ => help_search_general().into_any(),
    };

    view! {
        <nav class="help-subnav">
            <ul>
                <li><a href="/help/search">"General"</a></li>
                <li><a href="/help/search/posts">"Posts"</a></li>
                <li><a href="/help/search/tags">"Tags"</a></li>
                <li><a href="/help/search/users">"Users"</a></li>
                <li><a href="/help/search/pools">"Pools"</a></li>
            </ul>
        </nav>
        {content}
    }
}

fn help_search_general() -> impl IntoView {
    view! {
        <section>
            <h1>"Search syntax"</h1>
            <p>"Searches consist of space-separated tokens. Each token can be:"</p>
            <table>
                <thead>
                    <tr><th>"Type"</th><th>"Example"</th><th>"Description"</th></tr>
                </thead>
                <tbody>
                    <tr><td>"Anonymous"</td><td><code>"landscape"</code></td><td>"Searches tags matching the value"</td></tr>
                    <tr><td>"Named"</td><td><code>"score:5"</code></td><td>"Filters by the named field"</td></tr>
                    <tr><td>"Sort"</td><td><code>"sort:score"</code></td><td>"Sorts results by the given field"</td></tr>
                    <tr><td>"Special"</td><td><code>"liked"</code></td><td>"Pre-defined special filters"</td></tr>
                </tbody>
            </table>
            <h2>"Ranges"</h2>
            <p>"Named tokens support range values:"</p>
            <ul>
                <li><code>"value"</code>" \u{2014} exact match"</li>
                <li><code>"a,b,c"</code>" \u{2014} match any of these values"</li>
                <li><code>"1.."</code>" \u{2014} greater than or equal to 1"</li>
                <li><code>"..4"</code>" \u{2014} less than or equal to 4"</li>
                <li><code>"1..4"</code>" \u{2014} between 1 and 4 (inclusive)"</li>
            </ul>
            <h2>"Dates"</h2>
            <p>"Date values can be:"</p>
            <ul>
                <li><code>"today"</code>", "<code>"yesterday"</code></li>
                <li><code>"2024"</code>" \u{2014} a full year"</li>
                <li><code>"2024-01"</code>" \u{2014} a month"</li>
                <li><code>"2024-01-15"</code>" \u{2014} a specific day"</li>
            </ul>
            <h2>"Negation"</h2>
            <p>"Prefix any token with "<code>"-"</code>" to negate it. For example, "<code>"-tag:landscape"</code>" excludes posts tagged \"landscape\"."</p>
            <h2>"Wildcards"</h2>
            <p>"Use "<code>"*"</code>" in anonymous tokens for wildcard matching. For example, "<code>"land*"</code>" matches \"landscape\", \"landing\", etc."</p>
        </section>
    }
}

fn help_search_posts() -> impl IntoView {
    view! {
        <section>
            <h1>"Post search tokens"</h1>
            <table>
                <thead>
                    <tr><th>"Token"</th><th>"Description"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"tag:NAME"</code></td><td>"Posts with the given tag"</td></tr>
                    <tr><td><code>"score:N"</code></td><td>"Post score"</td></tr>
                    <tr><td><code>"uploader:NAME"</code></td><td>"Posts uploaded by the given user"</td></tr>
                    <tr><td><code>"comment:TEXT"</code></td><td>"Posts with comments containing text"</td></tr>
                    <tr><td><code>"fav:NAME"</code></td><td>"Posts favorited by the given user"</td></tr>
                    <tr><td><code>"pool:ID"</code></td><td>"Posts in the given pool"</td></tr>
                    <tr><td><code>"safety:RATING"</code></td><td>"safe, sketchy, or unsafe"</td></tr>
                    <tr><td><code>"type:TYPE"</code></td><td>"image, animation, video, or flash"</td></tr>
                    <tr><td><code>"file-size:N"</code></td><td>"File size in bytes"</td></tr>
                    <tr><td><code>"image-width:N"</code></td><td>"Canvas width"</td></tr>
                    <tr><td><code>"image-height:N"</code></td><td>"Canvas height"</td></tr>
                    <tr><td><code>"image-area:N"</code></td><td>"Canvas area (width * height)"</td></tr>
                    <tr><td><code>"aspect-ratio:W:H"</code></td><td>"Aspect ratio (e.g. 16:9)"</td></tr>
                    <tr><td><code>"creation-date:DATE"</code></td><td>"Upload date"</td></tr>
                    <tr><td><code>"edit-date:DATE"</code></td><td>"Last edit date"</td></tr>
                    <tr><td><code>"comment-date:DATE"</code></td><td>"Date of latest comment"</td></tr>
                    <tr><td><code>"comment-count:N"</code></td><td>"Number of comments"</td></tr>
                    <tr><td><code>"fav-count:N"</code></td><td>"Number of favorites"</td></tr>
                    <tr><td><code>"note-count:N"</code></td><td>"Number of notes"</td></tr>
                    <tr><td><code>"tag-count:N"</code></td><td>"Number of tags"</td></tr>
                    <tr><td><code>"relation-count:N"</code></td><td>"Number of related posts"</td></tr>
                    <tr><td><code>"feature-count:N"</code></td><td>"Number of times featured"</td></tr>
                    <tr><td><code>"feature-date:DATE"</code></td><td>"Date of last feature"</td></tr>
                </tbody>
            </table>
            <h2>"Sort styles"</h2>
            <p>"Use "<code>"sort:FIELD"</code>" where FIELD is: random, id, score, tag-count, comment-count, fav-count, note-count, relation-count, feature-count, file-size, image-width, image-height, image-area, creation-date, edit-date, comment-date, fav-date, feature-date."</p>
            <h2>"Special tokens"</h2>
            <p><code>"liked"</code>", "<code>"disliked"</code>", "<code>"fav"</code>" \u{2014} filter to own liked/disliked/favorited posts."</p>
        </section>
    }
}

fn help_search_tags() -> impl IntoView {
    view! {
        <section>
            <h1>"Tag search tokens"</h1>
            <table>
                <thead>
                    <tr><th>"Token"</th><th>"Description"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"name:TEXT"</code></td><td>"Tag name (supports wildcards)"</td></tr>
                    <tr><td><code>"category:NAME"</code></td><td>"Tag category"</td></tr>
                    <tr><td><code>"creation-date:DATE"</code></td><td>"Creation date"</td></tr>
                    <tr><td><code>"edit-date:DATE"</code></td><td>"Last edit date"</td></tr>
                    <tr><td><code>"usages:N"</code></td><td>"Number of posts using the tag"</td></tr>
                    <tr><td><code>"suggestion-count:N"</code></td><td>"Number of suggestions"</td></tr>
                    <tr><td><code>"implication-count:N"</code></td><td>"Number of implications"</td></tr>
                </tbody>
            </table>
            <h2>"Sort styles"</h2>
            <p>"name, category, creation-date, edit-date, usages, suggestion-count, implication-count."</p>
        </section>
    }
}

fn help_search_users() -> impl IntoView {
    view! {
        <section>
            <h1>"User search tokens"</h1>
            <table>
                <thead>
                    <tr><th>"Token"</th><th>"Description"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"name:TEXT"</code></td><td>"Username (supports wildcards)"</td></tr>
                    <tr><td><code>"creation-date:DATE"</code></td><td>"Registration date"</td></tr>
                    <tr><td><code>"last-login-date:DATE"</code></td><td>"Last login date"</td></tr>
                </tbody>
            </table>
            <h2>"Sort styles"</h2>
            <p>"name, creation-date, last-login-date."</p>
        </section>
    }
}

fn help_search_pools() -> impl IntoView {
    view! {
        <section>
            <h1>"Pool search tokens"</h1>
            <table>
                <thead>
                    <tr><th>"Token"</th><th>"Description"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"name:TEXT"</code></td><td>"Pool name (supports wildcards)"</td></tr>
                    <tr><td><code>"category:NAME"</code></td><td>"Pool category"</td></tr>
                    <tr><td><code>"creation-date:DATE"</code></td><td>"Creation date"</td></tr>
                    <tr><td><code>"post-count:N"</code></td><td>"Number of posts in pool"</td></tr>
                </tbody>
            </table>
            <h2>"Sort styles"</h2>
            <p>"name, category, creation-date, post-count."</p>
        </section>
    }
}

fn help_comments() -> impl IntoView {
    view! {
        <section>
            <h1>"Comment formatting"</h1>
            <p>"Comments support Markdown syntax with some extensions."</p>
            <h2>"Standard Markdown"</h2>
            <ul>
                <li><code>"**bold**"</code>" \u{2192} "<strong>"bold"</strong></li>
                <li><code>"*italic*"</code>" \u{2192} "<em>"italic"</em></li>
                <li><code>"~~strikethrough~~"</code>" \u{2192} strikethrough"</li>
                <li><code>"[link text](url)"</code>" \u{2192} link"</li>
                <li><code>"![alt text](image url)"</code>" \u{2192} embedded image"</li>
            </ul>
            <h2>"Special syntax"</h2>
            <table>
                <thead>
                    <tr><th>"Syntax"</th><th>"Result"</th></tr>
                </thead>
                <tbody>
                    <tr><td><code>"@123"</code></td><td>"Link to post #123"</td></tr>
                    <tr><td><code>"#tag_name"</code></td><td>"Link to tag"</td></tr>
                    <tr><td><code>"+username"</code></td><td>"Link to user"</td></tr>
                    <tr><td><code>"[spoiler]text[/spoiler]"</code></td><td>"Hidden spoiler text"</td></tr>
                </tbody>
            </table>
        </section>
    }
}

fn help_tos() -> impl IntoView {
    view! {
        <section>
            <h1>"Terms of Service"</h1>
            <p>"By using this site, you agree to the following:"</p>
            <ul>
                <li>"You must comply with all applicable laws."</li>
                <li>"Content you upload must not violate the rights of others."</li>
                <li>"The site administrators reserve the right to remove any content."</li>
                <li>"Accounts may be suspended or terminated at the discretion of the administrators."</li>
            </ul>
            <h2>"Privacy Policy"</h2>
            <p>"We collect minimal data necessary for site operation:"</p>
            <ul>
                <li>"Account information (username, email if provided)"</li>
                <li>"Upload history and activity logs"</li>
                <li>"Standard server access logs"</li>
            </ul>
            <p>"We do not sell or share your personal information with third parties."</p>
        </section>
    }
}
