/** The cases of `apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json`: the inputs of the unchanged
 * TypeScript record builders (`buildAnalysisRunV03`, `buildAnalysisRunPreview`) and their
 * outcomes, which the native builder (`src-tauri/src/analysis_run_v03_build.rs`) must reproduce
 * byte for byte in the record's stored form (`analysis_run_v03_build_tests.rs`). Written only
 * with ANALYSIS_RUN_V03_CASES_WRITE=1 (`analysisRunV03Parity.test.ts`); otherwise byte-checked.
 *
 * - **Sources.** A file's bytes, a file's top-level member's exact bytes, or a text: the held
 *   bytes the native command reads. The TypeScript reads a value parsed from the same text with
 *   the text kept as received (`receivedFromText`), as previewService keeps a solve's bytes, so
 *   both readers read the same statement.
 * - **Builder cases** call `buildAnalysisRunV03(result, manifest, ruleCheckStatus, loadBasisRefs,
 *   rulePackExclusions)`: every fixture `analysis_run.json` carrier's source (its route, model load
 *   basis and manifest ref, as `loadReferenceAnalysisRun.test.ts` and `exactOutputTwins.test.tsx`
 *   build them), the records and refusals `analysisRunCompatibility.test.ts` builds, the retained
 *   successors, and invented cases for the rule-pack exclusions, manifest and producer bindings,
 *   the checked profile and the builder's `TypeError` points.
 * - **Preview cases** call `buildAnalysisRunPreview(result, {inputManifest, ruleCheckAggregate,
 *   rulePackExclusions})` with a manifest `buildCurrentSessionInputManifest` builds; the native
 *   command receives its evidence without the parsed copy. Each also carries `intake_text`, the
 *   model text the session's solve sends for the model its manifest was built from
 *   (`checkedJsonText(model)`, in the model's own member order, as `startPreviewMechanicsJob` and
 *   `deliveredIntakeText` write it): the native persistence tests start each job on these bytes
 *   and send them, as the session does (`native_persistence_tests.rs`).
 * - **Where.** Outside `fixtures/`, as the desktop-shaped records are (`exactOutputTwins.test.tsx`):
 *   JavaScript writes a float zero as `0`, so a copied receipt here is value- and hash-equal to its
 *   original but not equal as a serde_json value, which the publication corpus
 *   (`core/reporting/result_export/tests/hash_corpus.rs`) requires of every file under `fixtures/`.
 * - **Outcomes.** `{record}`: the record as the JSON value `JSON.stringify` writes; `{error}`: the
 *   refusal's message; `{type_error}`: a JavaScript `TypeError` (the native refusal is then
 *   `ANALYSIS-RUN-INPUT-SHAPE-INVALID`). An `undefined` member cannot be held bytes, so the
 *   compatibility test's `basis_ref: undefined` case has no counterpart here. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../types";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../services/analysisRunCompatibility";
import { buildAnalysisRunPreview } from "../services/previewService";
import { buildCurrentSessionInputManifest, inputManifestRef, type CurrentSessionInputManifestEvidence } from "../services/inputManifestService";
import { canonicalJsonString, canonicalSha256Hex, checkedJsonText } from "../services/hashService";
import { jsonTextKeepingNegativeZero } from "../services/jsonText";
import type { ExcludedRulePackRef } from "../services/rulePackRefDiagnostics";
import { minimalPreviewStatement, syncPreviewCases } from "./retiredPrecision";
import { receivedFromText } from "./receivedText";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const ROOT = resolve(__dirname, "../../../../");
const read = (path: string) => readFileSync(resolve(ROOT, path), "utf8");
const json = (path: string): Json => JSON.parse(read(path));

export const ANALYSIS_RUN_V03_CASES = "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json";

type Source = { path: string; member?: string } | { text: string };
type Outcome = { record: AnalysisRunEnvelope } | { error: string } | { type_error: string };
type Manifest = { manifest_ref: Json; manifest_sha256: Json; manifest: Json };
type Case =
  | { id: string; kind: "builder"; source: Source; manifest: Manifest; rule_check_status?: Json; load_basis_refs?: Json; rule_pack_exclusions: ExcludedRulePackRef[]; expected: Outcome }
  | { id: string; kind: "preview"; source: Source; input_manifest: Pick<CurrentSessionInputManifestEvidence, "manifest_ref" | "manifest_sha256" | "canonical_bytes">; rule_check_aggregate: string | null; rule_pack_exclusions: ExcludedRulePackRef[]; intake_text: string; expected: Outcome };

/** The exact text of a top-level member of a JSON object text (the last, as `Value` keeps). */
export function rawTopLevelMember(text: string, key: string): string {
  let at = 0, found: string | null = null;
  const ws = () => { while (at < text.length && " \t\r\n".includes(text[at])) at += 1; };
  const string = (): string => {
    const start = at; at += 1;
    while (text[at] !== "\"") at += text[at] === "\\" ? 2 : 1;
    at += 1;
    return JSON.parse(text.slice(start, at));
  };
  const skip = () => {
    if (text[at] === "\"") { string(); return; }
    if (text[at] === "{" || text[at] === "[") {
      let depth = 0;
      do {
        if (text[at] === "\"") { string(); continue; }
        if (text[at] === "{" || text[at] === "[") depth += 1;
        if (text[at] === "}" || text[at] === "]") depth -= 1;
        at += 1;
      } while (depth > 0);
      return;
    }
    while (at < text.length && !",}] \t\r\n".includes(text[at])) at += 1;
  };
  ws(); if (text[at] !== "{") throw new Error("not an object text"); at += 1;
  for (;;) {
    ws(); if (text[at] === "}") break;
    const name = string(); ws(); at += 1; ws();
    const start = at; skip();
    if (name === key) found = text.slice(start, at);
    ws(); if (text[at] === ",") at += 1;
  }
  if (found === null) throw new Error(`no member ${key}`);
  return found;
}

