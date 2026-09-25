import { expect, test } from "@playwright/test";
import { openWorkspaceSection as openSection } from "./workspace-driver";

// The retained 830-row / 828-witness package oracles remain in
// StressNeutralExportPanel.test.tsx's pure compatibility projection tests.
// A real browser can inspect preserved rows, but cannot create native Current.
test("built browser keeps preserved references separate from native Current exports", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  for (const mode of ["sparse", "dense"]) {
    await openSection(page, "solve");
    await page.getByTestId(`solver-mode-${mode}`).click();
    await expect(page.getByTestId(`solver-mode-${mode}`)).toHaveAttribute("aria-pressed", "true");
    await page.getByTestId("run-mechanics-preview").click();
    await expect(page.getByTestId("solve-job-summary")).toContainText("state=failed");
    await expect(page.getByTestId("solve-job-error")).toContainText("BROWSER_SOLVE_BACKEND_REQUIRED");
    await openSection(page, "results");
    await page.getByRole("button", { name: "Inspect bundled reference", exact: true }).click();
    await expect(page.getByRole("region", { name: "Bundled reference — not a solve for the current model" })).toBeVisible();
    await expect(page.getByTestId("result-filter-summary")).toContainText(mode === "sparse" ? "830" : "832");
    await expect(page.getByTestId("status-pill-solve-proof")).toHaveCount(0);
    await openSection(page, "exports");
    await expect(page.getByTestId("result-export-empty")).toBeVisible();
    await expect(page.getByTestId("stress-neutral-empty")).toBeVisible();
    await expect(page.getByTestId("result-export-link")).toHaveCount(0);
    await expect(page.getByTestId("stress-neutral-export-link")).toHaveCount(0);
    await expect(page.getByTestId("stress-neutral-csv-link")).toHaveCount(0);
  }
});
