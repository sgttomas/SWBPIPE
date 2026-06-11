import { ListPlus, Scale, SlidersHorizontal } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";

type LoadCase = PreviewModel["load_cases"][number];
type PrimitiveLoad = Record<string, unknown>;

type PrimitiveLoadView = {
  index: number;
  load: PrimitiveLoad;
  loadCase: LoadCase;
};

type LoadMetadataField = "status" | "kind";

export function LoadCaseManagerPanel({
  model,
  onQueueIntent,
  onSelect,
  selection
}: {
  model: PreviewModel;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  onSelect: (selection: EntityRef) => void;
  selection: EntityRef;
}) {
  const primitiveLoads = useMemo(() => primitiveLoadViews(model), [model]);
  const [selectedPrimitiveKey, setSelectedPrimitiveKey] = useState(primitiveLoads[0] ? primitiveKey(primitiveLoads[0]) : "");
  const selectedPrimitive =
    primitiveLoads.find((primitive) => primitiveKey(primitive) === selectedPrimitiveKey) ?? primitiveLoads[0] ?? null;
  const selectedLoadCase =
    (selection.type === "load" ? model.load_cases.find((loadCase) => loadCase.id === selection.id) : null) ??
    selectedPrimitive?.loadCase ??
    model.load_cases[0] ??
    null;
  const currentMagnitude = selectedPrimitive ? primitiveMagnitudeDisplay(selectedPrimitive.load) : "";
  const [proposedMagnitude, setProposedMagnitude] = useState(currentMagnitude);
  const [rationale, setRationale] = useState("user_entered_load_case_preview_change");
  const [metadataField, setMetadataField] = useState<LoadMetadataField>("status");
  const currentMetadataValue = selectedLoadCase ? loadCaseMetadataValue(selectedLoadCase, metadataField) : "";
  const [proposedMetadataValue, setProposedMetadataValue] = useState(currentMetadataValue);
  const [metadataRationale, setMetadataRationale] = useState("user_entered_load_case_metadata_preview_change");
  const changed = selectedPrimitive ? proposedMagnitude.trim() !== currentMagnitude : false;
  const intent =
    selectedPrimitive && changed
      ? buildLoadMagnitudeIntent({
          model,
          primitive: selectedPrimitive,
          proposedMagnitude,
          rationale
        })
      : null;
  const metadataChanged = selectedLoadCase
    ? proposedMetadataValue.trim() !== "" && proposedMetadataValue.trim() !== currentMetadataValue
    : false;
  const metadataIntent =
    selectedLoadCase && metadataChanged
      ? buildLoadMetadataIntent({
          model,
          loadCase: selectedLoadCase,
          field: metadataField,
          proposedValue: proposedMetadataValue,
          rationale: metadataRationale
        })
      : null;
  const metadataOptions = loadMetadataOptions(model, metadataField, currentMetadataValue);

  useEffect(() => {
    if (!primitiveLoads.length) {
      setSelectedPrimitiveKey("");
      return;
    }
    if (!primitiveLoads.some((primitive) => primitiveKey(primitive) === selectedPrimitiveKey)) {
      setSelectedPrimitiveKey(primitiveKey(primitiveLoads[0]));
    }
  }, [primitiveLoads, selectedPrimitiveKey]);

  useEffect(() => {
    setProposedMagnitude(currentMagnitude);
    setRationale("user_entered_load_case_preview_change");
  }, [currentMagnitude, selectedPrimitiveKey]);

  useEffect(() => {
    setProposedMetadataValue(currentMetadataValue);
    setMetadataRationale("user_entered_load_case_metadata_preview_change");
  }, [currentMetadataValue, metadataField, selectedLoadCase?.id]);

  function handleSelectPrimitive(primitive: PrimitiveLoadView) {
    setSelectedPrimitiveKey(primitiveKey(primitive));
    onSelect({ type: "load", id: primitive.loadCase.id });
  }

  function handleSelectLoadCase(loadCase: LoadCase) {
    onSelect({ type: "load", id: loadCase.id });
    const firstPrimitive = primitiveLoads.find((primitive) => primitive.loadCase.id === loadCase.id);
    if (firstPrimitive) setSelectedPrimitiveKey(primitiveKey(firstPrimitive));
  }

  return (
    <section className="panel load-case-manager-panel" aria-label="Load case manager" data-testid="load-case-manager">
      <div className="panel-title">
        <Scale size={16} aria-hidden="true" />
        Load Cases
      </div>
      <p className="muted" data-testid="load-case-manager-summary">
        {model.load_cases.length} load cases; {primitiveLoads.length} primitive loads; {model.combinations?.length ?? 0} combinations;
        unit posture=single_unit_system_preview; saved_project_mutated=false
      </p>

      <div className="load-case-list" data-testid="load-case-manager-cases">
        {model.load_cases.map((loadCase) => (
          <button
            className={selection.type === "load" && selection.id === loadCase.id ? "load-case-row active" : "load-case-row"}
            data-testid={`load-manager-case-${loadCase.id}`}
            key={loadCase.id}
            onClick={() => handleSelectLoadCase(loadCase)}
            type="button"
          >
            <strong>{loadCase.label}</strong>
            <small>
              {loadCase.id}; {loadCase.kind}; {loadCase.status}; primitives={loadCase.primitive_loads?.length ?? 0}
            </small>
            <small>{loadCase.provenance}</small>
          </button>
        ))}
      </div>

      {selectedLoadCase ? (
        <section className="load-metadata-editor" aria-label="Load case metadata editor">
          <div className="load-editor-heading" data-testid="load-manager-selected-case">
            <SlidersHorizontal size={14} aria-hidden="true" />
            <strong>{selectedLoadCase.id}</strong>
            <span>
              field={metadataField}; current={currentMetadataValue}; path={metadataField}
            </span>
          </div>
          <div className="load-metadata-controls">
            <label>
              <span>Field</span>
              <select
                aria-label="Load case metadata field"
                data-testid="load-manager-metadata-field"
                onChange={(event) => setMetadataField(event.target.value as LoadMetadataField)}
                value={metadataField}
              >
                <option value="status">Status</option>
                <option value="kind">Kind</option>
              </select>
            </label>
            <label>
              <span>Value</span>
              <select
                aria-label="Load case metadata value"
                data-testid="load-manager-metadata-value"
                onChange={(event) => setProposedMetadataValue(event.target.value)}
                value={proposedMetadataValue}
              >
                {metadataOptions.map((value) => (
                  <option key={value} value={value}>
                    {value}
                  </option>
                ))}
              </select>
            </label>
            <label>
              <span>Rationale</span>
              <input
                aria-label="Load case metadata rationale"
                data-testid="load-manager-metadata-rationale"
                onChange={(event) => setMetadataRationale(event.target.value)}
                value={metadataRationale}
              />
            </label>
            <button
              data-testid="queue-load-metadata-intent"
              disabled={!metadataIntent}
              onClick={() => metadataIntent && onQueueIntent(metadataIntent)}
              title="Queue load-case metadata operation"
              type="button"
            >
              <ListPlus size={14} aria-hidden="true" />
              Queue metadata
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-metadata-preview">
            {metadataIntent
              ? `${metadataIntent.operation_id}; before=${metadataIntent.change.before}; after=${metadataIntent.change.after}; unit=${metadataIntent.change.unit}; ${metadataIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : `current=${currentMetadataValue}; no changed ${metadataField} queued`}
          </p>
        </section>
      ) : null}

      <div className="load-primitive-list" data-testid="load-case-manager-primitives">
        {primitiveLoads.map((primitive) => (
          <button
            className={selectedPrimitive && primitiveKey(primitive) === primitiveKey(selectedPrimitive) ? "primitive-load-row active" : "primitive-load-row"}
            data-testid={`load-manager-primitive-${primitiveId(primitive.load)}`}
            key={primitiveKey(primitive)}
            onClick={() => handleSelectPrimitive(primitive)}
            type="button"
          >
            <strong>
              {primitiveCategory(primitive.load)}; {primitiveMagnitudeDisplay(primitive.load)} {primitiveUnit(primitive.load)}
            </strong>
            <small>
              {primitiveId(primitive.load)}; {primitiveTarget(primitive.load)}; {primitiveDirection(primitive.load)};
              dimension={primitiveDimension(primitive.load)}
            </small>
          </button>
        ))}
      </div>

      {selectedPrimitive ? (
        <section className="load-magnitude-editor" aria-label="Primitive load magnitude editor">
          <div className="load-editor-heading" data-testid="load-manager-selected-primitive">
            <SlidersHorizontal size={14} aria-hidden="true" />
            <strong>{primitiveId(selectedPrimitive.load)}</strong>
            <span>
              {selectedPrimitive.loadCase.id}; path=primitive_loads.{selectedPrimitive.index}.magnitude.value
            </span>
          </div>
          <div className="load-editor-controls">
            <label>
              <span>Magnitude</span>
              <input
                aria-label="Primitive load magnitude"
                data-testid="load-manager-magnitude-value"
                onChange={(event) => setProposedMagnitude(event.target.value)}
                value={proposedMagnitude}
              />
            </label>
            <label>
              <span>Rationale</span>
              <input
                aria-label="Load case edit rationale"
                data-testid="load-manager-rationale"
                onChange={(event) => setRationale(event.target.value)}
                value={rationale}
              />
            </label>
            <button
              data-testid="queue-load-magnitude-intent"
              disabled={!intent}
              onClick={() => intent && onQueueIntent(intent)}
              title="Queue load-magnitude operation"
              type="button"
            >
              <ListPlus size={14} aria-hidden="true" />
              Queue magnitude
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-edit-preview">
            {intent
              ? `${intent.operation_id}; before=${intent.change.before}; after=${intent.change.after}; unit=${intent.change.unit}; ${intent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : `current=${currentMagnitude} ${primitiveUnit(selectedPrimitive.load)}; no changed magnitude queued`}
          </p>
        </section>
      ) : (
        <p className="muted" data-testid="load-case-manager-empty">
          No primitive loads are available in the current model.
        </p>
      )}

      {model.combinations?.length ? (
        <div className="load-combination-list" data-testid="load-case-manager-combinations">
          {model.combinations.map((combination) => (
            <article className="load-combination-row" data-testid={`load-manager-combination-${combination.id}`} key={combination.id}>
              <strong>{combination.label}</strong>
              <small>
                {combination.id}; basis={combination.basis}; terms=
                {combination.terms.map((term) => `${term.load_case} x ${term.factor}`).join("; ")}
              </small>
            </article>
          ))}
        </div>
      ) : null}

      <small className="report-note" data-testid="load-case-manager-boundary">
        Load edits are queued as structured operations, applied only after user action, and remain local-session changes
        until saved. This panel does not add rule-code combinations, protected standards content, or professional
        approval.
      </small>
    </section>
  );
}

function loadCaseMetadataValue(loadCase: LoadCase, field: LoadMetadataField): string {
  return loadCase[field];
}

function loadMetadataOptions(model: PreviewModel, field: LoadMetadataField, currentValue: string): string[] {
  const values = new Set<string>();
  for (const loadCase of model.load_cases) {
    const value = loadCaseMetadataValue(loadCase, field).trim();
    if (value) values.add(value);
  }
  if (currentValue.trim()) values.add(currentValue.trim());
  values.add("TBD");
  if (field === "status") values.add("preview_only");
  if (field === "kind") values.add("primitive_user_load");
  return Array.from(values).sort((left, right) => {
    if (left === currentValue) return -1;
    if (right === currentValue) return 1;
    if (left === "TBD") return 1;
    if (right === "TBD") return -1;
    return left.localeCompare(right);
  });
}

function buildLoadMetadataIntent({
  model,
  loadCase,
  field,
  proposedValue,
  rationale
}: {
  model: PreviewModel;
  loadCase: LoadCase;
  field: LoadMetadataField;
  proposedValue: string;
  rationale: string;
}): EditorOperationIntent {
  const operationToken = `${safeToken(loadCase.id)}-${field}`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Load",
      ref: loadCase.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "update_load",
      field_label: `Load case ${field}`,
      field_path: field,
      before: loadCaseMetadataValue(loadCase, field),
      after: proposedValue.trim() || "TBD",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered load-case metadata"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `load case metadata edit intent for ${model.project.id}`
  };
}

