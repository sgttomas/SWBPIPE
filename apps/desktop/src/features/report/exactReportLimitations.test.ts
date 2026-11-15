// W-11 (DEL-08-01 Design `exact-route-report-sections.md` §2.2, §2.3 and §7,
// slice (a)): the exact-route statement table against the producer's coverage
// fixture (`fixtures/reports/exact_route_limitations.json`, written from the
// public producer by `core/product_physics/tests/exact_route_limitations.rs`), its digest
// keying, its verbatim statements and the source-paragraph omission, its fail-
// closed refusal and the claims check. It writes the table's Rust form,
// `fixtures/reports/exact_report_statements.json`, and fails when that file is
// stale; `OPENPIPESTRESS_WRITE_EXACT_REPORT_STATEMENTS=<path>` writes it there
// instead. Standard claim fence applies (claims taxonomy per DEC-081).
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  ELONGATION_TRANSLITERATION,
  EXACT_REPORT_STATEMENTS,
  REPORT_EXACT_LIMITATION_UNREGISTERED,
  exactReportStatementFor,
  exactReportStatements,
  producerStringSha256
} from "./exactReportLimitations";

const here = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(here, "../../../../..");
const COVERAGE = path.join(projectRoot, "fixtures/reports/exact_route_limitations.json");
const STATEMENTS = path.join(projectRoot, "fixtures/reports/exact_report_statements.json");
const SECTIONS_PARITY = path.join(projectRoot, "fixtures/reports/invented/exact_report_sections_parity.json");
const REGISTRY = path.join(projectRoot, "docs/claims_registry.md");
const WRITE_ENV = "OPENPIPESTRESS_WRITE_EXACT_REPORT_STATEMENTS";
const GENERATOR = "apps/desktop/src/features/report/exactReportLimitations.test.ts";

/** The source paragraphs' last sentence, which the report omits (DEC-107). */
const OMITTED_SENTENCE = "No code compliance or professional acceptance is produced, and actual numerical admission remains separate.";

/** The claim-litany vocabulary these product statements must not use. */
const LITANY_TERMS = [
  "certif", "seal", "authenticat", "code-compliance", "code compliance", "professional",
  "approval", "approve", "endorse", "release-readiness", "production-readiness"
];

type CoverageList = {
  semantic_contract_id: string;
  load_reference_states: boolean;
  objective_connectors: boolean;
  realized_bends: boolean;
  /** RT-L2: the elongation statement the list carries. */
  pressure_elongation: "not_included" | "included" | "no_effect";
  limitations: string[];
};

/** RT-L2: the three fixed elongation statements, by the coverage list's key. */
const ELONGATION_STATEMENTS: Readonly<Record<CoverageList["pressure_elongation"], string>> = Object.freeze({
  not_included: "Pressure elongation is not included in any load case: the closed-end free pressure strain is omitted; pressure thrust at unbalanced locations is applied.",
  included: "Pressure elongation is included only in the operating load cases the model settings name, as the closed-end free strain (1 \u2212 2\u03bd)\u00b7p\u00b7A_i/(E\u00b7A_s), with A_i on the pressure bore and A_s the nominal section area; it is not a thin-wall mean-diameter form.",
  no_effect: "Pressure elongation is set, but none of the cases it names carries pressure; it has no effect."
});
type Coverage = { schema_version: string; generator: string; lists: CoverageList[]; strings: string[] };

const coverage = JSON.parse(readFileSync(COVERAGE, "utf8")) as Coverage;

function normalize(text: string): string {
  return text.replace(/\s+/g, " ").trim().toLowerCase();
}

/** The BS-ACCEPT canonical text and short variants, read from the registry's §1 entry. */
function bsAcceptTexts(): string[] {
  const registry = readFileSync(REGISTRY, "utf8");
  const start = registry.indexOf("### BS-ACCEPT");
  const end = registry.indexOf("**Belongs on:**", start);
  expect(start).toBeGreaterThan(0);
  expect(end).toBeGreaterThan(start);
  const texts = Array.from(registry.slice(start, end).matchAll(/"([^"]+)"/g), (match) => normalize(match[1]));
  expect(texts).toHaveLength(5);
  return texts;
}

/** WinAnsiEncoding (CP1252) coverage, as the PDF emitter maps characters
 * (`core/reporting/pdf_emitter/src/lib.rs`, `winansi_byte`). */
const WINANSI_EXTRA = new Set([
  0x20ac, 0x201a, 0x0192, 0x201e, 0x2026, 0x2020, 0x2021, 0x02c6, 0x2030, 0x0160, 0x2039, 0x0152, 0x017d,
  0x2018, 0x2019, 0x201c, 0x201d, 0x2022, 0x2013, 0x2014, 0x02dc, 0x2122, 0x0161, 0x203a, 0x0153, 0x017e, 0x0178
]);
function winAnsiEncodable(text: string): boolean {
  return Array.from(text).every((ch) => {
    const code = ch.codePointAt(0) ?? 0;
    return (code >= 0x20 && code <= 0x7e) || (code >= 0xa0 && code <= 0xff) || WINANSI_EXTRA.has(code);
  });
}

