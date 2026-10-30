/** T6S-3 (I74 PLAN 1.1; RR decisions 2, 3 and 12): the per-route desktop output policy.
 *
 * Inputs: PP's pinned milestone successors (D-U6-5, by the carrier case file's sha256),
 * delivered through mocked IPC with their pinned request models, so that the product's
 * own capture registers them; a load/reference-state fixture; and the preview-physics-2
 * transport replay. Unit tests over mocked IPC only: NOT a native witness (CQ-10, CQ-11).
 *
 * Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §5.1-5.4, §6.1; owner, 2026-10-21):
 * the exact routes (pressure-2, pressure-source-1) are `admitted` on the result JSON
 * export, the stress-neutral export and the rule check, and refused with
 * PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE and the reworded texts on every other surface, the
 * report package included. A load/reference-state result decides as its route does. */
import { afterEach, describe, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { runPreviewMechanics, type PreviewSolverMode } from "../../services/previewService";
import { createNativeMechanicsReplay, createSolveJobTransport, nativeMechanicsReplayPair, solveBytes } from "../../test/nativeMechanicsReplay";
import { type SourceContract } from "./numericalResultQuality";
import { numericalResultStanding } from "./resultStanding";
import { RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED, RETAINED_PRECISION_VALIDATION_REQUIRED } from "./retainedPrecisionStanding";
import {
  PRESSURE_2_OUTPUT_NOT_YET_AVAILABLE, PRESSURE_SOURCE_OUTPUT_REFUSAL, N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE, N_PRESSURE_2_OUTPUT, N_PRESSURE_SOURCE_OUTPUT, OUTPUT_POLICY, OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL, OUTPUT_SURFACES, PRESSURE_2_OUTPUT_REFUSAL,
  RETAINED_PRECISION_OUTPUT_REFUSAL, RETAINED_PHYSICS_OUTPUT_REFUSAL, routeOutputPolicy, routeOutputRefusal, routeSurfaceDecision, surfaceOutputRefusal, surfaceRouteRefusal,
  type OutputSurface,
} from "./outputPolicy";
import { receivedFixture } from "../../test-support/receivedText";
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
  invokeMock.mockImplementation(createSolveJobTransport(() => solveBytes(source)).invoke);
  const received = await runPreviewMechanics(model, mode);
  expect(numericalResultStanding(received, model)).toMatchObject({ eligible: true, findings: [] });
  return { source, received, model };
}
const moved = (model: PreviewModel) => { const copy = structuredClone(model) as Json; copy.nodes[0].position.x += 1; return copy as PreviewModel; };

/** Every SourceContract value, tied to the type in both directions by `tsc`. */
const ROUTES: Record<SourceContract, true> = { pressure_source: true, preview_physics: true, pressure: true, retained_preview_physics: true, retained_physics: true, unsupported: true };
/** T1's group a, by test-id prefix (the eighteen surfaces of the refusal tests). */
const GATED_PREFIXES = ["pcf-export", "caepipe-mbf", "caepipe-external", "export-adapter-sdk", "adapter-framework", "external-prover", "missing-data", "design-workspace", "rule-check", "report-lint", "solve-job", "headless-runner", "local-fea", "native-package", "handoff", "export-review", "report", "rendered-report"];
const PANELS = ["result-export", "stress-neutral"];
/** Epoch 8 (ruling 10A; design §5.1) and W-11's opening (DEL-08-01 Design
 * `exact-route-report-sections.md` §5.1; owner, 2026-10-26): the surfaces opened to the
 * exact routes. */
const OPENED = ["result-export", "stress-neutral", "rule-check", "rendered-report", "report-package"];
/** The gated (group a) surfaces opened to the exact routes. */
const OPENED_GATED = ["rule-check", "rendered-report"];
const EXACT_ROUTES = ["pressure", "pressure_source"] as const;
/** The exact entries, cell by cell (design §5.1): admitted on the five opened surfaces,
 * refused on every other, the report JSON packet included. */
const EXACT_SURFACES = {
  "pcf-export": "refused", "caepipe-mbf": "refused", "caepipe-external": "refused", "export-adapter-sdk": "refused",
  "adapter-framework": "refused", "external-prover": "refused", "missing-data": "refused", "design-workspace": "refused",
  "rule-check": "admitted", "report-lint": "refused", "solve-job": "refused", "headless-runner": "refused",
  "local-fea": "refused", "native-package": "refused", "handoff": "refused", "export-review": "refused",
  "report": "refused", "rendered-report": "admitted", "report-package": "admitted",
  "result-export": "admitted", "stress-neutral": "admitted",
} as const;
/** §5.2: the gate's refuse/admit decision before epoch 8, per route, on the eighteen gated
 * surfaces. pressure-2 and pressure-source-1 refused everywhere (B7b); the retained
 * successors refuse on all eighteen (their two admitted panels are not gated surfaces);
 * the ungated routes add no refusal. */
