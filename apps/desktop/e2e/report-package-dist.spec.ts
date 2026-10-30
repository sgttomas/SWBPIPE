import { expect, test } from "@playwright/test";
import { openWorkspaceSection } from "./workspace-driver";

test("production browser keeps the report-package route unavailable for bundled references", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await page.getByTestId("menu-file").click();
  await expect(page.getByTestId("menu-item-file.save-report-package")).toBeDisabled();
  await page.getByTestId("app-menu-backdrop").click();
  await openWorkspaceSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=failed");
  await openWorkspaceSection(page, "results");
  await page.getByRole("button", { name: "Inspect bundled reference", exact: true }).click();
  await expect(page.getByRole("region", { name: "Bundled reference — not a solve for the current model" })).toBeVisible();
  await openWorkspaceSection(page, "report");
  await page.getByTestId("report-package-private-intent").check();
  await page.getByTestId("menu-file").click();
  await expect(page.getByTestId("menu-item-file.save-report-package")).toBeDisabled();
  await expect(page.getByTestId("report-package-save-status")).toHaveCount(0);
  await expect(page.locator("a[download$='.opsproj']")).toHaveCount(0);
});
