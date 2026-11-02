// The model's rule-pack references, resolved against the local rule-pack store
// at solve time (FR-CMP-002; DEL-14-01 Design `desktop-states-runs-comparison.md`
// §3). A reference (`rule_pack_refs[]`, model 0.7.0 §3.4) is the stored pack's
// identity and its stamped checksum. It enters the input manifest's
// `active_rule_packs` only when the local store holds that pack and the stored
// document's stamped checksum equals the reference's and matches the document's
// own recomputation. Any other reference is excluded and becomes a diagnostic of
// the run's analysis record (`rulePackRefDiagnostics.ts`), never dropped silently.
//
// This is identity recording only. The solve does not read rule content, run a
// rule check or apply a pack; a reference never makes a check pass (F-PIP-2).
import { isNativeAnalysisRun, nativeRunPreview, type SessionAnalysisRun } from "./nativeResult";
import type { AnalysisRunEnvelope, PreviewModel } from "../types";
import type { CurrentSessionInputManifest, InputManifestRulePack } from "./inputManifestService";
import { openLocalRulePack, type RulePackOpenRoute } from "./rulePackService";
import {
  RULE_PACK_REF_CHECKSUM_MISMATCH,
  RULE_PACK_REF_UNRESOLVED,
  RULE_PACK_REF_DIAGNOSTIC_SOURCE,
  rulePackIdentity,
  rulePackRefDiagnosticEntries,
  type ExcludedRulePackRef,
  type ExcludedRulePackRefCode
} from "./rulePackRefDiagnostics";

export { RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED, type ExcludedRulePackRef };

export type ModelRulePackResolution = {
  active_rule_packs: InputManifestRulePack[];
  /** The references left out, in the model's order: the run record's rule-pack diagnostics. */
  excluded: ExcludedRulePackRef[];
};

/** A model without references: no store call, nothing recorded. */
export const NO_RULE_PACK_RESOLUTION: Readonly<ModelRulePackResolution> = Object.freeze({ active_rule_packs: [], excluded: [] });

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
  const resolution: ModelRulePackResolution = { active_rule_packs: [], excluded: [] };
  for (const reference of model.rule_pack_refs ?? []) {
    const exclude = (code: ExcludedRulePackRefCode) => resolution.excluded.push({
      rule_pack_id: reference.rule_pack_id,
      rule_pack_version: reference.rule_pack_version,
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

/** The excluded references a run record names in its rule-pack diagnostics (after the
 * producer's), for re-recording the record (a rule-check aggregate) without a new solve.
 * Each identity is read back through the manifest's model references. */
export function recordRulePackExclusions(run: SessionAnalysisRun | null | undefined, manifest: CurrentSessionInputManifest, producerDiagnosticCount: number): ExcludedRulePackRef[] {
  // A native record carries its rule-pack reference diagnostics whole (the producer's stay native).
  const appended = isNativeAnalysisRun(run) ? nativeRunPreview(run).rule_pack_ref_diagnostics : (run?.analysis_run.diagnostics ?? []).slice(producerDiagnosticCount);
  const entries = rulePackRefDiagnosticEntries(appended) ?? [];
  const references = new Map((manifest.model_basis.model_payload.rule_pack_refs ?? []).map((reference) =>
    [rulePackIdentity(reference.rule_pack_id, reference.rule_pack_version), reference] as const));
  return entries.flatMap(({ code, identity }) => {
    const reference = references.get(identity);
    return reference ? [{ rule_pack_id: reference.rule_pack_id, rule_pack_version: reference.rule_pack_version, code }] : [];
  });
}

/** What a run shows of its rule-pack references: the packs its input manifest records,
 * then the diagnostic message of each excluded reference, read from the run record's
 * diagnostics. Null when the solved model referenced no pack. */
/** C1 slice 6: a restored record's rule-pack reference diagnostics (its header's), read as a record's. */
export type RestoredRecordDiagnostics = Readonly<{ analysis_run: Readonly<{ diagnostics: readonly unknown[] }> }>;
export function runRulePackLines(manifest: CurrentSessionInputManifest | null | undefined, run: SessionAnalysisRun | RestoredRecordDiagnostics | null | undefined): { recorded: string[]; diagnostics: string[] } | null {
  if (!manifest || !(manifest.model_basis?.model_payload?.rule_pack_refs?.length)) return null;
  const recorded = (manifest.active_rule_packs ?? []).map((pack) =>
    `${pack.rule_pack_id} v${pack.version} (sha256 ${pack.checksum_sha256.slice(0, 12)}…)`);
  const listed = isNativeAnalysisRun(run) ? nativeRunPreview(run).rule_pack_ref_diagnostics : (run as { analysis_run?: { diagnostics?: unknown } } | null | undefined)?.analysis_run?.diagnostics;
  const diagnostics = (Array.isArray(listed) ? listed : [])
    .map((item) => item?.source_annotation as { code?: unknown; message?: unknown; source?: unknown } | undefined)
    .flatMap((annotation) => annotation?.source === RULE_PACK_REF_DIAGNOSTIC_SOURCE && typeof annotation.code === "string" && typeof annotation.message === "string"
      ? [`${annotation.code} (warning): ${annotation.message}`]
      : []);
  return { recorded, diagnostics };
}
