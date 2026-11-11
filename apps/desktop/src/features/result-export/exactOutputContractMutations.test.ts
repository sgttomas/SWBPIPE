/** Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.2, §4.3, §6.1): the contract
 * mutations of the exact identities' result JSON document and stress-neutral package,
 * each refused by name. The bases are the committed cross-language twins (Rust's
 * `*.document.json`, Python's `*.stress_neutral.json` and `*.analysis_run.json`) with
 * their source raws, for one pressure-2 and one pressure-source-1 load/reference-state
 * input in both modes. Every edit is resealed (the document's derivative hash; the
 * package's member, manifest and package checksums), so only the check under test can
 * refuse. No digest is pinned: each is derived from the fixtures. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { canonicalSha256Hex, canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { RETAINED_PRECISION_DOWNGRADE_FORBIDDEN } from "../results/numericalResultQuality";
import { readerValidate, sourceContract } from "../results/resultReader";
import { validateStressNeutralExportPacket } from "../stress-neutral/StressNeutralExportPanel";
import { CURRENT_ORIGIN_LIMIT, currentReceivedOrigin, deriveResultDocument, validateResultDocument, type JsonObject } from "./resultExportAdapter";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const MODES = [["sparse_interactive", "sparse"], ["dense_scrutiny", "dense"]] as const;
const IDENTITIES = [
  { route: "pressure", folder: "pressure/load_reference", stem: "pressure_load_state", name: "connected" },
  { route: "pressure_source", folder: "pressure_source/load_reference_source", stem: "pressure_source_load_state", name: "n05" },
] as const;

/** One committed twin set: the source raw and request model its writers read, Rust's
 * document, Python's package and Python's record. */
function twins(identity: (typeof IDENTITIES)[number], mode: readonly [string, string]) {
  const [solverMode, short] = mode;
  const raw = json(`fixtures/product_preview/${identity.folder}/${identity.name}-${solverMode}.raw.json`) as MechanicsResult;
  const model = json(`fixtures/product_preview/${identity.folder}/${identity.name}.request.json`).model as PreviewModel;
  const twin = `fixtures/results/${identity.stem}_${identity.name}_${short}`;
  return { raw, model, document: json(`${twin}.document.json`) as JsonObject, packet: json(`${twin}.stress_neutral.json`), record: json(`${twin}.analysis_run.json`) as AnalysisRunEnvelope };
}
/** Recomputes the derivative hash after a deliberate edit. */
async function resealDocument(doc: JsonObject): Promise<JsonObject> {
  const payload = structuredClone(doc); delete payload.result_envelope.reproducibility.derivative_hash;
  doc.result_envelope.reproducibility.derivative_hash.value = await canonicalSha256Hex(payload);
  return doc;
}
async function rawSha(text: string) { const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)); return [...new Uint8Array(digest)].map(x => x.toString(16).padStart(2, "0")).join(""); }
/** Recomputes every member, manifest and package checksum after a deliberate edit
 * (the recipe of `retainedPrecisionStressNeutral.test.tsx`). */
async function resealPacket(packet: Json): Promise<Json> {
  const payloads: Json = { "stress_neutral_results.csv": packet.csv_text, "result_rows.json": packet.result_rows, "unit_system_disclosure.json": packet.unit_system_disclosure, "unit_preservation_witnesses.json": packet.unit_preservation_witnesses, "stable_id_map.json": packet.stable_id_map, "loss_report.json": packet.loss_report, "validation_report.json": packet.validation_report, "diagnostics.json": packet.diagnostics };
  const non = packet.manifest.checksums.filter((x: Json) => x.payload_ref.ref !== "manifest.json");
  for (const c of non) c.value = c.payload_ref.ref.endsWith(".csv") ? await rawSha(payloads[c.payload_ref.ref]) : await canonicalSha256HexCheckedV1(payloads[c.payload_ref.ref]);
  const seed = { manifest_id: packet.manifest.manifest_id, source_result_ref: packet.source_result_ref, source_run_ref: packet.source_run_ref, source_model_ref: packet.source_model_ref, received_source_checksums: packet.received_source_checksums, unresolved_assumption_refs: packet.unresolved_assumption_refs, reproducibility_refs: packet.reproducibility_refs, export_profile_ref: packet.manifest.export_profile_ref, boundary_notes: packet.manifest.boundary_notes, member_checksums: non, diagnostics: packet.diagnostics };
  const m = packet.manifest.checksums.find((x: Json) => x.payload_ref.ref === "manifest.json"); m.value = await canonicalSha256HexCheckedV1(seed);
  const by = new Map(packet.manifest.checksums.map((x: Json) => [x.payload_ref.ref, x]));
  for (const member of packet.manifest.package_members) member.checksum = structuredClone(by.get(member.filename));
  const projection = structuredClone(packet); delete projection.package_checksum;
  packet.package_checksum.value = await canonicalSha256HexCheckedV1(projection);
  return packet;
}
/** The pressure-source-1 receipt of the other identity's twin (a well-formed receipt). */
const foreignReceipt = (short: string) => json(`fixtures/product_preview/pressure_source/load_reference_source/n05-${short === "sparse" ? "sparse_interactive" : "dense_scrutiny"}.raw.json`).source_block_recovery;
/** A retained-precision successor's receipt (a well-formed receipt of another identity). */
const retainedReceipt = (solverMode: string) => json(`fixtures/results/retained_precision_exact_successor_${solverMode}.json`).source.retained_precision;
/** A different, still well-shaped contract_evidence: one published identifier changed. */
const changeEvidence = (evidence: Json) => { evidence.load_reference_states[0].reference_configuration_id += ":changed"; };

