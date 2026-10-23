import { useEffect, useState } from "react";
import { clone, parseQuantities, QueueFeedback, QuantityField, record, requireText, setMember, TextField, useRichQueue, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
import { isPressureV3 } from "./pressureContract";
import { MODEL_SCHEMA_VERSION } from "../../services/modelSchemaVersion";

export function JointPressureModelEditor(props: RichFormProps) {
  const component = props.model.components.find(component => props.selection.type === "component" && component.id === props.selection.id);
  const connector = record(record(component).objective_connector);
  if (props.model.schema_version !== MODEL_SCHEMA_VERSION || !isPressureV3(props.model) || component?.kind !== "expansion_joint" || connector.version !== "1.0.0") return null;
  return <Editor key={component.id} {...props} componentId={component.id} pressureModel={connector.pressure_model} />;
}
function Editor(props: RichFormProps & { componentId: string; pressureModel: unknown }) {
  const [draft, setDraft] = useState<DraftRecord>(() => clone(record(props.pressureModel)));
  useEffect(() => setDraft(clone(record(props.pressureModel))), [props.pressureModel]);
  const state = useRichQueue(props);
  const update = (key: string, value: unknown) => setDraft(current => setMember(current, key, value));
  async function submit() { try {
    const kind = requireText(draft.kind, "Joint pressure model");
    let after: DraftRecord = { kind };
    if (kind === "constant_effective_area_reference_v1") {
      const effectiveArea = record(parseQuantities(draft.effective_area));
      if (typeof effectiveArea.value !== "number" || effectiveArea.value <= 0) throw new Error("Enter a positive joint effective area.");
      const pressureBasis = requireText(draft.pressure_basis, "Joint pressure basis");
      if (pressureBasis !== "internal_differential_zero_external_v1") throw new Error("Select the internal differential pressure basis.");
      after = { kind, effective_area: effectiveArea, pressure_basis: pressureBasis, source_reference: requireText(draft.source_reference, "Joint area source reference") };
    } else if (kind !== "unpressurized") throw new Error("Select a supported joint pressure model.");
    await state.queue({ object_type: "Component", ref: props.componentId }, "set_field", "objective_connector.pressure_model", props.pressureModel, after, "Joint pressure model");
  } catch (error) { state.setError(String(error)); } }
  return <section aria-label="Joint pressure model"><h3>Joint pressure model</h3>
    <p>The effective area is explicit user data. Pressure magnitude comes from the case region that lists this connector.</p>
    <fieldset disabled={state.busy}>
      <TextField label="Joint pressure model kind" value={draft.kind} choices={["unpressurized", "constant_effective_area_reference_v1"]} onChange={value => update("kind", value)} />
      {draft.kind === "constant_effective_area_reference_v1" && <>
        <QuantityField label="Joint effective area" dimension="area" value={draft.effective_area} onChange={value => update("effective_area", value)} />
        <TextField label="Joint pressure basis" value={draft.pressure_basis} choices={["internal_differential_zero_external_v1"]} onChange={value => update("pressure_basis", value)} />
        <TextField label="Joint area source reference" value={draft.source_reference} onChange={value => update("source_reference", value)} />
      </>}
      <button type="button" onClick={() => void submit()}>Queue joint pressure model</button>
    </fieldset><QueueFeedback {...state} />
  </section>;
}
