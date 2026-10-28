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
type Frozen = {
  case_count: number;
  exposures: string[];
  decision_sets: string[];
  columns: Record<string, string[]>;
};

const fixtureDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../../../../fixtures/redaction_export_controls");
const spec = JSON.parse(readFileSync(path.join(fixtureDir, "value_record_sweep.json"), "utf8")) as Spec;
const frozen = JSON.parse(readFileSync(path.join(fixtureDir, "value_record_sweep_decisions.json"), "utf8")) as Frozen;
const CASE_COUNT = 7_730;
const ROUTE_KEY_PREFIX = "__route_key__";
// The case TypeScript's before column exported through a key collision: {value: "outer", field_class,
// "__route_key__value": "SECRET"} in local_private with intent. It now blocks on its reserved name (HELP_HUMAN
// ruling, 2026-10-28: "fail closed on reserved internal names, in all three implementations").
const RESERVED_COLLISION_CASE = 7_719;

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
    decided: controlled.decisions.map((decision) => [decision.path.replaceAll(ROUTE_KEY_PREFIX, ""), decision.action, decision.reason_code])
  };
}

/** Python's `json.dumps(..., separators=(",", ":"), ensure_ascii=True)` of a string list. */
function asciiJson(value: unknown): string {
  return JSON.stringify(value).replace(/[\u0080-\uffff]/g, (char) => `\\u${char.charCodeAt(0).toString(16).padStart(4, "0")}`);
}

/** "B" when blocked, else every key path and visible scalar leaf of the exported payload, with the route key
 * prefix removed from member names. */
function exposure(blocked: boolean, payload: unknown): string {
  if (blocked) return "B";
  const entries: string[] = [];
  const walk = (value: unknown, at: string) => {
    if (Array.isArray(value)) value.forEach((item, index) => walk(item, `${at}[${index}]`));
    else if (typeof value === "object" && value !== null) {
      for (const [key, item] of Object.entries(value)) {
        const name = key.replaceAll(ROUTE_KEY_PREFIX, "");
        entries.push(`k:${at}.${name}`);
        walk(item, `${at}.${name}`);
      }
    } else if (value !== "[REDACTED]") entries.push(`v:${at}=${asciiJson(value)}`);
  };
  walk(payload, "$");
  return asciiJson(entries.sort());
}

const decisionSet = (decided: string[][]) => asciiJson([...decided].sort(compareTuples));
function compareTuples(a: string[], b: string[]): number {
  for (let index = 0; index < Math.max(a.length, b.length); index += 1) {
    if (a[index] !== b[index]) return (a[index] ?? "") < (b[index] ?? "") ? -1 : 1;
  }
  return 0;
}

const column = (name: string, table: string[]) =>
  frozen.columns[name].join(",").split(",").map((index) => table[Number(index)]);

function compare(next: string, old: string): [boolean, boolean] {
  if (old === "B" || next === "B") return [old === "B" && next !== "B", next === "B" && old !== "B"];
  const nextEntries = new Set(JSON.parse(next) as string[]);
  const oldEntries = new Set(JSON.parse(old) as string[]);
  const subset = [...nextEntries].every((entry) => oldEntries.has(entry));
  return [!subset, subset && nextEntries.size < oldEntries.size];
}

