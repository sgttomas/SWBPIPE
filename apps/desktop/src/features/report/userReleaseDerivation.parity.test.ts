// W-18b slice (r2b) (DEL-08-01 Design `report-user-release.md` §2.6, §6): the derivation of
// a user-released report package request, held equal across the layers. The desktop's
// builders (`assembleUserReleasedReportPackageRequest`, `userReleasedRenderableReportInput`)
// build each vector's expected request from its inputs M, S, m, I, R, H and U; the native
// mirror (`core/reporting/report_package/src/user_release.rs`) derives it again from the same
// file in `core/reporting/report_package/tests/user_release_derivation.rs`. This test writes
// `fixtures/reports/invented/user_release_derivation_vectors.json` and fails when it is
// stale; `OPENPIPESTRESS_WRITE_USER_RELEASE_VECTORS=<path>` writes it there instead.
//
// The solves are the unit transport replay of committed producer bytes (NOT a native UI
// qualification). The record is invented; no route uses the derivation yet, and nothing here
// records a user release or implies acceptance, issuance or lifecycle standing.
// Standard claim fence applies (claims taxonomy per DEC-081).
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics, type PreviewSolverMode } from "../../services/previewService";
import { canonicalSha256Hex, canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import desktopAppConfig from "../../../src-tauri/tauri.conf.json";
import type { MechanicsResult } from "../../types";
import { buildRenderableReportInput, USER_RELEASED_REPORT_CONTEXT, userReleasedRenderableReportInput } from "./renderableReportInput";
import { assembleUserReleasedReportPackageRequest, REPORT_PACKAGE_USER_RELEASED_PROFILE } from "./reportPackageRequest";
import { reportResultRows } from "./reportResultRows";

afterEach(() => {
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const here = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(here, "../../../../..");
const VECTORS = path.join(projectRoot, "fixtures/reports/invented/user_release_derivation_vectors.json");
const WRITE_ENV = "OPENPIPESTRESS_WRITE_USER_RELEASE_VECTORS";
const GENERATOR = "apps/desktop/src/features/report/userReleaseDerivation.parity.test.ts";
/** An invented confirmation time (the record's one caller-supplied value). */
const INVENTED_USER_RELEASED_AT = "2026-10-27T00:00:00Z";
const SOFTWARE = { name: desktopAppConfig.productName, version: desktopAppConfig.version };

type Json = Record<string, any>;

const safeId = (value: string) => value.replace(/[^A-Za-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "") || "report-package";

/** A Current preview-physics-2 session as the workspace solves it: the replayed native solve,
 * the input manifest with the solve's fixed settings for the mode (`workspaceSession.ts`) and
 * the solve's own transient run record. */
async function solvedSession(mode: PreviewSolverMode) {
  const pair = nativeMechanicsReplayPair(mode, { profile: "preview" });
  const model = structuredClone(pair.model);
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "preview" }).invoke);
  const result = await runPreviewMechanics(model, mode);
  const producer = result.producer!;
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: producer.component_name,
      solver_version: producer.component_version,
      solver_build_ref: `${producer.component_name}@${producer.component_version}`,
      solver_mode: mode,
      settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: mode === "sparse_interactive" }
    },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, inputManifest, analysisRun, pair };
}

/** The invented record U (§2.3), bound to the session's identities. */
async function inventedRecord(session: Awaited<ReturnType<typeof solvedSession>>, documentEnvelope: Json) {
  const run = session.result.run_id;
  return {
    record_kind: "user_release_of_own_project_data",
    record_version: "1.0.0",
    statement: "I hold the right to share this project's data, and I release my project data in this report for sharing.",
    statement_basis: "docs/IP_AND_DATA_BOUNDARY.md §6: a user intentionally exports their own data with documented redistribution rights",
    standing: "user_statement_of_data_rights_only",
    scope: {
      scope_kind: "this_report_only",
      report_id: `report:${run}`,
      package_id: `desktop-user-released-report-${safeId(run)}`,
      user_released_data_class: "private_project_data",
      not_carried: ["rule_pack_references", "user_rule_outcomes", "run_comparisons", "handoff_records", "library_records", "audit_assets"]
    },
    bound_to: {
      model_ref: session.model.project.id,
      model_document_sha256: await canonicalSha256Hex(session.model),
      analysis_run_ref: run,
      input_manifest_ref: session.inputManifest.manifest_ref.ref,
      input_manifest_sha256: session.inputManifest.manifest_sha256,
      result_envelope_id: documentEnvelope.envelope_id,
      result_document_derivative_sha256: documentEnvelope.reproducibility.derivative_hash.value
    },
    user_released_at: INVENTED_USER_RELEASED_AT
  };
}

