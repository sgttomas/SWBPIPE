import { useState } from "react";
import { TextField } from "../rich-authoring/formSupport";
import { batchId, issue, useBatchQueue, type BatchFormProps } from "../geometry-tools/batchFormSupport";
import { boundaryMembers, buildBoundaryBatch, DOFS, emptyBoundaryDraft, type BoundaryAssociation, type BoundaryDraft } from "./boundaryDraft";
export { boundaryMembers, buildBoundaryBatch } from "./boundaryDraft";
export type { BoundaryDraft } from "./boundaryDraft";

export function BoundaryAuthoringPanel(props: BatchFormProps) {
  const [draft, setDraft] = useState(emptyBoundaryDraft);
  const state = useBatchQueue(props, draft);
  const invalid = issue(() => boundaryMembers(props.model, draft));
  const set = <K extends keyof BoundaryDraft>(key: K, value: BoundaryDraft[K]) => setDraft(previous => ({ ...previous, [key]: value }));
  const members = props.model.supports.filter(s => Boolean((s as { boundary_association?: BoundaryAssociation }).boundary_association));
  const node = props.model.nodes.find(n => n.id === draft.node);
  return <section id="boundary-authoring" tabIndex={-1} aria-labelledby="boundary-authoring-title">
    <h2 id="boundary-authoring-title">Equipment and nozzle boundaries</h2>
    <p>Enter an equipment reference and explicit global DOF mechanics at an existing node. A single batch creates ordinary rigid and spring support members. No equipment flexibility or nozzle allowable checks are implied.</p>
    <fieldset disabled={state.preparing}>
      <legend>Boundary draft</legend>
      <TextField label="Boundary ID" value={draft.boundaryId} onChange={value => set("boundaryId", value)} />
      <TextField label="Boundary label" value={draft.label} onChange={value => set("label", value)} />
      <TextField label="Boundary kind" value={draft.kind} choices={["equipment", "equipment_nozzle"]} onChange={value => set("kind", value as BoundaryDraft["kind"])} />
      <TextField label="Equipment reference" value={draft.equipmentReference} onChange={value => set("equipmentReference", value)} />
      {draft.kind === "equipment_nozzle" && <TextField label="Nozzle reference" value={draft.nozzleReference} onChange={value => set("nozzleReference", value)} />}
      <TextField label="Boundary node" value={draft.node} choices={props.model.nodes.map(n => n.id)} onChange={value => set("node", value)} />
      {node && <p>Node {node.id}: X {node.position.x}, Y {node.position.y}, Z {node.position.z} {props.model.project.units.length} (entered global coordinates).</p>}
      <TextField label="Boundary provenance" value={draft.provenance} onChange={value => set("provenance", value)} />
      <TextField label="Boundary coordinate system" value={draft.coordinateSystem} choices={["global"]} onChange={value => set("coordinateSystem", value)} />
      {DOFS.map(dof => {
        const row = draft.dofs[dof];
        const update = (key: string, value: string) => set("dofs", { ...draft.dofs, [dof]: { ...row, [key]: value } });
        return <fieldset key={dof}>
          <legend>{dof}</legend>
          <TextField label={`${dof} mode`} value={row.mode} choices={["free", "rigid", "spring"]} onChange={value => update("mode", value)} />
          {row.mode === "spring" && <>
            <p>Positive {dof.startsWith("U") ? "linear" : "rotational"} stiffness with an explicit compatible unit is required.</p>
            <TextField label={`${dof} stiffness value`} value={row.value} onChange={value => update("value", value)} />
            <TextField label={`${dof} stiffness unit`} value={row.unit} onChange={value => update("unit", value)} />
          </>}
        </fieldset>;
      })}
    </fieldset>
    {!invalid && <div aria-label="Boundary member draft">
      <p>Proposed members in batch order (IDs generated from your boundary ID and chosen DOFs):</p>
      <ol>{boundaryMembers(props.model, draft).map(member => <li key={member.id}>{member.id}: {member.family} {member.restraints.join(", ")}</li>)}</ol>
    </div>}
    {invalid && <p id="boundary-draft-reason" role="status">{invalid}</p>}
    <button type="button" disabled={state.preparing || Boolean(invalid)} aria-describedby={invalid ? "boundary-draft-reason" : undefined}
      onClick={() => void state.queue(() => buildBoundaryBatch(props.model, draft, batchId("boundary")))}>
      Queue boundary batch
    </button>
    <button type="button" onClick={() => { state.cancel("Boundary draft cleared. No model changes applied."); setDraft(emptyBoundaryDraft()); }}>Clear / cancel boundary draft</button>
    {state.feedback && <p role="status">{state.feedback}</p>}
    <details>
      <summary>Existing boundary members ({members.length})</summary>
      <p>Read-only association inventory of actual remaining supports. Ordinary member edits preserve this metadata; deletion may leave a partial group.</p>
      {members.length === 0 ? <p>No boundary associations in this model.</p> : <ul>{members.map(member => {
        const association = (member as typeof member & { boundary_association: BoundaryAssociation }).boundary_association;
        return <li key={member.id}>{member.id}: {association.boundary_id}; {association.kind}; equipment {association.equipment_reference}
          {association.nozzle_reference ? `; nozzle ${association.nozzle_reference}` : ""}; {association.coordinate_system}; node {member.node}; {member.family} {member.restraints.join(", ")}</li>;
      })}</ul>}
    </details>
  </section>;
}
