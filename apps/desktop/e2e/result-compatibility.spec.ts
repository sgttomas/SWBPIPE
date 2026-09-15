import { expect, test, type Page } from "@playwright/test";

async function openSection(page: Page, id: string) {
  const section = page.getByTestId(`workspace-section-${id}`);
  if (!await section.isVisible()) {
    await page.getByTestId("menu-view").click();
    await page.getByTestId(`menu-item-view.section.${id}`).click();
  }
  await expect(section).toBeVisible();
}

function dataJson(href: string) {
  return JSON.parse(decodeURIComponent(href.slice(href.indexOf(",") + 1)));
}

test("fresh Current exports strict 0.2 result and stress-neutral evidence", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await openSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await openSection(page, "exports");

  const resultHref = await page.getByTestId("result-export-link").getAttribute("href");
  expect(resultHref).toBeTruthy();
  const result = dataJson(resultHref!);
  expect(result.schema_version).toBe("0.2.0");
  expect(result.result_envelope.reproducibility.run_hashes.map((item: any) => item.payload_ref.ref)).toContain("run:preview-linear-static-001");

  const stressHref = await page.getByTestId("stress-neutral-export-link").getAttribute("href");
  expect(stressHref).toBeTruthy();
  const stress = dataJson(stressHref!);
  expect(stress.schema_version).toBe("0.2.0");
  expect(stress.manifest.package_members).toHaveLength(9);
  expect(stress.manifest.checksums).toHaveLength(9);
  expect(stress.package_checksum.payload_scope).toBe("complete_package_excluding_self_checksum");
  expect(stress.received_source_checksums.map((item: any) => item.payload_scope)).toContain("received_result");
  await expect(page.getByTestId("stress-neutral-unit-witnesses")).toContainText("count=828");
});
