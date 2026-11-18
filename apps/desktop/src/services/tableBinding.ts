/** A stored run's binding to the semantic table that gives its rows their meaning (DEL-05-02,
 * DEL-14-02): the refusal words of a record whose binding is not accepted, and the one decision a
 * reopen makes before it reads any row. Python's twin of the list and the words is
 * `core/analysis_runs/compatibility.py`; the native twin is `src-tauri/src/analysis_run_v03_validate.rs`. */

/** Former digests of a table, `{id, sha256}`: the SHA-256 of that table file's bytes at a main-line
 * revision, or the digest a committed recorded run of main binds (`fixtures/persistence/recorded_model_*`).
 * Read only to choose the words of `ANALYSIS_SEMANTIC_CONTRACT_MISMATCH`; never to admit a record.
 * One row per digest: append a line. The twins, row for row and in order: `compatibility.py` and
 * `analysis_run_v03_validate.rs` `KNOWN_FORMER_TABLE_DIGESTS` (`stored_custody_tests.rs` requires the three equal). */
const KNOWN_FORMER_TABLE_DIGESTS: ReadonlyArray<Readonly<{ id: string; sha256: string }>> = Object.freeze([
  { id: "openpipestress.result_semantics/0.3.0/pressure-source-1", sha256: "609a4e92ac19de0f5a1dd067dab64b4abeb7b3b70511e2095455f1d000d28e2c" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "e7b93f416f700f8efdb31bf1ba26dedde52963d768d6ead5a73dbf1c645a360c" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "f41b9d3ed0c1b7abaee0b98ca1a28aa1a2d109bce443b98d53a2080c1c504ce9" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "563db3c7c44279face3adc60a76a512de38520357e89ea4b49ad11d20946db89" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", sha256: "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", sha256: "ce49ebe07bc05431ea1f770c1933b5bf58f4b1c8b66fb6f36810f2ef7a58cd43" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", sha256: "6f6494907e3ead979ea27e5ff85f54617589c9195dc84d40a02752a0b006ef44" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", sha256: "f64473f5a60ac2bb34d8006b40bf044800f0c97454c570bbe88d1f1d96b7bdfc" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-retained-1", sha256: "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-retained-1", sha256: "26dce9c21261b1ca894af05a3b6f4a94db88904894cb47f7296f892e72ecfb7e" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-retained-1", sha256: "b4377e73e8a83edb57b4f0c41dbaef384385e0f6b4810103a106a334c8edbe14" },
  { id: "openpipestress.result_semantics/0.3.0/preview-physics-retained-1", sha256: "d0bab3502b4018ce0596757a933bd6306f402dc3741a10af24a094e8c66544e8" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "33cc0c4dc1d7dea07e14c16e07f4ee3f93cc2f84e4dd0494e232397d4e3861c1" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "48a969580673b00d25970b472fc28d974f7edc2cc602eb5deb6627a1164e1f41" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "1ba0a08bfdaebaa53514602d6e1b2d4da263580cff8fd0b2c3de85dc01910bf1" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "6218734669b76d98f64e22804ca862ba43d543b7c0d22894b196724998c39be8" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "320dc3f1f75944af8b66da1287e6f8ee5e1fcd5fe005752f2c90bf8c89a9c894" },
  { id: "openpipestress.result_semantics/0.3.0/physics-retained-1", sha256: "f17bd9d57ee8d1cef4b465715e4edbf030faa5aa4f5282ad7927c25e3f6668d3" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "be61c6ea828797ad5664d06a2c835b28deffaff07cc33fec6bb4d48a8fcb0919" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "5451fbf3e10e085c90b523baa7daf5fc723659e0d874d988d23807be7d2fefe6" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "d46e1963d7502b8e41898805128f2807b2670a1d4c738c71876002ed727e5faf" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "032e861fc66bf5f34a081b104ec879a39831bfd6472b186954966ea57d65f025" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "a2db16a515d8e4e1c1bcd6607861df0e7e58cbf1cf82a0b984e2f1c0f128382e" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "be29fabcfacb1070a51bfb56dcfe61b4e4125638e249c3c0e96d092cc8aba43f" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "10455bf348b46efac998b6635b537c9e3149a7d1c7a29ea96c0e3783a2de9ad6" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "2c20583d3c06e226a13bb7f9649b04aeea0f0f6948b5fa1cbd944afd07413645" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "21b7426ec1c50ba5f74539984261554d06b9b6a26ec26f7d5765b5faa7d6bb90" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad" },
  { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "3e2aac3cbe1dd967292dc2524cc7e12a35b162f14795c13b741acaa2b964625d" },
]);

export const ANALYSIS_SEMANTIC_CONTRACT_MISMATCH = "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH";
/** A saved result with no record of the revision of its table. */
export const ANALYSIS_SEMANTIC_CONTRACT_UNRECORDED = "ANALYSIS_SEMANTIC_CONTRACT_UNRECORDED";
const FORMER_TABLE_TEXT = "this run was recorded under an earlier revision of these result rules, which this version cannot interpret; the stored run is unchanged; run the analysis again on its saved model state to create a new result";
const UNRECOGNISED_TABLE_TEXT = "this run names a revision of these result rules that this version does not recognise; the stored run is unchanged; run the analysis again on its saved model state to create a new result";
const UNRECORDED_TABLE_TEXT = "this saved result names its result rules but does not record their revision, so its rows cannot be interpreted reliably; the saved result is preserved; run the analysis again to view current results";
const UNREADABLE_RESULT_TEXT = "the stored result cannot be read as a result, so it has no rows to show; the stored run is unchanged; run the analysis again on its saved model state to create a new result";
const INTEGRITY_TEXT = "the stored run does not agree with the hashes recorded with it, or they cannot be checked, so its rows cannot be interpreted reliably; the stored run is unchanged; run the analysis again on its saved model state to create a new result";

/** Whether `recorded` names, by exact strings, a known former digest of its table. */
function isKnownFormerDigest(recorded: unknown): boolean {
  if (!recorded || typeof recorded !== "object" || Array.isArray(recorded)) return false;
  const { id, sha256 } = recorded as Record<string, unknown>;
  return KNOWN_FORMER_TABLE_DIGESTS.some((row) => row.id === id && row.sha256 === sha256);
}

/** The refusal of a record whose table binding is not accepted, worded by whether the binding it
 * names is a known former digest of that table. */
export function semanticContractMismatch(recorded: unknown): string {
  return `${ANALYSIS_SEMANTIC_CONTRACT_MISMATCH}: ${isKnownFormerDigest(recorded) ? FORMER_TABLE_TEXT : UNRECOGNISED_TABLE_TEXT}`;
}

/** Why a reopened run's rows are withheld: the record names a former digest; it names a digest (or
 * binding) this version does not read; nothing records a binding; the stored run fails the hashes
 * recorded with it; or there is no stored result that reads as one (nothing to bind). */
export type WithheldReason = "former_digest" | "unrecognised_digest" | "unrecorded" | "integrity" | "unreadable_result";
/** The one decision of a reopen: its rows are read under the installed table or a registry variant
 * of the statement's own table, or withheld for one reason. The native reopen states the same JSON. */
export type TableBinding = Readonly<{ read: "installed" | "recorded" }> | Readonly<{ withheld: WithheldReason }>;

export function bindingRead(binding: TableBinding): boolean {
  return "read" in binding;
}

/** The hash findings that withhold a reopened run's rows (DEL-14-02 requirement 015: the record,
 * the received result, the input manifest and the state binding against their recorded hashes).
 * The project's own model and envelope hashes are not the run's, and a saved run has no stored
 * input manifest, so neither withholds anything. */
export const INTEGRITY_FINDINGS: ReadonlySet<string> = new Set([
  "HISTORICAL_ANALYSIS_HASH_MISMATCH", "HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE",
  "HISTORICAL_RESULT_HASH_MISMATCH", "HISTORICAL_RESULT_HASH_MISSING",
  "HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE",
  "RECORDED_RUN_INPUT_MANIFEST_HASH_MISMATCH", "RECORDED_RUN_MODEL_STATE_HASH_MISMATCH", "RECORDED_RUN_RECORD_HASH_MISMATCH",
]);

/** What the record claims, read before any row: no binding (`unrecorded`), a registry variant of
 * the statement's own table, the installed table's binding, or another binding. */
export type BindingClaim = Readonly<{ claim: "unrecorded" }> | Readonly<{ claim: "recorded" | "installed" }> | Readonly<{ claim: "other"; recorded: unknown }>;

/** The claim of a stored record over its statement. `statementId` is the statement header's
 * `producer.semantic_contract_id`; `installed` is the installed table's binding the header
 * dispatch names for it (null when the header names no table this version reads); `variant` is the
 * registry variant the record's binding names exactly (null for none). */
export function bindingClaim(record: unknown, statementId: unknown, installed: Readonly<{ id: string; sha256: string }> | null, variant: Readonly<{ id: string; sha256: string }> | null): BindingClaim {
  const envelope = record && typeof record === "object" && !Array.isArray(record) ? record as Record<string, unknown> : null;
  const run = envelope?.schema_version === "0.3.0" && envelope.analysis_run && typeof envelope.analysis_run === "object" && !Array.isArray(envelope.analysis_run)
    ? envelope.analysis_run as Record<string, unknown> : null;
  const reproducibility = run?.reproducibility && typeof run.reproducibility === "object" && !Array.isArray(run.reproducibility) ? run.reproducibility as Record<string, unknown> : null;
  const recorded = reproducibility?.semantic_contract;
  const named = recorded && typeof recorded === "object" && !Array.isArray(recorded) ? recorded as Record<string, unknown> : null;
  if (!named || Object.keys(named).length !== 2 || typeof named.id !== "string" || typeof named.sha256 !== "string") return { claim: "unrecorded" };
  if (variant && variant.id === statementId) return { claim: "recorded" };
  if (installed && named.id === statementId && named.id === installed.id && named.sha256 === installed.sha256) return { claim: "installed" };
  return { claim: "other", recorded };
}

/** The decision: a claim to a table this version reads holds only when no integrity finding was found. */
export function decideBinding(claim: BindingClaim, checkFindings: readonly string[]): TableBinding {
  if (claim.claim === "unrecorded") return { withheld: "unrecorded" };
  if (checkFindings.some((finding) => INTEGRITY_FINDINGS.has(finding))) return { withheld: "integrity" };
  if (claim.claim === "other") return { withheld: isKnownFormerDigest(claim.recorded) ? "former_digest" : "unrecognised_digest" };
  return { read: claim.claim };
}

/** The code a withheld run is refused under (none for integrity or an unreadable result: their
 * findings are the refusal). */
export function withheldCode(reason: WithheldReason): string | null {
  switch (reason) {
    case "former_digest": case "unrecognised_digest": return ANALYSIS_SEMANTIC_CONTRACT_MISMATCH;
    case "unrecorded": return ANALYSIS_SEMANTIC_CONTRACT_UNRECORDED;
    case "integrity": case "unreadable_result": return null;
  }
}

/** What a person is told of a withheld run. */
export function withheldMessage(reason: WithheldReason): string {
  switch (reason) {
    case "former_digest": return FORMER_TABLE_TEXT;
    case "unrecognised_digest": return UNRECOGNISED_TABLE_TEXT;
    case "unrecorded": return UNRECORDED_TABLE_TEXT;
    case "integrity": return INTEGRITY_TEXT;
    case "unreadable_result": return UNREADABLE_RESULT_TEXT;
  }
}
