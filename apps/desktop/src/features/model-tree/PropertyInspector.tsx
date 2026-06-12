import { ListPlus, PlusCircle, SearchCheck, Trash2 } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import type { EditorOperationIntent, EditorOperationObjectType, EntityRef, OperationOutcome, PreviewModel } from "../../types";
import { entityLabel, selectedProperties } from "../model-workspace/modelView";

export function PropertyInspector({
  model,
  onQueueIntent,
  onValidateIntent,
  operationBusy = false,
  operationOutcomes = {},
  queuedIntents = [],
  selection
}: {
  model: PreviewModel;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  onValidateIntent?: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
  operationOutcomes?: Record<string, OperationOutcome>;
  queuedIntents?: EditorOperationIntent[];
  selection: EntityRef;
}) {
  const properties = selectedProperties(model, selection);
  const editableFields = useMemo(() => editorFieldOptions(model, selection), [model, selection]);
  const [selectedFieldPath, setSelectedFieldPath] = useState(editableFields[0]?.fieldPath ?? "");
  const [proposedValue, setProposedValue] = useState(editableFields[0]?.before ?? "");
  const [rationale, setRationale] = useState("user_entered_preview_change");
  const [sectionDraft, setSectionDraft] = useState(() => defaultSectionDraft(model, queuedIntents));
  const [materialDraft, setMaterialDraft] = useState(() => defaultMaterialDraft(model, queuedIntents));
  const [supportDraft, setSupportDraft] = useState(() => defaultSupportDraft(model, selection, queuedIntents));
  const selectedPipe = selection.type === "pipe" ? model.pipe_segments.find((pipe) => pipe.id === selection.id) : null;
  const selectedSupport = selection.type === "support" ? model.supports.find((support) => support.id === selection.id) : null;
  const selectedField = editableFields.find((field) => field.fieldPath === selectedFieldPath) ?? editableFields[0];
  const operationIntent = selectedField
    ? buildOperationIntent({
        field: selectedField,
        model,
        proposedValue,
        rationale,
        selection
      })
    : null;
  const sectionCreateIntent = isSectionDraftValid(model, sectionDraft) ? buildCreateSectionIntent(sectionDraft, model) : null;
  const materialCreateIntent = isMaterialDraftValid(model, materialDraft) ? buildCreateMaterialIntent(materialDraft, model) : null;
  const supportCreateIntent = isSupportDraftValid(model, supportDraft) ? buildCreateSupportIntent(supportDraft, model) : null;
  const pipeDeleteIntent = selectedPipe ? buildDeletePipeIntent(selectedPipe, model) : null;
  const supportDeleteIntent = selectedSupport ? buildDeleteSupportIntent(selectedSupport, model) : null;
  const inlineValidationOutcome = operationIntent
    ? matchingInlineValidationOutcome(operationOutcomes[operationIntentKey(operationIntent)], operationIntent)
    : null;
  const fieldChanged = Boolean(operationIntent && operationIntent.change.before !== operationIntent.change.after);

  useEffect(() => {
    const firstField = editableFields[0];
    setSelectedFieldPath(firstField?.fieldPath ?? "");
    setProposedValue(firstField?.before ?? "");
    setRationale("user_entered_preview_change");
  }, [editableFields, selection.id]);

  useEffect(() => {
    setSupportDraft(defaultSupportDraft(model, selection, queuedIntents));
  }, [model.project.id, model.nodes.length, model.supports.length, selection.id]);

  useEffect(() => {
    setMaterialDraft(defaultMaterialDraft(model, queuedIntents));
  }, [model.project.id, model.materials?.length]);

  useEffect(() => {
    setSectionDraft(defaultSectionDraft(model, queuedIntents));
  }, [model.project.id]);

  function handleFieldChange(fieldPath: string) {
    const nextField = editableFields.find((field) => field.fieldPath === fieldPath);
    setSelectedFieldPath(fieldPath);
    setProposedValue(nextField?.before ?? "");
  }

  function handleQueueIntent() {
    if (!operationIntent || !fieldChanged) return;
    onQueueIntent(operationIntent);
  }

  function handleQueueSupportIntent() {
    if (!supportCreateIntent) return;
    onQueueIntent(supportCreateIntent);
    setSupportDraft(defaultSupportDraftWithReserved(model, selection, [...queuedIntents, supportCreateIntent]));
  }

  function handleQueueDeleteSupportIntent() {
    if (!supportDeleteIntent) return;
    onQueueIntent(supportDeleteIntent);
  }

  function handleQueueDeletePipeIntent() {
    if (!pipeDeleteIntent) return;
    onQueueIntent(pipeDeleteIntent);
  }

  function handleQueueMaterialIntent() {
    if (!materialCreateIntent) return;
    onQueueIntent(materialCreateIntent);
    setMaterialDraft(defaultMaterialDraftWithReserved(model, [...queuedIntents, materialCreateIntent]));
  }

  function handleQueueSectionIntent() {
    if (!sectionCreateIntent) return;
    onQueueIntent(sectionCreateIntent);
    setSectionDraft(defaultSectionDraftWithReserved(model, [...queuedIntents, sectionCreateIntent]));
  }

  function updateSectionDraft<K extends keyof SectionDraft>(key: K, value: SectionDraft[K]) {
    setSectionDraft((current) => ({ ...current, [key]: value }));
  }

  function updateMaterialDraft<K extends keyof MaterialDraft>(key: K, value: MaterialDraft[K]) {
    setMaterialDraft((current) => ({ ...current, [key]: value }));
  }

  function updateSupportDraft<K extends keyof SupportDraft>(key: K, value: SupportDraft[K]) {
    setSupportDraft((current) => ({ ...current, [key]: value }));
  }

  function toggleSupportRestraint(restraint: string) {
    setSupportDraft((current) => {
      const present = current.restraints.includes(restraint);
      return {
        ...current,
        restraints: present ? current.restraints.filter((item) => item !== restraint) : [...current.restraints, restraint]
      };
    });
  }

  return (
    <div className="panel inspector" aria-label="Property inspector">
      <div className="panel-title">Properties</div>
      <h2>{entityLabel(model, selection.id)}</h2>
      <dl>
        {properties.map(([label, value]) => (
          <div key={label}>
            <dt>{label}</dt>
            <dd>{value}</dd>
          </div>
        ))}
      </dl>
      <section className="editor-intent" aria-label="Editor operation intent" data-testid="editor-intent-panel">
        <h3>Review-only edit intent</h3>
        {operationIntent ? (
          <>
            <div className="editor-intent-controls">
              <label>
                <span>Field</span>
                <select
                  aria-label="Intent field"
                  data-testid="editor-intent-field"
                  onChange={(event) => handleFieldChange(event.target.value)}
                  value={selectedField?.fieldPath ?? ""}
                >
                  {editableFields.map((field) => (
                    <option key={field.fieldPath} value={field.fieldPath}>
                      {field.label}
                    </option>
                  ))}
                </select>
              </label>
              <label>
                <span>Proposed value</span>
                <input
                  aria-label="Proposed editor value"
                  data-testid="editor-intent-value"
                  onChange={(event) => setProposedValue(event.target.value)}
                  value={proposedValue}
                />
              </label>
              <label>
                <span>Rationale</span>
                <input
                  aria-label="Editor intent rationale"
                  data-testid="editor-intent-rationale"
                  onChange={(event) => setRationale(event.target.value)}
                  value={rationale}
                />
              </label>
              <button
                data-testid="queue-editor-intent"
                disabled={!fieldChanged}
                onClick={handleQueueIntent}
                title="Queue as transient review-only operation"
                type="button"
              >
                <ListPlus size={14} aria-hidden="true" />
                Queue review intent
              </button>
              <button
                data-testid="validate-editor-intent-inline"
                disabled={!fieldChanged || operationBusy || !onValidateIntent}
                onClick={() => operationIntent && onValidateIntent?.(operationIntent)}
                title="Validate without applying"
                type="button"
              >
                <SearchCheck size={14} aria-hidden="true" />
                Validate intent
              </button>
            </div>
            <OperationIntentPreview intent={operationIntent} />
            <InlineValidationPreview intent={operationIntent} outcome={inlineValidationOutcome} />
            <IntentQueue intents={queuedIntents} />
          </>
        ) : (
          <p className="muted" data-testid="editor-intent-empty">
            Select an editable model entity to draft a structured operation intent.
          </p>
        )}
      </section>
      <section className="editor-intent" aria-label="Create section intent" data-testid="create-section-intent-panel">
        <h3>Create section</h3>
        <div className="editor-intent-controls">
          <label>
            <span>Section ID</span>
            <input
              aria-label="New section ID"
              data-testid="create-section-id"
              onChange={(event) => updateSectionDraft("id", event.target.value)}
              value={sectionDraft.id}
            />
          </label>
          <label>
            <span>Name</span>
            <input
              aria-label="New section name"
              data-testid="create-section-name"
              onChange={(event) => updateSectionDraft("name", event.target.value)}
              value={sectionDraft.name}
            />
          </label>
          <label>
            <span>Type</span>
            <select
              aria-label="New section type"
              data-testid="create-section-type"
              onChange={(event) => updateSectionDraft("sectionType", event.target.value)}
              value={sectionDraft.sectionType}
            >
              <option value="pipe">pipe</option>
            </select>
          </label>
          <label>
            <span>Outside diameter ({lengthUnit(model)})</span>
            <input
              aria-label="New section outside diameter"
              data-testid="create-section-od"
              inputMode="decimal"
              onChange={(event) => updateSectionDraft("outsideDiameter", event.target.value)}
              value={sectionDraft.outsideDiameter}
            />
          </label>
          <label>
            <span>Wall thickness ({lengthUnit(model)})</span>
            <input
              aria-label="New section wall thickness"
              data-testid="create-section-wall"
              inputMode="decimal"
              onChange={(event) => updateSectionDraft("wallThickness", event.target.value)}
              value={sectionDraft.wallThickness}
            />
          </label>
          <label>
            <span>Provenance</span>
            <input
              aria-label="New section provenance"
              data-testid="create-section-provenance"
              onChange={(event) => updateSectionDraft("provenance", event.target.value)}
              value={sectionDraft.provenance}
            />
          </label>
          <button
            data-testid="queue-create-section-intent"
            disabled={!sectionCreateIntent}
            onClick={handleQueueSectionIntent}
            title="Queue section create intent"
            type="button"
          >
            <PlusCircle size={14} aria-hidden="true" />
            Queue section
          </button>
        </div>
        {sectionCreateIntent ? <OperationIntentPreview intent={sectionCreateIntent} /> : null}
      </section>
      <section className="editor-intent" aria-label="Create material intent" data-testid="create-material-intent-panel">
        <h3>Create material</h3>
        <div className="editor-intent-controls">
          <label>
            <span>Material ID</span>
            <input
              aria-label="New material ID"
              data-testid="create-material-id"
              onChange={(event) => updateMaterialDraft("id", event.target.value)}
              value={materialDraft.id}
            />
          </label>
          <label>
            <span>Label</span>
            <input
              aria-label="New material label"
              data-testid="create-material-label"
              onChange={(event) => updateMaterialDraft("label", event.target.value)}
              value={materialDraft.label}
            />
          </label>
          <label>
            <span>Elastic modulus ({stressUnit(model)})</span>
            <input
              aria-label="New material elastic modulus"
              data-testid="create-material-elastic"
              inputMode="decimal"
              onChange={(event) => updateMaterialDraft("elasticModulus", event.target.value)}
              value={materialDraft.elasticModulus}
            />
          </label>
          <label>
            <span>Shear modulus ({stressUnit(model)})</span>
            <input
              aria-label="New material shear modulus"
              data-testid="create-material-shear"
              inputMode="decimal"
              onChange={(event) => updateMaterialDraft("shearModulus", event.target.value)}
              value={materialDraft.shearModulus}
            />
          </label>
          <label>
            <span>Thermal expansion ({thermalExpansionUnit(model)})</span>
            <input
              aria-label="New material thermal expansion"
              data-testid="create-material-thermal"
              inputMode="decimal"
              onChange={(event) => updateMaterialDraft("thermalExpansion", event.target.value)}
              value={materialDraft.thermalExpansion}
            />
          </label>
          <label>
            <span>Provenance</span>
            <input
              aria-label="New material provenance"
              data-testid="create-material-provenance"
              onChange={(event) => updateMaterialDraft("provenance", event.target.value)}
              value={materialDraft.provenance}
            />
          </label>
          <button
            data-testid="queue-create-material-intent"
            disabled={!materialCreateIntent}
            onClick={handleQueueMaterialIntent}
            title="Queue material create intent"
            type="button"
          >
            <PlusCircle size={14} aria-hidden="true" />
            Queue material
          </button>
        </div>
        {materialCreateIntent ? <OperationIntentPreview intent={materialCreateIntent} /> : null}
      </section>
      <section className="editor-intent" aria-label="Create support intent" data-testid="create-support-intent-panel">
        <h3>Create support</h3>
        <div className="editor-intent-controls">
          <label>
            <span>Support ID</span>
            <input
              aria-label="New support ID"
              data-testid="create-support-id"
              onChange={(event) => updateSupportDraft("id", event.target.value)}
              value={supportDraft.id}
            />
          </label>
          <label>
            <span>Label</span>
            <input
              aria-label="New support label"
              data-testid="create-support-label"
              onChange={(event) => updateSupportDraft("label", event.target.value)}
              value={supportDraft.label}
            />
          </label>
          <label>
            <span>Node</span>
            <select
              aria-label="New support node"
              data-testid="create-support-node"
              onChange={(event) => updateSupportDraft("node", event.target.value)}
              value={supportDraft.node}
            >
              {model.nodes.map((node) => (
                <option key={node.id} value={node.id}>
                  {node.label} ({node.id})
                </option>
              ))}
            </select>
          </label>
          <div className="editor-intent-checkbox-grid" aria-label="New support restraints">
            {RESTRAINT_OPTIONS.map((restraint) => (
              <label key={restraint}>
                <input
                  checked={supportDraft.restraints.includes(restraint)}
                  data-testid={`create-support-restraint-${restraint}`}
                  onChange={() => toggleSupportRestraint(restraint)}
                  type="checkbox"
                />
                <span>{restraint}</span>
              </label>
            ))}
          </div>
          <label>
            <span>Provenance</span>
            <input
              aria-label="New support provenance"
              data-testid="create-support-provenance"
              onChange={(event) => updateSupportDraft("provenance", event.target.value)}
              value={supportDraft.provenance}
            />
          </label>
          <button
            data-testid="queue-create-support-intent"
            disabled={!supportCreateIntent}
            onClick={handleQueueSupportIntent}
            title="Queue support create intent"
            type="button"
          >
            <PlusCircle size={14} aria-hidden="true" />
            Queue support
          </button>
        </div>
        {supportCreateIntent ? <OperationIntentPreview intent={supportCreateIntent} /> : null}
      </section>
      {pipeDeleteIntent ? (
        <section className="editor-intent" aria-label="Delete pipe intent" data-testid="delete-pipe-intent-panel">
          <h3>Delete pipe</h3>
          <div className="editor-intent-controls">
            <button
              data-testid="queue-delete-pipe-intent"
              onClick={handleQueueDeletePipeIntent}
              title="Queue pipe delete intent"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete pipe
            </button>
          </div>
          <OperationIntentPreview intent={pipeDeleteIntent} />
        </section>
      ) : null}
      {supportDeleteIntent ? (
        <section className="editor-intent" aria-label="Delete support intent" data-testid="delete-support-intent-panel">
          <h3>Delete support</h3>
          <div className="editor-intent-controls">
            <button
              data-testid="queue-delete-support-intent"
              onClick={handleQueueDeleteSupportIntent}
              title="Queue support delete intent"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete support
            </button>
          </div>
          <OperationIntentPreview intent={supportDeleteIntent} />
        </section>
      ) : null}
    </div>
  );
}

