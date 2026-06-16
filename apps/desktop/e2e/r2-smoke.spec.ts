import { expect, test, type Page } from "@playwright/test";
import { readFileSync } from "node:fs";
import { inflateSync } from "node:zlib";

type RehearsalStep = {
  change_kind: string;
  payload: Record<string, any>;
  target?: {
    ref: string;
  };
};

type RehearsalFixture = {
  steps: RehearsalStep[];
};

const rehearsal = JSON.parse(
  readFileSync(new URL("../../../fixtures/product_preview/r2_from_blank_rehearsal.json", import.meta.url), "utf8")
) as RehearsalFixture;

// TP-APP-R2-UXSHELL-001: the shell keeps the spatial core (model tree |
// viewport | property inspector) persistent and organizes the remaining
// panels behind an always-visible workspace section navigation. The specs
// drive that navigation through its visible controls exactly as a human
// following SMOKE.md TP-MAC-141 would.
async function openWorkspaceSection(page: Page, sectionId: string): Promise<void> {
  await page.getByTestId(`workspace-nav-${sectionId}`).click();
  await expect(page.getByTestId(`workspace-section-${sectionId}`)).toBeVisible();
}

test("R2 desktop preview smoke covers solve, results, report, and viewport overlay", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByRole("heading", { name: "OpenPipeStress Technical Preview" })).toBeVisible();
  // Engine-ready guard (DEC-020 / ADR-0001): browser mode answers operations
  // through the wasm32 operation_applier build; wait for init before edits.
  await expect(page.getByTestId("operation-engine-status")).toContainText(
    "engine_route=local_wasm_engine; engine_state=ready"
  );
  await expect(page.getByTestId("property-unit-catalog-status")).toContainText(
    "browser preview uses model metadata"
  );
  await expect(page.getByTestId("property-unit-basis-summary")).toContainText("m, model metadata");
  await expect(page.getByTestId("property-unit-basis-summary")).toContainText("Pa, model metadata");
  await expect(page.getByTestId("viewport-deformation-status")).toContainText("not started; result rows=0");
  await expect(page.getByTestId("local-project-status")).toContainText("network=false");
  await expect(page.getByTestId("local-project-status")).toContainText("telemetry=false");
  await openWorkspaceSection(page, "loads");
  await expect(page.getByTestId("load-case-manager-summary")).toContainText(
    "2 load cases; 7 primitive loads; 1 combinations"
  );
  await expect(page.getByTestId("load-manager-create-load-id")).toHaveValue("load:L-300");
  await expect(page.getByTestId("load-manager-create-load-preview")).toContainText(
    "op:load-manager-create-load:L-300"
  );
  await expect(page.getByTestId("load-manager-create-load-preview")).toContainText("primitive_loads=0");
  await expect(page.getByTestId("load-manager-create-primitive-id")).toHaveValue("load:L-100-F300");
  await expect(page.getByTestId("load-manager-create-primitive-load-case")).toHaveValue("load:L-100");
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "op:load-manager-load:L-100-load:L-100-F300-primitive"
  );
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "target=node:N-100; direction=global_y; unit=N; force"
  );
  await page.getByTestId("load-manager-create-primitive-category").selectOption("distributed_force");
  await expect(page.getByTestId("load-manager-create-primitive-id")).toHaveValue("load:L-100-D300");
  await expect(page.getByTestId("load-manager-create-primitive-pipe")).toHaveValue("pipe:P-100");
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "op:load-manager-load:L-100-load:L-100-D300-primitive"
  );
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "target=pipe:P-100; direction=global_y; unit=N/m; force_per_length"
  );
  await page.getByTestId("load-manager-create-primitive-category").selectOption("concentrated_moment");
  await expect(page.getByTestId("load-manager-create-primitive-id")).toHaveValue("load:L-100-M300");
  await expect(page.getByTestId("load-manager-create-primitive-node")).toHaveValue("node:N-100");
  await expect(page.getByTestId("load-manager-create-primitive-direction")).toHaveValue("rotation_z");
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "op:load-manager-load:L-100-load:L-100-M300-primitive"
  );
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "target=node:N-100; direction=rotation_z; unit=N*m; moment"
  );
  await page.getByTestId("load-manager-create-primitive-category").selectOption("pressure");
  await expect(page.getByTestId("load-manager-create-primitive-id")).toHaveValue("load:L-100-P300");
  await expect(page.getByTestId("load-manager-create-primitive-pipe")).toHaveValue("pipe:P-100");
  await expect(page.getByTestId("load-manager-create-primitive-direction")).toHaveValue("global_x");
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "op:load-manager-load:L-100-load:L-100-P300-primitive"
  );
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "target=pipe:P-100; direction=global_x; unit=Pa; pressure"
  );
  await page.getByTestId("load-manager-create-primitive-category").selectOption("thermal");
  await expect(page.getByTestId("load-manager-create-primitive-id")).toHaveValue("load:L-100-T300");
  await expect(page.getByTestId("load-manager-create-primitive-pipe")).toHaveValue("pipe:P-100");
  await expect(page.getByTestId("load-manager-create-primitive-direction")).toHaveValue("global_z");
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "op:load-manager-load:L-100-load:L-100-T300-primitive"
  );
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "target=pipe:P-100; direction=global_z; unit=degC; temperature_interval"
  );
  await page.getByTestId("load-manager-create-primitive-category").selectOption("imposed_displacement");
  await expect(page.getByTestId("load-manager-create-primitive-id")).toHaveValue("load:L-100-I300");
  await expect(page.getByTestId("load-manager-create-primitive-support")).toHaveValue("support:S-100");
  await expect(page.getByTestId("load-manager-create-primitive-direction")).toHaveValue("UZ");
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "op:load-manager-load:L-100-load:L-100-I300-primitive"
  );
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "target=support:S-100; direction=UZ; unit=m; displacement"
  );
  await page.getByTestId("load-manager-create-primitive-direction").selectOption("RX");
  await expect(page.getByTestId("load-manager-create-primitive-preview")).toContainText(
    "target=support:S-100; direction=RX; unit=rad; rotation"
  );
  await expect(page.getByTestId("load-manager-create-combination-id")).toHaveValue("combination:C-300");
  await expect(page.getByTestId("load-manager-create-combination-preview")).toContainText(
    "op:load-manager-create-combination:C-300"
  );
  await expect(page.getByTestId("load-manager-create-combination-preview")).toContainText(
    "before=not_present; after=combination:C-300; term=load:L-100 x 1; unit=none; dimensionless"
  );
  await page.getByTestId("load-manager-create-combination-factor").fill("0.5");
  await expect(page.getByTestId("load-manager-create-combination-preview")).toContainText(
    "before=not_present; after=combination:C-300; term=load:L-100 x 0.5; unit=none; dimensionless"
  );
  await page.getByTestId("load-manager-primitive-load:L-100-P").click();
  await expect(page.getByTestId("load-manager-selected-primitive")).toContainText(
    "primitive_loads.2.magnitude.value"
  );
  await expect(page.getByTestId("load-manager-selected-case")).toContainText("field=status");
  await expect(page.getByTestId("load-manager-metadata-preview")).toContainText("current=preview_only");
  await page.getByTestId("load-manager-metadata-value").selectOption("TBD");
  await expect(page.getByTestId("load-manager-metadata-preview")).toContainText(
    "op:load-manager-load:L-100-status"
  );
  await page.getByTestId("load-manager-metadata-field").selectOption("kind");
  await expect(page.getByTestId("load-manager-selected-case")).toContainText(
    "field=kind; current=primitive_user_load"
  );
  await page.getByTestId("load-manager-metadata-value").selectOption("TBD");
  await expect(page.getByTestId("load-manager-metadata-preview")).toContainText(
    "op:load-manager-load:L-100-kind"
  );
  await expect(page.getByTestId("load-manager-load-case-delete-preview")).toContainText(
    "op:load-manager-load:L-100-delete"
  );
  await expect(page.getByTestId("load-manager-load-case-delete-preview")).toContainText(
    "before=load:L-100; Invented operating gravity and pressure preview; primitive_user_load; preview_only; primitives=4; after=not_present; unit=none; dimensionless"
  );
  await expect(page.getByTestId("load-manager-selected-combination")).toContainText(
    "field=basis; current=mechanics"
  );
  await expect(page.getByTestId("load-manager-combination-basis-preview")).toContainText("current=mechanics");
  // TP-APP-R2-COMBEXPR-001 behavior change: the basis editor is a closed-set
  // selector (mechanics | result_state_subtraction | range_envelope) instead
  // of free text; cross-shape edits stay honest proposals the engine blocks.
  await page.getByTestId("load-manager-combination-basis-value").selectOption("result_state_subtraction");
  await expect(page.getByTestId("load-manager-combination-basis-preview")).toContainText(
    "op:load-manager-combination:C-OPER-ALT-basis"
  );
  await expect(page.getByTestId("load-manager-combination-basis-preview")).toContainText(
    "before=mechanics; after=result_state_subtraction"
  );
  await expect(page.getByTestId("load-manager-combination-entity-delete-preview")).toContainText(
    "op:load-manager-combination:C-OPER-ALT-delete"
  );
  await expect(page.getByTestId("load-manager-combination-entity-delete-preview")).toContainText(
    "before=combination:C-OPER-ALT; Invented explicit operating plus alternate preview; basis=mechanics; terms=load:L-100 x 1; load:L-200 x 0.5"
  );
  await expect(page.getByTestId("load-manager-create-combination-term-heading")).toContainText(
    "combination:C-OPER-ALT"
  );
  await expect(page.getByTestId("load-manager-create-combination-term-preview")).toContainText(
    "op:load-manager-combination:C-OPER-ALT-term-2-create"
  );
  await expect(page.getByTestId("load-manager-create-combination-term-preview")).toContainText(
    "before=not_present; after=load:L-100 x 1; unit=none; dimensionless"
  );
  await page.getByTestId("load-manager-create-combination-term-factor").fill("0.25");
  await expect(page.getByTestId("load-manager-create-combination-term-preview")).toContainText(
    "before=not_present; after=load:L-100 x 0.25; unit=none; dimensionless"
  );
  await page.getByTestId("load-manager-combination-term-combination:C-OPER-ALT-1").click();
  await expect(page.getByTestId("load-manager-selected-combination-term")).toContainText("terms.1.factor");
  await expect(page.getByTestId("load-manager-combination-factor-preview")).toContainText("current=0.5");
  await page.getByTestId("load-manager-combination-factor-value").fill("0.75");
  await expect(page.getByTestId("load-manager-combination-factor-preview")).toContainText(
    "op:load-manager-combination:C-OPER-ALT-term-1-factor"
  );
  await expect(page.getByTestId("load-manager-combination-factor-preview")).toContainText(
    "before=0.5; after=0.75"
  );
  await expect(page.getByTestId("load-manager-combination-delete-preview")).toContainText(
    "op:load-manager-combination:C-OPER-ALT-term-1-delete"
  );
  await expect(page.getByTestId("load-manager-combination-delete-preview")).toContainText(
    "before=load:L-200 x 0.5; after=not_present; unit=none; dimensionless"
  );
  await page.getByTestId("tree-row-node:N-120").click();
  await expect(page.getByTestId("delete-node-intent-panel")).toContainText("delete_node");
  await expect(page.getByTestId("delete-node-intent-panel")).toContainText(
    "before=Riser elbow; x=3.2; y=2.4; z=0"
  );
  await expect(page.getByTestId("delete-node-intent-panel")).toContainText("after=not_present");
  await page.getByTestId("tree-row-pipe:P-130").click();
  await expect(page.getByTestId("delete-pipe-intent-panel")).toContainText("delete_pipe_run");
  await expect(page.getByTestId("delete-pipe-intent-panel")).toContainText(
    "before=Tie-in rise; node:N-130->node:N-140; material=material:invented-carbon-steel"
  );
  await expect(page.getByTestId("delete-pipe-intent-panel")).toContainText("after=not_present");

  const canvas = page.locator(".viewport-canvas canvas");
  await expect(canvas).toBeVisible();
  await canvas.click({ position: { x: 64, y: 64 } });
  await expect(page.getByTestId("viewport-create-node-id")).toHaveValue("node:V-001");
  await expect(page.getByTestId("viewport-create-node-label")).toHaveValue("Viewport node V-001");
  await expect(page.getByTestId("viewport-create-node-x")).toHaveValue(/^-?\d/);
  await expect(page.getByTestId("viewport-create-node-y")).toHaveValue("0");
  await expect(page.getByTestId("viewport-create-node-z")).toHaveValue(/^-?\d/);
  await expect(page.getByTestId("queue-explicit-node-intent")).toBeEnabled();

  await page.getByTestId("viewport-pick-pipe-from").click();
  await expect(page.getByTestId("viewport-pick-pipe-from")).toHaveAttribute("aria-pressed", "true");
  await page.getByTestId("viewport-select-node:N-100").click();
  await expect(page.getByTestId("viewport-create-pipe-from")).toHaveValue("node:N-100");
  await expect(page.getByTestId("viewport-pick-pipe-to")).toHaveAttribute("aria-pressed", "true");
  await page.getByTestId("viewport-select-node:N-140").click();
  await expect(page.getByTestId("viewport-create-pipe-to")).toHaveValue("node:N-140");
  await expect(page.getByTestId("viewport-pick-pipe-to")).toHaveAttribute("aria-pressed", "false");

  const before = await canvas.screenshot();
  await page.waitForTimeout(700);
  const after = await canvas.screenshot();
  expect(pngStats(before).uniqueColors).toBeGreaterThan(100);
  expect(diffPngPixels(before, after)).toBeGreaterThan(100);

  await openWorkspaceSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=737");
  await expect(page.getByTestId("viewport-deformation-status")).toContainText("available; nodes=5; max=33.211157 mm");
  await expect(page.getByTestId("viewport-deformation-boundary")).toContainText(
    "scale=normalized_display_offset_not_physical_length"
  );
  // TP-APP-R2-DEFORMEDDIR-001: the canned preview fixture now carries signed
  // global ux/uy/uz rows, so the overlay must disclose true directional
  // rendering instead of vector_direction=TBD.
  await expect(page.getByTestId("viewport-deformation-boundary")).toContainText(
    "vector_direction=global_cartesian_displacement_components"
  );

  const solvedCanvas = await canvas.screenshot();
  expect(pngStats(solvedCanvas).uniqueColors).toBeGreaterThan(100);

  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("results-panel")).toBeVisible();
  await expect(page.getByTestId("result-filter-summary")).toContainText("737 of 737 results match filter");
  await expect(page.getByTestId("result-family-count-reaction")).toContainText("9");
  await page.getByTestId("result-family-reaction").click();
  await expect(page.getByTestId("result-filter-summary")).toContainText("9 of 737 results match filter");
  await expect(page.getByTestId("result-page-summary")).toContainText(
    "Showing 1 to 9 of 9 matching results; page 1 of 1"
  );
  await expect(page.getByTestId("result-row-result:reaction:support-S-120")).toBeVisible();
  await page.getByTestId("result-family-all").click();
  await expect(page.getByTestId("result-filter-summary")).toContainText("737 of 737 results match filter");
  await page.getByTestId("result-filter-input").fill("pipe-P-120");
  await expect(page.getByTestId("result-filter-summary")).toContainText("167 of 737 results match filter");
  await expect(page.getByTestId("result-page-summary")).toContainText(
    "Showing 1 to 50 of 167 matching results; page 1 of 4"
  );
  await page.getByTestId("result-row-result:force:pipe-P-120:axial").click();
  await expect(page.getByTestId("result-detail-panel")).toContainText("pipe:P-120");
  await expect(page.getByTestId("result-detail-panel")).toContainText("recovered_from_local_element_stiffness");

  await openWorkspaceSection(page, "report");
  const report = page.getByLabel("Report packet");
  await expect(report).toContainText("run:preview-linear-static-001");
  await expect(report.getByTestId("report-unit-system")).toContainText(
    "unit-system:dec-018-si-dual-display"
  );
  await expect(report.getByTestId("report-unit-system")).toContainText("length=m");
  await expect(report.getByTestId("report-unit-system")).toContainText("conversion=false");
  const reportHref = await report.getByTestId("report-export-link").getAttribute("href");
  expect(reportHref).toBeTruthy();
  const reportPacket = JSON.parse(decodeURIComponent(reportHref!.split(",", 2)[1]));
  expect(reportPacket.document_kind).toBe("openpipestress.technical_preview.report_packet_export");
  expect(reportPacket.run_audit.analysis_run_ref.ref).toBe("run:preview-linear-static-001");
  expect(reportPacket.unit_system_disclosure.unit_system_ref.ref_id).toBe(
    "unit-system:dec-018-si-dual-display"
  );
  expect(reportPacket.unit_system_disclosure.conversion_performed).toBe(false);
  expect(reportPacket.private_payload_included).toBe(false);
  expect(reportPacket.protected_content_included).toBe(false);
  expect(reportPacket.release_or_professional_claim).toBe(false);

  // Engine-route receipt (TP-SEAM-SWAP-001): apply the prepared explicit
  // node intent through the structured-operation seam in a real browser and
  // verify the honest wasm-engine route on the applied receipt. Runs last:
  // applying clears earlier solve results by design.
  await page.getByTestId("queue-explicit-node-intent").click();
  await openWorkspaceSection(page, "operations");
  const applyPanel = page.getByTestId("operation-apply-panel");
  await expect(applyPanel.getByTestId("operation-apply-summary")).toContainText("1 queued; 0 applied");
  await page.getByTestId("apply-intent-editor-intent-1").click();
  await expect(applyPanel.getByTestId("applied-operation-route-applied-1-editor-intent-1")).toContainText(
    "route=local_wasm_engine"
  );
  await expect(applyPanel.getByTestId("operation-apply-summary")).toContainText("0 queued; 1 applied");
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=not_started");

  // TP-APP-R2-COMBEXPR-001: author a result_state_subtraction combination
  // through the visible create-form controls and apply it through the wasm
  // engine in browser mode.
  await openWorkspaceSection(page, "loads");
  await page.getByTestId("load-manager-create-combination-basis").selectOption("result_state_subtraction");
  await expect(page.getByTestId("load-manager-create-combination-minuend")).toHaveValue("load:L-100");
  await page.getByTestId("load-manager-create-combination-subtrahend").selectOption("load:L-200");
  await page.getByTestId("load-manager-create-combination-label").fill("Operating minus alternate subtraction");
  await expect(page.getByTestId("load-manager-create-combination-preview")).toContainText(
    "before=not_present; after=combination:C-300; minuend=load:L-100; subtrahend=load:L-200; unit=none; dimensionless"
  );
  await page.getByTestId("queue-create-combination-intent").click();
  await openWorkspaceSection(page, "operations");
  await expect(applyPanel.getByTestId("operation-apply-summary")).toContainText("1 queued; 1 applied");
  await page.getByTestId("apply-intent-editor-intent-2").click();
  await expect(applyPanel.getByTestId("applied-operation-route-applied-2-editor-intent-2")).toContainText(
    "route=local_wasm_engine"
  );
  await expect(applyPanel.getByTestId("operation-apply-summary")).toContainText("0 queued; 2 applied");
  await expect(page.getByTestId("load-case-manager-summary")).toContainText(
    "2 load cases; 7 primitive loads; 2 combinations"
  );
  await expect(page.getByTestId("load-manager-combination-combination:C-300")).toContainText(
    "basis=result_state_subtraction"
  );
  await expect(page.getByTestId("load-manager-combination-combination:C-300")).toContainText(
    "minuend=load:L-100; subtrahend=load:L-200"
  );
});