/** Every string value in a JSON value. */
function strings(value: unknown, out: string[] = []): string[] {
  if (typeof value === "string") out.push(value);
  else if (Array.isArray(value)) value.forEach((item) => strings(item, out));
  else if (value && typeof value === "object") Object.values(value).forEach((item) => strings(item, out));
  return out;
}

describe("exact-route statement table (W-11 slice (a))", () => {
  it("covers exactly the coverage fixture's strings, keyed by the SHA-256 of their UTF-8 bytes", async () => {
    expect(coverage.schema_version).toBe("1.0.0");
    // The ten shapes the producer emits, each with every elongation statement it
    // can carry (RT-L2): not included and no effect on all ten, included on the
    // seven with a pressurised input. The six pressure-source-1 shapes with a
    // connector or a realized bend are never emitted, because source recovery refuses both.
    expect(coverage.lists).toHaveLength(27);
    expect(coverage.strings).toHaveLength(22);
    const fixtureDigests = await Promise.all(coverage.strings.map(producerStringSha256));
    const tableDigests = EXACT_REPORT_STATEMENTS.map((item) => item.producer_sha256);
    expect(EXACT_REPORT_STATEMENTS).toHaveLength(22);
    expect(new Set(tableDigests).size).toBe(22);
    expect([...tableDigests].sort()).toEqual([...fixtureDigests].sort());
    for (const digest of tableDigests) expect(digest).toMatch(/^[0-9a-f]{64}$/);
    // The fixture's strings are its lists' strings, and nothing else.
    expect(new Set(coverage.lists.flatMap((list) => list.limitations))).toEqual(new Set(coverage.strings));
  });

  it("matches only the exact producer bytes", async () => {
    const text = coverage.strings[0];
    expect(exactReportStatementFor(await producerStringSha256(text))).not.toBeNull();
    for (const variant of [`${text} `, text.toUpperCase(), text.normalize("NFD"), text.slice(0, -1)]) {
      if (variant === text) continue;
      expect(exactReportStatementFor(await producerStringSha256(variant))).toBeNull();
    }
    expect(exactReportStatementFor(text)).toBeNull();
  });

  it("copies each statement verbatim, except the two source paragraphs, which omit their last sentence, and the spelled-out elongation formula", async () => {
    const omitted = coverage.strings.filter((text) => text.endsWith(OMITTED_SENTENCE));
    expect(omitted).toHaveLength(2);
    for (const text of coverage.strings) {
      const statement = exactReportStatementFor(await producerStringSha256(text));
      if (omitted.includes(text)) {
        expect(statement).toBe(text.slice(0, -(OMITTED_SENTENCE.length + 1)));
        expect(`${statement} ${OMITTED_SENTENCE}`).toBe(text);
      } else {
        const spelled = ELONGATION_TRANSLITERATION.reduce((out, [from, to]) => out.split(from).join(to), text);
        expect(statement).toBe(spelled);
      }
    }
    // RT-L2: only S4b's formula is spelled out, and only its two characters outside WinAnsi.
    const spelledOut = coverage.strings.filter((text) => ELONGATION_TRANSLITERATION.some(([from]) => text.includes(from)));
    expect(spelledOut).toHaveLength(1);
    expect(spelledOut[0].startsWith("Pressure elongation is included only in the operating load cases")).toBe(true);
    // The source paragraphs are S9 and S10: the pressure-3 and the
    // pressure-source-1 lists' ninth entry (after RT-L2's elongation statement).
    const ninth = new Set(coverage.lists.map((list) => list.limitations[8]));
    expect(ninth).toEqual(new Set(omitted));
    // RT-L2: each list carries exactly one elongation statement, the one its key names, after "Not modelled".
    for (const list of coverage.lists) {
      const at = list.limitations.findIndex((text) => text.startsWith("Not modelled: "));
      const stated = list.limitations.filter((text) => text.startsWith("Pressure elongation is "));
      expect(stated).toEqual([list.limitations[at + 1]]);
      expect(stated[0]).toBe(ELONGATION_STATEMENTS[list.pressure_elongation]);
    }
    for (const text of Object.values(ELONGATION_STATEMENTS)) expect(coverage.strings).toContain(text);
  });

  it("maps every fixture list, in the producer's order, one statement per string", async () => {
    for (const list of coverage.lists) {
      const statements = await exactReportStatements(list.semantic_contract_id, list.limitations);
      expect(statements).toHaveLength(list.limitations.length);
      for (const [index, text] of list.limitations.entries()) {
        expect(statements[index]).toBe(exactReportStatementFor(await producerStringSha256(text)));
      }
    }
  });

  it("fails closed on a producer string it has no reviewed statement for", async () => {
    const list = coverage.lists[0];
    const identity = list.semantic_contract_id;
    const expected = `${REPORT_EXACT_LIMITATION_UNREGISTERED}: ${identity} publishes a limitation this report has no reviewed statement for.`;
    const source = list.limitations[8];
    expect(source.endsWith(OMITTED_SENTENCE)).toBe(true);
    for (const changed of [
      [...list.limitations, "An invented limitation the producer never published."],
      // RT-A3 §4 review cert2 (item 2 withdrawn): an appended certificate claim the source readers admit as
      // untrusted text never reaches a report.
      [...list.limitations, "combination comb:cancel: all derived row groups are established within their stated bound of 1e-30."],
      list.limitations.map((text, index) => (index === 8 ? text.replace("ordinary structural recovery", "ordinary structural response") : text)),
      list.limitations.map((text, index) => (index === 8 ? text.slice(0, -(OMITTED_SENTENCE.length + 1)) : text))
    ]) {
      expect(changed).not.toEqual(list.limitations);
      await expect(exactReportStatements(identity, changed)).rejects.toThrow(expected);
    }
  });

  it("RT-L2: binds the included elongation statement and refuses the retired bend statement and a transliterated producer string", async () => {
    const included = coverage.lists.find((list) => list.pressure_elongation === "included" && list.realized_bends);
    expect(included).toBeDefined();
    const list = included!.limitations;
    const at = list.findIndex((text) => text.startsWith("Pressure elongation is included"));
    const expected = `${REPORT_EXACT_LIMITATION_UNREGISTERED}: ${included!.semantic_contract_id} publishes a limitation this report has no reviewed statement for.`;
    // The retired bend statement: every realized arc owned the free strain in every case.
    const retiredBend = "each owns the term K_b*u_free(eps_p) - c_b, with eps_p = (1-2nu)pAi/(E As) (the straight Lame mean Poisson term on the arc) and c_b its own end caps along the end tangents;";
    const revisedBend = "each owns the term -c_b (c_b its own end caps along the end tangents) and, in a load case that includes pressure elongation, the term K_b*u_free(eps_p) from the arc's own stiffness, with eps_p = (1-2nu)pAi/(E As) (the straight Lame mean Poisson term on the arc);";
    const bend = list.findIndex((text) => text.includes(revisedBend));
    expect(bend).toBeGreaterThan(at);
    for (const changed of [
      list.map((text, index) => (index === bend ? text.replace(revisedBend, retiredBend) : text)),
      list.map((text, index) => (index === at ? text.replace("(1 \u2212 2\u03bd)", "(1 - 2nu)") : text))
    ]) {
      expect(changed).not.toEqual(list);
      await expect(exactReportStatements(included!.semantic_contract_id, changed)).rejects.toThrow(expected);
    }
    const statements = await exactReportStatements(included!.semantic_contract_id, list);
    expect(statements[at]).toBe("Pressure elongation is included only in the operating load cases the model settings name, as the closed-end free strain (1 - 2nu)\u00b7p\u00b7A_i/(E\u00b7A_s), with A_i on the pressure bore and A_s the nominal section area; it is not a thin-wall mean-diameter form.");
  });

  it("writes its Rust form, exact_report_statements.json, and fails when it is stale", () => {
    const document = {
      schema_version: "1.0.0",
      generator: GENERATOR,
      entries: [...EXACT_REPORT_STATEMENTS]
        .sort((left, right) => (left.producer_sha256 < right.producer_sha256 ? -1 : 1))
        .map((item) => ({ producer_sha256: item.producer_sha256, statement: item.statement }))
    };
    const text = `${JSON.stringify(document, null, 2)}\n`;
    const target = process.env[WRITE_ENV];
    if (target) {
      writeFileSync(target, text);
      return;
    }
    const committed = readFileSync(STATEMENTS, "utf8");
    expect(text === committed, `fixtures/reports/exact_report_statements.json is stale; regenerate it with ${WRITE_ENV}=<path>`).toBe(true);
  });

  it("passes the claims check: no BS-ACCEPT text, no omitted sentence, no litany, no acceptance, WinAnsi only", () => {
    const parity = JSON.parse(readFileSync(SECTIONS_PARITY, "utf8")) as { vectors: { expected: unknown }[] };
    const sectionTexts = parity.vectors.flatMap((vector) => strings(vector.expected));
    expect(sectionTexts.length).toBeGreaterThan(0);
    const texts = [
      ...EXACT_REPORT_STATEMENTS.map((item) => item.statement),
      `${REPORT_EXACT_LIMITATION_UNREGISTERED}: <identity> publishes a limitation this report has no reviewed statement for.`,
      ...sectionTexts
    ];
    const accept = bsAcceptTexts();
    const omitted = normalize(OMITTED_SENTENCE);
    for (const text of texts) {
      const normalized = normalize(text);
      for (const variant of accept) expect(normalized.includes(variant), text).toBe(false);
      expect(normalized.includes(omitted), text).toBe(false);
      expect(LITANY_TERMS.filter((term) => normalized.includes(term)).length < 3, text).toBe(true);
      expect(normalized.includes("acceptance"), text).toBe(false);
      expect(winAnsiEncodable(text), text).toBe(true);
    }
  });
});
