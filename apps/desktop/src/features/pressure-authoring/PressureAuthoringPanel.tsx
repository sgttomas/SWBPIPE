import { useEffect, useId, useRef, useState } from "react";
import { clone, makeRichIntent, parseQuantities, QueueFeedback, QuantityField, record, requireText, setMember, TextField, useRichQueue, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
import { setShearModelIntent, type Canonicalize } from "../model-settings/modelSettingsIntents";
import { canonicalJsonString } from "../../services/hashService";
import type { OperationBatch } from "../../services/operationBatchService";
import type { EntityRef, PreviewModel } from "../../types";
import { VirtualMultiTargetPicker } from "../workspace/VirtualTargetPicker";
import { carriedShearModel, DEFAULT_SHEAR_MODEL, documentShearModel, isPressureV3, isShearModel, PRESSURE_V3, pressureProfileTarget, SHEAR_MODEL_CHOICES, shearModelClause, SHEAR_MODEL_LABEL, type ShearModel } from "./pressureContract";
import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";
import { CopyFromCase } from "../load-cases/BulkFillControls";
import { copyPressureRegions } from "../load-cases/bulkFill";

export const MATERIAL_KEYS = ["constitutive_basis", "elastic_modulus", "shear_modulus", "poisson_ratio", "thermal_expansion_coefficient", "provenance"] as const;
export function materialProjection(material: object): DraftRecord {
  const value = material as DraftRecord;
  return Object.fromEntries(MATERIAL_KEYS.filter(key => value[key] !== undefined).map(key => [key, value[key]]));
}
/** G5/G5b: the optional coefficient is absent unless a value or unit was entered;
 * a half-entered quantity is refused by parseQuantities, never completed. */
function blankAlphaAbsent(draft: DraftRecord): DraftRecord {
  const alpha = record(draft.thermal_expansion_coefficient);
  const blank = (v: unknown) => v === undefined || String(v).trim() === "";
  return blank(alpha.value) && blank(alpha.unit) ? setMember(draft, "thermal_expansion_coefficient", undefined) : draft;
}
function checkedMaterial(draft: DraftRecord) {
  const after = record(parseQuantities(blankAlphaAbsent(draft)));
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
    <QuantityField label="Thermal expansion coefficient" dimension="thermal expansion coefficient (1/temperature); optional, absent when left blank" value={draft.thermal_expansion_coefficient} onChange={v => update("thermal_expansion_coefficient", v)} />
    <p>A material-level coefficient is recorded on the material. Thermal states that name an expansion law or their own coefficient use those instead.</p>
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
/** The line a document carrying the current pressure profile shows at rest, read from the applied document alone (a
 * profile only queued is said by its queue notice): pressure is activated, and, where the profile names an admitted
 * shear model, its straight members' theory in the control's words. Null where the document carries no current
 * profile. A profile with no admitted shear model says only the first sentence; the open items say the rest. */
export function pressureProfileLine(model: RichFormProps["model"]): string | null {
  if (!isPressureV3(model)) return null;
  const selected = documentShearModel(model);
  if (!selected) return "Pressure is activated.";
  const clause = shearModelClause(selected);
  return `Pressure is activated. ${clause.charAt(0).toUpperCase()}${clause.slice(1)}.`;
}
/** What an activated document still lacks before it will solve, from the facts the activation gate (a corrosion
 * allowance on every section), the regions editor (a case's pressure_regions, an explicit [] included) and the
 * selector (an admitted shear model) already read; each named as the tree names it. No other rule is stated here:
 * every other refusal is the solve's, shown in the Issues list. */
export type PressureOpenItem = Readonly<{ kind: "allowance" | "regions" | "shear"; id?: string; text: string }>;
export function pressureOpenItems(model: RichFormProps["model"]): PressureOpenItem[] {
  if (!isPressureV3(model)) return [];
  return [
    ...model.pipe_segments.filter(pipe => !Object.hasOwn(pipe.section, "corrosion_allowance"))
      .map(pipe => ({ kind: "allowance" as const, id: pipe.id, text: `${pipe.label || pipe.id} has not been given a corrosion allowance` })),
    ...model.load_cases.filter(loadCase => loadCase.pressure_regions === undefined)
      .map(loadCase => ({ kind: "regions" as const, id: loadCase.id, text: `${loadCase.label || loadCase.id} has not been given pressure regions` })),
    ...(documentShearModel(model) ? [] : [{ kind: "shear" as const, text: "whether shear deformation is included is chosen" }])
  ];
}
/** How many load cases carry pressure regions, by the solve's own test (a non-empty `pressure_regions`, a region at
 * zero pressure included), on a document carrying the current profile; null where every case has some, or where the
 * document carries no current profile. */
export function pressureRegionsCountLine(model: RichFormProps["model"]): string | null {
  if (!isPressureV3(model)) return null;
  const total = model.load_cases.length;
  const carrying = model.load_cases.filter(loadCase => (loadCase.pressure_regions?.length ?? 0) > 0).length;
  if (carrying === total) return null;
  return carrying === 0 ? "No load case has a pressure region yet." : `${carrying} of ${total} load cases have pressure regions.`;
}
/** What a case's pressure_regions states: not given, given as none, or how many. */
export function pressureRegionsFact(loadCase: RichFormProps["model"]["load_cases"][number]): string {
  const regions = loadCase.pressure_regions;
  if (regions === undefined) return "This case has not been given pressure regions.";
  if (regions.length === 0) return "This case has no pressure regions.";
  return `${regions.length} pressure region${regions.length === 1 ? "" : "s"}.`;
}
export const PRESSURE_REGION_DEFINITION = "A region is one unbranched, equal-bore run of pipe (and any inline expansion-joint connector) between its two terminals, changing direction only at realized bends.";
/** Show actual authored presence, including malformed objects. The one current
 * document format selects v3 explicitly; no retired input is reinterpreted. */
export function pressureModeText(model: RichFormProps["model"]): string {
  const contract = model.pressure_contract;
  const reauthor = `re-authored to ${PRESSURE_V3.mode}`;
  if (Object.hasOwn(model, "pressure_contract")) {
    if (!contract || typeof contract !== "object" || Array.isArray(contract)) return "invalid authored pressure contract (an object is required)";
    if (isPressureV3(model)) return contract.mode;
    const declared = `${contract.version}/${contract.mode}`;
    if (contract.version === "1.0.0" && contract.mode === "legacy_pressure_v1") return `${declared} (retired; solves are refused until the model is ${reauthor})`;
    return `${declared} (unsupported; solves are refused until the model is ${reauthor})`;
  }
  const legacy = model.load_cases.reduce((count, loadCase) => count + (loadCase.primitive_loads ?? []).filter(load => load.category === "pressure" || load.dimension === "pressure").length, 0);
  return legacy > 0
    ? `none (pressure-free namespace; this model carries ${legacy} legacy pressure primitive${legacy === 1 ? "" : "s"}, which ${legacy === 1 ? "is" : "are"} refused: remove ${legacy === 1 ? "it" : "them"} or re-author the model to ${PRESSURE_V3.mode})`
    : "none (pressure-free namespace; legacy pressure primitives are refused)";
}
/** A region member's label: a realized bend names its k; a geometry-only
 * bend's span must be realized before it can carry pressure. */
function memberLabel(model: RichFormProps["model"], pipe: RichFormProps["model"]["pipe_segments"][number]): string {
  const name = pipe.label || pipe.id;
  const bend = model.components.find(c => (c.kind === "bend" || c.kind === "elbow") && c.geometry?.bend_pipe_ref === pipe.id);
  if (!bend) return name;
  if (bend.mechanics_interface?.solver_consumption === "curved_bend_macro_element") {
    const k = bend.modifiers?.flexibility_factor_user_value?.value;
    return `${name} (realized bend ${bend.id}, k = ${k ?? "not entered"})`;
  }
  return `${name} (geometry-only bend span ${bend.id}: realize this bend first)`;
}
/** The selection a pressure-free document states in its model settings, or null. */
export function statedFreeShearModel(model: Pick<PreviewModel, "model_settings">): ShearModel | null {
  const stated = model.model_settings?.shear_model;
  return isShearModel(stated) ? stated : null;
}
const theoryName = (model: ShearModel) => model === "euler_bernoulli" ? "Euler–Bernoulli" : "Timoshenko";

/** The line under the selector, or null at rest: where the pick differs from what model settings state, or,
 * for a document without a contract that states no choice, from Euler–Bernoulli, which it solves as today. */
export function shearSelectionLine(model: PreviewModel, pick: ShearModel): string | null {
  if (model.pressure_contract != null) return null;
  const stated = statedFreeShearModel(model);
  if (stated !== null) return stated === pick ? null : `Model settings state ${theoryName(stated)}. Queuing this changes it to ${theoryName(pick)}.`;
  return pick === "euler_bernoulli" ? null
    : `This document solves as Euler–Bernoulli today. Activating the pressure contract changes its straight members to ${theoryName(pick)}.`;
}

/** The one batch a pick that differs from the stated model queues: an explicit set of the model setting to the
 * pick, then the activation, which carries that model into the contract and clears the setting. */
export async function shearChangeAndActivationBatch(model: PreviewModel, pick: ShearModel, canonicalize: Canonicalize = canonicalJsonString): Promise<OperationBatch> {
  const current = { schema_version: model.schema_version, ...(Object.hasOwn(model, "pressure_contract") ? { pressure_contract: model.pressure_contract } : {}) };
  const setting = await setShearModelIntent(canonicalize, model, pick);
  const profile = makeRichIntent({ object_type: "Model", ref: model.project.id }, "set_field", "pressure_profile", await canonicalize(current), pressureProfileTarget(model, pick), "Pressure profile");
  return { batch_id: `batch:pressure-profile-shear:${model.project.id}`, operations: [setting, profile] };
}

/** What a batch handler answers: true where the batch was registered for review. */
export type QueueBatchResult = void | boolean | Promise<void | boolean>;
/** `onSelect` is the load manager's: a case the open items name is a press that selects it. */
export function PressureAuthoringPanel(props: RichFormProps & { loadCaseId?: string; onQueueBatch?: (batch: OperationBatch) => QueueBatchResult; onSelect?: (selection: EntityRef) => void }) {
  const state = useRichQueue(props);
  // The batch path's own guards, as useRichQueue keeps for a single intent: one submission at a time, nothing
  // after unmount, and nothing once the model, pending changes or preparation epoch moved while it was formed.
  const latest = useRef(props);
  latest.current = props;
  const live = useRef(false);
  useEffect(() => {
    live.current = true;
    return () => { live.current = false; };
  }, []);
  const batchPending = useRef(false);
  const [batchBusy, setBatchBusy] = useState(false);
  const epochOf = (current: typeof props) => current.getPreparationEpoch?.() ?? current.preparationEpoch;
  const [profile, setProfile] = useState("");
  // T4-U8 (D-6/O3): null until the user changes the selector, so the queued profile
  // keeps the document's own selection (a pressure-free one is carried into the
  // contract), or else writes the Timoshenko default.
  const [shearChoice, setShearChoice] = useState<ShearModel | null>(null);
  const shearModel = shearChoice ?? carriedShearModel(props.model) ?? DEFAULT_SHEAR_MODEL;
  const selectionLine = shearSelectionLine(props.model, shearModel);
  const stated = props.model.pressure_contract == null ? statedFreeShearModel(props.model) : null;
  const [batchNotice, setBatchNotice] = useState("");
  // A pick that differs from the stated model queues the setting's change and the activation as one batch.
  async function queueProfile() {
    setBatchNotice("");
    const current = { schema_version: props.model.schema_version, ...(Object.hasOwn(props.model, "pressure_contract") ? { pressure_contract: props.model.pressure_contract } : {}) };
    if (stated === null || stated === shearModel || !props.onQueueBatch) {
      await state.queue({ object_type: "Model", ref: props.model.project.id }, "set_field", "pressure_profile", current, pressureProfileTarget(props.model, shearChoice ?? undefined), "Pressure profile");
      return;
    }
    if (batchPending.current || props.operationBusy) return;
    const captured = props;
    const capturedEpoch = epochOf(captured);
    batchPending.current = true;
    setBatchBusy(true);
    state.setError("");
    state.setNotice("");
    try {
      if (props.queuedIntents?.some(i => i.change.field_path === "pressure_profile" || i.change.field_path === "model_settings.shear_model"))
        throw new Error("Review or remove the pending pressure profile or shear deformation change before queuing another.");
      const batch = await shearChangeAndActivationBatch(captured.model, shearModel);
      if (!live.current) return;
      const now = latest.current;
      if (epochOf(now) !== capturedEpoch || now.model !== captured.model || now.queuedIntents !== captured.queuedIntents || now.operationBusy || !now.onQueueBatch)
        throw new Error("The model or pending changes changed. Review this choice and queue it again.");
      const registered = await now.onQueueBatch(batch);
      if (live.current && registered === true)
        setBatchNotice("Shear deformation change and pressure profile queued for validation and review.");
    } catch (e) {
      if (live.current) state.setError(e instanceof Error ? e.message : String(e));
    } finally {
      batchPending.current = false;
      if (live.current) setBatchBusy(false);
    }
  }
  const profileReasonId = useId();
  const missingAllowance = props.model.pipe_segments.some(pipe => !Object.hasOwn(pipe.section, "corrosion_allowance"));
  const profileDisabledReason = state.busy || batchBusy ? "Wait for the current change to finish queuing."
    : missingAllowance ? "Enter an explicit corrosion allowance on every member section before activating pressure; zero must be entered."
    : profile !== PRESSURE_V3.mode ? "Select a pressure profile before queuing this change." : undefined;
  const loadCase = props.model.load_cases.find(c => c.id === props.loadCaseId);
  const shearSelectRef = useRef<HTMLSelectElement>(null);
  const profileLine = pressureProfileLine(props.model);
  const openItems = pressureOpenItems(props.model);
  const countLine = pressureRegionsCountLine(props.model);
  const headingId = useId();
  const { onSelect } = props;
  return <section aria-labelledby={headingId}><h3 id={headingId}>Pressure</h3>
    {profileLine ? <p data-testid="pressure-profile-line">{profileLine}</p>
      : <p>Current model: {props.model.schema_version}; pressure mode: {pressureModeText(props.model)}.</p>}
    {countLine ? <p data-testid="pressure-regions-count">{countLine}</p> : null}
    {openItems.length ? <div data-testid="pressure-profile-open"><p>This model will not solve until:</p><ul>
      {openItems.map(item => <li key={`${item.kind}:${item.id ?? ""}`}>{item.kind === "shear"
        ? <button type="button" className="link-button" onClick={() => shearSelectRef.current?.focus()}>{item.text}</button>
        : item.kind === "regions" && item.id && onSelect
          ? <button type="button" className="link-button" onClick={() => onSelect({ type: "load", id: item.id! })}>{item.text}</button>
          : item.text}</li>)}
    </ul></div> : null}
    <fieldset disabled={state.busy || batchBusy}>
      <TextField label="Pressure profile" value={profile} choices={[PRESSURE_V3.mode]} onChange={setProfile} />
      <label className="form-field">{SHEAR_MODEL_LABEL}
        <select ref={shearSelectRef} aria-label={SHEAR_MODEL_LABEL} value={shearModel} onChange={e => { const value = e.target.value; if (isShearModel(value)) setShearChoice(value); }}>
          {SHEAR_MODEL_CHOICES.map(choice => <option key={choice.value} value={choice.value}>{choice.label}</option>)}
        </select>
      </label>
      {selectionLine ? <p data-testid="pressure-shear-selection-line">{selectionLine}</p> : null}
      <p>Realized arcs are Euler–Bernoulli under either choice.</p>
      <button type="button" disabled={!!profileDisabledReason} title={profileDisabledReason} aria-describedby={profileDisabledReason ? profileReasonId : undefined} onClick={() => void queueProfile()}>Queue pressure profile</button>
    </fieldset>
    {profileDisabledReason && <p id={profileReasonId}>{profileDisabledReason}</p>}
    <QueueFeedback {...state} />
    {batchNotice ? <p role="status" data-testid="pressure-shear-batch-notice">{batchNotice}</p> : null}
    <CreateExactMaterial {...props} />
    {loadCase && <PressureRegionsEditor key={loadCase.id} {...props} loadCase={loadCase} />}
  </section>;
}
export function PressureRegionsEditor(props: RichFormProps & { loadCase: RichFormProps["model"]["load_cases"][number] }) {
  const [rows, setRows] = useState<DraftRecord[]>(() => clone(props.loadCase.pressure_regions ?? []) as DraftRecord[]);
  useEffect(() => setRows(clone(props.loadCase.pressure_regions ?? []) as DraftRecord[]), [props.loadCase]);
  const state = useRichQueue(props);
  const update = (index: number, key: string, value: unknown) => setRows(current => current.map((row, i) => i === index ? setMember(row, key, value) : row));
  const pipes = props.model.pipe_segments.map(p => ({ value: p.id, label: memberLabel(props.model, p) }));
  const connectors = props.model.components.filter(component => component.kind === "expansion_joint" && "objective_connector" in component).map(component => ({ value: component.id, label: component.label || component.id }));
  const signed = isPressureV3(props.model);
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
  // With no rows: queue a stated "none" where the case has not been given regions; where it already states none,
  // there is nothing to queue (the fact line says so).
  const statedNone = Array.isArray(props.loadCase.pressure_regions) && props.loadCase.pressure_regions.length === 0;
  return <section aria-label="Case pressure regions"><h4>Pressure regions — {props.loadCase.label || props.loadCase.id}</h4>
    <p data-testid="pressure-regions-fact">{pressureRegionsFact(props.loadCase)}</p>
    <p>{PRESSURE_REGION_DEFINITION}</p>
    <fieldset disabled={state.busy}>
      <CopyFromCase what="pressure regions" cases={props.model.load_cases.filter(c => c.id !== props.loadCase.id && (c.pressure_regions?.length ?? 0) > 0).map(c => ({ value: c.id, label: c.label || c.id }))} onCopy={id => {
        const copied = copyPressureRegions(props.model.load_cases.find(c => c.id === id)?.pressure_regions, id, props.loadCase.id);
        setRows(copied);
        return `Copied ${copied.length} region${copied.length === 1 ? "" : "s"} from ${id}, with ids for this case: ${copied.map(r => String(r.id)).join(", ")}. Review the draft before queueing.`;
      }} />
      {rows.map((row, index) => <fieldset key={index}><legend>Region {index + 1}</legend>
      <TextField label={`Region ${index + 1} ID`} value={row.id} onChange={v => update(index, "id", v)} />
      <VirtualMultiTargetPicker label={`Region ${index + 1} member pipes`} options={pipes} values={Array.isArray(row.member_pipe_ids) ? row.member_pipe_ids as string[] : []} onChange={v => update(index, "member_pipe_ids", v)} />
      {props.model.schema_version === MODEL_SCHEMA_VERSION && <VirtualMultiTargetPicker label={`Region ${index + 1} member connectors`} options={connectors} values={Array.isArray(row.member_connector_ids) ? row.member_connector_ids as string[] : []} onChange={v => update(index, "member_connector_ids", v)} />}
      <TextField label={`Region ${index + 1} pressure basis`} value={row.pressure_basis} choices={["internal_differential_zero_external_v1"]} onChange={v => update(index, "pressure_basis", v)} />
      <QuantityField label={`Region ${index + 1} pressure`} dimension={signed ? "pressure; signed (a negative value is a net external differential; external-pressure stability and collapse are not assessed)" : "pressure; p >= 0"} value={row.pressure} onChange={v => update(index, "pressure", v)} />
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
      <button type="button" disabled={!rows.length && statedNone} onClick={() => void submit()}>{rows.length ? "Queue pressure regions" : "Queue no pressure regions for this case"}</button>
    </fieldset><QueueFeedback {...state} />
  </section>;
}
