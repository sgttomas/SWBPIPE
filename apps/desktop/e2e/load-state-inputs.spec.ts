import { expect, test, type Page } from "@playwright/test";
import {
  chooseVirtualTarget,
  closeWorkspacePanels,
  expectTreeEntity,
  openWorkspaceSection,
} from "./workspace-driver";

// T1 WP3: plain model 0.4.0 load/reference-state fields on the browser lane.
// Every value is an invented test input. The browser has no solver backend, so
// no e2e here shows a 0.4.0 solve: the resolved-state block for the committed
// raws is proven in vitest (LoadReferenceStatesBlock.test.tsx), and the native
// solve and block display remain the owner's-Mac witness.
const P = "invented_wp3_ui_acceptance_input";

async function modelHash(page: Page): Promise<string> {
  await openWorkspaceSection(page, "project");
  const intent = page.getByTestId("project-validation-export-link-local-private-intent");
  if (!await intent.isChecked()) await intent.check();
  const link = page.getByTestId("project-validation-export-link");
  const read = async () => {
    const href = await link.getAttribute("href");
    return href ? JSON.parse(decodeURIComponent(href.split(",").slice(1).join(","))).model_hash?.value ?? "" : "";
  };
  await expect.poll(read).toMatch(/^sha256:[0-9a-f]{64}$/);
  const hash = await read();
  await closeWorkspacePanels(page);
  return hash;
}

async function applyQueued(page: Page) {
  await openWorkspaceSection(page, "operations");
  const apply = page.locator('[data-testid^="apply-intent-"]').first();
  await expect(apply).toBeEnabled();
  await apply.click();
  await expect(page.getByTestId("operation-apply-message")).toContainText("Applied");
  await closeWorkspacePanels(page);
}

async function createNode(page: Page, id: string, x: string) {
  const idField = page.getByTestId("viewport-create-node-id");
  // The node tool stays armed after an apply; the command then toggles it.
  if (!await idField.isVisible()) await page.getByTestId("command-node").click();
  await expect(idField).toBeVisible();
  await idField.fill(id);
  await page.getByTestId("viewport-create-node-label").fill(id);
  await page.getByTestId("viewport-create-node-x").fill(x);
  await page.getByTestId("viewport-create-node-y").fill("0");
  await page.getByTestId("viewport-create-node-z").fill("0");
  await page.getByTestId("viewport-create-node-provenance").fill(P);
  await page.getByTestId("queue-explicit-node-intent").click();
  await page.getByTestId("apply-reviewed-draft").click();
  await expectTreeEntity(page, "node", id);
}

