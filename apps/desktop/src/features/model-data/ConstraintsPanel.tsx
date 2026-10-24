// Constraints and equipment interfaces (W-13 §3.6, FR-MOD-002): typed records
// stored with the model. Nothing evaluates them (OD-1; HELP_HUMAN 2026-10-23,
// "keep them in"), and the panel says so. The user adds and edits records;
// every edit is a structured operation queued for review.
import { useState } from "react";
import type { EntityRef, EquipmentInterfaceRecord, ModelConstraint } from "../../types";
import { CONSTRAINT_KINDS, CONSTRAINT_STATES, RECORD_FAMILIES, disabledBy, nextModelDataId, recordIntent, stableJson, useReseed } from "./modelDataIntents";
import { subjectOf } from "./AssumptionsPanel";
import { useModelDataQueue, type ModelDataQueueProps } from "./useModelDataQueue";

export const NOT_CHECKED_NOTICE =
  "These records are not checked: SWBPIPE stores them with the model and does not evaluate constraints or equipment interfaces.";

export function ConstraintsPanel(props: ModelDataQueueProps & { selection?: EntityRef | null }) {
  return (
    <section className="panel model-data-panel" aria-label="Constraints and equipment interfaces" data-testid="constraints-panel">
      <div className="panel-title">Constraints and equipment interfaces</div>
      <p className="model-data-note" role="note" data-testid="constraints-not-checked">{NOT_CHECKED_NOTICE}</p>
      <ConstraintList {...props} />
      <EquipmentInterfaceList {...props} />
    </section>
  );
}

function ConstraintList(props: ModelDataQueueProps & { selection?: EntityRef | null }) {
  const { model, selection } = props;
  const records = model.constraints ?? [];
  const [name, setName] = useState("");
  const [kind, setKind] = useState<string>("clearance");
  const [state, setState] = useState<string>("proposed");
  const [statement, setStatement] = useState("");
  const { busy, error, notice, queue } = useModelDataQueue(props);
  const target = subjectOf(selection, model.project.id);
  function add() {
    const record: ModelConstraint = {
      constraint_id: nextModelDataId(model, "constraint:K", props.queuedIntents),
      name: name.trim(),
      constraint_kind: kind,
      state,
      provenance: "user_entered"
    };
    if (target) record.target_refs = [target];
    if (statement.trim()) record.statement = statement.trim();
    void queue((canonicalize) => recordIntent(canonicalize, RECORD_FAMILIES.constraint, "create", record)).then((queued) => {
      if (queued) {
        setName("");
        setStatement("");
      }
    });
  }
  return (
    <div data-testid="constraints-list">
      <h3>Constraints</h3>
      {records.length === 0 ? <p>No constraints in this model.</p> : (
        <ul className="model-data-list">
          {records.map((record) => <ConstraintRow key={record.constraint_id} record={record} {...props} />)}
        </ul>
      )}
      <fieldset className="model-data-fieldset">
        <legend>Add constraint</legend>
        <label>Name<input aria-label="Constraint name" data-testid="constraint-create-name" value={name} onChange={(event) => setName(event.target.value)} /></label>
        <label>Kind
          <select aria-label="Constraint kind" data-testid="constraint-create-kind" value={kind} onChange={(event) => setKind(event.target.value)}>
            {CONSTRAINT_KINDS.map((item) => <option key={item} value={item}>{item.replaceAll("_", " ")}</option>)}
          </select>
        </label>
        <label>State
          <select aria-label="Constraint state" data-testid="constraint-create-state" value={state} onChange={(event) => setState(event.target.value)}>
            {CONSTRAINT_STATES.map((item) => <option key={item} value={item}>{item.replaceAll("_", " ")}</option>)}
          </select>
        </label>
        <label>Statement<input aria-label="Constraint statement" data-testid="constraint-create-statement" value={statement} onChange={(event) => setStatement(event.target.value)} /></label>
        <p className="model-data-note">Target: {target ?? "none (select an entity to name it)"}</p>
        <button type="button" data-testid="constraint-create" {...disabledBy(busy, [!name.trim(), "Enter a name."])} onClick={add}>Add constraint</button>
      </fieldset>
      {error ? <p role="alert">{error}</p> : null}
      {notice ? <p role="status">{notice}</p> : null}
    </div>
  );
}

function ConstraintRow(props: ModelDataQueueProps & { record: ModelConstraint }) {
  const { record } = props;
  const [name, setName] = useState(record.name);
  const [state, setState] = useState(record.state);
  const [statement, setStatement] = useState(record.statement ?? "");
  useReseed(record, () => {
    setName(record.name);
    setState(record.state);
    setStatement(record.statement ?? "");
  });
  const { busy, error, queue } = useModelDataQueue(props);
  const id = record.constraint_id;
  const next = (): ModelConstraint => {
    const edited: ModelConstraint = { ...record, name: name.trim(), state };
    if (statement.trim()) edited.statement = statement.trim();
    else delete edited.statement;
    return edited;
  };
  const changed = stableJson(next()) !== stableJson(record);
  return (
    <li data-testid={`constraint-row-${id}`}>
      <code>{id}</code> {record.constraint_kind.replaceAll("_", " ")}{" "}
      <span>{(record.target_refs ?? []).join(", ") || "no target"}</span>
      <label>Name<input aria-label={`Name ${id}`} data-testid={`constraint-name-${id}`} value={name} onChange={(event) => setName(event.target.value)} /></label>
      <label>State
        <select aria-label={`State ${id}`} data-testid={`constraint-state-${id}`} value={state} onChange={(event) => setState(event.target.value)}>
          {CONSTRAINT_STATES.map((item) => <option key={item} value={item}>{item.replaceAll("_", " ")}</option>)}
        </select>
      </label>
      <label>Statement<input aria-label={`Statement ${id}`} data-testid={`constraint-statement-${id}`} value={statement} onChange={(event) => setStatement(event.target.value)} /></label>
      <div className="model-data-actions">
        <button type="button" data-testid={`constraint-edit-${id}`} {...disabledBy(busy, [!name.trim(), "Enter a name."], [!changed, "Nothing has changed."])}
          onClick={() => void queue((canonicalize) => recordIntent(canonicalize, RECORD_FAMILIES.constraint, "update", next(), record))}>
          Save
        </button>
        <button type="button" data-testid={`constraint-remove-${id}`} {...disabledBy(busy)}
          onClick={() => void queue((canonicalize) => recordIntent(canonicalize, RECORD_FAMILIES.constraint, "delete", record, record))}>
          Remove
        </button>
      </div>
      {error ? <p role="alert">{error}</p> : null}
    </li>
  );
}

