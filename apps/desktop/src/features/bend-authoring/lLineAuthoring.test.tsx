/** T4-U2 phase 3: the first usable path on the desktop. The L line is authored
 * from its corner through the panels (Insert bend at corner with the user's k,
 * the exact_pressure_v3 profile, regions through the realized bend) and the
 * wasm operation engine, and the result equals the document the Rust applier
 * authors (`fixtures/model_operations/pressure_v3_l_line_authoring.json`,
 * pinned by `operation_applier/tests/l_line_authoring.rs`). Its solve through
 * the native job path returns pressure-1. The desktop reads results only through
 * the Rust reader's verdict (b7-plan R2), whose header dispatch names no
 * pressure-1 route: the result reads as unsupported, is never eligible and
 * lists no rows as current. All data are invented. */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import fixture from "../../../../../fixtures/model_operations/pressure_v3_l_line_authoring.json";
import { BendAuthoringForms } from "./BendAuthoringForms";
import { PressureAuthoringPanel } from "../pressure-authoring/PressureAuthoringPanel";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { applyModelOperation } from "../../services/operationService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { startPreviewMechanicsJob } from "../../services/previewService";
import { awaitBackendSolveJob } from "../workspace/solveJobAudit";
import { ResultsPanel } from "../results/ResultsPanel";
import { KnownSemanticNotices } from "../results/KnownSemanticNotices";
import { sourceContract } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import type { EditorOperationIntent, MechanicsResult, PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const PROVENANCE = "invented T4-U2 authoring input (from T4-I7's L line); not library, component, catalog or code-rule data";
const corner = fixture.corner_document as unknown as PreviewModel;
const expected = fixture.authored_model as unknown as PreviewModel;
const pressure1 = fixture.sparse_result as unknown as MechanicsResult;

afterEach(() => { cleanup(); invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });

async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyModelOperation(model, intent, await computeModelHash(model));
  expect(outcome.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  return outcome.applied_model as PreviewModel;
}
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
async function queued(click: () => void, queue: ReturnType<typeof vi.fn>): Promise<EditorOperationIntent> {
  click();
  await waitFor(() => expect(queue).toHaveBeenCalledOnce());
  return queue.mock.calls[0][0] as EditorOperationIntent;
}

describe("the L line authored on the desktop (T4-U2)", () => {
  it("is authored from its corner through the panels, equals the applier's document and is refused by name after its solve", async () => {
    // Material E/nu (an existing rich form; queued directly here).
    const keys = ["constitutive_basis", "elastic_modulus", "shear_modulus", "poisson_ratio", "provenance"] as const;
    const projection = (m: Json) => Object.fromEntries(keys.filter(k => m[k] !== undefined).map(k => [k, m[k]]));
    let model = await apply(corner, makeRichIntent({ object_type: "Material", ref: "material:steel" }, "set_field", "constitutive_properties",
      await canonicalJsonString(projection(corner.materials![0])), projection(expected.materials![0]), "Constitutive properties"));

    // Insert bend at corner: R, the user's k and its source, identities.
    let queue = vi.fn();
    render(<BendAuthoringForms model={model} selection={{ type: "node", id: "node:X" }} onQueueIntent={queue} />);
    const form = screen.getByTestId("insert-bend-at-corner-form");
    expect(form).toHaveTextContent("k = 1 means no extra flexibility beyond the curved beam");
    expect((screen.getByLabelText("Corner bend flexibility factor k") as HTMLInputElement).value).toBe("");
    change("Bend radius R value", "250"); change("Bend radius R unit", "mm");
    change("Corner bend flexibility factor k", "2");
    change("Corner bend source of k", "invented user value for this test (k = 2)");
    change("Bend start node ID", "node:B"); change("Bend end node ID", "node:C");
    change("Bend arc pipe ID", "pipe:BEND"); change("Bend component ID", "component:bend-1");
    change("Geometry source reference", "inserted at the corner by the user");
    change("Corner bend provenance", PROVENANCE);
    const insert = await queued(() => fireEvent.click(within(form).getByRole("button", { name: "Queue insert bend at corner" })), queue);
    expect(insert.change.change_kind).toBe("insert_bend_at_corner");
    expect(insert.author_type).toBe("user");
    model = await apply(model, insert);
    cleanup();

    // Regions through the realized bend (the picker names it with its k).
    const regions = (expected.load_cases[0] as Json).pressure_regions;
    render(<PressureAuthoringPanel model={model} selection={{ type: "load", id: "case:u2" }} loadCaseId="case:u2" onQueueIntent={vi.fn()} />);
    fireEvent.click(screen.getByRole("button", { name: "Add pressure region" }));
    expect(screen.getAllByText(/realized bend component:bend-1, k = 2/).length).toBeGreaterThan(0);
    cleanup();
    model = await apply(model, makeRichIntent({ object_type: "Load", ref: "case:u2" }, "update_load", "pressure_regions", "not_present", regions, "Pressure regions"));

    // The profile: exact_pressure_v3 is the only choice.
    queue = vi.fn();
    render(<PressureAuthoringPanel model={model} selection={{ type: "load", id: "case:u2" }} onQueueIntent={queue} />);
    change("Pressure profile", "exact_pressure_v3");
    const profile = await queued(() => fireEvent.click(screen.getByRole("button", { name: "Queue pressure profile" })), queue);
    expect(JSON.parse(profile.change.after)).toEqual({ schema_version: "0.3.0", pressure_contract: { version: "3.0.0", mode: "exact_pressure_v3" } });
    model = await apply(model, profile);
    cleanup();

    // Thermal and weight loads, as entered.
    for (const load of (expected.load_cases[0] as Json).primitive_loads) {
      const intent = makeRichIntent({ object_type: "Load", ref: "case:u2" }, "create_primitive_load", "primitive_loads", "not_present", load, "Primitive load");
      model = await apply(model, { ...intent, operation_kind: "create", change: { ...intent.change, unit: load.magnitude.unit, dimension: load.dimension } });
    }
    expect(model).toEqual(expected);

    // Solve through the native job path: the producer returns pressure-1.
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async (command: string, args: Json) => {
      if (command === "start_preview_mechanics_job_with_solver_mode") {
        expect(args.model).toEqual(expected);
        return { job_id: "job:l-line", backend_cancellation_token: "job:l-line:token", state: "queued", cancellation_scope: "unit_test" };
      }
      if (command === "poll_preview_mechanics_job") return { job_id: "job:l-line", state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "unit_test", result: structuredClone(pressure1), error_message: null };
      throw new Error(`unexpected command ${command}`);
    });
    const receipt = await startPreviewMechanicsJob(model, "sparse_interactive");
    expect(receipt.mode).toBe("backend_job");
    const terminal = await awaitBackendSolveJob("job:l-line");
    expect(terminal.state).toBe("completed");
  });

  it("reads a pressure-1 result as unsupported through the Rust reader (b7-plan R2)", () => {
    expect(sourceContract(pressure1)).toBe("unsupported");
    const standing = numericalResultStanding(pressure1, expected);
    expect(standing.eligible).toBe(false);
    render(<ResultsPanel result={pressure1} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={vi.fn()} />);
    expect(screen.getByTestId("numerical-result-standing")).toHaveTextContent("Unsupported numerical contract; historical values only. Needs recompute.");
    cleanup();
    render(<KnownSemanticNotices result={pressure1} testIdPrefix="results" />);
  });
});
