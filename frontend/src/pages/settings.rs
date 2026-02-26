use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::settings::SettingsState;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let settings_state = expect_context::<SettingsState>();

    // Local draft
    let draft = RwSignal::new(settings_state.inner.get_untracked());
    let (saved, set_saved) = signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let snapshot = draft.get_untracked();
        settings_state.inner.set(snapshot.clone());
        snapshot.save();
        set_saved.set(true);
    };

    view! {
        <Title text="Settings" />
        <div class="content-wrapper settings">
            <h1>"Browsing settings"</h1>
            <p class="settings-description">
                "These settings are saved to the browser\u{2019}s local storage and are not coupled to the user account, so they don\u{2019}t apply to other devices or browsers alike."
            </p>
            <form on:submit=on_submit>
                <ul class="input">
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.keyboard_shortcuts)
                                on:change=move |_| {
                                    draft.update(|s| s.keyboard_shortcuts = !s.keyboard_shortcuts);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Enable keyboard shortcuts"</span>
                        </label>
                        <span class="append"><A href="/help/keyboard"><i class="fa fa-question-circle-o" /></A></span>
                    </li>
                    <li>
                        <label>"Number of posts per page"</label>
                        <input
                            type="number"
                            min="10"
                            max="100"
                            prop:value=move || draft.with(|s| s.posts_per_page.to_string())
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<u32>() {
                                    if (10..=100).contains(&val) {
                                        draft.update(|s| s.posts_per_page = val);
                                        set_saved.set(false);
                                    }
                                }
                            }
                        />
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.dark_theme)
                                on:change=move |_| {
                                    draft.update(|s| s.dark_theme = !s.dark_theme);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Use dark theme"</span>
                        </label>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.upscale_small_posts)
                                on:change=move |_| {
                                    draft.update(|s| s.upscale_small_posts = !s.upscale_small_posts);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Upscale small posts"</span>
                        </label>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.endless_scroll)
                                on:change=move |_| {
                                    draft.update(|s| s.endless_scroll = !s.endless_scroll);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Enable endless scroll"</span>
                        </label>
                        <span class="hint">"Rather than using a paged navigation, smoothly scrolls through the content."</span>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.post_flow)
                                on:change=move |_| {
                                    draft.update(|s| s.post_flow = !s.post_flow);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Use post flow"</span>
                        </label>
                        <span class="hint">"Use a content-aware flow for thumbnails on the post search page."</span>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.transparency_grid)
                                on:change=move |_| {
                                    draft.update(|s| s.transparency_grid = !s.transparency_grid);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Enable transparency grid"</span>
                        </label>
                        <span class="hint">"Renders a checkered pattern behind posts with transparent background."</span>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.tag_suggestions_enabled)
                                on:change=move |_| {
                                    draft.update(|s| s.tag_suggestions_enabled = !s.tag_suggestions_enabled);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Show tag suggestions"</span>
                        </label>
                        <span class="hint">"Shows a popup with suggested tags in edit forms."</span>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.autoplay_videos)
                                on:change=move |_| {
                                    draft.update(|s| s.autoplay_videos = !s.autoplay_videos);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Automatically play video posts"</span>
                        </label>
                    </li>
                    <li>
                        <label>
                            <input
                                type="checkbox"
                                prop:checked=move || draft.with(|s| s.tag_underscores)
                                on:change=move |_| {
                                    draft.update(|s| s.tag_underscores = !s.tag_underscores);
                                    set_saved.set(false);
                                }
                            />
                            <span class="checkbox">"Display underscores as spaces"</span>
                        </label>
                        <span class="hint">"Display all underscores as if they were spaces. This is only a visual change, which means that you\u{2019}ll still have to use underscores when searching or editing tags."</span>
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
