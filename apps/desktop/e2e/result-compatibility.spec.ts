import { expect, test, type Page } from "@playwright/test";
import { openWorkspaceSection as openSection } from "./workspace-driver";

function dataJson(href: string) {
  return JSON.parse(decodeURIComponent(href.slice(href.indexOf(",") + 1)));
}

async function replaceCurrentSource(page: Page) {
  await openSection(page, "solve");
  await page.getByTestId("solver-mode-dense").click();
  await expect(page.getByTestId("solver-mode-dense")).toHaveAttribute("aria-pressed", "true");
}

test("fresh Current exports strict 0.2 result and stress-neutral evidence", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await openSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await openSection(page, "exports");

  await expect(page.getByTestId("result-export-link")).not.toHaveAttribute("href");
  await page.getByTestId("result-export-link-local-private-intent").check();
  const resultHref = await page.getByTestId("result-export-link").getAttribute("href");
  expect(resultHref).toBeTruthy();
  const result = dataJson(resultHref!);
  expect(result.schema_version).toBe("0.2.0");
  expect(result.result_envelope.reproducibility.run_hashes.map((item: any) => item.payload_ref.ref_id)).toContain("run:preview-linear-static-001");

  await expect(page.getByTestId("stress-neutral-export-link")).not.toHaveAttribute("href");
  await expect(page.getByTestId("stress-neutral-csv-link")).not.toHaveAttribute("href");
  await page.getByTestId("stress-neutral-export-link-local-private-intent").check();
  await expect(page.getByTestId("stress-neutral-export-link")).toHaveAttribute("href", /^data:application\/json/);
  const stressHref = await page.getByTestId("stress-neutral-export-link").getAttribute("href");
  expect(stressHref).toBeTruthy();
  const stress = dataJson(stressHref!);
  expect(stress.schema_version).toBe("0.2.0");
  expect(stress.manifest.package_members).toHaveLength(9);
  expect(stress.manifest.checksums).toHaveLength(9);
  expect(stress.export_profile).toMatchObject({ profile_id: "ops.stress_neutral.v2", profile_version: "0.2.0" });
  expect(stress.manifest.export_profile_ref).toEqual({ object_type: "StressNeutralExportProfile", ref: "ops.stress_neutral.v2" });
  expect(stress.package_checksum.payload_scope).toBe("complete_package_excluding_self_checksum");
  expect(stress.received_source_checksums.map((item: any) => item.payload_scope)).toContain("received_result");
  expect(stress.result_rows).toHaveLength(830);
  expect(stress.unit_preservation_witnesses).toHaveLength(828);
  expect(stress.diagnostics.filter((item: any) => item.code === "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK")).toHaveLength(2);
  await expect(page.getByTestId("stress-neutral-csv-link")).not.toHaveAttribute("href");
  await expect(page.getByTestId("stress-neutral-unit-witnesses")).toContainText("count=828");

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
  const replacement = dataJson(replacementHref!);
  expect(replacementHref).not.toBe(stressHref);
  expect(replacement.package_checksum.value).not.toBe(stress.package_checksum.value);
  expect(replacement.source_model_ref).toEqual(stress.source_model_ref);
  await expect(page.getByTestId("stress-neutral-state-binding")).toContainText(replacement.source_result_ref.ref);
});