/** A case's held text. */
export function heldText(source: Source): string {
  if ("text" in source) return source.text;
  const text = read(source.path);
  return source.member ? rawTopLevelMember(text, source.member) : text;
}

async function outcome(build: () => Promise<AnalysisRunEnvelope>): Promise<Outcome> {
  try {
    return { record: await build() };
  } catch (error) {
    if (error instanceof TypeError) return { type_error: error.message };
    return { error: (error as Error).message };
  }
}

/** A builder case's outcome, read from its held text. */
async function builderCase(id: string, source: Source, manifest: Manifest, options: { rule_check_status?: Json; load_basis_refs?: Json; rule_pack_exclusions?: ExcludedRulePackRef[] } = {}): Promise<Case> {
  const result = receivedFromText<MechanicsResult>(heldText(source));
  const exclusions = options.rule_pack_exclusions ?? [];
  const expected = await outcome(() => buildAnalysisRunV03(result, structuredClone(manifest), options.rule_check_status, options.load_basis_refs === undefined ? undefined : structuredClone(options.load_basis_refs), exclusions));
  return { id, kind: "builder", source, manifest, ...("rule_check_status" in options ? { rule_check_status: options.rule_check_status } : {}), ...("load_basis_refs" in options ? { load_basis_refs: options.load_basis_refs } : {}), rule_pack_exclusions: exclusions, expected };
}

/** A preview case's outcome (`buildAnalysisRunPreview`). */
async function previewCase(id: string, source: Source, inputManifest: CurrentSessionInputManifestEvidence, aggregate: string | null = null, exclusions: ExcludedRulePackRef[] = [], evidence?: Pick<CurrentSessionInputManifestEvidence, "manifest_ref" | "manifest_sha256" | "canonical_bytes">): Promise<Case> {
  const result = receivedFromText<MechanicsResult>(heldText(source));
  const sent = evidence ?? { manifest_ref: inputManifest.manifest_ref, manifest_sha256: inputManifest.manifest_sha256, canonical_bytes: inputManifest.canonical_bytes };
  // The TypeScript reads the manifest the canonical bytes fix, as the native command does.
  const manifest = { ...sent, manifest: JSON.parse(sent.canonical_bytes) } as CurrentSessionInputManifestEvidence;
  const expected = await outcome(() => buildAnalysisRunPreview(result, { inputManifest: manifest, ruleCheckAggregate: aggregate, rulePackExclusions: exclusions }));
  // The model text the session's solve sent for the model this manifest was built from.
  const intake = checkedJsonText(inputManifest.manifest.model_basis.model_payload);
  return { id, kind: "preview", source, input_manifest: sent, rule_check_aggregate: aggregate, rule_pack_exclusions: exclusions, intake_text: intake, expected };
}

const minimalManifest = (result: MechanicsResult, ref: Json = { object_type: "InputManifest", ref: "input-manifest:test" }, build = "build:test"): Manifest => ({
  manifest_ref: ref, manifest_sha256: "1".repeat(64),
  manifest: { model_basis: { model_ref: result.model_ref }, solver_basis: { solver_name: result.producer?.component_name, solver_version: result.producer?.component_version, solver_build_ref: build } },
});
const text = (value: unknown): Source => ({ text: jsonTextKeepingNegativeZero(value) });

