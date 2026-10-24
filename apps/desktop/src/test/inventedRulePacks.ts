/** Invented rule-pack references and stored-pack envelopes for tests of the solve's
 * rule-pack resolution. Not engineering data: every name, notice and checksum is invented. */
import type { LocalRulePackEnvelope } from "../services/rulePackService";
import type { RulePackReference } from "../types";

export const hex = (digit: string) => digit.repeat(64);

export function inventedReference(id: string, checksum: string, version = "1.0.0"): RulePackReference {
  return {
    rule_pack_id: id,
    name: `Invented pack ${id}`,
    rule_pack_version: version,
    checksum: { algorithm: "sha256", canonicalization: "JCS", payload_scope: "rule_pack_payload", payload_excludes: ["checksums"], value: checksum },
    redistribution_status: "invented_non_engineering_example",
    source_notice: `Invented notice for ${id}; not an engineering basis.`
  };
}

export function inventedEnvelope(projectId: string, id: string, stamped: string, options: { version?: string; matchStatus?: string; notice?: string } = {}): LocalRulePackEnvelope {
  return {
    project_id: projectId, rule_pack_id: id, storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 2,
    document: {
      metadata: { rule_pack_id: id, name: `Invented pack ${id}`, rule_pack_version: options.version ?? "1.0.0", source_notice: options.notice ?? `Invented notice for ${id}; not an engineering basis.` },
      classification: { redistribution_status: "invented_non_engineering_example" },
      checksums: { rule_pack_checksum: { value: stamped } }
    },
    validation: { checksum: { declared_value: stamped, computed: null, match_status: options.matchStatus ?? "match" } } as unknown as LocalRulePackEnvelope["validation"],
    message: "Opened rule-pack document from the local SQLite store."
  };
}
