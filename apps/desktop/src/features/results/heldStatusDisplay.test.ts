// RT-A5 `minuend_final` (DEL-05-02 `operating-case.md` §7.6): the desktop's display classification
// of a held-constraint comparison, over the producer's reader corpus. Held-operand rows are the held
// system's, never one solved state of a case, so they are never drawn; the difference rows are drawn
// as a plain subtraction's and the navigator names them; the held standing is stated as a notice.
import { describe, expect, it } from "vitest";
import corpus from "../../../../../fixtures/results/preview_physics_held_status_corpus.json";
import type { MechanicsResult } from "../../types";
import { declaredDifferenceNote, HELD_STATUS_OPERAND_BASIS, isHeldOperandRow, isOneSolvedState } from "../viewport/resultColourOverlay";
import { heldStatusStandings, knownSemanticNotices } from "./knownSemanticLimitations";
import { navigatorCases } from "./resultsNavigator";

const COMBINATION = "comb:o-s-held";
type Entry = { name: string; document: unknown };
const documentOf = (name: string): MechanicsResult =>
  structuredClone((corpus as { documents: Entry[] }).documents.find((entry) => entry.name === name)!.document) as MechanicsResult;

describe("a held-constraint comparison on the desktop", () => {
  it("never draws a held-operand row and draws the difference rows as one subtraction", () => {
    const result = documentOf("h2_released");
    const combination = result.results.filter((row) => row.basis_ref?.ref_type === "combination" && row.basis_ref.ref_id === COMBINATION);
    const held = combination.filter(isHeldOperandRow);
    const difference = combination.filter((row) => !isHeldOperandRow(row) && row.metadata && row.kind !== "combination_modulus_basis_record");
    expect(held.length).toBeGreaterThan(0);
    expect(difference.length).toBeGreaterThan(0);
    for (const row of held) {
      expect(row.id).toContain(":held:");
      expect(row.metadata?.basis).toBe(HELD_STATUS_OPERAND_BASIS);
      expect(isOneSolvedState(row, result.results)).toBe(false);
    }
    for (const row of difference) {
      expect(declaredDifferenceNote(row)).toBe("operating minus sustained difference at the operating support status");
      expect(isOneSolvedState(row, result.results)).toBe(true);
    }
    // A case row is never held.
    expect(result.results.filter((row) => row.basis_ref?.ref_type === "load_case").some(isHeldOperandRow)).toBe(false);
  });

  it("names the held difference in the navigator, not the held operand", () => {
    const result = documentOf("h2_released");
    const entry = navigatorCases(result.results).find((c) => c.basis?.ref_type === "combination" && c.basis.ref_id === COMBINATION);
    expect(entry?.note).toBe("operating minus sustained difference at the operating support status");
  });

  it("states each admitted record's held standing, and none for a withheld one", () => {
    const released = documentOf("h2_released");
    expect(heldStatusStandings(released)).toEqual([{
      combination_id: COMBINATION,
      kind: "not_established",
      reason: "Not established: the held status is not consistent with this case's loads; rows are inspection evidence.",
    }]);
    const notice = knownSemanticNotices(released).find((n) => n.id === `held-status:${COMBINATION}`);
    expect(notice?.text).toContain("subtrahend held at the minuend's final support status.");
    const established = documentOf("h4_kept_explicit");
    expect(heldStatusStandings(established)[0].kind).toBe("established");
    const withheld = documentOf("source_unavailable");
    expect(heldStatusStandings(withheld)).toEqual([]);
    expect(knownSemanticNotices(withheld).some((n) => n.id === `combination-gate:${COMBINATION}` && n.text.endsWith("HELD_STATUS_SOURCE_UNAVAILABLE"))).toBe(true);
  });
});
