import { describe, expect, it } from "vitest";
import parityFixture from "../../../../../fixtures/reports/invented/state_comparison_handoff_desktop_parity.json";
import { buildStateComparisonHandoffParityFixture } from "./stateComparisonHandoffSections";

describe("DEL-08-06 desktop parity fixture", () => {
  it("matches the Python-engine-generated shared fixture", () => {
    expect(buildStateComparisonHandoffParityFixture(parityFixture.inputs)).toEqual(parityFixture.expected);
  });

  it("fails a deliberate parity mutation", () => {
    const mutation = {
      ...parityFixture.expected,
      section_contract_status: "mutated"
    };
    expect(buildStateComparisonHandoffParityFixture(parityFixture.inputs)).not.toEqual(mutation);
  });
});
