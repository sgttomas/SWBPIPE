import { useEffect, useState } from "react";
import { clone, parseQuantities, QueueFeedback, QuantityField, record, requireText, setMember, TextField, useRichQueue, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
const DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const FAMILIES = ["anchor", "guide", "LineStop", "VerticalSupport", "spring", "variable_spring_hanger", "constant_effort_support", "one_way", "gap", "lift_off", "friction"];
const CONFIG = ["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"];
function projection(support: unknown): DraftRecord {
  const s = record(support);
  return Object.fromEntries(CONFIG.filter(k => s[k] !== undefined).map(k => [k, clone(s[k])]));
}
export function SupportConfigurationForm(props: RichFormProps) {
  if (!["support", "node", "project"].includes(props.selection.type))
    return null;
  const support = props.selection.type === "support" ? props.model.supports.find(s => s.id === props.selection.id) : undefined;
  if (props.selection.type === "support" && !support)
    return null;
  return <SupportEditor
    key={`${props.selection.type}:${props.selection.id}`}
    {...props}
    support={support}
  />;
}
function StiffnessFields({ label, value, onChange }: {
  label: string;
  value: unknown;
  onChange: (value: unknown) => void;
}) {
  const s = record(value);
  return <fieldset>
    <legend>{label}</legend>
    <TextField
      label={`${label} DOF`}
      value={s.dof}
      choices={DOFS}
      onChange={dof => onChange(setMember(s, "dof", dof))}
    />
    <QuantityField
      dimension={String(s.dof).toUpperCase().startsWith("R") ? "rotational stiffness (moment per angle)" : String(s.dof).toUpperCase().startsWith("U") ? "linear stiffness (force per length)" : "stiffness — select a DOF"}
      label={`${label} quantity`}
      value={s.value}
      onChange={q => onChange(setMember(s, "value", q))}
    />
    <button type="button" onClick={() => onChange(undefined)}>Remove {label.toLowerCase()}</button>
  </fieldset>;
}
function SupportEditor(props: RichFormProps & {
  support?: RichFormProps["model"]["supports"][number];
}) {
  const { support } = props;
  const [create, setCreate] = useState(!support);
  const initial = () => support && !create ? projection(support) : { restraints: [] };
  const [draft, setDraft] = useState<DraftRecord>(initial);
  const [id, setId] = useState("");
  const [label, setLabel] = useState("");
  const [node, setNode] = useState(props.selection.type === "node" ? props.selection.id : "");
  useEffect(() => {
    setDraft(support && !create ? projection(support) : { restraints: [] });
  }, [support, create]);
  const state = useRichQueue(props);
  const set = (key: string, value: unknown) => setDraft(d => setMember(d, key, value));
  const hanger = record(draft.hanger), nonlinear = record(draft.nonlinear);
  const setH = (key: string, value: unknown) => set("hanger", setMember(hanger, key, value));
  const setN = (key: string, value: unknown) => set("nonlinear", setMember(nonlinear, key, value));
  const restraints = Array.isArray(draft.restraints) ? draft.restraints as string[] : [];
  const source = record(nonlinear.normal_reaction_source);
  const eligible = props.model.supports.filter(s => s.id !== (create ? id : support?.id) && !record(s).nonlinear && s.restraints.some(d => ["UX", "UY", "UZ"].includes(d.toUpperCase())));
  const selectedSource = eligible.find(s => s.id === source.support_ref);
  const collisions = [props.model.nodes, props.model.pipe_segments, props.model.supports, props.model.components, props.model.materials ?? [], props.model.sections ?? [], props.model.load_cases, props.model.combinations ?? []].flat().some(e => e.id === id) || Boolean(props.queuedIntents?.some(i => i.target.ref === id && i.operation_kind === "create"));
  async function submit() {
    try {
      const after = record(parseQuantities(draft));
      for (const key of ["stiffness"]) {
        if (after[key] !== undefined) {
          const stiffness = record(after[key]);
          requireText(stiffness.dof, "Stiffness DOF");
          if (stiffness.value === undefined)
            throw new Error("Enter stiffness value and unit, or remove stiffness.");
        }
      }
      if (after.hanger !== undefined && record(after.hanger).stiffness !== undefined) {
        const s = record(record(after.hanger).stiffness);
        requireText(s.dof, "Hanger stiffness DOF");
        if (s.value === undefined)
          throw new Error("Enter hanger stiffness value and unit, or remove hanger stiffness.");
      }
      if (after.nonlinear !== undefined) {
        const n = record(after.nonlinear);
        requireText(n.behavior, "Nonlinear behavior");
        requireText(n.dof, "Nonlinear DOF");
        requireText(n.initial_state, "Initial state");
        if (n.normal_reaction_source !== undefined) {
          const src = record(n.normal_reaction_source);
          requireText(src.support_ref, "Reaction source support");
          requireText(src.dof, "Reaction source DOF");
          if (!selectedSource || !selectedSource.restraints.some(d => d.toUpperCase() === String(src.dof).toUpperCase()))
            throw new Error("Select an existing linear support and one of its restrained translational DOFs.");
        }
        if (n.normal_reaction !== undefined && n.normal_reaction_source !== undefined)
          throw new Error("Use either an entered normal reaction or a derived source, not both.");
      }
      if (create) {
        requireText(id, "Support ID");
        requireText(label, "Support label");
        requireText(node, "Support node");
        requireText(after.provenance, "Support provenance");
        if (collisions)
          throw new Error("Support ID already exists or is queued. Enter a unique ID.");
        await state.queue(
          { object_type: "Support", ref: id },
          "create_support",
          "supports",
          undefined,
          {
            id,
            label,
            node,
            ...after
          },
          "Support creation",
          true
        );
      }
      else if (support)
        await state.queue(
          { object_type: "Support", ref: support.id },
          "update_support",
          "configuration",
          projection(support),
          after,
          "Support configuration"
        );
    }
    catch (e) {
      state.setError(e instanceof Error ? e.message : String(e));
    }
  }
  return <section id="rich-support-form" tabIndex={-1} aria-label="Support configuration">
    <h3>Support configuration</h3>
    <p>Choose the support family and enter its physical inputs. Family names do not supply restraints or engineering values. Missing inputs remain missing; validation determines readiness.</p>
    <fieldset disabled={state.busy}>
      <legend>{create ? "Create support" : "Edit support"}</legend>{support && <label>
        <input
          type="checkbox"
          checked={create}
          onChange={e => setCreate(e.target.checked)}
        />Create a new support</label>}
      {create ? <>
        <TextField
          label="Support ID"
          value={id}
          onChange={setId}
        />
        {id && collisions && <p role="alert">Support ID already exists or is queued.</p>}
        <TextField
          label="Support label"
          value={label}
          onChange={setLabel}
        />
        <TextField
          label="Support node"
          value={node}
          choices={props.model.nodes.map(n => n.id)}
          onChange={setNode}
        />
      </> : <p>{support?.label} — {support?.id}, node {support?.node}</p>}
      <TextField
        label="Support family"
        value={draft.family}
        choices={FAMILIES}
        onChange={v => set("family", v)}
      />
      <fieldset>
        <legend>Restrained degrees of freedom</legend>{DOFS.map(dof => <label key={dof}>
          <input
            type="checkbox"
            aria-label={`Restrain ${dof}`}
            checked={restraints.some(r => r.toUpperCase() === dof)}
            onChange={e => set("restraints", e.target.checked ? [...restraints, dof] : restraints.filter(r => r.toUpperCase() !== dof))}
          />
          {dof}</label>)}</fieldset>
      <TextField
        label="Support provenance"
        value={draft.provenance}
        onChange={v => set("provenance", v)}
      />
      {draft.stiffness !== undefined ? <StiffnessFields
        label="Support stiffness"
        value={draft.stiffness}
        onChange={v => set("stiffness", v)}
      /> : <button type="button" onClick={() => set("stiffness", {})}>Add support stiffness</button>}
      <label>
        <input
          type="checkbox"
          checked={draft.hanger !== undefined}
          onChange={e => set("hanger", e.target.checked ? {} : undefined)}
        />Include hanger data</label>
      {draft.hanger !== undefined && <fieldset>
        <legend>Hanger</legend>
        <TextField
          label="Hanger type"
          value={hanger.hanger_type}
          choices={["variable_spring_hanger", "spring_hanger", "constant_effort_support"]}
          onChange={v => setH("hanger_type", v)}
        />
        <p>Variable hangers require explicit stiffness, installed/cold/hot loads, travel or movement limit, and source. Constant-effort supports require constant load, travel or movement limit, and source.</p>{hanger.stiffness !== undefined ? <StiffnessFields
          label="Hanger stiffness"
          value={hanger.stiffness}
          onChange={v => setH("stiffness", v)}
        /> : <button type="button" onClick={() => setH("stiffness", {})}>Add hanger stiffness</button>}
        {["installed_load", "cold_load", "hot_load", "constant_load", "travel_range", "movement_limit"].map(key => <QuantityField
          dimension={["travel_range", "movement_limit"].includes(key) ? "length" : "force"}
          key={key}
          label={`Hanger ${key.replaceAll("_", " ")}`}
          value={hanger[key]}
          onChange={v => setH(key, v)}
        />)}
        {["manufacturer_reference", "source_reference", "load_side_review_reference", "mechanics_consumption"].map(key => <TextField
          key={key}
          label={`Hanger ${key.replaceAll("_", " ")}`}
          value={hanger[key]}
          onChange={v => setH(key, v)}
        />)}</fieldset>}
      <label>
        <input
          type="checkbox"
          checked={draft.nonlinear !== undefined}
          onChange={e => set("nonlinear", e.target.checked ? {} : undefined)}
        />Include nonlinear behavior</label>
      {draft.nonlinear !== undefined && <fieldset>
        <legend>Nonlinear behavior</legend>
        <TextField
          label="Nonlinear behavior"
          value={nonlinear.behavior}
          choices={["one_way", "gap", "lift_off", "friction"]}
          onChange={v => setN("behavior", v)}
        />
        <TextField
          label="Nonlinear DOF"
          value={nonlinear.dof}
          choices={DOFS}
          onChange={v => setN("dof", v)}
        />
        <TextField
          label="Initial state"
          value={nonlinear.initial_state}
          choices={["active", "inactive", "sticking", "sliding"]}
          onChange={v => setN("initial_state", v)}
        />
        {["active_when", "contact_when", "closes_when"].map(key => <TextField
          key={key}
          label={key.replaceAll("_", " ")}
          value={nonlinear[key]}
          choices={key === "closes_when" ? ["positive_displacement", "negative_displacement"] : ["positive_reaction", "negative_reaction"]}
          onChange={v => setN(key, v)}
        />)}
        {["gap", "friction_coefficient", "normal_reaction"].map(key => <QuantityField
          dimension={key === "gap" ? "length" : key === "normal_reaction" ? "force" : "dimensionless coefficient"}
          key={key}
          label={key.replaceAll("_", " ")}
          value={nonlinear[key]}
          onChange={v => setN(key, v)}
        />)}
        <label>
          <input
            type="checkbox"
            checked={nonlinear.normal_reaction_source !== undefined}
            onChange={e => setN("normal_reaction_source", e.target.checked ? {} : undefined)}
          />Derive normal reaction from a support</label>{nonlinear.normal_reaction_source !== undefined && <>
            <TextField
              label="Reaction source support"
              value={source.support_ref}
              choices={eligible.map(s => s.id)}
              onChange={v => setN("normal_reaction_source", setMember(setMember(source, "dof", undefined), "support_ref", v))}
            />
            <TextField
              label="Reaction source DOF"
              value={source.dof}
              choices={selectedSource?.restraints.filter(d => ["UX", "UY", "UZ"].includes(d.toUpperCase())) ?? []}
              onChange={v => setN("normal_reaction_source", setMember(source, "dof", v))}
            />
            <p>Choose a linear support’s restrained translational DOF. Clear any entered normal reaction to use this source.</p>
          </>}
      </fieldset>}
      <button type="button" onClick={() => void submit()}>{create ? "Queue support creation" : "Queue support configuration"}</button>
    </fieldset>
    <QueueFeedback {...state} />
  </section>;
}
