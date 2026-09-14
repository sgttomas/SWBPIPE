import { expect, test } from "@playwright/test";

test("production dist exposes the report-package File route and fails honestly in browser mode", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();

  await page.getByTestId("menu-file").click();
  await expect(page.getByTestId("menu-item-file.save-report-package")).toBeDisabled();
  await page.getByTestId("app-menu-backdrop").click();

  await page.getByTestId("menu-view").click();
  await page.getByTestId("menu-item-view.section.solve").click();
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("readiness-mechanics")).toContainText("computed result rows");

  await page.getByTestId("menu-view").click();
  await page.getByTestId("menu-item-view.section.report").click();
  await page.getByTestId("report-package-private-intent").check();
  await page.getByTestId("menu-file").click();
  await page.getByTestId("menu-item-file.save-report-package").click();

  await expect(page.getByTestId("report-package-redaction-summary")).toContainText(
    "route=DREP-PACKAGE-SAVE-009"
  );
  await expect(page.getByTestId("report-package-redaction-summary")).toContainText("blocked=false");
  await expect(page.getByTestId("report-package-save-status")).toContainText(
    "REPORT-PACKAGE-SAVE-DESKTOP-ONLY"
  );
  await expect(page.locator("a[download$='.opsproj']")).toHaveCount(0);
});
