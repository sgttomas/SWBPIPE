import { useEffect, useId, useState } from "react";
import { clone, parseQuantities, QueueFeedback, QuantityField, record, requireText, setMember, TextField, useRichQueue, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
import { VirtualMultiTargetPicker } from "../workspace/VirtualTargetPicker";

export const EXACT_PROFILE = { schema_version: "0.3.0", pressure_contract: { version: "2.0.0", mode: "exact_straight_pressure_v2" } };
export const MATERIAL_KEYS = ["constitutive_basis", "elastic_modulus", "shear_modulus", "poisson_ratio", "provenance"] as const;
export function materialProjection(material: object): DraftRecord {
  const value = material as DraftRecord;
  return Object.fromEntries(MATERIAL_KEYS.filter(key => value[key] !== undefined).map(key => [key, value[key]]));
}
function checkedMaterial(draft: DraftRecord) {
  const after = record(parseQuantities(draft));
  requireText(after.provenance, "Material provenance");
  if (after.constitutive_basis !== "homogeneous_isotropic_E_nu_v1") throw new Error("Select the explicit E/nu constitutive basis.");
  if (typeof record(after.elastic_modulus).value !== "number" || Number(record(after.elastic_modulus).value) <= 0) throw new Error("Explicit positive elastic modulus is required.");
  const nu = record(after.poisson_ratio);
  if (nu.unit !== "1" || typeof nu.value !== "number" || nu.value <= -1 || nu.value >= 0.5) throw new Error("Poisson ratio requires unit 1 and -1 < nu < 0.5.");
  return after;
}
function MaterialInputs({ draft, update }: { draft: DraftRecord; update: (key: string, value: unknown) => void }) {
  return <>
    <TextField label="Constitutive basis" value={draft.constitutive_basis} choices={["homogeneous_isotropic_E_nu_v1"]} onChange={v => update("constitutive_basis", v)} />
    <QuantityField label="Elastic modulus E" dimension="stress" value={draft.elastic_modulus} onChange={v => update("elastic_modulus", v)} />
    <QuantityField label="Poisson ratio nu" dimension="dimensionless; unit 1; -1 < nu < 0.5" value={draft.poisson_ratio} onChange={v => update("poisson_ratio", v)} />
    <TextField label="Constitutive provenance" value={draft.provenance} onChange={v => update("provenance", v)} />
    <p>The exact profile derives G from the selected E/nu pair. Any existing G remains recorded and is nonauthoritative in that profile.</p>
  </>;
}
export function MaterialConstitutiveForm(props: RichFormProps) {
  const material = props.model.materials?.find(m => props.selection.type === "material" && m.id === props.selection.id);
  return material ? <ExistingMaterialEditor key={material.id} {...props} material={material} /> : null;
}
function ExistingMaterialEditor(props: RichFormProps & { material: NonNullable<RichFormProps["model"]["materials"]>[number] }) {
  const [draft, setDraft] = useState(() => materialProjection(props.material));
  useEffect(() => setDraft(materialProjection(props.material)), [props.material]);
  const state = useRichQueue(props);
  async function submit() { try {
    await state.queue({ object_type: "Material", ref: props.material.id }, "set_field", "constitutive_properties", materialProjection(props.material), checkedMaterial(draft), "Constitutive properties");
  } catch (e) { state.setError(String(e)); } }
  return <section aria-label="Exact material constitutive inputs"><h3>Exact material E/nu</h3><fieldset disabled={state.busy}>
    <MaterialInputs draft={draft} update={(k, v) => setDraft(current => setMember(current, k, v))} />
    <button type="button" onClick={() => void submit()}>Queue constitutive properties</button>
  </fieldset><QueueFeedback {...state} /></section>;
}
function CreateExactMaterial(props: RichFormProps) {
  const [draft, setDraft] = useState<DraftRecord>({});
  const state = useRichQueue({ ...props, onQueueIntent: intent => {
    const after = JSON.parse(intent.change.after) as DraftRecord;
    props.onQueueIntent({ ...intent, operation_kind: "create", change: { ...intent.change, dimension: "stress", unit: String(record(after.elastic_modulus).unit) } });
  } });
  const update = (k: string, v: unknown) => setDraft(current => setMember(current, k, v));
  async function submit() { try {
    const after = checkedMaterial(draft);
    const id = requireText(after.id, "Material ID"); requireText(after.label, "Material label");
    if (props.model.materials?.some(m => m.id === id)) throw new Error("Material ID already exists.");
    await state.queue({ object_type: "Material", ref: id }, "create_material", "materials", undefined, after, "Exact E/nu material", true);
  } catch (e) { state.setError(String(e)); } }
  return <details><summary>Create explicit E/nu material</summary><fieldset disabled={state.busy}>
    <TextField label="Exact material ID" value={draft.id} onChange={v => update("id", v)} />
    <TextField label="Exact material label" value={draft.label} onChange={v => update("label", v)} />
    <MaterialInputs draft={draft} update={update} />
    <button type="button" onClick={() => void submit()}>Queue exact material</button>
  </fieldset><QueueFeedback {...state} /></details>;
}
export function PressureAuthoringPanel(props: RichFormProps & { loadCaseId?: string }) {
  const state = useRichQueue(props);
  const [profile, setProfile] = useState("");
  const profileReasonId = useId();
  const profileDisabledReason = state.busy ? "Wait for the current change to finish queuing."
    : profile !== "exact_straight_pressure_v2" ? "Select a pressure profile before queuing this change." : undefined;
  const currentProfile = { schema_version: props.model.schema_version, ...(props.model.pressure_contract ? { pressure_contract: props.model.pressure_contract } : {}) };
  const loadCase = props.model.load_cases.find(c => c.id === props.loadCaseId);
  return <section aria-label="Exact pressure authoring"><h3>Pressure mechanics profile</h3>
    <p>Current model: {props.model.schema_version}; pressure mode: {props.model.pressure_contract?.mode ?? (props.model.schema_version === "0.3.0" ? "not declared" : "legacy")}. Every exact-profile case needs an explicit pressure-region array, including an explicitly reviewed empty array for unpressurized cases. Existing pressure primitives must be removed explicitly before this profile can be applied. Unsupported combinations, fittings and nonlinear cases remain subject to solver diagnostics.</p>
    <p>Pressure is uniform within each region. Structural line loads act as entered; a contents-density value does not imply hydrostatic pressure head or a coupled static-fluid pressure and weight state.</p>
    <fieldset disabled={state.busy}>
      <TextField label="Pressure profile" value={profile} choices={["exact_straight_pressure_v2"]} onChange={setProfile} />
      <button type="button" disabled={profile !== "exact_straight_pressure_v2"} title={profileDisabledReason} aria-describedby={profileDisabledReason ? profileReasonId : undefined} onClick={() => void state.queue({ object_type: "Model", ref: props.model.project.id }, "set_field", "pressure_profile", currentProfile, EXACT_PROFILE, "Exact pressure profile")}>Queue pressure profile</button>
    </fieldset>
    {profileDisabledReason && <p id={profileReasonId}>{profileDisabledReason}</p>}
    <QueueFeedback {...state} />
    <CreateExactMaterial {...props} />
    {loadCase && <PressureRegionsEditor key={loadCase.id} {...props} loadCase={loadCase} />}
  </section>;
}
function PressureRegionsEditor(props: RichFormProps & { loadCase: RichFormProps["model"]["load_cases"][number] }) {
  const [rows, setRows] = useState<DraftRecord[]>(() => clone(props.loadCase.pressure_regions ?? []) as DraftRecord[]);
  useEffect(() => setRows(clone(props.loadCase.pressure_regions ?? []) as DraftRecord[]), [props.loadCase]);
  const state = useRichQueue(props);
  const update = (index: number, key: string, value: unknown) => setRows(current => current.map((row, i) => i === index ? setMember(row, key, value) : row));
  const pipes = props.model.pipe_segments.map(p => ({ value: p.id, label: p.label || p.id }));
  async function submit() { try {
    const after = parseQuantities(rows) as DraftRecord[];
    const ids = after.map(row => requireText(row.id, "Region ID"));
    if (new Set(ids).size !== ids.length) throw new Error("Region IDs must be unique.");
    for (const row of after) {
      requireText(row.provenance, "Region provenance");
      requireText(row.pressure_basis, "Pressure basis");
      if (!Array.isArray(row.member_pipe_ids) || !row.member_pipe_ids.length) throw new Error("Select region member pipes.");
      if (!row.pressure) throw new Error("Enter explicit pressure, including zero if intended.");
      for (const terminal of row.terminals as DraftRecord[]) {
        requireText(terminal.node_ref, "Terminal node"); requireText(terminal.closure_transfer, "Terminal closure transfer"); requireText(terminal.provenance, "Terminal provenance");
      }
    }
    await state.queue({ object_type: "Load", ref: props.loadCase.id }, "update_load", "pressure_regions", props.loadCase.pressure_regions, after, "Pressure regions", props.loadCase.pressure_regions === undefined);
  } catch (e) { state.setError(String(e)); } }
  return <section aria-label="Case pressure regions"><h4>Pressure regions — {props.loadCase.label || props.loadCase.id}</h4>
    <p>{props.loadCase.pressure_regions === undefined ? "No pressure-region declaration has been applied." : `${props.loadCase.pressure_regions.length} applied regions.`} Terminal order and closure transfer are user-authored. The solver checks continuity, straightness and physical compatibility.</p>
    <fieldset disabled={state.busy}>{rows.map((row, index) => <fieldset key={index}><legend>Region {index + 1}</legend>
      <TextField label={`Region ${index + 1} ID`} value={row.id} onChange={v => update(index, "id", v)} />
      <VirtualMultiTargetPicker label={`Region ${index + 1} member pipes`} options={pipes} values={Array.isArray(row.member_pipe_ids) ? row.member_pipe_ids as string[] : []} onChange={v => update(index, "member_pipe_ids", v)} />
      <TextField label={`Region ${index + 1} pressure basis`} value={row.pressure_basis} choices={["internal_differential_zero_external_v1"]} onChange={v => update(index, "pressure_basis", v)} />
      <QuantityField label={`Region ${index + 1} pressure`} dimension="pressure" value={row.pressure} onChange={v => update(index, "pressure", v)} />
      {[0, 1].map(t => {
        const terminals = Array.isArray(row.terminals) ? row.terminals as DraftRecord[] : [{}, {}]; const terminal = terminals[t] ?? {};
        const edit = (key: string, value: string) => update(index, "terminals", [0, 1].map(i => i === t ? setMember(terminal, key, value) : terminals[i] ?? {}));
        return <fieldset key={t}><legend>Ordered terminal {t + 1}</legend>
          <TextField label={`Region ${index + 1} terminal ${t + 1} node`} value={terminal.node_ref} choices={props.model.nodes.map(n => n.id)} onChange={v => edit("node_ref", v)} />
          <TextField label={`Region ${index + 1} terminal ${t + 1} closure transfer`} value={terminal.closure_transfer} choices={["transfers_to_wall", "separately_supported_or_compensated"]} onChange={v => edit("closure_transfer", v)} />
          <TextField label={`Region ${index + 1} terminal ${t + 1} provenance`} value={terminal.provenance} onChange={v => edit("provenance", v)} />
        </fieldset>;
      })}
      <TextField label={`Region ${index + 1} provenance`} value={row.provenance} onChange={v => update(index, "provenance", v)} />
      <button type="button" onClick={() => setRows(rows.filter((_, i) => i !== index))}>Remove region {index + 1}</button>
    </fieldset>)}
      <button type="button" onClick={() => setRows([...rows, { terminals: [{}, {}] }])}>Add pressure region</button>
      <button type="button" onClick={() => void submit()}>{rows.length ? "Queue pressure regions" : "Queue explicit empty pressure regions"}</button>
    </fieldset><QueueFeedback {...state} />
  </section>;
}
