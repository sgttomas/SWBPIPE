// DEL-12-02 value records (HELP_HUMAN decision, 2026-10-28: "take the stricter decision per case"). A
// `{value, ...}` record with metadata but no scalar label was one item in TypeScript and a projected record in
// Python; all three implementations now decide it as the stricter of the two.
// `fixtures/redaction_export_controls/value_record_sweep.json` is expanded here, in
// `tests/security/test_redaction_value_records.py` and in `core/runner/headless/src/redaction_binding.rs`
// (test module) by the same rule. All data is invented.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { controlRouteExport, type RedactionExportContext } from "./redactionExportControls";

type Item = Record<string, unknown>;
type Spec = {
  absent: string;
  max_axes: number;
  axes: [string, unknown[]][];
  extra_records: Item[];
  record_key: string;
  route_id: string;
  contexts: [RedactionExportContext, boolean][];
};
type Frozen = { case_count: number; code_order: string[]; outcomes: Record<string, string[]> };

const fixtureDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../../fixtures/redaction_export_controls");
const spec = JSON.parse(readFileSync(path.join(fixtureDir, "value_record_sweep.json"), "utf8")) as Spec;
const frozen = JSON.parse(readFileSync(path.join(fixtureDir, "value_record_sweep_decisions.json"), "utf8")) as Frozen;
const CASE_COUNT = 7_580;
const CODES = "IWRO";
// The unsafe detail keys (`redactionExportControls.ts` `UNSAFE_DETAIL_KEYS`) the sweep's records carry.
const UNSAFE_MEMBERS = new Set(["file_path"]);

function expand(sweep: Spec): [Item, RedactionExportContext, boolean][] {
  const records: Item[] = [];
  const digits = sweep.axes.map(() => 0);
  for (;;) {
    const chosen = sweep.axes.map(([key, values], index) => [key, values[digits[index]]] as const);
    if (chosen.filter(([, value]) => value !== sweep.absent).length <= sweep.max_axes) {
      const record: Item = { value: "INVENTED" };
      for (const [key, value] of chosen) if (value !== sweep.absent) record[key] = value;
      records.push(record);
    }
    let position = sweep.axes.length;
    let exhausted = true;
    while (position > 0) {
      position -= 1;
      digits[position] += 1;
      if (digits[position] < sweep.axes[position][1].length) {
        exhausted = false;
        break;
      }
      digits[position] = 0;
    }
    if (exhausted) break;
  }
  records.push(...sweep.extra_records);
  return records.flatMap((record) => sweep.contexts.map(([context, intent]) => [record, context, intent] as [Item, RedactionExportContext, boolean]));
}

function routeDecisions(record: Item, exportContext: RedactionExportContext, explicitLocalPrivateIntent: boolean) {
  const controlled = controlRouteExport({ [spec.record_key]: record }, { routeId: spec.route_id, exportContext, explicitLocalPrivateIntent });
  return {
    controlled,
    decided: controlled.decisions.map((decision) => [decision.path.replaceAll("__route_key__", ""), decision.action, decision.reason_code])
  };
}

function outcome(record: Item, decisions: string[][]): string {
  if (decisions.some(([, action]) => action === "block_export")) return "B";
  return Object.keys(record)
    .sort()
    .map((member) => {
      let code: string | null = null;
      for (const [decisionPath, action] of decisions) {
        let candidate: string;
        if (decisionPath === "$.record") {
          if (action === "redact_value" && ["value", "text", "privacy_classification"].includes(member)) candidate = "R";
          else if (action === "redact_value" && UNSAFE_MEMBERS.has(member)) candidate = "O";
          else if (action === "redact_value" || action === "warning_only") candidate = "W";
          else candidate = action === "include" ? "I" : action === "omit_field" ? "O" : "R";
        } else if (
          decisionPath === `$.record.${member}` ||
          decisionPath.startsWith(`$.record.${member}.`) ||
          decisionPath.startsWith(`$.record.${member}[`)
        ) {
          candidate = action === "include" ? "I" : action === "warning_only" ? "W" : action === "omit_field" ? "O" : "R";
        } else continue;
        if (code === null || CODES.indexOf(candidate) > CODES.indexOf(code)) code = candidate;
      }
      return code ?? "N";
    })
    .join("");
}

const column = (name: string) => frozen.outcomes[name].join(",").split(",");

function compare(next: string, old: string): [boolean, boolean] {
  if (old === "-") return [false, false];
  if (old === "B" || next === "B") return [old === "B" && next !== "B", next === "B" && old !== "B"];
  const pairs = [...next]
    .map((code, index) => [CODES.indexOf(code), CODES.indexOf(old[index])])
    .filter(([n, o]) => n >= 0 && o >= 0);
  return [pairs.some(([n, o]) => n < o), pairs.some(([n, o]) => n > o)];
}

