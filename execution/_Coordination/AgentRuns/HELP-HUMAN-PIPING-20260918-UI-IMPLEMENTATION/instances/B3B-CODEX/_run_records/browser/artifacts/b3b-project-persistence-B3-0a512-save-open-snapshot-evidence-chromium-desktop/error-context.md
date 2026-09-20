# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: b3b-project-persistence.spec.ts >> B3B blank/create/save/open snapshot evidence
- Location: e2e/b3b-project-persistence.spec.ts:16:1

# Error details

```
Error: expect(locator).toContainText(expected) failed

Locator: getByTestId('project-validation-model-hash')
Expected substring: "model_hash_verified_on_open"
Received string:    "Model hash evidencemodel_hash=model_hash_persisted_open_verification_not_run; persisted_model_hashes=1; persisted_model_hash_ref=sha256:65971a7aa85a00d844676ba6f534ed70d9ac062a35bc95d3c065210101d89280; integrity=persistence_verification_not_run_this_session; claim_standing=not_recorded; source=none; observed_at=not_recorded; persisted snapshot only; later local edits are not verified"
Timeout: 10000ms

Call log:
  - Expect "toContainText" with timeout 10000ms
  - waiting for getByTestId('project-validation-model-hash')
    24 × locator resolved to <div class="report-line" data-testid="project-validation-model-hash">…</div>
       - unexpected value "Model hash evidencemodel_hash=model_hash_persisted_open_verification_not_run; persisted_model_hashes=1; persisted_model_hash_ref=sha256:65971a7aa85a00d844676ba6f534ed70d9ac062a35bc95d3c065210101d89280; integrity=persistence_verification_not_run_this_session; claim_standing=not_recorded; source=none; observed_at=not_recorded; persisted snapshot only; later local edits are not verified"

```

```yaml
- text: Model hash evidence
- strong: model_hash=model_hash_persisted_open_verification_not_run; persisted_model_hashes=1; persisted_model_hash_ref=sha256:65971a7aa85a00d844676ba6f534ed70d9ac062a35bc95d3c065210101d89280; integrity=persistence_verification_not_run_this_session; claim_standing=not_recorded; source=none; observed_at=not_recorded; persisted snapshot only; later local edits are not verified
```

# Test source

```ts
  1  | import { expect, test, type Page } from "@playwright/test";
  2  | import { openWorkspaceSection, projectCommand } from "./workspace-driver";
  3  | import { attachBrowserIdentity } from "./ui-foundation-workflows";
  4  | 
  5  | test.beforeAll(async ({ browser }, info) => {
  6  |   await attachBrowserIdentity(browser, info);
  7  | });
  8  | 
  9  | async function packet(page: Page, kind: "validation" | "storage") {
  10 |   const id = `project-${kind}-export-link`;
  11 |   await page.getByTestId(`${id}-local-private-intent`).check();
  12 |   const href = await page.getByTestId(id).getAttribute("href");
  13 |   return JSON.parse(decodeURIComponent(href!.split(",").slice(1).join(",")));
  14 | }
  15 | 
  16 | test("B3B blank/create/save/open snapshot evidence", async ({ page }) => {
  17 |     await page.goto("/");
  18 |     await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  19 |     await projectCommand(page, "new-blank");
  20 |     await openWorkspaceSection(page, "project");
  21 |     await expect(page.getByTestId("project-validation-model-hash")).toContainText("source=create");
  22 |     const created = await packet(page, "validation");
  23 |     expect(created.model_hash_integrity.integrity_status).toBe("verified_match");
  24 |     expect(created.model_hash_integrity.observed_at).toBeTruthy();
  25 |     await projectCommand(page, "save-local");
  26 |     await openWorkspaceSection(page, "project");
  27 |     await expect(page.getByTestId("project-validation-model-hash")).toContainText("model_hash_verified_at_save");
  28 |     const saved = await packet(page, "validation");
  29 |     const audit = await packet(page, "storage");
  30 |     expect(audit.model_hash_integrity).toEqual(saved.model_hash_integrity);
  31 |     expect(audit.project_envelope_hash_integrity).toEqual(saved.project_envelope_hash_integrity);
  32 |     await expect(page.getByTestId("model-hash-integrity")).toContainText(saved.model_hash_integrity.observed_at);
  33 |     await projectCommand(page, "list-local");
  34 |     await projectCommand(page, "open-local");
  35 |     await openWorkspaceSection(page, "project");
> 36 |     await expect(page.getByTestId("project-validation-model-hash")).toContainText("model_hash_verified_on_open");
     |                                                                     ^ Error: expect(locator).toContainText(expected) failed
  37 |     const reopened = await packet(page, "validation");
  38 |     expect(reopened.model_hash_integrity.persisted_value).toBe(saved.model_hash_integrity.persisted_value);
  39 |     expect(reopened.model_hash_integrity.verification_source).toBe("open");
  40 |     await expect(page.getByTestId("project-edited")).toHaveCount(0);
  41 |   });
  42 | 
```