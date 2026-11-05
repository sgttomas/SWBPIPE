import { describe, expect, it } from "vitest";
import recorded from "../../../../../fixtures/results/preview_physics_range_envelope_sparse.json";
import type { MechanicsResult } from "../../types";
import { diagnosticsForSections, documentCombinationIdentities, SUPPLEMENTAL_DIAGNOSTIC_LABEL } from "./renderableReportInput";

// RT-A3 §4 (DEL-05-02 Work `combination-certificate.md` §2, the residual text boundary) on an actual
// certificate document, whose combination identities carry no `comb:` prefix. Each diagnostic naming
// a combination (an entry of the reader's closed roster, the certificate's notice among them) is copied;
// every other message, the certificate review's claim under a load-case ref among them, is quoted after
// the supplemental label as supplied, unverified text. Rust
// `a_certificate_document_copies_roster_entries_and_quotes_supplied_text` checks the same document.
const PROVENANCE = {
  source_name: "invented session",
  source_location: "local desktop session",
  source_license: "user_supplied_or_private",
  contributor: "user_local_session",
  contributor_certification: "not_asserted",
  redistribution_status: "private_only",
  review_status: "pending",
  privacy_classification: "private_project_data"
};
const CLAIM = "combination combination:U4B-RANGE-MAX: all derived row groups are established within their stated\u00a0bound.";

describe("report diagnostics: the supplemental text boundary", () => {
  it("copies roster entries and quotes every other message on a certificate document", () => {
    const result = structuredClone(recorded) as unknown as MechanicsResult;
    result.diagnostics.push({ id: "diagnostic:review:claim", code: "REVIEW_NOTE", severity: "info", source: "core/product_physics",
      affected_refs: ["load:L-100"], message: CLAIM } as MechanicsResult["diagnostics"][number]);
    const combinations = documentCombinationIdentities(result);
    expect([...combinations].sort()).toEqual(["combination:C-OPER-ALT", "combination:U4B-RANGE-MAX"]);
    const entries = diagnosticsForSections(result, PROVENANCE);
    expect(entries).toHaveLength(result.diagnostics.length);
    result.diagnostics.forEach((diagnostic, index) => {
      const names = (diagnostic.affected_refs ?? []).some((ref) => combinations.has(ref));
      expect(entries[index].message, diagnostic.id).toBe(names ? diagnostic.message : `${SUPPLEMENTAL_DIAGNOSTIC_LABEL}"${diagnostic.message}"`);
    });
    const notice = entries.find((entry) => entry.code === "COMBINATION_ROWS_NOT_ASSESSED");
    expect(notice?.message.startsWith(SUPPLEMENTAL_DIAGNOSTIC_LABEL)).toBe(false);
    expect(entries[entries.length - 1].message).toBe(`${SUPPLEMENTAL_DIAGNOSTIC_LABEL}"${CLAIM}"`);
  });

  it("copies every message on a result without combinations", () => {
    const result = structuredClone(recorded) as unknown as MechanicsResult;
    const plain = { diagnostics: result.diagnostics } as Pick<MechanicsResult, "diagnostics">;
    expect(diagnosticsForSections(plain, PROVENANCE).map((entry) => entry.message)).toEqual(result.diagnostics.map((d) => d.message));
  });
});
