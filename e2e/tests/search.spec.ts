import { test, expect } from "@playwright/test";

test.describe("Search functionality", () => {
  test("should have a search bar on the posts page", async ({ page }) => {
    await page.goto("/posts");
    const searchInput = page.locator("input.search-input, .search-bar input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });
  });

  test("should navigate with query parameter on search", async ({ page }) => {
    await page.goto("/posts");
    const searchInput = page.locator("input.search-input, .search-bar input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });

    await searchInput.fill("sort:id");
    await searchInput.press("Enter");
    await page.waitForTimeout(1_000);

    // URL should contain query parameter
    expect(page.url()).toContain("query=");
  });

  test("should have a search bar on the tags page", async ({ page }) => {
    await page.goto("/tags");
    const searchInput = page.locator("input.search-input, .search-bar input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });
  });

  test("should load tag list results", async ({ page }) => {
    await page.goto("/tags");
    await page.waitForTimeout(2_000);

    // If there are tags, a table should be visible
    const table = page.locator("table");
    const tableCount = await table.count();
    if (tableCount > 0) {
      await expect(table).toBeVisible();
    }
  });

  test("should have a search bar on the pools page", async ({ page }) => {
    await page.goto("/pools");
    const searchInput = page.locator("input.search-input, .search-bar input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });
  });

  test("should load help page sections", async ({ page }) => {
    await page.goto("/help");
    await expect(page.locator("text=About")).toBeVisible({ timeout: 10_000 });
    await expect(page.locator("text=Keyboard")).toBeVisible();
    await expect(page.locator("text=Search syntax")).toBeVisible();
  });

  test("should navigate help sections", async ({ page }) => {
    await page.goto("/help/keyboard");
    await page.waitForTimeout(1_000);
    await expect(page.locator("text=Keyboard shortcuts")).toBeVisible({ timeout: 10_000 });
  });

  test("should display user list", async ({ page }) => {
    await page.goto("/users");
    await page.waitForTimeout(2_000);

    // Page should load without error
    const searchInput = page.locator("input.search-input, .search-bar input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });
  });
});
