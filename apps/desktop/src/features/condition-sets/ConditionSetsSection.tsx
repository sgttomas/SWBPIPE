// Model document 0.9.0 S3b (DEL-02-01 Work `model-document-0.9.0.md` §1.2, §4.2, §11): the
// model tree's Condition sets. Create, edit and delete a set and its keyed conditions
// through S3a's operations; each edit is checked by the operation engine and queued for
// Review/Apply. Every value is entered with its unit; an empty cell reads "not stated";
// nothing is prefilled or suggested. Members the solve does not read in 0.9.0 carry that
// statement; members the forms do not edit are shown and carried as authored.
import { useEffect, useId, useMemo, useState } from "react";
import type { ConditionSet, PreviewModel } from "../../types";
import { acceptedUnits, loadUnitCatalog, unitEntryMatchesDimension, type UnitCatalogRoute } from "../../services/unitCatalogService";
import { disabledBy, useReseed } from "../model-data/modelDataIntents";
import { loadedUnitCatalog } from "../project-units/projectUnitSelection";
import {
  CONDITION_DIRECTIONS,
  DIMENSIONS,
  NOT_STATED,
  NOT_USED_BY_SOLVE,
  createConditionSetIntent,
  deleteConditionSetIntent,
  draftFromSet,
  emptyConditionDraft,
  emptyConditionSetDraft,
  isPressureFree,
  pipesUsingSet,
  draftUnchanged,
  refusedMembers,
  uneditedMembers,
  type RefusedMember,
  setFromDraft,
  updateConditionSetIntent,
  type ConditionDraft,
  type ConditionSetDraft,
  type QuantityDraft,
  type TriState
} from "./conditionSetIntents";
import { EngineFeedback, useEngineCheckedQueue, type EngineCheckedQueueProps } from "./engineCheckedQueue";

export const PRESSURE_FREE_PRESSURE_NOTE =
  "No solve of this pressure-free document can use a condition's pressure (it has no pressure contract; a case's pressure use is refused, LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT). An entered pressure is kept as a stored member.";

export function ConditionSetsSection(props: EngineCheckedQueueProps) {
  const { model } = props;
  const sets = model.condition_sets ?? [];
  const [route, setRoute] = useState<UnitCatalogRoute | null>(null);
  useEffect(() => {
    let active = true;
    loadUnitCatalog().then((loaded) => { if (active) setRoute(loaded); }).catch(() => undefined);
    return () => { active = false; };
  }, []);
  return (
    <section className="model-tree-condition-sets" aria-label="Condition sets" data-testid="condition-sets-section">
      <h3>Condition sets</h3>
      <p className="model-data-note">
        A condition set states a line's operating values once; a pipe carries one set, and a load case uses a condition key.
        Every value is entered with its unit; nothing is prefilled.
      </p>
      {sets.length === 0 ? <p data-testid="condition-sets-empty">No condition sets in this model.</p> : (
        <ul className="model-data-list">
          {sets.map((set, index) => <ConditionSetRow key={`${set.id}:${index}`} set={set} route={route} {...props} />)}
        </ul>
      )}
      <details className="condition-set-create" data-testid="condition-set-create">
        <summary>New condition set</summary>
        <CreateConditionSet route={route} {...props} />
      </details>
    </section>
  );
}

function CreateConditionSet(props: EngineCheckedQueueProps & { route: UnitCatalogRoute | null }) {
  const [draft, setDraft] = useState<ConditionSetDraft>(emptyConditionSetDraft);
  const queue = useEngineCheckedQueue(props);
  function create() {
    void queue.submit(() => createConditionSetIntent(setFromDraft(draft))).then((queued) => { if (queued) setDraft(emptyConditionSetDraft()); });
  }
  return (
    <div data-testid="condition-set-create-form">
      <ConditionSetFields draft={draft} onChange={setDraft} model={props.model} route={props.route} idEditable testId="condition-set-new" />
      <button type="button" data-testid="condition-set-create-queue"
        {...disabledBy(queue.busy, [!draft.id.trim(), "Enter the set's id."], [!draft.provenance.trim(), "Enter the set's provenance (the source of its values)."])}
        onClick={create}>
        Check and queue new condition set
      </button>
      <EngineFeedback error={queue.error} notice={queue.notice} diagnostics={queue.diagnostics} testId="condition-set-new" />
    </div>
  );
}