test("R2 from-blank GUI journey authors the A12 rehearsal script", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("operation-engine-status")).toContainText(
    "engine_route=local_wasm_engine; engine_state=ready"
  );
  await page.getByRole("button", { name: "New blank" }).click();
  await expect(page.getByTestId("local-project-message")).toContainText(
    "Created blank local model document without fixture entities or external file copies."
  );
  await expect(page.getByTestId("load-case-manager-summary")).toContainText(
    "0 load cases; 0 primitive loads; 0 combinations"
  );

  const startNode = stepPayload("create_node", "node:R2-100");
  await fillNodeDraft(page, startNode);
  await page.getByTestId("queue-explicit-node-intent").click();
  await applyQueuedIntent(page, 1, startNode.id);

  const loadedNode = stepPayload("create_node", "node:R2-110");
  await fillNodeDraft(page, loadedNode);
  await page.getByTestId("queue-explicit-node-intent").click();
  await applyQueuedIntent(page, 2, loadedNode.id);

  const material = stepPayload("create_material", "material:r2-carbon-steel");
  await page.getByTestId("create-material-id").fill(material.id);
  await page.getByTestId("create-material-label").fill(material.label);
  await page.getByTestId("create-material-elastic").fill(String(material.elastic_modulus.value));
  await page.getByTestId("create-material-shear").fill(String(material.shear_modulus.value));
  await page.getByTestId("create-material-provenance").fill(material.provenance);
  await page.getByTestId("queue-create-material-intent").click();
  await applyQueuedIntent(page, 3, material.id);

  const section = stepPayload("create_section", "section:r2-pipe");
  await page.getByTestId("create-section-id").fill(section.id);
  await page.getByTestId("create-section-name").fill(section.name);
  await page.getByTestId("create-section-od").fill(String(section.properties.outside_diameter.value));
  await page.getByTestId("create-section-wall").fill(String(section.properties.wall_thickness.value));
  await page.getByTestId("create-section-provenance").fill(section.provenance);
  await page.getByTestId("queue-create-section-intent").click();
  await applyQueuedIntent(page, 4, section.id);

  const pipe = stepPayload("connect_pipe_run", "pipe:R2-100");
  await page.getByTestId("viewport-create-pipe-id").fill(pipe.id);
  await page.getByTestId("viewport-create-pipe-label").fill(pipe.label);
  await page.getByTestId("viewport-create-pipe-from").selectOption(pipe.from);
  await page.getByTestId("viewport-create-pipe-to").selectOption(pipe.to);
  await page.getByTestId("viewport-create-pipe-material").selectOption(pipe.material);
  await page.getByTestId("viewport-create-pipe-od").fill(String(pipe.section.outside_diameter.value));
  await page.getByTestId("viewport-create-pipe-wall").fill(String(pipe.section.wall_thickness.value));
  await page.getByTestId("viewport-create-pipe-yref-x").fill(String(pipe.y_reference.x));
  await page.getByTestId("viewport-create-pipe-yref-y").fill(String(pipe.y_reference.y));
  await page.getByTestId("viewport-create-pipe-yref-z").fill(String(pipe.y_reference.z));
  await page.getByTestId("viewport-create-pipe-provenance").fill(pipe.provenance);
  await page.getByTestId("queue-explicit-pipe-intent").click();
  await applyQueuedIntent(page, 5, pipe.id);

  const support = stepPayload("create_support", "support:R2-anchor");
  await page.getByTestId("create-support-id").fill(support.id);
  await page.getByTestId("create-support-label").fill(support.label);
  await page.getByTestId("create-support-node").selectOption(support.node);
  for (const restraint of ["RX", "RY", "RZ"]) {
    await page.getByTestId(`create-support-restraint-${restraint}`).setChecked(true);
  }
  await page.getByTestId("create-support-provenance").fill(support.provenance);
  await page.getByTestId("queue-create-support-intent").click();
  await applyQueuedIntent(page, 6, support.id);

  const loadCase = stepPayload("create_load_case", "load:R2-L-100");
  await openWorkspaceSection(page, "loads");
  await page.getByTestId("load-manager-create-load-id").fill(loadCase.id);
  await page.getByTestId("load-manager-create-load-label").fill(loadCase.label);
  await page.getByTestId("load-manager-create-load-kind").fill(loadCase.kind);
  await page.getByTestId("load-manager-create-load-status").fill(loadCase.status);
  await page.getByTestId("load-manager-create-load-provenance").fill(loadCase.provenance);
  await page.getByTestId("queue-create-load-case-intent").click();
  await applyQueuedIntent(page, 7, loadCase.id);

  const primitive = stepPayload("create_primitive_load", "load:R2-L-100-FY");
  await openWorkspaceSection(page, "loads");
  await page.getByTestId("load-manager-create-primitive-load-case").selectOption(loadCase.id);
  await page.getByTestId("load-manager-create-primitive-category").selectOption(primitive.category);
  await page.getByTestId("load-manager-create-primitive-id").fill(primitive.id);
  await page.getByTestId("load-manager-create-primitive-node").selectOption(primitive.target.node);
  await page.getByTestId("load-manager-create-primitive-direction").selectOption(primitive.direction);
  await page.getByTestId("load-manager-create-primitive-magnitude").fill(String(primitive.magnitude.value));
  await page.getByTestId("load-manager-create-primitive-provenance").fill(primitive.provenance);
  await page.getByTestId("queue-create-primitive-intent").click();
  await applyQueuedIntent(page, 8, primitive.id);

  const combination = stepPayload("create_combination", "combination:R2-C-100");
  await openWorkspaceSection(page, "loads");
  await page.getByTestId("load-manager-create-combination-id").fill(combination.id);
  await page.getByTestId("load-manager-create-combination-label").fill(combination.label);
  await page.getByTestId("load-manager-create-combination-load-case").selectOption(combination.terms[0].load_case);
  await page.getByTestId("load-manager-create-combination-factor").fill(String(combination.terms[0].factor));
  await page.getByTestId("load-manager-create-combination-provenance").fill(combination.provenance);
  await page.getByTestId("load-manager-create-combination-rationale").fill("A8 GUI replay of the A12 invented rehearsal.");
  await page.getByTestId("queue-create-combination-intent").click();
  await applyQueuedIntent(page, 9, combination.id);

  await expect(page.getByTestId("load-case-manager-summary")).toContainText(
    "1 load cases; 1 primitive loads; 1 combinations"
  );
  await openWorkspaceSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=0");
  await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toContainText(
    "BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL"
  );
  await openWorkspaceSection(page, "report");
  await expect(page.getByTestId("rendered-report-render")).toBeEnabled();
  await page.getByTestId("rendered-report-render").click();
  await expect(page.getByTestId("rendered-report-route")).toContainText("REPORT-RENDERER-DESKTOP-ONLY");
});

