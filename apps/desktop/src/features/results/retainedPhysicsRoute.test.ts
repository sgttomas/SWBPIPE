import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import exactTable from "../../../../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json";
import pressureTable from "../../../../../fixtures/results/semantic_contract_v0_3_pressure_2.json";
import { PHYSICS_RETAINED_CONTRACT_ID } from "./numericalResultQuality";
import { PHYSICS_RETAINED_CONTRACT_SHA256, PRESSURE_CONTRACT_SHA256 } from "../../test-support/contractBindings";
import { readerDispatch, readerRowSemantics, retainedPrecisionDowngrade, sourceContract, sourceSemanticBinding } from "./resultReader";
import { validateRetainedPrecisionTransport } from "./retainedPrecisionReader";
import { numericalResultStanding } from "./resultStanding";
import { RETAINED_PRECISION_VALIDATION_REQUIRED } from "./retainedPrecisionStanding";
import { knownSemanticNotices, resultRowLabel, ruleBindingRefusal, RULE_QUANTITY_NOT_COVERED } from "./knownSemanticLimitations";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const RUST_READER = readFileSync(resolve(root, "core/reporting/result_export/src/retained_precision.rs"), "utf8");
const rustConstant = (name: string) => RUST_READER.match(new RegExp(`pub const ${name}: &str =\\s*"([^"]*)";`))![1];

/** B3b's exact route `retained_physics` on the desktop's carriers (moved from the deleted TypeScript reader's
 * tests, R1), on lane P's m3x exact successor. */
describe("the exact retained route on the desktop's carriers", () => {
  it("header dispatch, binding, downgrade guard, transport through the Rust reader, standing, table and notices", async () => {
    const { source, invocation } = JSON.parse(readFileSync(resolve(root, "fixtures/results/retained_precision_exact_successor_sparse_interactive.json"), "utf8")) as Json;
    // The route's identity equals the Rust reader's constant, and its profile is the one the source states.
    expect(PHYSICS_RETAINED_CONTRACT_ID).toBe(rustConstant("EXACT_CONTRACT_ID"));
    expect(source.formulation_basis.profile_id).toBe(rustConstant("EXACT_PROFILE"));
    expect(sourceContract(source)).toBe("retained_physics");
    expect(sourceSemanticBinding(source)).toEqual({ id: PHYSICS_RETAINED_CONTRACT_ID, sha256: PHYSICS_RETAINED_CONTRACT_SHA256 });
    expect(retainedPrecisionDowngrade(source)).toBe(false);
    // The header dispatch runs the reader's transport checks (G5): a tampered receipt is refused with the reader's code.
    expect(readerDispatch(source)).toMatchObject({ ok: true, route: "retained_physics" });
    const tampered = structuredClone(source); tampered.retained_precision.body.work_policy = "W1-LME-20B-60B-v2";
    expect(readerDispatch(tampered)).toMatchObject({ ok: false, route: "unsupported", error: { code: "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED" } });
    await expect(validateRetainedPrecisionTransport(tampered)).rejects.toMatchObject({ gate: "G0", code: "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED" });
    // The downgrade guard: the base identity, pressure-2 since B7b D4 (a), carrying a receipt is unsupported.
    const downgraded = structuredClone(source); downgraded.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/pressure-2"; downgraded.formulation_basis.profile_id = "exact_pressure_v3";
    expect(sourceContract(downgraded)).toBe("unsupported");
    expect(retainedPrecisionDowngrade(downgraded)).toBe(true);
    // The exact identity without its receipt is not the successor's route.
    const bare = structuredClone(source); delete bare.retained_precision;
    expect(sourceContract(bare)).toBe("unsupported");
    // Standing comes only from a registered validation: unregistered bytes need recompute.
    expect(numericalResultStanding(source, invocation.request.model)).toMatchObject({ contract: "retained_physics", eligible: false, findings: [RETAINED_PRECISION_VALIDATION_REQUIRED] });
    // XTABLE's rows are pressure-2's (B7b D4 (a) rebased the successor onto pressure-1);
    // the row semantics read XTABLE, which inherits the pressure-2 table by its sha256.
    const rows = readerRowSemantics(source);
    for (const row of source.results) { const signature = rows(row).signature; if (signature) expect(exactTable.rows).toContainEqual(signature); }
    expect(source.results.some((row: Json) => rows(row).signature !== null)).toBe(true);
    expect(exactTable.rows).toEqual(pressureTable.rows);
    expect(exactTable.inherited_semantic_contract_sha256).toBe(PRESSURE_CONTRACT_SHA256);
    // Notices: no preview notice (pressure-2's rows); the unvalidated successor notice; no kind label.
    expect(knownSemanticNotices(source).map(n => n.id)).toEqual(["retained-precision-unvalidated"]);
    expect(resultRowLabel(source.results.find((r: Json) => r.kind === "pipe_elastic_normal_stress_maximum_v2"), source)).toBeNull();
    // Rule binding fails closed without a registered validation.
    expect(ruleBindingRefusal(source, source.results[0])).toBe(RULE_QUANTITY_NOT_COVERED);
  });
});