/** `analysisRunCompatibility.test.ts`'s synthetic preview-physics-2 statement. */
function syntheticPreview(): MechanicsResult {
  return minimalPreviewStatement({ run_id: "run:test", model_ref: "model:test", cases: [],
    rows: [{ id: "result:test", entity_ref: "node:test", kind: "global_nodal_rotation_x", value: 1, unit: "rad",
      metadata: { component: "nodal_rotation_x", coordinate_system: "global", location: "node", basis: "solved_from_global_linear_system", sign_convention: "positive value follows the right-hand-rule rotation about the global cartesian X axis" } }] as never });
}
function syntheticRecordSource(): MechanicsResult {
  const raw = syntheticPreview();
  raw.results[0].basis_ref = { ref_type: "load_case", ref_id: "load:test" };
  return raw;
}

const MODE: Record<string, string> = { sparse: "sparse_interactive", dense: "dense_scrutiny" };

/** The fixture carriers' sources, each with its model's load basis and the carrier's manifest ref. */
async function carrierCases(): Promise<Case[]> {
  const out: Case[] = [];
  const family = [
    ...["connected-sparse", "connected-dense", "pressure-sparse", "pressure-dense"].map(name => ({ name, folder: "pressure/load_reference", carrier: "pressure_load_state" })),
    ...["n05", "n06", "fields", "mixed", "eigen_motion"].flatMap(base => ["sparse", "dense"].map(mode => ({ name: `${base}-${mode}`, folder: "pressure_source/load_reference_source", carrier: "pressure_source_load_state" }))),
  ];
  for (const { name, folder, carrier } of family) {
    const [base, mode] = [name.slice(0, name.lastIndexOf("-")), name.slice(name.lastIndexOf("-") + 1)];
    const path = `fixtures/product_preview/${folder}/${base}-${MODE[mode]}.raw.json`;
    const carrierPath = `fixtures/results/${carrier}_${name.replace("-", "_")}.analysis_run.json`;
    const result = JSON.parse(read(path)) as MechanicsResult;
    const model = json(`fixtures/product_preview/${folder}/${base}.request.json`).model as PreviewModel;
    const ref = json(carrierPath).analysis_run.reproducibility.input_manifest_refs[0];
    out.push(await builderCase(`carrier:${carrierPath}`, { path }, minimalManifest(result, ref, "unavailable:solver-build-not-supplied"), { load_basis_refs: modelLoadBasisRefs(model) }));
  }
  for (const short of ["sparse", "dense"]) {
    for (const [path, request, twin] of [
      [`fixtures/results/pressure_connected_mechanics_${short}.json`, "core/product_physics/tests/fixtures/exact_pressure_connected_request.json", `fixtures/results/pressure_plain_connected_${short}.analysis_run.json`],
      [`fixtures/product_preview/pressure_source/physics_source/n05-${MODE[short]}.raw.json`, "fixtures/product_preview/pressure_source/physics_source/n05.request.json", `fixtures/results/pressure_source_plain_n05_${short}.analysis_run.json`],
    ]) {
      const result = JSON.parse(read(path)) as MechanicsResult;
      const model = json(request).model as PreviewModel;
      const ref = json(twin).analysis_run.reproducibility.input_manifest_refs[0];
      out.push(await builderCase(`carrier:${twin}`, { path }, minimalManifest(result, ref, "unavailable:solver-build-not-supplied"), { load_basis_refs: modelLoadBasisRefs(model) }));
    }
  }
  return out;
}

