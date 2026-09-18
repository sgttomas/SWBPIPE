import { describe, expect, it } from "vitest";
import {
  REGISTERED_STATUS_LABELS,
  professionalStatusToken,
  readableStatusValue,
  registeredStatusLabel,
  ruleCheckStatusToken,
  statusDisplay,
  statusDisplayWithToken
} from "./statusLabels";

// The display-form table of docs/claims_registry.md §2 (DEC-102): eight rows,
// each with its authority domain (docs/SPEC.md §4.3).
const REGISTERED_FORMS = [
  ["MODEL_INCOMPLETE", "Model incomplete", "Solver"],
  ["MECHANICS_SOLVED", "Mechanics solved", "Solver"],
  ["RULE_INPUTS_INCOMPLETE", "Rule inputs incomplete", "Rule pack"],
  ["USER_RULE_CHECKED", "User-rule checked", "Rule pack"],
  ["USER_RULE_FAILED", "User-rule failed", "Rule pack"],
  ["HUMAN_REVIEW_REQUIRED", "Human review required", "Human"],
  ["INTERNALLY_VERIFIED", "Internally verified", "Evidence"],
  ["PROVER_CORRELATED", "Prover correlated", "Evidence"]
] as const;

describe("registered status labels (DEC-102)", () => {
  it("holds exactly the eight registered rows", () => {
    expect(Object.keys(REGISTERED_STATUS_LABELS)).toEqual(REGISTERED_FORMS.map(([token]) => token));
  });

  it.each(REGISTERED_FORMS)("pins %s as %s under %s", (token, label, domain) => {
    const row = registeredStatusLabel(token);
    expect(row).toMatchObject({ token, label, domain });
    expect(statusDisplay(token)).toBe(`${domain} · ${label}`);
    expect(statusDisplayWithToken(token)).toBe(`${domain} · ${label} (${token})`);
    // Recorded values are matched without regard to case.
    expect(registeredStatusLabel(token.toLowerCase())).toBe(row);
  });

  it("classes the six software statuses and the two evidence labels", () => {
    const kinds = Object.values(REGISTERED_STATUS_LABELS).map((row) => row.kind);
    expect(kinds.filter((kind) => kind === "status")).toHaveLength(6);
    expect(kinds.filter((kind) => kind === "evidence")).toHaveLength(2);
  });

  it("gives no registered form to a value outside the table", () => {
    expect(registeredStatusLabel("ENGINEER_ACCEPTED")).toBeNull();
    expect(registeredStatusLabel("NOT_PROVIDED")).toBeNull();
    expect(registeredStatusLabel("toString")).toBeNull();
    expect(statusDisplay("ENGINEER_ACCEPTED")).toBe("engineer accepted");
    expect(statusDisplayWithToken("ready_for_preview_diagnostics")).toBe("ready for preview diagnostics");
  });

  it("keeps unregistered values readable without the retired curated labels", () => {
    expect(readableStatusValue("not_run")).toBe("Not run");
    expect(readableStatusValue("NOT_COMPUTED")).toBe("Not run");
    expect(readableStatusValue("computed_for_invented_demo")).toBe("Demo computed");
    const everyDisplay = [
      ...REGISTERED_FORMS.map(([token]) => statusDisplay(token)),
      statusDisplay("not_provided"),
      statusDisplay("not_performed_user_rule_inputs_missing")
    ].join("\n");
    expect(everyDisplay).not.toContain("Review required");
    expect(everyDisplay).not.toContain("Inputs needed");
  });

  it("maps the two legacy fixture enums to the token the product records for them", () => {
    expect(ruleCheckStatusToken("not_performed_user_rule_inputs_missing")).toBe("RULE_INPUTS_INCOMPLETE");
    expect(ruleCheckStatusToken("USER_RULE_FAILED")).toBe("USER_RULE_FAILED");
    expect(professionalStatusToken("not_provided")).toBe("HUMAN_REVIEW_REQUIRED");
    expect(professionalStatusToken("NOT_PROVIDED")).toBe("HUMAN_REVIEW_REQUIRED");
    expect(professionalStatusToken("other")).toBe("other");
  });
});
