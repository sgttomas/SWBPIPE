// DEL-04-11 REQ-008 / AC-008: the hanger-selection builder with sized values. The model and library are the invented
// two-span sizing input (read, not changed); the sizing run's digests and sized loads are invented here. Each batch is
// applied by the wasm operation engine, the route the person's acceptance runs, to show the applied support.
import { describe, expect, it } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyOperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import { buildHangerSelectionBatch, type HangerDocument, type HangerSizingApplication } from "./hangerSelection";

const model = () => structuredClone(twoSpanInput.request.model) as unknown as PreviewModel;
const library = () => structuredClone(twoSpanInput.library) as unknown as HangerDocument;
const LIBRARY = "invented.hangers.sizing";

const run = (hangerId: string) => ({
  sizingRequestSha256: "1".repeat(64),
  solveRequestSha256: "2".repeat(64),
  libraryId: LIBRARY,
  librarySha256: "3".repeat(64),
  hangerId,
  recordSha256: "4".repeat(64)
});
const VARIABLE: HangerSizingApplication = {
  run: run("VS-200-L"),
  loads: { family: "variable_spring", coldLoadN: 5197.458880641297, hotLoadN: 5000, designOperatingCase: "load:O1" }
};
const CONSTANT: HangerSizingApplication = { run: run("CE-1"), loads: { family: "constant_effort", constantLoadN: 5012.25 } };
const RUN_NOTE = `Hanger sizing run with sizing request sha256 ${"1".repeat(64)} and solve request sha256 ${"2".repeat(64)}; library ${LIBRARY} sha256 ${"3".repeat(64)};`;
const TARGET = { supportId: "support:B", restraints: ["UZ"] };
const BEFORE = { family: "vertical_support", restraints: ["UZ"], provenance: "invented_test_value" };

async function applied(source: PreviewModel, batch: Awaited<ReturnType<typeof buildHangerSelectionBatch>>) {
  const result = await applyOperationBatch(source, batch, await computeModelHash(source));
  expect(result.validation.application_status, JSON.stringify(result.diagnostics)).toBe("applied_to_session_model");
  return result.applied_model!.supports.find((support) => support.id === "support:B")!;
}

describe("a sized selection through the hanger-selection builder", () => {
  it("writes a variable record's family, stiffness, travel range, references and provenance, and the sized cold, installed and hot loads", async () => {
    const source = model();
    const original = await canonicalJsonString(source);
    const batch = await buildHangerSelectionBatch(source, library(), "VS-200-L", TARGET, VARIABLE);
    const [configuration, record] = batch.operations;
    expect(configuration.change.field_path).toBe("configuration");
    expect(configuration.audit_boundary.requires_user_acceptance).toBe(true);
    expect(JSON.parse(configuration.change.before)).toEqual(BEFORE);
    const after = {
      family: "variable_spring_hanger",
      restraints: ["UZ"],
      provenance: `library:hanger:${LIBRARY}#VS-200-L`,
      hanger: {
        hanger_type: "variable_spring_hanger",
        source_reference: "invented-local:VS-200-L",
        stiffness: { dof: "UZ", value: { value: 200000, unit: "N/m" } },
        travel_range: { value: 50, unit: "mm" },
        installed_load: { value: 5197.458880641297, unit: "N" },
        cold_load: { value: 5197.458880641297, unit: "N" },
        hot_load: { value: 5000, unit: "N" }
      }
    };
    // The record's own installed load (1000 N) is replaced, and no other record load is carried.
    expect(JSON.parse(configuration.change.after)).toEqual(after);
    expect(configuration.change.source_note).toBe(
      `${RUN_NOTE} record VS-200-L sha256 ${"4".repeat(64)}. ` +
      "Sized values: hanger.cold_load 5197.458880641297 N and hanger.installed_load 5197.458880641297 N, both the sized cold load (the setting at the drawn position); " +
      "hanger.hot_load 5000 N, the sized operating load in the design operating case load:O1. " +
      `Family, stiffness, travel range and references from the library record library:hanger:${LIBRARY}#VS-200-L.`
    );
    expect(record.change.field_path).toBe("provenance_record");
    expect(record.change.source_note).toBe(`library:hanger:${LIBRARY}#VS-200-L`);
    // Building writes nothing; the operation route applies the batch, and the support it writes has one restraint.
    expect(await canonicalJsonString(source)).toBe(original);
    const support = await applied(source, batch);
    expect(support.restraints).toEqual(["UZ"]);
    expect(support.hanger).toEqual(after.hanger);
    expect(support.family).toBe("variable_spring_hanger");
  });

  it("writes a constant record with the sized constant load, the hot-load basis, and no record load of its own", async () => {
    const source = model();
    const batch = await buildHangerSelectionBatch(source, library(), "CE-1", TARGET, CONSTANT);
    const [configuration] = batch.operations;
    expect(JSON.parse(configuration.change.before)).toEqual(BEFORE);
    const after = {
      family: "constant_effort_support",
      restraints: ["UZ"],
      provenance: `library:hanger:${LIBRARY}#CE-1`,
      hanger: {
        hanger_type: "constant_effort_support",
        source_reference: "invented-local:CE-1",
        travel_range: { value: 100, unit: "mm" },
        constant_load: { value: 5012.25, unit: "N" }
      }
    };
    expect(JSON.parse(configuration.change.after)).toEqual(after);
    expect(configuration.change.source_note).toBe(
      `${RUN_NOTE} record CE-1 sha256 ${"4".repeat(64)}. Sized value: hanger.constant_load 5012.25 N, the hot-load basis. ` +
      `Family, stiffness, travel range and references from the library record library:hanger:${LIBRARY}#CE-1.`
    );
    const support = await applied(source, batch);
    expect(support.restraints).toEqual(["UZ"]);
    expect(support.hanger).toEqual(after.hanger);
  });

  it("refuses a restraint that disagrees with the record's stiffness direction, more than one restraint, and another record or family", async () => {
    const mismatch = { ...VARIABLE, run: run("VS-200-UY") };
    await expect(buildHangerSelectionBatch(model(), library(), "VS-200-UY", TARGET, mismatch)).rejects.toThrow("declares its stiffness on UY");
    await expect(buildHangerSelectionBatch(model(), library(), "VS-200-L", { ...TARGET, restraints: ["UY", "UZ"] }, VARIABLE)).rejects.toThrow("exactly one");
    await expect(buildHangerSelectionBatch(model(), library(), "VS-200-S", TARGET, VARIABLE)).rejects.toThrow("another library or record");
    await expect(buildHangerSelectionBatch(model(), library(), "CE-1", TARGET, { ...VARIABLE, run: run("CE-1") })).rejects.toThrow("is constant effort support");
    const negative = { ...CONSTANT, loads: { family: "constant_effort" as const, constantLoadN: -1 } };
    await expect(buildHangerSelectionBatch(model(), library(), "CE-1", TARGET, negative)).rejects.toThrow("positive finite");
  });

  it("copies a record's own values unchanged when no sized value is supplied", async () => {
    const batch = await buildHangerSelectionBatch(model(), library(), "VS-200-L", TARGET);
    const after = JSON.parse(batch.operations[0].change.after);
    expect(after.hanger.installed_load).toEqual({ value: 1000, unit: "N" });
    expect(after.hanger.cold_load).toBeUndefined();
    expect(batch.operations[0].change.source_note).toBe(`library:hanger:${LIBRARY}#VS-200-L`);
  });
});
