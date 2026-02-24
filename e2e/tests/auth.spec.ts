import { test, expect } from "@playwright/test";

test.describe("Authentication pages", () => {
  test("login page renders form with username and password", async ({
    page,
  }) => {
    await page.goto("/login");
    await expect(page.locator("#user-name")).toBeVisible({ timeout: 10_000 });
    await expect(page.locator("#user-password")).toBeVisible();
    await expect(page.locator('input[type="submit"]')).toBeVisible();
    await expect(page.locator("text=Forgot password")).toBeVisible();
  });

  test("register page renders form with username, password, and email", async ({
    page,
  }) => {
    await page.goto("/register");
    await expect(page.locator("#user-name")).toBeVisible({ timeout: 10_000 });
    await expect(page.locator("#user-password")).toBeVisible();
    await expect(page.locator("#user-email")).toBeVisible();
    await expect(
      page.locator('input[value="Create account"]')
    ).toBeVisible();
  });

  test("password reset page renders request form", async ({ page }) => {
    await page.goto("/password-reset");
    await expect(page.locator("#user-name")).toBeVisible({ timeout: 10_000 });
    await expect(
      page.locator('input[value="Reset password"]')
    ).toBeVisible();
  });

  test("logout page redirects to home", async ({ page }) => {
    await page.goto("/logout");
    // Should redirect to home page
    await expect(page).toHaveURL("/", { timeout: 10_000 });
  });

  test("login with invalid credentials shows error", async ({ page }) => {
    await page.goto("/login");
    await page.locator("#user-name").fill("nonexistent_user");
    await page.locator("#user-password").fill("wrong_password");
    await page.locator('input[type="submit"]').click();

    // Should show error message
    await expect(page.locator(".message.error")).toBeVisible({
      timeout: 10_000,
    });
  });
});