/** The records and refusals `analysisRunCompatibility.test.ts` builds. */
async function compatibilityCases(): Promise<Case[]> {
  const out: Case[] = [];
  // The retired producer-less 0.1.0 carrier (`retiredMechanicsResult`'s edit, on the synthetic statement).
  const historical = structuredClone(syntheticPreview()) as unknown as Record<string, unknown>;
  historical.schema_version = "0.1.0";
  for (const key of ["producer", "numerical_quality", "formulation_basis", "contract_evidence"]) delete historical[key];
  out.push(await builderCase("compat:historical", text(historical), minimalManifest(historical as unknown as MechanicsResult)));
  const unknown = { schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "run:test", model_ref: "model:test", status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" }, diagnostics: [], summary: {}, results: [{ id: "result:test", entity_ref: "node:test", kind: "displacement_magnitude", value: 1, unit: "mm" }] } as unknown as MechanicsResult;
  out.push(await builderCase("compat:unknown-header", text(unknown), minimalManifest(unknown)));
  const raw = syntheticPreview(); raw.results[0].value = 1.0000000000000002;
  out.push(await builderCase("compat:preview-binding", text(raw), minimalManifest(raw)));
  const record = syntheticRecordSource();
  const input = minimalManifest(record);
  out.push(await builderCase("compat:record", text(record), input));
  out.push(await builderCase("compat:rule-checked", text(record), input, { rule_check_status: "USER_RULE_CHECKED" }));
  for (const invalid of ["", false, 0, "unknown", "USER_RULE_FAILED"]) out.push(await builderCase(`compat:rule-status:${JSON.stringify(invalid)}`, text(record), input, { rule_check_status: invalid }));
  out.push(await builderCase("compat:rule-status:null", text(record), input, { rule_check_status: null }));
  const noRule = structuredClone(record); delete (noRule.status as Json).rule_check;
  out.push(await builderCase("compat:source-rule-absent", text(noRule), input));
  out.push(await builderCase("compat:source-rule-absent-checked", text(noRule), input, { rule_check_status: "USER_RULE_CHECKED" }));
  for (const rule of ["", false, 0, "unknown"]) {
    const bad = structuredClone(record); bad.status.rule_check = rule as never;
    out.push(await builderCase(`compat:source-rule:${JSON.stringify(rule)}`, text(bad), input, { rule_check_status: "USER_RULE_CHECKED" }));
  }
  const nullRule = structuredClone(record); nullRule.status.rule_check = null as never;
  out.push(await builderCase("compat:source-rule-null", text(nullRule), input));
  const refs = [{ object_type: "LoadCase", ref: "load:test" }, { object_type: "Combination", ref: "comb:requested" }];
  out.push(await builderCase("compat:basis-broad", text(record), input, { load_basis_refs: refs }));
  out.push(await builderCase("compat:basis-empty", text(record), input, { load_basis_refs: [] }));
  for (const [name, basis] of [["dup", [refs[0], refs[0]]], ["not-list", { object_type: "LoadCase", ref: "load:test" }], ["null", null], ["extra", [{ ...refs[0], extra: 1 }]], ["kind", [{ object_type: "Other", ref: "load:test" }]], ["blank", [{ object_type: "LoadCase", ref: "" }]], ["string", ["ab"]]] as const) {
    out.push(await builderCase(`compat:basis-invalid:${name}`, text(record), input, { load_basis_refs: basis }));
  }
  for (const [name, basis] of [["empty", {}], ["blank-type", { ref_type: "", ref_id: "x" }], ["number-type", { ref_type: 3, ref_id: "x" }], ["blank-id", { ref_type: "load_case", ref_id: "" }]] as const) {
    const bad = structuredClone(record); bad.results[0].basis_ref = basis as never;
    out.push(await builderCase(`compat:row-basis:${name}`, text(bad), input));
  }
  const generic = structuredClone(record); generic.results[0].basis_ref = { ref_type: "generic", ref_id: "basis:generic" };
  out.push(await builderCase("compat:basis-generic", text(generic), input));
  const nullBasis = structuredClone(record); nullBasis.results[0].basis_ref = null as never;
  out.push(await builderCase("compat:basis-null", text(nullBasis), input, { load_basis_refs: [] }));
  const noBasis = structuredClone(record); delete noBasis.results[0].basis_ref;
  out.push(await builderCase("compat:basis-absent", text(noBasis), input, { load_basis_refs: [] }));
  for (const key of ["run_id", "model_ref", "row_id"]) for (const value of ["", 0, null]) {
    const bad = structuredClone(record) as Json;
    if (key === "row_id") bad.results[0].id = value; else bad[key] = value;
    out.push(await builderCase(`compat:identity:${key}:${JSON.stringify(value)}`, text(bad), input));
  }
  const cases = structuredClone(record);
  cases.numerical_quality!.cases = [{ basis_ref: { ref_type: "load_case", ref_id: "load:without-row" }, structural_status: "numerically_unresolved", solve_quality: "unresolved", model_matrix_fidelity: "not_assessed", accuracy_evidence: "unresolved", evidence_refs: [] }] as never;
  syncPreviewCases(cases);
  out.push(await builderCase("compat:case-available", text(cases), input));
  out.push(await builderCase("compat:case-uncovered", text(cases), input, { load_basis_refs: [{ object_type: "LoadCase", ref: "load:test" }] }));
  out.push(await builderCase("compat:case-covered", text(cases), input, { load_basis_refs: [{ object_type: "LoadCase", ref: "load:test" }, { object_type: "LoadCase", ref: "load:without-row" }] }));
  // T6S: the committed combination successor's Combination basis.
  const successorPath = "fixtures/results/retained_precision_combination_successor_sparse_interactive.json";
  const doc = json(successorPath), source = doc.source as MechanicsResult, model = doc.invocation.request.model as PreviewModel;
  const own = minimalManifest(source);
  const modelRefs = modelLoadBasisRefs(model);
  out.push(await builderCase("compat:successor-refs", { path: successorPath, member: "source" }, own, { load_basis_refs: modelRefs }));
  out.push(await builderCase("compat:successor-uncovered", { path: successorPath, member: "source" }, own, { load_basis_refs: modelRefs.filter(r => r.object_type !== "Combination") }));
  // The pressure-source reader's receipt and publication refusals, in the record surface's codes.
  const n05 = "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json";
  const n05Source = JSON.parse(read(n05)) as Json;
  const n05Model = json("fixtures/product_preview/pressure_source/physics_source/n05.request.json").model as PreviewModel;
  const badReceipt = structuredClone(n05Source); badReceipt.source_block_recovery.receipt_sha256 = "0".repeat(64);
  out.push(await builderCase("compat:pressure-source-receipt", text(badReceipt), minimalManifest(n05Source), { load_basis_refs: modelLoadBasisRefs(n05Model) }));
  const badPublication = structuredClone(n05Source); badPublication.run_id = "edited-publication";
  out.push(await builderCase("compat:pressure-source-publication", text(badPublication), minimalManifest(badPublication), { load_basis_refs: modelLoadBasisRefs(n05Model) }));
  return out;
}

/** The retained successors (U6d) and the base routes' other fixtures. */
async function routeCases(): Promise<Case[]> {
  const out: Case[] = [];
  for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
    const path = `fixtures/results/retained_precision_milestone_successor_${mode}.json`;
    const doc = json(path), source = doc.source as MechanicsResult;
    out.push(await builderCase(`retained:milestone:${mode}`, { path, member: "source" }, minimalManifest(source, { object_type: "InputManifest", ref: "manifest:invented-retained" }, "test:invented"), { load_basis_refs: modelLoadBasisRefs(doc.invocation.request.model) }));
  }
  // Held integers written as integral floats: the retained reader reads JavaScript's text of the
  // value (`validateRetainedPrecision`), where each is an integer again; the base dispatch reads
  // the held bytes.
  const milestoneMember = heldText({ path: "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json", member: "source" });
  const milestoneModel = json("fixtures/results/retained_precision_milestone_successor_sparse_interactive.json").invocation.request.model;
  for (const [name, integer, float] of [["receipt-version", '"receipt_version": 1,', '"receipt_version": 1.0,'], ["case-limit", '"case_limit": 20000000000,', '"case_limit": 20000000000.0,'], ["invocation-after", '"invocation_after": 2377759,', '"invocation_after": 2377759.0,'], ["build-work", '"work": 250377', '"work": 250377.0']]) {
    if (!milestoneMember.includes(integer)) throw new Error(`no ${integer}`);
    const edited = milestoneMember.split(integer).join(float);
    const source = JSON.parse(edited) as MechanicsResult;
    out.push(await builderCase(`retained:integral-float:${name}`, { text: edited }, minimalManifest(source, { object_type: "InputManifest", ref: "manifest:invented-retained" }, "test:invented"), { load_basis_refs: modelLoadBasisRefs(milestoneModel) }));
  }
  const milestone = json("fixtures/results/retained_precision_milestone_successor_sparse_interactive.json");
  const edited = structuredClone(milestone.source); edited.results[0].value = 12345;
  out.push(await builderCase("retained:edited-receipt", text(edited), minimalManifest(edited), { load_basis_refs: modelLoadBasisRefs(milestone.invocation.request.model) }));
  for (const [path, modelPath] of [
    ["fixtures/results/preview_physics_connected_sparse.json", "fixtures/model_operations/precision_connected_ui_model.json"],
    ["fixtures/results/preview_physics_connected_dense.json", "fixtures/model_operations/precision_connected_ui_model.json"],
    ["fixtures/results/preview_physics_unicode_ids_sparse.json", "core/product_physics/tests/fixtures/preview_physics_unicode_ids_model.json"],
    ["fixtures/results/preview_physics_timoshenko_partial_span_blocked_sparse.json", null],
  ] as const) {
    const result = JSON.parse(read(path)) as MechanicsResult;
    const model = modelPath ? json(modelPath) : null;
    out.push(await builderCase(`route:${path}`, { path }, minimalManifest(result), model ? { load_basis_refs: modelLoadBasisRefs(model.model ?? model) } : {}));
  }
  return out;
}

