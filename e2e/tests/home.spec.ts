import { test, expect } from "@playwright/test";

test.describe("Home page", () => {
  test("should load and display the page title", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle(/oxibooru/i);
  });

  test("should display server info", async ({ page }) => {
    await page.goto("/");
    // The home page should show post count
    await expect(page.locator("text=Posts")).toBeVisible({ timeout: 10_000 });
  });

  test("should render navigation bar", async ({ page }) => {
    await page.goto("/");
    const nav = page.locator("nav.top-navigation");
    await expect(nav).toBeVisible();
    // Home link should always be visible
    await expect(nav.locator("text=Home")).toBeVisible();
    await expect(nav.locator("text=Help")).toBeVisible();
  });

  test("should show login link for anonymous user", async ({ page }) => {
    await page.goto("/");
    const nav = page.locator("nav.top-navigation");
    await expect(nav.locator("text=Log in")).toBeVisible();
    await expect(nav.locator("text=Register")).toBeVisible();
  });
});
