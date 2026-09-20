import { expect, test, type Page } from "@playwright/test";
import { openWorkspaceSection, projectCommand } from "./workspace-driver";
import { attachBrowserIdentity } from "./ui-foundation-workflows";

test.beforeAll(async ({ browser }, info) => {
  await attachBrowserIdentity(browser, info);
});

async function packet(page: Page, kind: "validation" | "storage") {
  const id = `project-${kind}-export-link`;
  await page.getByTestId(`${id}-local-private-intent`).check();
  const href = await page.getByTestId(id).getAttribute("href");
  return JSON.parse(decodeURIComponent(href!.split(",").slice(1).join(",")));
}

test("B3B blank/create/save/open snapshot evidence", async ({ page }) => {
    await page.goto("/");
    await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
    await projectCommand(page, "new-blank");
    await openWorkspaceSection(page, "project");
    await expect(page.getByTestId("project-validation-model-hash")).toContainText("source=create");
    const created = await packet(page, "validation");
    expect(created.model_hash_integrity.integrity_status).toBe("verified_match");
    expect(created.model_hash_integrity.observed_at).toBeTruthy();
    await projectCommand(page, "save-local");
    await openWorkspaceSection(page, "project");
    await expect(page.getByTestId("project-validation-model-hash")).toContainText("model_hash_verified_at_save");
    const saved = await packet(page, "validation");
    const audit = await packet(page, "storage");
    expect(audit.model_hash_integrity).toEqual(saved.model_hash_integrity);
    expect(audit.project_envelope_hash_integrity).toEqual(saved.project_envelope_hash_integrity);
    await expect(page.getByTestId("model-hash-integrity")).toContainText(saved.model_hash_integrity.observed_at);
    await projectCommand(page, "list-local");
    await projectCommand(page, "open-local");
    await openWorkspaceSection(page, "project");
    await expect(page.getByTestId("project-validation-model-hash")).toContainText("model_hash_verified_on_open");
    const reopened = await packet(page, "validation");
    expect(reopened.model_hash_integrity.persisted_value).toBe(saved.model_hash_integrity.persisted_value);
    expect(reopened.model_hash_integrity.verification_source).toBe("open");
    await expect(page.getByTestId("project-edited")).toHaveCount(0);
  });