function primitiveLoadViews(model: PreviewModel): PrimitiveLoadView[] {
  return model.load_cases.flatMap((loadCase) =>
    (loadCase.primitive_loads ?? []).map((load, index) => ({
      index,
      load,
      loadCase
    }))
  );
}

function buildLoadMagnitudeIntent({
  model,
  primitive,
  proposedMagnitude,
  rationale
}: {
  model: PreviewModel;
  primitive: PrimitiveLoadView;
  proposedMagnitude: string;
  rationale: string;
}): EditorOperationIntent {
  const fieldPath = `primitive_loads.${primitive.index}.magnitude.value`;
  const operationToken = `${safeToken(primitive.loadCase.id)}-${safeToken(primitiveId(primitive.load))}-magnitude`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Load",
      ref: primitive.loadCase.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "update_load",
      field_label: `${primitiveCategory(primitive.load)} primitive magnitude`,
      field_path: fieldPath,
      before: primitiveMagnitudeDisplay(primitive.load),
      after: proposedMagnitude.trim() || "TBD",
      unit: primitiveUnit(primitive.load),
      dimension: primitiveDimension(primitive.load),
      source_note: "unit metadata required"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `load magnitude edit intent for ${model.project.id}`
  };
}

function primitiveKey(primitive: PrimitiveLoadView): string {
  return `${primitive.loadCase.id}::${primitive.index}::${primitiveId(primitive.load)}`;
}