const PRE_EPOCH_8_GATED_REFUSES: Record<SourceContract, boolean> = {
  pressure: true, pressure_source: true, retained_preview_physics: true, retained_physics: true, preview_physics: false, unsupported: false,
};
/** The cells opened on a gated surface: the exact routes on `rule-check` (epoch 8, §5.2)
 * and on `rendered-report` (W-11). */
const opened = (route: string, surface: string) => (EXACT_ROUTES as readonly string[]).includes(route) && OPENED_GATED.includes(surface);
const refuses = (decided: ReturnType<typeof routeSurfaceDecision>) => decided !== null && decided.decision === "refused";
/** Every committed load/reference-state raw of the exact identities, with its request
 * model and the route's reason: pressure-2's two load/reference results, its
 * result-export fallback (not eligible), and pressure-source-1's five joined results. */
const PRESSURE_LR = "fixtures/product_preview/pressure/load_reference";
const SOURCE_LR = "fixtures/product_preview/pressure_source/load_reference_source";
const FALLBACK = "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz";
const LOAD_REFERENCE_STEMS = [
  ...["connected", "pressure"].map(name => [`${PRESSURE_LR}/${name}`, PRESSURE_2_OUTPUT_REFUSAL] as const),
  [FALLBACK, PRESSURE_2_OUTPUT_REFUSAL] as const,
  ...["n05", "n06", "fields", "mixed", "eigen_motion"].map(name => [`${SOURCE_LR}/${name}`, PRESSURE_SOURCE_OUTPUT_REFUSAL] as const),
];
const LOAD_REFERENCE_RAWS = LOAD_REFERENCE_STEMS.flatMap(([stem, reason]) => MODES.map(mode => [`${stem.split("/").at(-1)}-${mode}`, stem, mode, reason] as const));
const loadReferenceRaw = (stem: string, mode: string) => ({
  result: receivedFixture<MechanicsResult>(`${stem}-${mode}.raw.json`),
  model: structuredClone(json(`${stem}.request.json`).model) as PreviewModel,
});

