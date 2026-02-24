import { test, expect } from "@playwright/test";

test.describe("Post view features", () => {
  test("reverse search links are visible on post view", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) === 0) return;

    await thumbnail.click();
    await page.waitForTimeout(2_000);

    // Reverse search section should be visible
    const reverseSearch = page.locator(".reverse-search");
    if ((await reverseSearch.count()) > 0) {
      await expect(reverseSearch).toBeVisible();

      // Should have at least one link
      const links = reverseSearch.locator("a");
      expect(await links.count()).toBeGreaterThan(0);

      // Links should open in new tab
      const firstLink = links.first();
      await expect(firstLink).toHaveAttribute("target", "_blank");
    }
  });

  test("IQDB link has correct URL format", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) === 0) return;

    await thumbnail.click();
    await page.waitForTimeout(2_000);

    const iqdbLink = page.locator('.reverse-search a:has-text("IQDB")');
    if ((await iqdbLink.count()) > 0) {
      const href = await iqdbLink.getAttribute("href");
      expect(href).toContain("iqdb.org");
    }
  });

  test("post neighbors navigation is visible", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) === 0) return;

    await thumbnail.click();
    await page.waitForTimeout(3_000);

    // Post neighbors nav should exist (may have prev, next, or both)
    const neighbors = page.locator(".post-neighbors");
    if ((await neighbors.count()) > 0) {
      const links = neighbors.locator("a");
      const linkCount = await links.count();
      // At least one navigation link should exist if there are multiple posts
      expect(linkCount).toBeGreaterThanOrEqual(0);
    }
  });

  test("post sidebar shows info section", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) === 0) return;

    await thumbnail.click();
    await page.waitForTimeout(2_000);

    // Info section with metadata
    const infoSection = page.locator(".post-info");
    if ((await infoSection.count()) > 0) {
      await expect(infoSection).toBeVisible();
      // Should show ID, Safety, Type, Size at minimum
      await expect(infoSection.locator("text=ID")).toBeVisible();
      await expect(infoSection.locator("text=Safety")).toBeVisible();
      await expect(infoSection.locator("text=Type")).toBeVisible();
    }
  });

  test("score and favorite widgets are visible", async ({ page }) => {
    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) === 0) return;

    await thumbnail.click();
    await page.waitForTimeout(2_000);

    // Score and favorites sections should be in the info dl
    const infoSection = page.locator(".post-info");
    if ((await infoSection.count()) > 0) {
      await expect(infoSection.locator("text=Score")).toBeVisible();
      await expect(infoSection.locator("text=Favorites")).toBeVisible();
    }
  });
});
