import { expect, test, type Page } from "@playwright/test";
import { openWorkspaceSection } from "./workspace-driver";

// DEL-07-12: each export's step between its row's button and its save, opened from the keyboard and left by
// Escape, with focus back on the row's button. The browser build cannot solve, so the exports that need a solved
// result (the result JSON, stress-neutral and local FEA rows) say so in their rows with their buttons disabled; the
// shared step's keyboard behaviour for those is checked in ExportStep.test.tsx.
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

test("each export's step opens from the keyboard and closes by Escape, returning focus to its button", async ({ page }) => {
  await openExports(page);
  let opened = 0;
  for (const { prefix, saves } of EXPORTS) {
    const button = page.getByTestId(`${prefix}-open`);
    await expect(button).toBeVisible();
    for (const save of saves) await expect(page.getByTestId(save)).toHaveCount(0);
    if (await button.isDisabled()) {
      // Refused or unavailable: the row says why, beside the disabled button, and nothing can be saved.
      const reason = await button.getAttribute("aria-describedby");
      expect(reason, `${prefix}: a disabled button names the line that says why`).toBeTruthy();
      await expect(page.locator(`[id="${reason}"]`)).toBeVisible();
      continue;
    }
    await button.focus();
    await page.keyboard.press("Enter");
    const step = page.getByTestId(`${prefix}-step`);
    await expect(step).toBeVisible();
    await expect(step).toHaveAttribute("aria-modal", "true");
    // Focus is inside the step, and Tab stays there.
    expect(await step.evaluate((element) => element.contains(document.activeElement))).toBe(true);
    for (let index = 0; index < 8; index += 1) {
      await page.keyboard.press("Tab");
      expect(await step.evaluate((element) => element.contains(document.activeElement))).toBe(true);
    }
    // The save controls are inside the step, after the record.
    for (const save of saves) await expect(step.getByTestId(save)).toHaveCount(1);
    // The page behind is covered: a pointer cannot reach the row's button.
    const box = await button.boundingBox();
    expect(box).not.toBeNull();
    const hit = await page.evaluate(({ x, y }) => document.elementFromPoint(x, y)?.closest("[data-testid$='-step-backdrop']") !== null, { x: box!.x + box!.width / 2, y: box!.y + box!.height / 2 });
    expect(hit).toBe(true);
    await page.keyboard.press("Escape");
    await expect(step).toHaveCount(0);
    for (const save of saves) await expect(page.getByTestId(save)).toHaveCount(0);
    await expect(button).toBeFocused();
    opened += 1;
  }
  // PCF and review geometry need no solve, so their steps open in the browser build.
  expect(opened).toBeGreaterThanOrEqual(2);
});

test("Close leaves the step, writes nothing and returns focus to the row's button", async ({ page }) => {
  await openExports(page);
  const button = page.getByTestId("review-geometry-open");
  await button.click();
  const step = page.getByTestId("review-geometry-step");
  await expect(step).toBeVisible();
  const downloads: string[] = [];
  page.on("download", (download) => downloads.push(download.suggestedFilename()));
  await step.getByTestId("review-geometry-step-close").click();
  await expect(step).toHaveCount(0);
  await expect(button).toBeFocused();
  expect(downloads).toEqual([]);
});
