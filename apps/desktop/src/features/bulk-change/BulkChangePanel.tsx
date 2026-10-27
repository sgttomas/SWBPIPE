// B2 (DEL-07-02 Design `bulk-change.md`): the Bulk change panel. It previews the plan for the
// current selection and queues it as one batch through the session's existing batch queue.
import { useEffect, useMemo, useRef, useState } from "react";
import type { OperationBatch } from "../../services/operationBatchService";
import { canonicalJsonString } from "../../services/hashService";
import type { EntityRef, PreviewModel } from "../../types";
import {
  BULK_CHANGE_LABELS,
  DOFS,
  SUPPORT_FAMILIES,
  TARGET_SUPPORT_FAMILIES,
  buildBulkChangeBatch,
  familyLabel,
  planBlockedReason,
  planBulkChange,
  type BulkChangeKind,
  type BulkChangeRequest,
  type Dof,
  type ReasonEntry
} from "./bulkChange";

type Props = {
  model: PreviewModel;
  /** The ordered selection the change acts on. */
  scope: readonly EntityRef[];
  onQueueBatch: (batch: OperationBatch) => void | Promise<void>;
  busy?: boolean;
  /** Each new value moves focus to the panel ("Change selected…"). */
  focusRequest?: number;
  canonicalize?: (value: unknown) => Promise<string>;
};

const KINDS = Object.keys(BULK_CHANGE_LABELS) as BulkChangeKind[];

