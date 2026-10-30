/** The pinned semantic tables' bindings `{id, sha256}` as the Rust header dispatch
 * states them (`reader_verdict::BINDINGS`: each sha256 is the table file's), computed
 * here from the table files, for tests' expected values. B7b: the current tables only
 * (the retired v2 tables of physics-1, physics-source-1, load-reference-1 and
 * load-reference-source-1 are deleted). */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const TABLES = resolve(__dirname, "../../../../fixtures/results");
const sha256 = (file: string) => createHash("sha256").update(readFileSync(resolve(TABLES, file))).digest("hex");
export const PRESSURE_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_pressure_2.json");
export const PRESSURE_SOURCE_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_pressure_source_1.json");
export const PREVIEW_PHYSICS_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_preview_physics_2.json");
/** DEL-05-03 pf-timoshenko.md §4: the pressure-free Timoshenko identity's table. */
export const PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_preview_physics_timoshenko_1.json");
export const PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_preview_physics_retained_1.json");
export const PHYSICS_RETAINED_CONTRACT_SHA256 = sha256("semantic_contract_v0_3_physics_retained_1.json");
