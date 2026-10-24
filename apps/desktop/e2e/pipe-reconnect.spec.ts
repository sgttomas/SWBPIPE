import { expect, test, type Page } from "@playwright/test";
import { attachBrowserIdentity, gotoModel } from "./ui-foundation-workflows";
import { chooseVirtualMultiTarget, chooseVirtualTarget, ensureTreeExpanded } from "./workspace-driver";

// W-10 (DEL-16-06 Design `pipe-reconnect.md` §4): reconnect one pipe end through
// Geometry tools and batch review; Apply waits for a clean Validate; From/To in
// the table, undo and redo; then a component-attached pipe is refused by name
// with the model unchanged. Invented model: A -P- B -Q- D -R- C, and S from D
// to E where a geometry-only bend marker sits.
const PROV = "invented W-10 e2e input";
const node = (id: string, x: number, y: number, z = 0) => ({ id, label: id.replace("node:", ""), position: { x, y, z }, provenance: PROV });
const section = { outside_diameter: { value: 0.1, unit: "m" }, wall_thickness: { value: 0.005, unit: "m" } };
const pipe = (id: string, from: string, to: string) => ({ id, label: id.replace("pipe:", ""), from, to, section, material: "material:steel", y_reference: { x: 0, y: 0, z: 1 }, provenance: PROV });
const anchor = (id: string, at: string) => ({ id, node: at, family: "anchor", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: PROV });
const model = {
  schema_version: "0.7.0", document_kind: "openpipestress.product_preview.model",
  data_boundary: { public_examples_policy: "invented_or_cleared_data_only", protected_source_policy: "no_bundled_protected_owner_or_standards_data", private_data_policy: "no_private_project_data", professional_boundary: "technical_preview_requires_human_engineering_review" },
  project: { id: "project:w10-e2e", name: "W-10 reconnect e2e", units: { length: "m", force: "N", angle: "rad", pressure: "Pa", stress: "Pa", temperature: "degC" } },
  analysis_status: { mechanics: "ready_for_preview_diagnostics", rule_check: "not_performed_user_rule_inputs_missing", professional_acceptance: "not_provided" },
  nodes: [node("node:A", 0, 0), node("node:B", 2, 0), node("node:C", 2, 2), node("node:D", 4, 0), node("node:E", 6, 0)],
  pipe_segments: [pipe("pipe:P", "node:A", "node:B"), pipe("pipe:Q", "node:B", "node:D"), pipe("pipe:R", "node:D", "node:C"), pipe("pipe:S", "node:D", "node:E")],
  supports: [anchor("support:A", "node:A"), anchor("support:D", "node:D")],
  components: [{ id: "component:bend-E", label: "Bend marker", kind: "bend", node: "node:E", geometry: { bend_radius: { value: 0.3, unit: "m" } }, provenance: PROV }],
  materials: [{ id: "material:steel", elastic_modulus: { value: 200e9, unit: "Pa" }, shear_modulus: { value: 80e9, unit: "Pa" }, provenance: PROV }],
  load_cases: [{ id: "case:w", label: "Weight", kind: "sustained", status: "draft", primitive_loads: [], provenance: PROV }],
  combinations: [], diagnostics: []
};

async function queueReconnect(page: Page, pipeId: string, end: "from" | "to", target: string) {
  // Review changes, then its Geometry tab (the toolkit's Build > Reconnect pipe end routes here too).
  await page.getByTestId("workspace-review").click();
  await expect(page.getByTestId("workspace-section-operations")).toBeVisible();
  await page.getByTestId("operation-tab-geometry").click();
  const geometry = page.locator("#geometry-tools");
  await expect(geometry).toBeVisible();
  await geometry.getByLabel("Geometry tool").selectOption("reconnect");
  await chooseVirtualMultiTarget(geometry, "geometry-source-pipes", pipeId);
  await geometry.getByLabel("Reconnect end").selectOption(end);
  await chooseVirtualTarget(geometry, "reconnect-target-node", target);
  await geometry.getByRole("button", { name: "Queue geometry batch" }).click();
  await expect(page.getByTestId("batch-review-summary")).toContainText("1 queued batches");
}
/** The pipe's table row; its accessible name reads `id label from to ...` (From/To are read-only cells). */
async function pipeRow(page: Page, id: string) {
  await ensureTreeExpanded(page);
  await page.getByTestId("layout-mode-grid").click();
  await page.getByTestId("entity-grid-type-pipes").click();
  return page.getByRole("row", { name: new RegExp(`^${id.replace(":", "\\:")} `) });
}
const ends = (id: string, from: string, to: string) => new RegExp(`^${id} ${id.replace("pipe:", "")} ${from} ${to} `);

test("W-10 reconnect applies after Validate, shows in the table, undoes and redoes; a component-attached pipe is refused by name", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  await gotoModel(page, model);
  await expect(await pipeRow(page, "pipe:P")).toHaveAccessibleName(ends("pipe:P", "node:A", "node:B"));

  await queueReconnect(page, "pipe:P", "to", "node:C");
  const apply = page.getByTestId("apply-batch-operation-batch-1");
  await expect(apply).toBeDisabled();
  await expect(page.getByTestId("preview-required-operation-batch-1")).toContainText("This batch contains a reconnect.");
  await page.getByTestId("validate-batch-operation-batch-1").click();
  await expect(page.getByText(/Preview only. Temporary state was discarded/)).toBeVisible();
  await expect(apply).toBeEnabled();
  await apply.click();
  await expect(page.getByTestId("batch-review-summary")).toContainText("1 batches applied");

  const row = await pipeRow(page, "pipe:P");
  await expect(row).toHaveAccessibleName(ends("pipe:P", "node:A", "node:C"));
  await page.getByTestId("workspace-undo").click();
  await expect(row).toHaveAccessibleName(ends("pipe:P", "node:A", "node:B"));
  await page.getByTestId("workspace-redo").click();
  await expect(row).toHaveAccessibleName(ends("pipe:P", "node:A", "node:C"));

  // S's to end is E, where a bend marker sits: refused, naming the component.
  await queueReconnect(page, "pipe:S", "to", "node:C");
  await page.getByTestId("validate-batch-operation-batch-2").click();
  // Shown in the batch's summary and in its step.
  const refusal = page.getByTestId("batch-diagnostic-OP-RECONNECT-COMPONENT-ATTACHED").last();
  await expect(refusal).toContainText("Cannot reconnect pipe:S: component component:bend-E (bend) sits at node node:E, where this reconnect removes the pipe.");
  await expect(refusal).toContainText("Affected: pipe:S, component:bend-E, node:E");
  await expect(page.getByTestId("apply-batch-operation-batch-2")).toBeDisabled();
  await expect(await pipeRow(page, "pipe:S")).toHaveAccessibleName(ends("pipe:S", "node:D", "node:E"));
  await page.screenshot({ path: info.outputPath("w10-reconnect-refusal.png") });
});
