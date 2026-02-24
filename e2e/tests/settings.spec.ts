import { test, expect } from "@playwright/test";

test.describe("Settings page", () => {
  test("renders settings form with all toggles", async ({ page }) => {
    await page.goto("/settings");
    await expect(page.locator("text=Browsing settings")).toBeVisible({
      timeout: 10_000,
    });
    await expect(page.locator("text=dark theme")).toBeVisible();
    await expect(page.locator("text=Upscale small posts")).toBeVisible();
    await expect(page.locator("text=endless scroll")).toBeVisible();
    await expect(page.locator("text=tag suggestions")).toBeVisible();
  });

  test("renders posts per page input with default value", async ({
    page,
  }) => {
    await page.goto("/settings");
    const input = page.locator('input[type="number"]');
    await expect(input).toBeVisible({ timeout: 10_000 });
    // Default is 42
    await expect(input).toHaveValue("42");
  });

  test("renders fit mode select", async ({ page }) => {
    await page.goto("/settings");
    const select = page.locator("select");
    await expect(select).toBeVisible({ timeout: 10_000 });
    // Should have fit-both as default
    await expect(select).toHaveValue("fit-both");
  });

  test("shows success message after saving", async ({ page }) => {
    await page.goto("/settings");
    await page.locator('input[value="Save settings"]').click();
    await expect(page.locator("text=Settings saved")).toBeVisible({
      timeout: 5_000,
    });
  });
});
