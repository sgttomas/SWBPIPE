// The model's rule-pack references, resolved against the local rule-pack store
// at solve time (FR-CMP-002; DEL-14-01 Design `desktop-states-runs-comparison.md`
// §3). A reference (`rule_pack_refs[]`, model 0.7.0 §3.4) is the stored pack's
// identity and its stamped checksum. It enters the input manifest's
// `active_rule_packs` only when the local store holds that pack and the stored
// document's stamped checksum equals the reference's and matches the document's
// own recomputation. Any other reference is excluded and named
// (`excluded_rule_pack_refs`), never dropped silently.
//
// This is identity recording only. The solve does not read rule content, run a
// rule check or apply a pack; a reference never makes a check pass (F-PIP-2).
import type { PreviewModel, RulePackReference } from "../types";
import type { InputManifestRulePack } from "./inputManifestService";
import { openLocalRulePack, type RulePackOpenRoute } from "./rulePackService";
import {
  RULE_PACK_REF_CHECKSUM_MISMATCH,
  RULE_PACK_REF_UNRESOLVED,
  type ExcludedRulePackRef,
  type ExcludedRulePackRefCode
} from "./rulePackRefDiagnostics";

export { RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED, type ExcludedRulePackRef };

export type ModelRulePackResolution = {
  active_rule_packs: InputManifestRulePack[];
  excluded_rule_pack_refs: ExcludedRulePackRef[];
};

const SHA256_HEX = /^[0-9a-f]{64}$/;

type OpenPack = (projectId: string, rulePackId: string) => Promise<RulePackOpenRoute>;

function record(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : {};
}

function text(value: unknown): string | null {
  return typeof value === "string" && value.trim() ? value : null;
}

/** Resolves each of the model's `rule_pack_refs`, in the model's order, against the
 * project's packs in the local store. Never throws for a reference: a store that is
 * unavailable (browser preview) or fails leaves the reference unresolved. */
export async function resolveModelRulePacks(model: PreviewModel, open: OpenPack = openLocalRulePack): Promise<ModelRulePackResolution> {
  const resolution: ModelRulePackResolution = { active_rule_packs: [], excluded_rule_pack_refs: [] };
  for (const reference of model.rule_pack_refs ?? []) {
    const exclude = (code: ExcludedRulePackRefCode) => resolution.excluded_rule_pack_refs.push({
      rule_pack_id: reference.rule_pack_id,
      rule_pack_version: reference.rule_pack_version,
      checksum_sha256: reference.checksum?.value ?? "",
      code
    });
    let route: RulePackOpenRoute;
    try {
      route = await open(model.project.id, reference.rule_pack_id);
    } catch {
      exclude(RULE_PACK_REF_UNRESOLVED);
      continue;
    }
    if (route.route !== "tauri_backend" || !route.envelope) {
      exclude(RULE_PACK_REF_UNRESOLVED);
      continue;
    }
    const document = record(route.envelope.document);
    const metadata = record(document.metadata);
    const stamped = text(record(record(document.checksums).rule_pack_checksum).value);
    const id = text(metadata.rule_pack_id);
    const version = text(metadata.rule_pack_version);
    const sourceNotice = text(metadata.source_notice);
    if (id !== reference.rule_pack_id || !version || !sourceNotice) {
      exclude(RULE_PACK_REF_UNRESOLVED);
      continue;
    }
    const recomputedMatches = route.envelope.validation?.checksum?.match_status === "match";
    if (!stamped || !SHA256_HEX.test(stamped) || stamped !== reference.checksum?.value || !recomputedMatches || version !== reference.rule_pack_version) {
      exclude(RULE_PACK_REF_CHECKSUM_MISMATCH);
      continue;
    }
    resolution.active_rule_packs.push({ rule_pack_id: id, version, checksum_sha256: stamped, source_notice: sourceNotice });
  }
  return resolution;
}

/** The rule-pack references the input manifest records, as display lines: each pack it
 * records, then each excluded model reference with its diagnostic. Empty when the solved
 * model referenced no pack. */
export function manifestRulePackLines(manifest: {
  model_basis?: { model_payload?: { rule_pack_refs?: RulePackReference[] } };
  active_rule_packs?: InputManifestRulePack[];
  excluded_rule_pack_refs?: ExcludedRulePackRef[];
} | null | undefined): { recorded: string[]; diagnostics: string[] } {
  const recorded = (manifest?.active_rule_packs ?? []).map((pack) =>
    `${pack.rule_pack_id} v${pack.version} (sha256 ${pack.checksum_sha256.slice(0, 12)}…)`);
  const diagnostics = (manifest?.excluded_rule_pack_refs ?? []).map(excludedRulePackRefText);
  return { recorded, diagnostics };
}

export function excludedRulePackRefText(excluded: ExcludedRulePackRef): string {
  const subject = `${excluded.rule_pack_id} v${excluded.rule_pack_version}`;
  const reason = excluded.code === RULE_PACK_REF_CHECKSUM_MISMATCH
    ? `the stored pack's checksum is not the model reference's sha256 ${excluded.checksum_sha256.slice(0, 12)}…`
    : "the local rule-pack store held no such pack for this project at solve time";
  return `${excluded.code} (warning): ${subject} is referenced by the model, but ${reason}; it is not among this run's rule-pack references.`;
}
