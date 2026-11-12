// DEL-14-04 AC-002: "Where a row has no unit or dimension, the dimensions disagree,
// or a value is not a finite number, the pair is reported not compared with its
// reason and no number." Rows map only when their whole identity is equal
// (`compareRunInputs`), so two mapped rows never disagree in dimension; the open
// question is a mapped pair whose dimension is missing.
//
// A row's family and dimension both come from one place: the signature the Rust
// reader returns for it from its source's semantic table (`flattenRunResult`). A
// row the reader cannot read has neither. These tests pin why a row the reader
// can read never has a family without a dimension: every physical-quantity
// signature of every table the reader selects (the current tables and the
// recorded pre-RT-A4 variants it reads recorded runs under) states both.
import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { compareRunInputs, flattenRunResult, NOT_COMPARED_REASONS, type RunComparisonInput, type RunRowIdentity } from "./runComparison";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

const RESULTS = resolve(__dirname, "../../../../../fixtures/results");

/** Every semantic table the reader's crate pins (`semantic_contract.rs`, `recorded_tables.rs`). */
function tables(): Array<{ path: string; table: Json }> {
  const paths = readdirSync(RESULTS).filter((name) => /^semantic_contract_v0_3_.*\.json$/.test(name)).map((name) => join(RESULTS, name));
  for (const directory of readdirSync(RESULTS).filter((name) => name.startsWith("recorded_pre_"))) {
    for (const name of readdirSync(join(RESULTS, directory))) if (/^semantic_contract_.*\.json$/.test(name)) paths.push(join(RESULTS, directory, name));
  }
  return paths.map((path) => ({ path, table: JSON.parse(readFileSync(path, "utf8")) }));
}

describe("a run row the reader can read has a dimension whenever it has a family (DEL-14-04 AC-002)", () => {
  it("every physical-quantity signature of every pinned table states its family and its dimension", () => {
    const all = tables();
    // The six current tables and the recorded variants.
    expect(all.length).toBeGreaterThanOrEqual(6 + 16);
    for (const { path, table } of all) {
      const rows = table.rows as Json[];
      expect(Array.isArray(rows), path).toBe(true);
      for (const row of rows) {
        if (row.category === "physical_quantity") {
          expect(typeof row.family === "string" && row.family.length > 0, `${path} ${row.signature_id}: family`).toBe(true);
          expect(typeof row.source_physical_semantic_dimension === "string" && row.source_physical_semantic_dimension.length > 0, `${path} ${row.signature_id}: dimension`).toBe(true);
        }
      }
    }
  });

  it("a flattened producer result gives every row with a family a dimension", () => {
    for (const mode of ["sparse_interactive", "dense_scrutiny"] as const) {
      for (const profile of ["preview", "pressure"] as const) {
        const rows = flattenRunResult(nativeMechanicsReplayPair(mode, { profile }).source);
        expect(rows.some((row) => row.identity.family !== null), `${mode} ${profile}`).toBe(true);
        for (const row of rows) {
          if (row.identity.family !== null) expect(row.identity.dimension, `${mode} ${profile} ${row.identity.result_id}`).not.toBeNull();
        }
      }
    }
  });

  it("a hand-made pair of a supported family with no dimension is not compared, with its reason and no number", () => {
    const identity: RunRowIdentity = {
      result_id: "result:force:invented", family: "force", object_ref: "element:invented", basis_ref: null, dimension: null,
      component: "fx", location: "end_i", basis: null, coordinate_system: "local", sign_convention: null
    };
    const side = (value: number): RunComparisonInput => ({
      rows: [{ identity, value, unit: "N", row_index: 0 }],
      standing: { eligible: true, status: "integrity_checked", contract: "preview_physics", findings: [] },
      semantic_identity: { route: "preview_physics", contract_id: "invented", contract_sha256: null },
      context: {},
      diagnostics: []
    });
    const comparison = compareRunInputs(side(1), side(4));
    expect(comparison.pairs).toEqual([
      { status: "not_compared", result_id: identity.result_id, identity, left_unit: "N", right_unit: "N", left: 1, right: 4, reasons: ["dimension_missing"] }
    ]);
    expect(comparison.summary.compared).toBe(0);
    expect(NOT_COMPARED_REASONS.at(-1)).toBe("dimension_missing");
    // With the dimension stated, the same pair is compared.
    const stated = (value: number) => ({ ...side(value), rows: [{ identity: { ...identity, dimension: "force" }, value, unit: "N", row_index: 0 }] });
    expect(compareRunInputs(stated(1), stated(4)).pairs[0]).toMatchObject({ status: "compared", delta: 3 });
  });
});
