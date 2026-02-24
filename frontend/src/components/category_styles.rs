use leptos::prelude::*;
use oxibooru_shared::category::{PoolCategoryInfo, TagCategoryInfo};

/// Renders a dynamic `<style>` element with CSS rules for tag and pool category colors.
/// Category colors are user-configurable, so they cannot be hardcoded in the stylesheet.
#[component]
pub fn CategoryStyles() -> impl IntoView {
    let tag_categories = expect_context::<RwSignal<Vec<TagCategoryInfo>>>();
    let pool_categories = expect_context::<RwSignal<Vec<PoolCategoryInfo>>>();

    let css = Memo::new(move |_| {
        let mut rules = String::new();
        for cat in tag_categories.get() {
            if let (Some(name), Some(color)) = (&cat.name, &cat.color) {
                let color = normalize_color(color);
                rules.push_str(&format!(".tag-category-{name} {{ color: {color}; }}\n"));
            }
        }
        for cat in pool_categories.get() {
            if let (Some(name), Some(color)) = (&cat.name, &cat.color) {
                let color = normalize_color(color);
                rules.push_str(&format!(".pool-category-{name} {{ color: {color}; }}\n"));
            }
        }
        rules
    });

    view! {
        <style>{move || css.get()}</style>
    }
}

/// Ensure the color value is a valid CSS color.
/// If it looks like a bare hex value (e.g. "ff0000"), prepend '#'.
fn normalize_color(color: &str) -> String {
    let trimmed = color.trim();
    if trimmed.is_empty() {
        return "inherit".to_string();
    }
    // Already has # or is a named color / rgb() / etc.
    if trimmed.starts_with('#') || trimmed.starts_with("rgb") || trimmed.contains('(') {
        return trimmed.to_string();
    }
    // Looks like a bare hex value
    if (trimmed.len() == 3 || trimmed.len() == 6) && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return format!("#{trimmed}");
    }
    trimmed.to_string()
}
