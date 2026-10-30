/** E3 current construction bytes and real route refusals.
 * IPC replays committed producer bytes through the real native-capture path;
 * Rust readers, source binding, row semantics and hashes are never mocked.
 * Optional migration capture/compare paths are evidence only. Routine tests
 * depend on maintained inputs, not on a dated capture directory. */
import { afterEach, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { runPreviewMechanics, buildAnalysisRunPreview } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { canonicalJsonCheckedV1, checkedJsonText } from "../../services/hashService";
import { numericalResultStanding } from "../results/resultStanding";
import { modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { buildStressNeutralExportPacket, liveStressBinding, validateStressNeutralExportPacket } from "./StressNeutralExportPanel";
import type { MechanicsResult, PreviewModel, AnalysisRunEnvelope } from "../../types";

const project = resolve(__dirname, "../../../../../");
const json = (name: string) => JSON.parse(readFileSync(resolve(project, name), "utf8"));
const modes = ["sparse_interactive", "dense_scrutiny"] as const;
afterEach(() => { vi.mocked(invoke).mockReset(); delete (window as any).__TAURI_INTERNALS__; });

async function admitted(profile: "preview", mode: typeof modes[number]) {
  const { model } = nativeMechanicsReplayPair(mode, { profile });
  (window as any).__TAURI_INTERNALS__ = {};
  vi.mocked(invoke).mockImplementation(createNativeMechanicsReplay({ profile }).invoke);
  const result = await runPreviewMechanics(model, mode);
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: {
    solver_name: result.producer!.component_name, solver_version: result.producer!.component_version,
    solver_build_ref: "unit-transport-replay", solver_mode: mode, settings: {},
  }, active_rule_packs: [], external_assets: [] });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, analysisRun };
}

/** Exact hash-input bytes for the eight payloads and the manifest seed, plus
 * actual desktop JSON download, CSV, claims and provenance representations.
 * The desktop does not expose Python's nine-file transport writer. */
async function representations(packet: any): Promise<Record<string, Uint8Array>> {
  const nonManifest = packet.manifest.checksums.filter((c: any) => c.payload_ref.ref !== "manifest.json");
  const seed = { manifest_id: packet.manifest.manifest_id, source_result_ref: packet.source_result_ref,
    source_run_ref: packet.source_run_ref, source_model_ref: packet.source_model_ref,
    received_source_checksums: packet.received_source_checksums, unresolved_assumption_refs: packet.unresolved_assumption_refs,
    reproducibility_refs: packet.reproducibility_refs, export_profile_ref: packet.manifest.export_profile_ref,
    boundary_notes: packet.manifest.boundary_notes, member_checksums: nonManifest, diagnostics: packet.diagnostics };
  const values: Record<string, unknown> = { "manifest-seed.jcs.json": seed,
    "result_rows.json": packet.result_rows, "unit_system_disclosure.json": packet.unit_system_disclosure,
    "unit_preservation_witnesses.json": packet.unit_preservation_witnesses, "stable_id_map.json": packet.stable_id_map,
    "loss_report.json": packet.loss_report, "validation_report.json": packet.validation_report, "diagnostics.json": packet.diagnostics,
    "provenance.jcs.json": packet.provenance };
  const encoded: Record<string, Uint8Array> = {
    "download.json": new TextEncoder().encode(JSON.stringify(packet, null, 2)),
    "stress_neutral_results.csv": new TextEncoder().encode(packet.csv_text),
    "checksums.download.json": new TextEncoder().encode(JSON.stringify(packet.manifest.checksums, null, 2)),
    "provenance.download.json": new TextEncoder().encode(JSON.stringify(packet.provenance, null, 2)),
  };
  for (const [name, value] of Object.entries(values)) encoded[name] = new TextEncoder().encode(await canonicalJsonCheckedV1(value));
  for (const claim of packet.manifest.checksums) {
    const name = claim.payload_ref.ref === "manifest.json" ? "manifest-seed.jcs.json" : claim.payload_ref.ref;
    expect(createHash("sha256").update(encoded[name]).digest("hex")).toBe(claim.value);
  }
  return encoded;
}

