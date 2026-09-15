import { expect, test, type Page } from "@playwright/test";

async function openSection(page: Page, id: string) {
  const section = page.getByTestId(`workspace-section-${id}`);
  if (!await section.isVisible()) {
    await page.getByTestId("menu-view").click();
    await page.getByTestId(`menu-item-view.section.${id}`).click();
  }
  await expect(section).toBeVisible();
}

async function replaceCurrentSource(page: Page) {
  await openSection(page, "solve");
  await page.getByTestId("solver-mode-dense").click();
  await expect(page.getByTestId("solver-mode-dense")).toHaveAttribute("aria-pressed", "true");
}

test("built dist carries checked 0.2 Current and stress-neutral packages", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await openSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await openSection(page, "exports");
  await expect(page.getByTestId("stress-neutral-export-link-local-private-intent")).toBeVisible();
  await expect(page.getByTestId("stress-neutral-export-link")).not.toHaveAttribute("href");
  await expect(page.getByTestId("stress-neutral-csv-link")).not.toHaveAttribute("href");
  await page.getByTestId("stress-neutral-export-link-local-private-intent").check();
  await expect(page.getByTestId("stress-neutral-export-link")).toHaveAttribute("href", /^data:application\/json/);
  const href = await page.getByTestId("stress-neutral-export-link").getAttribute("href");
  expect(href).toBeTruthy();
  const packet = JSON.parse(decodeURIComponent(href!.slice(href!.indexOf(",") + 1)));
  expect(packet.schema_version).toBe("0.2.0");
  expect(packet.manifest.package_members.map((item: any) => item.filename)).toEqual([
    "manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json",
    "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"
  ]);
  expect(packet.package_checksum.canonicalization).toBe("openpipestress_jcs_ijson_v1");
  expect(packet.export_profile).toMatchObject({ profile_id: "ops.stress_neutral.v2", profile_version: "0.2.0" });
  expect(packet.manifest.export_profile_ref.ref).toBe("ops.stress_neutral.v2");
  expect(packet.result_rows).toHaveLength(830);
  expect(packet.unit_preservation_witnesses).toHaveLength(828);
  expect(packet.diagnostics.filter((item: any) => item.code === "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK")).toHaveLength(2);
  await expect(page.getByTestId("stress-neutral-csv-link")).not.toHaveAttribute("href");

  await replaceCurrentSource(page);
  await openSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await openSection(page, "exports");
  await expect(page.getByTestId("stress-neutral-export-link-local-private-intent")).toBeVisible();
  await expect(page.getByTestId("stress-neutral-export-link")).not.toHaveAttribute("href");
  await page.getByTestId("stress-neutral-export-link-local-private-intent").check();
  await expect(page.getByTestId("stress-neutral-export-link")).toHaveAttribute("href", /^data:application\/json/);
  const replacementHref = await page.getByTestId("stress-neutral-export-link").getAttribute("href");
  const replacement = JSON.parse(decodeURIComponent(replacementHref!.slice(replacementHref!.indexOf(",") + 1)));
  expect(replacementHref).not.toBe(href);
  expect(replacement.package_checksum.value).not.toBe(packet.package_checksum.value);
  await expect(page.getByTestId("stress-neutral-state-binding")).toContainText(replacement.source_result_ref.ref);
});
