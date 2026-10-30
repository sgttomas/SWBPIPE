// The diagnostics a model rule-pack reference (`rule_pack_refs[]`, model 0.7.0
// §3.4) receives when it is resolved against the local rule-pack store: in the
// Rule Pack Manager, and at solve time (`modelRulePackResolution.ts`), where a
// reference left out of the input manifest's `active_rule_packs` becomes a
// diagnostic of the run's analysis record (analysis_run v0.3 `diagnostics`,
// after the producer's), under the record hash. Warnings: a reference never
// makes a check pass, and an excluded one is named, never dropped silently.
//
// Python mirror: `core/analysis_runs/compatibility.py` (`rule_pack_ref_diagnostic`,
// `rule_pack_ref_diagnostic_identities`). The two must produce identical bytes.

/** The model references a pack the local store does not hold for the project. */
export const RULE_PACK_REF_UNRESOLVED = "MODEL-RULE-PACK-REF-UNRESOLVED";
/** The stored pack's checksum is not the one the model reference names. */
export const RULE_PACK_REF_CHECKSUM_MISMATCH = "MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH";
/** The `source` of a run record's rule-pack reference diagnostic. */
export const RULE_PACK_REF_DIAGNOSTIC_SOURCE = "desktop/solve-rule-pack-resolution";

export type ExcludedRulePackRefCode = typeof RULE_PACK_REF_UNRESOLVED | typeof RULE_PACK_REF_CHECKSUM_MISMATCH;

/** A model reference left out of the input manifest's `active_rule_packs`, with the reason. */
export type ExcludedRulePackRef = {
  rule_pack_id: string;
  rule_pack_version: string;
  code: ExcludedRulePackRefCode;
};

/** The producer's diagnostic shape (`id`, `code`, `severity`, `message`, `source`, `affected_refs`). */
export type RulePackRefDiagnosticAnnotation = {
  id: string;
  code: ExcludedRulePackRefCode;
  severity: "warning";
  message: string;
  source: typeof RULE_PACK_REF_DIAGNOSTIC_SOURCE;
  affected_refs: [string];
};

const CODES: readonly string[] = [RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH];
const MESSAGE_IDENTITY_LIMIT = 160;

/** One component of a rule-pack identity: `%` as `%25`, then `@` as `%40`. */
function encodeIdentityComponent(component: string): string {
  return component.replaceAll("%", "%25").replaceAll("@", "%40");
}

/** A pack's identity as the recorded revision's `rule_pack_refs` and the rule-pack reference
 * diagnostics name it: `<rule_pack_id>@<version>`, each component with `%` and `@`
 * percent-encoded, so the one `@` left separates them and distinct (id, version) pairs never
 * share a string. An id and version without `%` or `@` are unchanged. Rust
 * (`analysis_runs::rule_pack_identity`) and Python (`rule_pack_identity`) encode the same. */
export function rulePackIdentity(rulePackId: string, version: string): string {
  return `${encodeIdentityComponent(rulePackId)}@${encodeIdentityComponent(version)}`;
}

/** The (id, version) pair an identity names, or null when the text is not an identity. */
export function parseRulePackIdentity(identity: string): { rule_pack_id: string; rule_pack_version: string } | null {
  const parts = identity.split("@");
  if (parts.length !== 2) return null;
  const decode = (component: string): string | null => {
    if (/%(?!25|40)/.test(component)) return null;
    return component.replace(/%40|%25/g, (escape) => escape === "%40" ? "@" : "%");
  };
  const id = decode(parts[0]), version = decode(parts[1]);
  if (id === null || version === null || rulePackIdentity(id, version) !== identity) return null;
  return { rule_pack_id: id, rule_pack_version: version };
}

/** The identity as the message shows it: at most 160 code points, then an ellipsis. */
function shownIdentity(identity: string): string {
  const points = [...identity];
  return points.length > MESSAGE_IDENTITY_LIMIT ? `${points.slice(0, MESSAGE_IDENTITY_LIMIT).join("")}…` : identity;
}