export function BulkChangePanel({ model, scope, onQueueBatch, busy = false, focusRequest = 0, canonicalize = canonicalJsonString }: Props) {
  const panel = useRef<HTMLElement>(null);
  const [kind, setKind] = useState<BulkChangeKind>("support-kind");
  const [sectionId, setSectionId] = useState("");
  const [materialId, setMaterialId] = useState("");
  const [fromFamily, setFromFamily] = useState("guide");
  const [toFamily, setToFamily] = useState("");
  const [restraintMode, setRestraintMode] = useState<"keep" | "enter">("enter");
  const [restraints, setRestraints] = useState<readonly Dof[]>([]);
  const [removeFamily, setRemoveFamily] = useState("guide");
  const [radius, setRadius] = useState("");
  const [radiusUnit, setRadiusUnit] = useState(model.project.units.length ?? "");
  const [tag, setTag] = useState("");
  const [queuing, setQueuing] = useState(false);
  const [message, setMessage] = useState<{ tone: "status" | "alert"; text: string } | null>(null);
  const sequence = useRef(0);
  const latest = useRef({ model, scope });
  latest.current = { model, scope };

  useEffect(() => {
    if (focusRequest > 0) panel.current?.focus();
  }, [focusRequest]);

  const request: BulkChangeRequest = useMemo(() => {
    switch (kind) {
      case "section": return { kind, sectionId };
      case "material": return { kind, materialId };
      case "support-kind": return { kind, fromFamily, toFamily, restraints: restraintMode === "keep" ? "keep" : restraints };
      case "support-remove": return { kind, family: removeFamily };
      case "bend-radius": return { kind, value: radius, unit: radiusUnit };
      case "tag": return { kind, tag };
    }
  }, [kind, sectionId, materialId, fromFamily, toFamily, restraintMode, restraints, removeFamily, radius, radiusUnit, tag]);
  const plan = useMemo(() => planBulkChange(model, scope, request), [model, scope, request]);
  const blocked = scope.length === 0 ? "Select the entities to change first." : busy || queuing ? "Another operation is running." : planBlockedReason(plan);

  async function queue() {
    if (blocked) return;
    const captured = { model, scope };
    setQueuing(true);
    setMessage(null);
    try {
      const batch = await buildBulkChangeBatch(model, scope, request, canonicalize, `bulk-${kind}-${++sequence.current}`);
      if (latest.current.model !== captured.model || latest.current.scope !== captured.scope) {
        setMessage({ tone: "alert", text: "The model or the selection changed. Review the preview and queue again." });
        return;
      }
      await onQueueBatch(batch);
      setMessage({ tone: "status", text: `${batch.operations.length} changes queued as one batch for review.` });
    } catch (error) {
      setMessage({ tone: "alert", text: `Nothing was queued: ${error instanceof Error ? error.message : String(error)}` });
    } finally {
      setQueuing(false);
    }
  }

  const familyOptions = [...SUPPORT_FAMILIES, ""].map((family) => <option key={family || "none"} value={family}>{familyLabel(family)}</option>);
  return (
    <section className="panel" id="bulk-change" ref={panel} tabIndex={-1} aria-label="Bulk change" data-testid="bulk-change">
      <h3>Bulk change</h3>
      <p data-testid="bulk-change-scope">Acts on the selection: {scope.length} selected {scope.length === 1 ? "entity" : "entities"}. Select a range in the tree or grid with Shift, or add with Ctrl or Cmd.</p>
      <label>Change
        <select aria-label="Change" data-testid="bulk-change-kind" value={kind} onChange={(event) => { setKind(event.target.value as BulkChangeKind); setMessage(null); }}>
          {KINDS.map((item) => <option key={item} value={item}>{BULK_CHANGE_LABELS[item]}</option>)}
        </select>
      </label>
      {kind === "section" ? (
        <label>Shared section
          <select aria-label="Shared section" data-testid="bulk-change-section" value={sectionId} onChange={(event) => setSectionId(event.target.value)}>
            <option value="">Choose a section</option>
            {(model.sections ?? []).filter((section) => section.section_type === "pipe").map((section) => <option key={section.id} value={section.id}>{section.name} ({section.id})</option>)}
          </select>
        </label>
      ) : null}
      {kind === "material" ? (
        <label>Material
          <select aria-label="Material" data-testid="bulk-change-material" value={materialId} onChange={(event) => setMaterialId(event.target.value)}>
            <option value="">Choose a material</option>
            {(model.materials ?? []).map((material) => <option key={material.id} value={material.id}>{material.id}</option>)}
          </select>
        </label>
      ) : null}
      {kind === "support-kind" ? (
        <fieldset>
          <legend>Replace a support kind</legend>
          <label>Replace
            <select aria-label="Replace kind" data-testid="bulk-change-from-family" value={fromFamily} onChange={(event) => setFromFamily(event.target.value)}>{familyOptions}</select>
          </label>
          <label>With
            <select aria-label="With kind" data-testid="bulk-change-to-family" value={toFamily} onChange={(event) => setToFamily(event.target.value)}>
              <option value="">Choose a kind</option>
              {TARGET_SUPPORT_FAMILIES.map((family) => <option key={family} value={family}>{familyLabel(family)}</option>)}
            </select>
          </label>
          <fieldset>
            <legend>Restraints written</legend>
            <label><input type="radio" name="bulk-change-restraints" data-testid="bulk-change-restraints-keep" checked={restraintMode === "keep"} onChange={() => setRestraintMode("keep")} /> Keep each support's restraints</label>
            <label><input type="radio" name="bulk-change-restraints" data-testid="bulk-change-restraints-enter" checked={restraintMode === "enter"} onChange={() => setRestraintMode("enter")} /> Write these restraints</label>
            {restraintMode === "enter" ? <p>Tick each restrained degree of freedom. None is filled in from the kind.</p> : null}
            {restraintMode === "enter" ? DOFS.map((dof) => (
              <label key={dof}><input type="checkbox" data-testid={`bulk-change-dof-${dof}`} checked={restraints.includes(dof)}
                onChange={(event) => setRestraints(DOFS.filter((item) => item === dof ? event.target.checked : restraints.includes(item)))} /> {dof}</label>
            )) : null}
          </fieldset>
        </fieldset>
      ) : null}
      {kind === "support-remove" ? (
        <label>Remove supports of kind
          <select aria-label="Remove kind" data-testid="bulk-change-remove-family" value={removeFamily} onChange={(event) => setRemoveFamily(event.target.value)}>{familyOptions}</select>
        </label>
      ) : null}
      {kind === "bend-radius" ? (
        <fieldset>
          <legend>Bend radius</legend>
          <label>Value<input aria-label="Bend radius value" data-testid="bulk-change-radius" inputMode="decimal" value={radius} onChange={(event) => setRadius(event.target.value)} /></label>
          <label>Unit<input aria-label="Bend radius unit" data-testid="bulk-change-radius-unit" value={radiusUnit} onChange={(event) => setRadiusUnit(event.target.value)} /></label>
          <p>Applies to bends and elbows that are not realized. A realized bend's radius forms its arc and is changed in the inspector.</p>
        </fieldset>
      ) : null}
      {kind === "tag" ? (
        <label>Tag<input aria-label="Tag" data-testid="bulk-change-tag" value={tag} onChange={(event) => setTag(event.target.value)} /></label>
      ) : null}
      <p data-testid="bulk-change-count" aria-live="polite">
        {plan.affected.length} {plan.affected.length === 1 ? "entity" : "entities"} affected; {plan.unchanged.length} unchanged; {plan.notMatched.length} not matched; {plan.refused.length} refused.
      </p>
      {plan.affected.length ? (
        <table data-testid="bulk-change-affected">
          <caption>Affected entities</caption>
          <thead><tr><th scope="col">Entity</th><th scope="col">Before</th><th scope="col">After</th></tr></thead>
          <tbody>{plan.affected.map((entry) => <tr key={`${entry.ref.type}:${entry.ref.id}`}><td>{entry.ref.id}</td><td>{entry.before}</td><td>{entry.after}</td></tr>)}</tbody>
        </table>
      ) : null}
      <ReasonList title="Refused" testId="bulk-change-refused" entries={plan.refused} />
      <ReasonList title="Unchanged" testId="bulk-change-unchanged" entries={plan.unchanged} />
      <ReasonList title="Not matched" testId="bulk-change-not-matched" entries={plan.notMatched} />
      <button type="button" data-testid="bulk-change-queue" disabled={Boolean(blocked)} title={blocked ?? undefined}
        aria-describedby={blocked ? "bulk-change-blocked" : undefined} onClick={() => void queue()}>
        Queue as one batch
      </button>
      {blocked ? <p id="bulk-change-blocked" data-testid="bulk-change-blocked">{blocked}</p> : null}
      {message ? <p role={message.tone} data-testid="bulk-change-message">{message.text}</p> : null}
      <p>Validate and apply the batch in Batch review. It is applied whole or not at all, and one Undo reverts it.</p>
    </section>
  );
}

function ReasonList({ title, testId, entries }: { title: string; testId: string; entries: readonly ReasonEntry[] }) {
  if (!entries.length) return null;
  return (
    <details data-testid={testId} open={title === "Refused"}>
      <summary>{title}: {entries.length}</summary>
      <ul>{entries.map((entry) => <li key={`${entry.ref.type}:${entry.ref.id}`}><code>{entry.ref.id}</code>: {entry.reason}</li>)}</ul>
    </details>
  );
}
