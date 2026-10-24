import type { ReactElement } from "react";
import {
  CONNECTOR_CONTRACT_NOTICE, connectorHardwareKinds, connectorPressureModels, CONNECTOR_REFERENCE_STATES,
  connectorCreationAvailable, emptyTieRodDraft, TIE_ROD_LAW, tieRodDrafts, type ConnectorDraft, type TieRodDraft
} from "./componentIntent";
import type { PreviewModel } from "../../types";
import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";

type TextInput = (props: { label: string; testId: string; value: string; onChange: (value: string) => void }) => ReactElement;

/** The objective connector's inputs (objective_connector 1.0.0), shared by the
 * inspector and viewport creation forms. The replaced span is the form's pipe
 * picker; its end nodes are the connector's ends. Closed vocabularies are
 * offered as choices the user selects; nothing is preselected. */
export function ConnectorDraftFields({ model, draft, prefix, TextInput, update }: {
  model: PreviewModel;
  draft: ConnectorDraft;
  prefix: string;
  TextInput: TextInput;
  update: (key: keyof ConnectorDraft, value: string) => void;
}) {
  if (!connectorCreationAvailable(model)) {
    return <p role="note" data-testid={`${prefix}-component-connector-unavailable`}>{CONNECTOR_CONTRACT_NOTICE}</p>;
  }
  const text = (label: string, key: keyof ConnectorDraft, id: string) =>
    <TextInput label={label} testId={`${prefix}-component-connector-${id}`} value={draft[key]} onChange={(value) => update(key, value)} />;
  const choice = (label: string, key: keyof ConnectorDraft, id: string, options: readonly string[]) => (
    <label>
      <span>{label}</span>
      <select aria-label={`Connector ${label.toLowerCase()}`} data-testid={`${prefix}-component-connector-${id}`} value={draft[key]} onChange={(event) => update(key, event.target.value)}>
        <option value="">Select</option>
        {options.map((option) => <option key={option} value={option}>{option.replaceAll("_", " ")}</option>)}
      </select>
    </label>
  );
  return (
    <fieldset data-testid={`${prefix}-component-connector-fields`}>
      <legend>Objective connector (replaces the selected span)</legend>
      <p>Matrices are three rows of three numbers separated by semicolons; vectors are three numbers. The work matrix is the 21 upper-triangle coefficients in N*m, ordered tx, ty, tz, rx, ry, rz. No value is defaulted.</p>
      {text("Connector axes (global)", "connectorAxes", "axes")}
      {text("End i node axes (global)", "endINodeAxes", "end-i-axes")}
      {text("End j node axes (global)", "endJNodeAxes", "end-j-axes")}
      {text("End i offset (local)", "endIOffset", "end-i-offset")}
      {text("End j offset (local)", "endJOffset", "end-j-offset")}
      {text("Offset unit", "offsetUnit", "offset-unit")}
      {choice("Reference state", "referenceState", "reference-state", CONNECTOR_REFERENCE_STATES)}
      {text("Reference translation q_ref", "qRefTranslation", "qref-translation")}
      {text("Reference translation unit", "qRefTranslationUnit", "qref-translation-unit")}
      {text("Reference rotation q_ref", "qRefRotation", "qref-rotation")}
      {text("Reference rotation unit", "qRefRotationUnit", "qref-rotation-unit")}
      {text("Installed reference temperature", "installedTemperature", "temperature")}
      {text("Installed reference temperature unit", "installedTemperatureUnit", "temperature-unit")}
      {text("Translation scale", "translationScale", "translation-scale")}
      {text("Translation scale unit", "translationScaleUnit", "translation-scale-unit")}
      {text("Work matrix upper triangle (N*m)", "workMatrixUpperTriangle", "work-matrix")}
      {text("Installed geometry (calibration)", "installedGeometry", "installed-geometry")}
      {choice("Hardware", "hardwareKind", "hardware", connectorHardwareKinds(model))}
      {draft.hardwareKind === "explicit_axial_rods_v1" &&
        <TieRodTable draft={draft} prefix={prefix} TextInput={TextInput} update={update} />}
      {choice("Pressure model", "pressureModel", "pressure-model", connectorPressureModels(model))}
      {model.schema_version === MODEL_SCHEMA_VERSION && draft.pressureModel === "constant_effective_area_reference_v1" && <>
        {text("Joint effective area", "effectiveArea", "effective-area")}
        {text("Joint effective area unit", "effectiveAreaUnit", "effective-area-unit")}
        {choice("Joint pressure basis", "pressureBasis", "pressure-basis", ["internal_differential_zero_external_v1"])}
        {text("Joint pressure area source reference", "pressureSourceReference", "pressure-source")}
      </>}
      {text("Stiffness source reference", "stiffnessSourceReference", "source")}
      {text("Measurement restraints", "measurementRestraints", "measurement")}
      {text("Basis transform reference", "basisTransformReference", "basis-transform")}
      {text("Validity statement", "validityStatement", "validity")}
    </fieldset>
  );
}

const ROD_COLUMNS: [keyof TieRodDraft, string, string][] = [
  ["id", "Rod id", "id"],
  ["offsetI", "Offset at end i (connector axes)", "offset-i"],
  ["offsetJ", "Offset at end j (connector axes)", "offset-j"],
  ["offsetUnit", "Offset unit", "offset-unit"],
  ["stiffness", "Rod stiffness", "stiffness"],
  ["stiffnessUnit", "Rod stiffness unit", "stiffness-unit"],
  ["referenceExtension", "Reference extension (negative is pretension)", "reference-extension"],
  ["referenceExtensionUnit", "Reference extension unit", "reference-extension-unit"],
  ["sourceReference", "Rod source reference", "source"]
];

/** T4-U5b (DEL-04-07 Design u4b-u5b-implementation.md §3.1, §5.2): the
 * explicit tie rods of `explicit_axial_rods_v1`, one row per rod between the
 * connector's end nodes, law bilateral linear. Offsets are three numbers in
 * the connector axes; nothing is defaulted. */
function TieRodTable({ draft, prefix, TextInput, update }: {
  draft: ConnectorDraft;
  prefix: string;
  TextInput: TextInput;
  update: (key: keyof ConnectorDraft, value: string) => void;
}) {
  const rows = tieRodDrafts(draft.tieRods);
  const write = (next: TieRodDraft[]) => update("tieRods", next.length ? JSON.stringify(next) : "");
  return (
    <fieldset data-testid={`${prefix}-component-connector-tie-rods`}>
      <legend>Tie rods (law {TIE_ROD_LAW.replaceAll("_", " ")}, between the connector's end nodes)</legend>
      {rows.map((row, index) => (
        <div key={index} data-testid={`${prefix}-component-connector-tie-rod-${index}`}>
          {ROD_COLUMNS.map(([key, label, id]) => (
            <TextInput key={key} label={`${label} (rod ${index + 1})`} testId={`${prefix}-component-connector-tie-rod-${index}-${id}`} value={row[key]}
              onChange={(value) => write(rows.map((other, at) => at === index ? { ...other, [key]: value } : other))} />
          ))}
          <button type="button" data-testid={`${prefix}-component-connector-tie-rod-${index}-remove`} onClick={() => write(rows.filter((_, at) => at !== index))}>
            Remove tie rod {index + 1}
          </button>
        </div>
      ))}
      <button type="button" data-testid={`${prefix}-component-connector-tie-rod-add`} onClick={() => write([...rows, emptyTieRodDraft()])}>
        Add tie rod
      </button>
    </fieldset>
  );
}