/** The diagnostic for an excluded reference, determined by its code and identity alone. */
export function rulePackRefDiagnostic(code: ExcludedRulePackRefCode, identity: string): RulePackRefDiagnosticAnnotation {
  const shown = shownIdentity(identity);
  const reason = code === RULE_PACK_REF_CHECKSUM_MISMATCH
    ? "the stored pack's checksum was not the reference's at solve time"
    : "the local rule-pack store held no such pack for this project at solve time";
  return {
    id: `diagnostic:rule-pack-ref:${identity}`,
    code,
    severity: "warning",
    message: `Rule pack ${shown} is referenced by the model, but ${reason}; it is not among this run's rule-pack references.`,
    source: RULE_PACK_REF_DIAGNOSTIC_SOURCE,
    affected_refs: [identity]
  };
}

/** The run record's diagnostics for the excluded references, in the model's order. */
export function rulePackRefRecordDiagnostics(excluded: readonly ExcludedRulePackRef[]): Array<{ source_annotation: RulePackRefDiagnosticAnnotation }> {
  return excluded.map((item) => ({ source_annotation: rulePackRefDiagnostic(item.code, rulePackIdentity(item.rule_pack_id, item.rule_pack_version)) }));
}

function canonicalText(value: unknown): string {
  const order = (item: unknown): unknown => Array.isArray(item)
    ? item.map(order)
    : item && typeof item === "object"
      ? Object.fromEntries(Object.keys(item as Record<string, unknown>).sort().map((key) => [key, order((item as Record<string, unknown>)[key])]))
      : item;
  return JSON.stringify(order(value));
}

/** The identities (and codes) a record's appended rule-pack reference diagnostics name, or
 * null when any is not exactly such a diagnostic or an identity repeats. */
export function rulePackRefDiagnosticEntries(diagnostics: readonly unknown[]): Array<{ code: ExcludedRulePackRefCode; identity: string }> | null {
  const entries: Array<{ code: ExcludedRulePackRefCode; identity: string }> = [];
  const seen = new Set<string>();
  for (const item of diagnostics) {
    const annotation = item && typeof item === "object" && !Array.isArray(item) ? (item as { source_annotation?: unknown }).source_annotation : undefined;
    if (!item || Object.keys(item as object).length !== 1 || !annotation || typeof annotation !== "object" || Array.isArray(annotation)) return null;
    const { code, affected_refs: refs } = annotation as { code?: unknown; affected_refs?: unknown };
    if (typeof code !== "string" || !CODES.includes(code) || !Array.isArray(refs) || refs.length !== 1 || typeof refs[0] !== "string" || !refs[0]) return null;
    const identity = refs[0];
    if (!parseRulePackIdentity(identity) || seen.has(identity) || canonicalText(annotation) !== canonicalText(rulePackRefDiagnostic(code as ExcludedRulePackRefCode, identity))) return null;
    seen.add(identity);
    entries.push({ code: code as ExcludedRulePackRefCode, identity });
  }
  return entries;
}

/** The rule-pack reference diagnostics a run record appends after its producer's
 * `producerDiagnosticCount` diagnostics, as recorded, in the record's order. The outputs built
 * from a record (result export, stress-neutral package, preview handoff) project these, so
 * that each output and the record name the same exclusions. A record with none gives `[]`;
 * appended diagnostics that are not exactly such are refused, as the record validator refuses
 * them ("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH"). */
export function recordRulePackRefAnnotations(recordDiagnostics: readonly unknown[] | null | undefined, producerDiagnosticCount: number): RulePackRefDiagnosticAnnotation[] {
  const entries = rulePackRefDiagnosticEntries((recordDiagnostics ?? []).slice(producerDiagnosticCount));
  if (!entries) throw new Error("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH");
  return entries.map(({ code, identity }) => rulePackRefDiagnostic(code, identity));
}

