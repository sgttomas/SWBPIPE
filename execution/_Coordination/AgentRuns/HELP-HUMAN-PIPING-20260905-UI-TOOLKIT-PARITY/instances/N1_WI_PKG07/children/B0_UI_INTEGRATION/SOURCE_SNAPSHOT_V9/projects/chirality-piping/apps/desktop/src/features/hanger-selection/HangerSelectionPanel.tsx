import { useState } from "react";
import { listLocalLibraries, openLocalLibrary, saveLocalLibrary, validateLibraryImport, type LocalLibraryIndexEntry } from "../../services/libraryImportService";
import type { OperationBatch } from "../../services/operationBatchService";
import { buildHangerSelectionBatch, type HangerDocument } from "./hangerSelection";
import { usePreparation, type WorkflowProps } from "../offline-proposal-intake/workflowSupport";

export function HangerSelectionPanel(props: WorkflowProps) {
  const [libraries, setLibraries] = useState<LocalLibraryIndexEntry[]>([]);
  const [document, setDocument] = useState<HangerDocument | null>(null);
  const [input, setInput] = useState({ libraryId: "", recordId: "", supportId: "", label: "", node: "", create: false, restraints: [] as string[] });
  const [notice, setNotice] = useState("");
  const [review, setReview] = useState<{ batch: OperationBatch; inputs: typeof input; model: WorkflowProps["model"]; selection: string; epoch?: number; revision: number } | null>(null);
  const state = usePreparation(props, input);
  const disabledReason = props.busy ? "Wait for the current operation to finish before changing this draft."
    : state.pending ? "Preparing the hanger library or selection. Wait for it to finish." : undefined;
  const current = review && review.revision === state.revision && review.inputs === input && review.model === props.model &&
    review.selection === `${props.selection.type}:${props.selection.id}` && review.epoch === props.requestEpoch && !props.busy;
  const selected = document?.hanger_records.find(record => record.hanger_id === input.recordId);
  async function refresh() {
    await state.run(async () => {
      const result = await listLocalLibraries(props.model.project.id);
      if (result.route !== "tauri_backend") throw new Error(result.diagnostic);
      return result.entries.filter(entry => entry.library_kind === "hanger");
    }, setLibraries);
  }
  async function open(libraryId: string) {
    await state.run(async () => {
      const result = await openLocalLibrary(props.model.project.id, "hanger", libraryId);
      if (result.route !== "tauri_backend") throw new Error(result.diagnostic);
      if (!result.envelope) throw new Error("This library no longer exists.");
      if (!result.envelope.validation.accepted || result.envelope.validation.has_blocking_findings)
        throw new Error("The stored hanger library has blocking import findings.");
      return result.envelope.document as unknown as HangerDocument;
    }, next => { setDocument(next); setInput({ ...input, libraryId, recordId: "" }); setReview(null); });
  }
  async function importFile(file: File) {
    await state.run(async () => {
      const payload: unknown = JSON.parse(await file.text());
      if (!payload || typeof payload !== "object" || Array.isArray(payload)) throw new Error("Choose a hanger library JSON object.");
      const result = await saveLocalLibrary(props.model.project.id, "hanger", payload as Record<string, unknown>);
      if (result.route !== "tauri_backend") throw new Error(result.diagnostic);
      if (!result.result.stored || !result.result.validation.accepted || result.result.validation.has_blocking_findings)
        throw new Error(result.result.validation.findings.map(f => `${f.message} ${f.remediation}`).join("\n") || "The library was not stored.");
      return result.result;
    }, result => {
      setDocument(result.document as unknown as HangerDocument);
      setInput({ ...input, libraryId: result.library_id, recordId: "" });
      setNotice(result.validation.findings.map(f => f.message).join("\n") || "Private library imported locally.");
      setReview(null);
    });
  }
  async function prepare() {
    setReview(null);
    await state.run(async () => {
      if (!input.libraryId || !input.recordId) throw new Error("Choose a local hanger library and record.");
      // Reopen and validate the actual current stored document before retaining the selected snapshot.
      const result = await openLocalLibrary(props.model.project.id, "hanger", input.libraryId);
      if (result.route !== "tauri_backend") throw new Error(result.diagnostic);
      if (!result.envelope) throw new Error("The selected library no longer exists.");
      const checked = await validateLibraryImport(result.envelope.document, "hanger", "private");
      if (checked.route !== "tauri_backend") throw new Error(checked.diagnostic);
      if (!checked.validation.accepted || checked.validation.has_blocking_findings)
        throw new Error(checked.validation.findings.map(f => `${f.message} ${f.remediation}`).join("\n") || "The import is blocked.");
      const currentDocument = result.envelope.document as unknown as HangerDocument;
      const batch = await buildHangerSelectionBatch(props.model, currentDocument, input.recordId, {
        supportId: input.supportId, restraints: input.restraints,
        ...(input.create ? { newSupport: { label: input.label, node: input.node } } : {})
      });
      return { batch, currentDocument, findings: checked.validation.findings };
    }, result => {
      setDocument(result.currentDocument);
      setNotice(result.findings.map(f => f.message).join("\n"));
      setReview({ batch: result.batch, inputs: input, model: props.model,
        selection: `${props.selection.type}:${props.selection.id}`, epoch: props.requestEpoch, revision: state.revision });
    });
  }
  return <section id="hanger-selection" tabIndex={-1} aria-label="Imported hanger selection">
    <h3>Select an imported hanger</h3>
    {disabledReason && <p id="hanger-selection-disabled-reason" role="status">{disabledReason}</p>}
    <p>Library import and storage are available in the native desktop application. Browser preview reports this route as unavailable.</p>
    <p>Choose a record from your local library and supply the support location and restraint directions. Selection does not size a hanger or establish solve readiness.</p>
    <fieldset disabled={props.busy || state.pending} aria-describedby={disabledReason ? "hanger-selection-disabled-reason" : undefined} title={disabledReason}><legend>Local hanger library</legend>
      <button type="button" onClick={() => void refresh()}>Refresh hanger libraries</button>
      <label>Import hanger library<input type="file" accept=".json,application/json" onChange={event => {
        const file = event.target.files?.[0]; if (file) void importFile(file); event.target.value = "";
      }} /></label>
      <label>Hanger library<select value={input.libraryId} onChange={event => void open(event.target.value)}>
        <option value="">Choose a library</option>
        {libraries.map(lib => <option key={lib.library_id} value={lib.library_id}>{lib.library_name} ({lib.library_id})</option>)}
        {document && !libraries.some(lib => lib.library_id === document.hanger_library.library_id) &&
          <option value={document.hanger_library.library_id}>{document.hanger_library.name}</option>}
      </select></label>
      <label>Hanger record<select value={input.recordId} onChange={event => setInput({ ...input, recordId: event.target.value })}>
        <option value="">Choose a record</option>{document?.hanger_records.map(record =>
          <option key={record.hanger_id} value={record.hanger_id}>{record.name} ({record.hanger_id})</option>)}
      </select></label>
    </fieldset>
    {selected && <div><p>{selected.name}: {selected.hanger.hanger_type.replaceAll("_", " ")}</p>
      <p>Source: {selected.hanger.source_reference}</p>
      <dl>{Object.entries(selected.provenance).map(([key, value]) => <div key={key}><dt>{key.replaceAll("_", " ")}</dt><dd>{String(value)}</dd></div>)}</dl>
      <details><summary>Imported values and all source provenance</summary><pre>{JSON.stringify({ library: document?.hanger_library, record: selected }, null, 2)}</pre></details>
    </div>}
    <fieldset disabled={props.busy || state.pending} aria-describedby={disabledReason ? "hanger-selection-disabled-reason" : undefined} title={disabledReason}><legend>Support assignment</legend>
      <label><input type="checkbox" checked={input.create} onChange={event => setInput({ ...input, create: event.target.checked, supportId: "" })} />Create a new support</label>
      {input.create ? <>
        <label>New hanger support ID<input value={input.supportId} onChange={event => setInput({ ...input, supportId: event.target.value })} /></label>
        <label>New hanger support label<input value={input.label} onChange={event => setInput({ ...input, label: event.target.value })} /></label>
        <label>Hanger support node<select value={input.node} onChange={event => setInput({ ...input, node: event.target.value })}>
          <option value="">Choose a node</option>{props.model.nodes.map(node => <option key={node.id}>{node.id}</option>)}
        </select></label>
      </> : <label>Existing hanger support<select value={input.supportId} onChange={event => setInput({ ...input, supportId: event.target.value })}>
        <option value="">Choose a support</option>{props.model.supports.map(support => <option key={support.id} value={support.id}>{support.label} ({support.id})</option>)}
      </select></label>}
      <fieldset><legend>Confirmed translational restraints</legend>{["UX", "UY", "UZ"].map(dof => <label key={dof}>
        <input type="checkbox" checked={input.restraints.includes(dof)} onChange={event => setInput({ ...input,
          restraints: event.target.checked ? [...input.restraints, dof] : input.restraints.filter(value => value !== dof) })} />{dof}
      </label>)}</fieldset>
      <p>This replacement clears existing top-level stiffness and nonlinear settings. Missing fields remain absent.</p>
      <button type="button" onClick={() => void prepare()}>Prepare hanger selection</button>
    </fieldset>
    {state.error && <p role="alert">{state.error}</p>}{notice && <p role="status">{notice}</p>}
    {current && review && <div><h4>Review configuration replacement</h4>
      <p>Previous top-level stiffness and nonlinear settings will be removed. Review the complete before and after configuration.</p>
      <details open><summary>Before and after</summary><pre>{review.batch.operations[0].change.before}</pre><pre>{review.batch.operations[0].change.after}</pre></details>
      <button type="button" disabled={state.pending} aria-describedby={disabledReason ? "hanger-selection-disabled-reason" : undefined} title={disabledReason} onClick={() => { props.onQueueBatch(review.batch); setReview(null); }}>Queue hanger selection for review</button>
    </div>}
    {review && !current && <p role="status">This selection draft is stale. Prepare it again.</p>}
  </section>;
}
