/** Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §5.4, §6.1; owner ruling 11B,
 * 2026-10-21): the one disclosure line each opened output panel shows for an exact-route
 * result that carries resolved load/reference states. The line reads only the published
 * records' reference configuration, load case and history kind, in published order and
 * without duplicates. Unit tests over committed producer bytes; all inputs are invented. */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import type { MechanicsResult } from "../../types";
import { receivedFixture } from "../../test-support/receivedText";
import { sourceContract } from "./resultReader";
import {
  LoadReferenceDisclosure, N_LOAD_REFERENCE_EXPORT, N_LOAD_REFERENCE_INCLUDED, N_LOAD_REFERENCE_PACKAGE, N_LOAD_REFERENCE_RENDERED_REPORT, N_LOAD_REFERENCE_RULE_CHECK,
  loadReferenceDisclosure, type LoadReferenceDisclosureSurface,
} from "./loadReferenceDisclosure";

afterEach(() => cleanup());
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

const PRESSURE_LR = "fixtures/product_preview/pressure/load_reference";
const SOURCE_LR = "fixtures/product_preview/pressure_source/load_reference_source";
const SURFACES: readonly [LoadReferenceDisclosureSurface, string][] = [
  ["result-export", N_LOAD_REFERENCE_EXPORT],
  ["stress-neutral", N_LOAD_REFERENCE_PACKAGE],
  ["rule-check", N_LOAD_REFERENCE_RULE_CHECK],
  ["rendered-report", N_LOAD_REFERENCE_RENDERED_REPORT],
];
const fill = (template: string, configurations: string, cases: string, histories: string) =>
  template.replace("{configurations}", configurations).replace("{cases}", cases).replace("{histories}", histories);
/** The published identifiers, read from the source's own records (in order, distinct). */
function published(source: MechanicsResult) {
  const records = (source.contract_evidence as Json).load_reference_states as Json[];
  const distinct = (values: string[]) => values.filter((value, index) => values.indexOf(value) === index);
  return {
    configurations: distinct(records.map(r => r.reference_configuration_id)).join(", "),
    cases: distinct(records.map(r => r.load_case_id)).join(", "),
    histories: distinct(records.map(r => r.history.kind)).join(", "),
  };
}

describe("the disclosure texts (§5.4)", () => {
  it("are the design's constants, one first sentence and the surface's own tail", () => {
    expect(N_LOAD_REFERENCE_INCLUDED).toBe("This result includes resolved load/reference states (reference configuration {configurations}; load cases {cases}; history {histories}).");
    expect(N_LOAD_REFERENCE_EXPORT).toBe(`${N_LOAD_REFERENCE_INCLUDED} The export carries them in contract_evidence.load_reference_states, with the producer's limitations.`);
    expect(N_LOAD_REFERENCE_PACKAGE).toBe(`${N_LOAD_REFERENCE_INCLUDED} The package carries them in contract_evidence.load_reference_states, with the producer's limitations.`);
    expect(N_LOAD_REFERENCE_RULE_CHECK).toBe(`${N_LOAD_REFERENCE_INCLUDED} Rules bind the published rows of these load cases, which are solved with these states' resolved contributions.`);
    // W-11 §3.2; the report package's Rust producer composes the same text (`LOAD_REFERENCE_TEMPLATE`).
    expect(N_LOAD_REFERENCE_RENDERED_REPORT).toBe(`${N_LOAD_REFERENCE_INCLUDED} Their published limitations are listed under Limitations.`);
    // A deliverable-specific scope statement in ordinary prose (docs/claims_registry.md §4):
    // no boundary statement, status label or finding about the result.
    for (const [, text] of SURFACES) {
      expect(text.toLowerCase()).not.toMatch(/invalid|unsupported|authoritative|accept|certif|complian|approv|seal|not a finding/);
      expect(text).not.toMatch(/\bT[0-9]/);
    }
  });
});

