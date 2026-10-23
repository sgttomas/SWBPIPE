// "Apply to model" for an opened stored library (W-17, DEL-03-01 Design
// apply-library-records.md §3): choose a record, insert it as a new entity or
// apply it to the selected one, preview what is copied and what is not, then
// queue the batch for the existing review (validate, diff, apply, one undo).
// Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081):
// "Source review" describes source-data review only.
import { useMemo, useRef, useState } from "react";
import type { OperationBatch } from "../../services/operationBatchService";
import type { LibraryKind } from "../../services/libraryImportService";
import type { UnitCatalog } from "../../services/unitCatalogService";
import type { EntityRef, PreviewModel } from "../../types";
import {
  buildComponentApply, buildMaterialApply, buildMaterialInsert, buildSectionApply, buildSectionInsert,
  defaultMaterialMode, type LibraryApplyPlan, type MaterialMode, type ReplaceOption
} from "./libraryApply";

type Json = Record<string, unknown>;
const RECORDS: Record<Exclude<LibraryKind, "hanger">, { records: string; id: string }> = {
  material: { records: "material_records", id: "material_id" },
  section: { records: "section_records", id: "section_id" },
  component: { records: "component_records", id: "component_id" }
};
const COLLECTION = { material: "materials", section: "sections", component: "components" } as const;

export type LibraryApplyTarget = { recordId: string; entityId: string };