// Phase C2 slice 1 (TP-C2-EDITOR-001): the rule-pack manager authors a
// private draft in memory and reports the honest desktop-only seam for
// validation, checksum, persistence, and listing in browser mode — the
// same recorded boundary pattern as report rendering and the unit catalog.
test("rule-pack manager drafts privately and reports the desktop-only backend seam", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("operation-engine-status")).toContainText(
    "engine_route=local_wasm_engine; engine_state=ready"
  );

  await openWorkspaceSection(page, "rule-packs");
  await expect(page.getByTestId("rule-pack-scope-status")).toContainText("local SQLite only");
  await expect(page.getByTestId("rule-pack-boundary-note")).toContainText("D-02b");

  await page.getByTestId("rule-pack-new-draft").click();
  const draftText = await page.getByTestId("rule-pack-draft-json").inputValue();
  const draft = JSON.parse(draftText);
  expect(draft.rule_pack_kind).toBe("open_pipe_stress_rule_pack");
  expect(draft.grammar_version).toBe("1.0.0");
  expect(draft.classification.privacy_class).toBe("private_user_data");
  expect(draft.classification.redistribution_status).toBe("private_only");
  await expect(page.getByTestId("rule-pack-action-status")).toContainText("private_user_data");

  // Slice 2 (TP-C2-COMPOSER-001) + DEC-037 follow-up: the structured AST
  // composer rewrites the selected formula's expression through visible
  // controls. It may render display-only text, but ships no writable text
  // syntax and no parser. Switching the root node type rewrites the canonical
  // document JSON the validate/save flow reads.
  await expect(page.getByTestId("rule-pack-expression-composer")).toBeVisible();
  await expect(page.getByTestId("rule-pack-variable-browser")).toContainText("user_required_input_1");
  await expect(page.getByTestId("rule-pack-expression-rendering")).toContainText(
    "Display only; not input; notation non-frozen."
  );
  await expect(page.getByTestId("rule-pack-expression-rendering")).toContainText("user_required_input_1");
  expect(draft.formula_declarations[0].declaration_payload.expression_ast.node).toBe("variable_ref");

  // Slice 4 (TP-C2-DECLEDITOR-001): the declarations editor authors the
  // required_inputs / value_slots the composer's variable_ref binds to. Add a
  // required input from blank; the canonical document JSON grows and the
  // composer's variable picker reflects the new id — no raw JSON. Still
  // structured-only (D-02b).
  await expect(page.getByTestId("rule-pack-declarations-editor")).toBeVisible();
  await page.getByTestId("rule-pack-input-add").click();
  // TP-UNITS-B2-RULEPACKUNITS-001: browser preview cannot call the desktop
  // get_unit_catalog command, so declaration unit refs stay editable as stored
  // unit text here. The desktop catalog-picker path is covered by mocked-Tauri
  // Vitest; this e2e assertion protects the no-fallback/manual-entry route.
  await page.getByTestId("rule-pack-input-dimension").last().selectOption("stress");
  await page.getByTestId("rule-pack-input-unit").last().fill("MPa");
  const declText = await page.getByTestId("rule-pack-draft-json").inputValue();
  expect(JSON.parse(declText).required_inputs).toHaveLength(2);
  expect(JSON.parse(declText).required_inputs[1].quantity_intent).toMatchObject({
    dimension: "stress",
    unit_ref: "MPa"
  });
  await expect(page.getByTestId("rule-pack-variable-browser")).toContainText(
    "user_required_input_2 (required_input)"
  );
  await page.getByTestId("rule-pack-node-type").first().selectOption("compare");
  // TP-UNITS-B2-RULEEXPRUNITS-001: browser preview cannot call the desktop
  // catalog command for expression literal/table units either, so expression
  // unit refs remain manual stored-unit text in this route. Mocked-Tauri
  // Vitest covers the desktop catalog-selector path.
  await page.getByTestId("rule-pack-literal-dimension").selectOption("stress");
  await page.getByTestId("rule-pack-literal-unit").fill("MPa");
  await expect(page.getByTestId("rule-pack-expression-rendering")).toContainText(
    "(user_required_input_1 <= 0 MPa [stress])"
  );
  const composedText = await page.getByTestId("rule-pack-draft-json").inputValue();
  expect(JSON.parse(composedText).formula_declarations[0].declaration_payload.expression_ast).toMatchObject({
    node: "compare",
    operator: "less_than_or_equal",
    left: { node: "variable_ref" },
    right: { node: "literal", quantity: { dimension: "stress", unit_ref: "MPa" } }
  });

  // Slice 3 (TP-C2-TABLENODE-001): switch the root to a table-backed
  // interpolate node and edit a row. The canonical document JSON updates with
  // a schema-valid default table (uppercase "TBD" placeholders, two
  // strictly-increasing rows) plus the edited result. Still AST-only (D-02b).
  await page.getByTestId("rule-pack-node-type").first().selectOption("interpolate");
  await page.getByTestId("rule-pack-table-argument-dimension").selectOption("temperature");
  await page.getByTestId("rule-pack-table-argument-unit").fill("degC");
  await page.getByTestId("rule-pack-table-result-dimension").selectOption("stress");
  await page.getByTestId("rule-pack-table-result-unit").fill("MPa");
  await page.getByTestId("rule-pack-table-row-result").first().fill("1.5");
  await expect(page.getByTestId("rule-pack-expression-rendering")).toContainText(
    "interpolate(user_table_1, user_required_input_1)"
  );
  const tableText = await page.getByTestId("rule-pack-draft-json").inputValue();
  expect(JSON.parse(tableText).formula_declarations[0].declaration_payload.expression_ast).toMatchObject({
    node: "interpolate",
    table: {
      table_id: "user_table_1",
      argument_dimension: "temperature",
      argument_unit_ref: "degC",
      result_dimension: "stress",
      result_unit_ref: "MPa",
      rows: [
        { argument: 0, result: 1.5 },
        { argument: 1, result: 0 }
      ]
    },
    argument: { node: "variable_ref" }
  });

  // Slice 5 (TP-C2-CHECKDEF-001): the check-definitions editor binds the
  // declared inputs/slots/formula into acceptability checks through visible
  // controls — no text syntax (D-02b). The template check's formula ref is
  // bound to the declared formula; add a check and the canonical document JSON
  // grows; toggling a result status rewrites it.
  await expect(page.getByTestId("rule-pack-check-definitions-editor")).toBeVisible();
  await expect(page.getByTestId("rule-pack-check-formula-ref").first()).toHaveValue(
    "user_formula_1"
  );
  await page.getByTestId("rule-pack-check-add").click();
  const checkText = await page.getByTestId("rule-pack-draft-json").inputValue();
  expect(JSON.parse(checkText).check_definitions).toHaveLength(2);
  await page.getByTestId("rule-pack-check-status-MODEL_INCOMPLETE").first().check();
  const statusText = await page.getByTestId("rule-pack-draft-json").inputValue();
  expect(JSON.parse(statusText).check_definitions[0].result_statuses).toContain("MODEL_INCOMPLETE");

  // TP-C4-ACCEPTREL-001: the check authors its top-level acceptability relation
  // (computed <relation> limit) through a structured selector — the template
  // authors the explicit less_than_or_equal default (no reliance on the runner's
  // absent->default fallback); choosing another ordering relation rewrites the
  // canonical document. No text syntax (D-02b); equality acceptance is not offered.
  await expect(page.getByTestId("rule-pack-check-acceptability-relation").first()).toHaveValue(
    "less_than_or_equal"
  );
  await page
    .getByTestId("rule-pack-check-acceptability-relation")
    .first()
    .selectOption("greater_than_or_equal");
  const relationText = await page.getByTestId("rule-pack-draft-json").inputValue();
  expect(JSON.parse(relationText).check_definitions[0].acceptability_relation).toBe(
    "greater_than_or_equal"
  );

  // TP-C3-LIBREFAUTHOR-001: an input that draws from a private library authors a
  // library_value_ref (library_kind/library_id/record_id/slot_id) through
  // structured controls — the pointer the C4 runner resolves from the local
  // library store at run time (TP-C3C4-LIBREF-001). Setting source_kind to
  // private_library_value reveals the sub-form and seeds a complete reference;
  // the private value itself is never embedded in the pack. Still form-only
  // (D-02b); no raw JSON. Target the appended (second) input row via .last().
  await page.getByTestId("rule-pack-input-source-kind").last().selectOption("private_library_value");
  await expect(page.getByTestId("rule-pack-input-library-ref")).toBeVisible();
  await expect(page.getByTestId("rule-pack-input-library-ref-note")).toContainText(
    "never embedded in the rule pack"
  );
  await page.getByTestId("rule-pack-input-library-kind").selectOption("material");
  await page.getByTestId("rule-pack-input-library-id").fill("my_private_lib");
  await page.getByTestId("rule-pack-input-library-record").fill("rec_A106B");
  await page.getByTestId("rule-pack-input-library-slot").fill("allowable_stress");
  const libRefText = await page.getByTestId("rule-pack-draft-json").inputValue();
  const libRefInputs = JSON.parse(libRefText).required_inputs;
  expect(libRefInputs[libRefInputs.length - 1]).toMatchObject({
    source_kind: "private_library_value",
    library_value_ref: {
      library_kind: "material",
      library_id: "my_private_lib",
      record_id: "rec_A106B",
      slot_id: "allowable_stress"
    }
  });

  // TP-C4-SOLVERREFAUTHOR-001: an input that draws from a solved result authors
  // a solver_result_ref (a single result_id) through a structured control — the
  // in-pack, canonical form of the previously caller-supplied {input_id,
  // result_id} solver binding (PRD §12.5), resolved at run time from the solved
  // envelope's results[] by id (TP-C4-SOLVERREF-001). Add a fresh input and set
  // source_kind to solver_result: the sub-form reveals and seeds a
  // {result_id:"TBD"} placeholder (unfilled -> blocks, never a silent pass); the
  // authored reference supersedes the run panel's per-input selector. Still
  // form-only (D-02b); no raw JSON. Target the appended (last) input row.
  await page.getByTestId("rule-pack-input-add").click();
  await page.getByTestId("rule-pack-input-source-kind").last().selectOption("solver_result");
  await expect(page.getByTestId("rule-pack-input-solver-ref")).toBeVisible();
  await expect(page.getByTestId("rule-pack-input-solver-ref-note")).toContainText(
    "supersedes the run panel"
  );
  await page.getByTestId("rule-pack-input-solver-result-id").fill("result:stress:demo");
  const solverRefText = await page.getByTestId("rule-pack-draft-json").inputValue();
  const solverRefInputs = JSON.parse(solverRefText).required_inputs;
  expect(solverRefInputs[solverRefInputs.length - 1]).toMatchObject({
    source_kind: "solver_result",
    solver_result_ref: { result_id: "result:stress:demo" }
  });

  // Validate routes to the desktop-only seam: the status transitions away
  // from the just-created "private_user_data" draft message. (Per-button
  // seam coverage for compute-checksum and save lives in the Vitest panel
  // suite, where each transition is asserted from a distinct prior state;
  // asserting them here too would be vacuous — all three write the same
  // action-status text.)
  await page.getByTestId("rule-pack-validate").click();
  await expect(page.getByTestId("rule-pack-action-status")).toContainText(
    "RULE-PACK-BACKEND-DESKTOP-ONLY"
  );

  // Refresh reports on a distinct surface (list-status), so this assertion
  // is independent of the validate result above.
  await page.getByTestId("rule-pack-refresh-list").click();
  await expect(page.getByTestId("rule-pack-list-status")).toContainText(
    "RULE-PACK-BACKEND-DESKTOP-ONLY"
  );

  await page.getByTestId("rule-pack-discard-draft").click();
  await expect(page.getByTestId("rule-pack-action-status")).toContainText("Draft discarded");
});

