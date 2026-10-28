// RT-R6 serializer tests (DEL-08-04 AC-002; Design `csv-export.md` §3, §5 slice 2).
import { describe, expect, it } from "vitest";
import {
  CSV_BOM,
  CSV_PREAMBLE_KEYS,
  CSV_PREAMBLE_NOTICE,
  CsvSerializationError,
  csvExportFileName,
  csvNumber,
  csvStatus,
  csvText,
  formatCsvNumber,
  isCsvExportFileName,
  isCsvKind,
  neutralizeCsvText,
  serializeCsvTable,
  slugToken,
  type CsvCell,
  type CsvTable
} from "./csvTable";

const BS_VALID =
  "Candidate designs are validated in the user's accepted professional tools (external-prover correlation, PRD §22.5). Internal benchmarks and rule checks are development verification and screening evidence.";

const preamble = {
  table: "stress-ratio",
  softwareVersion: "0.9.0+abc1234",
  modelHash: "sha256:1111",
  resultHash: "sha256:2222",
  analysisRoutes: ["linear-static"],
  caseOrFilter: "SUS",
  unitBasis: "SI display (mm, N, MPa)"
};

function table(overrides: Partial<CsvTable> = {}): CsvTable {
  return {
    preamble,
    columns: [
      { kind: "text", label: "Node" },
      { kind: "number", label: "UX", unit: "mm" },
      { kind: "text", label: "Status" }
    ],
    rows: [
      [csvText("10"), csvNumber(1.25), csvText("Mechanics solved")],
      [csvText("20"), csvStatus("No result"), csvText("withheld")]
    ],
    ...overrides
  };
}

/** A minimal RFC 4180 reader for the tests: returns rows of fields (after the BOM). */
function parse(text: string): string[][] {
  expect(text.startsWith(CSV_BOM)).toBe(true);
  const body = text.slice(1);
  const rows: string[][] = [];
  let row: string[] = [];
  let field = "";
  let quoted = false;
  for (let i = 0; i < body.length; i += 1) {
    const c = body[i];
    if (quoted) {
      if (c === '"' && body[i + 1] === '"') { field += '"'; i += 1; }
      else if (c === '"') quoted = false;
      else field += c;
    } else if (c === '"' && field === "") quoted = true;
    else if (c === ",") { row.push(field); field = ""; }
    else if (c === "\r" && body[i + 1] === "\n") { row.push(field); rows.push(row); row = []; field = ""; i += 1; }
    else field += c;
  }
  expect(field).toBe("");
  expect(row).toEqual([]);
  return rows;
}