describe.each(MODES)("%s: contract mutations of the exact identities' outputs are refused by name (§6.1)", (...mode) => {
  it.each(IDENTITIES)("$route: the unmutated twins are accepted, and contract_evidence that differs from its source's is refused", async (identity) => {
    const { raw, model, document, packet, record } = twins(identity, mode);
    expect(sourceContract(raw)).toBe(identity.route);
    await expect(validateResultDocument(structuredClone(document), raw)).resolves.toBeUndefined();
    await expect(validateStressNeutralExportPacket(structuredClone(packet), raw, record, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    const doc = structuredClone(document); changeEvidence(doc.result_envelope.contract_evidence);
    await expect(validateResultDocument(await resealDocument(doc), raw)).rejects.toThrow("SOURCE_PHYSICAL_METADATA_MISMATCH");
    const pkg = structuredClone(packet); changeEvidence(pkg.contract_evidence);
    await expect(validateStressNeutralExportPacket(await resealPacket(pkg), raw, record)).rejects.toThrow("SN-PHYSICAL-EVIDENCE-MISMATCH");
    const missing = structuredClone(packet); delete missing.contract_evidence;
    await expect(validateStressNeutralExportPacket(await resealPacket(missing), raw, record)).rejects.toThrow("SN-PHYSICAL-EVIDENCE-MISMATCH");
  }, 60_000);

  it("pressure-2: a receipt is refused on its document (SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN) and its package (SN-SOURCE-RECOVERY-CONTRADICTION)", async () => {
    const { raw, document, packet, record } = twins(IDENTITIES[0], mode);
    const doc = structuredClone(document); doc.result_envelope.source_block_recovery = foreignReceipt(mode[1]);
    await expect(validateResultDocument(await resealDocument(doc), raw)).rejects.toThrow("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN");
    const pkg = structuredClone(packet); pkg.source_block_recovery = foreignReceipt(mode[1]);
    await expect(validateStressNeutralExportPacket(await resealPacket(pkg), raw, record)).rejects.toThrow("SN-SOURCE-RECOVERY-CONTRADICTION");
  }, 60_000);

  it("pressure-source-1: a missing or changed receipt is refused on its document and its package", async () => {
    const { raw, document, packet, record } = twins(IDENTITIES[1], mode);
    const missingDoc = structuredClone(document); delete missingDoc.result_envelope.source_block_recovery;
    await expect(validateResultDocument(await resealDocument(missingDoc), raw)).rejects.toThrow("SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH");
    const missingPkg = structuredClone(packet); delete missingPkg.source_block_recovery;
    await expect(validateStressNeutralExportPacket(await resealPacket(missingPkg), raw, record)).rejects.toThrow("SN-SOURCE-RECOVERY-CONTRADICTION");
    // A changed receipt: Rust's own receipt mutation (load_reference_source_contract.rs).
    const changedDoc = structuredClone(document); changedDoc.result_envelope.source_block_recovery.body.policy = "PHYSICS-SOURCE-1";
    await expect(validateResultDocument(await resealDocument(changedDoc), raw)).rejects.toThrow("SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH");
    const changedPkg = structuredClone(packet); changedPkg.source_block_recovery.body.policy = "PHYSICS-SOURCE-1";
    await expect(validateStressNeutralExportPacket(await resealPacket(changedPkg), raw, record)).rejects.toThrow("SN-SOURCE-BLOCK-RECEIPT-MISMATCH");
    // Another statement's (well-formed) receipt is a changed receipt too.
    const other = foreignReceipt(mode[1] === "sparse" ? "dense" : "sparse");
    const swappedDoc = structuredClone(document); swappedDoc.result_envelope.source_block_recovery = structuredClone(other);
    await expect(validateResultDocument(await resealDocument(swappedDoc), raw)).rejects.toThrow("SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH");
    const swappedPkg = structuredClone(packet); swappedPkg.source_block_recovery = structuredClone(other);
    await expect(validateStressNeutralExportPacket(await resealPacket(swappedPkg), raw, record)).rejects.toThrow("SN-SOURCE-BLOCK-RECEIPT-MISMATCH");
  }, 60_000);

  it.each(IDENTITIES)("$route: a retained-precision receipt is refused (RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)", async (identity) => {
    const { raw, document, packet, record } = twins(identity, mode);
    const doc = structuredClone(document); doc.result_envelope.retained_precision = retainedReceipt(mode[0]);
    await expect(validateResultDocument(await resealDocument(doc), raw)).rejects.toThrow(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
    const pkg = structuredClone(packet); pkg.retained_precision = retainedReceipt(mode[0]);
    await expect(validateStressNeutralExportPacket(await resealPacket(pkg), raw, record)).rejects.toThrow(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN);
  }, 60_000);

  it("pressure-2: a package validated without its source keeps Rust's refusal (SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED); pressure-source-1's transport reader reads its header", async () => {
    const pressure = twins(IDENTITIES[0], mode);
    await expect(validateStressNeutralExportPacket(structuredClone(pressure.packet))).rejects.toThrow("SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED");
    const source = twins(IDENTITIES[1], mode);
    await expect(validateStressNeutralExportPacket(structuredClone(source.packet))).resolves.toBeUndefined();
  }, 60_000);

  it("pressure-source-1: a source with a row mutated and its hashes recomputed coherently passes the shallow check and is refused by the full reader", async () => {
    const { raw, model, document, packet, record } = twins(IDENTITIES[1], mode);
    const engine = await loadWasmEngine();
    // The writers' reseal (`load_reference_source_mutations.json`, `reseal` op): the
    // publication hash over the source less its receipt, then the receipt hash over its body.
    const hash = (domain: string, payload: unknown) => engine.canonicalSha256HexCheckedV1(`{"domain":${JSON.stringify(domain)},"payload":${jsonTextKeepingNegativeZero(payload)}}`);
    const resealSource = (source: Json) => {
      const publication = structuredClone(source); delete publication.source_block_recovery;
      source.source_block_recovery.body.publication_sha256 = hash("source_blocks_publication_v1", publication);
      source.source_block_recovery.receipt_sha256 = hash("source_blocks_receipt_v1", source.source_block_recovery.body);
      return source as MechanicsResult;
    };
    // The reseal recipe reproduces the committed hashes on the unmutated source.
    const unmutated = resealSource(structuredClone(raw));
    expect(unmutated.source_block_recovery).toStrictEqual(raw.source_block_recovery);
    // A misdimensioned reaction row (the pressure-2 suite's "misdimensioned reaction").
    const mutated = structuredClone(raw) as Json;
    const row = mutated.results.find((r: Json) => r.kind === "support_reaction_component_v2" && r.metadata?.component === "Mx");
    expect(row.unit).toBe("N*m");
    row.unit = "N";
    resealSource(mutated);
    expect(mutated.source_block_recovery.receipt_sha256).not.toBe((raw.source_block_recovery as Json).receipt_sha256);
    // reader_verdict.rs `source_recovery`: header, transport metadata and publication only.
    expect(() => readerValidate("source-recovery", mutated)).not.toThrow();
    expect(() => readerValidate("pressure-source", mutated)).toThrow("SOURCE_COMPONENT_CONTRADICTION: support_reaction_component_v2");
    // The pure projection, the document validator and the package validator read the full reader.
    const origin = await currentReceivedOrigin(model, mutated, "test:coherent-reseal", CURRENT_ORIGIN_LIMIT);
    const base = structuredClone(document); delete base.result_envelope.contract_evidence; delete base.result_envelope.source_block_recovery;
    await expect(deriveResultDocument(base, model, mutated, origin)).rejects.toThrow("SOURCE_COMPONENT_CONTRADICTION: support_reaction_component_v2");
    const doc = structuredClone(document); doc.result_envelope.source_block_recovery = structuredClone(mutated.source_block_recovery);
    await expect(validateResultDocument(await resealDocument(doc), mutated)).rejects.toThrow("SOURCE_COMPONENT_CONTRADICTION: support_reaction_component_v2");
    // The package validator, with the committed record: the record validator reads the
    // source through the full reader (DEL-14-02) before it compares the committed receipt.
    // (A package made coherent with this source: the test below.)
    const pkg = structuredClone(packet); pkg.source_block_recovery = structuredClone(mutated.source_block_recovery);
    await expect(validateStressNeutralExportPacket(await resealPacket(pkg), mutated, record)).rejects.toThrow("SOURCE_COMPONENT_CONTRADICTION: support_reaction_component_v2");
  }, 60_000);
  it("pressure-source-1: a coherent package of a coherently resealed source that the full reader refuses is refused by the package validator (design §3)", async () => {
    const { raw, model, packet, record } = twins(IDENTITIES[1], mode);
    const engine = await loadWasmEngine();
    const hash = (domain: string, payload: unknown) => engine.canonicalSha256HexCheckedV1(`{"domain":${JSON.stringify(domain)},"payload":${jsonTextKeepingNegativeZero(payload)}}`);
    // A row value changed and the source resealed coherently (publication, then receipt).
    const id = "result:force:independent-member:axial", value = 0.125;
    const mutated = structuredClone(raw) as Json;
    const row = mutated.results.find((r: Json) => r.id === id);
    expect(row.value).not.toBe(value);
    row.value = value;
    const publication = structuredClone(mutated); delete publication.source_block_recovery;
    mutated.source_block_recovery.body.publication_sha256 = hash("source_blocks_publication_v1", publication);
    mutated.source_block_recovery.receipt_sha256 = hash("source_blocks_receipt_v1", mutated.source_block_recovery.body);
    expect(() => readerValidate("source-recovery", mutated)).not.toThrow();
    expect(() => readerValidate("pressure-source", mutated)).toThrow("SOURCE_BLOCKS_PROJECTION_VALUE_BINDING");
    // The package made coherent with that source: its row, CSV value, annotation, witness,
    // received-source checksums and receipt, then every member, manifest and package checksum.
    const pkg = structuredClone(packet);
    pkg.result_rows.find((r: Json) => r.result_id === id).value = value;
    pkg.csv_text = pkg.csv_text.split("\n").map((line: string) => line.startsWith(`${id},`) ? line.split(",").map((field, i) => i === 7 ? String(value) : field).join(",") : line).join("\n");
    const bits = new DataView(new ArrayBuffer(8)); bits.setFloat64(0, value);
    const annotation = pkg.source_annotations.find((a: Json) => a.source_result_id === id);
    annotation.source_row.value = value; annotation.source_row_sha256 = await canonicalSha256HexCheckedV1(row);
    annotation.source_value_bits = bits.getBigUint64(0).toString(16).padStart(16, "0");
    for (const w of pkg.unit_preservation_witnesses) if (w.result_id === id) { w.source_quantity.value = value; w.target_quantity.value = value; }
    const carrier = await canonicalSha256HexCheckedV1(mutated);
    for (const h of pkg.received_source_checksums) if (h.payload_scope === "received_result") h.value = carrier;
    pkg.source_carrier_checksum.value = carrier;
    pkg.source_block_recovery = structuredClone(mutated.source_block_recovery);
    await resealPacket(pkg);
    // The 0.3 record builder reads the full reader too (DEL-14-02), so it refuses this source.
    const manifest = { manifest_ref: { object_type: "InputManifest", ref: "manifest:invented-coherent-reseal" }, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: mutated.model_ref, model_payload: model }, solver_basis: { solver_name: mutated.producer.component_name, solver_version: mutated.producer.component_version, solver_build_ref: "test:coherent-reseal", solver_mode: mode[0] } } };
    await expect(buildAnalysisRunV03(mutated, manifest as Json, undefined, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_BLOCKS_PROJECTION_VALUE_BINDING");
    // Its validator, reopening the committed record against this source, refuses it the same way.
    await expect(validateAnalysisRunV03(record, mutated, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_BLOCKS_PROJECTION_VALUE_BINDING");
    await expect(validateStressNeutralExportPacket(structuredClone(pkg), mutated)).rejects.toThrow("SOURCE_BLOCKS_PROJECTION_VALUE_BINDING");
    // With the committed record, which the record validator now reads through the full reader.
    await expect(validateStressNeutralExportPacket(structuredClone(pkg), mutated, record, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_BLOCKS_PROJECTION_VALUE_BINDING");
    // The unmutated package still validates, with its source alone and with its record.
    await expect(validateStressNeutralExportPacket(structuredClone(packet), raw)).resolves.toBeUndefined();
    await expect(validateStressNeutralExportPacket(structuredClone(packet), raw, record, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  }, 60_000);
});