export function LibraryApplySection(props: {
  model: PreviewModel;
  libraryKind: LibraryKind;
  document: Json;
  selection: EntityRef | null;
  catalog: UnitCatalog | null;
  busy: boolean;
  preset?: LibraryApplyTarget | null;
  /** Re-open and re-validate the stored library at prepare time, so the plan
   * (and its `library_version`) is built from the current stored record. */
  reopen?: () => Promise<Json>;
  onQueueBatch: (batch: OperationBatch) => void;
}) {
  const { model, libraryKind, document, selection, catalog } = props;
  const kind = libraryKind === "hanger" ? null : libraryKind;
  const records = useMemo(() => {
    if (!kind) return [];
    const list = document[RECORDS[kind].records];
    return (Array.isArray(list) ? list : []).map((record) => record as Json)
      .map((record) => ({ id: String(record[RECORDS[kind].id] ?? ""), name: String(record.name ?? "") }));
  }, [document, kind]);
  const [recordId, setRecordId] = useState(props.preset?.recordId ?? records[0]?.id ?? "");
  const selectedTarget = props.preset?.entityId ?? (selection && kind && selection.type === kind ? selection.id : "");
  const [action, setAction] = useState<"insert" | "apply">(props.preset || kind === "component" ? "apply" : "insert");
  const [newId, setNewId] = useState("");
  const targetEntity = kind && selectedTarget
    ? (model[COLLECTION[kind]] as Array<{ id: string }> | undefined)?.find((entity) => entity.id === selectedTarget)
    : undefined;
  const [mode, setMode] = useState<MaterialMode | null>(null);
  const effectiveMode: MaterialMode = mode ?? (action === "apply" && kind === "material"
    ? defaultMaterialMode(targetEntity as Parameters<typeof defaultMaterialMode>[0]) : "e_nu");
  const [replace, setReplace] = useState<ReplaceOption>("none");
  // A plan belongs to the inputs it was prepared from: the model, the opened
  // library document, the selection and target, and every choice below. Any
  // change makes it stale, and a preparation still in flight is discarded.
  const basis = [model, document, selection?.type, selection?.id, targetEntity?.id, recordId, action, newId, effectiveMode, replace] as const;
  const [plan, setPlan] = useState<{ plan: LibraryApplyPlan; basis: typeof basis } | null>(null);
  const [error, setError] = useState("");
  const [pending, setPending] = useState(false);
  const latestBasis = useRef(basis);
  latestBasis.current = basis;
  const sameBasis = (left: typeof basis, right: typeof basis) => left.every((value, index) => value === right[index]);

  if (!kind) {
    return (
      <div className="report-list" data-testid="library-apply-section">
        <strong>Apply to model</strong>
        <small>Hanger records are applied to supports in Supports, "Select an imported hanger".</small>
      </div>
    );
  }
  const reset = () => { setPlan(null); setError(""); };
  const canInsert = kind !== "component";
  const current = plan && sameBasis(plan.basis, basis) ? plan.plan : null;

  async function preview() {
    reset();
    setPending(true);
    const preparedFrom = basis;
    try {
      const stored = props.reopen ? await props.reopen() : document;
      const base = { model, document: stored, recordId };
      let next: LibraryApplyPlan;
      if (action === "insert") {
        next = kind === "material"
          ? await buildMaterialInsert({ ...base, newId: newId.trim(), mode: effectiveMode, catalog })
          : await buildSectionInsert({ ...base, newId: newId.trim() });
      } else {
        if (!targetEntity) throw new Error(`Select a ${kind} in the model to apply this record to.`);
        next = kind === "material"
          ? await buildMaterialApply({ ...base, targetId: targetEntity.id, mode: effectiveMode, replace, catalog })
          : kind === "section"
            ? await buildSectionApply({ ...base, targetId: targetEntity.id, replace })
            : await buildComponentApply({ ...base, targetId: targetEntity.id, replace });
      }
      if (sameBasis(preparedFrom, latestBasis.current)) setPlan({ plan: next, basis: preparedFrom });
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setPending(false);
    }
  }

  const disabled = props.busy || pending;
  return (
    <div className="report-list" data-testid="library-apply-section">
      <strong>Apply to model</strong>
      <small>Copies the listed values, their units and a provenance record into the model. Library metadata and other records stay in the local store.</small>
      <label htmlFor="library-apply-record">Library record</label>
      <select id="library-apply-record" data-testid="library-apply-record" value={recordId} disabled={disabled}
        onChange={(event) => { setRecordId(event.target.value); reset(); }}>
        {records.map((record) => <option key={record.id} value={record.id}>{record.name} ({record.id})</option>)}
      </select>
      <fieldset disabled={disabled}>
        <legend>Action</legend>
        {canInsert ? (
          <label><input type="radio" name="library-apply-action" data-testid="library-apply-insert" checked={action === "insert"}
            onChange={() => { setAction("insert"); reset(); }} />Insert as new {kind}</label>
        ) : null}
        <label><input type="radio" name="library-apply-action" data-testid="library-apply-existing" checked={action === "apply"}
          onChange={() => { setAction("apply"); reset(); }} />
          Apply to the selected {kind}{targetEntity ? `: ${targetEntity.id}` : " (select one in the model)"}</label>
      </fieldset>
      {action === "insert" ? (
        <label>New {kind} ID<input data-testid="library-apply-new-id" value={newId} disabled={disabled}
          onChange={(event) => { setNewId(event.target.value); reset(); }} /></label>
      ) : null}
      {kind === "material" ? (
        <label>Material mode
          <select data-testid="library-apply-mode" value={effectiveMode} disabled={disabled}
            onChange={(event) => { setMode(event.target.value as MaterialMode); reset(); }}>
            <option value="e_nu">E/ν: E, ν and G with the isotropic E/ν basis (G stored, or derived as E/(2(1+ν)))</option>
            <option value="e_g">E/G: E and G, no basis (refused when the record holds ν)</option>
          </select>
        </label>
      ) : null}
      {action === "apply" ? (
        <label>Values the record does not supply
          <select data-testid="library-apply-replace" value={replace} disabled={disabled}
            onChange={(event) => { setReplace(event.target.value as ReplaceOption); reset(); }}>
            <option value="none">Keep them (they keep their previous attribution)</option>
            {kind === "component" ? <option value="group">Replace group: remove a written group's unsupplied values</option> : null}
            <option value="all">Replace all values: remove every value the record does not supply</option>
          </select>
        </label>
      ) : null}
      <button type="button" data-testid="library-apply-preview" disabled={disabled || !recordId} onClick={() => void preview()}>
        Preview library {action === "insert" ? "insert" : "apply"}
      </button>
      {error ? <p role="alert" data-testid="library-apply-error">{error}</p> : null}
      {current ? (
        <div data-testid="library-apply-preview-result">
          <table>
            <caption>Copied</caption>
            <thead><tr><th>Member</th><th>Value</th><th>Unit</th><th>Source</th></tr></thead>
            <tbody>{current.copied.map((row) => (
              <tr key={row.member} data-testid={`library-apply-copied-${row.member}`}><td>{row.member}</td><td>{row.value}</td><td>{row.unit}</td><td>{row.source}</td></tr>
            ))}</tbody>
          </table>
          <table>
            <caption>Not copied</caption>
            <thead><tr><th>Source</th><th>Reason</th></tr></thead>
            <tbody>{current.notCopied.map((row) => (
              <tr key={`${row.source}:${row.reason}`} data-testid={`library-apply-not-copied-${row.source}`}><td>{row.source}</td><td>{row.reason}</td></tr>
            ))}</tbody>
          </table>
          {current.removed.length ? <p data-testid="library-apply-removed">Removed: {current.removed.join(", ")}</p> : null}
          {current.dependentPipes.length ? (
            <p data-testid="library-apply-dependents">Bound pipes updated with the section: {current.dependentPipes.join(", ")}</p>
          ) : null}
          <p data-testid="library-apply-provenance">
            Source: {current.provenanceRecord.source_name}; redistribution {current.provenanceRecord.redistribution_status};
            Source review {current.provenanceRecord.review_status}.
            {current.provenanceRecord.verification_note ? ` ${current.provenanceRecord.verification_note}` : ""}
          </p>
          <button type="button" data-testid="library-apply-queue" disabled={disabled}
            onClick={() => { props.onQueueBatch(current.batch); setPlan(null); }}>
            Queue for review
          </button>
          <small>Review validates the batch, shows every changed row (including bound pipes) and applies it with one undo checkpoint.</small>
        </div>
      ) : null}
    </div>
  );
}
