import { ListPlus, PlusCircle, SearchCheck, Trash2 } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import type {
  EditorOperationIntent,
  EditorOperationObjectType,
  EntityRef,
  OperationOutcome,
  PreviewComponent,
  PreviewModel
} from "../../types";
import {
  acceptedUnits,
  describeUnitBasis,
  loadUnitCatalog,
  type UnitBasisDisplay,
  type UnitCatalogEntry,
  type UnitCatalogRoute
} from "../../services/unitCatalogService";
import { entityLabel, selectedProperties } from "../model-workspace/modelView";
import { dualUnitDisplay } from "../../services/unitConversion";

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
  const [proposedUnit, setProposedUnit] = useState(editableFields[0]?.unit ?? "");
  const [rationale, setRationale] = useState("user_entered_preview_change");
  const [sectionDraft, setSectionDraft] = useState(() => defaultSectionDraft(model, queuedIntents));
  const [materialDraft, setMaterialDraft] = useState(() => defaultMaterialDraft(model, queuedIntents));
  const [supportDraft, setSupportDraft] = useState(() => defaultSupportDraft(model, selection, queuedIntents));
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  const selectedNode = selection.type === "node" ? model.nodes.find((node) => node.id === selection.id) : null;
  const selectedPipe = selection.type === "pipe" ? model.pipe_segments.find((pipe) => pipe.id === selection.id) : null;
  const selectedSupport =
    selection.type === "support" ? model.supports.find((support) => support.id === selection.id) : null;
  const lengthBasis = describeUnitBasis(unitCatalogRoute, sectionDraft.lengthUnit, "length");
  const stressBasis = describeUnitBasis(unitCatalogRoute, materialDraft.stressUnit, "stress");
  const supportStiffnessBasis = describeUnitBasis(
    unitCatalogRoute,
    supportDraft.linearStiffnessUnit,
    "linear_stiffness"
  );
  const thermalExpansionBasis = describeUnitBasis(
    unitCatalogRoute,
    materialDraft.thermalExpansionUnit,
    "thermal_expansion_coefficient"
  );
  const lengthUnitOptions = unitOptions(unitCatalogRoute, "length", lengthUnit(model));
  const stressUnitOptions = unitOptions(unitCatalogRoute, "stress", stressUnit(model));
  const supportStiffnessUnitOptions = unitOptions(unitCatalogRoute, "linear_stiffness", linearStiffnessUnit(model));
  const thermalExpansionUnitOptions = unitOptions(
    unitCatalogRoute,
    "thermal_expansion_coefficient",
    thermalExpansionUnit(model)
  );
  const selectedField = editableFields.find((field) => field.fieldPath === selectedFieldPath) ?? editableFields[0];
  const selectedFieldUnitOptions = selectedField
    ? unitOptions(unitCatalogRoute, selectedField.dimension, selectedField.unit)
    : [];
  const selectedFieldUnitBasis = selectedField
    ? describeUnitBasis(unitCatalogRoute, proposedUnit || selectedField.unit, selectedField.dimension)
    : null;
  const operationIntent = selectedField
    ? buildOperationIntent({
        field: selectedField,
        model,
        proposedValue,
        proposedUnit,
        rationale,
        selection,
        unitCatalogRoute
      })
    : null;
  const sectionCreateIntent = isSectionDraftValid(model, sectionDraft)
    ? buildCreateSectionIntent(sectionDraft, model, unitCatalogRoute)
    : null;
  const materialCreateIntent = isMaterialDraftValid(model, materialDraft)
    ? buildCreateMaterialIntent(materialDraft, model, unitCatalogRoute)
    : null;
  const supportCreateIntent = isSupportDraftValid(model, supportDraft)
    ? buildCreateSupportIntent(supportDraft, model, unitCatalogRoute)
    : null;
  const nodeDeleteIntent = selectedNode ? buildDeleteNodeIntent(selectedNode, model) : null;
  const pipeDeleteIntent = selectedPipe ? buildDeletePipeIntent(selectedPipe, model) : null;
  const supportDeleteIntent = selectedSupport ? buildDeleteSupportIntent(selectedSupport, model) : null;
  const inlineValidationOutcome = operationIntent
    ? matchingInlineValidationOutcome(operationOutcomes[operationIntentKey(operationIntent)], operationIntent)
    : null;
  const fieldChanged = Boolean(
    operationIntent &&
    selectedField &&
    ((proposedValue.trim() || "TBD") !== selectedField.before ||
      (selectedField.unitEditable && (proposedUnit.trim() || selectedField.unit) !== selectedField.unit))
  );

  useEffect(() => {
    const firstField = editableFields[0];
    setSelectedFieldPath(firstField?.fieldPath ?? "");
    setProposedValue(firstField?.before ?? "");
    setProposedUnit(firstField?.unit ?? "");
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

  useEffect(() => {
    let active = true;
    loadUnitCatalog()
      .then((route) => {
        if (active) setUnitCatalogRoute(route);
      })
      .catch((error) => {
        if (!active) return;
        setUnitCatalogRoute({
          route: "unavailable_browser_preview",
          diagnostic: `UNIT-CATALOG-LOAD-FAILED: ${String(error)}`
        });
      });
    return () => {
      active = false;
    };
  }, []);

  function handleFieldChange(fieldPath: string) {
    const nextField = editableFields.find((field) => field.fieldPath === fieldPath);
    setSelectedFieldPath(fieldPath);
    setProposedValue(nextField?.before ?? "");
    setProposedUnit(nextField?.unit ?? "");
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

  function handleQueueDeleteNodeIntent() {
    if (!nodeDeleteIntent) return;
    onQueueIntent(nodeDeleteIntent);
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
        restraints: present
          ? current.restraints.filter((item) => item !== restraint)
          : [...current.restraints, restraint]
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
      <section className="inspector-context-grid" aria-label="Inspector review context">
        <DualUnitValue
          value={selectedField?.before ?? "TBD"}
          unit={selectedField?.unit ?? "TBD"}
          basis={selectedFieldUnitBasis}
        />
        <ProvenanceBlock rows={entityProvenanceRows(model, selection)} />
        <RequiredFlagList flags={requiredFlagsForSelection(model, selection, properties)} />
      </section>
      <UnitCatalogPanel
        route={unitCatalogRoute}
        bases={[lengthBasis, stressBasis, supportStiffnessBasis, thermalExpansionBasis]}
      />
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
                <span>
                  Proposed value
                  {selectedField?.unitEditable && selectedFieldUnitBasis ? ` (${selectedFieldUnitBasis.label})` : ""}
                </span>
                <input
                  aria-label="Proposed editor value"
                  data-testid="editor-intent-value"
                  onChange={(event) => setProposedValue(event.target.value)}
                  value={proposedValue}
                />
              </label>
              {selectedField?.unitEditable ? (
                <label>
                  <span>Unit</span>
                  <select
                    aria-label="Proposed editor unit"
                    data-testid="editor-intent-unit"
                    onChange={(event) => setProposedUnit(event.target.value)}
                    value={proposedUnit || selectedField.unit}
                  >
                    {selectedFieldUnitOptions.map((option) => (
                      <option key={option.symbol} value={option.symbol}>
                        {option.label}
                      </option>
                    ))}
                  </select>
                </label>
              ) : null}
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
            <span>Length unit</span>
            <select
              aria-label="New section length unit"
              data-testid="create-section-length-unit"
              onChange={(event) => updateSectionDraft("lengthUnit", event.target.value)}
              value={sectionDraft.lengthUnit}
            >
              {lengthUnitOptions.map((option) => (
                <option key={option.symbol} value={option.symbol}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>Outside diameter ({lengthBasis.label})</span>
            <input
              aria-label="New section outside diameter"
              data-testid="create-section-od"
              inputMode="decimal"
              onChange={(event) => updateSectionDraft("outsideDiameter", event.target.value)}
              value={sectionDraft.outsideDiameter}
            />
          </label>
          <label>
            <span>Wall thickness ({lengthBasis.label})</span>
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
            <span>Modulus unit</span>
            <select
              aria-label="New material modulus unit"
              data-testid="create-material-stress-unit"
              onChange={(event) => updateMaterialDraft("stressUnit", event.target.value)}
              value={materialDraft.stressUnit}
            >
              {stressUnitOptions.map((option) => (
                <option key={option.symbol} value={option.symbol}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>Elastic modulus ({stressBasis.label})</span>
            <input
              aria-label="New material elastic modulus"
              data-testid="create-material-elastic"
              inputMode="decimal"
              onChange={(event) => updateMaterialDraft("elasticModulus", event.target.value)}
              value={materialDraft.elasticModulus}
            />
          </label>
          <label>
            <span>Shear modulus ({stressBasis.label})</span>
            <input
              aria-label="New material shear modulus"
              data-testid="create-material-shear"
              inputMode="decimal"
              onChange={(event) => updateMaterialDraft("shearModulus", event.target.value)}
              value={materialDraft.shearModulus}
            />
          </label>
          <label>
            <span>Thermal expansion unit</span>
            <select
              aria-label="New material thermal expansion unit"
              data-testid="create-material-thermal-unit"
              onChange={(event) => updateMaterialDraft("thermalExpansionUnit", event.target.value)}
              value={materialDraft.thermalExpansionUnit}
            >
              {thermalExpansionUnitOptions.map((option) => (
                <option key={option.symbol} value={option.symbol}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>Thermal expansion ({thermalExpansionBasis.label})</span>
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
            <span>Linear stiffness unit</span>
            <select
              aria-label="New support linear stiffness unit"
              data-testid="create-support-stiffness-unit"
              onChange={(event) => updateSupportDraft("linearStiffnessUnit", event.target.value)}
              value={supportDraft.linearStiffnessUnit}
            >
              {supportStiffnessUnitOptions.map((option) => (
                <option key={option.symbol} value={option.symbol}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>Linear stiffness ({supportStiffnessBasis.label})</span>
            <input
              aria-label="New support linear stiffness"
              data-testid="create-support-stiffness"
              inputMode="decimal"
              onChange={(event) => updateSupportDraft("linearStiffness", event.target.value)}
              value={supportDraft.linearStiffness}
            />
          </label>
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
      {nodeDeleteIntent ? (
        <section className="editor-intent" aria-label="Delete node intent" data-testid="delete-node-intent-panel">
          <h3>Delete node</h3>
          <div className="editor-intent-controls">
            <button
              data-testid="queue-delete-node-intent"
              onClick={handleQueueDeleteNodeIntent}
              title="Queue node delete intent"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete node
            </button>
          </div>
          <OperationIntentPreview intent={nodeDeleteIntent} />
        </section>
      ) : null}
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
        {outcome.mode} — application status: {outcome.validation.application_status}; schema:{" "}
        {outcome.validation.schema_validation}; units: {outcome.validation.unit_validation}; prior state:{" "}
        {outcome.validation.before_state_validation}
      </strong>
      {outcome.diff_preview.map((row) => (
        <small
          data-testid="editor-intent-inline-validation-diff"
          key={`${row.entity_ref}-${row.field_path}-${row.after}`}
        >
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
  unitEditable: boolean;
};

type SupportDraft = {
  id: string;
  label: string;
  node: string;
  restraints: string[];
  linearStiffnessUnit: string;
  linearStiffness: string;
  provenance: string;
};

type MaterialDraft = {
  id: string;
  label: string;
  stressUnit: string;
  elasticModulus: string;
  shearModulus: string;
  thermalExpansionUnit: string;
  thermalExpansion: string;
  provenance: string;
};

type SectionDraft = {
  id: string;
  name: string;
  sectionType: string;
  lengthUnit: string;
  outsideDiameter: string;
  wallThickness: string;
  provenance: string;
};

type UnitOption = {
  symbol: string;
  label: string;
};

const RESTRAINT_OPTIONS = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

function OperationIntentPreview({ intent }: { intent: EditorOperationIntent }) {
  return (
    <article className="editor-intent-preview" data-testid="editor-operation-preview">
      <div className="editor-intent-meta">
        <IntentFact
          label="Operation"
          value={`${intent.operation_id}; ${intent.operation_kind}; ${intent.operation_status}`}
        />
        <IntentFact label="Target" value={`${intent.target.object_type}; ${intent.target.ref}`} />
        <IntentFact
          label="Change"
          value={`${intent.change.change_kind}; ${intent.change.field_label}; ${intent.change.field_path}; before=${intent.change.before}; after=${intent.change.after}`}
        />
        <IntentFact
          label="Unit basis"
          value={`${intent.change.dimension}; unit=${intent.change.unit}; ${intent.change.source_note}`}
        />
        <IntentFact
          label="Validation"
          value={`${intent.validation.schema_validation}; ${intent.validation.constraint_validation}; ${intent.validation.unit_validation}; ${intent.validation.diff_preview_status}; ${intent.validation.application_status}`}
          testId="editor-intent-validation"
        />
        <IntentFact
          label="Audit boundary"
          value={`Routed through the ${intent.audit_boundary.mutation_route.replaceAll("_", " ")}; ${
            intent.audit_boundary.direct_model_mutation_allowed
              ? "allows direct model mutation"
              : "no direct model mutation"
          }; ${intent.audit_boundary.requires_user_acceptance ? "requires your acceptance" : "no acceptance required"}; ${
            intent.audit_boundary.mutates_accepted_model_state
              ? "changes the accepted model"
              : "does not change the accepted model until applied"
          }`}
          testId="editor-intent-audit-boundary"
        />
        <IntentFact
          label="Professional boundary"
          value={`${intent.professional_boundary.human_review_required ? "Requires human review" : "No human review flagged"}; ${
            intent.professional_boundary.software_makes_compliance_claim
              ? "makes a compliance claim"
              : "no compliance claim"
          }; ${intent.professional_boundary.software_makes_approval_claim ? "makes an approval claim" : "no approval claim"}`}
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
            {intent.change.field_path}: {intent.change.before} to {intent.change.after}; status:{" "}
            {intent.validation.application_status};{" "}
            {intent.audit_boundary.mutates_accepted_model_state ? "changes accepted model" : "no accepted model change"}
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

function DualUnitValue({ value, unit, basis }: { value: string; unit: string; basis: UnitBasisDisplay | null }) {
  const dual = dualUnitDisplay(value, unit);
  return (
    <section
      className="inspector-context-card"
      data-testid="inspector-dual-unit-display"
      aria-label="Dual-unit display"
    >
      <span>Dual units</span>
      <strong>
        {dual.enteredValue} {dual.enteredUnit}
      </strong>
      <small data-testid="inspector-dual-unit-converted">
        {dual.convertedValue ? `≈ ${dual.convertedValue} ${dual.convertedUnit}` : "No SI/US companion for this unit."}
      </small>
      <small>{basis ? `Display basis: ${basis.label}` : "Display basis unavailable"}</small>
      <small>Display only — storage stays entered-units-preserved.</small>
    </section>
  );
}

function ProvenanceBlock({ rows }: { rows: Array<[string, string]> }) {
  return (
    <section className="inspector-context-card" data-testid="inspector-provenance-block" aria-label="Provenance">
      <span>Provenance</span>
      {rows.length > 0 ? (
        <dl>
          {rows.map(([label, value]) => (
            <div key={label}>
              <dt>{label}</dt>
              <dd>{value}</dd>
            </div>
          ))}
        </dl>
      ) : (
        <small>No provenance fields on this selected entity.</small>
      )}
    </section>
  );
}

function RequiredFlagList({ flags }: { flags: string[] }) {
  return (
    <section className="inspector-context-card" data-testid="inspector-required-flags" aria-label="Required flags">
      <span>Required</span>
      {flags.length > 0 ? (
        flags.map((flag) => (
          <strong className="required-flag" key={flag}>
            ⚑ {flag}
          </strong>
        ))
      ) : (
        <small>No missing selected fields detected.</small>
      )}
    </section>
  );
}

function entityProvenanceRows(model: PreviewModel, selection: EntityRef): Array<[string, string]> {
  const entity = selectedEntity(model, selection);
  if (!entity || typeof entity !== "object") return [];
  const record = entity as Record<string, unknown>;
  const rows: Array<[string, string]> = [];
  for (const key of [
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "redistribution_status",
    "review_status"
  ]) {
    const value = record[key];
    if (typeof value === "string" && value.trim()) rows.push([labelFromKey(key), value]);
  }
  const source = record.source ?? record.provenance ?? record.source_ref ?? null;
  if (source && typeof source === "object") {
    const sourceRecord = source as Record<string, unknown>;
    for (const key of ["source_name", "source_ref", "source_license", "review_status"]) {
      const value = sourceRecord[key];
      if (typeof value === "string" && value.trim()) rows.push([labelFromKey(key), value]);
    }
  } else if (typeof source === "string" && source.trim()) {
    rows.push(["Source", source]);
  }
  if (selection.type === "component") {
    const component = model.components.find((item) => item.id === selection.id);
    if (component?.geometry?.bend_geometry_source_reference) {
      rows.push(["Geometry source", component.geometry.bend_geometry_source_reference]);
    }
    if (component?.geometry?.branch_geometry_source_reference) {
      rows.push(["Geometry source", component.geometry.branch_geometry_source_reference]);
    }
    if (component?.geometry?.rigid_component_source_reference) {
      rows.push(["Geometry source", component.geometry.rigid_component_source_reference]);
    }
    if (component?.geometry?.expansion_joint_source_reference) {
      rows.push(["Geometry source", component.geometry.expansion_joint_source_reference]);
    }
    if (component?.geometry?.stiffness_behavior_reference) {
      rows.push(["Stiffness behavior", component.geometry.stiffness_behavior_reference]);
    }
    if (component?.geometry?.pressure_thrust_reference) {
      rows.push(["Pressure thrust", component.geometry.pressure_thrust_reference]);
    }
    if (component?.geometry?.hardware_reference) {
      rows.push(["Hardware reference", component.geometry.hardware_reference]);
    }
    if (component?.geometry?.manufacturer_reference) {
      rows.push(["Manufacturer reference", component.geometry.manufacturer_reference]);
    }
    if (component?.modifiers?.source_reference) {
      rows.push(["Modifier source", component.modifiers.source_reference]);
    }
    if (component?.mechanics_interface?.solver_consumption) {
      rows.push(["Solver consumption", component.mechanics_interface.solver_consumption]);
    }
  }
  return rows;
}

function requiredFlagsForSelection(
  model: PreviewModel,
  selection: EntityRef,
  properties: Array<[string, string]>
): string[] {
  const flags = properties.filter(([, value]) => value.trim() === "" || value.trim() === "TBD").map(([label]) => label);
  const entity = selectedEntity(model, selection);
  if (entity && typeof entity === "object") {
    const record = entity as Record<string, unknown>;
    if (!record.provenance && !record.source && !record.source_ref) flags.push("provenance");
  }
  if (selection.type === "component") {
    const component = model.components.find((item) => item.id === selection.id);
    if (component && isBendComponent(component)) {
      const geometry = component.geometry;
      const geometryComplete = Boolean(
        geometry?.bend_radius &&
        geometry.bend_angle &&
        geometry.bend_plane_orientation?.trim() &&
        geometry.bend_geometry_source_reference?.trim()
      );
      const modifiersComplete = Boolean(
        component.modifiers?.sif_user_value &&
        component.modifiers.flexibility_factor_user_value &&
        component.modifiers.source_reference?.trim()
      );
      if (!geometryComplete) flags.push("BEND_GEOMETRY_INCOMPLETE");
      if (!modifiersComplete) flags.push("BEND_RULE_INPUT_MISSING");
    }
    if (component && isBranchComponent(component)) {
      const geometry = component.geometry;
      const geometryComplete = Boolean(
        geometry?.branch_header_pipe_ref?.trim() &&
        geometry.branch_branch_pipe_ref?.trim() &&
        geometry.branch_run_size &&
        geometry.branch_header_size &&
        geometry.branch_connection_angle &&
        geometry.branch_connection_type?.trim() &&
        geometry.branch_reinforcement_reference?.trim() &&
        geometry.branch_geometry_source_reference?.trim()
      );
      const modifiersComplete = Boolean(
        component.modifiers?.branch_header_sif_user_value &&
        component.modifiers.branch_branch_sif_user_value &&
        component.modifiers.flexibility_factor_user_value &&
        component.modifiers.source_reference?.trim()
      );
      if (!geometryComplete) flags.push("BRANCH_GEOMETRY_INCOMPLETE");
      if (!modifiersComplete) flags.push("BRANCH_RULE_INPUT_MISSING");
    }
    if (component && isRigidComponent(component)) {
      const geometry = component.geometry;
      const geometryComplete = Boolean(
        geometry?.rigid_pipe_ref?.trim() &&
        geometry.rigid_body_length &&
        geometry.end_a_size &&
        geometry.end_b_size &&
        geometry.weight &&
        geometry.center_of_gravity &&
        geometry.connection_end_a_reference?.trim() &&
        geometry.connection_end_b_reference?.trim() &&
        geometry.stiffness_behavior_reference?.trim() &&
        geometry.rigid_component_source_reference?.trim()
      );
      const modifiersComplete = Boolean(
        component.modifiers?.stiffness_scaling_user_value &&
        component.modifiers.linear_stiffness_user_value &&
        component.modifiers.rotational_stiffness_user_value &&
        component.modifiers.source_reference?.trim()
      );
      if (!geometryComplete) flags.push("RIGID_COMPONENT_GEOMETRY_INCOMPLETE");
      if (!modifiersComplete) flags.push("RIGID_COMPONENT_STIFFNESS_DATA_MISSING");
    }
    if (component && isExpansionJointComponent(component)) {
      const geometry = component.geometry;
      const geometryComplete = Boolean(
        geometry?.expansion_joint_pipe_ref?.trim() &&
        geometry.effective_area &&
        geometry.movement_limit &&
        geometry.hardware_reference?.trim() &&
        geometry.manufacturer_reference?.trim() &&
        geometry.pressure_thrust_reference?.trim() &&
        geometry.expansion_joint_source_reference?.trim()
      );
      const modifiersComplete = Boolean(
        component.modifiers?.axial_stiffness_user_value &&
        component.modifiers.lateral_stiffness_user_value &&
        component.modifiers.angular_stiffness_user_value &&
        component.modifiers.torsional_stiffness_user_value &&
        component.modifiers.source_reference?.trim()
      );
      if (!geometryComplete) flags.push("EXPANSION_JOINT_GEOMETRY_INCOMPLETE");
      if (!modifiersComplete) flags.push("EXPANSION_JOINT_STIFFNESS_DATA_MISSING");
    }
  }
  return Array.from(new Set(flags)).slice(0, 4);
}

function selectedEntity(model: PreviewModel, selection: EntityRef): unknown {
  if (selection.type === "node") return model.nodes.find((item) => item.id === selection.id);
  if (selection.type === "pipe") return model.pipe_segments.find((item) => item.id === selection.id);
  if (selection.type === "support") return model.supports.find((item) => item.id === selection.id);
  if (selection.type === "component") return model.components.find((item) => item.id === selection.id);
  if (selection.type === "load") return model.load_cases.find((item) => item.id === selection.id);
  return null;
}

function labelFromKey(key: string): string {
  return key.replaceAll("_", " ");
}

function UnitCatalogPanel({ bases, route }: { bases: UnitBasisDisplay[]; route: UnitCatalogRoute | null }) {
  return (
    <section className="editor-intent" aria-label="Unit catalog status" data-testid="property-unit-catalog-panel">
      <h3>Unit basis</h3>
      <div className="editor-intent-meta">
        <IntentFact label="Catalog" value={unitCatalogStatus(route)} testId="property-unit-catalog-status" />
        <IntentFact
          label="Field units"
          value={bases.map((basis) => basis.label).join("; ")}
          testId="property-unit-basis-summary"
        />
      </div>
    </section>
  );
}

function unitOptions(route: UnitCatalogRoute | null, dimensionId: string, fallbackSymbol: string): UnitOption[] {
  const fallbackBasis = describeUnitBasis(route, fallbackSymbol, dimensionId);
  const fallback = {
    symbol: fallbackBasis.symbol,
    label: fallbackBasis.label
  };
  if (route?.route !== "tauri_unit_catalog") return [fallback];

  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, dimensionId))
    .sort((left, right) => Number(right.canonical) - Number(left.canonical) || left.symbol.localeCompare(right.symbol))
    .map((entry) => {
      const basis = describeUnitBasis(route, entry.symbol, dimensionId);
      return {
        symbol: entry.symbol,
        label: basis.label
      };
    });

  if (!options.some((option) => option.symbol === fallback.symbol)) {
    options.unshift(fallback);
  }
  return options.length ? options : [fallback];
}

function unitEntryMatchesDimension(entry: UnitCatalogEntry, dimensionId: string): boolean {
  if (entry.dimension_id === dimensionId) return true;
  return equivalentUnitDimensions(dimensionId).includes(entry.dimension_id);
}

function equivalentUnitDimensions(dimensionId: string): string[] {
  switch (dimensionId) {
    case "stress":
      return ["pressure"];
    case "displacement":
      return ["length"];
    case "rotation":
      return ["angle"];
    case "linear_stiffness":
      return ["force_per_length"];
    case "volume_per_length":
      return ["area"];
    default:
      return [];
  }
}

function unitCatalogStatus(route: UnitCatalogRoute | null): string {
  if (!route) return "loading DEC-018 catalog";
  if (route.route === "unavailable_browser_preview") return "browser preview uses model metadata; no fallback catalog";
  return `${route.catalog.decision_basis}; ${route.catalog.entry_count} entries; ${route.catalog.storage_convention}`;
}

function editorFieldOptions(model: PreviewModel, selection: EntityRef): EditableField[] {
  const material = model.materials?.find((item) => item.id === selection.id);
  if (material) {
    return [
      scalarField("Label", "label", material.label, "Material", "dimensionless", "none", "material label only"),
      quantityField(
        "Elastic modulus",
        "elastic_modulus.value",
        material.elastic_modulus.value,
        "Material",
        "stress",
        material.elastic_modulus.unit,
        "set_field",
        true
      ),
      quantityField(
        "Shear modulus",
        "shear_modulus.value",
        material.shear_modulus.value,
        "Material",
        "stress",
        material.shear_modulus.unit,
        "set_field",
        true
      ),
      quantityField(
        "Thermal expansion",
        "thermal_expansion_coefficient.value",
        material.thermal_expansion_coefficient?.value ?? "TBD",
        "Material",
        "thermal_expansion_coefficient",
        material.thermal_expansion_coefficient?.unit ?? "TBD",
        "set_field",
        true
      ),
      scalarField(
        "Provenance",
        "provenance",
        material.provenance,
        "Material",
        "dimensionless",
        "none",
        "public/private source note"
      )
    ];
  }

  const node = model.nodes.find((item) => item.id === selection.id);
  if (node) {
    return [
      scalarField("Label", "label", node.label, "Node", "dimensionless", "none", "node label only"),
      quantityField(
        "X position",
        "position.x",
        node.position.x,
        "Node",
        "length",
        model.project.units.length ?? "m",
        "set_field",
        true
      ),
      quantityField(
        "Y position",
        "position.y",
        node.position.y,
        "Node",
        "length",
        model.project.units.length ?? "m",
        "set_field",
        true
      ),
      quantityField(
        "Z position",
        "position.z",
        node.position.z,
        "Node",
        "length",
        model.project.units.length ?? "m",
        "set_field",
        true
      ),
      scalarField(
        "Provenance",
        "provenance",
        node.provenance,
        "Node",
        "dimensionless",
        "none",
        "public/private source note"
      )
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
        pipe.section.outside_diameter?.unit ?? "TBD",
        "set_field",
        true
      ),
      quantityField(
        "Wall thickness",
        "section.wall_thickness.value",
        pipe.section.wall_thickness?.value ?? "TBD",
        "Element",
        "length",
        pipe.section.wall_thickness?.unit ?? "TBD",
        "set_field",
        true
      ),
      quantityField(
        "Mill tolerance",
        "section.mill_tolerance.value",
        pipe.section.mill_tolerance?.value ?? "TBD",
        "Element",
        "length",
        pipe.section.mill_tolerance?.unit ?? pipe.section.wall_thickness?.unit ?? model.project.units.length ?? "m",
        "set_field",
        true
      ),
      scalarField("Material", "material", pipe.material, "Element", "dimensionless", "none", "material reference"),
      scalarField(
        "Provenance",
        "provenance",
        pipe.provenance,
        "Element",
        "dimensionless",
        "none",
        "public/private source note"
      )
    ];
  }

  const support = model.supports.find((item) => item.id === selection.id);
  if (support) {
    return [
      scalarField(
        "Label",
        "label",
        support.label,
        "Support",
        "dimensionless",
        "none",
        "support label only",
        "update_support"
      ),
      scalarField(
        "Node",
        "node",
        support.node,
        "Support",
        "dimensionless",
        "none",
        "target node reference",
        "update_support"
      ),
      scalarField(
        "Restraints",
        "restraints",
        support.restraints.join(", "),
        "Support",
        "dimensionless",
        "none",
        "restraint direction set",
        "update_support"
      ),
      scalarField(
        "Provenance",
        "provenance",
        support.provenance,
        "Support",
        "dimensionless",
        "none",
        "public/private source note",
        "update_support"
      )
    ];
  }

  const component = model.components.find((item) => item.id === selection.id);
  if (component) {
    const fields: EditableField[] = [
      scalarField("Label", "label", component.label, "Component", "dimensionless", "none", "component label only"),
      scalarField("Node", "node", component.node, "Component", "dimensionless", "none", "target node reference"),
      scalarField(
        "Provenance",
        "provenance",
        component.provenance,
        "Component",
        "dimensionless",
        "none",
        "public/private source note"
      )
    ];
    if (isBendComponent(component)) {
      fields.splice(
        2,
        0,
        quantityField(
          "Bend radius",
          "geometry.bend_radius.value",
          component.geometry?.bend_radius?.value ?? "TBD",
          "Component",
          "length",
          component.geometry?.bend_radius?.unit ?? model.project.units.length ?? "m",
          "set_field",
          true
        ),
        quantityField(
          "Bend angle",
          "geometry.bend_angle.value",
          component.geometry?.bend_angle?.value ?? "TBD",
          "Component",
          "angle",
          component.geometry?.bend_angle?.unit ?? model.project.units.angle ?? "rad",
          "set_field",
          true
        ),
        scalarField(
          "Bend plane",
          "geometry.bend_plane_orientation",
          component.geometry?.bend_plane_orientation ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "user-entered bend plane orientation"
        ),
        scalarField(
          "Bend pipe",
          "geometry.bend_pipe_ref",
          component.geometry?.bend_pipe_ref ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "user-entered curved-bend span mapping (DEC-070); no inferred span"
        ),
        scalarField(
          "SIF user value",
          "modifiers.sif_user_value.value",
          String(component.modifiers?.sif_user_value?.value ?? "TBD"),
          "Component",
          "dimensionless",
          "none",
          "user-entered modifier value; no code table default"
        ),
        scalarField(
          "Flexibility user value",
          "modifiers.flexibility_factor_user_value.value",
          String(component.modifiers?.flexibility_factor_user_value?.value ?? "TBD"),
          "Component",
          "dimensionless",
          "none",
          "user-entered modifier value; no code table default"
        )
      );
    }
    if (isBranchComponent(component)) {
      fields.splice(
        2,
        0,
        scalarField(
          "Header pipe",
          "geometry.branch_header_pipe_ref",
          component.geometry?.branch_header_pipe_ref ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "user-entered header pipe reference"
        ),
        scalarField(
          "Branch pipe",
          "geometry.branch_branch_pipe_ref",
          component.geometry?.branch_branch_pipe_ref ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "user-entered branch pipe reference"
        ),
        quantityField(
          "Branch run size",
          "geometry.branch_run_size.value",
          component.geometry?.branch_run_size?.value ?? "TBD",
          "Component",
          "length",
          component.geometry?.branch_run_size?.unit ?? model.project.units.length ?? "m",
          "set_field",
          true
        ),
        quantityField(
          "Branch header size",
          "geometry.branch_header_size.value",
          component.geometry?.branch_header_size?.value ?? "TBD",
          "Component",
          "length",
          component.geometry?.branch_header_size?.unit ?? model.project.units.length ?? "m",
          "set_field",
          true
        ),
        quantityField(
          "Branch angle",
          "geometry.branch_connection_angle.value",
          component.geometry?.branch_connection_angle?.value ?? "TBD",
          "Component",
          "angle",
          component.geometry?.branch_connection_angle?.unit ?? model.project.units.angle ?? "rad",
          "set_field",
          true
        ),
        scalarField(
          "Connection type",
          "geometry.branch_connection_type",
          component.geometry?.branch_connection_type ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "user-entered branch connection type"
        ),
        scalarField(
          "Header SIF user value",
          "modifiers.branch_header_sif_user_value.value",
          String(component.modifiers?.branch_header_sif_user_value?.value ?? "TBD"),
          "Component",
          "dimensionless",
          "none",
          "user-entered modifier value; no code table default"
        ),
        scalarField(
          "Branch SIF user value",
          "modifiers.branch_branch_sif_user_value.value",
          String(component.modifiers?.branch_branch_sif_user_value?.value ?? "TBD"),
          "Component",
          "dimensionless",
          "none",
          "user-entered modifier value; no code table default"
        ),
        scalarField(
          "Flexibility user value",
          "modifiers.flexibility_factor_user_value.value",
          String(component.modifiers?.flexibility_factor_user_value?.value ?? "TBD"),
          "Component",
          "dimensionless",
          "none",
          "user-entered modifier value; no code table default"
        )
      );
    }
    if (isRigidComponent(component)) {
      fields.splice(
        2,
        0,
        scalarField(
          "Mapped pipe",
          "geometry.rigid_pipe_ref",
          component.geometry?.rigid_pipe_ref ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "generic frame member mapping"
        ),
        quantityField(
          "Rigid body length",
          "geometry.rigid_body_length.value",
          component.geometry?.rigid_body_length?.value ?? "TBD",
          "Component",
          "length",
          component.geometry?.rigid_body_length?.unit ?? model.project.units.length ?? "m",
          "set_field",
          true
        ),
        quantityField(
          "End A size",
          "geometry.end_a_size.value",
          component.geometry?.end_a_size?.value ?? "TBD",
          "Component",
          "length",
          component.geometry?.end_a_size?.unit ?? model.project.units.length ?? "m",
          "set_field",
          true
        ),
        quantityField(
          "End B size",
          "geometry.end_b_size.value",
          component.geometry?.end_b_size?.value ?? "TBD",
          "Component",
          "length",
          component.geometry?.end_b_size?.unit ?? model.project.units.length ?? "m",
          "set_field",
          true
        ),
        quantityField(
          "Weight",
          "geometry.weight.value",
          component.geometry?.weight?.value ?? "TBD",
          "Component",
          "force",
          component.geometry?.weight?.unit ?? model.project.units.force ?? "N",
          "set_field",
          true
        ),
        scalarField(
          "Center of gravity",
          "geometry.center_of_gravity",
          component.geometry?.center_of_gravity
            ? `x=${component.geometry.center_of_gravity.x}, y=${component.geometry.center_of_gravity.y}, z=${component.geometry.center_of_gravity.z} ${component.geometry.center_of_gravity.unit}`
            : "TBD",
          "Component",
          "length",
          component.geometry?.center_of_gravity?.unit ?? model.project.units.length ?? "m",
          "user-entered COG vector in the component reference convention"
        ),
        scalarField(
          "Stiffness behavior",
          "geometry.stiffness_behavior_reference",
          component.geometry?.stiffness_behavior_reference ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "user-entered semi-rigid behavior reference"
        ),
        scalarField(
          "Stiffness scale",
          "modifiers.stiffness_scaling_user_value.value",
          String(component.modifiers?.stiffness_scaling_user_value?.value ?? "TBD"),
          "Component",
          "dimensionless",
          component.modifiers?.stiffness_scaling_user_value?.unit ?? "none",
          "user-entered rigid stiffness scaling; no catalog default"
        ),
        quantityField(
          "Linear stiffness",
          "modifiers.linear_stiffness_user_value.value",
          component.modifiers?.linear_stiffness_user_value?.value ?? "TBD",
          "Component",
          "linear_stiffness",
          component.modifiers?.linear_stiffness_user_value?.unit ?? "N/m",
          "set_field",
          true
        ),
        quantityField(
          "Rotational stiffness",
          "modifiers.rotational_stiffness_user_value.value",
          component.modifiers?.rotational_stiffness_user_value?.value ?? "TBD",
          "Component",
          "rotational_stiffness",
          component.modifiers?.rotational_stiffness_user_value?.unit ?? "N*m/rad",
          "set_field",
          true
        )
      );
    }
    if (isExpansionJointComponent(component)) {
      fields.splice(
        2,
        0,
        scalarField(
          "Expansion joint pipe",
          "geometry.expansion_joint_pipe_ref",
          component.geometry?.expansion_joint_pipe_ref ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "expansion-joint frame member mapping"
        ),
        quantityField(
          "Effective pressure area",
          "geometry.effective_area.value",
          component.geometry?.effective_area?.value ?? "TBD",
          "Component",
          "area",
          component.geometry?.effective_area?.unit ?? "m^2",
          "set_field",
          true
        ),
        quantityField(
          "Movement limit",
          "geometry.movement_limit.value",
          component.geometry?.movement_limit?.value ?? "TBD",
          "Component",
          "length",
          component.geometry?.movement_limit?.unit ?? model.project.units.length ?? "m",
          "set_field",
          true
        ),
        scalarField(
          "Hardware reference",
          "geometry.hardware_reference",
          component.geometry?.hardware_reference ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "user-entered expansion-joint hardware reference"
        ),
        scalarField(
          "Manufacturer reference",
          "geometry.manufacturer_reference",
          component.geometry?.manufacturer_reference ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "user-entered manufacturer provenance reference"
        ),
        scalarField(
          "Pressure thrust reference",
          "geometry.pressure_thrust_reference",
          component.geometry?.pressure_thrust_reference ?? "TBD",
          "Component",
          "dimensionless",
          "none",
          "load-side pressure thrust handling reference"
        ),
        quantityField(
          "Axial stiffness",
          "modifiers.axial_stiffness_user_value.value",
          component.modifiers?.axial_stiffness_user_value?.value ?? "TBD",
          "Component",
          "linear_stiffness",
          component.modifiers?.axial_stiffness_user_value?.unit ?? "N/m",
          "set_field",
          true
        ),
        quantityField(
          "Lateral stiffness",
          "modifiers.lateral_stiffness_user_value.value",
          component.modifiers?.lateral_stiffness_user_value?.value ?? "TBD",
          "Component",
          "linear_stiffness",
          component.modifiers?.lateral_stiffness_user_value?.unit ?? "N/m",
          "set_field",
          true
        ),
        quantityField(
          "Angular stiffness",
          "modifiers.angular_stiffness_user_value.value",
          component.modifiers?.angular_stiffness_user_value?.value ?? "TBD",
          "Component",
          "rotational_stiffness",
          component.modifiers?.angular_stiffness_user_value?.unit ?? "N*m/rad",
          "set_field",
          true
        ),
        quantityField(
          "Torsional stiffness",
          "modifiers.torsional_stiffness_user_value.value",
          component.modifiers?.torsional_stiffness_user_value?.value ?? "TBD",
          "Component",
          "rotational_stiffness",
          component.modifiers?.torsional_stiffness_user_value?.unit ?? "N*m/rad",
          "set_field",
          true
        )
      );
    }
    return fields;
  }

  const loadCase = model.load_cases.find((item) => item.id === selection.id);
  if (loadCase) {
    const firstLoad = loadCase.primitive_loads?.[0] ?? {};
    const firstMagnitude = firstLoad.magnitude as { value?: unknown; unit?: unknown } | undefined;
    return [
      scalarField(
        "Label",
        "label",
        loadCase.label,
        "Load",
        "dimensionless",
        "none",
        "load case label only",
        "update_load"
      ),
      scalarField(
        "Status",
        "status",
        loadCase.status,
        "Load",
        "dimensionless",
        "none",
        "load case status",
        "update_load"
      ),
      scalarField("Kind", "kind", loadCase.kind, "Load", "dimensionless", "none", "load case kind", "update_load"),
      scalarField(
        "Modulus basis",
        "modulus_basis_ref",
        loadCase.modulus_basis_ref ?? "TBD",
        "Load",
        "dimensionless",
        "none",
        "user-assigned temperature-point basis id; exact selection; mutually exclusive with solve temperature",
        "update_load"
      ),
      quantityField(
        "Modulus basis solve temperature",
        "modulus_basis_temperature.value",
        loadCase.modulus_basis_temperature?.value ?? "TBD",
        "Load",
        "temperature",
        loadCase.modulus_basis_temperature?.unit ?? "K",
        "update_load",
        true
      ),
      quantityField(
        "Seismic gravity acceleration",
        "equivalent_static.seismic.gravity_acceleration.value",
        loadCase.equivalent_static?.seismic?.gravity_acceleration?.value ?? "TBD",
        "Load",
        "acceleration",
        loadCase.equivalent_static?.seismic?.gravity_acceleration?.unit ?? "m/s^2",
        "update_load",
        true
      ),
      scalarField(
        "Seismic g-factor X",
        "equivalent_static.seismic.g_factor_x.value",
        String(loadCase.equivalent_static?.seismic?.g_factor_x?.value ?? "TBD"),
        "Load",
        "dimensionless",
        "none",
        "user-entered per-axis g-factor; no code coefficient or default",
        "update_load"
      ),
      scalarField(
        "Seismic g-factor Y",
        "equivalent_static.seismic.g_factor_y.value",
        String(loadCase.equivalent_static?.seismic?.g_factor_y?.value ?? "TBD"),
        "Load",
        "dimensionless",
        "none",
        "user-entered per-axis g-factor; no code coefficient or default",
        "update_load"
      ),
      scalarField(
        "Seismic g-factor Z",
        "equivalent_static.seismic.g_factor_z.value",
        String(loadCase.equivalent_static?.seismic?.g_factor_z?.value ?? "TBD"),
        "Load",
        "dimensionless",
        "none",
        "user-entered per-axis g-factor; no code coefficient or default",
        "update_load"
      ),
      quantityField(
        "Wind pressure",
        "equivalent_static.wind.pressure.value",
        loadCase.equivalent_static?.wind?.pressure?.value ?? "TBD",
        "Load",
        "pressure",
        loadCase.equivalent_static?.wind?.pressure?.unit ?? model.project.units.pressure ?? "Pa",
        "update_load",
        true
      ),
      scalarField(
        "Wind shape factor",
        "equivalent_static.wind.shape_factor.value",
        String(loadCase.equivalent_static?.wind?.shape_factor?.value ?? "TBD"),
        "Load",
        "dimensionless",
        "none",
        "user-entered wind shape parameter; no code coefficient or default",
        "update_load"
      ),
      scalarField(
        "Wind direction",
        "equivalent_static.wind.direction",
        loadCase.equivalent_static?.wind?.direction ?? "TBD",
        "Load",
        "dimensionless",
        "none",
        "user-entered global axis token (global_x | global_y | global_z)",
        "update_load"
      ),
      scalarField(
        "Wind exposed spans",
        "equivalent_static.wind.exposed_pipe_refs",
        (loadCase.equivalent_static?.wind?.exposed_pipe_refs ?? []).join(", ") || "TBD",
        "Load",
        "dimensionless",
        "none",
        "user-marked exposed spans by pipe id (comma-separated); no span is marked by default",
        "update_load"
      ),
      quantityField(
        "First primitive magnitude",
        "primitive_loads.0.magnitude.value",
        typeof firstMagnitude?.value === "number" || typeof firstMagnitude?.value === "string"
          ? firstMagnitude.value
          : "TBD",
        "Load",
        typeof firstLoad.dimension === "string" ? firstLoad.dimension : "TBD",
        typeof firstMagnitude?.unit === "string" ? firstMagnitude.unit : "TBD",
        "update_load",
        true
      ),
      scalarField(
        "Provenance",
        "provenance",
        loadCase.provenance,
        "Load",
        "dimensionless",
        "none",
        "public/private source note",
        "update_load"
      )
    ];
  }

  const combination = model.combinations?.find((item) => item.id === selection.id);
  if (combination) {
    return [
      scalarField(
        "Label",
        "label",
        combination.label,
        "Combination",
        "dimensionless",
        "none",
        "combination label only"
      ),
      scalarField(
        "Basis",
        "basis",
        combination.basis,
        "Combination",
        "dimensionless",
        "none",
        "mechanics or user rule basis"
      ),
      scalarField(
        "Terms",
        "terms",
        combination.terms.map((term) => `${term.load_case} x ${term.factor}`).join("; "),
        "Combination",
        "dimensionless",
        "none",
        "explicit user-defined terms"
      ),
      scalarField(
        "Provenance",
        "provenance",
        combination.provenance,
        "Combination",
        "dimensionless",
        "none",
        "public/private source note"
      )
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
  return {
    label,
    fieldPath,
    before,
    objectType,
    dimension,
    unit,
    sourceNote,
    changeKind,
    unitEditable: false
  };
}

function quantityField(
  label: string,
  fieldPath: string,
  before: number | string,
  objectType: EditorOperationObjectType,
  dimension: string,
  unit: string,
  changeKind: EditableField["changeKind"] = "set_field",
  unitEditable = false
): EditableField {
  return {
    label,
    fieldPath,
    before: String(before),
    objectType,
    dimension,
    unit,
    sourceNote: "unit metadata required",
    changeKind,
    unitEditable
  };
}

function isBendComponent(component: PreviewComponent): boolean {
  return component.kind === "bend" || component.kind === "elbow";
}

function isBranchComponent(component: PreviewComponent): boolean {
  return component.kind === "branch" || component.kind === "tee" || component.kind === "branch_connection";
}

function isRigidComponent(component: PreviewComponent): boolean {
  return ["valve", "flange", "reducer", "rigid", "specialty"].includes(component.kind);
}

function isExpansionJointComponent(component: PreviewComponent): boolean {
  return component.kind === "expansion_joint";
}

function buildOperationIntent({
  field,
  model,
  proposedValue,
  proposedUnit,
  rationale,
  selection,
  unitCatalogRoute
}: {
  field: EditableField;
  model: PreviewModel;
  proposedValue: string;
  proposedUnit: string;
  rationale: string;
  selection: EntityRef;
  unitCatalogRoute: UnitCatalogRoute | null;
}): EditorOperationIntent {
  const operationToken = `${safeToken(selection.id)}-${safeToken(field.fieldPath)}`;
  const intentUnit = field.unitEditable ? proposedUnit.trim() || field.unit : field.unit;
  const changeAfter = field.unitEditable
    ? JSON.stringify({
        value: parseQuantityPayloadValue(proposedValue),
        unit: intentUnit
      })
    : proposedValue.trim() || "TBD";
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
      after: changeAfter,
      unit: intentUnit,
      dimension: field.dimension,
      source_note: field.unitEditable ? `${field.sourceNote}; entered unit captured explicitly` : field.sourceNote
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: field.unitEditable
        ? propertyUnitValidationStatus(unitCatalogRoute, intentUnit, field.dimension)
        : "not_required_dimensionless",
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

function parseQuantityPayloadValue(raw: string): number | string {
  const trimmed = raw.trim();
  if (!trimmed) return "TBD";
  const parsed = Number(trimmed);
  return Number.isFinite(parsed) ? parsed : trimmed;
}

function materialDraftUnitValidationStatus(draft: MaterialDraft, route: UnitCatalogRoute | null): string {
  const stressStatus = propertyUnitValidationStatus(route, draft.stressUnit, "stress");
  const thermalStatus = draft.thermalExpansion.trim()
    ? propertyUnitValidationStatus(route, draft.thermalExpansionUnit, "thermal_expansion_coefficient")
    : "not_provided";
  return `stress=${stressStatus}; thermal_expansion_coefficient=${thermalStatus}`;
}

function propertyUnitValidationStatus(route: UnitCatalogRoute | null, unit: string, dimension: string): string {
  const normalizedUnit = unit.trim();
  const normalizedDimension = dimension.trim();
  if (!normalizedUnit || normalizedUnit === "TBD" || !normalizedDimension || normalizedDimension === "TBD") {
    return "missing_unit_or_dimension";
  }
  if (normalizedUnit === "none" || normalizedDimension === "dimensionless") {
    return "not_required_dimensionless";
  }

  const basis = describeUnitBasis(route, normalizedUnit, normalizedDimension);
  switch (basis.source) {
    case "dec018_catalog_accepted":
      return "dec018_catalog_dimension_match";
    case "dec018_catalog_unreviewed":
      return `dec018_catalog_${basis.review_status ?? "unreviewed"}_dimension_match`;
    case "dec018_catalog_miss":
      return "dec018_catalog_dimension_mismatch";
    case "browser_preview_model_metadata":
      return "model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview";
    case "catalog_loading":
      return "catalog_loading_unit_dimension_declared";
    default:
      return "unit_dimension_status_unknown";
  }
}

function buildCreateSectionIntent(
  draft: SectionDraft,
  model: PreviewModel,
  unitCatalogRoute: UnitCatalogRoute | null
): EditorOperationIntent {
  const sectionId = draft.id.trim();
  const payload = {
    id: sectionId,
    name: draft.name.trim(),
    section_type: draft.sectionType.trim(),
    properties: {
      outside_diameter: {
        value: Number(draft.outsideDiameter),
        unit: draft.lengthUnit
      },
      wall_thickness: {
        value: Number(draft.wallThickness),
        unit: draft.lengthUnit
      }
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
      unit: draft.lengthUnit,
      dimension: "length",
      source_note: "explicit user-entered pipe section geometry; entered unit preserved"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: `length=${propertyUnitValidationStatus(unitCatalogRoute, draft.lengthUnit, "length")}`,
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

function buildCreateMaterialIntent(
  draft: MaterialDraft,
  model: PreviewModel,
  unitCatalogRoute: UnitCatalogRoute | null
): EditorOperationIntent {
  const materialId = draft.id.trim();
  const payload: Record<string, unknown> = {
    id: materialId,
    label: draft.label.trim(),
    elastic_modulus: {
      value: Number(draft.elasticModulus),
      unit: draft.stressUnit
    },
    shear_modulus: {
      value: Number(draft.shearModulus),
      unit: draft.stressUnit
    },
    provenance: draft.provenance.trim()
  };
  if (draft.thermalExpansion.trim()) {
    payload.thermal_expansion_coefficient = {
      value: Number(draft.thermalExpansion),
      unit: draft.thermalExpansionUnit
    };
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
      unit: draft.stressUnit,
      dimension: "stress",
      source_note: "explicit user-entered material quantities; entered units preserved"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: materialDraftUnitValidationStatus(draft, unitCatalogRoute),
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

function buildCreateSupportIntent(
  draft: SupportDraft,
  model: PreviewModel,
  unitCatalogRoute: UnitCatalogRoute | null
): EditorOperationIntent {
  const supportId = draft.id.trim();
  const stiffnessProvided = draft.linearStiffness.trim() !== "";
  const payload: Record<string, unknown> = {
    id: supportId,
    label: draft.label.trim(),
    node: draft.node.trim(),
    restraints: draft.restraints,
    provenance: draft.provenance.trim()
  };
  if (stiffnessProvided) {
    payload.properties = {
      linear_stiffness: {
        value: Number(draft.linearStiffness),
        unit: draft.linearStiffnessUnit
      }
    };
  }
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
      unit: stiffnessProvided ? draft.linearStiffnessUnit : "none",
      dimension: stiffnessProvided ? "linear_stiffness" : "dimensionless",
      source_note: stiffnessProvided
        ? "explicit user-entered support node, restraint tokens, and linear stiffness; entered unit preserved"
        : "explicit user-entered support node and restraint tokens"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: stiffnessProvided
        ? `linear_stiffness=${propertyUnitValidationStatus(unitCatalogRoute, draft.linearStiffnessUnit, "linear_stiffness")}`
        : "not_required_dimensionless",
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

function buildDeleteSupportIntent(
  support: PreviewModel["supports"][number],
  model: PreviewModel
): EditorOperationIntent {
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
      unit_validation: "not_required_dimensionless",
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

function buildDeleteNodeIntent(node: PreviewModel["nodes"][number], model: PreviewModel): EditorOperationIntent {
  const nodeId = node.id.trim();
  const before = `${node.label}; x=${node.position.x}; y=${node.position.y}; z=${node.position.z}`;
  return {
    operation_id: `op:delete-node-${safeToken(nodeId)}`,
    operation_kind: "delete",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Node",
      ref: nodeId
    },
    change: {
      change_id: `change:delete-node:${safeToken(nodeId)}`,
      change_kind: "delete_node",
      field_label: "Explicit node deletion",
      field_path: "nodes",
      before,
      after: "not_present",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered node deletion; endpoint and load reference integrity required"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_required_dimensionless",
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
    rationale: `explicit user-entered node deletion for ${model.project.id}; requires reference validation before durable model change.`
  };
}

function buildDeletePipeIntent(
  pipe: PreviewModel["pipe_segments"][number],
  model: PreviewModel
): EditorOperationIntent {
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
      unit_validation: "not_required_dimensionless",
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
    lengthUnit: lengthUnit(model),
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
    stressUnit: stressUnit(model),
    elasticModulus: "",
    shearModulus: "",
    thermalExpansionUnit: thermalExpansionUnit(model),
    thermalExpansion: "",
    provenance: "user_entered_local_preview"
  };
}

function defaultSupportDraft(
  model: PreviewModel,
  selection: EntityRef,
  queuedIntents: EditorOperationIntent[]
): SupportDraft {
  return defaultSupportDraftWithReserved(model, selection, queuedIntents);
}

function defaultSupportDraftWithReserved(
  model: PreviewModel,
  selection: EntityRef,
  queuedIntents: EditorOperationIntent[]
): SupportDraft {
  const id = nextSupportId(model, queuedIntents);
  const selectedNode =
    selection.type === "node" && model.nodes.some((node) => node.id === selection.id) ? selection.id : null;
  const node = selectedNode ?? model.nodes[0]?.id ?? "";
  return {
    id,
    label: `Support ${shortEntityToken(id)}`,
    node,
    restraints: ["UX", "UY", "UZ"],
    linearStiffnessUnit: linearStiffnessUnit(model),
    linearStiffness: "",
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
    draft.lengthUnit !== "TBD" &&
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
    draft.stressUnit !== "TBD" &&
    isPositiveInput(draft.elasticModulus) &&
    isPositiveInput(draft.shearModulus) &&
    (!thermalProvided || (draft.thermalExpansionUnit !== "TBD" && isFiniteInput(draft.thermalExpansion))) &&
    !(model.materials ?? []).some((material) => material.id === draft.id.trim())
  );
}

function isSupportDraftValid(model: PreviewModel, draft: SupportDraft): boolean {
  const stiffnessProvided = draft.linearStiffness.trim() !== "";
  return (
    Boolean(draft.id.trim() && draft.label.trim() && draft.node.trim() && draft.provenance.trim()) &&
    model.nodes.some((node) => node.id === draft.node.trim()) &&
    draft.restraints.length > 0 &&
    draft.restraints.every((restraint) => RESTRAINT_OPTIONS.includes(restraint)) &&
    (!stiffnessProvided || (draft.linearStiffnessUnit !== "TBD" && isPositiveInput(draft.linearStiffness))) &&
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

function linearStiffnessUnit(model: PreviewModel): string {
  const direct = model.project.units.linear_stiffness;
  if (direct) return direct;
  const force = model.project.units.force;
  const length = model.project.units.length;
  return force && length ? `${force}/${length}` : "TBD";
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
