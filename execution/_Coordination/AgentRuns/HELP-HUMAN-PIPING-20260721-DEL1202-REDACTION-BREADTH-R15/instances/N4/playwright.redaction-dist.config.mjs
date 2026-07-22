import baseConfig from "/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop/playwright.dist.config.ts";

const desktopRoot =
  "/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop";

export default {
  ...baseConfig,
  testDir: `${desktopRoot}/e2e`,
  testMatch: ["**/r2-smoke.spec.ts"],
  webServer: {
    ...baseConfig.webServer,
    cwd: desktopRoot,
  },
};