/** Invented cases: the rule-pack exclusions, the manifest and producer bindings, the checked
 * profile and the builder's `TypeError` points. */
async function inventedCases(): Promise<Case[]> {
  const out: Case[] = [];
  const record = syntheticRecordSource();
  const input = minimalManifest(record);
  const withPacks = (active: Json[], refs: Json[]): Manifest => ({ ...input, manifest: { ...input.manifest, active_rule_packs: active, model_basis: { model_ref: record.model_ref, model_payload: { rule_pack_refs: refs } } } });
  const pack = (id: string, version: string) => ({ rule_pack_id: id, rule_pack_version: version, checksum_sha256: "2".repeat(64) });
  const long = `pack:${"δ".repeat(170)}`;
  const refs = [pack("pack:a", "1.0"), pack("pack%b@x", "2@0"), pack(long, "1")];
  const active = [{ rule_pack_id: "pack:a", version: "1.0" }];
  const excluded: ExcludedRulePackRef[] = [{ rule_pack_id: "pack%b@x", rule_pack_version: "2@0", code: "MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH" }, { rule_pack_id: long, rule_pack_version: "1", code: "MODEL-RULE-PACK-REF-UNRESOLVED" }];
  out.push(await builderCase("packs:excluded", text(record), withPacks(active, refs), { rule_pack_exclusions: excluded }));
  out.push(await builderCase("packs:mismatch", text(record), withPacks(active, refs), { rule_pack_exclusions: excluded.slice(0, 1) }));
  out.push(await builderCase("packs:not-resolved", text(record), withPacks([], refs)));
  out.push(await builderCase("packs:refs-not-list", text(record), withPacks(active, { a: 1 } as never)));
  out.push(await builderCase("packs:active-not-list", { ...text(record) }, { ...input, manifest: { ...input.manifest, active_rule_packs: "x", model_basis: { model_ref: record.model_ref, model_payload: { rule_pack_refs: refs } } } }));
  out.push(await builderCase("packs:unknown-code", text(record), withPacks(active, refs), { rule_pack_exclusions: [excluded[0], { ...excluded[1], code: "INVENTED-CODE" as never }] }));
  out.push(await builderCase("packs:repeated", text(record), withPacks(active, [...refs, refs[1]]), { rule_pack_exclusions: [excluded[0], excluded[1], excluded[0]] }));
  out.push(await builderCase("manifest:model-mismatch", text(record), { ...input, manifest: { ...input.manifest, model_basis: { model_ref: "model:other" } } }));
  out.push(await builderCase("manifest:producer-mismatch", text(record), { ...input, manifest: { ...input.manifest, solver_basis: { ...input.manifest.solver_basis, solver_name: "other-solver" } } }));
  out.push(await builderCase("manifest:solver-name-absent", text(record), { ...input, manifest: { ...input.manifest, solver_basis: { solver_version: "0.2.0", solver_build_ref: "build:test" } } }));
  out.push(await builderCase("manifest:solver-name-unsafe", text(record), { ...input, manifest: { ...input.manifest, solver_basis: { ...input.manifest.solver_basis, solver_name: 1e300 } } }));
  out.push(await builderCase("manifest:solver-name-object", text(record), { ...input, manifest: { ...input.manifest, solver_basis: { ...input.manifest.solver_basis, solver_name: { name: "open_pipe_stress_product_physics" } } } }));
  out.push(await builderCase("manifest:solver-basis-absent", text(record), { ...input, manifest: { model_basis: input.manifest.model_basis } }));
  out.push(await builderCase("manifest:model-basis-absent", text(record), { ...input, manifest: { solver_basis: input.manifest.solver_basis } }));
  out.push(await builderCase("manifest:ref-null", text(record), { ...input, manifest_ref: null }));
  out.push(await builderCase("manifest:ref-extra-members", text(record), { ...input, manifest_ref: { object_type: "InputManifest", ref: "input-manifest:test", "é": -0, "𝄞": [1.5, 2] } }));
  out.push(await builderCase("manifest:sha-number", text(record), { ...input, manifest_sha256: 12345 }));
  // The producer's diagnostics are copied whole: other keys, signed zero and integral floats.
  const diagnosed = structuredClone(record);
  (diagnosed as Json).diagnostics = [{ id: "diagnostic:invented", code: "INVENTED", severity: "info", message: "Invented – δ 𝄞", "z": -0, "é": 1, "𝄞": 2.5, "\ue000": [0.000001, 1.5e-7, 1e15] }];
  out.push(await builderCase("source:diagnostics-copied", text(diagnosed), input));
  const unsafeDiagnostic = structuredClone(record);
  (unsafeDiagnostic as Json).diagnostics = [{ id: "diagnostic:invented", code: "INVENTED", severity: "info", message: "x", count: 9007199254740993 }];
  out.push(await builderCase("source:diagnostic-unsafe", { text: jsonTextKeepingNegativeZero(unsafeDiagnostic).replace("9007199254740992", "9007199254740993") }, input));
  const notList = structuredClone(record); (notList as Json).diagnostics = { a: 1 };
  out.push(await builderCase("source:diagnostics-not-list", text(notList), input));
  const twoRows = structuredClone(record); twoRows.results.push({ ...structuredClone(twoRows.results[0]) });
  out.push(await builderCase("source:duplicate-ids", text(twoRows), input));
  const mechanics = structuredClone(record); mechanics.status.mechanics = "MODEL_INCOMPLETE" as never;
  out.push(await builderCase("source:mechanics-incomplete", text(mechanics), input));
  return out;
}

