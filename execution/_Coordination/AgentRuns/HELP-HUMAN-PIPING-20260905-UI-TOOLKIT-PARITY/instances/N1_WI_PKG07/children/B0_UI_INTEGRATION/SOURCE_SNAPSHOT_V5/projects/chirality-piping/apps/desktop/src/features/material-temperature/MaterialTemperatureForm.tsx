import { useEffect, useState } from "react";
import { clone, parseQuantities, QueueFeedback, QuantityField, requireText, setMember, TextField, useRichQueue, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
type Material = NonNullable<RichFormProps["model"]["materials"]>[number] & {
  temperature_points?: DraftRecord[];
};
export function MaterialTemperatureForm(props: RichFormProps) {
  const material = props.selection.type === "material" ? props.model.materials?.find(m => m.id === props.selection.id) as Material | undefined : undefined;
  return material ? <TemperatureEditor
    key={material.id}
    {...props}
    material={material}
  /> : null;
}
function TemperatureEditor(props: RichFormProps & {
  material: Material;
}) {
  const { material } = props;
  const [rows, setRows] = useState<DraftRecord[]>(() => clone(material.temperature_points ?? []));
  const [keys, setKeys] = useState(() => rows.map((_, i) => i));
  const [nextKey, setNextKey] = useState(rows.length);
  useEffect(() => {
    setRows(clone(material.temperature_points ?? []));
    setKeys((material.temperature_points ?? []).map((_, i) => i));
    setNextKey((material.temperature_points ?? []).length);
  }, [material]);
  const state = useRichQueue(props);
  function update(index: number, key: string, value: unknown) {
    setRows(current => current.map((row, i) => i === index ? setMember(row, key, value) : row));
  }
  async function submit() {
    try {
      const ids = rows.map(row => requireText(row.id, "Temperature point ID"));
      if (new Set(ids).size !== ids.length)
        throw new Error("Temperature point IDs must be unique.");
      const after = parseQuantities(rows);
      await state.queue(
        { object_type: "Material", ref: material.id },
        "set_field",
        "temperature_points",
        material.temperature_points,
        after,
        "Temperature points",
        material.temperature_points === undefined
      );
    }
    catch (e) {
      state.setError(e instanceof Error ? e.message : String(e));
    }
  }
  return <section id="temperature-table" tabIndex={-1} aria-label="Material temperature table">
    <h3>Temperature-dependent properties</h3>
    <p>Enter values with their units and provenance. An incomplete point remains incomplete; load-case validation checks whether selected points can be used. Removing a referenced point may be blocked.</p>
    <fieldset disabled={state.busy}>
      <legend>{material.label}</legend>
      {rows.length === 0 && <p>No temperature points. Add a point to enter temperature-dependent properties.</p>}
      {rows.map((row, index) => <fieldset key={keys[index]}>
        <legend>Point {index + 1}</legend>
        <TextField
          label={`Point ${index + 1} ID`}
          value={row.id}
          onChange={value => update(index, "id", value)}
        />
        {([['temperature', 'Temperature'], ['elastic_modulus', 'Elastic modulus'], ['shear_modulus', 'Shear modulus'], ['thermal_expansion_coefficient', 'Thermal expansion coefficient']] as const).map(([key, label]) => <QuantityField
          dimension={key === "temperature" ? "temperature" : key === "thermal_expansion_coefficient" ? "inverse temperature" : "stress"}
          key={key}
          label={`Point ${index + 1} ${label}`}
          value={row[key]}
          onChange={value => update(index, key, value)}
        />)}
        <TextField
          label={`Point ${index + 1} provenance`}
          value={row.provenance}
          onChange={value => update(index, "provenance", value)}
        />
        <button type="button" onClick={() => {
          setRows(rows.filter((_, i) => i !== index));
          setKeys(keys.filter((_, i) => i !== index));
        }}>Remove point {index + 1}</button>
      </fieldset>)}
      <button type="button" onClick={() => {
        setRows([...rows, {}]);
        setKeys([...keys, nextKey]);
        setNextKey(nextKey + 1);
      }}>Add temperature point</button>
      <button type="button" onClick={() => void submit()}>Queue temperature points</button>
    </fieldset>
    <QueueFeedback {...state} />
  </section>;
}
