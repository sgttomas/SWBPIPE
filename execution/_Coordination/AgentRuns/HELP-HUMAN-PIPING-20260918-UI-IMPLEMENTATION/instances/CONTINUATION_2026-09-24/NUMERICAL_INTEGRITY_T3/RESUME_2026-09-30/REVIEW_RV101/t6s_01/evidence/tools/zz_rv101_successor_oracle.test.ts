/** RV101 reviewer oracle for the successor (scratch; placed only in the reviewer's candidate copy).
 * Writes inputs and TypeScript outputs for the Rust oracle (zz_rv101_oracle.rs) and records the
 * closure probes and stress-neutral facts as JSONL. Env: RV101_SO_OUT (dir), RV101_I76_INPUTS (dir). */
import { afterEach, expect, it, vi } from "vitest";
import { createHash } from "node:crypto";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../types";
import { canonicalJsonString, canonicalSha256HexCheckedV1 } from "../services/hashService";
import { buildCurrentSessionInputManifest } from "../services/inputManifestService";
import { runPreviewMechanics, type PreviewSolverMode } from "../services/previewService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../services/analysisRunCompatibility";
import { buildCurrentResultExport, currentReceivedOrigin, currentResultDocumentBase, deriveResultDocument, validateResultDocument } from "../features/result-export/resultExportAdapter";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket } from "../features/stress-neutral/StressNeutralExportPanel";
import { loadReferenceOutputRefusal } from "../features/results/loadReferenceOutputAvailability";
import { OUTPUT_SURFACES, RETAINED_PRECISION_OUTPUT_REFUSAL, surfaceOutputRefusal, surfaceRouteRefusal } from "../features/results/outputPolicy";
import { decodeBinary64 } from "../features/results/retainedPrecision";
import { rustLowerExp, retainedClassDisclosure } from "../features/results/retainedPrecisionDisclosure";
import { numericalResultStanding } from "../features/results/numericalResultQuality";
import { registerRetainedPrecision } from "../features/results/retainedPrecisionStanding";
import { reportPackageUnavailableReason } from "../features/report/reportPackageRequest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => { invokeMock.mockReset(); delete (window as Json).__TAURI_INTERNALS__; });
const P = resolve(__dirname, "../../../../");
const OUT = process.env.RV101_SO_OUT!, I76 = process.env.RV101_I76_INPUTS!;
const LOG = resolve(OUT, "facts.jsonl");
const fact = (o: Json) => appendFileSync(LOG, JSON.stringify(o) + "\n");
const sha = (t: string | Buffer) => createHash("sha256").update(t).digest("hex");
const MODES = ["sparse_interactive", "dense_scrutiny"] as const;
const I76_MANIFEST = "test:t6s-golden-reference-only-manifest";
const I76_BUILD = "test:t6s-golden-pinned-producer-test-bytes-not-native-attestation";
const I76_LIMIT = "Test-built desktop-shaped origin (T6S golden): pinned producer test bytes, not a qualified Current received carrier; independent authentic original producer bytes unavailable; dimension absence is not producer attestation";
const STATUS = ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"];
const BOUNDARY = { human_review_required: true, software_makes_approval_claim: false, software_makes_authentication_claim: false, software_makes_certification_claim: false, software_makes_compliance_claim: false, software_makes_sealing_claim: false };
const corpus = JSON.parse(readFileSync(resolve(P, "fixtures/results/retained_precision_cases.json"), "utf8"));