test("library manager loads an invented private sample and reports the desktop-only backend seam", async ({
  page
}) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("operation-engine-status")).toContainText(
    "engine_route=local_wasm_engine; engine_state=ready"
  );

  await openWorkspaceSection(page, "libraries");
  await expect(page.getByTestId("library-scope-status")).toContainText("local SQLite only");
  await expect(page.getByTestId("library-boundary-note")).toContainText("DEC-036");
  await expect(page.getByTestId("library-boundary-note")).toContainText(
    "never committed to the repository"
  );

  // Phase C3 (TP-C3-LIBGUI-001): load the invented private-by-default sample
  // for the default (material) kind through visible controls. The canonical
  // import document populates the textarea with a private-classified,
  // provenance-complete library.
  await page.getByTestId("library-load-template").click();
  const materialText = await page.getByTestId("library-draft-json").inputValue();
  const material = JSON.parse(materialText);
  expect(material.material_library.privacy_class).toBe("private_user_data");
  expect(material.material_library.provenance.redistribution_status).toBe("private_only");
  expect(material.material_records).toEqual([]);
  await expect(page.getByTestId("library-action-status")).toContainText("private_user_data");

  // Switching the kind selector and reloading yields the matching library
  // family shape — the component library here.
  await page.getByTestId("library-kind-select").selectOption("component");
  await page.getByTestId("library-load-template").click();
  const componentText = await page.getByTestId("library-draft-json").inputValue();
  expect(JSON.parse(componentText).component_library).toBeTruthy();

  // Validate routes to the desktop-only seam in browser preview: the
  // validate/save/store backend runs in the Tauri runtime only. Accept/store
  // and §13.5 findings outcomes are covered by the src-tauri Rust tests and
  // the Vitest desktop-mode mocked panel suite, not by this browser lane.
  await page.getByTestId("library-validate").click();
  await expect(page.getByTestId("library-action-status")).toContainText(
    "LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY"
  );

  // Refresh reports on a distinct surface (list-status), independent of the
  // validate result above.
  await page.getByTestId("library-refresh-list").click();
  await expect(page.getByTestId("library-list-status")).toContainText(
    "LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY"
  );

  await page.getByTestId("library-discard-draft").click();
  await expect(page.getByTestId("library-action-status")).toContainText("discarded");
});

