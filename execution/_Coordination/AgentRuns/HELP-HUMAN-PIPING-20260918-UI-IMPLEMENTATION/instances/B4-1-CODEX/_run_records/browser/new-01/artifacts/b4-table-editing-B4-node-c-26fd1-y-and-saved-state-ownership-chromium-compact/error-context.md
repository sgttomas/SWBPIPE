# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b4-table-editing.spec.ts >> B4 node coordinates apply through one operation with keyboard, history and saved-state ownership
- Location: e2e/b4-table-editing.spec.ts:6:1

# Error details

```
Error: expect(locator).not.toBeFocused() failed

Locator:  getByTestId('engineering-table').locator('[data-table-cell]').last()
Expected: not focused
Received: focused
Timeout:  10000ms

Call log:
  - Expect "not toBeFocused" with timeout 10000ms
  - waiting for getByTestId('engineering-table').locator('[data-table-cell]').last()
    24 × locator resolved to <button tabindex="0" type="button" data-column-key="z" data-table-cell="true" aria-label="node:N-140 Z: 2.2 m" data-row-key="["node","node:N-140"]" data-testid="table-cell-node:N-140-z">2.2</button>
       - unexpected value "focused"

```

```yaml
- 'button "node:N-140 Z: 2.2 m"': "2.2"
```

# Test source

```ts
  1  | import { expect, test } from "@playwright/test";
  2  | import { attachBrowserIdentity, currentModelHashThroughVisibleExport, gotoRoutedFixture } from "./ui-foundation-workflows";
  3  | import { ensureTreeExpanded, openWorkspaceSection, showModelTree } from "./workspace-driver";
  4  | 
  5  | // One connected journey per configured source viewport; no explicit-size repetition.
  6  | test("B4 node coordinates apply through one operation with keyboard, history and saved-state ownership", async ({ page, browser }, info) => {
  7  |   await attachBrowserIdentity(browser, info);
  8  |   await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  9  |   await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  10 |   const table = page.getByTestId("engineering-table");
  11 |   const cell = page.getByTestId("table-cell-node:N-100-y");
  12 |   await cell.click(); await expect(table.getByRole("button", { name: "Apply", exact: true })).toHaveCount(0);
  13 |   await cell.click(); const editor = table.getByRole("textbox", { name: "node:N-100 Y [m]" });
  14 |   await editor.fill("invalid"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  15 |   await expect(editor).toHaveValue("invalid"); await expect(editor).toHaveAttribute("aria-invalid", "true");
  16 |   await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  17 |   await expect(editor).toHaveValue("invalid"); await table.getByRole("button", { name: "Cancel", exact: true }).click();
  18 |   await expect(cell).toHaveText("0");
  19 |   await cell.dblclick(); await editor.fill("0.5"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  20 |   await expect(cell).toHaveText("0.5"); await expect(page.getByTestId("project-edited")).toBeVisible();
  21 |   await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText("0"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  22 |   await page.getByTestId("workspace-redo").click(); await expect(cell).toHaveText("0.5");
  23 |   await table.getByRole("button", { name: "Sort X", exact: true }).click(); await table.getByRole("button", { name: "Sort X", exact: true }).click();
  24 |   await cell.focus(); await page.keyboard.press("Enter"); await editor.fill("0.75"); await page.keyboard.press("Enter");
  25 |   await expect(cell).toHaveText("0.75");
  26 |   await expect(page.getByTestId("table-cell-node:N-110-y")).toBeFocused();
  27 |   await page.keyboard.press("Tab"); await expect(page.getByTestId("table-cell-node:N-110-z")).toBeFocused();
  28 |   await page.keyboard.press("2"); await page.keyboard.press("Escape"); await expect(page.getByTestId("table-cell-node:N-110-z")).toHaveText("0");
  29 |   // The boundaries must leave the coordinate control, including blur-driven no-op Apply.
  30 |   await table.getByRole("button", { name: /Sorted by/ }).click();
  31 |   const first = table.locator("[data-table-cell]").first(); await first.focus(); await page.keyboard.press("Shift+Tab"); await expect(first).not.toBeFocused();
  32 |   const last = table.locator("[data-table-cell]").last(); const lastLabel = await last.getAttribute("aria-label");
  33 |   await last.focus(); await page.keyboard.press("Tab"); await expect(last).not.toBeFocused();
  34 |   await last.dblclick(); const lastEditor = table.getByRole("textbox"); await page.keyboard.press("Tab");
> 35 |   await expect(lastEditor).toHaveCount(0); await expect(last).not.toBeFocused();
     |                                                                   ^ Error: expect(locator).not.toBeFocused() failed
  36 |   expect(await page.evaluate(() => document.activeElement !== document.body)).toBe(true);
  37 |   expect(lastLabel).toBeTruthy();
  38 |   await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click();
  39 |   await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project"); await expect(page.getByTestId("project-edited")).toHaveCount(0);
  40 |   await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  41 |   await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await expect(cell).toHaveText("0.75"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  42 |   await page.screenshot({ path: info.outputPath("b4-reopened-coordinate.png") });
  43 | });
  44 | 
  45 | 
  46 | test("B4 virtualized invalid editor survives scrolling and a filter threshold without changing the model", async ({ page, browser }, info) => {
  47 |   await attachBrowserIdentity(browser, info);
  48 |   const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  49 |   const hashBefore = await currentModelHashThroughVisibleExport(page);
  50 |   await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  51 |   const table = page.getByTestId("engineering-table");
  52 |   const first = model.nodes[0]; const last = model.nodes.at(-1);
  53 |   const cell = page.getByTestId(`table-cell-${first.id}-x`);
  54 |   await cell.dblclick(); const editor = table.getByRole("textbox", { name: `${first.id} X [${model.project.units.length}]` });
  55 |   await editor.fill("retained invalid coordinate"); await editor.press("Enter");
  56 |   await expect(editor).toHaveAttribute("aria-invalid", "true"); await expect(editor).toBeFocused();
  57 |   const rows = page.getByTestId("engineering-table-rows"); await rows.hover(); await page.mouse.wheel(0, 2400);
  58 |   await expect.poll(() => rows.evaluate((element) => element.scrollTop)).toBeGreaterThan(1000);
  59 |   await expect(editor).toHaveValue("retained invalid coordinate"); await expect(editor).toBeFocused();
  60 |   const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(last.id);
  61 |   await expect(table.getByText("Editing row retained outside the filter.")).toBeVisible();
  62 |   await expect(editor).toHaveValue("retained invalid coordinate"); await expect(filter).toBeFocused();
  63 |   await expect(rows.locator('[role="row"]')).toHaveCount(2);
  64 |   await filter.fill(""); await expect(editor).toHaveValue("retained invalid coordinate");
  65 |   await table.getByRole("button", { name: "Cancel", exact: true }).click();
  66 |   await expect(cell).toBeFocused(); await expect(cell).toHaveText(String(first.position.x));
  67 |   expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
  68 |   await page.screenshot({ path: info.outputPath("b4-virtual-editor-cancelled.png") });
  69 | });
  70 | 
```