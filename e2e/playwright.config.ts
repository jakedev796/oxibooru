import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  timeout: 30_000,
  retries: 0,
  use: {
    baseURL: "http://localhost:8080",
    headless: true,
    screenshot: "only-on-failure",
  },
  projects: [
    {
      name: "chromium",
      use: { browserName: "chromium" },
    },
  ],
  webServer: {
    // Assumes trunk serve is running on port 8080 and backend on 6666.
    // Remove this block to skip auto-starting the dev server.
    command: "cd ../frontend && trunk serve --port 8080",
    port: 8080,
    reuseExistingServer: true,
    timeout: 120_000,
  },
});