// The corpus harness (as the reader tests apply it): edits, then rehash "all".
function rehashRef(items: any[], ref: unknown): any { return typeof ref === "number" && Number.isInteger(ref) && ref >= 0 && !Object.is(ref, -0) && ref < items.length ? items[ref] : undefined; }
async function rehash(source: any) {
  const body = source.retained_precision?.body;
  if (!body || typeof body !== "object") return;
  for (const s of body.sources) if (s.preparation) {
    const a = rehashRef(body.product_attempts, s.preparation.attempt_ref);
    if (!a || !a.preparation.members.every((m: any) => m.result.kind === "prepared")) continue;
    s.preparation.sha256 = await canonicalSha256HexCheckedV1({ domain: "retained_precision_preparation_v1", payload: { definition_id: a.definition_id, definition_sha256: "a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349", owner_ref: a.owner_ref, ordinary_attempt_ref: a.ordinary_attempt_ref, material_basis_ref: a.material_basis_ref, members: a.preparation.members.map((m: any) => ({ member: m.member, old_source: m.old_source, old_facts: m.old_facts, section: m.result.section })) } });
  }
  for (const c of body.cases) {
    const src = c.status === "selected" ? rehashRef(body.sources, c.source_ref) : undefined;
    if (src) { const { index: _, ...s } = src; c.source_identity_sha256 = await canonicalSha256HexCheckedV1({ domain: "retained_precision_source_mp_v2", payload: s }); }
  }
  const { retained_precision: _, ...publication } = source;
  body.publication_sha256 = await canonicalSha256HexCheckedV1({ domain: "retained_precision_publication_mp_v2", payload: publication });
  source.retained_precision.receipt_sha256 = await canonicalSha256HexCheckedV1({ domain: "retained_precision_receipt_mp_v2", payload: body });
}
function applyEdits(root: any, edits: any[] | undefined) {
  for (const edit of edits ?? []) {
    let value = root; for (const key of edit.path.slice(0, -1)) value = value[key];
    const key = edit.path.at(-1);
    if (edit.op === "remove") { if (Array.isArray(value)) value.splice(key, 1); else delete value[key]; }
    else if (edit.op === "set") value[key] = structuredClone(edit.value);
    else throw new Error("op " + edit.op);
  }
}
async function applyEntry(m: any) {
  const base = corpus.cases.find((c: any) => c.id === m.base), source = structuredClone(base.source), invocation = structuredClone(base.invocation);
  applyEdits(source, m.edits); applyEdits(invocation, m.invocation_edits);
  if (m.invocation_edits?.length) source.retained_precision.body.invocation.value = await canonicalSha256HexCheckedV1({ domain: "source_blocks_invocation_v1", payload: invocation });
  await rehash(source); applyEdits(source, m.after_rehash);
  return { source, invocation };
}
async function productBaseOrigin(model: PreviewModel, source: MechanicsResult, manifestRef: string, build: string, limit: string) {
  let refs: Json[] = []; try { refs = modelLoadBasisRefs(model) as Json[]; } catch { refs = []; }
  const run = { run_id: source.run_id, load_basis_refs: refs, hashes: [], analysis_status: STATUS, professional_boundary: BOUNDARY } as Json;
  const base = currentResultDocumentBase(model, source, run, manifestRef, { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: build });
  const origin = await currentReceivedOrigin(model, source, manifestRef, false, limit);
  return { base, origin };
}
async function writeDerive(stem: string, model: PreviewModel, source: MechanicsResult, base: Json, origin: Json) {
  writeFileSync(resolve(OUT, "derive", `${stem}.input.json`), JSON.stringify({ base, model, source, origin }));
  try {
    const doc = await deriveResultDocument(structuredClone(base), structuredClone(model), structuredClone(source), structuredClone(origin));
    const text = await canonicalJsonString(doc);
    writeFileSync(resolve(OUT, "derive", `${stem}.ts.json`), text);
    let validates = "OK"; try { await validateResultDocument(JSON.parse(text), source); } catch (e) { validates = (e as Error).message; }
    fact({ kind: "derive", stem, ok: true, sha256: sha(text), ts_validates: validates });
    return text;
  } catch (e) {
    writeFileSync(resolve(OUT, "derive", `${stem}.ts.err`), (e as Error).message);
    fact({ kind: "derive", stem, ok: false, error: (e as Error).message });
    return null;
  }
}
function keysDeep(v: unknown, path = "", out: string[] = []): string[] {
  if (Array.isArray(v)) v.forEach((x, i) => keysDeep(x, `${path}/${i}`, out));
  else if (v && typeof v === "object") for (const [k, x] of Object.entries(v)) { out.push(`${path}/${k}`); keysDeep(x, `${path}/${k}`, out); }
  return out;
}

