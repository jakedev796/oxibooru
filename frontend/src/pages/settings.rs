use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;

use crate::settings::SettingsState;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let settings_state = expect_context::<SettingsState>();
    let settings = settings_state.inner;

    let (saved, set_saved) = signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        settings.with_untracked(|s| s.save());
        set_saved.set(true);
    };

    view! {
        <Title text="Settings" />
        <div class="content-wrapper settings">
            <h1>"Browsing settings"</h1>
            <form on:submit=on_submit>
                <ul class="input">
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.dark_theme)
                                on:change=move |_| {
                                    settings_state.update(|s| s.dark_theme = !s.dark_theme);
                                    set_saved.set(false);
                                }
                            />
                            "Use dark theme"
                        </label>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.upscale_small_posts)
                                on:change=move |_| {
                                    settings_state.update(|s| s.upscale_small_posts = !s.upscale_small_posts);
                                    set_saved.set(false);
                                }
                            />
                            "Upscale small posts"
                        </label>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.endless_scroll)
                                on:change=move |_| {
                                    settings_state.update(|s| s.endless_scroll = !s.endless_scroll);
                                    set_saved.set(false);
                                }
                            />
                            "Enable endless scroll"
                        </label>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.keyboard_shortcuts)
                                on:change=move |_| {
                                    settings_state.update(|s| s.keyboard_shortcuts = !s.keyboard_shortcuts);
                                    set_saved.set(false);
                                }
                            />
                            "Enable keyboard shortcuts"
                        </label>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.tag_suggestions_enabled)
                                on:change=move |_| {
                                    settings_state.update(|s| s.tag_suggestions_enabled = !s.tag_suggestions_enabled);
                                    set_saved.set(false);
                                }
                            />
                            "Show tag suggestions"
                        </label>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.tag_underscores)
                                on:change=move |_| {
                                    settings_state.update(|s| s.tag_underscores = !s.tag_underscores);
                                    set_saved.set(false);
                                }
                            />
                            "Display underscores as spaces"
                        </label>
                    </li>
                    <li class="safety-settings">
                        <span>"Content visibility:"</span>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.list_posts_safe)
                                on:change=move |_| {
                                    settings_state.update(|s| s.list_posts_safe = !s.list_posts_safe);
                                    set_saved.set(false);
                                }
                            />
                            "Safe"
                        </label>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.list_posts_sketchy)
                                on:change=move |_| {
                                    settings_state.update(|s| s.list_posts_sketchy = !s.list_posts_sketchy);
                                    set_saved.set(false);
                                }
                            />
                            "Sketchy"
                        </label>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || settings.with(|s| s.list_posts_unsafe)
                                on:change=move |_| {
                                    settings_state.update(|s| s.list_posts_unsafe = !s.list_posts_unsafe);
                                    set_saved.set(false);
                                }
                            />
                            "Unsafe"
                        </label>
                    </li>
                    <li>
                        <label>
                            "Posts per page "
                            <input
                                type="number"
                                min="10"
                                max="100"
                                prop:value=move || settings.with(|s| s.posts_per_page.to_string())
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<u32>() {
                                        if (10..=100).contains(&val) {
                                            settings_state.update(|s| s.posts_per_page = val);
                                            set_saved.set(false);
                                        }
                                    }
                                }
                            />
                        </label>
                    </li>
                    <li>
                        <label>
                            "Post fit mode "
                            <select
                                prop:value=move || settings.with(|s| s.fit_mode.clone())
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    settings_state.update(|s| s.fit_mode = val);
                                    set_saved.set(false);
                                }
                            >
                                <option value="fit-both">"Fit both"</option>
                                <option value="fit-width">"Fit width"</option>
                                <option value="fit-height">"Fit height"</option>
                                <option value="original">"Original size"</option>
                            </select>
                        </label>
                    </li>
                </ul>
                <div class="messages">
                    {move || saved.get().then(|| view! {
                        <div class="message success">"Settings saved."</div>
                    })}
                </div>
                <input type="submit" value="Save settings" />
            </form>
        </div>
    }
}