function primitiveId(load: PrimitiveLoad): string {
  return stringField(load, "id");
}

function primitiveCategory(load: PrimitiveLoad): string {
  return stringField(load, "category");
}

function primitiveDirection(load: PrimitiveLoad): string {
  return stringField(load, "direction");
}

function primitiveDimension(load: PrimitiveLoad): string {
  return stringField(load, "dimension");
}

function primitiveTarget(load: PrimitiveLoad): string {
  const target = load.target;
  if (!target || typeof target !== "object" || Array.isArray(target)) return "target:TBD";
  const record = target as Record<string, unknown>;
  const type = stringField(record, "type");
  const pipe = optionalString(record.pipe);
  const node = optionalString(record.node);
  if (pipe) return `${type}:${pipe}`;
  if (node) return `${type}:${node}`;
  return `${type}:TBD`;
}

function primitiveMagnitudeDisplay(load: PrimitiveLoad): string {
  const magnitude = load.magnitude;
  if (!magnitude || typeof magnitude !== "object" || Array.isArray(magnitude)) return "TBD";
  const value = (magnitude as Record<string, unknown>).value;
  return typeof value === "number" || typeof value === "string" ? String(value) : "TBD";
}

function primitiveUnit(load: PrimitiveLoad): string {
  const magnitude = load.magnitude;
  if (!magnitude || typeof magnitude !== "object" || Array.isArray(magnitude)) return "TBD";
  return optionalString((magnitude as Record<string, unknown>).unit) ?? "TBD";
}

function stringField(record: Record<string, unknown>, key: string): string {
  return optionalString(record[key]) ?? "TBD";
}

function optionalString(value: unknown): string | null {
  return typeof value === "string" && value.trim() ? value : null;
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}
