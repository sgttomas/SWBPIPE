/** RT-A4 §10.4 (DEL-05-02 Design `absolute-sum-srss-combinations.md`, "Recorded results from
 * before RT-A4 (historical read)"): the closed registry of the semantic tables a run recorded
 * before RT-A4 binds. RT-A4 restated the preview-physics combination policy and limitation 6, so
 * the preview-physics tables' digests moved, and a preview-physics statement carries its
 * table's limitation array, which the readers require to equal the table's.
 *
 * Each variant is the table's bytes as the main line held them
 * (`fixtures/results/recorded_pre_rt_a4/`, verbatim), read as raw text and checked against its
 * SHA-256 before it is used. A variant is selected only by a record's exact binding
 * `{id, sha256}`, and only on the recorded-run read and compare paths
 * (`validateAnalysisRunV03(..., { recordedRead: true })`, `buildRecordedRunContext`,
 * `buildHistoricalRunContext`, `recordedRunSide`); the record builder, the exports and every
 * current standing read the current tables only. Rust's twin is result_export
 * `recorded_tables::VARIANTS`; Python's, `compatibility.py` `RECORDED_PRE_RT_A4_TABLES`. */
import previewPhysics1ccc from "../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_preview_physics_2.1ccc9561.json?raw";
import previewTimoshenkoA7b8 from "../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_preview_physics_timoshenko_1.a7b83c37.json?raw";
import previewRetainedB001 from "../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_preview_physics_retained_1.b001070e.json?raw";
import physicsRetainedCf29 from "../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_physics_retained_1.cf29fd5f.json?raw";
import physicsRetained045f from "../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_physics_retained_1.045f6f0d.json?raw";
import physicsRetained040e from "../../../../fixtures/results/recorded_pre_rt_a4/semantic_contract_v0_3_physics_retained_1.040ef9a6.json?raw";
// Operating-case slice 1b: main's tables at bbf384dec, before 1b restated the preview-physics combination policy.
import previewPhysicsE7b9 from "../../../../fixtures/results/recorded_pre_rt_a5_1b/semantic_contract_v0_3_preview_physics_2.e7b93f41.json?raw";
import previewTimoshenkoCe49 from "../../../../fixtures/results/recorded_pre_rt_a5_1b/semantic_contract_v0_3_preview_physics_timoshenko_1.ce49ebe0.json?raw";
import previewRetained26dc from "../../../../fixtures/results/recorded_pre_rt_a5_1b/semantic_contract_v0_3_preview_physics_retained_1.26dce9c2.json?raw";
// RT-A5 minuend_final: main's tables at b32a17761, before the combination policy's v4 restatement.
import previewPhysicsF41b from "../../../../fixtures/results/recorded_pre_rt_a5_mf/semantic_contract_v0_3_preview_physics_2.f41b9d3e.json?raw";
import previewTimoshenko6f64 from "../../../../fixtures/results/recorded_pre_rt_a5_mf/semantic_contract_v0_3_preview_physics_timoshenko_1.6f649490.json?raw";
import previewRetainedB437 from "../../../../fixtures/results/recorded_pre_rt_a5_mf/semantic_contract_v0_3_preview_physics_retained_1.b4377e73.json?raw";
// R-6/R-7 S-K1: the S-K1 base's tables at 6ebd92ce9, before the along-direction kind
// (support_reaction_along_direction_v1) joined them.
import previewPhysics563d from "../../../../fixtures/results/recorded_pre_sk1/semantic_contract_v0_3_preview_physics_2.563db3c7.json?raw";
import previewTimoshenkoF644 from "../../../../fixtures/results/recorded_pre_sk1/semantic_contract_v0_3_preview_physics_timoshenko_1.f64473f5.json?raw";
import previewRetainedD0ba from "../../../../fixtures/results/recorded_pre_sk1/semantic_contract_v0_3_preview_physics_retained_1.d0bab350.json?raw";

export type RecordedBinding = Readonly<{ id: string; sha256: string }>;
type Variant = RecordedBinding & { text: string };

const PREVIEW = "openpipestress.result_semantics/0.3.0/preview-physics-2";
const TIMOSHENKO = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
const PREVIEW_RETAINED = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1";
const PHYSICS_RETAINED = "openpipestress.result_semantics/0.3.0/physics-retained-1";

/** The closed registry: main at 6c3cb46d3's four, RT-A3's final constant-effort
 * candidate's exact successor table (#101 at 01e05a486), and the RT-A4 main line's exact
 * successor table (main at bbf384dec), which RT-A3 increment 4's pressure-2 restatement
 * makes history, then main's three preview-physics tables at bbf384dec, before
 * operating-case slice 1b, main's three at b32a17761, before RT-A5 `minuend_final`, and the
 * S-K1 base's three at 6ebd92ce9, before the along-direction kind. */
