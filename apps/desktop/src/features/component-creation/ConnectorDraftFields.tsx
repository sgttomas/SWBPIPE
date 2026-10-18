import type { ReactElement } from "react";
import {
  CONNECTOR_CONTRACT_NOTICE, CONNECTOR_HARDWARE_KINDS, connectorPressureModels, CONNECTOR_REFERENCE_STATES,
  connectorCreationAvailable, type ConnectorDraft
} from "./componentIntent";
import type { PreviewModel } from "../../types";

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
      {choice("Hardware", "hardwareKind", "hardware", CONNECTOR_HARDWARE_KINDS)}
      {choice("Pressure model", "pressureModel", "pressure-model", connectorPressureModels(model))}
      {model.schema_version === "0.6.0" && draft.pressureModel === "constant_effective_area_reference_v1" && <>
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
