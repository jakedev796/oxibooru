import { test, expect } from "@playwright/test";

test.describe("Fit mode cycling", () => {
  test("fit mode button is visible on post view", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) > 0) {
      await thumbnail.click();
      await page.waitForTimeout(2_000);

      const fitBtn = page.locator(".fit-mode-btn");
      await expect(fitBtn).toBeVisible({ timeout: 5_000 });
      // Default should be "Fit both"
      await expect(fitBtn).toHaveText("Fit both");
    }
  });

  test("clicking fit mode button cycles through modes", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) > 0) {
      await thumbnail.click();
      await page.waitForTimeout(2_000);

      const fitBtn = page.locator(".fit-mode-btn");
      if ((await fitBtn.count()) > 0) {
        // Click through the cycle: Fit both → Fit width → Fit height → Original → Fit both
        await fitBtn.click();
        await expect(fitBtn).toHaveText("Fit width");

        await fitBtn.click();
        await expect(fitBtn).toHaveText("Fit height");

        await fitBtn.click();
        await expect(fitBtn).toHaveText("Original");

        await fitBtn.click();
        await expect(fitBtn).toHaveText("Fit both");
      }
    }
  });

  test("fit mode is saved to settings", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) > 0) {
      await thumbnail.click();
      await page.waitForTimeout(2_000);

      const fitBtn = page.locator(".fit-mode-btn");
      if ((await fitBtn.count()) > 0) {
        // Cycle to "Fit width"
        await fitBtn.click();
        await expect(fitBtn).toHaveText("Fit width");

        // Check that settings were updated in localStorage
        const fitMode = await page.evaluate(() => {
          const settings = JSON.parse(
            localStorage.getItem("oxibooru-settings") || "{}"
          );
          return settings.fit_mode;
        });
        expect(fitMode).toBe("fit-width");

        // Reset to default
        await page.evaluate(() => {
          const settings = JSON.parse(
            localStorage.getItem("oxibooru-settings") || "{}"
          );
          settings.fit_mode = "fit-both";
          localStorage.setItem("oxibooru-settings", JSON.stringify(settings));
        });
      }
    }
  });

  test("post content has correct fit mode CSS class", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) > 0) {
      await thumbnail.click();
      await page.waitForTimeout(2_000);

      const content = page.locator(".post-content");
      if ((await content.count()) > 0) {
        // Default should have fit-both class
        await expect(content).toHaveClass(/fit-both/);

        // Cycle to fit-width
        const fitBtn = page.locator(".fit-mode-btn");
        await fitBtn.click();
        await expect(content).toHaveClass(/fit-width/);
      }
    }
  });
});