test("run-rule-checks panel loads the demo pack, derives bindings, and reports the desktop-only run seam", async ({
  page
}) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();

  // The run-checks panel lives in the Solve section (it consumes solved
  // results). Phase C4 GUI (TP-C4-CHECKGUI-001): make the R3 exit criterion
  // GUI-true — load a pack, bind inputs, run, see pass/fail/blocked.
  await openWorkspaceSection(page, "solve");
  await expect(page.getByTestId("rule-check-run-panel")).toBeVisible();
  await expect(page.getByTestId("rule-check-boundary-note")).toContainText("code-compliance");

  // Load the bundled invented demo pack through a visible control; the binding
  // plan is derived from the loaded document.
  await page.getByTestId("rule-check-load-demo").click();
  await expect(page.getByTestId("rule-check-binding-plan")).toBeVisible();
  const packText = await page.getByTestId("rule-check-pack-json").inputValue();
  expect(JSON.parse(packText).metadata.rule_pack_id).toBe("invented_demo_rule_pack");

  // solver_result input -> result-row select; user value input + value slot ->
  // entry fields.
  await expect(page.getByTestId("rule-check-solver-select-demo_actual_quantity")).toBeVisible();
  await expect(page.getByTestId("rule-check-value-input-demo_limit_quantity")).toBeVisible();
  await expect(page.getByTestId("rule-check-slot-input-demo_limit_slot")).toBeVisible();

  // Running routes to the desktop-only seam in browser preview: the runner
  // (completeness gate + frozen-grammar evaluation + acceptability comparison)
  // runs in the Tauri runtime only. Pass/fail/blocked outcomes are covered by
  // the src-tauri Rust command tests and the Vitest desktop-mode mocked panel
  // suite, not by this browser lane.
  await page.getByTestId("rule-check-run").click();
  await expect(page.getByTestId("rule-check-run-status")).toContainText(
    "RULE-CHECK-BACKEND-DESKTOP-ONLY"
  );
  await expect(page.getByTestId("rule-check-run-result")).toHaveCount(0);

  // Refresh reports on a distinct surface (list-status), independent of the
  // run result above.
  await page.getByTestId("rule-check-refresh-list").click();
  await expect(page.getByTestId("rule-check-run-list-status")).toContainText(
    "RULE-PACK-BACKEND-DESKTOP-ONLY"
  );

  // TP-C3-LIBREFPICKER-001: a private_library_value input gets a
  // read-only-to-the-pack resolution-preview picker. The browser-reachable part
  // — the "Preview resolution" control and its honest desktop-only store seam —
  // is asserted here; the actual resolution/browse against the local private
  // library store is desktop-only (covered by the Vitest desktop-mode mocked
  // suite), so in the browser the preview reports the unavailable seam.
  const libraryPack = JSON.stringify({
    metadata: { rule_pack_id: "lib_ref_demo" },
    required_inputs: [
      {
        input_id: "lib_allow",
        name: "Library allowable",
        source_kind: "private_library_value",
        quantity_intent: { dimension: "stress", unit_ref: "demo_unit" },
        library_value_ref: {
          library_kind: "material",
          library_id: "lib:steel",
          record_id: "mat:a",
          slot_id: "allow:Sh"
        }
      }
    ]
  });
  await page.getByTestId("rule-check-pack-json").fill(libraryPack);
  await expect(page.getByTestId("rule-check-library-input-lib_allow")).toBeVisible();
  await page.getByTestId("rule-check-library-preview-lib_allow").click();
  await expect(page.getByTestId("rule-check-library-resolution-lib_allow")).toContainText(
    "LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY"
  );

  // TP-C4-SOLVERREFPICKER-001: a solver_result input with an authored
  // solver_result_ref gets a read-only preview against the current solved
  // envelope. In this browser smoke path no solved rows are present yet, so the
  // preview honestly reports no_result_rows instead of inventing a binding. The
  // resolving solved-envelope case is pinned in the component tests.
  const solverRefPack = JSON.stringify({
    metadata: { rule_pack_id: "solver_ref_demo" },
    required_inputs: [
      {
        input_id: "actual_stress",
        name: "Actual stress",
        source_kind: "solver_result",
        quantity_intent: { dimension: "stress", unit_ref: "kPa" },
        solver_result_ref: { result_id: "result:stress:pipe-P-100:end-i:axial-normal" }
      }
    ]
  });
  await page.getByTestId("rule-check-pack-json").fill(solverRefPack);
  await expect(page.getByTestId("rule-check-solver-input-actual_stress")).toContainText(
    "supersedes the run-panel selector"
  );
  await page.getByTestId("rule-check-solver-preview-actual_stress").click();
  await expect(page.getByTestId("rule-check-solver-resolution-actual_stress")).toHaveAttribute(
    "data-status",
    "no_result_rows"
  );
  await expect(page.getByTestId("rule-check-solver-browse-actual_stress")).toContainText(
    "Run a solve first"
  );
});

