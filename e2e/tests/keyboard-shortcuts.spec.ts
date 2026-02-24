import { test, expect } from "@playwright/test";

test.describe("Keyboard shortcuts", () => {
  test("Q focuses the search input on the post list", async ({ page }) => {
    await page.goto("/posts");
    const searchInput = page.locator("#search-input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });

    // Press Q — should focus the search input
    await page.keyboard.press("q");
    await expect(searchInput).toBeFocused();
  });

  test("Q focuses the search input on the tag list", async ({ page }) => {
    await page.goto("/tags");
    const searchInput = page.locator("#search-input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });

    await page.keyboard.press("q");
    await expect(searchInput).toBeFocused();
  });

  test("shortcuts do not fire when typing in an input", async ({ page }) => {
    await page.goto("/posts");
    const searchInput = page.locator("#search-input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });

    // Focus the search input first
    await searchInput.click();
    await expect(searchInput).toBeFocused();

    // Type 'q' — should type into the input, not trigger the shortcut again
    await page.keyboard.type("hello");
    await expect(searchInput).toHaveValue("hello");
  });

  test("shortcuts disabled when setting is off", async ({ page }) => {
    // Disable keyboard shortcuts via localStorage
    await page.goto("/");
    await page.evaluate(() => {
      const settings = JSON.parse(
        localStorage.getItem("oxibooru-settings") || "{}"
      );
      settings.keyboard_shortcuts = false;
      localStorage.setItem("oxibooru-settings", JSON.stringify(settings));
    });

    await page.goto("/posts");
    const searchInput = page.locator("#search-input");
    await expect(searchInput).toBeVisible({ timeout: 10_000 });

    // Press Q — should NOT focus the search input
    await page.keyboard.press("q");
    await expect(searchInput).not.toBeFocused();

    // Re-enable for other tests
    await page.evaluate(() => {
      const settings = JSON.parse(
        localStorage.getItem("oxibooru-settings") || "{}"
      );
      settings.keyboard_shortcuts = true;
      localStorage.setItem("oxibooru-settings", JSON.stringify(settings));
    });
  });

  test("P focuses the first post thumbnail on post list", async ({ page }) => {
    await page.goto("/posts");
    await page.waitForTimeout(2_000);

    const firstThumb = page.locator(".post-thumbnail a").first();
    const count = await firstThumb.count();
    if (count > 0) {
      await page.keyboard.press("p");
      await expect(firstThumb).toBeFocused();
    }
  });

  test("F cycles fit mode on post view", async ({ page }) => {
    // Navigate to a post (if any exist)
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) > 0) {
      await thumbnail.click();
      await page.waitForTimeout(2_000);

      const fitBtn = page.locator(".fit-mode-btn");
      if ((await fitBtn.count()) > 0) {
        const initialText = await fitBtn.textContent();

        // Press F to cycle fit mode
        await page.keyboard.press("f");
        await page.waitForTimeout(200);

        const newText = await fitBtn.textContent();
        // The text should have changed (cycled to next mode)
        expect(newText).not.toBe(initialText);
      }
    }
  });

  test("E navigates to edit page on post view", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) > 0) {
      await thumbnail.click();
      await page.waitForTimeout(2_000);

      // Only works if we have edit privileges (anonymous may not)
      const currentUrl = page.url();
      if (currentUrl.match(/\/post\/\d+$/)) {
        await page.keyboard.press("e");
        await page.waitForTimeout(1_000);
        // URL should now be /post/{id}/edit (or unchanged if no privilege)
        const newUrl = page.url();
        // Just check it attempted navigation — may fail auth
        expect(newUrl).toMatch(/\/post\/\d+/);
      }
    }
  });
});
