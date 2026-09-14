import { expect, test } from "@playwright/test";

// TP-APP-R2-WASMPKG-001 regression: the production bundle must ship the wasm
// operation engine under /wasm-engine/ and the loader must reach ready from
// the built static assets. This is exactly the defect the human packaged-app
// pass found ("New blank" -> WASM-ENGINE-ASSET-ABSENT: 'text/html' is not a
// valid JavaScript MIME type) replayed against `vite preview` of `dist/`.
test("production dist serves the wasm engine and New blank succeeds", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  // Engine-ready guard (DEC-020 / ADR-0001): browser mode answers operations
  // through the wasm32 operation_applier build; ready proves the glue and
  // _bg.wasm both resolved from the dist asset layout.
  await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");

  // First wasm-dependent authoring action — the step that failed in the
  // packaged app.
  await page.getByRole("button", { name: "New blank" }).click();
  await expect(page.getByTestId("local-project-message")).toContainText(
    "Created blank local model document without fixture entities or external file copies."
  );
  await expect(page.getByTestId("load-case-manager-summary")).toContainText(
    "0 load cases; 0 primitive loads; 0 combinations"
  );
});