function ConditionSetRow(props: EngineCheckedQueueProps & { set: ConditionSet; route: UnitCatalogRoute | null }) {
  const { set, model } = props;
  const [draft, setDraft] = useState<ConditionSetDraft>(() => draftFromSet(set));
  useReseed(set, () => setDraft(draftFromSet(set)));
  const queue = useEngineCheckedQueue(props);
  const users = useMemo(() => pipesUsingSet(model, set.id), [model, set.id]);
  const testId = `condition-set-${set.id}`;
  return (
    <li data-testid={`condition-set-row-${set.id}`}>
      <details>
        <summary>
          <strong>{set.label ?? set.id}</strong> <code>{set.id}</code>{" "}
          <span data-testid={`condition-set-summary-${set.id}`}>
            {(set.conditions ?? []).length} {(set.conditions ?? []).length === 1 ? "condition" : "conditions"}; {users.length ? `carried by ${users.join(", ")}` : "carried by no pipe"}
          </span>
        </summary>
        <ConditionSetFields draft={draft} onChange={setDraft} model={model} route={props.route} idEditable={false} testId={testId} />
        <div className="model-data-actions">
          <button type="button" data-testid={`condition-set-update-${set.id}`}
            {...disabledBy(queue.busy, [draftUnchanged(draft), "No member is edited; the stored record is unchanged."], [!draft.provenance.trim(), "Enter the set's provenance (the source of its values)."])}
            onClick={() => void queue.submit((canonicalize) => updateConditionSetIntent(canonicalize, set, setFromDraft(draft)))}>
            Check and queue edit
          </button>
          <button type="button" data-testid={`condition-set-delete-${set.id}`} {...disabledBy(queue.busy)}
            onClick={() => void queue.submit((canonicalize) => deleteConditionSetIntent(canonicalize, set))}>
            Check and queue removal
          </button>
        </div>
        {users.length ? (
          <p role="note" data-testid={`condition-set-in-use-${set.id}`}>
            In use by {users.join(", ")}. Removal is refused while any pipe carries this set (OP-CONDITION-SET-IN-USE); clear or reassign those pipes first.
          </p>
        ) : null}
        <EngineFeedback error={queue.error} notice={queue.notice} diagnostics={queue.diagnostics} testId={testId} />
      </details>
    </li>
  );
}

function NotUsed({ testId }: { testId?: string }) {
  return <small className="muted" data-testid={testId}> ({NOT_USED_BY_SOLVE})</small>;
}

