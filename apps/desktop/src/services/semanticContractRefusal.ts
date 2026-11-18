/** The record validator's refusal of a record whose table binding is not accepted (DEL-05-02),
 * worded by whether the binding names a known former digest of that table. Python's twin is
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
export const FORMER_TABLE_TEXT = "this run was recorded under an earlier revision of these result rules, which this version cannot interpret; the stored run is unchanged; run the analysis again on its saved model state to create a new result";
export const UNRECOGNISED_TABLE_TEXT = "this run names a revision of these result rules that this version does not recognise; the stored run is unchanged; run the analysis again on its saved model state to create a new result";

/** Whether `recorded` names, by exact strings, a known former digest of its table. */
export function isKnownFormerDigest(recorded: unknown): boolean {
  if (!recorded || typeof recorded !== "object" || Array.isArray(recorded)) return false;
  const { id, sha256 } = recorded as Record<string, unknown>;
  return KNOWN_FORMER_TABLE_DIGESTS.some((row) => row.id === id && row.sha256 === sha256);
}

/** The refusal of a record whose table binding is not accepted, worded by whether the binding it
 * names is a known former digest of that table. */
export function semanticContractMismatch(recorded: unknown): string {
  return `${ANALYSIS_SEMANTIC_CONTRACT_MISMATCH}: ${isKnownFormerDigest(recorded) ? FORMER_TABLE_TEXT : UNRECOGNISED_TABLE_TEXT}`;
}
