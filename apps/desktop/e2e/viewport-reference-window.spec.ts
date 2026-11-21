import { expect, test, type Page } from "@playwright/test";
import { gotoRoutedFixture } from "./ui-foundation-workflows";

// DEL-07-01 REQ-012's reference canvas widths: in the timed instrument's 1440 by 920 browser window,
// at first open with the default panes, the canvas is 603 wide with the model beside the table
// (Both) and 1000 wide with the model alone (Model). The heights depend on the panes and are recorded
// in the expectation messages, not asserted.

async function canvasSize(page: Page) {
  return page.getByTestId("viewport-canvas").evaluate((element) => {
    const box = element.getBoundingClientRect();
    return { width: Math.round(box.width), height: Math.round(box.height) };
  });
}

test("the canvas widths in the reference window @explicit-viewport", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoRoutedFixture(page);
  for (const [stage, width] of [["both", 603], ["model", 1000]] as const) {
    await page.getByTestId(`view-switch-${stage}`).click();
    await expect.poll(async () => (await canvasSize(page)).width).toBe(width);
    const size = await canvasSize(page);
    expect(size.width, `1440 by 920, ${stage}: canvas ${size.width} by ${size.height}`).toBe(width);
  }
});
