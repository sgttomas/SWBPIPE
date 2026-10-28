// DEL-12-02 input-contract sweep: the classifier never decides a case less strictly than the
// pre-contract code did, and the three implementations agree case for case.
// `fixtures/redaction_export_controls/input_contract_sweep.json` is expanded here, in
// `tests/security/test_redaction_input_contract_sweep.py` and in
// `core/runner/headless/src/redaction_binding.rs` (test module) by the same rule. All data is invented.
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  classifyExportItem,
  controlRouteExport,
  redactExportPayload,
  type ControlExportContext,
  type RedactionExportContext
} from "./redactionExportControls";

type Item = Record<string, unknown>;
type SweepCase = {
  family: "labels" | "flags" | "claims" | "alias_items" | "alias_projector" | "proto_items" | "proto_projector";
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
  aliases: {
    privacy_classification: unknown[];
    classification: unknown[];
    redistribution: unknown[];
    item_placements: string[];
    private_provenance: Item;
    projector: {
      route_id: string;
      record_key: string;
      leaf_key: string;
      leaf_value: unknown;
      placements: string[];
      public_provenance: Item;
      contexts: [ControlExportContext, boolean][];
    };
  };
  proto: {
    member: string;
    privacy_classification: unknown[];
    classification: unknown[];
    redistribution: unknown[];
    member_values: unknown[];
    item_placements: string[];
    projector_placements: string[];
  };
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
const CASE_COUNT = 100_744;

/** Sets a member as JSON parsing does: an own data member, `__proto__` included (assignment would make a
 * `__proto__` member the object's prototype). */
function setOwn(target: Item, key: string, value: unknown): void {
  Object.defineProperty(target, key, { value, writable: true, enumerable: true, configurable: true });
}

/** Items in family order labels, flags, claims, alias_items; each decided in the spec's twelve contexts
 * in order. Then the alias_projector records, each decided by the route projector in the projector's ten
 * contexts in order; then the proto_items and the proto_projector records in the same way. A value equal
 * to `spec.absent` leaves its member out. */
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
  const { aliases } = sweep;
  const aliasLabels = ([privacy, alias, redistribution]: unknown[]): Item => {
    const labels: Item = {};
    put(labels, "privacy_classification", privacy);
    put(labels, "classification", alias);
    put(labels, "redistribution_status", redistribution);
    return labels;
  };
  const aliasGrid = aliases.privacy_classification.flatMap((privacy) =>
    aliases.classification.flatMap((alias) => aliases.redistribution.map((redistribution) => [privacy, alias, redistribution]))
  );
  for (const grid of aliasGrid) {
    for (const placement of aliases.item_placements) {
      const labels = aliasLabels(grid);
      const item: Item =
        placement === "provenance" ? { value: "x", provenance: labels } : { value: "x", ...labels };
      if (placement === "item_private_provenance") item.provenance = { ...aliases.private_provenance };
      items.push(["alias_items", item]);
    }
  }
  const cases = items.flatMap(([family, item]) =>
    sweep.contexts.map(([context, localIntent, releaseIntent]) => ({ family, item, context, localIntent, releaseIntent }))
  );
  const { projector } = aliases;
  for (const grid of aliasGrid) {
    for (const placement of projector.placements) {
      const record = aliasLabels(grid);
      if (placement === "record_public_provenance") record.provenance = { ...projector.public_provenance };
      record[projector.leaf_key] = projector.leaf_value;
      for (const [context, localIntent] of projector.contexts) {
        cases.push({ family: "alias_projector", item: record, context, localIntent, releaseIntent: false });
      }
    }
  }
  const { proto } = sweep;
  const protoGrid = proto.privacy_classification.flatMap((privacy) =>
    proto.classification.flatMap((alias) =>
      proto.redistribution.flatMap((redistribution) =>
        proto.member_values.map((memberValue) => [privacy, alias, redistribution, memberValue])
      )
    )
  );
  for (const [privacy, alias, redistribution, memberValue] of protoGrid) {
    for (const placement of proto.item_placements) {
      const labels = aliasLabels([privacy, alias, redistribution]);
      setOwn(labels, proto.member, memberValue);
      const item: Item = placement === "provenance" ? { value: "x", provenance: labels } : { value: "x", ...labels };
      for (const [context, localIntent, releaseIntent] of sweep.contexts) {
        cases.push({ family: "proto_items", item, context, localIntent, releaseIntent });
      }
    }
  }
  for (const [privacy, alias, redistribution, memberValue] of protoGrid) {
    for (const placement of proto.projector_placements) {
      const record = aliasLabels([privacy, alias, redistribution]);
      if (placement === "provenance") {
        const provenance: Item = { ...projector.public_provenance };
        setOwn(provenance, proto.member, memberValue);
        record.provenance = provenance;
      } else {
        setOwn(record, proto.member, memberValue);
        if (placement === "record_public_provenance") record.provenance = { ...projector.public_provenance };
      }
      record[projector.leaf_key] = projector.leaf_value;
      for (const [context, localIntent] of projector.contexts) {
        cases.push({ family: "proto_projector", item: record, context, localIntent, releaseIntent: false });
      }
    }
  }
  return cases;
}