function EquipmentInterfaceList(props: ModelDataQueueProps & { selection?: EntityRef | null }) {
  const { model, selection } = props;
  const records = model.equipment_interfaces ?? [];
  const [label, setLabel] = useState("");
  const [equipment, setEquipment] = useState("");
  const { busy, error, notice, queue } = useModelDataQueue(props);
  const node = selection?.type === "node" && model.nodes.some((item) => item.id === selection.id) ? selection.id : null;
  function add() {
    if (!node) return;
    const record: EquipmentInterfaceRecord = {
      id: nextModelDataId(model, "equipment-interface:E", props.queuedIntents),
      node_ref: node,
      provenance: "user_entered"
    };
    if (label.trim()) record.label = label.trim();
    if (equipment.trim()) record.equipment_label = equipment.trim();
    void queue((canonicalize) => recordIntent(canonicalize, RECORD_FAMILIES.equipmentInterface, "create", record)).then((queued) => {
      if (queued) {
        setLabel("");
        setEquipment("");
      }
    });
  }
  return (
    <div data-testid="equipment-interfaces-list">
      <h3>Equipment interfaces</h3>
      {records.length === 0 ? <p>No equipment interfaces in this model.</p> : (
        <ul className="model-data-list">
          {records.map((record) => <EquipmentInterfaceRow key={record.id} record={record} {...props} />)}
        </ul>
      )}
      <fieldset className="model-data-fieldset">
        <legend>Add equipment interface</legend>
        <label>Label<input aria-label="Interface label" data-testid="equipment-interface-create-label" value={label} onChange={(event) => setLabel(event.target.value)} /></label>
        <label>Equipment<input aria-label="Equipment label" data-testid="equipment-interface-create-equipment" value={equipment} onChange={(event) => setEquipment(event.target.value)} /></label>
        <p className="model-data-note">Node: {node ?? "select a node"}</p>
        <button type="button" data-testid="equipment-interface-create" {...disabledBy(busy, [!node, "Select a node."])} onClick={add}>Add equipment interface</button>
      </fieldset>
      {error ? <p role="alert">{error}</p> : null}
      {notice ? <p role="status">{notice}</p> : null}
    </div>
  );
}

function EquipmentInterfaceRow(props: ModelDataQueueProps & { record: EquipmentInterfaceRecord }) {
  const { record } = props;
  const [label, setLabel] = useState(record.label ?? "");
  const [equipment, setEquipment] = useState(record.equipment_label ?? "");
  useReseed(record, () => {
    setLabel(record.label ?? "");
    setEquipment(record.equipment_label ?? "");
  });
  const { busy, error, queue } = useModelDataQueue(props);
  const next = (): EquipmentInterfaceRecord => {
    const edited: EquipmentInterfaceRecord = { ...record };
    if (label.trim()) edited.label = label.trim();
    else delete edited.label;
    if (equipment.trim()) edited.equipment_label = equipment.trim();
    else delete edited.equipment_label;
    return edited;
  };
  const changed = stableJson(next()) !== stableJson(record);
  return (
    <li data-testid={`equipment-interface-row-${record.id}`}>
      <code>{record.id}</code> at <code>{record.node_ref}</code>
      <label>Label<input aria-label={`Label ${record.id}`} data-testid={`equipment-interface-label-${record.id}`} value={label} onChange={(event) => setLabel(event.target.value)} /></label>
      <label>Equipment<input aria-label={`Equipment ${record.id}`} data-testid={`equipment-interface-equipment-${record.id}`} value={equipment} onChange={(event) => setEquipment(event.target.value)} /></label>
      <div className="model-data-actions">
        <button type="button" data-testid={`equipment-interface-edit-${record.id}`} {...disabledBy(busy, [!changed, "Nothing has changed."])}
          onClick={() => void queue((canonicalize) => recordIntent(canonicalize, RECORD_FAMILIES.equipmentInterface, "update", next(), record))}>
          Save
        </button>
        <button type="button" data-testid={`equipment-interface-remove-${record.id}`} {...disabledBy(busy)}
          onClick={() => void queue((canonicalize) => recordIntent(canonicalize, RECORD_FAMILIES.equipmentInterface, "delete", record, record))}>
          Remove
        </button>
      </div>
      {error ? <p role="alert">{error}</p> : null}
    </li>
  );
}
