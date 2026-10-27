// DEL-02-05 CLM-003 (b7-plan "Current model format"; DEL-02-01 Design
// model-document-0.7.0.md §2, model-document-0.8.0.md §2): the one current model document version and the
// retired versions refused by name with a re-author diagnostic. Nothing is
// migrated. Mirrors core/product_physics/src/model_format.rs. Production code
// names the version only through these constants (modelVersionLiterals guard).
export const MODEL_SCHEMA_VERSION = "0.8.0";
export const RETIRED_MODEL_SCHEMA_VERSIONS: ReadonlyArray<string> = ["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0"];
// Owner ruling, 2026-10-26 ("R-c1 A, 0.8.0 a2"; DEL-02-01 Design
// model-document-0.8.0.md §3.3): what a refused retired project or saved model
// state means for its recorded runs. Appended to a retired_schema refusal, as the
// native store does (src-tauri `model_document_version.rs` RETIRED_PROJECT_RUNS_NOTE).
export const RETIRED_PROJECT_RUNS_NOTE =
  "Recorded runs keep their own format and existing read and compare behaviour; this change adds no model-document-version gate for recorded runs. Existing integrity checks and retained-precision/source-reader restrictions still apply. Run history is listed for the active project's id, so this project's runs are reached through a current project with the same project id, and run comparison keeps its eligibility rules.";
