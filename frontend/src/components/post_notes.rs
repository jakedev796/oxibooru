use leptos::prelude::*;
use oxibooru_shared::post::Note;

use crate::components::markdown::render_markdown;

/// Read-only SVG overlay displaying post note polygons with hover tooltips.
#[component]
pub fn PostNotesOverlay(notes: Vec<Note>) -> impl IntoView {
    let hovered: RwSignal<Option<usize>> = RwSignal::new(None);

    // Pre-render markdown for each note
    let tooltip_html: Vec<String> = notes.iter().map(|n| render_markdown(&n.text)).collect();

    // Build polygon point strings
    let polygons: Vec<String> = notes
        .iter()
        .map(|note| {
            note.polygon
                .iter()
                .map(|[x, y]| format!("{x},{y}"))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();

    view! {
        <div class="post-notes-overlay">
            <svg viewBox="0 0 1 1" preserveAspectRatio="none">
                {polygons.into_iter().enumerate().map(|(idx, points)| {
                    view! {
                        <polygon
                            points=points
                            class="note-polygon"
                            class:hovered=move || hovered.get() == Some(idx)
                            on:mouseenter=move |_| hovered.set(Some(idx))
                            on:mouseleave=move |_| hovered.set(None)
                        />
                    }
                }).collect_view()}
            </svg>
            {move || hovered.get().and_then(|idx| tooltip_html.get(idx)).map(|html| {
                view! {
                    <div class="note-tooltip" inner_html=html.clone() />
                }
            })}
        </div>
    }
}