function InlineValidationPreview({
  intent,
  outcome
}: {
  intent: EditorOperationIntent;
  outcome: OperationOutcome | null;
}) {
  if (!outcome) {
    return (
      <p className="muted editor-inline-validation" data-testid="editor-intent-inline-validation-empty">
        Validate this draft intent to preview structured-operation findings before queuing or applying it.
      </p>
    );
  }

  return (
    <article className="editor-inline-validation" data-testid="editor-intent-inline-validation">
      <strong data-testid="editor-intent-inline-validation-status">
        {outcome.mode}; application_status={outcome.validation.application_status}; schema=
        {outcome.validation.schema_validation}; unit={outcome.validation.unit_validation}; before_state=
        {outcome.validation.before_state_validation}
      </strong>
      {outcome.diff_preview.map((row) => (
        <small data-testid="editor-intent-inline-validation-diff" key={`${row.entity_ref}-${row.field_path}-${row.after}`}>
          diff: {row.entity_ref} {row.field_path} {row.before} to {row.after} [{row.unit}]
        </small>
      ))}
      {outcome.diagnostics.length === 0 ? (
        <small data-testid="editor-intent-inline-validation-boundary">
          {intent.operation_id}; validate-only; no accepted model mutation; no professional approval.
        </small>
      ) : (
        outcome.diagnostics.map((diagnostic) => (
          <small data-testid={`editor-intent-inline-validation-diagnostic-${diagnostic.code}`} key={diagnostic.id}>
            {diagnostic.severity}: {diagnostic.code} - {diagnostic.message}
          </small>
        ))
      )}
    </article>
  );
}

