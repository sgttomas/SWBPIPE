import { useMemo, useState } from "react";
import { TextField } from "../rich-authoring/formSupport";
import { VirtualMultiTargetPicker } from "../workspace/VirtualTargetPicker";
import { VirtualList } from "../workspace/VirtualList";
import { batchId, issue, useBatchQueue, type BatchFormProps } from "./batchFormSupport";
import { buildGeometryBatch, emptyGeometryDraft, emptyIdentity, geometryPayload, selectedGeometry, type GeometryDraft, type NewIdentity } from "./geometryDraft";
export { buildGeometryBatch, geometryPayload } from "./geometryDraft";
export type { GeometryDraft } from "./geometryDraft";

function IdentityFields({ label, value, onChange }: { label: string; value: NewIdentity; onChange: (next: NewIdentity) => void }) {
  return <fieldset>
    <legend>{label}</legend>
    {(["id", "label", "provenance"] as const).map(key => <TextField
      key={key}
      label={`${label} ${key === "id" ? "ID" : key}`}
      value={value[key]}
      onChange={next => onChange({ ...value, [key]: next })}
    />)}
  </fieldset>;
}
export function GeometryToolsPanel(props: BatchFormProps & { selectedPipeRefs?: readonly string[] }) {
  const [draft, setDraft] = useState(emptyGeometryDraft);
  const [focusedCopyKey, setFocusedCopyKey] = useState<string | null>(null);
  const state = useBatchQueue(props, draft);
  const invalid = useMemo(() => issue(() => geometryPayload(props.model, draft)), [draft, props.model]);
  const pipeOptions = useMemo(() => props.model.pipe_segments.map((pipe) => ({
    value: pipe.id,
    label: pipe.label || pipe.id,
    keywords: [pipe.from, pipe.to]
  })), [props.model.pipe_segments]);
  const copyRows = useMemo(() => [
    ...draft.copyNodes.map((value) => ({ kind: "copyNodes" as const, value })),
    ...draft.copyPipes.map((value) => ({ kind: "copyPipes" as const, value }))
  ], [draft.copyNodes, draft.copyPipes]);
  const focusedCopyIndex = focusedCopyKey === null
    ? null
    : copyRows.findIndex((row) => `${row.kind}:${row.value.source_ref}` === focusedCopyKey);
  const set = <K extends keyof GeometryDraft>(key: K, value: GeometryDraft[K]) => setDraft(previous => ({ ...previous, [key]: value }));
  function selectPipes(refs: string[]) {
    // Populate source identities only. New IDs, labels and provenance stay blank.
    let selected: ReturnType<typeof selectedGeometry> = { nodes: [], pipe_segments: [] };
    try { if (refs.length) selected = selectedGeometry(props.model, refs); }
    catch { /* Retain selected IDs; the visible draft validation explains missing source records. */ }
    setDraft(previous => {
      const priorNodes = new Map(previous.copyNodes.map((row) => [row.source_ref, row]));
      const priorPipes = new Map(previous.copyPipes.map((row) => [row.source_ref, row]));
      return { ...previous, pipeRefs: refs,
        copyNodes: selected.nodes.map(n => priorNodes.get(n.id) ?? { source_ref: n.id, ...emptyIdentity() }),
        copyPipes: selected.pipe_segments.map(p => priorPipes.get(p.id) ?? { source_ref: p.id, ...emptyIdentity() })
      };
    });
  }
  return <section id="geometry-tools" tabIndex={-1} aria-labelledby="geometry-tools-title">
    <h2 id="geometry-tools-title">Geometry tools</h2>
    <p>Split a straight run or transform explicitly selected geometry. All changes enter one batch for validation, preview and separate application.</p>
    <p>Split preserves endpoint nodal loads and supports but blocks components at either endpoint, pipe loads and wind exposure. Transforms block attached supports, components, loads, wind and unknown references. In-place transforms also block unselected incident pipes. Rotation and mirror require existing explicit local Y orientation. The engine reports exact blockers in the batch preview.</p>
    <fieldset disabled={state.preparing}>
      <legend>Geometry draft</legend>
      <TextField label="Geometry tool" value={draft.action} choices={["split", "translate", "rotate", "mirror"]} onChange={value => { state.cancel(); setDraft({ ...emptyGeometryDraft(), action: value as GeometryDraft["action"] }); }} />
      <fieldset>
        <legend>Source pipe runs</legend>
        <VirtualMultiTargetPicker
          label="Source pipe runs"
          onChange={(values) => selectPipes([...values])}
          options={pipeOptions}
          testId="geometry-source-pipes"
          values={draft.pipeRefs}
        />
        <button
          disabled={!props.selectedPipeRefs?.length}
          title={props.selectedPipeRefs?.length ? "Freeze the current selected pipes into this geometry draft." : "Select one or more pipes first."}
          onClick={() => selectPipes([...(props.selectedPipeRefs ?? [])])}
          type="button"
        >Use selected pipes</button>
        <p>{draft.pipeRefs.length > 0
          ? `Frozen source snapshot: ${draft.pipeRefs.length} pipe${draft.pipeRefs.length === 1 ? "" : "s"}.`
          : "Select one or more pipes, then freeze them into this draft."}</p>
      </fieldset>
      {draft.action === "split" ? <>
        <TextField label="Split fraction" value={draft.fraction} onChange={value => set("fraction", value)} />
        <p>Fraction measured from the source start node; original pipe ID remains on the first half. Coordinates use entered project length units.</p>
        <IdentityFields label="New node" value={draft.newNode} onChange={value => set("newNode", value)} />
        <IdentityFields label="New pipe" value={draft.newPipe} onChange={value => set("newPipe", value)} />
      </> : draft.action && <>
        <TextField label="Transform mode" value={draft.mode} choices={["copy", "in_place"]} onChange={value => set("mode", value as GeometryDraft["mode"])} />
        <TextField label="Geometry coordinate system" value={draft.coordinateSystem} choices={["global"]} onChange={value => set("coordinateSystem", value)} />
        <fieldset>
          <legend>{draft.action === "translate" ? "Translation vector" : draft.action === "rotate" ? "Rotation pivot" : "Mirror plane origin"}</legend>
          {(["x", "y", "z", "unit"] as const).map(key => <TextField key={key}
            label={`Geometry ${key === "unit" ? "length unit" : key.toUpperCase()}`}
            value={draft.vector[key]} onChange={value => set("vector", { ...draft.vector, [key]: value })} />)}
        </fieldset>
        {draft.action !== "translate" && <fieldset>
          <legend>{draft.action === "rotate" ? "Rotation axis direction" : "Mirror plane normal"}</legend>
          {(["x", "y", "z"] as const).map(key => <TextField key={key} label={`Direction ${key.toUpperCase()}`}
            value={draft.direction[key]} onChange={value => set("direction", { ...draft.direction, [key]: value })} />)}
        </fieldset>}
        {draft.action === "rotate" && <>
          <TextField label="Rotation angle" value={draft.angle.value} onChange={value => set("angle", { ...draft.angle, value })} />
          <TextField label="Rotation angle unit" value={draft.angle.unit} onChange={unit => set("angle", { ...draft.angle, unit })} />
        </>}
        {draft.mode === "copy" && copyRows.length > 0 ? <section aria-label="Copied entity identities">
          <p>{copyRows.length} copied entity identities. Edit only the focused virtual row.</p>
          <VirtualList
            height={360}
            itemKey={(row) => `${row.kind}:${row.value.source_ref}`}
            items={copyRows}
            pinIndex={focusedCopyIndex === -1 ? null : focusedCopyIndex}
            renderItem={(row) => <div
              onBlurCapture={(event) => { if (!event.currentTarget.contains(event.relatedTarget as Node | null)) setFocusedCopyKey(null); }}
              onFocusCapture={() => setFocusedCopyKey(`${row.kind}:${row.value.source_ref}`)}
            ><IdentityFields
              label={`${row.kind === "copyNodes" ? "Copied node" : "Copied pipe"} ${row.value.source_ref}`}
              value={row.value}
              onChange={(value) => set(row.kind, draft[row.kind].map((old) => old.source_ref === row.value.source_ref ? { ...value, source_ref: old.source_ref } : old))}
            /></div>}
            rowHeight={180}
            testId="geometry-copy-identities"
          />
        </section> : null}
      </>}
    </fieldset>
    {invalid && <p id="geometry-draft-reason" role="status">{invalid}</p>}
    <button type="button" disabled={state.preparing || Boolean(invalid)} aria-describedby={invalid ? "geometry-draft-reason" : undefined}
      onClick={() => void state.queue(() => buildGeometryBatch(props.model, draft, batchId("geometry")))}>
      Queue geometry batch
    </button>
    <button type="button" onClick={() => { state.cancel("Geometry draft cleared. No model changes applied."); setDraft(emptyGeometryDraft()); }}>Clear / cancel geometry draft</button>
    {state.feedback && <p role="status">{state.feedback}</p>}
  </section>;
}