/** Whether `item`, a diagnostic already in an output, carries `annotation`: it has the same
 * code and names the same reference, by the annotation's id or the reference's identity, as its
 * `id`, among its `affected_refs`, or as its `affected_object`'s id (either reference shape). */
export function carriesRulePackRefDiagnostic(item: unknown, annotation: RulePackRefDiagnosticAnnotation): boolean {
  if (!item || typeof item !== "object" || Array.isArray(item)) return false;
  const { code, id, affected_refs: refs, affected_object: object } = item as Record<string, unknown>;
  if (code !== annotation.code) return false;
  const names = new Set<string>([annotation.id, annotation.affected_refs[0]]);
  const objectIds = object && typeof object === "object" ? [(object as Record<string, unknown>).ref_id, (object as Record<string, unknown>).ref] : [];
  return [id, ...(Array.isArray(refs) ? refs : []), ...objectIds].some((name) => typeof name === "string" && names.has(name));
}

/** The record's rule-pack reference diagnostics an output adds to the diagnostics it already
 * carries (`present`), in the record's order: each one no diagnostic there carries, so that an
 * exclusion appears once. */
export function rulePackRefAnnotationsToAdd(annotations: readonly RulePackRefDiagnosticAnnotation[], present: readonly unknown[]): RulePackRefDiagnosticAnnotation[] {
  return annotations.filter((annotation) => !present.some((item) => carriesRulePackRefDiagnostic(item, annotation)));
}

/** The model's references the manifest's `active_rule_packs` leaves out, as (id, version)
 * pairs in the model's order: the references a run record's rule-pack diagnostics must name. */
export function manifestExcludedRulePacks(manifest: {
  model_basis?: { model_payload?: { rule_pack_refs?: Array<{ rule_pack_id: string; rule_pack_version: string }> } };
  active_rule_packs?: Array<{ rule_pack_id: string; version: string }>;
}): Array<{ rule_pack_id: string; rule_pack_version: string }> {
  // As Rust's `manifest_excluded_rule_packs` and Python's: a list of packs and references
  // with text ids and versions is required ("ANALYSIS-RUN-MANIFEST-INVALID" otherwise).
  const invalid = (detail: string) => new Error(`ANALYSIS-RUN-MANIFEST-INVALID: ${detail}.`);
  const pair = (item: unknown, versionKey: "version" | "rule_pack_version") => {
    const record = item && typeof item === "object" && !Array.isArray(item) ? item as Record<string, unknown> : {};
    const id = record.rule_pack_id, version = record[versionKey];
    return typeof id === "string" && typeof version === "string" ? { rule_pack_id: id, rule_pack_version: version } : null;
  };
  // A model without references excludes nothing, whatever the manifest holds.
  const references: unknown = manifest.model_basis?.model_payload?.rule_pack_refs;
  if (references === undefined) return [];
  if (!Array.isArray(references)) throw invalid("the model's rule_pack_refs is not a list");
  if (references.length === 0) return [];
  if (!Array.isArray(manifest.active_rule_packs)) throw invalid("the input manifest has no active_rule_packs list");
  const active = manifest.active_rule_packs.map((pack) => pair(pack, "version") ?? (() => { throw invalid("an active rule pack has no id or version"); })());
  return references
    .map((reference) => pair(reference, "rule_pack_version") ?? (() => { throw invalid("a model rule-pack reference has no id or version"); })())
    .filter((reference) => !active.some((pack) => pack.rule_pack_id === reference.rule_pack_id && pack.rule_pack_version === reference.rule_pack_version));
}

/** [`manifestExcludedRulePacks`] as the identities the diagnostics carry. */
export function manifestExcludedRulePackIdentities(manifest: Parameters<typeof manifestExcludedRulePacks>[0]): string[] {
  return manifestExcludedRulePacks(manifest).map((pair) => rulePackIdentity(pair.rule_pack_id, pair.rule_pack_version));
}