type EditableField = {
  label: string;
  fieldPath: string;
  before: string;
  objectType: EditorOperationObjectType;
  changeKind: "set_field" | "update_load" | "update_support";
  dimension: string;
  unit: string;
  sourceNote: string;
};

type SupportDraft = {
  id: string;
  label: string;
  node: string;
  restraints: string[];
  provenance: string;
};

type MaterialDraft = {
  id: string;
  label: string;
  elasticModulus: string;
  shearModulus: string;
  thermalExpansion: string;
  provenance: string;
};

type SectionDraft = {
  id: string;
  name: string;
  sectionType: string;
  outsideDiameter: string;
  wallThickness: string;
  provenance: string;
};

const RESTRAINT_OPTIONS = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

function OperationIntentPreview({ intent }: { intent: EditorOperationIntent }) {
  return (
    <article className="editor-intent-preview" data-testid="editor-operation-preview">
      <div className="editor-intent-meta">
        <IntentFact label="Operation" value={`${intent.operation_id}; ${intent.operation_kind}; ${intent.operation_status}`} />
        <IntentFact label="Target" value={`${intent.target.object_type}; ${intent.target.ref}`} />
        <IntentFact
          label="Change"
          value={`${intent.change.change_kind}; ${intent.change.field_label}; ${intent.change.field_path}; before=${intent.change.before}; after=${intent.change.after}`}
        />
        <IntentFact label="Unit basis" value={`${intent.change.dimension}; unit=${intent.change.unit}; ${intent.change.source_note}`} />
        <IntentFact
          label="Validation"
          value={`${intent.validation.schema_validation}; ${intent.validation.constraint_validation}; ${intent.validation.unit_validation}; ${intent.validation.diff_preview_status}; ${intent.validation.application_status}`}
          testId="editor-intent-validation"
        />
        <IntentFact
          label="Audit boundary"
          value={`${intent.audit_boundary.mutation_route}; direct_model_mutation_allowed=${String(
            intent.audit_boundary.direct_model_mutation_allowed
          )}; requires_user_acceptance=${String(intent.audit_boundary.requires_user_acceptance)}; mutates_accepted_model_state=${String(
            intent.audit_boundary.mutates_accepted_model_state
          )}`}
          testId="editor-intent-audit-boundary"
        />
        <IntentFact
          label="Professional boundary"
          value={`human_review_required=${String(
            intent.professional_boundary.human_review_required
          )}; software_makes_compliance_claim=${String(
            intent.professional_boundary.software_makes_compliance_claim
          )}; software_makes_approval_claim=${String(intent.professional_boundary.software_makes_approval_claim)}`}
          testId="editor-intent-professional-boundary"
        />
        <IntentFact label="Rationale" value={intent.rationale || "TBD"} />
      </div>
    </article>
  );
}

