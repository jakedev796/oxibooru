import { test, expect } from "@playwright/test";

test.describe("PWA support", () => {
  test("should have a manifest link in the document head", async ({ page }) => {
    await page.goto("/");
    const manifest = page.locator('link[rel="manifest"]');
    await expect(manifest).toBeAttached();
    const href = await manifest.getAttribute("href");
    expect(href).toBe("/manifest.json");
  });

  test("should have a theme-color meta tag", async ({ page }) => {
    await page.goto("/");
    const themeColor = page.locator('meta[name="theme-color"]');
    await expect(themeColor).toBeAttached();
    const content = await themeColor.getAttribute("content");
    expect(content).toBe("#24aadd");
  });

  test("should have an apple-touch-icon link", async ({ page }) => {
    await page.goto("/");
    const icon = page.locator('link[rel="apple-touch-icon"]');
    await expect(icon).toBeAttached();
  });

  test("should serve the manifest.json file", async ({ page }) => {
    const response = await page.goto("/manifest.json");
    expect(response?.status()).toBe(200);
    const json = await response?.json();
    expect(json.name).toBe("oxibooru");
    expect(json.display).toBe("standalone");
    expect(json.theme_color).toBe("#24aadd");
    expect(json.icons).toBeDefined();
    expect(json.icons.length).toBeGreaterThan(0);
  });
});