/** `buildAnalysisRunPreview` with session manifests. */
async function previewCases(): Promise<Case[]> {
  const out: Case[] = [];
  const manifest = async (result: MechanicsResult, model: PreviewModel, active: Json[] = [], mode = "sparse_interactive") => buildCurrentSessionInputManifest({
    model, active_rule_packs: active, external_assets: [],
    solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: `${result.producer!.component_name}@${result.producer!.component_version}`, solver_mode: mode, settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: mode === "sparse_interactive" } },
  });
  const previewPath = "fixtures/results/preview_physics_connected_sparse.json";
  const preview = JSON.parse(read(previewPath)) as MechanicsResult;
  const previewModel = json("fixtures/model_operations/precision_connected_ui_model.json") as PreviewModel;
  const evidence = await manifest(preview, previewModel);
  out.push(await previewCase("preview:connected", { path: previewPath }, evidence));
  out.push(await previewCase("preview:aggregate-failed", { path: previewPath }, evidence, "USER_RULE_FAILED"));
  out.push(await previewCase("preview:aggregate-unrecognized", { path: previewPath }, evidence, "PASSED"));
  out.push(await previewCase("preview:evidence-incomplete", { path: previewPath }, evidence, null, [], { ...evidence, manifest_ref: { object_type: "Other", ref: evidence.manifest_ref.ref } as never }));
  out.push(await previewCase("preview:evidence-blank-ref", { path: previewPath }, evidence, null, [], { ...evidence, manifest_ref: { object_type: "InputManifest", ref: " 　" } }));
  out.push(await previewCase("preview:evidence-sha", { path: previewPath }, evidence, null, [], { ...evidence, manifest_sha256: "A".repeat(64) }));
  out.push(await previewCase("preview:hash-mismatch", { path: previewPath }, evidence, null, [], { ...evidence, manifest_sha256: "0".repeat(64) }));
  out.push(await previewCase("preview:ref-mismatch", { path: previewPath }, evidence, null, [], { ...evidence, manifest_ref: { object_type: "InputManifest", ref: "input-manifest:other" } }));
  out.push(await previewCase("preview:not-canonical", { path: previewPath }, evidence, null, [], { ...evidence, canonical_bytes: JSON.stringify(JSON.parse(evidence.canonical_bytes), null, 1) }));
  // A model with rule-pack references: one resolved, one left out (its diagnostic is recorded).
  const reference = (id: string, version: string, digit: string): NonNullable<PreviewModel["rule_pack_refs"]>[number] => ({
    rule_pack_id: id, name: `Invented ${id}`, rule_pack_version: version, redistribution_status: "invented_non_engineering_example",
    checksum: { algorithm: "sha256", canonicalization: "JCS", payload_scope: "rule_pack_payload", payload_excludes: ["checksums"], value: digit.repeat(64) },
  });
  const packed: PreviewModel = { ...structuredClone(previewModel), rule_pack_refs: [reference("pack:resolved", "1.0.0", "3"), reference("pack:missing", "2.0.0", "4")] };
  const packedEvidence = await manifest(preview, packed, [{ rule_pack_id: "pack:resolved", version: "1.0.0", checksum_sha256: "3".repeat(64), source_notice: "Invented rule pack." }]);
  out.push(await previewCase("preview:rule-pack-excluded", { path: previewPath }, packedEvidence, "USER_RULE_CHECKED", [{ rule_pack_id: "pack:missing", rule_pack_version: "2.0.0", code: "MODEL-RULE-PACK-REF-UNRESOLVED" }]));
  out.push(await previewCase("preview:rule-pack-unnamed", { path: previewPath }, packedEvidence));
  const pressurePath = "fixtures/results/pressure_connected_ui_mechanics_sparse.json";
  const pressure = JSON.parse(read(pressurePath)) as MechanicsResult;
  out.push(await previewCase("preview:pressure", { path: pressurePath }, await manifest(pressure, json("fixtures/model_operations/exact_pressure_authoring_model.json"))));
  const n05Path = "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json";
  const n05 = JSON.parse(read(n05Path)) as MechanicsResult;
  const n05Model = json("fixtures/product_preview/pressure_source/physics_source/n05.request.json").model as PreviewModel;
  const withStatus = (model: PreviewModel) => ({ ...model, load_cases: model.load_cases.map(c => ({ ...c, status: c.status ?? "invented_test_status" })) }) as PreviewModel;
  out.push(await previewCase("preview:pressure-source", { path: n05Path }, await manifest(n05, withStatus(n05Model))));
  const retainedPath = "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json";
  const retained = json(retainedPath);
  out.push(await previewCase("preview:retained", { path: retainedPath, member: "source" }, await manifest(retained.source, withStatus(retained.invocation.request.model))));
  out.push(await previewCase("preview:other-model", { path: previewPath }, await manifest(pressure, json("fixtures/model_operations/exact_pressure_authoring_model.json"))));
  out.push(await previewCase("preview:status-null", text({ ...preview, status: null }), evidence));
  const surrogate = structuredClone(preview) as Json; surrogate.results[0].metadata.basis = "\ud800";
  out.push(await previewCase("preview:lone-surrogate", text(surrogate), evidence));
  // Manifests the session's builder would not write, sealed with their own hashes and refs.
  const sealed = async (edit: (manifest: Json) => void): Promise<CurrentSessionInputManifestEvidence> => {
    const edited = structuredClone(evidence.manifest) as Json; edit(edited);
    const sha = await canonicalSha256Hex(edited);
    return { manifest: edited, manifest_ref: { object_type: "InputManifest", ref: inputManifestRef(edited.model_basis.model_ref, sha) }, manifest_sha256: sha, canonical_bytes: await canonicalJsonString(edited) };
  };
  out.push(await previewCase("preview:payload-load-cases-absent", { path: previewPath }, await sealed(m => { delete m.model_basis.model_payload.load_cases; })));
  out.push(await previewCase("preview:payload-combination-id-absent", { path: previewPath }, await sealed(m => { m.model_basis.model_payload.combinations = [{ name: "invented" }]; })));
  out.push(await previewCase("preview:payload-project-mismatch", { path: previewPath }, await sealed(m => { m.model_basis.model_payload.project.id = "project:other"; })));
  out.push(await previewCase("preview:solver-basis-absent", { path: previewPath }, await sealed(m => { delete m.solver_basis; })));
  return out;
}

