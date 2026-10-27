// DEL-12-02 input-contract sweep: the classifier never decides a case less strictly than the
// pre-contract code did, and the three implementations agree case for case.
// `fixtures/redaction_export_controls/input_contract_sweep.json` is expanded here, in
// `tests/security/test_redaction_input_contract_sweep.py` and in
// `core/runner/headless/src/redaction_binding.rs` (test module) by the same rule. All data is invented.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { classifyExportItem, type ControlExportContext } from "./redactionExportControls";

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

const cases = expandSweep(spec);

describe("DEL-12-02 input-contract sweep", () => {
  it("expands to the pinned case count", () => {
    expect(cases).toHaveLength(CASE_COUNT);
    expect(frozen.case_count).toBe(CASE_COUNT);
  });

  it("the frozen TypeScript column is the current classifier", () => {
    const expected = decodedColumn("typescript_before");
    expect(expected).toHaveLength(cases.length);
    const mismatches = cases.flatMap((sweepCase, index) => (decide(sweepCase) === expected[index] ? [] : [index]));
    expect(mismatches).toEqual([]);
  });
});
