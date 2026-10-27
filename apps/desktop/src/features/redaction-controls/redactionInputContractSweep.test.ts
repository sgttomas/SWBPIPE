// DEL-12-02 input-contract sweep: the classifier never decides a case less strictly than the
// pre-contract code did, and the three implementations agree case for case.
// `fixtures/redaction_export_controls/input_contract_sweep.json` is expanded here, in
// `tests/security/test_redaction_input_contract_sweep.py` and in
// `core/runner/headless/src/redaction_binding.rs` (test module) by the same rule. All data is invented.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { classifyExportItem, controlRouteExport, type ControlExportContext } from "./redactionExportControls";

type Item = Record<string, unknown>;
type SweepCase = {
  family: "labels" | "flags" | "claims";
  item: Item;
  context: ControlExportContext;
  localIntent: boolean;
  releaseIntent: boolean;
};
type SweepSpec = {
  absent: string;
  contexts: [ControlExportContext, boolean, boolean][];
  labels: { privacy: unknown[]; redistribution: unknown[]; review: unknown[]; placements: string[] };
  flags: { keys: string[]; values: unknown[] };
  claims: { values: unknown[] };
  bases: Item[];
};
type FrozenDecisions = {
  case_count: number;
  action_order: string[];
  alphabet: string;
  codes: string[];
  decisions: Record<string, string[]>;
};

const fixtureDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../../../../../fixtures/redaction_export_controls"
);
const spec = JSON.parse(readFileSync(path.join(fixtureDir, "input_contract_sweep.json"), "utf8")) as SweepSpec;
const frozen = JSON.parse(
  readFileSync(path.join(fixtureDir, "input_contract_sweep_decisions.json"), "utf8")
) as FrozenDecisions;
const CASE_COUNT = 74_760;

/** Items in family order labels, flags, claims; each decided in the spec's twelve contexts in order.
 * A value equal to `spec.absent` leaves its member out. */
function expandSweep(sweep: SweepSpec): SweepCase[] {
  const put = (target: Item, key: string, value: unknown) => {
    if (value !== sweep.absent) target[key] = value;
  };
  const items: [SweepCase["family"], Item][] = [];
  for (const privacy of sweep.labels.privacy) {
    for (const redistribution of sweep.labels.redistribution) {
      for (const review of sweep.labels.review) {
        for (const placement of sweep.labels.placements) {
          const item: Item = { value: "x" };
          if (placement === "item") {
            put(item, "privacy_classification", privacy);
            put(item, "redistribution_status", redistribution);
            put(item, "review_status", review);
          } else if (placement === "provenance") {
            const provenance: Item = {};
            put(provenance, "privacy_classification", privacy);
            put(provenance, "redistribution_status", redistribution);
            put(provenance, "review_status", review);
            item.provenance = provenance;
          } else {
            // split: the privacy and review on the item, the redistribution on a public provenance
            // whose review is quarantined only with the item's.
            put(item, "privacy_classification", privacy);
            const provenance: Item = {
              privacy_classification: "public",
              redistribution_status: "public_permissive",
              review_status: review === "quarantined" ? "quarantined" : "accepted"
            };
            put(provenance, "redistribution_status", redistribution);
            item.provenance = provenance;
            put(item, "review_status", review);
          }
          items.push(["labels", item]);
        }
      }
    }
  }
  for (const key of sweep.flags.keys) {
    for (const value of sweep.flags.values) {
      for (const base of sweep.bases) {
        const item: Item = { value: "x", ...base };
        if (key.startsWith("export_policy.")) item.export_policy = { [key.slice("export_policy.".length)]: value };
        else item[key] = value;
        items.push(["flags", item]);
      }
    }
  }
  for (const value of sweep.claims.values) {
    for (const base of sweep.bases) {
      const item: Item = { value: "x", ...base };
      put(item, "professional_claim", value);
      items.push(["claims", item]);
    }
  }
  return items.flatMap(([family, item]) =>
    sweep.contexts.map(([context, localIntent, releaseIntent]) => ({ family, item, context, localIntent, releaseIntent }))
  );
}

function decide(sweepCase: SweepCase): string {
  const decision = classifyExportItem(sweepCase.item, {
    exportContext: sweepCase.context,
    explicitLocalPrivateIntent: sweepCase.localIntent,
    explicitUserReleaseIntent: sweepCase.releaseIntent
  });
  return `${decision.action}/${decision.reason_code}`;
}

function decodedColumn(name: string): string[] {
  return [...frozen.decisions[name].join("")].map((symbol) => frozen.codes[frozen.alphabet.indexOf(symbol)]);
}