describe("loadReferenceDisclosure(source, surface)", () => {
  it.each([
    ["connected", "case:cold, case:hot"],
    ["pressure", "case:cold-pressure, case:hot-pressure"],
  ] as const)("names pressure-2 %s's reference configuration, load cases and history on each opened surface", (name, cases) => {
    for (const mode of ["sparse_interactive", "dense_scrutiny"]) {
      const source = receivedFixture<MechanicsResult>(`${PRESSURE_LR}/${name}-${mode}.raw.json`);
      expect(sourceContract(source)).toBe("pressure");
      // The published records, as committed: two records with one shared configuration
      // and history, so each is named once.
      expect(published(source)).toStrictEqual({ configurations: "reference:installed", cases, histories: "independent_equilibrium" });
      for (const [surface, template] of SURFACES) {
        const line = loadReferenceDisclosure(source, surface);
        expect(line, `${mode} ${surface}`).toBe(fill(template, "reference:installed", cases, "independent_equilibrium"));
        expect(line!.match(/reference:installed/g)).toHaveLength(1);
        expect(line!.match(/independent_equilibrium/g)).toHaveLength(1);
      }
    }
  });
  it("names pressure-source-1's published identifiers likewise, in published order and without duplicates", () => {
    // The function is route-level: a panel shows it only where it admits the result, and a
    // pressure-source-1 load/reference-state result is never admitted (Rust's standing).
    const source = receivedFixture<MechanicsResult>(`${SOURCE_LR}/mixed-sparse_interactive.raw.json`);
    expect(sourceContract(source)).toBe("pressure_source");
    const ids = published(source);
    expect(ids).toStrictEqual({ configurations: "reference:companion", cases: "case, case:ordinary-pressure", histories: "independent_equilibrium" });
    expect(loadReferenceDisclosure(source, "stress-neutral")).toBe(fill(N_LOAD_REFERENCE_PACKAGE, ids.configurations, ids.cases, ids.histories));
  });
  it("shows identifiers verbatim whatever they contain ($&, $', $$, a literal {cases}); the expected line is built by concatenation", () => {
    const source = structuredClone(receivedFixture<MechanicsResult>(`${PRESSURE_LR}/connected-sparse_interactive.raw.json`));
    const records = (source.contract_evidence as Json).load_reference_states as Json[];
    expect(records).toHaveLength(2);
    // Invented identifiers carrying String.replace's special patterns and a template placeholder.
    const configuration = "reference:$&-{cases}-$'";
    records[0].reference_configuration_id = configuration;
    records[1].reference_configuration_id = configuration;
    records[0].load_case_id = "case:$$cold";
    records[1].load_case_id = "case:{histories}$`";
    expect(sourceContract(source)).toBe("pressure");
    const included = "This result includes resolved load/reference states (reference configuration " + configuration
      + "; load cases " + "case:$$cold" + ", " + "case:{histories}$`" + "; history " + "independent_equilibrium" + ").";
    const expected: Record<LoadReferenceDisclosureSurface, string> = {
      "result-export": included + " " + "The export carries them in contract_evidence.load_reference_states, with the producer's limitations.",
      "stress-neutral": included + " " + "The package carries them in contract_evidence.load_reference_states, with the producer's limitations.",
      "rule-check": included + " " + "Rules bind the published rows of these load cases, which are solved with these states' resolved contributions.",
      // W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.2): the rendered report's line.
      "rendered-report": included + " " + "Their published limitations are listed under Limitations.",
    };
    for (const [surface] of SURFACES) {
      const line = loadReferenceDisclosure(source, surface)!;
      expect(line, surface).toBe(expected[surface]);
      for (const identifier of [configuration, "case:$$cold", "case:{histories}$`"]) expect(line.split(identifier).length - 1, `${surface} ${identifier}`).toBe(1);
      expect(line).not.toContain("{configurations}");
    }
  });
  it("reads only the published records and changes no byte", () => {
    const source = receivedFixture<MechanicsResult>(`${PRESSURE_LR}/connected-sparse_interactive.raw.json`);
    const before = JSON.stringify(source);
    for (const [surface] of SURFACES) loadReferenceDisclosure(source, surface);
    expect(JSON.stringify(source)).toBe(before);
  });
  it("is null for a plain exact result, a preview-physics-2 result, no result and an unsupported source", () => {
    const plainPressure = receivedFixture<MechanicsResult>("fixtures/results/pressure_connected_mechanics_sparse.json");
    const plainSource = receivedFixture<MechanicsResult>("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json");
    const preview = receivedFixture<MechanicsResult>("fixtures/results/preview_physics_connected_sparse.json");
    expect(sourceContract(plainPressure)).toBe("pressure");
    expect(sourceContract(plainSource)).toBe("pressure_source");
    expect(sourceContract(preview)).toBe("preview_physics");
    // A stored statement naming the retired load-reference-1 identity, still carrying the
    // group, reads unsupported (B7b) and discloses nothing.
    const unsupported = structuredClone(receivedFixture<MechanicsResult>(`${PRESSURE_LR}/connected-sparse_interactive.raw.json`));
    unsupported.producer = { ...unsupported.producer!, semantic_contract_id: "openpipestress.result_semantics/0.3.0/load-reference-1" };
    expect(sourceContract(unsupported)).toBe("unsupported");
    expect(Array.isArray((unsupported.contract_evidence as Json).load_reference_states)).toBe(true);
    for (const [surface] of SURFACES) {
      expect(loadReferenceDisclosure(plainPressure, surface)).toBeNull();
      expect(loadReferenceDisclosure(plainSource, surface)).toBeNull();
      expect(loadReferenceDisclosure(preview, surface)).toBeNull();
      expect(loadReferenceDisclosure(null, surface)).toBeNull();
      expect(loadReferenceDisclosure(undefined, surface)).toBeNull();
      expect(loadReferenceDisclosure(unsupported, surface)).toBeNull();
    }
  });
});

describe("<LoadReferenceDisclosure>", () => {
  it.each(SURFACES)("renders the %s line under {surface}-load-reference-disclosure", (surface) => {
    const text = loadReferenceDisclosure(receivedFixture<MechanicsResult>(`${PRESSURE_LR}/pressure-sparse_interactive.raw.json`), surface)!;
    render(<LoadReferenceDisclosure text={text} surface={surface} />);
    expect(screen.getByTestId(`${surface}-load-reference-disclosure`).textContent).toBe(text);
  });
  it("uses the testIdPrefix when given (the rule-check run panel), and renders nothing without a line", () => {
    const text = loadReferenceDisclosure(receivedFixture<MechanicsResult>(`${PRESSURE_LR}/connected-sparse_interactive.raw.json`), "rule-check")!;
    render(<LoadReferenceDisclosure text={text} surface="rule-check" testIdPrefix="rule-check-run" />);
    expect(screen.getByTestId("rule-check-run-load-reference-disclosure").textContent).toBe(text);
    expect(screen.queryByTestId("rule-check-load-reference-disclosure")).toBeNull();
    cleanup();
    for (const empty of [null, undefined, ""]) {
      const { container } = render(<LoadReferenceDisclosure text={empty} surface="result-export" />);
      expect(container.innerHTML).toBe("");
      cleanup();
    }
  });
});
