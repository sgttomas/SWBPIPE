// DEL-02-05 CLM-003 (b7-plan "Current model format"; DEL-02-01 Design
// model-document-0.7.0.md §2): the one current model document version and the
// retired versions refused by name with a re-author diagnostic. Nothing is
// migrated. Mirrors core/product_physics/src/model_format.rs. Production code
// names the version only through these constants (modelVersionLiterals guard).
export const MODEL_SCHEMA_VERSION = "0.7.0";
export const RETIRED_MODEL_SCHEMA_VERSIONS: ReadonlyArray<string> = ["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0"];