const MODES: { mode: PreviewSolverMode; reason: string }[] = [
  {
    mode: "sparse_interactive",
    reason: "user_release_display_rule: the connected preview-physics-2 sparse solve, whose rows include values stored in Pa and values with more than six significant digits; sparse settings"
  },
  {
    mode: "dense_scrutiny",
    reason: "the connected preview-physics-2 dense solve; dense settings (sparse_evidence_lane false)"
  }
];

async function vector({ mode, reason }: (typeof MODES)[number]) {
  const session = await solvedSession(mode);
  const { model, result, inputManifest, analysisRun, pair } = session;
  // The record binds the result document's identity, known before the report is built.
  const probe = await assembleUserReleasedReportPackageRequest({ model, result, analysisRun, inputManifest, userRelease: {} });
  const userRelease = await inventedRecord(session, probe.result_documents[0].result_envelope as Json);
  const request = await assembleUserReleasedReportPackageRequest({ model, result, analysisRun, inputManifest, userRelease });
  const { report, audit_manifest, result_documents, source_results, ...topLevel } = request;
  const document = result_documents[0] as Json;
  expect(source_results).toEqual([result]);
  return {
    name: `connected_${mode}`,
    reason,
    model_fixture: pair.provenance.model_fixture_ref,
    source_fixture: pair.provenance.source_fixture_ref,
    solver_mode: mode,
    software: SOFTWARE,
    user_release: userRelease,
    input_manifest: inputManifest.manifest,
    run_record: analysisRun,
    expected: {
      input_manifest_sha256: inputManifest.manifest_sha256,
      input_manifest_ref: inputManifest.manifest_ref.ref,
      request_sha256: await canonicalSha256Hex(request),
      result_document_sha256: await canonicalSha256Hex(document),
      result_document_derivative_sha256: document.result_envelope.reproducibility.derivative_hash.value,
      top_level: topLevel,
      audit_manifest,
      report
    }
  };
}

async function vectorsDocument() {
  const vectors = [];
  for (const entry of MODES) vectors.push(await vector(entry));
  return {
    schema_version: "1.0.0",
    generator: GENERATOR,
    description:
      "Invented data. W-18b derivation vectors (DEL-08-01 Design report-user-release.md section 2.6): for each vector, the inputs M (model_fixture), S (source_fixture, the producer's bytes), m (solver_mode), I (input_manifest), R (run_record, the solve's own transient record), H (software) and U (user_release, an invented record), and the expected request the desktop builders assemble: its canonical SHA-256, its result document's canonical SHA-256 and derivative hash, and in full its top-level members, audit manifest and minimal renderer input. The Rust derivation (report_package::user_release::derive) must give the same. Unit transport replay of committed producer bytes; not a native UI qualification, not engineering output, and no user release is recorded.",
    vectors
  };
}

