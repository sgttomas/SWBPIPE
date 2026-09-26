import { useEffect, useState } from "react";
import { record, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
import {
  clone, isLoadStateModel, LoadStateFeedback, LoadStateNeeds040, member, NOT_PRESENT, Quantity, replaceAt, rows,
  switchVariant, Text, toPayload, useLoadStateQueue, type LoadStateTarget
} from "../load-cases/loadStateAuthoring";

type Material = NonNullable<RichFormProps["model"]["materials"]>[number];
const DEFINITIONS = {
  engineering_secant: ["id", "datum_temperature", "data", "provenance"],
  engineering_dilation: ["id", "datum_temperature", "data", "provenance"],
  differential_per_datum_length: ["id", "datum_temperature", "data", "provenance"],
  logarithmic_per_current_length: ["id", "datum_temperature", "data", "provenance"]
} as const;
const DATA_KINDS: Record<string, readonly string[]> = {
  engineering_secant: ["constant", "table"],
  engineering_dilation: ["table"],
  differential_per_datum_length: ["table"],
  logarithmic_per_current_length: ["table"]
};
const DATA = { constant: ["coefficient"], table: ["interpolation", "points"] } as const;

/** Material 0.4.0 expansion laws (`Material.expansion_laws`): user-owned
 * definitions, never library values. Pre-0.4 models show the gating line. */
export function ExpansionLawsEditor(props: RichFormProps & { material: Material }) {
  if (!isLoadStateModel(props.model)) return <LoadStateNeeds040 testId="expansion-laws-needs-040" />;
  return <LawsForm key={props.material.id} {...props} />;
}

function LawsForm(props: RichFormProps & { material: Material }) {
  const { material } = props;
  const current = material.expansion_laws;
  const [draft, setDraft] = useState<DraftRecord[] | undefined>(() => clone(current ?? undefined) as DraftRecord[] | undefined);
  useEffect(() => setDraft(clone(current ?? undefined) as DraftRecord[] | undefined), [current]);
  const state = useLoadStateQueue(props);
  const spec: LoadStateTarget = { target: { object_type: "Material", ref: material.id }, kind: "set_field", path: "expansion_laws", label: "Expansion laws" };
  const list = draft ?? [];
  const editAt = (index: number, next: DraftRecord) => setDraft(replaceAt(list, index, next));
  return <section aria-label="Material expansion laws" data-testid="expansion-laws-editor">
    <h3>Expansion laws (model 0.4.0)</h3>
    <p>{current === undefined ? "expansion_laws is absent for this material." : current === null ? "expansion_laws is an explicit null." : `${current.length} applied expansion laws.`} Enter each datum, value and unit explicitly; no law or coefficient is supplied by the application.</p>
    <fieldset disabled={state.busy}>
      {list.map((law, l) => {
        const k = `Expansion law ${l + 1}`;
        const data = record(law.data);
        const definition = typeof law.definition === "string" ? law.definition : "";
        const tableValue = definition === "engineering_dilation" ? "dilation" : "coefficient";
        const interpolation = definition === "engineering_dilation" ? "linear_dilation" : "linear_coefficient";
        const points = rows(data.points) ?? [];
        const setData = (key: string, next: unknown) => editAt(l, member(law, "data", member(data, key, next)));
        const setPoints = (next: DraftRecord[]) => setData("points", next);
        return <fieldset key={l}><legend>{k}</legend>
          <Text label={`${k} definition`} value={law.definition} choices={Object.keys(DEFINITIONS)} onChange={v => editAt(l, switchVariant(law, "definition", v, DEFINITIONS) ?? {})} />
          <Text label={`${k} ID`} value={law.id} onChange={v => editAt(l, member(law, "id", v))} />
          <Quantity label={`${k} datum temperature`} dimension="temperature" value={law.datum_temperature} onChange={v => editAt(l, member(law, "datum_temperature", v))} />
          {definition && <Text label={`${k} data`} value={data.kind} choices={DATA_KINDS[definition] ?? []} onChange={v => editAt(l, member(law, "data", switchVariant(law.data, "kind", v, DATA)))} />}
          {data.kind === "constant" && <Quantity label={`${k} constant coefficient`} dimension="inverse temperature" value={data.coefficient} onChange={v => setData("coefficient", v)} />}
          {data.kind === "table" && <>
            <Text label={`${k} interpolation`} value={data.interpolation} choices={[interpolation]} onChange={v => setData("interpolation", v)} />
            {points.map((point, p) => {
              const kp = `${k} point ${p + 1}`;
              return <fieldset key={p}><legend>Point {p + 1}</legend>
                <Quantity label={`${kp} temperature`} dimension="temperature" value={point.temperature} onChange={v => setPoints(replaceAt(points, p, member(point, "temperature", v)))} />
                <Quantity label={`${kp} ${tableValue}`} dimension={tableValue === "dilation" ? "strain; unit 1" : "inverse temperature"} value={point[tableValue]} onChange={v => setPoints(replaceAt(points, p, member(point, tableValue, v)))} />
                <button type="button" onClick={() => setPoints(points.filter((_, i) => i !== p))}>Remove {kp}</button>
              </fieldset>;
            })}
            <button type="button" onClick={() => setPoints([...points, {}])}>Add point to {k.toLowerCase()}</button>
          </>}
          <Text label={`${k} provenance`} value={law.provenance} onChange={v => editAt(l, member(law, "provenance", v))} />
          <button type="button" onClick={() => setDraft(list.filter((_, i) => i !== l))}>Remove {k.toLowerCase()}</button>
        </fieldset>;
      })}
      <button type="button" onClick={() => setDraft([...list, {}])}>Add expansion law</button>
      <button type="button" onClick={() => void state.submit(spec, current, () => draft === undefined ? NOT_PRESENT : toPayload(draft))}>Queue expansion laws</button>
      {current !== undefined && <button type="button" onClick={() => void state.submit(spec, current, () => NOT_PRESENT)}>Queue removal of expansion laws</button>}
    </fieldset>
    <LoadStateFeedback {...state} testId="expansion-laws" />
  </section>;
}
