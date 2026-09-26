import { useEffect, useState } from "react";
import type { PreviewModel } from "../../types";
import { record, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
import {
  clone, isLoadStateModel, LOAD_STATE_CONTRACT, LoadStateFeedback, LoadStateNeeds040, member, NOT_PRESENT, NumberText,
  Quantity, replaceAt, rows, switchVariant, Text, toPayload, useLoadStateQueue, type LoadStateTarget
} from "./loadStateAuthoring";

type LoadCase = PreviewModel["load_cases"][number];
const DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"] as const;
const BASIS = { temperature_reference: ["installation_temperature"], direct_strain_reference: [] } as const;
const FIT = { none: [], natural_length_change: ["length_change"], fit_strain: ["strain"] } as const;
const SELECTION = {
  explicit_base_properties: ["material_ref", "applicability_reference"],
  exact_point: ["material_ref", "point_ref"],
  temperature_interpolation: ["material_ref", "temperature", "interpolation", "extrapolation"]
} as const;
const THERMAL = {
  unchanged_reference: ["provenance"],
  explicit_interval_strain: ["strain", "interval_reference", "provenance"],
  constant_alpha_interval: ["coefficient", "temperature_change", "coefficient_meaning", "provenance"],
  free_length_state: ["expansion_law_ref"]
} as const;
const PARTICIPATION = { active_model_device: [], inactive: [], locked_equivalent_support: ["components"] } as const;

/** Model 0.4.0 reference configurations and the selected case's analysis
 * state. On any other model version only the one gating line is shown. */
export function LoadReferenceStateInputs(props: RichFormProps & { loadCase: LoadCase | null }) {
  if (!isLoadStateModel(props.model)) return <LoadStateNeeds040 testId="load-state-needs-040" />;
  return <section aria-label="Load/reference state inputs" data-testid="load-state-inputs">
    <h3>Load/reference state (model 0.4.0)</h3>
    <p>Plain inputs for the resolved load/reference state. Every value and unit is entered explicitly; an empty field is an absent key. The operation engine checks each record before it is queued for Review/Apply.</p>
    <ReferenceConfigurationsEditor {...props} />
    {props.loadCase ? <AnalysisStateEditor key={props.loadCase.id} {...props} loadCase={props.loadCase} /> : <p>No load case is selected.</p>}
  </section>;
}

function ReferenceConfigurationsEditor(props: RichFormProps) {
  const current = props.model.reference_configurations;
  const [draft, setDraft] = useState<DraftRecord[] | undefined>(() => clone(current ?? undefined) as DraftRecord[] | undefined);
  useEffect(() => setDraft(clone(current ?? undefined) as DraftRecord[] | undefined), [current]);
  const state = useLoadStateQueue(props);
  const spec: LoadStateTarget = { target: { object_type: "Model", ref: props.model.project.id }, kind: "set_field", path: "reference_configurations", label: "Reference configurations" };
  const pipes = props.model.pipe_segments.map(p => p.id);
  const list = draft ?? [];
  const editAt = (index: number, next: DraftRecord) => setDraft(replaceAt(list, index, next));
  return <section aria-label="Reference configurations" data-testid="load-state-reference-configurations">
    <h4>Reference configurations</h4>
    <p>{current === undefined ? "reference_configurations is absent." : current === null ? "reference_configurations is an explicit null." : `${current.length} applied reference configurations.`}</p>
    <fieldset disabled={state.busy}>
      {list.map((configuration, c) => {
        const members = rows(configuration.member_references) ?? [];
        const setMembers = (next: DraftRecord[]) => editAt(c, member(configuration, "member_references", next));
        const n = c + 1;
        return <fieldset key={c}><legend>Reference configuration {n}</legend>
          <Text label={`Reference configuration ${n} ID`} value={configuration.id} onChange={v => editAt(c, member(configuration, "id", v))} />
          <Text label={`Reference configuration ${n} label`} value={configuration.label} onChange={v => editAt(c, member(configuration, "label", v))} />
          <Text label={`Reference configuration ${n} geometry`} value={record(configuration.geometry_ref).kind} choices={["authored_model_geometry"]} onChange={v => editAt(c, member(configuration, "geometry_ref", v ? { kind: v } : undefined))} />
          {members.map((row, m) => {
            const k = `Reference configuration ${n} member ${m + 1}`;
            const set = (key: string, value: unknown) => setMembers(replaceAt(members, m, member(row, key, value)));
            const basis = record(row.basis), fit = record(row.fit);
            return <fieldset key={m}><legend>Member {m + 1}</legend>
              <Text label={`${k} pipe`} value={row.pipe_ref} choices={pipes} onChange={v => set("pipe_ref", v)} />
              <Text label={`${k} basis`} value={basis.kind} choices={Object.keys(BASIS)} onChange={v => set("basis", switchVariant(row.basis, "kind", v, BASIS))} />
              {basis.kind === "temperature_reference" && <Quantity label={`${k} installation temperature`} dimension="temperature" value={basis.installation_temperature} onChange={v => set("basis", member(basis, "installation_temperature", v))} />}
              <Text label={`${k} fit`} value={fit.kind} choices={Object.keys(FIT)} onChange={v => set("fit", switchVariant(row.fit, "kind", v, FIT))} />
              {fit.kind === "natural_length_change" && <Quantity label={`${k} natural length change`} dimension="length" value={fit.length_change} onChange={v => set("fit", member(fit, "length_change", v))} />}
              {fit.kind === "fit_strain" && <Quantity label={`${k} fit strain`} dimension="strain; unit 1" value={fit.strain} onChange={v => set("fit", member(fit, "strain", v))} />}
              <Text label={`${k} provenance`} value={row.provenance} onChange={v => set("provenance", v)} />
              <button type="button" onClick={() => setMembers(members.filter((_, i) => i !== m))}>Remove {k}</button>
            </fieldset>;
          })}
          <button type="button" onClick={() => setMembers([...members, {}])}>Add member to reference configuration {n}</button>
          <Text label={`Reference configuration ${n} provenance`} value={configuration.provenance} onChange={v => editAt(c, member(configuration, "provenance", v))} />
          <button type="button" onClick={() => setDraft(list.filter((_, i) => i !== c))}>Remove reference configuration {n}</button>
        </fieldset>;
      })}
      <button type="button" onClick={() => setDraft([...list, {}])}>Add reference configuration</button>
      <button type="button" onClick={() => void state.submit(spec, current, () => draft === undefined ? NOT_PRESENT : toPayload(draft))}>Queue reference configurations</button>
      {current !== undefined && <button type="button" onClick={() => void state.submit(spec, current, () => NOT_PRESENT)}>Queue removal of reference configurations</button>}
    </fieldset>
    <LoadStateFeedback {...state} testId="load-state-reference" />
  </section>;
}

function AnalysisStateEditor(props: RichFormProps & { loadCase: LoadCase }) {
  const { loadCase, model } = props;
  const current = loadCase.analysis_state;
  const [draft, setDraft] = useState<DraftRecord>(() => clone(record(current)));
  useEffect(() => setDraft(clone(record(current))), [current]);
  const state = useLoadStateQueue(props);
  const spec: LoadStateTarget = { target: { object_type: "Load", ref: loadCase.id }, kind: "update_load", path: "analysis_state", label: "Analysis state" };
  const set = (key: string, value: unknown) => setDraft(d => member(d, key, value));
  const caseName = loadCase.label || loadCase.id;
  const references = (model.reference_configurations ?? []).map(r => r.id);
  const primitives = ((loadCase as { primitive_loads?: Array<{ id?: unknown }> }).primitive_loads ?? []).flatMap(p => typeof p.id === "string" ? [p.id] : []);
  return <section aria-label="Case analysis state" data-testid="load-state-analysis-state">
    <h4>Analysis state — {caseName}</h4>
    <p>{current === undefined ? "analysis_state is absent for this case." : current === null ? "analysis_state is an explicit null for this case." : "analysis_state is applied for this case."}</p>
    <fieldset disabled={state.busy}>
      <Text label="Analysis state contract" value={draft.contract} choices={[LOAD_STATE_CONTRACT]} onChange={v => set("contract", v)} />
      <Text label="Reference configuration" value={draft.reference_configuration_ref} choices={references} onChange={v => set("reference_configuration_ref", v)} />
      <ElementStates model={model} value={rows(draft.element_states)} onChange={v => set("element_states", v)} />
      <SupportStates model={model} value={rows(draft.support_states)} onChange={v => set("support_states", v)} />
      <LoadSources primitives={primitives} value={rows(draft.load_sources)} onChange={v => set("load_sources", v)} />
      <Text label="History" value={record(draft.history).kind} choices={["independent_equilibrium"]} onChange={v => set("history", v ? { kind: v } : undefined)} />
      <Text label="Analysis state provenance" value={draft.provenance} onChange={v => set("provenance", v)} />
      <button type="button" onClick={() => void state.submit(spec, current, () => toPayload(draft))}>Queue analysis state</button>
      {current !== undefined && <button type="button" onClick={() => void state.submit(spec, current, () => NOT_PRESENT)}>Queue removal of analysis state</button>}
    </fieldset>
    <LoadStateFeedback {...state} testId="load-state-analysis" />
  </section>;
}

type ListProps = { value: DraftRecord[] | undefined; onChange: (value: DraftRecord[] | undefined) => void };

function ListStatus({ name, value, onChange, add }: ListProps & { name: string; add: string }) {
  return <p>{value === undefined ? `${name} is absent.` : `${name}: ${value.length} entries.`}{" "}
    <button type="button" onClick={() => onChange([...(value ?? []), {}])}>{add}</button>
    {value !== undefined && <button type="button" onClick={() => onChange(undefined)}>Mark {name} absent</button>}
  </p>;
}

function ElementStates({ model, value, onChange }: ListProps & { model: PreviewModel }) {
  const list = value ?? [];
  const pipes = model.pipe_segments.map(p => p.id);
  const materials = model.materials ?? [];
  return <fieldset><legend>Element states</legend>
    <ListStatus name="element_states" value={value} onChange={onChange} add="Add element state" />
    {list.map((row, e) => {
      const k = `Element state ${e + 1}`;
      const set = (key: string, next: unknown) => onChange(replaceAt(list, e, member(row, key, next)));
      const selection = record(row.material_selection), thermal = record(row.thermal_state);
      const material = materials.find(m => m.id === selection.material_ref) as (typeof materials)[number] & { expansion_laws?: Array<{ id?: unknown }> | null } | undefined;
      const points = (material?.temperature_points ?? []).flatMap(p => typeof p.id === "string" ? [p.id] : []);
      const laws = (material?.expansion_laws ?? []).flatMap(l => typeof l?.id === "string" ? [l.id] : []);
      const setSelection = (key: string, next: unknown) => set("material_selection", member(selection, key, next));
      const setThermal = (key: string, next: unknown) => set("thermal_state", member(thermal, key, next));
      return <fieldset key={e}><legend>{k}</legend>
        <Text label={`${k} pipe`} value={row.pipe_ref} choices={pipes} onChange={v => set("pipe_ref", v)} />
        <Quantity label={`${k} operating temperature`} dimension="temperature; optional" value={row.operating_temperature} onChange={v => set("operating_temperature", v)} />
        <Text label={`${k} material selection`} value={selection.kind} choices={Object.keys(SELECTION)} onChange={v => set("material_selection", switchVariant(row.material_selection, "kind", v, SELECTION))} />
        {selection.kind !== undefined && <Text label={`${k} material`} value={selection.material_ref} choices={materials.map(m => m.id)} onChange={v => setSelection("material_ref", v)} />}
        {selection.kind === "explicit_base_properties" && <Text label={`${k} applicability reference`} value={selection.applicability_reference} onChange={v => setSelection("applicability_reference", v)} />}
        {selection.kind === "exact_point" && <Text label={`${k} material point`} value={selection.point_ref} choices={points} onChange={v => setSelection("point_ref", v)} />}
        {selection.kind === "temperature_interpolation" && <>
          <Quantity label={`${k} selection temperature`} dimension="temperature" value={selection.temperature} onChange={v => setSelection("temperature", v)} />
          <Text label={`${k} interpolation`} value={selection.interpolation} choices={["piecewise_linear"]} onChange={v => setSelection("interpolation", v)} />
          <Text label={`${k} extrapolation`} value={selection.extrapolation} choices={["forbidden"]} onChange={v => setSelection("extrapolation", v)} />
        </>}
        <Text label={`${k} thermal state`} value={thermal.kind} choices={Object.keys(THERMAL)} onChange={v => set("thermal_state", switchVariant(row.thermal_state, "kind", v, THERMAL))} />
        {thermal.kind === "explicit_interval_strain" && <>
          <Quantity label={`${k} thermal strain`} dimension="strain; unit 1" value={thermal.strain} onChange={v => setThermal("strain", v)} />
          <Text label={`${k} interval reference`} value={thermal.interval_reference} onChange={v => setThermal("interval_reference", v)} />
        </>}
        {thermal.kind === "constant_alpha_interval" && <>
          <Quantity label={`${k} expansion coefficient`} dimension="inverse temperature" value={thermal.coefficient} onChange={v => setThermal("coefficient", v)} />
          <Quantity label={`${k} temperature change`} dimension="temperature difference" value={thermal.temperature_change} onChange={v => setThermal("temperature_change", v)} />
          <Text label={`${k} coefficient meaning`} value={thermal.coefficient_meaning} choices={["engineering_interval"]} onChange={v => setThermal("coefficient_meaning", v)} />
        </>}
        {thermal.kind === "free_length_state" && <Text label={`${k} expansion law`} value={thermal.expansion_law_ref} choices={laws} onChange={v => setThermal("expansion_law_ref", v)} />}
        {thermal.kind !== undefined && thermal.kind !== "free_length_state" && <Text label={`${k} thermal provenance`} value={thermal.provenance} onChange={v => setThermal("provenance", v)} />}
        {(row.analysis_basis_override !== undefined || row.mass_state_ref !== undefined) && <p>Retained as authored (not edited here): {["analysis_basis_override", "mass_state_ref"].filter(key => row[key] !== undefined).join(", ")}.</p>}
        <button type="button" onClick={() => onChange(list.filter((_, i) => i !== e))}>Remove {k}</button>
      </fieldset>;
    })}
  </fieldset>;
}

function SupportStates({ model, value, onChange }: ListProps & { model: PreviewModel }) {
  const list = value ?? [];
  const supports = model.supports.map(s => s.id);
  return <fieldset><legend>Support states</legend>
    <ListStatus name="support_states" value={value} onChange={onChange} add="Add support state" />
    {list.map((row, s) => {
      const k = `Support state ${s + 1}`;
      const set = (key: string, next: unknown) => onChange(replaceAt(list, s, member(row, key, next)));
      const participation = record(row.participation);
      const motions = rows(row.boundary_motion);
      const choices = Object.keys(PARTICIPATION).filter(kind => kind !== "locked_equivalent_support" || participation.kind === kind);
      return <fieldset key={s}><legend>{k}</legend>
        <Text label={`${k} support`} value={row.support_ref} choices={supports} onChange={v => set("support_ref", v)} />
        <Text label={`${k} participation`} value={participation.kind} choices={choices} onChange={v => set("participation", switchVariant(row.participation, "kind", v, PARTICIPATION))} />
        <fieldset><legend>{k} boundary motion</legend>
          <ListStatus name={`${k} boundary_motion`} value={motions} onChange={v => set("boundary_motion", v)} add={`Add ${k} boundary motion`} />
          {(motions ?? []).map((motion, m) => {
            const km = `${k} motion ${m + 1}`;
            const setMotion = (key: string, next: unknown) => set("boundary_motion", replaceAt(motions ?? [], m, member(motion, key, next)));
            return <fieldset key={m}><legend>Motion {m + 1}</legend>
              <Text label={`${km} DOF`} value={motion.dof} choices={DOFS} onChange={v => setMotion("dof", v)} />
              <Quantity label={`${km} displacement`} dimension="length for UX/UY/UZ, angle for RX/RY/RZ" value={motion.value} onChange={v => setMotion("value", v)} />
              <Text label={`${km} meaning`} value={motion.meaning} choices={["absolute_reference_displacement"]} onChange={v => setMotion("meaning", v)} />
              <button type="button" onClick={() => set("boundary_motion", (motions ?? []).filter((_, i) => i !== m))}>Remove {km}</button>
            </fieldset>;
          })}
        </fieldset>
        {(row.base_motion !== undefined || row.device_reference !== undefined || participation.kind === "locked_equivalent_support") && <p>Retained as authored (not edited here): {["base_motion", "device_reference", ...(participation.kind === "locked_equivalent_support" ? ["participation.components"] : [])].filter(key => key.includes(".") || row[key] !== undefined).join(", ")}.</p>}
        <button type="button" onClick={() => onChange(list.filter((_, i) => i !== s))}>Remove {k}</button>
      </fieldset>;
    })}
  </fieldset>;
}

function LoadSources({ primitives, value, onChange }: ListProps & { primitives: string[] }) {
  const list = value ?? [];
  return <fieldset><legend>Load sources</legend>
    <p>Each stored primitive of this case that the state includes, with its explicit factor.</p>
    <ListStatus name="load_sources" value={value} onChange={onChange} add="Add load source" />
    {list.map((row, i) => {
      const k = `Load source ${i + 1}`;
      const set = (key: string, next: unknown) => onChange(replaceAt(list, i, member(row, key, next)));
      return <fieldset key={i}><legend>{k}</legend>
        <Text label={`${k} primitive`} value={row.source_ref} choices={primitives} onChange={v => set("source_ref", v)} />
        <NumberText label={`${k} factor (dimensionless)`} value={row.factor} onChange={v => set("factor", v)} />
        <button type="button" onClick={() => onChange(list.filter((_, j) => j !== i))}>Remove {k}</button>
      </fieldset>;
    })}
  </fieldset>;
}
