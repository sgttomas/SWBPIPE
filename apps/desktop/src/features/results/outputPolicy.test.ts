/** T6S-3 (I74 PLAN 1.1; RR decisions 2, 3 and 12): the per-route desktop output policy.
 *
 * Inputs: PP's pinned milestone successors (D-U6-5, by the carrier case file's sha256),
 * delivered through mocked IPC with their pinned request models, so that the product's
 * own capture registers them; a load/reference-state fixture; and the preview-physics-1
 * transport replay. Unit tests over mocked IPC only: NOT a native witness (CQ-10, CQ-11). */
import { afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { runPreviewMechanics, type PreviewSolverMode } from "../../services/previewService";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { numericalResultStanding, type SourceContract } from "./numericalResultQuality";
import { RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED, RETAINED_PRECISION_VALIDATION_REQUIRED } from "./retainedPrecisionStanding";
import {
  LOAD_REFERENCE_OUTPUT_REFUSAL, N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE, OUTPUT_POLICY, OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL, OUTPUT_SURFACES,
  RETAINED_PRECISION_OUTPUT_REFUSAL, RETAINED_PHYSICS_OUTPUT_REFUSAL, routeOutputPolicy, routeSurfaceDecision, surfaceOutputRefusal, surfaceRouteRefusal,
} from "./outputPolicy";
import { loadReferenceOutputRefusal } from "./loadReferenceOutputAvailability";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => { invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const caseFile = json("fixtures/results/retained_precision_carrier_cases.json");
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;

/** PP's pinned milestone successor bytes and their request model, checked by sha256. */
function milestone(mode: PreviewSolverMode) {
  const entry = caseFile.fixtures[`milestone_${mode}`];
  const bytes = readFileSync(resolve(root, entry.path));
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(entry.sha256);
  const doc = JSON.parse(bytes.toString("utf8"));
  return { source: doc.source as MechanicsResult, model: doc.invocation.request.model as PreviewModel };
}
/** Mocked direct IPC: the native command returns these bytes for this mode. */
async function eligible(mode: PreviewSolverMode) {
  const { source, model } = milestone(mode);
  (window as Json).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async () => structuredClone(source));
  const received = await runPreviewMechanics(model, mode);
  expect(numericalResultStanding(received, model)).toMatchObject({ eligible: true, findings: [] });
  return { source, received, model };
}
const moved = (model: PreviewModel) => { const copy = structuredClone(model) as Json; copy.nodes[0].position.x += 1; return copy as PreviewModel; };

/** Every SourceContract value, tied to the type in both directions by `tsc`. */
const ROUTES: Record<SourceContract, true> = { legacy: true, precision: true, physics: true, source_blocks: true, physics_source: true, preview_physics: true, load_reference: true, load_reference_source: true, retained_preview_physics: true, retained_physics: true, unsupported: true };
/** T1's group a, by test-id prefix (the eighteen surfaces of the refusal tests). */
const GATED_PREFIXES = ["pcf-export", "caepipe-mbf", "caepipe-external", "export-adapter-sdk", "adapter-framework", "external-prover", "missing-data", "design-workspace", "rule-check", "report-lint", "solve-job", "headless-runner", "local-fea", "native-package", "handoff", "export-review", "report", "rendered-report"];
const PANELS = ["result-export", "stress-neutral"];

describe("the output policy (T6S-3)", () => {
  it("has one deliberate entry per route and names every surface", () => {
    expect(Object.keys(OUTPUT_POLICY).sort()).toStrictEqual(Object.keys(ROUTES).sort());
    expect([...OUTPUT_SURFACES]).toStrictEqual([...GATED_PREFIXES, "report-package", ...PANELS]);
    for (const route of ["legacy", "precision", "physics", "source_blocks", "physics_source", "preview_physics", "unsupported"] as const) expect(OUTPUT_POLICY[route]).toStrictEqual({ gate: "none" });
    for (const route of ["load_reference", "load_reference_source"] as const) {
      const policy = OUTPUT_POLICY[route] as Json;
      expect([policy.gate, policy.reason]).toStrictEqual(["per_surface", LOAD_REFERENCE_OUTPUT_REFUSAL]);
      expect(Object.keys(policy.surfaces)).toStrictEqual([...OUTPUT_SURFACES]);
      expect(Object.values(policy.surfaces).every(decision => decision === "refused")).toBe(true);
    }
    const successor = OUTPUT_POLICY.retained_preview_physics as Json;
    expect([successor.gate, successor.reason]).toStrictEqual(["per_surface", RETAINED_PRECISION_OUTPUT_REFUSAL]);
    expect(Object.keys(successor.surfaces).sort()).toStrictEqual([...OUTPUT_SURFACES].sort());
    expect(Object.entries(successor.surfaces).filter(([, decision]) => decision === "admitted_when_eligible").map(([surface]) => surface)).toStrictEqual(PANELS);
    expect(Object.entries(successor.surfaces).filter(([, decision]) => decision === "refused").map(([surface]) => surface)).toStrictEqual([...GATED_PREFIXES, "report-package"]);
    // B3b (B3-D §7; B3D-14): the exact successor's own entry and reason, the same two panels.
    const exact = OUTPUT_POLICY.retained_physics as Json;
    expect([exact.gate, exact.reason]).toStrictEqual(["per_surface", RETAINED_PHYSICS_OUTPUT_REFUSAL]);
    expect(Object.keys(exact.surfaces)).toStrictEqual([...OUTPUT_SURFACES]);
    expect(Object.entries(exact.surfaces).filter(([, decision]) => decision === "admitted_when_eligible").map(([surface]) => surface)).toStrictEqual(PANELS);
    expect(Object.entries(exact.surfaces).filter(([, decision]) => decision === "refused").map(([surface]) => surface)).toStrictEqual([...GATED_PREFIXES, "report-package"]);
    expect(RETAINED_PHYSICS_OUTPUT_REFUSAL).not.toBe(RETAINED_PRECISION_OUTPUT_REFUSAL);
  });
  it("B3b: an exact successor reads its own reason on every refused surface, and the two panels need eligible standing", () => {
    // Header dispatch only (no reader run): the milestone relabelled as the exact successor routes to `retained_physics`.
    const { source, model } = milestone("sparse_interactive");
    const exact = structuredClone(source) as Json;
    exact.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/physics-retained-1";
    exact.formulation_basis.profile_id = "exact_straight_retained_w1a_v2";
    expect(routeSurfaceDecision("retained_physics", "stress-neutral")).toStrictEqual({ decision: "admitted_when_eligible", reason: RETAINED_PHYSICS_OUTPUT_REFUSAL });
    expect(routeSurfaceDecision("retained_physics", "invented-surface")).toStrictEqual({ decision: "refused", reason: RETAINED_PHYSICS_OUTPUT_REFUSAL });
    expect(loadReferenceOutputRefusal(exact)).toBe(RETAINED_PHYSICS_OUTPUT_REFUSAL);
    for (const surface of OUTPUT_SURFACES) {
      const admitted = PANELS.includes(surface);
      expect(surfaceRouteRefusal(exact, surface)).toBe(admitted ? null : RETAINED_PHYSICS_OUTPUT_REFUSAL);
      expect(surfaceOutputRefusal(exact, model, surface)).toBe(admitted ? `${RETAINED_PRECISION_VALIDATION_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}` : RETAINED_PHYSICS_OUTPUT_REFUSAL);
    }
  });
  it("fails closed for a route or a surface without an entry", () => {
    expect(routeOutputPolicy("physics_retained")).toBeNull();
    expect(routeSurfaceDecision("physics_retained", "result-export")).toStrictEqual({ decision: "refused", reason: OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL });
    expect(routeSurfaceDecision("retained_preview_physics", "invented-surface")).toStrictEqual({ decision: "refused", reason: RETAINED_PRECISION_OUTPUT_REFUSAL });
    expect(routeSurfaceDecision("retained_preview_physics", "stress-neutral")).toStrictEqual({ decision: "admitted_when_eligible", reason: RETAINED_PRECISION_OUTPUT_REFUSAL });
    expect(routeSurfaceDecision("preview_physics", "invented-surface")).toBeNull();
    expect(OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL.toLowerCase()).not.toMatch(/invalid|unsupported/);
  });
  it.each(MODES)("%s: the two panels admit an eligible successor, with the live capture, and no other surface does", async (mode) => {
    const { source, received, model } = await eligible(mode);
    for (const surface of OUTPUT_SURFACES) {
      const admitted = PANELS.includes(surface);
      expect(surfaceOutputRefusal(received, model, surface)).toBe(admitted ? null : RETAINED_PRECISION_OUTPUT_REFUSAL);
      expect(surfaceRouteRefusal(received, surface)).toBe(admitted ? null : RETAINED_PRECISION_OUTPUT_REFUSAL);
    }
    // The shared function's meaning is unchanged: the eighteen surfaces still read it.
    expect(loadReferenceOutputRefusal(received)).toBe(RETAINED_PRECISION_OUTPUT_REFUSAL);
    const refusal = (code: string) => `${code}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`;
    for (const surface of PANELS as typeof OUTPUT_SURFACES[number][]) {
      // A copy and the pinned file's own bytes are unregistered; a moved model is not the live capture.
      expect(surfaceOutputRefusal(structuredClone(received), model, surface)).toBe(refusal(RETAINED_PRECISION_VALIDATION_REQUIRED));
      expect(surfaceOutputRefusal(source, model, surface)).toBe(refusal(RETAINED_PRECISION_VALIDATION_REQUIRED));
      expect(surfaceOutputRefusal(received, moved(model), surface)).toBe(refusal(RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED));
      // Route level only (validators and pure projections) does not read standing.
      expect(surfaceRouteRefusal(structuredClone(received), surface)).toBeNull();
    }
  });
  it("keeps load/reference-state results refused on both panels and every other surface", () => {
    const result = json("fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json") as MechanicsResult;
    const model = structuredClone(json("fixtures/product_preview/load_reference/pressure.request.json").model) as PreviewModel;
    for (const surface of OUTPUT_SURFACES) {
      expect(surfaceOutputRefusal(result, model, surface)).toBe(LOAD_REFERENCE_OUTPUT_REFUSAL);
      expect(surfaceRouteRefusal(result, surface)).toBe(LOAD_REFERENCE_OUTPUT_REFUSAL);
    }
    expect(loadReferenceOutputRefusal(result)).toBe(LOAD_REFERENCE_OUTPUT_REFUSAL);
  });
  it("an ungated route and a relabelled statement add no policy refusal; their own checks apply", async () => {
    const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
    const result = await runPreviewMechanics(model, "sparse_interactive");
    for (const surface of OUTPUT_SURFACES) expect(surfaceOutputRefusal(result, model, surface)).toBeNull();
    const relabelled = structuredClone(milestone("sparse_interactive").source) as Json;
    relabelled.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-1";
    relabelled.formulation_basis.profile_id = "product_preview_mechanics_v1";
    expect(surfaceOutputRefusal(relabelled, model, "stress-neutral")).toBeNull();
    expect(loadReferenceOutputRefusal(relabelled)).toBeNull();
    expect(surfaceOutputRefusal(null, model, "stress-neutral")).toBeNull();
  });
});
