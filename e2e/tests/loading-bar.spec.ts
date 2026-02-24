import { test, expect } from "@playwright/test";

test.describe("Loading bar", () => {
  test("should have a loading bar element in the DOM", async ({ page }) => {
    await page.goto("/");
    const loadingBar = page.locator(".loading-bar");
    await expect(loadingBar).toBeAttached();
  });

  test("should show active class during page navigation", async ({ page }) => {
    await page.goto("/");
    // Wait for initial load to complete
    await expect(page.locator("text=Posts")).toBeVisible({ timeout: 10_000 });

    // Navigate to a page that triggers an API call
    // Use evaluate to catch the loading bar in its active state
    const wasActive = await page.evaluate(() => {
      return new Promise<boolean>((resolve) => {
        const observer = new MutationObserver(() => {
          const bar = document.querySelector(".loading-bar");
          if (bar?.classList.contains("active")) {
            observer.disconnect();
            resolve(true);
          }
        });
        const bar = document.querySelector(".loading-bar");
        if (bar) {
          observer.observe(bar, { attributes: true, attributeFilter: ["class"] });
        }
        // Navigate
        const link = document.querySelector('a[href="/tags"]') as HTMLElement;
        if (link) link.click();
        // Timeout fallback
        setTimeout(() => {
          observer.disconnect();
          resolve(false);
        }, 5000);
      });
    });

    // The loading bar should have been active at some point during navigation
    expect(wasActive).toBe(true);
  });
});