export function ConditionSetFields({ draft, onChange, model, route, idEditable, testId }: {
  draft: ConditionSetDraft;
  onChange: (draft: ConditionSetDraft) => void;
  model: PreviewModel;
  route: UnitCatalogRoute | null;
  idEditable: boolean;
  testId: string;
}) {
  const set = <K extends keyof ConditionSetDraft>(key: K, value: ConditionSetDraft[K]) => onChange({ ...draft, [key]: value });
  const pressureFree = isPressureFree(model);
  const retained = draft.base ? uneditedMembers(draft.base.record) : [];
  // Stored values the engine refuses (for example null): shown as stored, kept unless removed.
  const refused = (draft.base ? refusedMembers(draft.base.record) : []).filter((item) => item.row === undefined
    ? !draft.removed.includes(item.path)
    : !draft.conditions.some((row) => row.origin === item.row && item.member && row.removed.includes(item.member)));
  const storedAt = (path: string) => refused.find((item) => item.path === path);
  // An explicit removal is the last action on that member: any entry typed into the same
  // field, or a nested field of it, is cleared with it, so nothing is written back.
  const remove = (item: RefusedMember) => {
    if (item.row === undefined) {
      onChange({ ...draft, ...clearedSetFields(item.path), removed: [...draft.removed, item.path] });
    } else if (item.member) {
      const member = item.member;
      onChange({ ...draft, conditions: draft.conditions.map((row) => row.origin === item.row ? { ...row, ...clearedConditionFields(member), removed: [...row.removed, member] } : row) });
    }
  };
  return (
    <fieldset className="condition-set-fields" data-testid={`${testId}-fields`}>
      <legend>{idEditable ? "New condition set" : `Condition set ${draft.id}`}</legend>
      <label>Id
        <input aria-label="Condition set id" data-testid={`${testId}-id`} value={draft.id} readOnly={!idEditable} placeholder={NOT_STATED}
          onChange={(event) => set("id", event.target.value)} />
      </label>
      <label>Label
        <input aria-label="Condition set label" data-testid={`${testId}-label`} value={draft.label} placeholder={NOT_STATED} onChange={(event) => set("label", event.target.value)} />
      </label>
      <label>Provenance (required)
        <input aria-label="Condition set provenance" data-testid={`${testId}-provenance`} value={draft.provenance} placeholder={NOT_STATED} onChange={(event) => set("provenance", event.target.value)} />
      </label>
      <QuantityInput label="Installation temperature" dimension={DIMENSIONS.temperature} route={route} value={draft.installationTemperature} stored={storedAt("installation_temperature")} onChange={(value) => set("installationTemperature", value)} testId={`${testId}-installation-temperature`} />

      <table data-testid={`${testId}-conditions`}>
        <caption>Conditions</caption>
        <thead>
          <tr>
            <th scope="col">Key</th>
            <th scope="col">Label</th>
            <th scope="col">Temperature</th>
            <th scope="col">Pressure{pressureFree ? " (not usable on this pressure-free document)" : ""}</th>
            <th scope="col">Contents density <NotUsed /></th>
            <th scope="col"><span className="visually-hidden">Actions</span></th>
          </tr>
        </thead>
        <tbody>
          {draft.conditions.length === 0 ? <tr><td colSpan={6} data-testid={`${testId}-conditions-empty`}>No conditions stated.</td></tr> : null}
          {draft.conditions.map((condition, index) => {
            const row = `${testId}-condition-${index}`;
            const update = (next: Partial<typeof condition>) => set("conditions", draft.conditions.map((item, at) => at === index ? { ...item, ...next } : item));
            const name = `Condition ${index + 1}`;
            return (
              <tr key={index} data-testid={row}>
                <td><input aria-label={`${name} key`} data-testid={`${row}-key`} value={condition.key} placeholder={NOT_STATED} onChange={(event) => update({ key: event.target.value })} /></td>
                <td><input aria-label={`${name} label`} data-testid={`${row}-label`} value={condition.label} placeholder={NOT_STATED} onChange={(event) => update({ label: event.target.value })} /></td>
                <td><QuantityInput compact label={`${name} temperature`} dimension={DIMENSIONS.temperature} route={route} value={condition.temperature} stored={condition.origin === undefined ? undefined : storedAt(`conditions[${condition.origin}].temperature`)} onChange={(value) => update({ temperature: value })} testId={`${row}-temperature`} /></td>
                <td><QuantityInput compact label={`${name} pressure`} dimension={DIMENSIONS.pressure} route={route} value={condition.pressure} stored={condition.origin === undefined ? undefined : storedAt(`conditions[${condition.origin}].pressure`)} onChange={(value) => update({ pressure: value })} testId={`${row}-pressure`} /></td>
                <td><QuantityInput compact label={`${name} contents density`} dimension={DIMENSIONS.density} route={route} value={condition.contentsDensity} stored={condition.origin === undefined ? undefined : storedAt(`conditions[${condition.origin}].contents_density`)} onChange={(value) => update({ contentsDensity: value })} testId={`${row}-contents-density`} /></td>
                <td><button type="button" data-testid={`${row}-remove`} onClick={() => set("conditions", draft.conditions.filter((_, at) => at !== index))}>Remove {name.toLowerCase()}</button></td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <button type="button" data-testid={`${testId}-add-condition`} onClick={() => set("conditions", [...draft.conditions, emptyConditionDraft()])}>Add condition</button>
      {pressureFree ? <p className="muted" data-testid={`${testId}-pressure-note`}>{PRESSURE_FREE_PRESSURE_NOTE}</p> : null}

      <fieldset>
        <legend>Design values <NotUsed testId={`${testId}-design-not-used`} /></legend>
        <QuantityInput label="Design temperature" dimension={DIMENSIONS.temperature} route={route} value={draft.designTemperature} stored={storedAt("design.temperature")} onChange={(value) => set("designTemperature", value)} testId={`${testId}-design-temperature`} />
        <QuantityInput label="Design pressure" dimension={DIMENSIONS.pressure} route={route} value={draft.designPressure} stored={storedAt("design.pressure")} onChange={(value) => set("designPressure", value)} testId={`${testId}-design-pressure`} />
      </fieldset>
      <fieldset>
        <legend>Peak values <NotUsed testId={`${testId}-peak-not-used`} /></legend>
        <QuantityInput label="Peak temperature" dimension={DIMENSIONS.temperature} route={route} value={draft.peakTemperature} stored={storedAt("peak.temperature")} onChange={(value) => set("peakTemperature", value)} testId={`${testId}-peak-temperature`} />
        <QuantityInput label="Peak pressure" dimension={DIMENSIONS.pressure} route={route} value={draft.peakPressure} stored={storedAt("peak.pressure")} onChange={(value) => set("peakPressure", value)} testId={`${testId}-peak-pressure`} />
      </fieldset>
      <fieldset>
        <legend>Added weight (uniform, on every pipe carrying the set)</legend>
        <QuantityInput label="Added weight magnitude" dimension={DIMENSIONS.forcePerLength} route={route} value={draft.addedWeightMagnitude} stored={storedAt("added_weight.magnitude")} onChange={(value) => set("addedWeightMagnitude", value)} testId={`${testId}-added-weight-magnitude`} />
        <label>Added weight direction
          <select aria-label="Added weight direction" data-testid={`${testId}-added-weight-direction`} value={draft.addedWeightDirection}
            onChange={(event) => set("addedWeightDirection", event.target.value as ConditionSetDraft["addedWeightDirection"])}>
            <option value="">{NOT_STATED}</option>
            {draft.addedWeightDirection === "stored" ? <option value="stored">{storedText(storedAt("added_weight.direction"))}</option> : null}
            {CONDITION_DIRECTIONS.map((direction) => <option key={direction} value={direction}>{direction}</option>)}
          </select>
        </label>
        <p className="muted">The model has no vertical axis: enter the direction with the magnitude.</p>
      </fieldset>
      <fieldset>
        <legend>Exposure</legend>
        <ExposureSelect label="Wind exposure" stored={storedAt("exposure.wind")} value={draft.wind} onChange={(value) => set("wind", value)} testId={`${testId}-exposure-wind`} />
        <ExposureSelect label="Snow and ice exposure" stored={storedAt("exposure.snow_ice")} value={draft.snowIce} onChange={(value) => set("snowIce", value)} testId={`${testId}-exposure-snow-ice`} notUsed />
        <ExposureSelect label="Wave exposure" stored={storedAt("exposure.wave")} value={draft.wave} onChange={(value) => set("wave", value)} testId={`${testId}-exposure-wave`} notUsed />
      </fieldset>
      {refused.length ? (
        <div role="note" data-testid={`${testId}-refused`}>
          <strong>Stored values the operation engine refuses</strong> (shown as stored and written back unchanged unless you replace or remove them):
          <ul>{refused.map((item) => (
            <li key={item.path} data-testid={`${testId}-refused-${item.path}`}>
              <code>{item.path}: {JSON.stringify(item.value)}</code>{" "}
              {item.row === undefined || item.member ? (
                <button type="button" data-testid={`${testId}-refused-${item.path}-remove`} onClick={() => remove(item)}>Remove stored {item.path}</button>
              ) : <span>(remove the condition row to remove it)</span>}
            </li>
          ))}</ul>
        </div>
      ) : null}
      {retained.length ? (
        <div data-testid={`${testId}-retained`}>
          <strong>Retained as authored</strong> (not edited here; written back unchanged):
          <ul>{retained.map((line) => <li key={line}><code>{line}</code></li>)}</ul>
        </div>
      ) : null}
    </fieldset>
  );
}

/** A stored value the engine refuses, as an option or placeholder text. */
function storedText(stored: RefusedMember | undefined): string {
  return `stored: ${stored ? JSON.stringify(stored.value) : "a refused value"} (refused; kept)`;
}

function ExposureSelect({ label, value, onChange, testId, stored, notUsed = false }: { label: string; value: TriState; onChange: (value: TriState) => void; testId: string; stored?: RefusedMember; notUsed?: boolean }) {
  return (
    <label>{label}{notUsed ? <NotUsed testId={`${testId}-not-used`} /> : null}
      <select aria-label={label} data-testid={testId} value={value} onChange={(event) => onChange(event.target.value as TriState)}>
        <option value="">{NOT_STATED}</option>
        {value === "stored" ? <option value="stored">{storedText(stored)}</option> : null}
        <option value="true">marked (true)</option>
        <option value="false">not marked (false)</option>
      </select>
    </label>
  );
}

/** One entered quantity: a value and an explicit unit, both blank meaning not stated. The
 * catalog's accepted units of the dimension are offered where the catalog is available;
 * the operation engine checks the unit and its dimension either way. */
export function QuantityInput({ label, dimension, route, value, onChange, testId, stored, compact = false }: {
  label: string;
  /** A stored value the engine refuses, shown in place of "not stated" while it is kept. */
  stored?: RefusedMember;
  dimension: string;
  route: UnitCatalogRoute | null;
  value: QuantityDraft;
  onChange: (value: QuantityDraft) => void;
  testId: string;
  compact?: boolean;
}) {
  const listId = useId();
  const catalog = loadedUnitCatalog(route);
  const units = catalog ? acceptedUnits(catalog).filter((entry) => unitEntryMatchesDimension(entry, dimension)).map((entry) => entry.symbol) : [];
  const inputs = <>
    <input aria-label={`${label} value`} data-testid={`${testId}-value`} inputMode="decimal" value={value.value} placeholder={stored ? storedText(stored) : NOT_STATED}
      onChange={(event) => onChange({ ...value, value: event.target.value })} />
    <input aria-label={`${label} unit`} data-testid={`${testId}-unit`} value={value.unit} placeholder={compact ? "unit" : `unit (${dimension})`}
      list={units.length ? listId : undefined} onChange={(event) => onChange({ ...value, unit: event.target.value })} />
    {units.length ? <datalist id={listId}>{units.map((unit) => <option key={unit} value={unit} />)}</datalist> : null}
  </>;
  if (compact) return <span className="condition-quantity" title={`${label}: ${dimension}`}>{inputs}</span>;
  return (
    <fieldset className="condition-quantity">
      <legend>{label} <small className="muted">({dimension}; enter a value and its unit, or leave both blank)</small></legend>
      {inputs}
    </fieldset>
  );
}

/** The draft fields a removed set path (or its parent) covers, blank again. */
function clearedSetFields(path: string): Partial<ConditionSetDraft> {
  const blank = { value: "", unit: "" };
  const fields: Record<string, Partial<ConditionSetDraft>> = {
    installation_temperature: { installationTemperature: blank },
    "design.temperature": { designTemperature: blank }, "design.pressure": { designPressure: blank },
    "peak.temperature": { peakTemperature: blank }, "peak.pressure": { peakPressure: blank },
    "added_weight.magnitude": { addedWeightMagnitude: blank }, "added_weight.direction": { addedWeightDirection: "" },
    "exposure.wind": { wind: "" }, "exposure.snow_ice": { snowIce: "" }, "exposure.wave": { wave: "" }
  };
  return Object.entries(fields).filter(([field]) => field === path || field.startsWith(`${path}.`)).reduce((out, [, cleared]) => ({ ...out, ...cleared }), {});
}

/** The condition-row draft field a removed member covers, blank again. */
function clearedConditionFields(member: string): Partial<ConditionDraft> {
  const blank = { value: "", unit: "" };
  return member === "temperature" ? { temperature: blank } : member === "pressure" ? { pressure: blank }
    : member === "contents_density" ? { contentsDensity: blank } : member === "key" ? { key: "" } : member === "label" ? { label: "" } : {};
}