test("blank 0.4.0: author support motion, a thermal state and a material selection through the fields; Review/Apply, Undo/Redo restore the bytes", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await openWorkspaceSection(page, "operations");
  await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");

  // The pre-0.4 bundled fixture offers only the gating line.
  await openWorkspaceSection(page, "loads");
  await expect(page.getByTestId("load-state-needs-040")).toHaveText("Load/reference state needs a 0.4.0 model.");
  await expect(page.getByTestId("load-state-inputs")).toHaveCount(0);

  await openWorkspaceSection(page, "project");
  await page.getByRole("button", { name: "Blank 0.4.0 model", exact: true }).click();
  await expect(page.getByTestId("local-project-message")).toContainText("Created blank local model 0.4.0 document");

  // Existing authoring reaches a straight model: two nodes, a material, one pipe, a support, one case with one primitive.
  await createNode(page, "node:WP3-A", "0");
  await createNode(page, "node:WP3-B", "3");
  if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  await page.getByTestId("toolkit-entry").click();
  await page.getByTestId("toolkit-properties.material").click();
  await page.getByTestId("create-material-id").fill("material:wp3-invented");
  await page.getByTestId("create-material-label").fill("Invented material");
  await page.getByTestId("create-material-elastic").fill("200000000000");
  await page.getByTestId("create-material-shear").fill("77000000000");
  await page.getByTestId("create-material-provenance").fill(P);
  await page.getByTestId("queue-create-material-intent").click();
  await applyQueued(page);

  await page.getByTestId("command-pipe").click();
  await page.getByTestId("viewport-create-pipe-id").fill("pipe:WP3");
  await page.getByTestId("viewport-create-pipe-label").fill("Straight run");
  await chooseVirtualTarget(page, "viewport-create-pipe-from", "node:WP3-A");
  await page.getByRole("radio", { name: "Existing node", exact: true }).check();
  await chooseVirtualTarget(page, "viewport-create-pipe-to", "node:WP3-B");
  await chooseVirtualTarget(page, "viewport-create-pipe-material", "material:wp3-invented");
  await page.getByTestId("viewport-create-pipe-od").fill("0.168");
  await page.getByTestId("viewport-create-pipe-wall").fill("0.007");
  await page.getByTestId("viewport-create-pipe-yref-x").fill("0");
  await page.getByTestId("viewport-create-pipe-yref-y").fill("0");
  await page.getByTestId("viewport-create-pipe-yref-z").fill("1");
  await page.getByTestId("viewport-create-pipe-provenance").fill(P);
  await page.getByTestId("queue-explicit-pipe-intent").click();
  await page.getByTestId("apply-reviewed-draft").click();
  await expectTreeEntity(page, "pipe", "pipe:WP3");

  await page.getByTestId("command-support").click();
  await page.getByTestId("create-support-id").fill("support:WP3");
  await page.getByTestId("create-support-label").fill("Anchor");
  await chooseVirtualTarget(page, "create-support-node", "node:WP3-A");
  for (const restraint of ["RX", "RY", "RZ"]) await page.getByTestId(`create-support-restraint-${restraint}`).check();
  await page.getByTestId("create-support-provenance").fill(P);
  await page.getByTestId("queue-create-support-intent").click();
  await applyQueued(page);

  await openWorkspaceSection(page, "loads");
  await page.getByTestId("load-manager-create-load-id").fill("load:WP3");
  await page.getByTestId("load-manager-create-load-label").fill("Invented case");
  await page.getByTestId("load-manager-create-load-kind").fill("primitive_user_load");
  await page.getByTestId("load-manager-create-load-status").fill("preview_only");
  await page.getByTestId("load-manager-create-load-provenance").fill(P);
  await page.getByTestId("queue-create-load-case-intent").click();
  await applyQueued(page);
  await openWorkspaceSection(page, "loads");
  await chooseVirtualTarget(page, "load-manager-create-primitive-load-case", "load:WP3");
  await page.getByTestId("load-manager-create-primitive-category").selectOption("concentrated_force");
  await page.getByTestId("load-manager-create-primitive-id").fill("load:WP3-FY");
  await chooseVirtualTarget(page, "load-manager-create-primitive-node", "node:WP3-B");
  await page.getByTestId("load-manager-create-primitive-direction").selectOption("global_y");
  await page.getByTestId("load-manager-create-primitive-magnitude").fill("350");
  await page.getByTestId("load-manager-create-primitive-provenance").fill(P);
  await page.getByTestId("queue-create-primitive-intent").click();
  await applyQueued(page);

  // Reference configuration through the plain fields.
  const loads = await openWorkspaceSection(page, "loads");
  const inputs = loads.getByTestId("load-state-inputs");
  await expect(inputs).toBeVisible();
  const field = (label: string) => inputs.getByLabel(label, { exact: true });
  await inputs.getByRole("button", { name: "Add reference configuration", exact: true }).click();
  await field("Reference configuration 1 ID").fill("reference:wp3-installed");
  await field("Reference configuration 1 geometry").selectOption("authored_model_geometry");
  await inputs.getByRole("button", { name: "Add member to reference configuration 1", exact: true }).click();
  await field("Reference configuration 1 member 1 pipe").selectOption("pipe:WP3");
  await field("Reference configuration 1 member 1 basis").selectOption("temperature_reference");
  await field("Reference configuration 1 member 1 installation temperature value").fill("20");
  await field("Reference configuration 1 member 1 installation temperature unit").fill("degC");
  await field("Reference configuration 1 member 1 fit").selectOption("none");
  await field("Reference configuration 1 member 1 provenance").fill(P);
  await field("Reference configuration 1 provenance").fill(P);
  await inputs.getByRole("button", { name: "Queue reference configurations", exact: true }).click();
  await applyQueued(page);

  // Analysis state: material selection, element thermal state, support motion, load source.
  await openWorkspaceSection(page, "loads");
  await expect(inputs.getByTestId("load-state-analysis-state")).toContainText("analysis_state is absent for this case.");
  await field("Analysis state contract").selectOption("openpipestress.load_reference_state/1.0.0");
  await field("Reference configuration").selectOption("reference:wp3-installed");
  await inputs.getByRole("button", { name: "Add element state", exact: true }).click();
  await field("Element state 1 pipe").selectOption("pipe:WP3");
  await field("Element state 1 material selection").selectOption("explicit_base_properties");
  await field("Element state 1 material").selectOption("material:wp3-invented");
  await field("Element state 1 applicability reference").fill("invented_wp3_applicability_note");
  await field("Element state 1 thermal state").selectOption("constant_alpha_interval");
  await field("Element state 1 expansion coefficient value").fill("0.000012");
  await field("Element state 1 expansion coefficient unit").fill("1/K");
  await field("Element state 1 temperature change value").fill("100");
  await field("Element state 1 temperature change unit").fill("K");
  await field("Element state 1 coefficient meaning").selectOption("engineering_interval");
  await field("Element state 1 thermal provenance").fill(P);
  await inputs.getByRole("button", { name: "Add support state", exact: true }).click();
  await field("Support state 1 support").selectOption("support:WP3");
  await field("Support state 1 participation").selectOption("active_model_device");
  await inputs.getByRole("button", { name: "Add Support state 1 boundary motion", exact: true }).click();
  await field("Support state 1 motion 1 DOF").selectOption("UX");
  await field("Support state 1 motion 1 displacement value").fill("1");
  await field("Support state 1 motion 1 displacement unit").fill("mm");
  await field("Support state 1 motion 1 meaning").selectOption("absolute_reference_displacement");
  await inputs.getByRole("button", { name: "Add load source", exact: true }).click();
  await field("Load source 1 primitive").selectOption("load:WP3-FY");
  await field("Load source 1 factor (dimensionless)").fill("1");
  await field("History").selectOption("independent_equilibrium");
  await field("Analysis state provenance").fill(P);
  await inputs.getByRole("button", { name: "Queue analysis state", exact: true }).click();
  await expect(inputs.getByTestId("load-state-analysis-notice")).toContainText("checked by the operation engine and queued for Review/Apply");

  const beforeHash = await modelHash(page);
  await applyQueued(page);
  const afterHash = await modelHash(page);
  expect(afterHash).not.toBe(beforeHash);
  await openWorkspaceSection(page, "loads");
  await expect(inputs.getByTestId("load-state-analysis-state")).toContainText("analysis_state is applied for this case.");

  await openWorkspaceSection(page, "operations");
  await page.getByTestId("undo-session-model-edit").click();
  await expect.poll(() => modelHash(page)).toBe(beforeHash);
  await openWorkspaceSection(page, "loads");
  await expect(inputs.getByTestId("load-state-analysis-state")).toContainText("analysis_state is absent for this case.");
  await openWorkspaceSection(page, "operations");
  await page.getByTestId("redo-session-model-edit").click();
  await expect.poll(() => modelHash(page)).toBe(afterHash);
  await openWorkspaceSection(page, "loads");
  await expect(inputs.getByTestId("load-state-analysis-state")).toContainText("analysis_state is applied for this case.");

  // Browser solve refuses in both modes; no resolved-state block appears.
  for (const mode of ["sparse", "dense"]) {
    await openWorkspaceSection(page, "solve");
    await page.getByTestId(`solver-mode-${mode}`).click();
    await expect(page.getByTestId(`solver-mode-${mode}`)).toHaveAttribute("aria-pressed", "true");
    await page.getByTestId("run-mechanics-preview").click();
    await expect(page.getByTestId("solve-job-summary")).toContainText("state=failed");
    await expect(page.getByTestId("solve-job-error")).toContainText("BROWSER_SOLVE_BACKEND_REQUIRED");
    await expect(page.getByTestId("load-reference-states-block")).toHaveCount(0);
  }
  expect(await modelHash(page)).toBe(afterHash);
});