describe("CSV preamble and layout", () => {
  it("writes the confirmed preamble rows in order, one empty row, then the header", () => {
    const rows = parse(serializeCsvTable(table({
      preamble: { ...preamble, analysisRoutes: ["linear-static", "nonlinear-supports"], rulePacks: [
        { id: "pack-a", version: "1.2.0", checksum: "sha256:aaaa" },
        { id: "pack-b", version: "3", checksum: "sha256:bbbb" }
      ] }
    })));
    expect(rows.slice(0, 13)).toEqual([
      ["notice", BS_VALID],
      ["export_context", "local_private"],
      ["released_report", "false"],
      ["table", "stress-ratio"],
      ["software_version", "0.9.0+abc1234"],
      ["model_hash", "sha256:1111"],
      ["result_hash", "sha256:2222"],
      ["analysis_route", "linear-static nonlinear-supports"],
      ["case_or_filter", "SUS"],
      ["unit_basis", "SI display (mm, N, MPa)"],
      ["rule_pack", "pack-a 1.2.0 sha256:aaaa"],
      ["rule_pack", "pack-b 3 sha256:bbbb"],
      [""]
    ]);
    expect(rows[13]).toEqual(["Node", "UX [mm]", "Status"]);
    expect(rows.slice(14)).toEqual([["10", "1.25", "Mechanics solved"], ["20", "No result", "withheld"]]);
    expect(rows.map((row) => row[0]).slice(0, 12)).toEqual([...CSV_PREAMBLE_KEYS.slice(0, 10), "rule_pack", "rule_pack"]);
  });

  it("writes no rule_pack row for a table without rule outcomes, and the notice is BS-VALID verbatim", () => {
    const text = serializeCsvTable(table());
    expect(CSV_PREAMBLE_NOTICE).toBe(BS_VALID);
    expect(text).not.toContain("rule_pack");
    const rows = parse(text);
    expect(rows[10]).toEqual([""]);
    expect(rows[11]).toEqual(["Node", "UX [mm]", "Status"]);
    // The notice contains a comma, so it is one quoted field.
    expect(text.split("\r\n")[0]).toBe(`${CSV_BOM}notice,"${BS_VALID}"`);
  });

  it("starts with the UTF-8 byte-order mark and ends every row, including the last, with CRLF", () => {
    const text = serializeCsvTable(table());
    expect(text.charCodeAt(0)).toBe(0xfeff);
    expect(Array.from(new TextEncoder().encode(text).slice(0, 3))).toEqual([0xef, 0xbb, 0xbf]);
    expect(text.endsWith("\r\n")).toBe(true);
    expect(text.replace(/\r\n/g, "")).not.toMatch(/[\r\n]/);
    expect(text.split("\r\n").length - 1).toBe(14);
  });

  it("quotes per RFC 4180: comma, double quote, CR and LF; embedded quotes doubled", () => {
    const text = serializeCsvTable(table({
      columns: [{ kind: "text", label: "Label, \"quoted\"" }],
      rows: [[csvText("a,b")], [csvText("say \"hi\"")], [csvText("line\nbreak")], [csvText("mid\rcr")], [csvText("plain")]]
    }));
    const lines = text.split("\r\n");
    expect(lines.slice(11, 17)).toEqual(['"Label, ""quoted"""', '"a,b"', '"say ""hi"""', '"line\nbreak"', '"mid\rcr"', "plain"]);
    expect(parse(text).slice(11)).toEqual([["Label, \"quoted\""], ["a,b"], ["say \"hi\""], ["line\nbreak"], ["mid\rcr"], ["plain"]]);
  });

  it("keeps stable identifiers beside labels and rows in the given order", () => {
    const rows = parse(serializeCsvTable(table({
      columns: [{ kind: "text", label: "Result id" }, { kind: "text", label: "Node" }, { kind: "text", label: "Label" }, { kind: "number", label: "FX", unit: "N" }],
      rows: [
        [csvText("r-3"), csvText("30"), csvText("Anchor B"), csvNumber(3)],
        [csvText("r-1"), csvText("10"), csvText("Anchor A"), csvNumber(1)],
        [csvText("r-2"), csvText("20"), csvText(""), csvNumber(2)]
      ]
    })));
    expect(rows.slice(12)).toEqual([["r-3", "30", "Anchor B", "3"], ["r-1", "10", "Anchor A", "1"], ["r-2", "20", "", "2"]]);
  });
});