describe("the user-released request derivation (W-18b §2.6)", () => {
  it("writes its vectors, user_release_derivation_vectors.json, and fails when they are stale", async () => {
    // Compact: each vector carries a whole run record and minimal renderer input.
    const text = `${JSON.stringify(await vectorsDocument())}\n`;
    const target = process.env[WRITE_ENV];
    if (target) {
      writeFileSync(target, text);
      return;
    }
    const committed = existsSync(VECTORS) ? readFileSync(VECTORS, "utf8") : "";
    expect(text === committed, `fixtures/reports/invented/user_release_derivation_vectors.json is stale; regenerate it with ${WRITE_ENV}=<path>`).toBe(true);
  });

  it("builds the minimal renderer input: the local-private input less the entries R leaves out", async () => {
    const session = await solvedSession("sparse_interactive");
    const { model, result, analysisRun, inputManifest } = session;
    const request = await assembleUserReleasedReportPackageRequest({ model, result, analysisRun, inputManifest, userRelease: { invented: true } });
    const minimal = request.report as Json;
    const local = (await buildRenderableReportInput({ model, result, analysisRun, inputManifest, projectSummary: null, renderContext: { exportContext: "local_private", generatedAt: null } })) as Json;
    expect(request.export_profile_id).toBe(REPORT_PACKAGE_USER_RELEASED_PROFILE);
    expect(request.state_comparison_handoff_records).toEqual([]);
    expect(request.rule_check_aggregate).toBeNull();
    expect(request.solve_rule_check_status).toBe(result.status.rule_check);
    expect(request.audit_manifest.rule_pack_refs).toEqual([]);
    expect(request.audit_manifest.assets).toEqual([]);
    expect(minimal.report_context).toEqual({ ...local.report_context, export_context: USER_RELEASED_REPORT_CONTEXT, user_release: { invented: true } });
    expect(minimal.report_context.generated_at).toBeNull();
    expect(Object.hasOwn(minimal, "user_rule_checks")).toBe(false);
    expect(Object.hasOwn(minimal, "library_references")).toBe(false);
    expect(Object.hasOwn(minimal, "exact_route")).toBe(false);
    expect(minimal.result_rows).toEqual(reportResultRows(result));
    expect(minimal.report_title).toBe(local.report_title);
    // The model of this vector has no per-object provenance, so the two inputs' sections agree.
    expect(minimal.report_sections.diagnostics).toEqual(local.report_sections.diagnostics.slice(0, result.diagnostics.length));
    expect(minimal.report_sections.provenance_notes).toEqual([minimal.calculation_report.provenance]);
    expect(minimal.report_sections.user_supplied_values).toEqual([]);
    expect(minimal.calculation_report.rule_pack_refs).toEqual([]);
    expect(minimal.calculation_report.model_input_summary.persistence_ref).toEqual({ ref_type: "project_persistence", ref_id: "TBD" });
    const { report_section_refs: _minimalRefs, ...minimalReport } = minimal.calculation_report;
    const { report_section_refs: _localRefs, ...localReport } = local.calculation_report;
    if (JSON.stringify(minimal.report_sections) === JSON.stringify(local.report_sections)) expect(minimalReport).toEqual(localReport);
  });

  it("leaves out every per-object provenance entry and finding", async () => {
    const session = await solvedSession("sparse_interactive");
    // An invented model edit outside the solve: the report builders read the model's
    // components and supports directly, so a hanger-bearing model shows what R omits.
    const model = structuredClone(session.model) as Json;
    model.components = [{ id: "component:invented-bend", label: "Invented bend", kind: "bend", provenance: "user entered", modifiers: { source_reference: "user entered" } }];
    model.supports = [...model.supports, { id: "support:invented-hanger", label: "Invented hanger", node: model.nodes[0].id, family: "variable_spring_hanger", restraints: [], hanger: { hanger_type: "variable_spring_hanger", installed_load: { value: 1, unit: "N" }, source_reference: "invented" }, provenance: "invented" }];
    const local = (await buildRenderableReportInput({ model: model as never, result: session.result, analysisRun: session.analysisRun, projectSummary: null, renderContext: { exportContext: "local_private", generatedAt: null } })) as Json;
    const minimal = (await userReleasedRenderableReportInput({ model: model as never, result: session.result, analysisRun: session.analysisRun, inputManifest: session.inputManifest, userRelease: {} })) as Json;
    expect(local.report_sections.user_supplied_values.length).toBeGreaterThan(0);
    expect(local.report_sections.provenance_notes.length).toBeGreaterThan(1);
    expect(minimal.report_sections.user_supplied_values).toEqual([]);
    expect(minimal.report_sections.provenance_notes).toHaveLength(1);
    expect(minimal.report_sections.diagnostics).toHaveLength(session.result.diagnostics.length);
  });

  it("user_release_display_rule: rows show six significant digits and MPa, bound to full-precision checksums", async () => {
    const file = JSON.parse(readFileSync(VECTORS, "utf8")) as { vectors: Json[] };
    const vector = file.vectors.find((item) => item.reason.startsWith("user_release_display_rule"))!;
    const source = JSON.parse(readFileSync(path.join(projectRoot, vector.source_fixture), "utf8")) as MechanicsResult;
    const report = vector.expected.report as Json;
    const pascal = source.results.findIndex((row) => row.unit === "Pa");
    expect(pascal).toBeGreaterThanOrEqual(0);
    expect(report.result_rows[pascal].quantity_display).toMatch(/ MPa$/);
    expect(report.calculation_report.model_input_summary.unit_display_summary.conversion_performed).toBe(true);
    const long = source.results.findIndex((row) => String(row.value).replace(/[-.]|e.*$/g, "").replace(/^0+/, "").length > 6);
    expect(long).toBeGreaterThanOrEqual(0);
    for (const [index, row] of source.results.entries()) {
      expect(report.calculation_report.result_export_refs[index].checksum.value).toBe(await canonicalSha256HexCheckedV1(row));
    }
    // A stored value changed in its seventh significant digit leaves the displayed row
    // unchanged (the step that keeps the row on its rounding side).
    const value = source.results[long].value as number;
    const step = 10 ** (Math.floor(Math.log10(Math.abs(value))) - 6);
    const mutations = [value + step, value - step].map((changed) => {
      const mutated = structuredClone(source);
      mutated.results[long].value = changed;
      return mutated;
    });
    const unchanged = mutations.find((mutated) => JSON.stringify(reportResultRows(mutated)[long]) === JSON.stringify(reportResultRows(source)[long]));
    expect(unchanged).toBeDefined();
    expect(unchanged!.results[long].value).not.toBe(value);
  });
});
