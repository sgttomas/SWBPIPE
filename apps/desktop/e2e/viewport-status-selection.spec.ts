import { expect, test } from "@playwright/test";
import { gotoRoutedFixture } from "./ui-foundation-workflows";
import { viewportControl } from "./workspace-driver";

// The status bar names the ordered selection's primary: after a box drawn over empty canvas clears
// the selection, it names nothing (the inspector still shows the project as its fallback).

test("select a pipe, box-select empty canvas: the status bar names nothing", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoRoutedFixture(page);
  await page.getByTestId("view-switch-model").click();
  const status = page.getByTestId("status-selection");
  // A drawn pipe label (an entity key is the JSON pair of type and id).
  const pipeLabel = page.locator(`.viewport-selection-layer [data-entity-key^='["pipe"']:visible`).first();
  await pipeLabel.click();
  await expect(status).toContainText("pipe:");
  await (await viewportControl(page, page.getByTestId("viewport-box-select"))).click();
  if (await page.getByTestId("viewport-more-menu").count()) await page.keyboard.press("Escape");
  // A box in the canvas's empty top-left corner, clear of the model and the furniture.
  const canvas = await page.getByTestId("viewport-canvas").boundingBox();
  if (!canvas) throw new Error("the canvas has no box");
  await page.mouse.move(canvas.x + 120, canvas.y + 40);
  await page.mouse.down();
  await page.mouse.move(canvas.x + 200, canvas.y + 90, { steps: 6 });
  await page.mouse.up();
  await expect(status).toHaveText("");
  await expect(status).not.toHaveAttribute("title");
});