/** The `action/reason` for one sweep case: the classifier's for an item, the route projector's for the
 * one leaf of an `alias_projector` or `proto_projector` record. */
function decide(sweepCase: SweepCase): string {
  if (sweepCase.family === "alias_projector" || sweepCase.family === "proto_projector") {
    const { projector } = spec.aliases;
    const controlled = controlRouteExport(
      { [projector.record_key]: sweepCase.item },
      {
        routeId: projector.route_id,
        exportContext: sweepCase.context as RedactionExportContext,
        explicitLocalPrivateIntent: sweepCase.localIntent
      }
    );
    // A projected decision's path carries the projector's key prefix on each member.
    const leaf = `$.${projector.record_key}.${projector.leaf_key}`;
    const decided = controlled.decisions
      .filter((decision) => decision.path.replaceAll("__route_key__", "") === leaf)
      .map((decision) => `${decision.action}/${decision.reason_code}`);
    if (decided.length !== 1) throw new Error(`expected one decision at ${leaf}, got ${decided.length}`);
    return decided[0];
  }
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
// The Rust mirror had no route projector before the alias decision, so its frozen column holds no
// decision for an alias_projector case.
const NOT_MIRRORED = "not_mirrored/NO_ROUTE_PROJECTOR";
const NO_DECISION = [RAISED, NOT_MIRRORED];
// The TypeScript projector before the decision that no JSON member becomes a prototype (HELP_HUMAN,
// 2026-10-27: "Parse so that no JSON member can become a prototype") copied a record by assignment, so a
// `__proto__` member became the copy's prototype, or was dropped when it was not an object. Its
// proto_projector decisions are of a different record than the input, so those cases are held to the
// Python column, the reference whose reading of the member as an ordinary own member all three now share
// (the Rust column is `not_mirrored`).
const PROTOTYPE_COPY_COLUMN = ["proto_projector", "typescript_before"] as const;

function heldTo(family: SweepCase["family"], name: string, code: string): boolean {
  return !NO_DECISION.includes(code) && !(family === PROTOTYPE_COPY_COLUMN[0] && name === PROTOTYPE_COPY_COLUMN[1]);
}

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
    // the TypeScript and Rust columns. Nor is the Rust column's `not_mirrored` on an alias_projector or
    // proto_projector case; it is held to the Python and TypeScript columns. A proto_projector case is not
    // held to the TypeScript column (`PROTOTYPE_COPY_COLUMN`).
    const before = BEFORE_COLUMNS.map((name) => [name, decodedColumn(name)] as const);
    const looser: [number, string, string, string][] = [];
    cases.forEach((sweepCase, index) => {
      const decided = decide(sweepCase);
      for (const [name, column] of before) {
        if (heldTo(sweepCase.family, name, column[index]) && strictness(decided) < strictness(column[index])) {
          looser.push([index, name, column[index], decided]);
        }
      }
    });
    expect(looser).toEqual([]);
  });

  it("sets the TypeScript column aside only for the proto_projector cases, as pinned", () => {
    // Each set-aside case is still held to the Python column. The frozen TypeScript decisions there differ
    // from the contract as pinned: 780 less strict (a `__proto__` member's public labels, inherited by the
    // copy, lifted a sibling into a public basis) and 120 stricter (a member's restrictive privacy,
    // inherited by the copy, hid the record's own `classification` alias).
    const contract = decodedColumn("contract");
    const python = decodedColumn("python_before");
    const typescript = decodedColumn("typescript_before");
    const setAside = cases.flatMap((sweepCase, index) => (sweepCase.family === "proto_projector" ? [index] : []));
    expect(setAside).toHaveLength(10_080);
    expect(setAside[0]).toBeGreaterThanOrEqual(82_600);
    expect(setAside.every((index) => !NO_DECISION.includes(python[index]))).toBe(true);
    expect(setAside.filter((index) => strictness(typescript[index]) < strictness(contract[index]))).toHaveLength(780);
    expect(setAside.filter((index) => strictness(typescript[index]) > strictness(contract[index]))).toHaveLength(120);
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

  it("reads no classification alias under a null primary label on the route projector", () => {
    // HELP_HUMAN decision, 2026-10-27: "A null primary label must not let an alias lift a sibling into
    // `public_report`." This projector read the `classification` alias with `??` and so included `note`;
    // Python reads it only when `privacy_classification` is absent and redacts it, as all three now do.
    const record: Item = {
      privacy_classification: null,
      classification: "public",
      redistribution_status: "public_permissive",
      note: "INVENTED"
    };
    const noteOf = (payloadRecord: Item) => {
      const controlled = controlRouteExport(
        { record: payloadRecord },
        { routeId: "DREP-JSON-002", exportContext: "public_report" }
      );
      return {
        decisions: controlled.decisions
          .filter((decision) => decision.path.endsWith("note"))
          .map((decision) => [decision.action, decision.reason_code]),
        note: (controlled.payload as { record: { note: unknown } }).record.note
      };
    };
    expect(noteOf(record)).toEqual({ decisions: [["redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"]], note: "[REDACTED]" });
    // The alias is still the label when the primary member is absent.
    const { privacy_classification: _primary, ...aliasOnly } = record;
    expect(noteOf(aliasOnly)).toEqual({ decisions: [["include", "SAFE_PUBLIC_METADATA"]], note: "INVENTED" });
  });

  // A JSON `__proto__` member is an ordinary own member (HELP_HUMAN decision, 2026-10-27: "Parse so that no
  // JSON member can become a prototype"). One class per kind of member label in the code review's 180 cases
  // and their converse: the member's labels label only the member, never the record that holds it, on the
  // record or in its provenance, in the five projector contexts with and without intent.
  const PROTO_MEMBER_CLASSES: [string, Item, Item, boolean][] = [
    // [class, the member, the record's own labels, the record's labels are a public basis]
    ["member_unknown_privacy", { privacy_classification: "unknown" }, { classification: "public", redistribution_status: "public_permissive" }, true],
    ["member_private_privacy", { privacy_classification: "private_project_data" }, { classification: "public", redistribution_status: "public_permissive" }, true],
    [
      "member_private_labels",
      { privacy_classification: "private_project_data", redistribution_status: "private_only" },
      { classification: "public", redistribution_status: "public_permissive" },
      true
    ],
    ["member_private_redistribution", { redistribution_status: "private_only" }, { privacy_classification: "public", redistribution_status: "public_permissive" }, true],
    ["member_public_labels", { privacy_classification: "public", redistribution_status: "public_permissive" }, {}, false],
    ["member_public_redistribution", { redistribution_status: "public_permissive" }, { privacy_classification: "public" }, false],
    ["member_public_privacy", { privacy_classification: "public" }, { redistribution_status: "public_permissive" }, false],
    [
      "member_private_privacy_public_redistribution",
      { privacy_classification: "private_project_data", redistribution_status: "public_permissive" },
      { classification: "public" },
      false
    ]
  ];
  const PROJECTOR_CONTEXTS = spec.aliases.projector.contexts;
  const PUBLIC_PROVENANCE = { privacy_classification: "public", redistribution_status: "public_permissive" };
  const protoRecords = (member: Item, labels: Item, memberKey = "__proto__"): [string, Item][] => {
    const onRecord: Item = { ...labels };
    setOwn(onRecord, memberKey, member);
    onRecord.note = "INVENTED";
    const provenance: Item = { ...PUBLIC_PROVENANCE };
    setOwn(provenance, memberKey, member);
    return [
      ["record", onRecord],
      ["provenance", { ...labels, provenance, note: "INVENTED" }]
    ];
  };
  const noteDecision = (record: Item, exportContext: RedactionExportContext, explicitLocalPrivateIntent: boolean) => {
    const controlled = controlRouteExport({ record }, { routeId: "DREP-JSON-002", exportContext, explicitLocalPrivateIntent });
    const note = controlled.decisions
      .filter((decision) => decision.path.replaceAll("__route_key__", "") === "$.record.note")
      .map((decision) => [decision.action, decision.reason_code]);
    expect(note).toHaveLength(1);
    return { decided: note[0], payload: controlled.payload as { record: Item } | null };
  };
  const ownMember = (object: unknown, key: string) => Object.prototype.hasOwnProperty.call(object, key);

  it("reads a JSON __proto__ member as an ordinary own member on the route projector (the review's repro)", () => {
    // Code review r1 at 5be6c053ef: the copy made the member its prototype, so the inherited restrictive
    // label was ignored only by the own-property check and the alias was read. The member labels only
    // itself, so the record's own alias and redistribution are its public basis and `note` is included, as
    // Python always decided; the member is kept as an own member and its leaf is redacted.
    const payload = JSON.parse(
      '{"record":{"__proto__":{"privacy_classification":"private_project_data"},"classification":"public","redistribution_status":"public_permissive","note":"INVENTED"}}'
    ) as { record: Item };
    const { decided, payload: materialized } = noteDecision(payload.record, "public_report", false);
    expect(decided).toEqual(["include", "SAFE_PUBLIC_METADATA"]);
    expect(JSON.stringify(materialized)).toBe(
      '{"record":{"__proto__":{"privacy_classification":"[REDACTED]"},"classification":"public","redistribution_status":"public_permissive","note":"INVENTED"}}'
    );
    const record = (materialized as { record: Item }).record;
    expect(Object.getPrototypeOf(record)).toBe(Object.prototype);
    expect(ownMember(record, "__proto__")).toBe(true);
    expect(ownMember(record, "privacy_classification")).toBe(false);
  });

  it("labels only the member for each __proto__ member class on the route projector", () => {
    for (const [name, member, labels, publicBasis] of PROTO_MEMBER_CLASSES) {
      const renamed = protoRecords(member, labels, "proto_member");
      protoRecords(member, labels).forEach(([placement, record], index) => {
        for (const [context, intent] of PROJECTOR_CONTEXTS) {
          const where = `${name} ${placement} ${context} ${intent}`;
          const expected = publicBasis
            ? ["include", "SAFE_PUBLIC_METADATA"]
            : [context === "local_private" ? "warning_only" : "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"];
          const { decided, payload } = noteDecision(record, context as RedactionExportContext, intent);
          expect(decided, where).toEqual(expected);
          expect(noteDecision(renamed[index][1], context as RedactionExportContext, intent).decided, where).toEqual(decided);
          // The member is kept as an ordinary member wherever the payload is materialized.
          if (context !== "local_private" || intent) {
            const holder = placement === "record" ? payload?.record : payload?.record.provenance;
            expect(ownMember(holder, "__proto__"), where).toBe(true);
            expect(Object.getPrototypeOf(holder), where).toBe(Object.prototype);
          }
        }
      });
    }
  });

  it("labels only the member for each __proto__ member class in the classifier", () => {
    for (const [name, member, labels] of PROTO_MEMBER_CLASSES) {
      for (const placement of ["item", "provenance"]) {
        const item = (memberKey: string): Item => {
          const carried: Item = { ...labels };
          setOwn(carried, memberKey, member);
          return placement === "provenance" ? { value: "x", provenance: carried } : { value: "x", ...carried };
        };
        for (const [exportContext, explicitLocalPrivateIntent, explicitUserReleaseIntent] of spec.contexts) {
          const decide = (candidate: Item) => {
            const decision = classifyExportItem(candidate, { exportContext, explicitLocalPrivateIntent, explicitUserReleaseIntent });
            return [decision.action, decision.reason_code];
          };
          expect(decide(item("__proto__")), `${name} ${placement} ${exportContext}`).toEqual(decide(item("proto_member")));
        }
      }
    }
  });

  it("keeps a __proto__ member an own member through the redaction walk and its redaction", () => {
    // The walk's copy of an unlabelled object and the redaction's copy without unsafe detail kept a
    // `__proto__` member only as the copy's prototype, and so dropped it from the payload.
    const payload = JSON.parse(
      '{"container":{"__proto__":{"note":"INVENTED"}},"item":{"__proto__":{"privacy_classification":"public"},"value":"x","file_path":"INVENTED","privacy_classification":"private_project_data"}}'
    ) as Item;
    const controlled = redactExportPayload(payload, { exportContext: "public_report" });
    expect(JSON.stringify(controlled.payload)).toBe(
      '{"container":{"__proto__":{"note":"INVENTED"}},"item":{"__proto__":{"privacy_classification":"public"},"value":"[REDACTED]","privacy_classification":"redacted"}}'
    );
    const out = controlled.payload as { container: Item; item: Item };
    expect([out.container, out.item].map((copy) => Object.getPrototypeOf(copy))).toEqual([Object.prototype, Object.prototype]);
    expect(out.item.privacy_classification).toBe("redacted");
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
