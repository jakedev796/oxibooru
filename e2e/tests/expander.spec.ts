import { test, expect } from "@playwright/test";

test.describe("Expander component", () => {
  // The expander is used on the post edit page for file replacement.
  // These tests require a post to exist and the user to have edit privileges.
  // We test the component behavior generically where possible.

  test("expander renders with title and is expanded by default", async ({
    page,
  }) => {
    // Clear expander state
    await page.goto("/");
    await page.evaluate(() =>
      localStorage.removeItem("oxibooru-expanders")
    );

    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) === 0) return;

    await thumbnail.click();
    await page.waitForTimeout(1_000);

    // Navigate to edit page
    const editLink = page.locator('a:has-text("Edit post")');
    if ((await editLink.count()) === 0) return;
    await editLink.click();
    await page.waitForTimeout(2_000);

    // Expander should be visible
    const expander = page.locator(".expander").first();
    if ((await expander.count()) === 0) return;

    // Header should show the title
    const header = expander.locator(".expander-header");
    await expect(header).toBeVisible();
    await expect(header).toContainText("Replace Files");

    // Body should be visible (expanded by default)
    const body = expander.locator(".expander-body");
    await expect(body).toBeVisible();
  });

  test("clicking expander header toggles collapsed state", async ({
    page,
  }) => {
    await page.goto("/");
    await page.evaluate(() =>
      localStorage.removeItem("oxibooru-expanders")
    );

    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) === 0) return;

    await thumbnail.click();
    await page.waitForTimeout(1_000);

    const editLink = page.locator('a:has-text("Edit post")');
    if ((await editLink.count()) === 0) return;
    await editLink.click();
    await page.waitForTimeout(2_000);

    const expander = page.locator(".expander").first();
    if ((await expander.count()) === 0) return;

    const header = expander.locator(".expander-header");
    const body = expander.locator(".expander-body");

    // Should start expanded
    await expect(body).toBeVisible();

    // Click to collapse
    await header.click();
    await expect(body).toBeHidden();

    // Click to expand again
    await header.click();
    await expect(body).toBeVisible();
  });

  test("expander state persists in localStorage", async ({ page }) => {
    await page.goto("/");
    await page.evaluate(() =>
      localStorage.removeItem("oxibooru-expanders")
    );

    await page.goto("/posts?limit=1");
    await page.waitForTimeout(2_000);

    const thumbnail = page.locator(".post-thumbnail a").first();
    if ((await thumbnail.count()) === 0) return;

    await thumbnail.click();
    await page.waitForTimeout(1_000);

    const editLink = page.locator('a:has-text("Edit post")');
    if ((await editLink.count()) === 0) return;
    await editLink.click();
    await page.waitForTimeout(2_000);

    const expander = page.locator(".expander").first();
    if ((await expander.count()) === 0) return;

    // Collapse the expander
    await expander.locator(".expander-header").click();
    await expect(expander.locator(".expander-body")).toBeHidden();

    // Check localStorage was updated
    const stored = await page.evaluate(() =>
      localStorage.getItem("oxibooru-expanders")
    );
    expect(stored).toBeTruthy();
    const parsed = JSON.parse(stored!);
    expect(parsed["post-edit-files"]).toBe(false);
  });
});
