import { expect, test, type Page } from "@playwright/test";
import { gotoRoutedFixture } from "./ui-foundation-workflows";

// DEL-07-01 REQ-012's reference canvases, in the timed instrument's 1440 by 920 browser window at
// first open with the default panes: Both, the model beside the table, a canvas 603 wide and at
// least 743 high; Model, the model alone with its table drawer closed (by the control a person
// uses), 1000 wide and at least MODEL_DRAWER_CLOSED_MIN_HEIGHT high. (With the drawer open, the
// default, the Model canvas measured 1000 by 463; that arrangement is not asserted.)

const MODEL_DRAWER_CLOSED_MIN_HEIGHT = 743;

async function canvasSize(page: Page) {
  return page.getByTestId("viewport-canvas").evaluate((element) => {
    const box = element.getBoundingClientRect();
    return { width: Math.round(box.width), height: Math.round(box.height) };
  });
}

test("the reference canvases in the 1440 by 920 window @explicit-viewport", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoRoutedFixture(page);

  await page.getByTestId("view-switch-both").click();
  await expect.poll(async () => (await canvasSize(page)).width).toBe(603);
  const both = await canvasSize(page);
  expect(both.width, `Both: canvas ${both.width} by ${both.height}`).toBe(603);
  expect(both.height, `Both: canvas ${both.width} by ${both.height}`).toBeGreaterThanOrEqual(743);

  await page.getByTestId("view-switch-model").click();
  await expect.poll(async () => (await canvasSize(page)).width).toBe(1000);
  const drawer = page.getByTestId("toggle-tree");
  const open = await canvasSize(page);
  const wasOpen = await drawer.getAttribute("aria-expanded") === "true";
  if (wasOpen) await drawer.click();
  await expect(drawer).toHaveAttribute("aria-expanded", "false");
  if (wasOpen) await expect.poll(async () => (await canvasSize(page)).height).toBeGreaterThan(open.height);
  const model = await canvasSize(page);
  expect(model.width, `Model, drawer closed: canvas ${model.width} by ${model.height}`).toBe(1000);
  expect(model.height, `Model, drawer closed: canvas ${model.width} by ${model.height}`).toBeGreaterThanOrEqual(MODEL_DRAWER_CLOSED_MIN_HEIGHT);
});
