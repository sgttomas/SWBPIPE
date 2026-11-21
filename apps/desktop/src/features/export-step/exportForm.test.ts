import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { AS_GIVEN, FIXED_ENTRIES, chooseExportForm, type ExportFormat, type PackageRecord } from "./exportForm";

// DEL-07-12: the one rule for an export row's form, and the table of each format's fixed entries beside it.
// Invented records only.
const FEATURES = path.resolve(__dirname, "..");

function record(format: ExportFormat, extra: PackageRecord["entries"] = [], validation: PackageRecord["validation"] = null): PackageRecord {
  return {
    format,
    entries: [...FIXED_ENTRIES[format].map((entry) => ({ kind: entry.kind, term: entry.term, severity: "warning" })), ...extra],
    validation
  };
}

describe("the one rule", () => {
  it("writes on the press where the package holds only the format's placed standing limits", () => {
    const placed = { format: "review-geometry" as const, entries: FIXED_ENTRIES["review-geometry"].map((entry) => ({ kind: entry.kind, term: entry.term })) };
    const form = chooseExportForm(placed, AS_GIVEN);
    expect(form.form).toBe("write-on-press");
    // The row states the format's losses and unsupported entries; its standing findings are in the record.
    expect(form.standing.map((entry) => entry.term)).toEqual(FIXED_ENTRIES["review-geometry"].filter((entry) => entry.kind === "loss").map((entry) => entry.term));
  });

  it("does not state as a limit an entry that says what the file does carry", () => {
    const form = chooseExportForm(record("stress-neutral", [], { status: "passed", passed: true }), AS_GIVEN);
    expect(form.form).toBe("write-on-press");
    expect(form.standing.map((entry) => entry.term)).not.toContain("loss:desktop-preview:stress-neutral-exported");
    expect(form.standing).toHaveLength(2);
  });

  it("opens the record first for a loss, an unsupported entry or a finding this model produced, with them as its causes", () => {
    for (const kind of ["loss", "unsupported", "finding"] as const) {
      const own = { kind, term: `invented:${kind}`, severity: "warning" };
      const form = chooseExportForm(record("local-fea", [own]), AS_GIVEN);
      expect(form.form).toBe("record-first");
      if (form.form === "record-first") {
        expect(form.why).toBe("this-model");
        expect(form.causes).toEqual([own]);
      }
    }
  });

  it("does not open the record first for an informational finding of the model", () => {
    expect(chooseExportForm(record("review-geometry", [{ kind: "finding", term: "REVIEW-GEOMETRY-UNITS-METERED", severity: "info" }]), AS_GIVEN).form).toBe("write-on-press");
  });

  it("opens the record first for a failed validation, which heads the causes", () => {
    const form = chooseExportForm(record("stress-neutral", [], { status: "blocked", passed: false }), AS_GIVEN);
    expect(form.form).toBe("record-first");
    if (form.form === "record-first") expect(form.causes[0]).toMatchObject({ term: "validation", category: "blocked" });
  });

  it("keeps the record first while an unplaced entry is present", () => {
    const form = chooseExportForm(record("pcf", [], { status: "passed", passed: true }), AS_GIVEN);
    expect(form.form).toBe("record-first");
    if (form.form === "record-first") expect(form.causes.map((entry) => entry.term)).toEqual(["PCF-NOMINAL-SIZE-TBD"]);
  });

  it("never opens the record first for the result's content", () => {
    const content = Array.from({ length: 25 }, (_, index) => ({ kind: "content" as const, term: `invented-${index}`, severity: "blocking" }));
    expect(chooseExportForm({ format: "result-export", entries: content, validation: null }, AS_GIVEN).form).toBe("write-on-press");
  });

  it("opens the record first, with no causes but the reason, when the export is withheld, and with the count when values are replaced", () => {
    const withheld = chooseExportForm(record("review-geometry"), { kind: "withheld", reason: "Invented reason." });
    expect(withheld).toMatchObject({ form: "record-first", why: "withheld", reason: "Invented reason.", causes: [] });
    const replaced = chooseExportForm(record("review-geometry"), { kind: "replaced", count: 7 });
    expect(replaced).toMatchObject({ form: "record-first", why: "replaced", replaced: 7 });
  });

  it("is pure: the same record and disposition give the same form", () => {
    const input = record("pcf", [{ kind: "finding", term: "invented", severity: "warning" }], { status: "blocked", passed: false });
    const before = JSON.stringify(input);
    expect(chooseExportForm(input, AS_GIVEN)).toEqual(chooseExportForm(input, AS_GIVEN));
    expect(JSON.stringify(input)).toBe(before);
  });
});

describe("the table of fixed entries", () => {
  it.each(Object.entries(FIXED_ENTRIES).flatMap(([format, entries]) => entries.map((entry) => [format, entry.term, entry.source] as const)))(
    "%s %s: its cited generator line adds it",
    (_format, term, source) => {
      const [file, lines] = source.split(":");
      const first = Number(/\d+/.exec(lines)![0]);
      const text = readFileSync(path.join(FEATURES, file), "utf8").split("\n");
      // The term (or, for a diagnostic, its code) is on the cited line or within the call that starts there.
      expect(text.slice(first - 1, first + 3).join("\n")).toContain(term);
    }
  );
});