describe("DEL-12-02 value records", () => {
  it("gives the contract outcome for every sweep case and is never less strict than before", () => {
    const cases = expand(spec);
    expect(cases).toHaveLength(CASE_COUNT);
    expect(frozen.case_count).toBe(CASE_COUNT);
    expect(frozen.code_order).toEqual([...CODES]);
    // The sweep's unsafe members are exactly the unsafe detail keys its axes set.
    expect(spec.axes.map(([key]) => key).filter((key) => UNSAFE_MEMBERS.has(key))).toEqual(["file_path"]);
    const contract = column("contract");
    const names = ["python_before", "typescript_before", "rust_before"];
    const before = Object.fromEntries(names.map((name) => [name, column(name)]));
    const mismatches: number[] = [];
    const looser = Object.fromEntries(names.map((name) => [name, 0]));
    const stricter = Object.fromEntries(names.map((name) => [name, 0]));
    cases.forEach(([record, context, intent], index) => {
      const decided = outcome(record, routeDecisions(record, context, intent).decided);
      if (decided !== contract[index]) mismatches.push(index);
      for (const name of names) {
        const [less, more] = compare(decided, before[name][index]);
        looser[name] += Number(less);
        stricter[name] += Number(more);
      }
    });
    expect(mismatches).toEqual([]);
    expect(looser).toEqual({ python_before: 0, typescript_before: 0, rust_before: 0 });
    // Pinned: stricter than Python in 1,638 cases and than TypeScript in 1,461; the Rust mirror had no
    // route projector before.
    expect(stricter).toEqual({ python_before: 1_638, typescript_before: 1_461, rust_before: 0 });
  }, 120_000);

  const sortedDecisions = (record: Item, context: RedactionExportContext, intent: boolean) => {
    const { controlled, decided } = routeDecisions(record, context, intent);
    return { controlled, decided: decided.sort((a, b) => a.join("\u0000").localeCompare(b.join("\u0000"))) };
  };

  it("projects {value, field_class, note}, which TypeScript decided as one item", () => {
    // Python stricter: TypeScript left `note` visible in public_report.
    const { controlled, decided } = sortedDecisions({ value: "INVENTED", field_class: "note", note: "INVENTED" }, "public_report", false);
    expect(decided).toEqual([
      ["$.record.field_class", "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"],
      ["$.record.note", "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"],
      ["$.record.value", "redact_value", "PRIVATE_DATA_REDACTED"]
    ]);
    expect(controlled.payload).toEqual({ record: { value: "[REDACTED]", field_class: "[REDACTED]", note: "[REDACTED]" } });
  });

  it("blocks {value, professional_claim: true}, which Python redacted", () => {
    const { controlled, decided } = sortedDecisions({ value: "INVENTED", professional_claim: true }, "public_report", false);
    expect(decided).toEqual([["$.record", "block_export", "PROFESSIONAL_BOUNDARY_BLOCKED"]]);
    expect([controlled.blocked, controlled.payload]).toEqual([true, null]);
  });

  it("takes the item's whole-record block where both blocked on different paths (ruling 3(a))", () => {
    expect(sortedDecisions({ value: "INVENTED", contains_payload: true }, "local_private", false).decided).toEqual([
      ["$.record", "block_export", "PAYLOAD_METADATA_ONLY_REQUIRED"]
    ]);
  });

  it("keeps the projected value leaf's need for local-private intent", () => {
    expect(sortedDecisions({ value: "INVENTED", export_policy: {} }, "local_private", false).decided).toEqual([
      ["$.record.value", "block_export", "LOCAL_PRIVATE_INTENT_REQUIRED"]
    ]);
  });

  it("raises the value leaf to the item's reason and drops the unsafe path key (the union, 1(a) and 2(a))", () => {
    const { controlled, decided } = sortedDecisions({ value: "INVENTED", file_path: "INVENTED", export_policy: {} }, "local_private", true);
    expect(decided).toEqual([
      ["$.record.file_path", "omit_field", "CONCRETE_PATH_REDACTED"],
      ["$.record.value", "redact_value", "CONCRETE_PATH_REDACTED"]
    ]);
    expect(controlled.payload).toEqual({ record: { value: "[REDACTED]", export_policy: {} } });
  });

  it("takes the projection's decision records for a tie", () => {
    const { controlled, decided } = sortedDecisions({ value: "INVENTED", export_policy: {} }, "local_private", true);
    expect(decided).toEqual([["$.record.value", "warning_only", "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"]]);
    expect(controlled.payload).toEqual({ record: { value: "INVENTED", export_policy: {} } });
  });
});
