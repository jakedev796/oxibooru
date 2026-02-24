import { test, expect } from "@playwright/test";

test.describe("Dark theme", () => {
  test("body does not have darktheme class by default", async ({ page }) => {
    // Clear settings to ensure defaults
    await page.goto("/");
    await page.evaluate(() => localStorage.removeItem("oxibooru-settings"));
    await page.goto("/");
    await page.waitForTimeout(1_000);

    const body = page.locator("body");
    await expect(body).not.toHaveClass(/darktheme/);
  });

  test("toggling dark theme in settings adds darktheme class", async ({
    page,
  }) => {
    await page.goto("/settings");
    await expect(page.locator("text=dark theme")).toBeVisible({
      timeout: 10_000,
    });

    // Find and check the dark theme checkbox
    const darkThemeCheckbox = page
      .locator("label", { hasText: "dark theme" })
      .locator('input[type="checkbox"]');
    await darkThemeCheckbox.check();

    // Save settings
    await page.locator('input[value="Save settings"]').click();
    await expect(page.locator("text=Settings saved")).toBeVisible({
      timeout: 5_000,
    });

    // Body should now have darktheme class
    const body = page.locator("body");
    await expect(body).toHaveClass(/darktheme/);
  });

  test("dark theme persists across page navigation", async ({ page }) => {
    // Enable dark theme via localStorage
    await page.goto("/");
    await page.evaluate(() => {
      const settings = JSON.parse(
        localStorage.getItem("oxibooru-settings") || "{}"
      );
      settings.dark_theme = true;
      localStorage.setItem("oxibooru-settings", JSON.stringify(settings));
    });

    // Navigate to another page
    await page.goto("/posts");
    await page.waitForTimeout(1_000);

    const body = page.locator("body");
    await expect(body).toHaveClass(/darktheme/);

    // Navigate again
    await page.goto("/tags");
    await page.waitForTimeout(1_000);
    await expect(body).toHaveClass(/darktheme/);
  });

  test("disabling dark theme removes darktheme class", async ({ page }) => {
    // First enable it
    await page.goto("/");
    await page.evaluate(() => {
      const settings = JSON.parse(
        localStorage.getItem("oxibooru-settings") || "{}"
      );
      settings.dark_theme = true;
      localStorage.setItem("oxibooru-settings", JSON.stringify(settings));
    });

    await page.goto("/settings");
    await page.waitForTimeout(1_000);

    const body = page.locator("body");
    await expect(body).toHaveClass(/darktheme/);

    // Uncheck dark theme
    const darkThemeCheckbox = page
      .locator("label", { hasText: "dark theme" })
      .locator('input[type="checkbox"]');
    await darkThemeCheckbox.uncheck();
    await page.locator('input[value="Save settings"]').click();

    await expect(body).not.toHaveClass(/darktheme/);
  });
});