function migrationEvidence(caseName: string, bytes: Record<string, Uint8Array>) {
  if (process.env.E3_DESKTOP_CAPTURE_DIR) {
    const directory = resolve(process.env.E3_DESKTOP_CAPTURE_DIR, caseName); mkdirSync(directory, { recursive: true });
    for (const [name, value] of Object.entries(bytes)) writeFileSync(resolve(directory, name), value);
  }
  if (process.env.E3_DESKTOP_COMPARE_DIR) {
    for (const [name, value] of Object.entries(bytes)) expect(Buffer.from(value), `${caseName}/${name}`).toEqual(readFileSync(resolve(process.env.E3_DESKTOP_COMPARE_DIR, caseName, name)));
  }
}

// B7b: the exact route's v2 package (physics-1) is retired with its identity. Epoch 8
// packages its v3 twin, pressure-2, at Rust's standing (below, and
// `../result-export/exactOutputTwins.test.tsx`); preview-physics-2 keeps this byte construction.
it.each(["preview"] as const)("%s: already admitted current public construction keeps complete bytes in both modes", async profile => {
  for (const mode of modes) {
    const args = await admitted(profile, mode);
    expect(numericalResultStanding(args.result, args.model)).toMatchObject({ eligible: true, findings: [] });
    expect(liveStressBinding(args.model, args.result, args.analysisRun)).not.toBeNull();
    const before = checkedJsonText(args);
    const packet = await buildStressNeutralExportPacket(args);
    expect(packet.schema_version).toBe("0.3.0");
    await validateStressNeutralExportPacket(packet, args.result, args.analysisRun, modelLoadBasisRefs(args.model));
    expect(checkedJsonText(args)).toBe(before);
    migrationEvidence(`${profile}-${mode}`, { ...await representations(packet),
      "inputs.json": new TextEncoder().encode(JSON.stringify(args, null, 2)) });
  }
}, 60_000);

/** The load/reference-state results are the v3 twins, published under pressure-2
 * (ordinary) and pressure-source-1 (joined), each with the Python product path's
 * committed analysis-run record. Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md`
 * §5.4, ruling 11B): they follow the reader. The pressure-2 ones read numerically
 * eligible and package; the pressure-source-1 ones are never eligible and the builder
 * refuses them by its standing code. These bytes are read from the fixtures, not
 * natively captured, so the pressure-source-1 standing is the unregistered one; the
 * registered standing with Rust's T1 finding is checked in `../result-export/exactOutputTwins.test.tsx`. */
const loadReferenceCases = [
  ...["connected", "pressure"].map(name => ({ family: "pressure/load_reference", record: "pressure_load_state", name, route: "pressure" as const })),
  ...["eigen_motion", "fields", "mixed", "n05", "n06"].map(name => ({ family: "pressure_source/load_reference_source", record: "pressure_source_load_state", name, route: "pressure_source" as const })),
].flatMap(entry => modes.map(mode => ({ ...entry, mode })));

it.each(loadReferenceCases)("$family/$name/$mode: real public output follows Rust's standing", async ({ family, record, name, mode, route }) => {
  const model = json(`fixtures/product_preview/${family}/${name}.request.json`).model as PreviewModel;
  const result = json(`fixtures/product_preview/${family}/${name}-${mode}.raw.json`) as MechanicsResult;
  const suffix = mode === "sparse_interactive" ? "sparse" : "dense";
  const analysisRun = json(`fixtures/results/${record}_${name}_${suffix}.analysis_run.json`) as AnalysisRunEnvelope;
  expect(analysisRun.analysis_run.run_id).toBe(result.run_id);
  const before = checkedJsonText({ model, result, analysisRun });
  if (route === "pressure") {
    expect(numericalResultStanding(result, model)).toMatchObject({ contract: "pressure", eligible: true, findings: [] });
    const packet = await buildStressNeutralExportPacket({ model, result, analysisRun });
    expect(packet.contract_evidence).toEqual(result.contract_evidence);
    expect(Object.hasOwn(packet, "source_block_recovery")).toBe(false);
    await validateStressNeutralExportPacket(packet, result, analysisRun, modelLoadBasisRefs(model));
  } else {
    expect(numericalResultStanding(result, model)).toMatchObject({ contract: "pressure_source", eligible: false, findings: ["SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED"] });
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
  }
  expect(checkedJsonText({ model, result, analysisRun })).toBe(before);
}, 60_000);