describe("the output policy (T6S-3)", () => {
  it("has one deliberate entry per route and names every surface", () => {
    expect(Object.keys(OUTPUT_POLICY).sort()).toStrictEqual(Object.keys(ROUTES).sort());
    expect([...OUTPUT_SURFACES]).toStrictEqual([...GATED_PREFIXES, "report-package", ...PANELS]);
    for (const route of ["preview_physics", "unsupported"] as const) expect(OUTPUT_POLICY[route]).toStrictEqual({ gate: "none" });
    // Epoch 8 (design §5.1) and W-11: pressure-2 and pressure-source-1 (with or without
    // load/reference states) are admitted on the five opened surfaces and refused, by
    // their own reason, on every other, cell by cell.
    for (const [route, reason] of [["pressure", PRESSURE_2_OUTPUT_REFUSAL], ["pressure_source", PRESSURE_SOURCE_OUTPUT_REFUSAL]] as const) {
      const policy = OUTPUT_POLICY[route] as Json;
      expect([policy.gate, policy.reason]).toStrictEqual(["per_surface", reason]);
      expect(Object.keys(policy.surfaces)).toStrictEqual([...OUTPUT_SURFACES]);
      expect(policy.surfaces).toStrictEqual(EXACT_SURFACES);
      for (const surface of OUTPUT_SURFACES) {
        expect(policy.surfaces[surface], `${route} ${surface}`).toBe(OPENED.includes(surface) ? "admitted" : "refused");
        expect(routeSurfaceDecision(route, surface), `${route} ${surface}`).toStrictEqual(OPENED.includes(surface) ? null : { decision: "refused", reason });
      }
      expect(Object.entries(policy.surfaces).filter(([, decision]) => decision === "admitted").map(([surface]) => surface).sort()).toStrictEqual([...OPENED].sort());
      expect(policy.surfaces["report-package"]).toBe("admitted");
      expect(policy.surfaces["rendered-report"]).toBe("admitted");
      // §5.4: the report JSON packet stays refused.
      expect(policy.surfaces.report).toBe("refused");
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
    // R2 (G5): the header dispatch runs the reader's transport checks, so the exact successor's route is read
    // from lane P's genuine exact successor; the milestone relabelled as it reads unsupported, and the policy
    // adds no refusal to an unsupported statement (each surface refuses it through its own contract check).
    const { source } = milestone("sparse_interactive");
    const relabelled = structuredClone(source) as Json;
    relabelled.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/physics-retained-1";
    relabelled.formulation_basis.profile_id = "exact_straight_retained_w1a_v2";
    expect(loadReferenceOutputRefusal(relabelled)).toBeNull();
    const genuine = json("fixtures/results/retained_precision_exact_successor_sparse_interactive.json");
    const exact = genuine.source as Json, model = genuine.invocation.request.model as PreviewModel;
    expect(routeSurfaceDecision("retained_physics", "stress-neutral")).toStrictEqual({ decision: "admitted_when_eligible", reason: RETAINED_PHYSICS_OUTPUT_REFUSAL });
    expect(routeSurfaceDecision("retained_physics", "invented-surface")).toStrictEqual({ decision: "refused", reason: RETAINED_PHYSICS_OUTPUT_REFUSAL });
    expect(loadReferenceOutputRefusal(exact)).toBe(RETAINED_PHYSICS_OUTPUT_REFUSAL);
    for (const surface of OUTPUT_SURFACES) {
      const admitted = PANELS.includes(surface);
      expect(surfaceRouteRefusal(exact, surface)).toBe(admitted ? null : RETAINED_PHYSICS_OUTPUT_REFUSAL);
      expect(surfaceOutputRefusal(exact, model, surface)).toBe(admitted ? `${RETAINED_PRECISION_VALIDATION_REQUIRED}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}` : RETAINED_PHYSICS_OUTPUT_REFUSAL);
    }
  });
  it("D6/MJ8: pressure-source-1 refuses by name, under pressure-2's code, on every unopened surface, and the opened surfaces add no refusal (epoch 8)", () => {
    for (const surface of OUTPUT_SURFACES) {
      expect(routeSurfaceDecision("pressure_source", surface)).toStrictEqual(OPENED.includes(surface) ? null : { decision: "refused", reason: PRESSURE_SOURCE_OUTPUT_REFUSAL });
    }
    expect(routeSurfaceDecision("pressure_source", "invented-surface")).toStrictEqual({ decision: "refused", reason: PRESSURE_SOURCE_OUTPUT_REFUSAL });
    expect(routeSurfaceDecision("pressure", "invented-surface")).toStrictEqual({ decision: "refused", reason: PRESSURE_2_OUTPUT_REFUSAL });
    expect((OUTPUT_POLICY.pressure as Json).reason).toBe(PRESSURE_2_OUTPUT_REFUSAL);
  });
  it("§5.3: keeps the code and rewords both texts; neither calls the result invalid or unsupported", () => {
    expect(PRESSURE_2_OUTPUT_NOT_YET_AVAILABLE).toBe("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE");
    expect(N_PRESSURE_2_OUTPUT).toBe("This output of pressure-2 results (3.0.0/exact_pressure_v3) is not available on the desktop; only the result JSON and stress-neutral exports, the rule check, the rendered report and the report package admit a numerically eligible result. The result remains readable here; this is not a finding about the result.");
    expect(N_PRESSURE_SOURCE_OUTPUT).toBe(N_PRESSURE_2_OUTPUT.replace("pressure-2 results", "pressure-source-1 results"));
    expect(PRESSURE_2_OUTPUT_REFUSAL).toBe(`${PRESSURE_2_OUTPUT_NOT_YET_AVAILABLE}: ${N_PRESSURE_2_OUTPUT}`);
    expect(PRESSURE_SOURCE_OUTPUT_REFUSAL).toBe(`${PRESSURE_2_OUTPUT_NOT_YET_AVAILABLE}: ${N_PRESSURE_SOURCE_OUTPUT}`);
    for (const reason of [PRESSURE_2_OUTPUT_REFUSAL, PRESSURE_SOURCE_OUTPUT_REFUSAL]) {
      expect(reason.toLowerCase()).not.toMatch(/invalid|unsupported/);
      expect(reason).not.toMatch(/\bT[0-9]/);
    }
  });
  it("§5.2: on the eighteen gated surfaces every route decides as before epoch 8, except the exact routes on rule-check and (W-11) rendered-report", () => {
    for (const route of Object.keys(ROUTES) as SourceContract[]) {
      for (const surface of GATED_PREFIXES) {
        const before = PRE_EPOCH_8_GATED_REFUSES[route];
        expect(refuses(routeSurfaceDecision(route, surface)), `${route} ${surface}`).toBe(opened(route, surface) ? !before : before);
      }
    }
    // Exactly four cells changed, each from refuse to admit.
    const changed = (Object.keys(ROUTES) as SourceContract[]).flatMap(route => GATED_PREFIXES.filter(surface => refuses(routeSurfaceDecision(route, surface)) !== PRE_EPOCH_8_GATED_REFUSES[route]).map(surface => `${route}/${surface}`));
    expect(changed.sort()).toStrictEqual(["pressure/rendered-report", "pressure/rule-check", "pressure_source/rendered-report", "pressure_source/rule-check"]);
  });
  it.each([
    ["pressure-2", "fixtures/results/pressure_connected_mechanics_sparse.json", `${PRESSURE_LR}/connected`, PRESSURE_2_OUTPUT_REFUSAL],
    ["pressure-source-1", "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", `${SOURCE_LR}/mixed`, PRESSURE_SOURCE_OUTPUT_REFUSAL],
  ] as const)("§5.2: a plain and a load/reference-state %s source decide alike on every surface, at every level", (_id, plainPath, stem, reason) => {
    const plain = receivedFixture<MechanicsResult>(plainPath);
    const { result: withStates, model } = loadReferenceRaw(stem, "sparse_interactive");
    expect((plain.contract_evidence as Json)?.load_reference_states).toBeUndefined();
    expect(Array.isArray((withStates.contract_evidence as Json).load_reference_states)).toBe(true);
    for (const source of [plain, withStates]) {
      for (const surface of GATED_PREFIXES as OutputSurface[]) {
        // The gate's decision (`LoadReferenceOutputGate` reads `surfaceRouteRefusal`).
        expect(surfaceRouteRefusal(source, surface) !== null, surface).toBe(!OPENED_GATED.includes(surface));
      }
    }
    for (const surface of OUTPUT_SURFACES) {
      const expected = OPENED.includes(surface) ? null : reason;
      expect(surfaceRouteRefusal(plain, surface), surface).toBe(expected);
      expect(surfaceRouteRefusal(withStates, surface), surface).toBe(expected);
      expect(surfaceOutputRefusal(plain, model, surface), surface).toBe(expected);
      expect(surfaceOutputRefusal(withStates, model, surface), surface).toBe(expected);
    }
    expect(routeOutputRefusal(plain)).toBe(routeOutputRefusal(withStates));
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
  it.each(LOAD_REFERENCE_RAWS)("§5.4: a load/reference-state result (%s) follows its route: null on the opened surfaces at both levels, the reworded refusal elsewhere", (_id, stem, mode, reason) => {
    const { result, model } = loadReferenceRaw(stem, mode);
    expect(Array.isArray((result.contract_evidence as Json).load_reference_states)).toBe(true);
    for (const surface of OUTPUT_SURFACES) {
      const expected = OPENED.includes(surface) ? null : reason;
      // The policy functions do not read standing on an `admitted` cell (§5.1), so the
      // never-eligible pressure-source-1 raws and the fallback decide as the eligible ones.
      expect(surfaceRouteRefusal(result, surface), surface).toBe(expected);
      expect(surfaceOutputRefusal(result, model, surface), surface).toBe(expected);
      expect(surfaceOutputRefusal(result, null, surface), surface).toBe(expected);
    }
    // The route's shared reason (`loadReferenceOutputRefusal`, which no product surface reads since W-11).
    expect(routeOutputRefusal(result)).toBe(reason);
    expect(loadReferenceOutputRefusal(result)).toBe(reason);
  });
  it("an ungated route and a relabelled statement add no policy refusal; their own checks apply", async () => {
    const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
    const result = await runPreviewMechanics(model, "sparse_interactive");
    for (const surface of OUTPUT_SURFACES) expect(surfaceOutputRefusal(result, model, surface)).toBeNull();
    const relabelled = structuredClone(milestone("sparse_interactive").source) as Json;
    relabelled.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/preview-physics-2";
    relabelled.formulation_basis.profile_id = "product_preview_mechanics_v1";
    expect(surfaceOutputRefusal(relabelled, model, "stress-neutral")).toBeNull();
    expect(loadReferenceOutputRefusal(relabelled)).toBeNull();
    expect(surfaceOutputRefusal(null, model, "stress-neutral")).toBeNull();
  });
});