/** "Less strict" is this total order on the classifier's actions, least to most strict. `redact_value`
 * sits above `warning_only` by the existing code: `warning_only` returns the item with its value
 * (`applyAction`) and is the unknown-metadata action only in `local_private`, while `redact_value`
 * replaces `value` and `text` with `[REDACTED]`, drops the unsafe detail keys, and is that action in
 * every public or shared context. The classifier produces no `redact_field` or `omit_field`. */
const ACTION_ORDER = ["include", "warning_only", "redact_value", "block_export"];
const BEFORE_COLUMNS = ["python_before", "typescript_before", "rust_before"];
const RAISED = "raised/TypeError";

function strictness(code: string): number {
  const rank = ACTION_ORDER.indexOf(code.split("/")[0]);
  if (rank < 0) throw new Error(`unordered action in ${code}`);
  return rank;
}

const cases = expandSweep(spec);

describe("DEL-12-02 input-contract sweep", () => {
  it("expands to the pinned case count", () => {
    expect(cases).toHaveLength(CASE_COUNT);
    expect(frozen.case_count).toBe(CASE_COUNT);
    expect(frozen.action_order).toEqual(ACTION_ORDER);
    for (const name of [...BEFORE_COLUMNS, "contract"]) expect(decodedColumn(name)).toHaveLength(CASE_COUNT);
  });

  it("decides no case less strictly than any implementation's frozen pre-contract decision", () => {
    // Python's former TypeError on an array or object label is no decision; the case is held to
    // the TypeScript and Rust columns.
    const before = BEFORE_COLUMNS.map((name) => [name, decodedColumn(name)] as const);
    const looser: [number, string, string, string][] = [];
    cases.forEach((sweepCase, index) => {
      const decided = decide(sweepCase);
      for (const [name, column] of before) {
        if (column[index] !== RAISED && strictness(decided) < strictness(column[index])) {
          looser.push([index, name, column[index], decided]);
        }
      }
    });
    expect(looser).toEqual([]);
  });

  it("gives the contract decision for every case (with the pytest and cargo twins, 0 mismatches)", () => {
    const contract = decodedColumn("contract");
    const mismatches = cases.flatMap((sweepCase, index) => (decide(sweepCase) === contract[index] ? [] : [index]));
    expect(mismatches).toEqual([]);
  });
});

// Named regressions (HELP_HUMAN ruling 2026-10-27). The per-row cases of the (a′), (b′) and (c) tables
// are the `input-contract-*` cases of the shared corpus `cases.json`, decided by all three suites.
const INPUT_CONTRACT_CORPUS_CASES = 49;

describe("DEL-12-02 input contract: named regressions", () => {
  it("keeps the input-contract corpus cases, appended outside the pinned original cases", () => {
    const corpus = JSON.parse(readFileSync(path.join(fixtureDir, "cases.json"), "utf8")) as {
      cases: { case_id: string }[];
    };
    const added = corpus.cases.filter((parityCase) => parityCase.case_id.startsWith("input-contract-"));
    expect(added).toHaveLength(INPUT_CONTRACT_CORPUS_CASES);
    expect(added.every((parityCase) => "explicit_user_release_intent" in parityCase)).toBe(true);
    expect(corpus.cases.slice(-INPUT_CONTRACT_CORPUS_CASES)).toEqual(added);
  });

  it("reads an array label as no public basis on the route projector", () => {
    // Before the contract `String(["public"])` read as a public basis here and the sibling value was
    // included in `public_report`; Python raised TypeError on the same input. Both now redact it.
    const controlled = controlRouteExport(
      {
        record: {
          privacy_classification: ["public"],
          redistribution_status: ["public_permissive"],
          note: "Invented placeholder note"
        }
      },
      { routeId: "DREP-JSON-002", exportContext: "public_report" }
    );
    const note = controlled.decisions.filter((decision) => decision.path.endsWith("note"));
    expect(note.map((decision) => [decision.action, decision.reason_code])).toEqual([
      ["redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"]
    ]);
    expect((controlled.payload as { record: { note: unknown } }).record.note).toBe("[REDACTED]");
  });

  it("never turns a protected block into a redaction for a concrete path", () => {
    // Before the reorder `concrete_path_present: true` returned `redact_value` / `CONCRETE_PATH_REDACTED`
    // ahead of the protected-content block, in every context.
    const item = {
      field_id: "invented.input_contract.path_protected",
      concrete_path_present: true,
      privacy_classification: "protected_suspected",
      redistribution_status: "public_permissive",
      value: "Invented placeholder value"
    };
    for (const context of spec.contexts.map(([exportContext]) => exportContext)) {
      const decision = classifyExportItem(item, { exportContext: context });
      expect([decision.action, decision.reason_code], context).toEqual(["block_export", "PROTECTED_CONTENT_BLOCKED"]);
    }
  });
});