it("RV101 successor oracle", async () => {
  mkdirSync(resolve(OUT, "derive"), { recursive: true }); mkdirSync(resolve(OUT, "sn"), { recursive: true });
  // 1. Golden inputs: I76's recorded base/origin, and the product functions with I76's stand-ins.
  for (const mode of MODES) {
    const pinned = JSON.parse(readFileSync(resolve(P, `fixtures/results/retained_precision_milestone_successor_${mode}.json`), "utf8"));
    const model = pinned.invocation.request.model as PreviewModel, source = pinned.source as MechanicsResult;
    const i76Base = JSON.parse(readFileSync(resolve(I76, `${mode}.base.json`), "utf8")), i76Origin = JSON.parse(readFileSync(resolve(I76, `${mode}.origin.json`), "utf8"));
    const own = await productBaseOrigin(model, source, I76_MANIFEST, I76_BUILD, I76_LIMIT);
    fact({ kind: "golden_inputs", mode, product_base_eq_i76: (await canonicalJsonString(own.base)) === readFileSync(resolve(I76, `${mode}.base.json`), "utf8"), product_origin_eq_i76: (await canonicalJsonString(own.origin)) === readFileSync(resolve(I76, `${mode}.origin.json`), "utf8") });
    const text = await writeDerive(`golden_${mode}`, model, source, i76Base, i76Origin);
    const golden = readFileSync(resolve(P, `fixtures/results/retained_precision_successor_derivative_${mode}.json`), "utf8");
    fact({ kind: "golden_parity", mode, ts_eq_committed_golden: text === golden, golden_sha256: sha(golden), golden_trailing_newline: golden.endsWith("\n") });
    // Claims scan over the derivative.
    const doc = JSON.parse(golden), keys = keysDeep(doc);
    const disclosures = doc.result_envelope.row_disclosures;
    fact({ kind: "golden_claims", mode, request_or_solver_mode_keys: keys.filter(k => /\/(request|solver_mode)$/.test(k)), standing_tokens: ["numerically_eligible", "needs_recompute"].filter(t => golden.includes(t)), authentic_producer_available: doc.result_envelope.reproducibility.source_origin_bindings[0].authentic_producer_available, original_producer_checksum: doc.result_envelope.reproducibility.source_origin_bindings[0].original_producer_checksum, request_hash: doc.result_envelope.reproducibility.request_hash, raw_source_hashes: doc.result_envelope.reproducibility.raw_source_hashes, receipt_equal: (await canonicalJsonString(doc.result_envelope.retained_precision)) === (await canonicalJsonString(source.retained_precision)), contract_evidence_equal: (await canonicalJsonString(doc.result_envelope.contract_evidence)) === (await canonicalJsonString(source.contract_evidence)), class_codes: disclosures.reduce((m: Json, d: Json) => { m[d.reason_code] = (m[d.reason_code] ?? 0) + 1; return m; }, {}), values: doc.result_envelope.result_sets[0].values.length, witnesses: doc.result_envelope.unit_preservation_witnesses.length, plus_exponent: disclosures.filter((d: Json) => /b = [^ ]*e\+/.test(d.message)).length, valued_class_rows: doc.result_envelope.result_sets[0].values.filter((v: Json) => disclosures.some((d: Json) => d.source_result_id === v.result_id)).length });
  }
  // 2. Corpus: every case, must-pass and refusal entry, through the product's base/origin functions.
  for (const c of corpus.cases) {
    const model = c.invocation.request.model as PreviewModel;
    const bo = await productBaseOrigin(model, c.source, "test:rv101-corpus", "test:rv101-corpus-build", "RV101 test-built origin; not a qualified Current received carrier");
    await writeDerive(`case_${c.id}`, model, c.source, bo.base, bo.origin);
  }
  for (const [kind, list] of [["must", corpus.must_pass], ["mut", corpus.mutations]] as const) {
    for (const m of list) {
      let built; try { built = await applyEntry(m); } catch (e) { fact({ kind: "entry_build", stem: `${kind}_${m.id}`, error: (e as Error).message }); continue; }
      const model = built.invocation.request.model as PreviewModel;
      let bo; try { bo = await productBaseOrigin(model, built.source, "test:rv101-corpus", "test:rv101-corpus-build", "RV101 test-built origin; not a qualified Current received carrier"); } catch (e) { fact({ kind: "base_origin", stem: `${kind}_${m.id}`, error: (e as Error).message }); continue; }
      await writeDerive(`${kind}_${m.id}`, model, built.source, bo.base, bo.origin);
    }
  }
  // 3. {:e} words: edges, every corpus/golden bound, and random words (seeded xorshift).
  const words = new Set<string>(["0000000000000000", "8000000000000000", "0000000000000001", "000fffffffffffff", "0010000000000000", "7fefffffffffffff", "3ff0000000000000", "3ff8000000000000", "4093480000000000", "444b1ae4d6e2ef50", "4340000000000000", "3fb999999999999a", "3f50624dd2f1a9fc", "bff0000000000000", "c000000000000000"]);
  for (const c of corpus.cases) for (const r of c.expected_classifications) if (r.bound_bits) words.add(r.bound_bits);
  for (let e = -330; e <= 310; e++) for (const m of [1, 2, 5, 9.999999999999999]) { const v = m * Math.pow(10, e); if (Number.isFinite(v) && v > 0) { const b = new DataView(new ArrayBuffer(8)); b.setFloat64(0, v); words.add([...new Uint8Array(b.buffer)].map(x => x.toString(16).padStart(2, "0")).join("")); } }
  let s = 0x9e3779b97f4a7c15n; const next = () => { s ^= (s << 13n) & 0xffffffffffffffffn; s ^= s >> 7n; s ^= (s << 17n) & 0xffffffffffffffffn; return s & 0xffffffffffffffffn; };
  while (words.size < 60000) { const w = next(); if (((w >> 52n) & 0x7ffn) !== 0x7ffn) words.add(w.toString(16).padStart(16, "0")); const sub = next() & 0x000fffffffffffffn; words.add(sub.toString(16).padStart(16, "0")); }
  const list = [...words];
  writeFileSync(resolve(OUT, "exp_words.txt"), list.join("\n") + "\n");
  writeFileSync(resolve(OUT, "exp_words.txt.ts.txt"), list.map(w => rustLowerExp(decodeBinary64(w))).join("\n") + "\n");
  // 4. class_disclosure combinations.
  const kinds = ["displacement_magnitude", "rotation_rx", "force:axé", "k \"q\"", "", "moment;x"];
  const units = ["m", "mm", "rad", "N", "kN", "N*m", "kN*m", "Pa", "MPa", "in", "", "M", "mm ", "deg", "kPa"];
  const classes = ["relative_verified", "absolute_verified", "input_derived", "non_quantity", "not_covered", "none"];
  const bitsList = ["3b1a378ea78c5ce9", "0000000000000001", "0000000000000000", "7fefffffffffffff", "3ff0000000000000", "4093480000000000"];
  const combos: Json[] = []; for (const k of kinds) for (const u of units) for (const c of classes) for (const b of (c === "absolute_verified" ? bitsList : [null])) combos.push([k, u, c, b]);
  writeFileSync(resolve(OUT, "disc_cases.json"), JSON.stringify(combos));
  writeFileSync(resolve(OUT, "disc_cases.json.ts.json"), JSON.stringify(combos.map(([k, u, c, b]) => { const d = c === "none" ? retainedClassDisclosure(k, u, null) : retainedClassDisclosure(k, u, { result_id: "x", basis_ref: { ref_type: "t", ref_id: "i" }, normalized_bits: "0000000000000000", scale_bits: null, bound_bits: b, class: c } as Json); return d ? [d.code, d.message] : null; })));
  // 5. Closure probes and stress-neutral packages over the product's own capture (mocked IPC only).
  for (const mode of MODES) {
    const pinned = JSON.parse(readFileSync(resolve(P, `fixtures/results/retained_precision_milestone_successor_${mode}.json`), "utf8"));
    const model = pinned.invocation.request.model as PreviewModel, source = pinned.source as MechanicsResult;
    const probe = (label: string, src: MechanicsResult, mdl: PreviewModel) => {
      let standing: Json; try { standing = numericalResultStanding(src, mdl); } catch (e) { standing = { error: (e as Error).message }; }
      fact({ kind: "closure", mode, label, eligible: standing.eligible ?? null, findings: standing.findings ?? standing.error, result_export: surfaceOutputRefusal(src, mdl, "result-export"), stress_neutral: surfaceOutputRefusal(src, mdl, "stress-neutral"), shared: loadReferenceOutputRefusal(src), others_all_refuse_with_shared_text: OUTPUT_SURFACES.filter(x => x !== "result-export" && x !== "stress-neutral").every(x => surfaceOutputRefusal(src, mdl, x) === RETAINED_PRECISION_OUTPUT_REFUSAL && surfaceRouteRefusal(src, x) === RETAINED_PRECISION_OUTPUT_REFUSAL), report_package: reportPackageUnavailableReason(src) });
    };
    probe("file_bytes_unregistered", structuredClone(source), model);
    (window as Json).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async () => structuredClone(source));
    const received = await runPreviewMechanics(structuredClone(model), mode);
    probe("ipc_registered_captured_model", received, model);
    probe("structured_clone_of_registered", structuredClone(received), model);
    probe("json_roundtrip_of_registered", JSON.parse(JSON.stringify(received)), model);
    const moved = structuredClone(model) as Json; moved.nodes[0].position.x += 1;
    probe("registered_moved_model", received, moved);
    probe("registered_null_model", received, null as Json);
    const other = structuredClone(source) as Json; await registerRetainedPrecision(other, pinned.invocation, null);
    probe("registered_without_live_capture", other, model);
    const otherMode = mode === "sparse_interactive" ? "dense_scrutiny" : "sparse_interactive";
    let wrongMode: Json = null; try { invokeMock.mockImplementation(async () => structuredClone(source)); wrongMode = await runPreviewMechanics(structuredClone(model), otherMode); } catch (e) { fact({ kind: "closure", mode, label: "wrong_mode_ipc", error: (e as Error).message }); }
    if (wrongMode) probe("ipc_registered_under_other_solver_mode", wrongMode, model);
    // Current result export and stress-neutral package of the registered successor.
    let manifestEvidence: Json; try { manifestEvidence = await buildCurrentSessionInputManifest({ model, solver: { solver_name: received.producer!.component_name, solver_version: received.producer!.component_version, solver_build_ref: "rv101:test", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] } as Json); fact({ kind: "manifest", mode, product_builder: "ok" }); }
    catch (e) { fact({ kind: "manifest", mode, product_builder: (e as Error).message }); manifestEvidence = { manifest_ref: { object_type: "InputManifest", ref: "manifest:rv101-invented" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: received.model_ref, model_payload: model }, solver_basis: { solver_name: received.producer!.component_name, solver_version: received.producer!.component_version, solver_build_ref: "rv101:test", solver_mode: mode } } }; }
    const analysisRun = await buildAnalysisRunV03(received, manifestEvidence, undefined, modelLoadBasisRefs(model));
    try { const doc = await buildCurrentResultExport({ model, result: received, analysisRun, inputManifest: manifestEvidence }); fact({ kind: "current_export", mode, ok: true, sha256: sha(await canonicalJsonString(doc)) }); writeFileSync(resolve(OUT, "sn", `current_${mode}.json`), await canonicalJsonString(doc)); }
    catch (e) { fact({ kind: "current_export", mode, ok: false, error: (e as Error).message }); }
    const packet: Json = await buildStressNeutralExportPacket({ model, result: received, analysisRun });
    writeFileSync(resolve(OUT, "sn", `package_${mode}.json`), JSON.stringify(packet));
    let v1 = "OK", v2 = "OK"; try { await validateStressNeutralExportPacket(structuredClone(packet), received, analysisRun, modelLoadBasisRefs(model)); } catch (e) { v1 = (e as Error).message; }
    try { await validateStressNeutralExportPacket(structuredClone(packet)); } catch (e) { v2 = (e as Error).message; }
    const findings = packet.diagnostics.filter((d: Json) => String(d.code).startsWith("SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION"));
    fact({ kind: "sn_package", mode, validate_with_source: v1, validate_header_only: v2, schema_version: packet.schema_version, validation_status: packet.validation_report?.validation_status, findings: findings.length, finding_codes: [...new Set(findings.map((d: Json) => d.code))], finding_severities: [...new Set(findings.map((d: Json) => d.severity))], finding_classes: [...new Set(findings.map((d: Json) => d.class))], receipt_equal: (await canonicalJsonString(packet.retained_precision)) === (await canonicalJsonString(source.retained_precision)), contract_evidence_equal: (await canonicalJsonString(packet.contract_evidence)) === (await canonicalJsonString(source.contract_evidence)), request_or_solver_mode_keys: keysDeep(packet).filter(k => /\/(request|solver_mode)$/.test(k)), standing_tokens: ["numerically_eligible", "needs_recompute"].filter(t => JSON.stringify(packet).includes(t)), csv_rows: packet.csv_text.trimEnd().split("\n").length - 1, result_rows: packet.result_rows.length, witnesses: packet.unit_preservation_witnesses.length, loss_exported: packet.loss_report.filter((x: Json) => x.category === "exported").map((x: Json) => x.reason) });
    delete (window as Json).__TAURI_INTERNALS__;
  }
  // The corpus two-case base, with a test stand-in live capture (labelled; not the product capture).
  for (const id of ["two_case_synthetic", "two_case_facade_after_certificate_synthetic"]) {
    const c = corpus.cases.find((x: Json) => x.id === id), src = structuredClone(c.source), model = c.invocation.request.model as PreviewModel;
    await registerRetainedPrecision(src, c.invocation, (m: unknown) => JSON.stringify(m) === JSON.stringify(model));
    const standing = numericalResultStanding(src, model);
    const manifestEvidence = { manifest_ref: { object_type: "InputManifest", ref: "manifest:rv101-invented" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: src.model_ref, model_payload: model }, solver_basis: { solver_name: src.producer.component_name, solver_version: src.producer.component_version, solver_build_ref: "rv101:test-stand-in", solver_mode: "sparse_interactive" } } } as Json;
    let packetFact: Json;
    try { const analysisRun = await buildAnalysisRunV03(src, manifestEvidence, undefined, modelLoadBasisRefs(model)); const packet: Json = await buildStressNeutralExportPacket({ model, result: src, analysisRun }); writeFileSync(resolve(OUT, "sn", `package_${id}.json`), JSON.stringify(packet)); let v = "OK"; try { await validateStressNeutralExportPacket(structuredClone(packet), src, analysisRun, modelLoadBasisRefs(model)); } catch (e) { v = (e as Error).message; } packetFact = { ok: true, validate_with_source: v, findings: packet.diagnostics.filter((d: Json) => String(d.code).startsWith("SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION")).length }; }
    catch (e) { packetFact = { ok: false, error: (e as Error).message }; }
    fact({ kind: "two_case", id, eligible: standing.eligible, findings: standing.findings, packet: packetFact });
  }
  expect(true).toBe(true);
}, 3_600_000);
