// The diagnostics a model rule-pack reference (`rule_pack_refs[]`, model 0.7.0
// §3.4) receives when it is resolved against the local rule-pack store: in the
// Rule Pack Manager and at solve time (`modelRulePackResolution.ts`), where an
// excluded reference is recorded in the input manifest. Warnings: a reference
// never makes a check pass, and an excluded one is named, never dropped silently.

/** The model references a pack the local store does not hold for the project. */
export const RULE_PACK_REF_UNRESOLVED = "MODEL-RULE-PACK-REF-UNRESOLVED";
/** The stored pack's checksum is not the one the model reference names. */
export const RULE_PACK_REF_CHECKSUM_MISMATCH = "MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH";

export type ExcludedRulePackRefCode = typeof RULE_PACK_REF_UNRESOLVED | typeof RULE_PACK_REF_CHECKSUM_MISMATCH;

/** A model reference left out of the input manifest's `active_rule_packs`, with the reason. */
export type ExcludedRulePackRef = {
  rule_pack_id: string;
  rule_pack_version: string;
  /** The reference's stamped checksum value. */
  checksum_sha256: string;
  code: ExcludedRulePackRefCode;
};
