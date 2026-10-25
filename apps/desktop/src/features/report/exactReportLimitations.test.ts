// W-11 (DEL-08-01 Design `exact-route-report-sections.md` §2.2, §2.3 and §7,
// slice (a)): the exact-route statement table against the producer's coverage
// fixture (`fixtures/reports/exact_route_limitations.json`, written by
// `core/product_physics/src/exact_route_limitations_tests.rs`), its digest
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

/** GEN-13's litany vocabulary (`tools/validation/validate_claims_language.py`, LITANY_TERMS). */
const LITANY_TERMS = [
  "certif", "seal", "authenticat", "code-compliance", "code compliance", "professional",
  "approval", "approve", "endorse", "release-readiness", "production-readiness"
];

type CoverageList = {
  semantic_contract_id: string;
  load_reference_states: boolean;
  objective_connectors: boolean;
  realized_bends: boolean;
  reachable: boolean;
  limitations: string[];
};
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
    expect(coverage.lists).toHaveLength(16);
    expect(coverage.strings).toHaveLength(19);
    const fixtureDigests = await Promise.all(coverage.strings.map(producerStringSha256));
    const tableDigests = EXACT_REPORT_STATEMENTS.map((item) => item.producer_sha256);
    expect(EXACT_REPORT_STATEMENTS).toHaveLength(19);
    expect(new Set(tableDigests).size).toBe(19);
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

  it("copies each statement verbatim, except the two source paragraphs, which omit their last sentence", async () => {
    const omitted = coverage.strings.filter((text) => text.endsWith(OMITTED_SENTENCE));
    expect(omitted).toHaveLength(2);
    for (const text of coverage.strings) {
      const statement = exactReportStatementFor(await producerStringSha256(text));
      if (omitted.includes(text)) {
        expect(statement).toBe(text.slice(0, -(OMITTED_SENTENCE.length + 1)));
        expect(`${statement} ${OMITTED_SENTENCE}`).toBe(text);
      } else {
        expect(statement).toBe(text);
      }
    }
    // The source paragraphs are S9 and S10: the pressure-2 and the
    // pressure-source-1 lists' eighth entry.
    const eighth = new Set(coverage.lists.map((list) => list.limitations[7]));
    expect(eighth).toEqual(new Set(omitted));
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
    const source = list.limitations[7];
    expect(source.endsWith(OMITTED_SENTENCE)).toBe(true);
    for (const changed of [
      [...list.limitations, "An invented limitation the producer never published."],
      list.limitations.map((text, index) => (index === 7 ? text.replace("ordinary structural recovery", "ordinary structural response") : text)),
      list.limitations.map((text, index) => (index === 7 ? text.slice(0, -(OMITTED_SENTENCE.length + 1)) : text))
    ]) {
      expect(changed).not.toEqual(list.limitations);
      await expect(exactReportStatements(identity, changed)).rejects.toThrow(expected);
    }
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