export type AnalysisRunV03Cases = Readonly<{ document_kind: string; version: number; note: string; cases: Case[] }>;

/** Every case, in a fixed order, with the TypeScript builders' outcomes. */
export async function analysisRunV03Cases(): Promise<AnalysisRunV03Cases> {
  const cases = [...(await compatibilityCases()), ...(await inventedCases()), ...(await routeCases()), ...(await previewCases()), ...(await carrierCases())];
  const ids = new Set(cases.map(c => c.id));
  if (ids.size !== cases.length) throw new Error("duplicate case ids");
  return {
    document_kind: "swbpipe.desktop.analysis_run_v03_cases",
    version: 1,
    note: "Written by apps/desktop/src/test-support/analysisRunV03Cases.ts (ANALYSIS_RUN_V03_CASES_WRITE=1): the TypeScript record builders' outcomes the native builder must reproduce byte for byte in the record's stored form. Invented and committed-fixture inputs only.",
    cases,
  };
}

/** The file's text: one case per line. */
export function casesText(file: AnalysisRunV03Cases): string {
  const { cases, ...head } = file;
  const opening = JSON.stringify(head).slice(0, -1);
  return `${opening},"cases":[\n${cases.map(c => JSON.stringify(c)).join(",\n")}\n]}\n`;
}

export type { Case as AnalysisRunV03Case };
