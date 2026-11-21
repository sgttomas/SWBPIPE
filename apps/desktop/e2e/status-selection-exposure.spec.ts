import { expect, test, type Page } from "@playwright/test";
import { judgeStatusSelectionExposure } from "./ui-foundation/status-selection-exposure";

// The visual preflight's legibility check on the status bar's typed selection, on a small DOM
// fixture laid out as the product lays it out (a flex row whose spans clip with an ellipsis): text
// clipped in either span is refused, complete text passes.

const expected = { type: "pipe", id: "pipe:UIF-00077" };

async function fixture(page: Page, width: number, text = expected) {
  await page.setContent(`
    <style>
      .shell-status-selection { display: flex; align-items: baseline; gap: 4px; width: ${width}px; font: 13px sans-serif; white-space: nowrap; }
      .shell-status-selection > span { overflow: hidden; text-overflow: ellipsis; min-width: 0; }
      .shell-status-selection-type { flex-shrink: 1000; }
      .shell-status-selection-id { flex-shrink: 1; }
    </style>
    <span class="shell-status-selection" data-testid="status-selection"><span class="shell-status-selection-type">${text.type}:</span> <span class="shell-status-selection-id">${text.id}</span></span>
  `);
  return page.getByTestId("status-selection").evaluate(judgeStatusSelectionExposure, expected);
}

test("complete type and identifier pass", async ({ page }) => {
  const result = await fixture(page, 400);
  expect(result.status, JSON.stringify(result)).toBe("PASS_STATUS_SELECTION_EXPOSED");
});

test("an identifier clipped by the ellipsis is refused", async ({ page }) => {
  // Narrow enough that the type gives way to nothing and the identifier is cut.
  const result = await fixture(page, 60);
  expect(result.status, JSON.stringify(result)).toBe("FAIL_STATUS_SELECTION_NOT_EXPOSED");
  expect(result.id.complete).toBe(false);
});

test("a type clipped while the identifier is whole is refused", async ({ page }) => {
  // The type shrinks first: room for the identifier and a sliver of the type.
  const result = await fixture(page, 125);
  expect(result.status, JSON.stringify(result)).toBe("FAIL_STATUS_SELECTION_NOT_EXPOSED");
  expect(result.type.complete).toBe(false);
});

test("complete text that is not the expected typed identity is refused", async ({ page }) => {
  const result = await fixture(page, 400, { type: "pipe", id: "pipe:UIF-00078" });
  expect(result.status, JSON.stringify(result)).toBe("FAIL_STATUS_SELECTION_NOT_EXPOSED");
  expect(result.id.textMatches).toBe(false);
});
