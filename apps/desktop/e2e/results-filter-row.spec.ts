// The All rows tab's filter row at the product's default window: "Selected entity only" reads whole,
// on one line, at the default view's width (the model beside the table), not clipped to the clear
// control's square. The bundled reference results, the browser build's only results.
import { expect, test } from "@playwright/test";

test("Selected entity only reads on one line beside the result filter @explicit-viewport", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await page.goto("/");
  await page.getByTestId("rail-stage-results").click();
  await page.getByTestId("inspect-bundled-reference").click();
  const control = page.getByTestId("result-selection-only");
  await expect(control).toBeVisible({ timeout: 30_000 });
  await expect(control).toHaveText("Selected entity only");
  const shape = await control.evaluate((element) => {
    // The text's own line boxes: one top per line it takes.
    const range = document.createRange();
    range.selectNodeContents(element);
    const tops = new Set([...range.getClientRects()].filter((rect) => rect.width > 0).map((rect) => Math.round(rect.top)));
    return { lines: tops.size, clipped: element.scrollWidth > element.clientWidth + 1 || element.scrollHeight > element.clientHeight + 1 };
  });
  expect(shape).toEqual({ lines: 1, clipped: false });
});