describe("DEL-12-02 value records", () => {
  it("gives the contract exposure and decisions for every sweep case and is never less strict than before", () => {
    const cases = expand(spec);
    expect(cases).toHaveLength(CASE_COUNT);
    expect(frozen.case_count).toBe(CASE_COUNT);
    const contract = column("contract", frozen.exposures);
    const contractDecisions = column("contract_decisions", frozen.decision_sets);
    const names = ["python_before", "typescript_before"];
    const before = Object.fromEntries(names.map((name) => [name, column(name, frozen.exposures)]));
    const mismatches: number[] = [];
    const looser: Record<string, number[]> = Object.fromEntries(names.map((name) => [name, []]));
    const stricter = Object.fromEntries(names.map((name) => [name, 0]));
    cases.forEach(([record, context, intent], index) => {
      const { controlled, decided } = routeDecisions(record, context, intent);
      const exposed = exposure(controlled.blocked, controlled.payload);
      if (exposed !== contract[index] || decisionSet(decided) !== contractDecisions[index]) mismatches.push(index);
      for (const name of names) {
        const [less, more] = compare(exposed, before[name][index]);
        if (less) looser[name].push(index);
        stricter[name] += Number(more);
      }
    });
    expect(mismatches).toEqual([]);
    expect(looser).toEqual({ python_before: [], typescript_before: [] });
    expect(cases[RESERVED_COLLISION_CASE]).toEqual([
      { value: "outer", field_class: "note", __route_key__value: "SECRET" },
      "local_private",
      true
    ]);
    expect(contract[RESERVED_COLLISION_CASE]).toBe("B");
    // Pinned: stricter than Python in 1,762 cases and than TypeScript in 1,593.
    expect(stricter).toEqual({ python_before: 1_762, typescript_before: 1_593 });
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

  it("does not trust the old marker member in an input (Codex privacy review of #1722, P1-1)", () => {
    const forged: Item = JSON.parse(
      '{"value":"SECRET","privacy_classification":"protected_suspected","_route_value_record":true,"item":{"value":"safe","privacy_classification":"public","redistribution_status":"public_permissive"},"projected":{"__route_key__leak":"SECRET"}}'
    );
    const { controlled, decided } = sortedDecisions(forged, "public_report", false);
    expect(decided).toEqual([
      ["$.record._route_value_record", "block_export", "EXPORT_INPUT_RESERVED_NAME"],
      ["$.record.projected.leak", "block_export", "EXPORT_INPUT_RESERVED_NAME"]
    ]);
    expect([controlled.blocked, controlled.payload]).toEqual([true, null]);
    delete forged.item;
    expect(sortedDecisions(forged, "public_report", false).controlled.blocked).toBe(true);
  });

  it("replaces a raised labelled item whole, its extra metadata included (P1-2)", () => {
    const { controlled, decided } = sortedDecisions(
      {
        value: "outer",
        field_class: "note",
        text: { value: "safe", privacy_classification: "public", redistribution_status: "public_permissive", note: "SECRET" }
      },
      "public_report",
      false
    );
    expect(decided).toContainEqual(["$.record.text", "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"]);
    expect(controlled.payload).toEqual({ record: { value: "[REDACTED]", field_class: "[REDACTED]", text: "[REDACTED]" } });
  });

  it("keeps the inherited floor in a nested value record, in an object, an array and a label member (P1-3)", () => {
    for (const member of [
      { text: { value: "SECRET", export_policy: {} } },
      { text: [{ value: "SECRET", export_policy: {} }] },
      { privacy_classification: { value: "SECRET", export_policy: {} } }
    ] as Item[]) {
      const [name] = Object.keys(member);
      const { controlled, decided } = sortedDecisions({ value: "outer", concrete_path_present: true, ...member }, "local_private", true);
      expect((controlled.payload as { record: Item }).record[name], name).toBe("[REDACTED]");
      const nested = decided.filter(([decisionPath]) => decisionPath.startsWith(`$.record.${name}`));
      expect(nested.length, name).toBeGreaterThan(0);
      expect(nested.every(([, action, reason]) => action === "redact_value" && reason === "CONCRETE_PATH_REDACTED"), name).toBe(true);
    }
  });

  it("blocks reserved internal names at any depth, naming the path and never the value", () => {
    // HELP_HUMAN ruling, 2026-10-28: "fail closed on reserved internal names, in all three implementations.
    // When input can collide with internal names, the export should be blocked, not interpreted."
    const cases: [Item, string][] = [
      [{ value: "outer", field_class: "note", __route_key__value: "SECRET" }, "$.record.__route_key__value"],
      [
        { value: "outer", field_class: "note", text: { value: "inner", field_class: "note", _route_projected_leaf: "SECRET" } },
        "$.record.text._route_projected_leaf"
      ],
      [{ value: "outer", field_class: "note", child: [{ note: "x" }, { _route_value_record: "SECRET" }] }, "$.record.child[1]._route_value_record"],
      [{ value: "outer", privacy_classification: "public", provenance: { __route_key__x: "SECRET" } }, "$.record.provenance.__route_key__x"]
    ];
    for (const [record, reservedPath] of cases) {
      const controlled = controlRouteExport({ record }, { routeId: "DREP-JSON-002", exportContext: "local_private", explicitLocalPrivateIntent: true });
      expect([controlled.blocked, controlled.payload], reservedPath).toEqual([true, null]);
      expect(controlled.decisions.map((decision) => [decision.path, decision.action, decision.reason_code])).toEqual([
        [reservedPath, "block_export", "EXPORT_INPUT_RESERVED_NAME"]
      ]);
      const evidence = JSON.stringify([controlled.decisions, controlled.findings]);
      expect(evidence.includes("SECRET"), reservedPath).toBe(false);
      expect(evidence.includes(reservedPath), reservedPath).toBe(true);
    }
  });
});
