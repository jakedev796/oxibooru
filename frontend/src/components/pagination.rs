use leptos::prelude::*;

/// Manual pagination component that renders a list of page numbers and chevron icons for prev/next.
#[component]
pub fn Pagination(
    offset: i64,
    limit: i64,
    total: i64,
    #[prop(into)] href_for_page: Callback<(i64, i64), String>,
) -> impl IntoView {
    if total == 0 || limit == 0 {
        return ().into_any();
    }

    let current_page = offset / limit + 1;
    let total_pages = (total + limit - 1) / limit;

    if total_pages <= 1 {
        return ().into_any();
    }

    // Compute visible page numbers using threshold algorithm:
    // Always show first 2, last 2, and current ± 2. Insert ellipsis for gaps.
    let pages = visible_pages(current_page, total_pages);

    let prev_href = if current_page > 1 {
        Some(href_for_page.run(((current_page - 2) * limit, limit)))
    } else {
        None
    };
    let next_href = if current_page < total_pages {
        Some(href_for_page.run((current_page * limit, limit)))
    } else {
        None
    };

    view! {
        <nav class="buttons">
            <ul>
                <li>
                    {match prev_href {
                        Some(href) => view! {
                            <a rel="prev" class="prev" href=href>
                                <i class="fa fa-chevron-left"></i>
                            </a>
                        }.into_any(),
                        None => view! {
                            <a rel="prev" class="prev disabled">
                                <i class="fa fa-chevron-left"></i>
                            </a>
                        }.into_any(),
                    }}
                </li>
                {pages.into_iter().map(|p| {
                    match p {
                        PageItem::Ellipsis => view! {
                            <li class="ellipsis">{"\u{2026}"}</li>
                        }.into_any(),
                        PageItem::Page(num) => {
                            let href = href_for_page.run(((num - 1) * limit, limit));
                            let class = if num == current_page { "active" } else { "" };
                            view! {
                                <li class=class><a href=href>{num}</a></li>
                            }.into_any()
                        }
                    }
                }).collect_view()}
                <li>
                    {match next_href {
                        Some(href) => view! {
                            <a rel="next" class="next" href=href>
                                <i class="fa fa-chevron-right"></i>
                            </a>
                        }.into_any(),
                        None => view! {
                            <a rel="next" class="next disabled">
                                <i class="fa fa-chevron-right"></i>
                            </a>
                        }.into_any(),
                    }}
                </li>
            </ul>
        </nav>
    }
    .into_any()
}

#[derive(Clone)]
enum PageItem {
    Page(i64),
    Ellipsis,
}

/// Compute which page numbers to display.
/// Shows: first 2, last 2, current ± 2 pages. Gaps become ellipsis.
fn visible_pages(current: i64, total: i64) -> Vec<PageItem> {
    let mut nums: Vec<i64> = Vec::new();

    // Always include first 2 and last 2
    for i in 1..=2.min(total) {
        nums.push(i);
    }
    for i in (total - 1).max(1)..=total {
        nums.push(i);
    }
    // Include current ± 2
    for i in (current - 2).max(1)..=(current + 2).min(total) {
        nums.push(i);
    }

    nums.sort();
    nums.dedup();

    // Convert to PageItems with ellipsis for gaps
    let mut result = Vec::new();
    let mut prev = 0i64;
    for num in nums {
        if num > prev + 1 {
            result.push(PageItem::Ellipsis);
        }
        result.push(PageItem::Page(num));
        prev = num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Extract page numbers from PageItems, using -1 for ellipsis.
    fn page_nums(items: &[PageItem]) -> Vec<i64> {
        items
            .iter()
            .map(|p| match p {
                PageItem::Page(n) => *n,
                PageItem::Ellipsis => -1,
            })
            .collect()
    }

    #[test]
    fn visible_pages_single_page() {
        let pages = visible_pages(1, 1);
        assert_eq!(page_nums(&pages), vec![1]);
    }

    #[test]
    fn visible_pages_two_pages() {
        let pages = visible_pages(1, 2);
        assert_eq!(page_nums(&pages), vec![1, 2]);
    }

    #[test]
    fn visible_pages_five_pages_at_start() {
        // 5 pages, on page 1: should show all 5 (no ellipsis needed)
        let pages = visible_pages(1, 5);
        assert_eq!(page_nums(&pages), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn visible_pages_ten_pages_at_start() {
        // 10 pages, on page 1: first 2 + current±2 (1,2,3) + last 2 (9,10)
        let pages = visible_pages(1, 10);
        assert_eq!(page_nums(&pages), vec![1, 2, 3, -1, 9, 10]);
    }

    #[test]
    fn visible_pages_ten_pages_at_middle() {
        // 10 pages, on page 5: first 2 (1,2), gap, 3,4,5,6,7, gap, last 2 (9,10)
        let pages = visible_pages(5, 10);
        assert_eq!(page_nums(&pages), vec![1, 2, 3, 4, 5, 6, 7, -1, 9, 10]);
    }

    #[test]
    fn visible_pages_ten_pages_at_end() {
        // 10 pages, on page 10: first 2 (1,2), gap, 8,9,10
        let pages = visible_pages(10, 10);
        assert_eq!(page_nums(&pages), vec![1, 2, -1, 8, 9, 10]);
    }

    #[test]
    fn visible_pages_twenty_pages_middle() {
        // 20 pages, on page 10: first 2, gap, 8-12, gap, last 2
        let pages = visible_pages(10, 20);
        assert_eq!(page_nums(&pages), vec![1, 2, -1, 8, 9, 10, 11, 12, -1, 19, 20]);
    }

    #[test]
    fn visible_pages_three_pages_at_middle() {
        // 3 pages, on page 2: all visible
        let pages = visible_pages(2, 3);
        assert_eq!(page_nums(&pages), vec![1, 2, 3]);
    }

    #[test]
    fn visible_pages_no_double_ellipsis() {
        // Page 3 of 10: first 2 (1,2), current±2 (1,2,3,4,5), gap, last 2 (9,10)
        let pages = visible_pages(3, 10);
        assert_eq!(page_nums(&pages), vec![1, 2, 3, 4, 5, -1, 9, 10]);
    }
}
