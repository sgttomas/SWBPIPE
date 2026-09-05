import { expect, test, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

type WorkflowModelFixture = {
  project: { id: string };
  analysis_status: {
    mechanics: string;
    rule_check: string;
    professional_acceptance: string;
  };
  data_boundary: Record<string, string>;
  load_cases: Array<{
    id: string;
    primitive_loads: Array<{ magnitude: { value: number; unit: string } }>;
  }>;
};

type WorkflowResultFixture = {
  status: {
    mechanics: string;
    rule_check: string;
    professional_acceptance: string;
  };
  results: unknown[];
};

const modelFixturePath = fileURLToPath(
  new URL("../../../fixtures/product_preview/invented_preview_model.json", import.meta.url)
);
const resultFixturePath = fileURLToPath(
  new URL("../../../fixtures/product_preview/invented_mechanics_result.json", import.meta.url)
);
const modelFixture = JSON.parse(readFileSync(modelFixturePath, "utf8")) as WorkflowModelFixture;
const resultFixture = JSON.parse(readFileSync(resultFixturePath, "utf8")) as WorkflowResultFixture;
const editedLoadCase = modelFixture.load_cases.find((item) => item.id === "load:L-100");

if (!editedLoadCase) throw new Error(`GUI workflow fixture is missing load:L-100: ${modelFixturePath}`);

async function openWorkspaceSection(page: Page, sectionId: string): Promise<void> {
  const section = page.getByTestId(`workspace-section-${sectionId}`);
  if (await section.isVisible()) return;
  await page.getByTestId("menu-view").click();
  await page.getByTestId(`menu-item-view.section.${sectionId}`).click();
  await expect(section).toBeVisible();
}

async function ensureRailExpanded(page: Page, testId: "toggle-tree" | "toggle-inspector"): Promise<void> {
  const toggle = page.getByTestId(testId);
  if ((await toggle.getAttribute("aria-expanded")) !== "true") await toggle.click();
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
}

test("DEL-09-04 invented fixture exposes warnings, boundaries, and honest solve/result transitions", async ({ page }) => {
  const externalRequests: string[] = [];
  page.on("request", (request) => {
    const url = new URL(request.url());
    if (!["127.0.0.1", "localhost"].includes(url.hostname) && !["data:", "blob:"].includes(url.protocol)) {
      externalRequests.push(request.url());
    }
  });

  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");
  await ensureRailExpanded(page, "toggle-tree");
  await ensureRailExpanded(page, "toggle-inspector");

  // Bind the documented pre-solve state directly to the repository fixture.
  await expect(page.getByTestId(`tree-row-${modelFixture.project.id}`)).toBeVisible();
  await expect(page.getByTestId("status-pill-mechanics")).toContainText(modelFixture.analysis_status.mechanics);
  await expect(page.getByTestId("status-pill-rule-check")).toContainText("RULE_INPUTS_INCOMPLETE");
  await expect(page.getByTestId("status-pill-professional")).toContainText("HUMAN_REVIEW_REQUIRED");
  await openWorkspaceSection(page, "solve");
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=not_started");
  await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=0");
  await expect(page.getByTestId("readiness-mechanics")).toContainText("preview run not started");
  await expect(page.getByTestId("readiness-rule")).toContainText("rule inputs incomplete");
  await expect(page.getByTestId("readiness-professional")).toContainText("human review remains required");
  await expect(page.getByTestId("solve-job-boundary")).toContainText("private payload=false");
  await expect(page.getByTestId("solve-job-boundary")).toContainText("protected content=false");
  await expect(page.getByTestId("solve-job-boundary")).toContainText("human review required");

  // Establish the fixture's solved state and prove result rows become visible.
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await expect(page.getByTestId("solve-job-summary")).toContainText(`result_rows=${resultFixture.results.length}`);
  await expect(page.getByTestId("status-pill-mechanics")).toContainText(resultFixture.status.mechanics);
  await expect(page.getByTestId("status-pill-rule-check")).toContainText(resultFixture.status.rule_check);
  const visibleSolveProof = page.getByTestId("status-pill-solve-proof");
  await expect(visibleSolveProof).toContainText("seam=browser_fixture_no_backend_job");
  await expect(visibleSolveProof).toContainText(`project=${modelFixture.project.id}`);
  await expect(visibleSolveProof).toContainText(`result_model=${modelFixture.project.id}`);
  await expect(visibleSolveProof).toContainText("identity=match");
  await expect(visibleSolveProof).toContainText(`rows=${resultFixture.results.length}`);
  await expect(visibleSolveProof).toContainText("job=job:preview-linear-static:");
  await expect(visibleSolveProof).toContainText("model_sha256=sha256:");
  await expect(visibleSolveProof).toContainText("input_manifest_sha256=");
  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("result-filter-summary")).toContainText(
    `${resultFixture.results.length} of ${resultFixture.results.length} results match filter`
  );

  // The same solved state keeps missing rule inputs, provenance warnings, and
  // professional assumptions visible and textually distinct from mechanics.
  await page.getByTestId("issues-drawer-toggle").click();
  const solvedIssues = page.getByTestId("issues-home");
  await expect(solvedIssues.getByTestId("missing-data-summary")).toContainText("solve_blocked=false");
  await expect(solvedIssues.getByTestId("missing-data-summary")).toContainText("rule_blocked=true");
  await expect(solvedIssues.getByTestId("missing-data-status-separation")).toContainText(
    `mechanics=${resultFixture.status.mechanics}`
  );
  await expect(solvedIssues.getByTestId("missing-data-status-separation")).toContainText(
    `rule_check=${resultFixture.status.rule_check}`
  );
  await expect(solvedIssues.getByTestId("missing-data-warning-rule-check-required-inputs")).toContainText(
    "RULE_CHECK_BLOCKING"
  );
  await expect(solvedIssues.getByTestId("missing-data-warning-public-fixture-provenance-only")).toContainText(
    "PROVENANCE_WARNING"
  );
  await expect(solvedIssues.getByTestId("missing-data-warning-professional-acceptance-not-provided")).toContainText(
    "ASSUMPTION_WARNING"
  );
  await expect(solvedIssues.getByTestId("missing-data-boundary")).toContainText("silent_defaults=false");
  await expect(solvedIssues.getByTestId("missing-data-boundary")).toContainText("auto_fill=false");
  await solvedIssues.getByRole("button", { name: /Close/i }).click();

  await page.getByTestId("audit-drawer-toggle").click();
  const initialAudit = page.getByTestId("audit-boundary-drawer");
  for (const boundaryValue of Object.values(modelFixture.data_boundary)) {
    await expect(initialAudit.getByTitle(boundaryValue)).toBeVisible();
  }
  await expect(initialAudit.getByTestId("local-project-status")).toContainText("network=false");
  await expect(initialAudit.getByTestId("local-project-status")).toContainText("telemetry=false");
  await initialAudit.getByRole("button", { name: /Close/i }).click();

  // Restore the visible authoring layout before using the inspector. Keeping
  // Solve open lets its dock intercept compact-viewport pointer events.
  await page.getByTestId("workspace-dock-close").click();
  await expect(page.getByTestId("workspace-dock")).toHaveClass(/collapsed/);
  await expect(page.getByTestId("workspace-section-solve")).toBeHidden();
  const inspectorBounds = await page.locator(".workspace-pane-inspector").boundingBox();
  expect(inspectorBounds?.width).toBeGreaterThan(300);

  // Edit explicit invented load data through the visible inspector and apply it
  // through the product's local WASM operation engine.
  await page.getByTestId(`tree-row-${editedLoadCase.id}`).click();
  const editor = page.getByTestId("editor-intent-panel");
  await editor.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  await expect(editor.getByTestId("editor-intent-value")).toHaveValue(
    String(editedLoadCase.primitive_loads[0].magnitude.value)
  );
  await expect(editor.getByTestId("editor-intent-unit")).toHaveValue(editedLoadCase.primitive_loads[0].magnitude.unit);
  await editor.getByTestId("editor-intent-value").fill("-225");
  await expect(editor.getByTestId("editor-operation-preview")).toContainText(
    'after={"value":-225,"unit":"N/m"}'
  );
  await editor.getByTestId("queue-editor-intent").click();
  await openWorkspaceSection(page, "operations");
  await page.getByTestId("apply-intent-editor-intent-1").click();
  const receipt = page.getByTestId("applied-operation-route-applied-1-editor-intent-1");
  await expect(receipt).toContainText("Applied through local_wasm_engine");
  await expect(receipt).toContainText("professional approval not recorded");
  await expect(page.getByTestId("workspace-section-operations")).toBeVisible();
  await page.getByTestId("menu-view").click();
  await page.getByTestId("menu-item-view.section.solve").click();
  await expect(page.getByTestId("workspace-section-solve")).toBeVisible();
  await expect(page.getByTestId("readiness-mechanics")).toBeVisible();
  await expect(page.getByTestId("readiness-mechanics")).toContainText("preview run not started");
  const resetViewportStatus = page.getByTestId("viewport-deformation-status");
  await expect(resetViewportStatus).toBeVisible();
  await expect(resetViewportStatus).toContainText("not started; result rows=0");
  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("results-panel")).toContainText(
    "Run the bounded preview mechanics path to populate result summaries."
  );

  // Save, list, and reopen by stable project id through normal visible controls.
  await page.getByRole("button", { name: "Save local" }).click();
  await expect(page.getByTestId("local-project-message")).toContainText(
    "Saved local browser-preview project snapshot without external file copies."
  );
  await page.getByRole("button", { name: "List local" }).click();
  await expect(page.getByTestId("local-project-message")).toContainText(
    "Listed 1 local project snapshot from the local store index."
  );
  await page.getByTestId("project-index-open-project:invented-loop-01").click();
  await expect(page.getByTestId("local-project-message")).toContainText(
    "Opened local browser-preview project snapshot by id project:invented-loop-01."
  );

  // The edited unit-bearing value is still the current value after reopen.
  await page.getByTestId(`tree-row-${editedLoadCase.id}`).click();
  await editor.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  await expect(editor.getByTestId("editor-intent-value")).toHaveValue("-225");
  await expect(editor.getByTestId("editor-intent-unit")).toHaveValue("N/m");

  // Browser Playwright intentionally has no native solver fallback for an
  // edited model. Validate the user-visible blocking state instead of allowing
  // stale fixture results to masquerade as a successful re-solve.
  await openWorkspaceSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=0");
  await expect(page.getByTestId("status-pill-mechanics")).toContainText("MODEL_INCOMPLETE");
  await page.getByTestId("issues-drawer-toggle").click();
  await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toBeVisible();
  await expect(page.getByTestId("viewport-deformation-status")).toContainText(
    "blocked; mechanics=model incomplete; rows=0"
  );
  const blockedIssues = page.getByTestId("issues-home");
  await expect(blockedIssues.getByTestId("missing-data-summary")).toContainText("solve_blocked=true");
  await expect(blockedIssues.getByTestId("missing-data-summary")).toContainText("rule_blocked=true");
  await expect(blockedIssues.getByTestId("missing-data-warning-solve-required-physical-inputs")).toContainText(
    "SOLVE_BLOCKING"
  );
  await expect(blockedIssues.getByTestId("missing-data-warning-solve-required-physical-inputs")).toContainText(
    "Mechanics solve-required data is incomplete."
  );
  await blockedIssues.getByRole("button", { name: /Close/i }).click();
  await expect(blockedIssues).toHaveCount(0);

  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("result-filter-summary")).toContainText("0 of 0 results match filter");
  await expect(page.getByTestId("result-filter-empty")).toContainText("No computed preview result rows");

  await page.getByTestId("audit-drawer-toggle").click();
  const finalAudit = page.getByTestId("audit-boundary-drawer");
  await expect(finalAudit).toBeVisible();
  for (const boundaryValue of Object.values(modelFixture.data_boundary)) {
    await expect(finalAudit.getByTitle(boundaryValue)).toBeVisible();
  }
  await expect(finalAudit.getByTestId("local-project-status")).toContainText("network=false");
  await expect(finalAudit.getByTestId("local-project-status")).toContainText("telemetry=false");
  expect(externalRequests).toEqual([]);
});

