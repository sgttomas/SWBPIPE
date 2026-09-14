import { expect, test, type Page } from "@playwright/test";

async function expectCanvasInWorkspace(page: Page) {
  await expect.poll(async () => page.evaluate(() => {
    const canvas = document.querySelector("canvas")?.getBoundingClientRect();
    const host = document.querySelector('[data-testid="viewport-canvas"]')?.getBoundingClientRect();
    return Boolean(canvas && host && Math.abs(canvas.width - host.width) < 2 && Math.abs(canvas.height - host.height) < 2);
  })).toBe(true);
  const measured = await page.evaluate(() => {
    const canvas = document.querySelector("canvas")!.getBoundingClientRect();
    const workspace = document.querySelector('[data-testid="modeling-workspace"]')!.getBoundingClientRect();
    return { canvas: canvas.toJSON(), workspace: workspace.toJSON(), windowHeight: innerHeight,
      horizontalOverflow: document.body.scrollWidth - document.body.clientWidth };
  });
  expect(measured.horizontalOverflow).toBe(0);
  expect(measured.canvas.width).toBeGreaterThan(0);
  expect(measured.canvas.height).toBeGreaterThan(0);
  expect(measured.canvas.top).toBeGreaterThanOrEqual(measured.workspace.top);
  expect(measured.canvas.bottom).toBeLessThanOrEqual(measured.workspace.bottom + 1);
  expect(measured.canvas.bottom).toBeLessThanOrEqual(measured.windowHeight);
  return measured.canvas;
}

test("workspace preserves the visible model during discovery, routing, property editing and analysis navigation", async ({ page }, testInfo) => {
  // The compact lane exercises the owner-selected initial minimum; this is
  // layout regression evidence, not independent usability acceptance.
  if (testInfo.project.name.includes("compact")) await page.setViewportSize({ width: 1024, height: 768 });
  await page.goto("/");
  await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  const before = await expectCanvasInWorkspace(page);
  await page.getByTestId("toolkit-entry").click();
  const during = await expectCanvasInWorkspace(page);
  expect(during).toEqual(before);
  await page.getByTestId("toolkit-build.pipe").click();
  await expect(page.getByTestId("viewport-create-pipe-id")).toBeFocused();
  await page.getByTestId("queue-explicit-pipe-intent").scrollIntoViewIfNeeded();
  await expectCanvasInWorkspace(page);
  await page.getByTestId("cancel-pipe-draft").click();
  if (await page.getByTestId("toggle-tree").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-tree").click();
  await page.getByTestId("tree-row-pipe:P-100").click();
  if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  await page.getByTestId("toggle-tree").click();
  await expect(page.getByTestId("inspector-text-value")).toContainText("Pump discharge run");
  await expect(page.getByTestId("inspector-dual-unit-display")).toHaveCount(0);
  await expectCanvasInWorkspace(page);
  await page.getByTestId("command-support").click();
  await expect(page.getByTestId("create-support-id")).toBeFocused();
  await expect(page.getByTestId("create-support-intent-panel")).toBeVisible();
  await expectCanvasInWorkspace(page);
  for (const section of ["loads", "solve", "results"] as const) {
    await page.getByTestId(`workspace-task-${section}`).click();
    await expect(page.getByTestId(`workspace-section-${section}`)).toBeVisible();
    await expectCanvasInWorkspace(page);
  }
  await page.getByTestId("toolkit-entry").click();
  await page.keyboard.press("Escape");
  await expect(page.getByTestId("toolkit-entry")).toBeFocused();
  await expect(page.locator("#toolkit-commands")).toHaveCount(0);
});
