import { test, expect } from "@playwright/test";

test.describe("Endless scroll", () => {
  test.beforeEach(async ({ page }) => {
    // Enable endless scroll via localStorage
    await page.goto("/");
    await page.evaluate(() => {
      const settings = JSON.parse(
        localStorage.getItem("oxibooru-settings") || "{}"
      );
      settings.endless_scroll = true;
      settings.posts_per_page = 2; // Small page size to trigger scroll
      localStorage.setItem("oxibooru-settings", JSON.stringify(settings));
    });
  });

  test("post list loads initial results with endless scroll", async ({
    page,
  }) => {
    await page.goto("/posts");
    await page.waitForTimeout(2_000);

    // Should show posts (if any exist)
    const thumbnails = page.locator(".post-thumbnail");
    const count = await thumbnails.count();
    // Should not show pagination in endless mode
    const pagination = page.locator(".pagination");
    const paginationCount = await pagination.count();
    if (count > 0) {
      expect(paginationCount).toBe(0);
    }
  });

  test("scroll sentinel is visible in endless mode", async ({ page }) => {
    await page.goto("/posts");
    await page.waitForTimeout(2_000);

    const sentinel = page.locator(".scroll-sentinel");
    const sentinelCount = await sentinel.count();
    // Sentinel should exist when endless scroll is active and there are posts
    const thumbnails = page.locator(".post-thumbnail");
    if ((await thumbnails.count()) > 0) {
      expect(sentinelCount).toBeGreaterThan(0);
    }
  });

  test("tag list loads without pagination in endless mode", async ({
    page,
  }) => {
    await page.goto("/tags");
    await page.waitForTimeout(2_000);

    // Should not have pagination component
    const pagination = page.locator(".pagination");
    const count = await pagination.count();
    // In endless mode, pagination should not appear
    const table = page.locator("table");
    if ((await table.count()) > 0) {
      expect(count).toBe(0);
    }
  });

  test("pagination mode shows pagination controls", async ({ page }) => {
    // Disable endless scroll
    await page.evaluate(() => {
      const settings = JSON.parse(
        localStorage.getItem("oxibooru-settings") || "{}"
      );
      settings.endless_scroll = false;
      localStorage.setItem("oxibooru-settings", JSON.stringify(settings));
    });

    await page.goto("/posts?limit=2");
    await page.waitForTimeout(2_000);

    const thumbnails = page.locator(".post-thumbnail");
    const pagination = page.locator(".pagination");
    // If there are enough posts, pagination should appear
    if ((await thumbnails.count()) >= 2) {
      await expect(pagination).toBeVisible({ timeout: 5_000 });
    }
  });
});
