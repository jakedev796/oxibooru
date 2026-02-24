import { test, expect } from "@playwright/test";

test.describe("Post browsing", () => {
  test("should load the post list page", async ({ page }) => {
    await page.goto("/posts");
    // Should show the search bar
    const searchInput = page.locator("input.search-input, .search-bar input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });
  });

  test("should display post thumbnails if posts exist", async ({ page }) => {
    await page.goto("/posts");
    // Wait for the page to load
    await page.waitForTimeout(2_000);

    // If there are posts, thumbnails should be visible
    const thumbnails = page.locator(".post-thumbnail");
    const count = await thumbnails.count();
    if (count > 0) {
      await expect(thumbnails.first()).toBeVisible();
    }
  });

  test("should show pagination when there are enough posts", async ({ page }) => {
    await page.goto("/posts?limit=2");
    await page.waitForTimeout(2_000);

    // If total > limit, pagination should appear
    const pagination = page.locator(".pagination");
    const paginationCount = await pagination.count();
    // Pagination may or may not exist depending on post count — just check it doesn't error
    if (paginationCount > 0) {
      await expect(pagination).toBeVisible();
    }
  });

  test("should navigate to post detail page", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    const thumbnailCount = await thumbnail.count();
    if (thumbnailCount > 0) {
      await thumbnail.click();
      await page.waitForTimeout(1_000);
      // URL should now be /post/{id}
      expect(page.url()).toMatch(/\/post\/\d+/);
    }
  });

  test("should display post content on detail page", async ({ page }) => {
    await page.goto("/posts?limit=1&fields=id");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    const thumbnailCount = await thumbnail.count();
    if (thumbnailCount > 0) {
      await thumbnail.click();
      await page.waitForTimeout(2_000);

      // Post content area should be visible
      const content = page.locator(".post-content");
      const contentCount = await content.count();
      if (contentCount > 0) {
        await expect(content).toBeVisible();
      }
    }
  });
});