export const RECORDED_PRE_RT_A4_TABLES: ReadonlyArray<Readonly<Variant>> = Object.freeze([
  { id: PREVIEW, sha256: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2", text: previewPhysics1ccc },
  { id: TIMOSHENKO, sha256: "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b", text: previewTimoshenkoA7b8 },
  { id: PREVIEW_RETAINED, sha256: "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46", text: previewRetainedB001 },
  { id: PHYSICS_RETAINED, sha256: "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd", text: physicsRetainedCf29 },
  { id: PHYSICS_RETAINED, sha256: "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b", text: physicsRetained045f },
  { id: PHYSICS_RETAINED, sha256: "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034", text: physicsRetained040e },
  { id: PREVIEW, sha256: "e7b93f416f700f8efdb31bf1ba26dedde52963d768d6ead5a73dbf1c645a360c", text: previewPhysicsE7b9 },
  { id: TIMOSHENKO, sha256: "ce49ebe07bc05431ea1f770c1933b5bf58f4b1c8b66fb6f36810f2ef7a58cd43", text: previewTimoshenkoCe49 },
  { id: PREVIEW_RETAINED, sha256: "26dce9c21261b1ca894af05a3b6f4a94db88904894cb47f7296f892e72ecfb7e", text: previewRetained26dc },
  { id: PREVIEW, sha256: "f41b9d3ed0c1b7abaee0b98ca1a28aa1a2d109bce443b98d53a2080c1c504ce9", text: previewPhysicsF41b },
  { id: TIMOSHENKO, sha256: "6f6494907e3ead979ea27e5ff85f54617589c9195dc84d40a02752a0b006ef44", text: previewTimoshenko6f64 },
  { id: PREVIEW_RETAINED, sha256: "b4377e73e8a83edb57b4f0c41dbaef384385e0f6b4810103a106a334c8edbe14", text: previewRetainedB437 },
  { id: PREVIEW, sha256: "563db3c7c44279face3adc60a76a512de38520357e89ea4b49ad11d20946db89", text: previewPhysics563d },
  { id: TIMOSHENKO, sha256: "f64473f5a60ac2bb34d8006b40bf044800f0c97454c570bbe88d1f1d96b7bdfc", text: previewTimoshenkoF644 },
  { id: PREVIEW_RETAINED, sha256: "d0bab3502b4018ce0596757a933bd6306f402dc3741a10af24a094e8c66544e8", text: previewRetainedD0ba },
].map(variant => Object.freeze(variant)));

/** The variant a record's binding names: an object of exactly `id` and `sha256` equal to a
 * registry member's; null for anything else (a current binding included). */
export function recordedPreRtA4Binding(binding: unknown): RecordedBinding | null {
  if (!binding || typeof binding !== "object" || Array.isArray(binding)) return null;
  const named = binding as Record<string, unknown>;
  if (Object.keys(named).length !== 2) return null;
  const variant = RECORDED_PRE_RT_A4_TABLES.find(entry => entry.id === named.id && entry.sha256 === named.sha256);
  return variant ? { id: variant.id, sha256: variant.sha256 } : null;
}

/** The variant a record (an AnalysisRun envelope) binds, read from its
 * `reproducibility.semantic_contract`; null when it names none. */
export function recordedPreRtA4RecordBinding(record: unknown): RecordedBinding | null {
  const run = record && typeof record === "object" ? (record as { analysis_run?: { reproducibility?: { semantic_contract?: unknown } } }).analysis_run : undefined;
  return recordedPreRtA4Binding(run?.reproducibility?.semantic_contract);
}

/** The label of what was read under a recorded table: the table it was read under. */
export function recordedTableLabel(table: RecordedBinding): string {
  return `As recorded under ${table.id} (sha256 ${table.sha256}); historical evidence only, not a current standing`;
}

export const SOURCE_RECORDED_TABLE_HASH = "SOURCE_RECORDED_TABLE_HASH";
const checked = new Map<string, Promise<unknown>>();
async function sha256HexOfText(text: string): Promise<string> {
  const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return Array.from(new Uint8Array(digest), byte => byte.toString(16).padStart(2, "0")).join("");
}
/** The variant's table, its bytes checked against its SHA-256 (SOURCE_RECORDED_TABLE_HASH
 * when they differ). */
export function recordedPreRtA4Table(binding: RecordedBinding): Promise<unknown> {
  const variant = RECORDED_PRE_RT_A4_TABLES.find(entry => entry.id === binding.id && entry.sha256 === binding.sha256);
  if (!variant) return Promise.reject(new Error("SOURCE_RECORDED_BINDING_UNKNOWN"));
  let table = checked.get(variant.sha256);
  if (!table) {
    table = sha256HexOfText(variant.text).then(hex => {
      if (hex !== variant.sha256) throw new Error(SOURCE_RECORDED_TABLE_HASH);
      return Object.freeze(JSON.parse(variant.text));
    });
    table.catch(() => checked.delete(variant.sha256));
    checked.set(variant.sha256, table);
  }
  return table;
}