describe("CSV units", () => {
  it("states a fixed unit in the header, or a per-row unit column for mixed kinds", () => {
    const rows = parse(serializeCsvTable(table({
      columns: [
        { kind: "text", label: "Node" },
        { kind: "number", label: "UX", unit: "mm" },
        { kind: "number", label: "Value", unitColumn: "Unit" },
        { kind: "unit", label: "Unit" }
      ],
      rows: [
        [csvText("10"), csvNumber(0.5), csvNumber(12.5), csvText("MPa")],
        [csvText("10"), csvNumber(0.5), csvNumber(3), csvText("kN")]
      ]
    })));
    expect(rows[11]).toEqual(["Node", "UX [mm]", "Value", "Unit"]);
    expect(rows.slice(12)).toEqual([["10", "0.5", "12.5", "MPa"], ["10", "0.5", "3", "kN"]]);
  });

  it("refuses an empty per-row unit, and admits a status saying why the unit is missing", () => {
    const columns = [{ kind: "number", label: "Value", unitColumn: "Unit" }, { kind: "unit", label: "Unit" }] as const;
    for (const unit of [csvText(""), csvText("  ")]) {
      expect(() => serializeCsvTable(table({ columns: [...columns], rows: [[csvNumber(3), unit]] }))).toThrowError(/CSV_UNIT_EMPTY/);
    }
    const rows = parse(serializeCsvTable(table({ columns: [...columns], rows: [[csvNumber(3), csvStatus("unit not stated")]] })));
    expect(rows[12]).toEqual(["3", "unit not stated"]);
  });

  it("refuses a numeric column with no unit, both units, or a unit column that is not one", () => {
    for (const column of [
      { kind: "number", label: "UX" },
      { kind: "number", label: "UX", unit: "mm", unitColumn: "Unit" },
      { kind: "number", label: "UX", unitColumn: "Node" },
      { kind: "number", label: "UX", unitColumn: "Missing" },
      { kind: "number", label: "UX", unit: "" }
    ]) {
      expect(() => serializeCsvTable(table({
        columns: [{ kind: "text", label: "Node" }, column as never, { kind: "unit", label: "Unit" }],
        rows: [[csvText("10"), csvNumber(1), csvText("mm")]]
      }))).toThrowError(CsvSerializationError);
    }
  });
});

describe("CSV numbers", () => {
  const cases: [number, string][] = [
    [0, "0"], [-0, "-0"], [1, "1"], [-1, "-1"], [0.1, "0.1"], [0.1 + 0.2, "0.30000000000000004"], [1 / 3, "0.3333333333333333"],
    [123456.789, "123456.789"], [-2.5e-7, "-2.5e-7"], [1e-7, "1e-7"], [0.000001, "0.000001"], [1e21, "1e+21"], [123e20, "1.23e+22"],
    [Number.MAX_VALUE, "1.7976931348623157e+308"], [Number.MIN_VALUE, "5e-324"], [-Number.MIN_VALUE, "-5e-324"], [2 ** 53 + 2, "9007199254740994"]
  ];
  it("writes the shortest decimal that parses back exactly; exponent forms are kept; -0 stays -0", () => {
    for (const [value, written] of cases) {
      expect(formatCsvNumber(value), String(value)).toBe(written);
      expect(Object.is(Number(written), value), written).toBe(true);
    }
  });

  it("round-trips numbers through the file unrounded and unconverted", () => {
    const values = cases.map(([value]) => value);
    const text = serializeCsvTable(table({
      columns: [{ kind: "number", label: "V", unit: "mm" }],
      rows: values.map((value) => [csvNumber(value)])
    }));
    const parsed = parse(text).slice(12).map(([field]) => Number(field));
    parsed.forEach((value, index) => expect(Object.is(value, values[index])).toBe(true));
  });

  it("refuses NaN and ±Infinity: a non-finite value arrives as a status cell", () => {
    for (const value of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) {
      expect(() => formatCsvNumber(value)).toThrowError(/CSV_NUMBER_NOT_FINITE/);
      expect(() => serializeCsvTable(table({ rows: [[csvText("10"), csvNumber(value), csvText("x")]] }))).toThrowError(/CSV_NUMBER_NOT_FINITE/);
    }
    const rows = parse(serializeCsvTable(table({ rows: [[csvText("10"), csvStatus("Mechanics not solved"), csvText("x")]] })));
    expect(rows[12]).toEqual(["10", "Mechanics not solved", "x"]);
  });

  it("never writes a number as text or text in a numeric column", () => {
    expect(() => serializeCsvTable(table({ rows: [[csvText("10"), csvText("1.5"), csvText("x")]] }))).toThrowError(/CSV_CELL_KIND_INVALID/);
    expect(() => serializeCsvTable(table({ rows: [[csvNumber(10), csvNumber(1.5), csvText("x")]] }))).toThrowError(/CSV_CELL_KIND_INVALID/);
    expect(() => serializeCsvTable(table({ rows: [[csvText("10"), { kind: "number", value: "1" } as never, csvText("x")]] }))).toThrowError(/CSV_NUMBER_NOT_FINITE/);
  });
});