function stepPayload(changeKind: string, ref: string): any {
  const step = rehearsal.steps.find(
    (candidate) =>
      candidate.change_kind === changeKind && (candidate.target?.ref === ref || candidate.payload.id === ref)
  );
  if (!step) throw new Error(`missing rehearsal step ${changeKind} ${ref}`);
  return step.payload;
}

async function fillNodeDraft(page: Page, payload: any): Promise<void> {
  await page.getByTestId("viewport-create-node-id").fill(payload.id);
  await page.getByTestId("viewport-create-node-label").fill(payload.label);
  await page.getByTestId("viewport-create-node-x").fill(String(payload.position.x));
  await page.getByTestId("viewport-create-node-y").fill(String(payload.position.y));
  await page.getByTestId("viewport-create-node-z").fill(String(payload.position.z));
}

async function applyQueuedIntent(page: Page, sequence: number, expectedOperation: string): Promise<void> {
  // Authoring forms live in the persistent core or the Load Cases section;
  // applying always goes through the Operation Apply section, like the
  // TP-MAC-141 manual journey.
  await openWorkspaceSection(page, "operations");
  const key = `editor-intent-${sequence}`;
  const row = page.getByTestId(`operation-apply-row-${key}`);
  await expect(row).toContainText(expectedOperation);
  await page.getByTestId(`apply-intent-${key}`).click();
  await expect(page.getByTestId("operation-apply-summary")).toContainText(`0 queued; ${sequence} applied`);
  await expect(page.getByTestId(`applied-operation-route-applied-${sequence}-${key}`)).toContainText(
    "route=local_wasm_engine"
  );
  await expect(page.getByTestId(`applied-operation-route-applied-${sequence}-${key}`)).toContainText(
    "professional_approval=false"
  );
}