for (const { drawerId, toggleId, contentId, contentText } of [
  { drawerId: "issues-home", toggleId: "issues-drawer-toggle", contentId: "missing-data-summary", contentText: "solve_blocked=false" },
  { drawerId: "audit-boundary-drawer", toggleId: "audit-drawer-toggle", contentId: "local-project-status", contentText: "network=false" }
]) {
  test(`shared drawer menu overlap preserves ordinary Close and active menu priority: ${drawerId}`, async ({ page }, testInfo) => {
    await page.goto("/");
    await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");
    await openWorkspaceSection(page, "solve");
    await page.getByTestId("run-mechanics-preview").click();
    await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
    await expect(page.getByTestId("solve-job-summary")).toContainText(`result_rows=${resultFixture.results.length}`);

    const drawer = page.getByTestId(drawerId);
    const close = drawer.getByRole("button", { name: /Close/i });
    const menu = page.getByTestId("app-menu-bar");
    const measure = async (phase: string) => {
      const geometry = await close.evaluate((button) => {
        const nav = document.querySelector<HTMLElement>('[data-testid="app-menu-bar"]')!;
        const aside = button.closest<HTMLElement>(".workspace-drawer")!;
        const rect = button.getBoundingClientRect();
        const center = { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 };
        const hit = document.elementFromPoint(center.x, center.y);
        return {
          center, close: rect.toJSON(), menu: nav.getBoundingClientRect().toJSON(),
          drawer: aside.getBoundingClientRect().toJSON(),
          lastTriggerRight: Math.max(...Array.from(nav.querySelectorAll("button")).map((item) => item.getBoundingClientRect().right)),
          viewport: { width: innerWidth, height: innerHeight },
          hit: hit ? `${hit.tagName.toLowerCase()}.${hit.className}` : null,
          closeReceivesHit: hit === button || button.contains(hit),
          menuReceivesHit: hit === nav || nav.contains(hit),
          menuExpanded: nav.querySelector('[aria-expanded="true"]') !== null
        };
      });
      await testInfo.attach(`${drawerId}-${phase}`, { body: JSON.stringify(geometry, null, 2), contentType: "application/json" });
      return geometry;
    };

    // Exercise the ordinary fixture layout before imposing the controlled overlap.
    await page.getByTestId(toggleId).click();
    await expect(drawer.getByTestId(contentId)).toContainText(contentText);
    const ordinary = await measure("ordinary-layout");
    expect(ordinary.closeReceivesHit, `ordinary Close intercepted by ${ordinary.hit}`).toBe(true);
    await close.click();
    await expect(drawer).toHaveCount(0);

    await page.getByTestId(toggleId).click();
    await expect(drawer).toBeVisible();
    // Position only this existing drawer: no stacking, pointer-event, visibility,
    // or handler changes. This geometry is deterministic, not a hosted-pixel claim.
    await drawer.evaluate((aside) => {
      const nav = document.querySelector('[data-testid="app-menu-bar"]')!.getBoundingClientRect();
      const button = Array.from(aside.querySelectorAll("button")).find((item) => /Close/i.test(item.textContent ?? ""))!;
      const rect = button.getBoundingClientRect();
      const offset = rect.y + rect.height / 2 - aside.getBoundingClientRect().top;
      aside.style.top = `${nav.y + nav.height / 2 - offset}px`;
      aside.style.bottom = "auto";
    });
    const overlap = await measure("controlled-closed-menu");
    expect(overlap.menuExpanded).toBe(false);
    expect(overlap.center.x).toBeGreaterThan(overlap.lastTriggerRight);
    expect(overlap.center.x).toBeGreaterThan(overlap.menu.left);
    expect(overlap.center.x).toBeLessThan(overlap.menu.right);
    expect(overlap.center.y).toBeGreaterThan(overlap.menu.top);
    expect(overlap.center.y).toBeLessThan(overlap.menu.bottom);
    expect(overlap.close.left).toBeGreaterThanOrEqual(0);
    expect(overlap.close.top).toBeGreaterThanOrEqual(0);
    expect(overlap.close.right).toBeLessThanOrEqual(overlap.viewport.width);
    expect(overlap.close.bottom).toBeLessThanOrEqual(overlap.viewport.height);
    expect(overlap.closeReceivesHit, `closed menu intercepts Close: hit=${overlap.hit}`).toBe(true);
    await close.click();
    await expect(drawer).toHaveCount(0);

    // Reopen with ordinary positioning; command and click-catcher dismiss only
    // the menu, and a regular Close still dismisses the nonmodal drawer.
    await page.getByTestId(toggleId).click();
    await page.getByTestId("menu-view").click();
    await expect(page.getByTestId("menu-view")).toHaveAttribute("aria-expanded", "true");
    await expect(menu).toHaveCSS("z-index", "50");
    await measure("active-view-command");
    await page.getByTestId("menu-item-view.section.operations").click();
    await expect(page.getByTestId("workspace-section-operations")).toBeVisible();
    await expect(page.getByTestId("menu-dropdown-view")).toHaveCount(0);
    await expect(drawer).toBeVisible();
    await page.getByTestId("menu-view").click();
    const bounds = await drawer.boundingBox();
    expect(bounds).not.toBeNull();
    const point = { x: bounds!.x + bounds!.width - 30, y: bounds!.y + bounds!.height - 30 };
    const backdrop = page.getByTestId("app-menu-backdrop");
    const backdropBounds = await backdrop.boundingBox();
    expect(backdropBounds).not.toBeNull();
    const hit = await page.evaluate(({ x, y }) => document.elementFromPoint(x, y)?.getAttribute("data-testid"), point);
    await testInfo.attach(`${drawerId}-backdrop-hit`, { body: JSON.stringify({ point, drawer: bounds, backdrop: backdropBounds, hit }), contentType: "application/json" });
    expect(hit).toBe("app-menu-backdrop");
    await backdrop.click({ position: { x: point.x - backdropBounds!.x, y: point.y - backdropBounds!.y } });
    await expect(page.getByTestId("menu-dropdown-view")).toHaveCount(0);
    await expect(backdrop).toHaveCount(0);
    await expect(drawer).toBeVisible();
    await close.click();
    await expect(drawer).toHaveCount(0);
  });
}