describe("CSV absent values", () => {
  it("never emits an empty field in a numeric column", () => {
    const absent: unknown[] = [csvStatus(""), csvStatus("   "), null, undefined, {}, { kind: "status" }, csvText("")];
    for (const cell of absent) {
      expect(() => serializeCsvTable(table({ rows: [[csvText("10"), cell as CsvCell, csvText("x")]] })), JSON.stringify(cell)).toThrowError(CsvSerializationError);
    }
    // A sparse row (an array hole) is a missing cell, not an empty field.
    const sparse = new Array<CsvCell>(3);
    sparse[0] = csvText("10");
    sparse[2] = csvText("x");
    expect(() => serializeCsvTable(table({ rows: [sparse] }))).toThrowError(/CSV_CELL_KIND_INVALID/);
    // A row with too few cells cannot leave a numeric field empty either.
    expect(() => serializeCsvTable(table({ rows: [[csvText("10")]] }))).toThrowError(/CSV_ROW_WIDTH/);
    const text = serializeCsvTable(table({
      rows: [[csvText("10"), csvStatus("withheld"), csvText("")], [csvText("20"), csvStatus("No result"), csvText("")]]
    }));
    for (const row of parse(text).slice(12)) expect(row[1]).not.toBe("");
  });

  it("refuses missing preamble metadata rather than filling it", () => {
    for (const change of [{ softwareVersion: "" }, { modelHash: " " }, { resultHash: undefined }, { analysisRoutes: [] }, { analysisRoutes: [""] },
      { analysisRoutes: ["two words"] }, { caseOrFilter: "" }, { unitBasis: "" }, { table: "" }, { table: "model" }, { table: "results" },
      { rulePacks: [{ id: "p", version: "", checksum: "c" }] }, { rulePacks: [{ id: "p q", version: "1", checksum: "c" }] }]) {
      expect(() => serializeCsvTable(table({ preamble: { ...preamble, ...change } as never })), JSON.stringify(change)).toThrowError(CsvSerializationError);
    }
  });
});

describe("CSV formula guard", () => {
  it("prefixes ' to text beginning with =, +, -, @, TAB, CR, LF or a full-width ＝ ＋ － ＠, in cells, headers and preamble values", () => {
    for (const lead of ["=", "+", "-", "@", "\t", "\r", "\n", "\uFF1D", "\uFF0B", "\uFF0D", "\uFF20"]) {
      const label = `${lead}SUM(A1:A2)`;
      expect(neutralizeCsvText(label)).toBe(`'${label}`);
      const rows = parse(serializeCsvTable(table({
        preamble: { ...preamble, caseOrFilter: label },
        columns: [{ kind: "text", label }, { kind: "number", label: `${lead}UX`, unit: "mm" }],
        rows: [[csvText(label), csvStatus(label)]]
      })));
      expect(rows[8]).toEqual(["case_or_filter", `'${label}`]);
      expect(rows[11]).toEqual([`'${label}`, `'${lead}UX [mm]`]);
      expect(rows[12]).toEqual([`'${label}`, `'${label}`]);
    }
    for (const safe of ["SUM", " =x", "'=x", "", "1+1", "a-b"]) expect(neutralizeCsvText(safe)).toBe(safe);
  });

  it("leaves numbers untouched, so a negative number stays a number", () => {
    const text = serializeCsvTable(table({ rows: [[csvText("-10"), csvNumber(-12.5), csvText("x")], [csvText("20"), csvNumber(-0), csvText("x")]] }));
    const rows = parse(text);
    expect(rows[12]).toEqual(["'-10", "-12.5", "x"]);
    expect(rows[13]).toEqual(["20", "-0", "x"]);
    expect(text).toContain("\r\n'-10,-12.5,x\r\n");
  });
});