type PngImage = {
  bytesPerPixel: number;
  data: Uint8Array;
  height: number;
  width: number;
};

function pngStats(buffer: Buffer): { height: number; opaquePixels: number; uniqueColors: number; width: number } {
  const image = decodePng(buffer);
  const colors = new Set<string>();
  let opaquePixels = 0;
  for (let offset = 0; offset < image.data.length; offset += image.bytesPerPixel) {
    const alpha = image.bytesPerPixel === 4 ? image.data[offset + 3] : 255;
    if (alpha > 0) opaquePixels += 1;
    if (colors.size < 2_000) {
      colors.add(Array.from(image.data.slice(offset, offset + image.bytesPerPixel)).join(","));
    }
  }
  return { height: image.height, opaquePixels, uniqueColors: colors.size, width: image.width };
}

function diffPngPixels(leftBuffer: Buffer, rightBuffer: Buffer): number {
  const left = decodePng(leftBuffer);
  const right = decodePng(rightBuffer);
  const compared = Math.min(left.data.length, right.data.length);
  const bytesPerPixel = Math.min(left.bytesPerPixel, right.bytesPerPixel);
  let changed = 0;
  for (let offset = 0; offset + bytesPerPixel <= compared; offset += bytesPerPixel) {
    const channelDelta =
      Math.abs(left.data[offset] - right.data[offset]) +
      Math.abs(left.data[offset + 1] - right.data[offset + 1]) +
      Math.abs(left.data[offset + 2] - right.data[offset + 2]);
    if (channelDelta > 6) changed += 1;
  }
  return changed;
}

