/** The solve's rule-pack resolution (#1602, `resolveModelRulePacks`) and Save Handoff's
 * (W-6b, Rust `rule_pack_ref_exclusion` in `src-tauri/src/handoff_package.rs`) give every
 * shared vector the same outcome (`fixtures/handoff/rule_pack_ref_resolution_vectors.json`;
 * the Rust half is `rule_pack_ref_resolution_matches_the_solve_on_the_shared_vectors`). The
 * store is a fake of `open_local_rule_pack`: the pack stored under the asked project, with the
 * validator's checksum verdict the vector pins. All data are invented. */
import { describe, expect, it } from "vitest";
import vectors from "../../../../fixtures/handoff/rule_pack_ref_resolution_vectors.json";
import basePack from "../../../../fixtures/product_preview/invented_demo_rule_pack.json";
import { resolveModelRulePacks } from "./modelRulePackResolution";
import type { LocalRulePackEnvelope } from "./rulePackService";
import type { PreviewModel, RulePackReference } from "../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Vector = (typeof vectors.vectors)[number];

function storedDocument(vector: Vector): Json {
  const document: Json = structuredClone(basePack);
  for (const edit of vector.document_edits as Array<{ pointer: string; value?: unknown; delete?: boolean }>) {
    const keys = edit.pointer.split("/").slice(1), key = keys.pop()!;
    const parent = keys.reduce((value, part) => value[part], document);
    if (edit.delete) delete parent[key];
    else parent[key] = edit.value;
  }
  document.checksums.rule_pack_checksum.value = vector.stamp;
  return document;
}

describe("the solve resolves each shared vector as Save Handoff does", () => {
  it("covers every case the vectors name", () => {
    expect(vectors.vectors.length).toBeGreaterThanOrEqual(11);
    expect(new Set(vectors.vectors.map((vector) => vector.expected))).toEqual(new Set(["resolved", "MODEL-RULE-PACK-REF-UNRESOLVED", "MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH"]));
  });

  it.each(vectors.vectors)("$name: $expected", async (vector) => {
    const reference = vector.reference as unknown as RulePackReference;
    const model = { project: { id: vectors.model_project_id }, rule_pack_refs: [reference] } as unknown as PreviewModel;
    const opened: string[] = [];
    const resolution = await resolveModelRulePacks(model, async (projectId, rulePackId) => {
      opened.push(`${projectId}/${rulePackId}`);
      const held = vector.stored_under === projectId && rulePackId === reference.rule_pack_id;
      const envelope = held ? {
        project_id: projectId, rule_pack_id: rulePackId, storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 2,
        document: storedDocument(vector),
        validation: { checksum: { match_status: vector.match_status } },
        message: "Opened rule-pack document from the local SQLite store."
      } as unknown as LocalRulePackEnvelope : null;
      return { route: "tauri_backend", envelope };
    });
    // The model's project only, as the Rust lookup reads it.
    expect(opened).toEqual([`${vectors.model_project_id}/${reference.rule_pack_id}`]);
    const outcome = resolution.excluded.length ? resolution.excluded.map((item) => item.code) : resolution.active_rule_packs.map(() => "resolved");
    expect(outcome).toEqual([vector.expected]);
  });
});