describe("CSV determinism", () => {
  it("gives byte-identical output for identical input", () => {
    const build = () => table({
      preamble: { ...preamble, rulePacks: [{ id: "pack-a", version: "1", checksum: "sha256:aa" }] },
      rows: [[csvText("é ☃ 10"), csvNumber(0.1 + 0.2), csvText("=x")], [csvText("20"), csvStatus("withheld"), csvText("ok")]]
    });
    const first = new TextEncoder().encode(serializeCsvTable(build()));
    const second = new TextEncoder().encode(serializeCsvTable(build()));
    expect(Buffer.from(first).equals(Buffer.from(second))).toBe(true);
    expect(Buffer.from(first).toString("hex")).toBe(Buffer.from(new TextEncoder().encode(serializeCsvTable(build()))).toString("hex"));
  });

  it("does not depend on locale or the clock", () => {
    const input = table({ rows: [[csvText("10"), csvNumber(1234567.891), csvText("x")]] });
    const before = serializeCsvTable(input);
    const toLocale = Number.prototype.toLocaleString;
    const now = Date.now;
    try {
      Number.prototype.toLocaleString = () => "1.234.567,891";
      Date.now = () => 0;
      expect(serializeCsvTable(input)).toBe(before);
    } finally {
      Number.prototype.toLocaleString = toLocale;
      Date.now = now;
    }
    expect(before).toContain("\r\n10,1234567.891,x\r\n");
  });

  it("refuses text that cannot be encoded as UTF-8", () => {
    expect(() => serializeCsvTable(table({ rows: [[csvText("\uD800"), csvNumber(1), csvText("x")]] }))).toThrowError(/CSV_TEXT_NOT_UTF8/);
    expect(serializeCsvTable(table({ rows: [[csvText("😀"), csvNumber(1), csvText("x")]] }))).toContain("😀");
  });
});

describe("CSV file names (DEL-07-01 export-file-names.md)", () => {
  it("builds and checks swbpipe-<csv kind>[-<qualifier>].csv", () => {
    expect(csvExportFileName("stress-ratio")).toBe("swbpipe-stress-ratio.csv");
    expect(csvExportFileName("stress-ratio", "SUS Case 1")).toBe("swbpipe-stress-ratio-sus-case-1.csv");
    expect(csvExportFileName("results-displacements", "run 3")).toBe("swbpipe-results-displacements-run-3.csv");
    expect(csvExportFileName("support-loads-all")).toBe("swbpipe-support-loads-all.csv");
    expect(() => csvExportFileName("stress-ratio", "☃")).toThrowError(/CSV_QUALIFIER_EMPTY/);
    for (const kind of ["model", "results", "handoff", "Stress-ratio", "stress--ratio", "report"]) {
      expect(isCsvKind(kind), kind).toBe(false);
      expect(() => csvExportFileName(kind)).toThrowError(/CSV_KIND_INVALID/);
    }
    expect(slugToken("  Anchor A / Node 10 ")).toBe("anchor-a-node-10");
    for (const name of ["swbpipe-stress-ratio.csv", "swbpipe-weight-cog.csv", "swbpipe-run-comparison-run-3-run-4.csv", "swbpipe-results-loads-sus.csv"]) {
      expect(isCsvExportFileName(name), name).toBe(true);
    }
    for (const name of ["swbpipe-results.csv", "swbpipe-stress-ratio.json", "swbpipe-stress-ratio (1).csv", "../swbpipe-stress-ratio.csv", "swbpipe-stress-ratio-.csv", "openpipestress-preview-results-run-1.csv"]) {
      expect(isCsvExportFileName(name), name).toBe(false);
    }
  });
});
