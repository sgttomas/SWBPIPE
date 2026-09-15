import { expect, test, type Page } from "@playwright/test";

async function openSection(page: Page, id: string) {
  const section = page.getByTestId(`workspace-section-${id}`);
  if (!await section.isVisible()) {
    await page.getByTestId("menu-view").click();
    await page.getByTestId(`menu-item-view.section.${id}`).click();
  }
  await expect(section).toBeVisible();
}

test("built dist carries checked 0.2 Current and stress-neutral packages", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await openSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await openSection(page, "exports");
  const href = await page.getByTestId("stress-neutral-export-link").getAttribute("href");
  expect(href).toBeTruthy();
  const packet = JSON.parse(decodeURIComponent(href!.slice(href!.indexOf(",") + 1)));
  expect(packet.schema_version).toBe("0.2.0");
  expect(packet.manifest.package_members.map((item: any) => item.filename)).toEqual([
    "manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json",
    "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"
  ]);
  expect(packet.package_checksum.canonicalization).toBe("openpipestress_jcs_ijson_v1");
});