function decodePng(buffer: Buffer): PngImage {
  const signature = buffer.subarray(0, 8).toString("hex");
  if (signature !== "89504e470d0a1a0a") throw new Error("not a PNG");

  let offset = 8;
  let width = 0;
  let height = 0;
  let colorType = -1;
  const idatChunks: Buffer[] = [];

  while (offset < buffer.length) {
    const length = buffer.readUInt32BE(offset);
    const type = buffer.subarray(offset + 4, offset + 8).toString("ascii");
    const data = buffer.subarray(offset + 8, offset + 8 + length);
    if (type === "IHDR") {
      width = data.readUInt32BE(0);
      height = data.readUInt32BE(4);
      const bitDepth = data.readUInt8(8);
      colorType = data.readUInt8(9);
      if (bitDepth !== 8 || ![2, 6].includes(colorType)) {
        throw new Error(`unsupported PNG format: bitDepth=${bitDepth}, colorType=${colorType}`);
      }
    } else if (type === "IDAT") {
      idatChunks.push(Buffer.from(data));
    } else if (type === "IEND") {
      break;
    }
    offset += length + 12;
  }

  const bytesPerPixel = colorType === 6 ? 4 : 3;
  const stride = width * bytesPerPixel;
  const inflated = inflateSync(Buffer.concat(idatChunks));
  const data = new Uint8Array(height * stride);
  let sourceOffset = 0;

  for (let row = 0; row < height; row += 1) {
    const filter = inflated[sourceOffset];
    sourceOffset += 1;
    const raw = inflated.subarray(sourceOffset, sourceOffset + stride);
    sourceOffset += stride;
    const rowOffset = row * stride;
    const previousRowOffset = rowOffset - stride;
    for (let index = 0; index < stride; index += 1) {
      const left = index >= bytesPerPixel ? data[rowOffset + index - bytesPerPixel] : 0;
      const up = row > 0 ? data[previousRowOffset + index] : 0;
      const upLeft = row > 0 && index >= bytesPerPixel ? data[previousRowOffset + index - bytesPerPixel] : 0;
      data[rowOffset + index] = (raw[index] + filterValue(filter, left, up, upLeft)) & 0xff;
    }
  }

  return { bytesPerPixel, data, height, width };
}

function filterValue(filter: number, left: number, up: number, upLeft: number): number {
  if (filter === 0) return 0;
  if (filter === 1) return left;
  if (filter === 2) return up;
  if (filter === 3) return Math.floor((left + up) / 2);
  if (filter === 4) return paeth(left, up, upLeft);
  throw new Error(`unsupported PNG filter: ${filter}`);
}

function paeth(left: number, up: number, upLeft: number): number {
  const estimate = left + up - upLeft;
  const leftDistance = Math.abs(estimate - left);
  const upDistance = Math.abs(estimate - up);
  const upLeftDistance = Math.abs(estimate - upLeft);
  if (leftDistance <= upDistance && leftDistance <= upLeftDistance) return left;
  if (upDistance <= upLeftDistance) return up;
  return upLeft;
}
