// RT-S1/RT-U1: the `create_support` intent the New support form emits, with its family, passes the
// operation applier's `SUPPORT_FAMILIES` validation (`rich_authoring.rs`) in the shared Wasm engine,
// a family the form disables is one the engine refuses, and a value outside the list is refused. No hash or operation mocks: these checks require the
// accepted rebuilt Wasm. All entities are invented test input.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { changeFormControl } from "../../test-support/workspaceTestControls";
import type { EditorOperationIntent } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { applyModelOperation } from "../../services/operationService";
import { loadPreviewModel } from "../../services/previewService";
import { SUPPORT_FAMILIES } from "../results/displacementExtremes";
import { PropertyInspector } from "./PropertyInspector";
import { SUPPORT_FAMILY_OPTIONS } from "./supportFamilyDraft";

afterEach(cleanup);

async function emitted(choose: (() => void) | null) {
  const model = await loadPreviewModel();
  const node = model.nodes[0].id;
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={{ type: "node", id: node }} onQueueIntent={queue} /></DisplayUnitsProvider>);
  for (const dof of ["RX", "RY", "RZ"]) fireEvent.click(screen.getByTestId(`create-support-restraint-${dof}`));
  choose?.();
  fireEvent.click(screen.getByTestId("queue-create-support-intent"));
  expect(queue).toHaveBeenCalledOnce();
  return { model, node, intent: queue.mock.calls[0][0] };
}

const ENABLED = SUPPORT_FAMILY_OPTIONS.filter((option) => option.value !== "" && !option.disabled).map((option) => option.value);
const DISABLED = SUPPORT_FAMILY_OPTIONS.filter((option) => option.disabled).map((option) => option.value);
// Disabled although the engine admits it: without a nonlinear law the solve lowers it to a rigid restraint.
const SOLVES_AS_SOMETHING_ELSE = ["nonlinear"];
const ENGINE_REFUSED = DISABLED.filter((family) => !SOLVES_AS_SOMETHING_ELSE.includes(family));

describe("the New support form's family in the operation applier", () => {
  it("lists every SupportFamily value once, as enabled or disabled", () => {
    expect([...ENABLED, ...DISABLED].sort()).toEqual(SUPPORT_FAMILIES.map((item) => item.value).sort());
  });

  it.each(ENABLED)("applies the emitted support with enabled family %s", async (family) => {
    const { model, node, intent } = await emitted(family === "anchor" ? null : () => changeFormControl(screen.getByTestId("create-support-family"), { target: { value: family } }));
    const original = JSON.stringify(model);
    const applied = await applyModelOperation(model, intent, null);
    expect(applied.validation.application_status, JSON.stringify(applied.diagnostics)).toBe("applied_to_session_model");
    expect(applied.applied_model!.supports.find((support) => support.id === intent.target.ref))
      .toMatchObject({ node, family, restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"] });
    expect(JSON.stringify(model)).toBe(original);
  });

  // The form cannot emit a disabled family, so the emitted intent carries it in `after` here.
  it("disables nonlinear, which the engine admits from this form but would solve as a rigid restraint", async () => {
    expect(DISABLED).toEqual(expect.arrayContaining(SOLVES_AS_SOMETHING_ELSE));
    const { model, intent } = await emitted(null);
    const after = { ...JSON.parse(intent.change.after), family: "nonlinear" };
    const admitted = await applyModelOperation(model, { ...intent, change: { ...intent.change, after: JSON.stringify(after) } }, null);
    expect(admitted.validation.application_status).toBe("applied_to_session_model");
    expect(admitted.applied_model!.supports.find((support) => support.id === intent.target.ref)?.nonlinear).toBeUndefined();
  });

  it.each([...ENGINE_REFUSED, "strut"])("is refused by the engine for disabled (or unknown) family %s", async (family) => {
    const { model, intent } = await emitted(null);
    const after = { ...JSON.parse(intent.change.after), family };
    const refused = await applyModelOperation(model, { ...intent, change: { ...intent.change, after: JSON.stringify(after) } }, null);
    expect(refused.validation.application_status).not.toBe("applied_to_session_model");
    expect(refused.diagnostics.some((item) => item.severity === "blocking")).toBe(true);
  });
});
