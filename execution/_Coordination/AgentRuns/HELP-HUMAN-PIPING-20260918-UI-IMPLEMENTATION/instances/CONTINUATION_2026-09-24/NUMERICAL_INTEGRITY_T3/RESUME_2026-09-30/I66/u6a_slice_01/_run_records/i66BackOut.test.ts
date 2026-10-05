// I66 U6a control 2, TypeScript lane (scratch only, never committed): the receipt
// carried by the Rust derivative is byte-equal to the source's and revalidates
// in the accepted TypeScript reader, raw (with the invocation) and as transport.
import { expect, it } from "vitest";
import { readFileSync, appendFileSync } from "node:fs";
import { validateRetainedPrecision, validateRetainedPrecisionTransport } from "./retainedPrecision";

const P = "WT/scratch/i66_u6a_slice_01/cand/projects/chirality-piping";
const OUT = "WT/scratch/i66_u6a_slice_01/out";
it("i66 back-out: the carried receipt revalidates in the TypeScript reader", async () => {
  for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
    const fixture = JSON.parse(readFileSync(`${P}/fixtures/results/retained_precision_milestone_successor_${mode}.json`, "utf8"));
    const doc = JSON.parse(readFileSync(`${OUT}/derivative_${mode}.json`, "utf8"));
    const carried = doc.result_envelope.retained_precision;
    expect(carried).toStrictEqual(fixture.source.retained_precision);
    const before = await validateRetainedPrecision(fixture.source, fixture.invocation);
    const back = { ...structuredClone(fixture.source), retained_precision: structuredClone(carried) };
    const after = await validateRetainedPrecision(back, fixture.invocation);
    expect(after).toStrictEqual(before);
    expect(after.numerical_eligible).toBe(false);
    expect(after.standing).toBe("needs_recompute");
    const e = doc.result_envelope;
    const view = { schema_version: "0.2.0", producer: e.producer, numerical_quality: e.numerical_quality, formulation_basis: e.formulation_basis, contract_evidence: e.contract_evidence, retained_precision: carried };
    const transport = await validateRetainedPrecisionTransport(view);
    expect(transport.numerical_eligible).toBe(false);
    expect(transport.publication_sha256).toBe(before.publication_sha256);
    const absolute = after.classifications.filter(c => c.class === "absolute_verified").map(c => c.result_id).sort();
    const disclosed = e.row_disclosures.filter((x: { reason_code: string }) => x.reason_code === "retained_precision_absolute_verified").map((x: { source_result_id: string }) => x.source_result_id).sort();
    expect(disclosed).toStrictEqual(absolute);
    appendFileSync("WT/scratch/i66_u6a_slice_01/logs/lane_ts_backout.results", `I66_TS ${mode}: receipt equal; ts reader revalidates (${after.classifications.length} classes; ${absolute.length} absolute disclosed; eligible=${after.numerical_eligible})\n`);
  }
}, 600000);
