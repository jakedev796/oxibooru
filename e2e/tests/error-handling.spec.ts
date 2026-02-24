import { test, expect } from "@playwright/test";

test.describe("Error handling", () => {
  test("should display error message for invalid post ID", async ({ page }) => {
    await page.goto("/post/999999");
    // Should show the server's error description, not a generic "Failed to load"
    const errorMessage = page.locator(".message.error");
    await expect(errorMessage).toBeVisible({ timeout: 10_000 });
    // Should NOT contain the generic fallback text
    await expect(errorMessage).not.toContainText("Failed to load");
  });

  test("should show auth prompt for login-required pages", async ({ page }) => {
    // Attempting to edit a post without being logged in should show an auth error
    await page.goto("/post/1/edit");
    const errorMessage = page.locator(".message.error");
    await expect(errorMessage).toBeVisible({ timeout: 10_000 });
    // Auth errors should include a login link
    const loginLink = errorMessage.locator('a[href="/login"]');
    await expect(loginLink).toBeVisible();
  });

  test("should display 404 page with home link for unknown routes", async ({
    page,
  }) => {
    await page.goto("/this-route-does-not-exist");
    await expect(page.locator(".not-found-page")).toBeVisible();
    const homeLink = page.locator('.not-found-page a[href="/"]');
    await expect(homeLink).toBeVisible();
  });
});
