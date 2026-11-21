import { expect, test, type Page } from "@playwright/test";
import { openWorkspaceSection } from "./workspace-driver";

// DEL-07-12: each export's row in its form, and its record opened from the keyboard and left by Escape, with focus
// back on the control that opened it. Where the row writes on the press, its save controls are in the row and its
// Record control opens the whole record with no second save; where the record opens first, the save controls are in
// the record only. The browser build cannot solve, so the exports that need a solved result (the result JSON,
// stress-neutral and local FEA rows) say so in their rows with their buttons disabled; the shared record's keyboard
// behaviour for those is checked in ExportStep.test.tsx.
const EXPORTS: ReadonlyArray<{ prefix: string; saves: string[] }> = [
  { prefix: "result-export", saves: ["result-export-link"] },
  { prefix: "stress-neutral", saves: ["stress-neutral-export-link", "stress-neutral-csv-link"] },
  { prefix: "pcf-export", saves: ["pcf-export-link", "pcf-text-link"] },
  { prefix: "review-geometry", saves: ["review-geometry-export-link"] },
  { prefix: "local-fea", saves: ["local-fea-export-link"] }
];

async function openExports(page: Page): Promise<void> {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await openWorkspaceSection(page, "exports");
  await expect(page.getByTestId("export-list")).toBeVisible();
}

test("each export's record opens from the keyboard and closes by Escape, returning focus to the control that opened it", async ({ page }) => {
  await openExports(page);
  let opened = 0;
  for (const { prefix, saves } of EXPORTS) {
    const writes = (await page.getByTestId(`${prefix}-record-open`).count()) > 0;
    const control = writes ? page.getByTestId(`${prefix}-record-open`) : page.getByTestId(`${prefix}-open`);
    await expect(control).toBeVisible();
    if (writes) {
      // Writes on the press: the save controls are in the row.
      for (const save of saves) await expect(page.getByTestId(save)).toHaveCount(1);
    } else {
      for (const save of saves) await expect(page.getByTestId(save)).toHaveCount(0);
    }
    if (await control.isDisabled()) {
      // Refused or unavailable: the row says why, beside the disabled button, and nothing can be saved.
      const reason = await control.getAttribute("aria-describedby");
      expect(reason, `${prefix}: a disabled button names the line that says why`).toBeTruthy();
      await expect(page.locator(`[id="${reason}"]`)).toBeVisible();
      continue;
    }
    await control.focus();
    await page.keyboard.press("Enter");
    const step = page.getByTestId(`${prefix}-step`);
    await expect(step).toBeVisible();
    await expect(step).toHaveAttribute("aria-modal", "true");
    // Focus is inside the record, and Tab stays there.
    expect(await step.evaluate((element) => element.contains(document.activeElement))).toBe(true);
    for (let index = 0; index < 8; index += 1) {
      await page.keyboard.press("Tab");
      expect(await step.evaluate((element) => element.contains(document.activeElement))).toBe(true);
    }
    // Where the record opens first, the save controls are inside it, after the record; otherwise it holds none.
    for (const save of saves) await expect(step.getByTestId(save)).toHaveCount(writes ? 0 : 1);
    // The page behind is covered: a pointer cannot reach the row's control.
    const box = await control.boundingBox();
    expect(box).not.toBeNull();
    const hit = await page.evaluate(({ x, y }) => document.elementFromPoint(x, y)?.closest("[data-testid$='-step-backdrop']") !== null, { x: box!.x + box!.width / 2, y: box!.y + box!.height / 2 });
    expect(hit).toBe(true);
    await page.keyboard.press("Escape");
    await expect(step).toHaveCount(0);
    await expect(control).toBeFocused();
    opened += 1;
  }
  // PCF and review geometry need no solve, so their records open in the browser build.
  expect(opened).toBeGreaterThanOrEqual(2);
});

test("Close leaves the record, writes nothing and returns focus to the control that opened it", async ({ page }) => {
  await openExports(page);
  const button = page.getByTestId("pcf-export-open");
  await button.click();
  const step = page.getByTestId("pcf-export-step");
  await expect(step).toBeVisible();
  const downloads: string[] = [];
  page.on("download", (download) => downloads.push(download.suggestedFilename()));
  await step.getByTestId("pcf-export-step-close").click();
  await expect(step).toHaveCount(0);
  await expect(button).toBeFocused();
  expect(downloads).toEqual([]);
});

test("the demonstration model: review geometry writes on the press, PCF opens its record first with this model's entries at its head", async ({ page }) => {
  await openExports(page);
  const geometry = page.getByTestId("review-geometry-panel");
  await expect(geometry.getByTestId("review-geometry-export-link")).toHaveCount(1);
  await expect(geometry.getByTestId("review-geometry-standing")).toHaveText("5 things this format does not carry");
  await expect(geometry.getByTestId("review-geometry-record-label")).toHaveText("Preview");
  await expect(geometry.getByTestId("review-geometry-exchange")).toHaveText("For exchange with another pipe stress program use Model batch file (.mbf).");
  const pcf = page.getByTestId("pcf-export-panel");
  await expect(pcf.getByTestId("pcf-export-open")).toHaveText(/^Review \d+ entr(y|ies) from this model, then export…$/);
  await expect(pcf.getByTestId("pcf-export-not-written")).toContainText("Nominal size");
  await pcf.getByTestId("pcf-export-open").click();
  const step = page.getByTestId("pcf-export-step");
  const first = step.locator(".export-step-record > *").first();
  await expect(first).toHaveAttribute("data-testid", "pcf-export-step-causes");
});