function IntentQueue({ intents = [] }: { intents?: EditorOperationIntent[] }) {
  if (!intents.length) {
    return (
      <p className="muted" data-testid="editor-intent-queue-empty">
        No transient editor intents queued.
      </p>
    );
  }

  return (
    <section className="editor-intent-queue" aria-label="Queued editor intents" data-testid="editor-intent-queue">
      {intents.map((intent) => (
        <article key={`${intent.queue_id}-${intent.operation_id}`} data-testid={`queued-${intent.queue_id}`}>
          <strong>{intent.queue_id}</strong>
          <span>{intent.operation_id}</span>
          <small>
            {intent.change.field_path}: {intent.change.before} to {intent.change.after}; application_status=
            {intent.validation.application_status}; mutates_accepted_model_state=
            {String(intent.audit_boundary.mutates_accepted_model_state)}
          </small>
        </article>
      ))}
    </section>
  );
}

function IntentFact({ label, value, testId }: { label: string; value: string; testId?: string }) {
  return (
    <div className="editor-intent-fact" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function editorFieldOptions(model: PreviewModel, selection: EntityRef): EditableField[] {
  const material = model.materials?.find((item) => item.id === selection.id);
  if (material) {
    return [
      scalarField("Label", "label", material.label, "Material", "dimensionless", "none", "material label only"),
      quantityField("Elastic modulus", "elastic_modulus.value", material.elastic_modulus.value, "Material", "stress", material.elastic_modulus.unit),
      quantityField("Shear modulus", "shear_modulus.value", material.shear_modulus.value, "Material", "stress", material.shear_modulus.unit),
      quantityField(
        "Thermal expansion",
        "thermal_expansion_coefficient.value",
        material.thermal_expansion_coefficient?.value ?? "TBD",
        "Material",
        "thermal_expansion_coefficient",
        material.thermal_expansion_coefficient?.unit ?? "TBD"
      ),
      scalarField("Provenance", "provenance", material.provenance, "Material", "dimensionless", "none", "public/private source note")
    ];
  }

  const node = model.nodes.find((item) => item.id === selection.id);
  if (node) {
    return [
      scalarField("Label", "label", node.label, "Node", "dimensionless", "none", "node label only"),
      quantityField("X position", "position.x", node.position.x, "Node", "length", model.project.units.length ?? "m"),
      quantityField("Y position", "position.y", node.position.y, "Node", "length", model.project.units.length ?? "m"),
      quantityField("Z position", "position.z", node.position.z, "Node", "length", model.project.units.length ?? "m"),
      scalarField("Provenance", "provenance", node.provenance, "Node", "dimensionless", "none", "public/private source note")
    ];
  }

  const pipe = model.pipe_segments.find((item) => item.id === selection.id);
  if (pipe) {
    return [
      scalarField("Label", "label", pipe.label, "Element", "dimensionless", "none", "pipe segment label only"),
      quantityField(
        "Outside diameter",
        "section.outside_diameter.value",
        pipe.section.outside_diameter?.value ?? "TBD",
        "Element",
        "length",
        pipe.section.outside_diameter?.unit ?? "TBD"
      ),
      quantityField(
        "Wall thickness",
        "section.wall_thickness.value",
        pipe.section.wall_thickness?.value ?? "TBD",
        "Element",
        "length",
        pipe.section.wall_thickness?.unit ?? "TBD"
      ),
      scalarField("Material", "material", pipe.material, "Element", "dimensionless", "none", "material reference"),
      scalarField("Provenance", "provenance", pipe.provenance, "Element", "dimensionless", "none", "public/private source note")
    ];
  }

  const support = model.supports.find((item) => item.id === selection.id);
  if (support) {
    return [
      scalarField("Label", "label", support.label, "Support", "dimensionless", "none", "support label only", "update_support"),
      scalarField("Node", "node", support.node, "Support", "dimensionless", "none", "target node reference", "update_support"),
      scalarField("Restraints", "restraints", support.restraints.join(", "), "Support", "dimensionless", "none", "restraint direction set", "update_support"),
      scalarField("Provenance", "provenance", support.provenance, "Support", "dimensionless", "none", "public/private source note", "update_support")
    ];
  }

  const component = model.components.find((item) => item.id === selection.id);
  if (component) {
    return [
      scalarField("Label", "label", component.label, "Component", "dimensionless", "none", "component label only"),
      scalarField("Kind", "kind", component.kind, "Component", "dimensionless", "none", "component type"),
      scalarField("Node", "node", component.node, "Component", "dimensionless", "none", "target node reference"),
      scalarField("Provenance", "provenance", component.provenance, "Component", "dimensionless", "none", "public/private source note")
    ];
  }

  const loadCase = model.load_cases.find((item) => item.id === selection.id);
  if (loadCase) {
    const firstLoad = loadCase.primitive_loads?.[0] ?? {};
    const firstMagnitude = firstLoad.magnitude as { value?: unknown; unit?: unknown } | undefined;
    return [
      scalarField("Label", "label", loadCase.label, "Load", "dimensionless", "none", "load case label only", "update_load"),
      scalarField("Status", "status", loadCase.status, "Load", "dimensionless", "none", "load case status", "update_load"),
      scalarField("Kind", "kind", loadCase.kind, "Load", "dimensionless", "none", "load case kind", "update_load"),
      quantityField(
        "First primitive magnitude",
        "primitive_loads.0.magnitude.value",
        typeof firstMagnitude?.value === "number" || typeof firstMagnitude?.value === "string" ? firstMagnitude.value : "TBD",
        "Load",
        typeof firstLoad.dimension === "string" ? firstLoad.dimension : "TBD",
        typeof firstMagnitude?.unit === "string" ? firstMagnitude.unit : "TBD",
        "update_load"
      ),
      scalarField("Provenance", "provenance", loadCase.provenance, "Load", "dimensionless", "none", "public/private source note", "update_load")
    ];
  }

  const combination = model.combinations?.find((item) => item.id === selection.id);
  if (combination) {
    return [
      scalarField("Label", "label", combination.label, "Combination", "dimensionless", "none", "combination label only"),
      scalarField("Basis", "basis", combination.basis, "Combination", "dimensionless", "none", "mechanics or user rule basis"),
      scalarField(
        "Terms",
        "terms",
        combination.terms.map((term) => `${term.load_case} x ${term.factor}`).join("; "),
        "Combination",
        "dimensionless",
        "none",
        "explicit user-defined terms"
      ),
      scalarField("Provenance", "provenance", combination.provenance, "Combination", "dimensionless", "none", "public/private source note")
    ];
  }

  return [];
}

function scalarField(
  label: string,
  fieldPath: string,
  before: string,
  objectType: EditorOperationObjectType,
  dimension: string,
  unit: string,
  sourceNote: string,
  changeKind: EditableField["changeKind"] = "set_field"
): EditableField {
  return { label, fieldPath, before, objectType, dimension, unit, sourceNote, changeKind };
}

function quantityField(
  label: string,
  fieldPath: string,
  before: number | string,
  objectType: EditorOperationObjectType,
  dimension: string,
  unit: string,
  changeKind: EditableField["changeKind"] = "set_field"
): EditableField {
  return {
    label,
    fieldPath,
    before: String(before),
    objectType,
    dimension,
    unit,
    sourceNote: "unit metadata required",
    changeKind
  };
}

function buildOperationIntent({
  field,
  model,
  proposedValue,
  rationale,
  selection
}: {
  field: EditableField;
  model: PreviewModel;
  proposedValue: string;
  rationale: string;
  selection: EntityRef;
}): EditorOperationIntent {
  const operationToken = `${safeToken(selection.id)}-${safeToken(field.fieldPath)}`;
  return {
    operation_id: `op:editor-intent-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    target: {
      object_type: field.objectType,
      ref: selection.id
    },
    change: {
      change_id: `change:${operationToken}`,
      change_kind: field.changeKind,
      field_label: field.label,
      field_path: field.fieldPath,
      before: field.before,
      after: proposedValue.trim() || "TBD",
      unit: field.unit,
      dimension: field.dimension,
      source_note: field.sourceNote
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
    rationale: rationale.trim() || `preview edit intent for ${model.project.id}`
  };
}

function buildCreateSectionIntent(draft: SectionDraft, model: PreviewModel): EditorOperationIntent {
  const sectionId = draft.id.trim();
  const payload = {
    id: sectionId,
    name: draft.name.trim(),
    section_type: draft.sectionType.trim(),
    properties: {
      outside_diameter: { value: Number(draft.outsideDiameter), unit: lengthUnit(model) },
      wall_thickness: { value: Number(draft.wallThickness), unit: lengthUnit(model) }
    },
    provenance: draft.provenance.trim()
  };
  return {
    operation_id: `op:create-section-${safeToken(sectionId)}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Section",
      ref: sectionId
    },
    change: {
      change_id: `change:create-section:${safeToken(sectionId)}`,
      change_kind: "create_section",
      field_label: "Explicit pipe section",
      field_path: "sections",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: lengthUnit(model),
      dimension: "length",
      source_note: "explicit user-entered pipe section geometry"
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
    rationale: `explicit user-entered pipe section for ${model.project.id}; requires service validation before durable model change.`
  };
}

function buildCreateMaterialIntent(draft: MaterialDraft, model: PreviewModel): EditorOperationIntent {
  const materialId = draft.id.trim();
  const payload: Record<string, unknown> = {
    id: materialId,
    label: draft.label.trim(),
    elastic_modulus: { value: Number(draft.elasticModulus), unit: stressUnit(model) },
    shear_modulus: { value: Number(draft.shearModulus), unit: stressUnit(model) },
    provenance: draft.provenance.trim()
  };
  if (draft.thermalExpansion.trim()) {
    payload.thermal_expansion_coefficient = { value: Number(draft.thermalExpansion), unit: thermalExpansionUnit(model) };
  }
  return {
    operation_id: `op:create-material-${safeToken(materialId)}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Material",
      ref: materialId
    },
    change: {
      change_id: `change:create-material:${safeToken(materialId)}`,
      change_kind: "create_material",
      field_label: "Explicit material",
      field_path: "materials",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: stressUnit(model),
      dimension: "stress",
      source_note: "explicit user-entered material quantities"
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
    rationale: `explicit user-entered material for ${model.project.id}; requires service validation before durable model change.`
  };
}

function buildCreateSupportIntent(draft: SupportDraft, model: PreviewModel): EditorOperationIntent {
  const supportId = draft.id.trim();
  const payload = {
    id: supportId,
    label: draft.label.trim(),
    node: draft.node.trim(),
    restraints: draft.restraints,
    provenance: draft.provenance.trim()
  };
  return {
    operation_id: `op:create-support-${safeToken(supportId)}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Support",
      ref: supportId
    },
    change: {
      change_id: `change:create-support:${safeToken(supportId)}`,
      change_kind: "create_support",
      field_label: "Explicit support",
      field_path: "supports",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered support node and restraint tokens"
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
    rationale: `explicit user-entered support for ${model.project.id}; requires service validation before durable model change.`
  };
}

function buildDeleteSupportIntent(support: PreviewModel["supports"][number], model: PreviewModel): EditorOperationIntent {
  const supportId = support.id.trim();
  return {
    operation_id: `op:delete-support-${safeToken(supportId)}`,
    operation_kind: "delete",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Support",
      ref: supportId
    },
    change: {
      change_id: `change:delete-support:${safeToken(supportId)}`,
      change_kind: "delete_support",
      field_label: "Explicit support deletion",
      field_path: "supports",
      before: support.label,
      after: "not_present",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered support deletion; reference integrity required"
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
    rationale: `explicit user-entered support deletion for ${model.project.id}; requires reference validation before durable model change.`
  };
}

function buildDeletePipeIntent(pipe: PreviewModel["pipe_segments"][number], model: PreviewModel): EditorOperationIntent {
  const pipeId = pipe.id.trim();
  const before = `${pipe.label}; ${pipe.from}->${pipe.to}; material=${pipe.material}`;
  return {
    operation_id: `op:delete-pipe-${safeToken(pipeId)}`,
    operation_kind: "delete",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Element",
      ref: pipeId
    },
    change: {
      change_id: `change:delete-pipe:${safeToken(pipeId)}`,
      change_kind: "delete_pipe_run",
      field_label: "Explicit pipe deletion",
      field_path: "pipe_segments",
      before,
      after: "not_present",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered pipe deletion; primitive-load reference integrity required"
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
    rationale: `explicit user-entered pipe deletion for ${model.project.id}; requires reference validation before durable model change.`
  };
}

function matchingInlineValidationOutcome(
  outcome: OperationOutcome | undefined,
  intent: EditorOperationIntent
): OperationOutcome | null {
  if (!outcome) return null;
  if (outcome.operation_id !== intent.operation_id || outcome.change_id !== intent.change.change_id) return null;
  if (outcome.target_ref !== intent.target.ref || outcome.target_object_type !== intent.target.object_type) return null;
  if (outcome.change_kind !== intent.change.change_kind) return null;
  if (outcome.diff_preview.length === 0) {
    return outcome.validation.application_status === "blocked" ? outcome : null;
  }
  return outcome.diff_preview.some(
    (row) =>
      row.entity_ref === intent.target.ref &&
      row.field_path === intent.change.field_path &&
      row.before === intent.change.before &&
      row.after === intent.change.after
  )
    ? outcome
    : null;
}

function operationIntentKey(intent: EditorOperationIntent): string {
  return intent.queue_id ?? intent.operation_id;
}

function defaultSectionDraft(model: PreviewModel, queuedIntents: EditorOperationIntent[]): SectionDraft {
  return defaultSectionDraftWithReserved(model, queuedIntents);
}

function defaultSectionDraftWithReserved(model: PreviewModel, queuedIntents: EditorOperationIntent[]): SectionDraft {
  const id = nextSectionId(model, queuedIntents);
  return {
    id,
    name: `Section ${shortEntityToken(id)}`,
    sectionType: "pipe",
    outsideDiameter: "",
    wallThickness: "",
    provenance: "user_entered_local_preview"
  };
}

function defaultMaterialDraft(model: PreviewModel, queuedIntents: EditorOperationIntent[]): MaterialDraft {
  return defaultMaterialDraftWithReserved(model, queuedIntents);
}

function defaultMaterialDraftWithReserved(model: PreviewModel, queuedIntents: EditorOperationIntent[]): MaterialDraft {
  const id = nextMaterialId(model, queuedIntents);
  return {
    id,
    label: `Material ${shortEntityToken(id)}`,
    elasticModulus: "",
    shearModulus: "",
    thermalExpansion: "",
    provenance: "user_entered_local_preview"
  };
}

function defaultSupportDraft(model: PreviewModel, selection: EntityRef, queuedIntents: EditorOperationIntent[]): SupportDraft {
  return defaultSupportDraftWithReserved(model, selection, queuedIntents);
}

function defaultSupportDraftWithReserved(
  model: PreviewModel,
  selection: EntityRef,
  queuedIntents: EditorOperationIntent[]
): SupportDraft {
  const id = nextSupportId(model, queuedIntents);
  const selectedNode = selection.type === "node" && model.nodes.some((node) => node.id === selection.id) ? selection.id : null;
  const node = selectedNode ?? model.nodes[0]?.id ?? "";
  return {
    id,
    label: `Support ${shortEntityToken(id)}`,
    node,
    restraints: ["UX", "UY", "UZ"],
    provenance: "user_entered_local_preview"
  };
}

function nextSupportId(model: PreviewModel, queuedIntents: EditorOperationIntent[]): string {
  const reserved = new Set(model.supports.map((support) => support.id));
  for (const intent of queuedIntents) {
    if (intent.change.change_kind === "create_support") {
      reserved.add(intent.target.ref);
    }
  }
  for (let index = 1; index < 100000; index += 1) {
    const candidate = `support:S-${index}`;
    if (!reserved.has(candidate)) return candidate;
  }
  return "support:S-TBD";
}

function nextMaterialId(model: PreviewModel, queuedIntents: EditorOperationIntent[]): string {
  const reserved = new Set((model.materials ?? []).map((material) => material.id));
  for (const intent of queuedIntents) {
    if (intent.change.change_kind === "create_material") {
      reserved.add(intent.target.ref);
    }
  }
  for (let index = 1; index < 100000; index += 1) {
    const candidate = `material:M-${index}`;
    if (!reserved.has(candidate)) return candidate;
  }
  return "material:M-TBD";
}

function nextSectionId(model: PreviewModel, queuedIntents: EditorOperationIntent[]): string {
  const reserved = new Set((model.sections ?? []).map((section) => section.id));
  for (const intent of queuedIntents) {
    if (intent.change.change_kind === "create_section") {
      reserved.add(intent.target.ref);
    }
  }
  for (let index = 1; index < 100000; index += 1) {
    const candidate = `section:S-${index}`;
    if (!reserved.has(candidate)) return candidate;
  }
  return "section:S-TBD";
}

function isSectionDraftValid(model: PreviewModel, draft: SectionDraft): boolean {
  const outsideDiameter = Number(draft.outsideDiameter);
  const wallThickness = Number(draft.wallThickness);
  return (
    Boolean(draft.id.trim() && draft.name.trim() && draft.provenance.trim()) &&
    draft.sectionType === "pipe" &&
    lengthUnit(model) !== "TBD" &&
    isPositiveInput(draft.outsideDiameter) &&
    isPositiveInput(draft.wallThickness) &&
    wallThickness < outsideDiameter / 2 &&
    !(model.sections ?? []).some((section) => section.id === draft.id.trim())
  );
}

function isMaterialDraftValid(model: PreviewModel, draft: MaterialDraft): boolean {
  const thermalProvided = draft.thermalExpansion.trim() !== "";
  return (
    Boolean(draft.id.trim() && draft.label.trim() && draft.provenance.trim()) &&
    stressUnit(model) !== "TBD" &&
    isPositiveInput(draft.elasticModulus) &&
    isPositiveInput(draft.shearModulus) &&
    (!thermalProvided || (thermalExpansionUnit(model) !== "TBD" && isFiniteInput(draft.thermalExpansion))) &&
    !(model.materials ?? []).some((material) => material.id === draft.id.trim())
  );
}

function isSupportDraftValid(model: PreviewModel, draft: SupportDraft): boolean {
  return (
    Boolean(draft.id.trim() && draft.label.trim() && draft.node.trim() && draft.provenance.trim()) &&
    model.nodes.some((node) => node.id === draft.node.trim()) &&
    draft.restraints.length > 0 &&
    draft.restraints.every((restraint) => RESTRAINT_OPTIONS.includes(restraint)) &&
    !model.supports.some((support) => support.id === draft.id.trim())
  );
}

function lengthUnit(model: PreviewModel): string {
  return model.project.units.length ?? "TBD";
}

function stressUnit(model: PreviewModel): string {
  return model.project.units.pressure ?? "TBD";
}

function thermalExpansionUnit(model: PreviewModel): string {
  const temperature = model.project.units.temperature;
  return temperature ? `1/${temperature}` : "TBD";
}

function isFiniteInput(value: string): boolean {
  return value.trim() !== "" && Number.isFinite(Number(value));
}

function isPositiveInput(value: string): boolean {
  return isFiniteInput(value) && Number(value) > 0;
}

function shortEntityToken(value: string): string {
  const parts = value.split(":");
  return parts[parts.length - 1] || value;
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}
