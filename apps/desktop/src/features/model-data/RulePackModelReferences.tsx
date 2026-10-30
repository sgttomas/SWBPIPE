// Rule Pack Manager: "Use with this model" and "Remove from model" (W-13 §3.4).
// A reference carries the stored pack's identity and its stamped checksum,
// projected to the subset the document validator checks; rule content never
// enters the model. A reference never makes a check pass. References whose
// pack the refreshed local list no longer holds are named
// (MODEL-RULE-PACK-REF-UNRESOLVED, a warning).
import type { LocalRulePackIndexEntry, RulePackOpenRoute } from "../../services/rulePackService";
import { RECORD_FAMILIES, disabledBy, recordIntent, rulePackReferenceFromDocument } from "./modelDataIntents";
import { useModelDataQueue, type ModelDataQueueProps } from "./useModelDataQueue";
import { RULE_PACK_REF_UNRESOLVED } from "../../services/rulePackRefDiagnostics";

export { RULE_PACK_REF_UNRESOLVED };

export function RulePackModelReferences(props: ModelDataQueueProps & {
  entries: LocalRulePackIndexEntry[];
  listRefreshed: boolean;
  openPack: (entry: LocalRulePackIndexEntry) => Promise<RulePackOpenRoute>;
}) {
  const { model, entries, listRefreshed } = props;
  const references = model.rule_pack_refs ?? [];
  const { busy, error, notice, queue } = useModelDataQueue(props);
  const family = RECORD_FAMILIES.rulePackRef;
  const unresolved = listRefreshed
    ? references.filter((reference) => !entries.some((entry) => entry.rule_pack_id === reference.rule_pack_id))
    : [];

  async function use(entry: LocalRulePackIndexEntry) {
    await queue(async (canonicalize) => {
      const route = await props.openPack(entry);
      if (route.route === "unavailable_browser_preview") throw new Error(route.diagnostic);
      if (!route.envelope) throw new Error(`No stored rule pack matched ${entry.rule_pack_id}; refresh the list and retry.`);
      return recordIntent(canonicalize, family, "create", rulePackReferenceFromDocument(route.envelope.document));
    });
  }

  return (
    <div className="report-list" data-testid="rule-pack-model-references">
      <strong>Used with this model</strong>
      <small>A reference records which pack the model uses; it never makes a check pass.</small>
      {references.length === 0 ? <small data-testid="rule-pack-model-references-empty">No rule pack is referenced by this model.</small> : null}
      {references.map((reference) => (
        <article className="operation-record" key={reference.rule_pack_id} data-testid={`rule-pack-model-ref-${reference.rule_pack_id}`}>
          <span>{reference.name} ({reference.rule_pack_id} v{reference.rule_pack_version}) sha256 {reference.checksum.value.slice(0, 12)}…</span>
          <button type="button" data-testid={`rule-pack-remove-from-model-${reference.rule_pack_id}`} {...disabledBy(busy)}
            onClick={() => void queue((canonicalize) => recordIntent(canonicalize, family, "delete", reference, reference))}>
            Remove from model
          </button>
        </article>
      ))}
      {unresolved.map((reference) => (
        <small key={reference.rule_pack_id} role="note" data-testid={`rule-pack-ref-unresolved-${reference.rule_pack_id}`}>
          {RULE_PACK_REF_UNRESOLVED} (warning): {reference.rule_pack_id} is referenced by the model but is not in the local store.
        </small>
      ))}
      {entries.map((entry) => {
        const used = references.some((reference) => reference.rule_pack_id === entry.rule_pack_id);
        return (
          <button key={JSON.stringify([entry.project_id, entry.rule_pack_id])} type="button" data-testid={`rule-pack-use-with-model-${entry.project_id}-${entry.rule_pack_id}`} {...disabledBy(busy, [used, "Already referenced by this model."])}
            onClick={() => void use(entry)}>
            Use with this model: {entry.rule_pack_name} ({entry.project_id})
          </button>
        );
      })}
      {error ? <p role="alert">{error}</p> : null}
      {notice ? <p role="status">{notice}</p> : null}
    </div>
  );
}
