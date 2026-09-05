import { useEffect, useState } from "react";
import { clone, parseQuantities, QueueFeedback, QuantityField, requireText, setMember, TextField, useRichQueue, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
type Wind = {
  exposed_pipe_refs?: string[];
  exposed_spans?: DraftRecord[];
};
export function WindExposureForm(props: RichFormProps) {
  const load = props.selection.type === "load" ? props.model.load_cases.find(l => l.id === props.selection.id) : undefined;
  const wind = load?.equivalent_static?.wind as Wind | undefined;
  return load && wind ? <ExposureEditor
    key={load.id}
    {...props}
    wind={wind}
    loadId={load.id}
  /> : null;
}
function ExposureEditor(props: RichFormProps & {
  wind: Wind;
  loadId: string;
}) {
  const { wind } = props;
  const [whole, setWhole] = useState(() => clone(wind.exposed_pipe_refs ?? []));
  const [spans, setSpans] = useState(() => clone(wind.exposed_spans ?? []));
  const [keys, setKeys] = useState(() => spans.map((_, i) => i));
  const [nextKey, setNextKey] = useState(spans.length);
  useEffect(() => {
    setWhole(clone(wind.exposed_pipe_refs ?? []));
    setSpans(clone(wind.exposed_spans ?? []));
    setKeys((wind.exposed_spans ?? []).map((_, i) => i));
    setNextKey((wind.exposed_spans ?? []).length);
  }, [wind]);
  const state = useRichQueue(props);
  const pipeIds = props.model.pipe_segments.map(p => p.id);
  function update(index: number, key: string, value: unknown) {
    setSpans(current => current.map((s, i) => i === index ? setMember(s, key, value) : s));
  }
  async function submit() {
    try {
      const after = parseQuantities({ exposed_pipe_refs: whole, exposed_spans: spans }) as {
        exposed_pipe_refs: string[];
        exposed_spans: Array<{
          pipe_ref: string;
          start_fraction?: {
            value: number;
            unit: string;
          };
          end_fraction?: {
            value: number;
            unit: string;
          };
        }>;
      };
      const ranges = new Map<string, Array<[
        number,
        number
      ]>>();
      for (const span of after.exposed_spans) {
        requireText(span.pipe_ref, "Span pipe");
        if (!pipeIds.includes(span.pipe_ref))
          throw new Error("Every exposure span must reference an existing pipe.");
        if (whole.includes(span.pipe_ref))
          throw new Error("Choose either whole-pipe or partial exposure for each pipe.");
        if (!span.start_fraction || !span.end_fraction)
          throw new Error("Enter both start and end fractions with units.");
        if (!["1", "none"].includes(span.start_fraction.unit) || !["1", "none"].includes(span.end_fraction.unit))
          throw new Error("Fraction units must be 1 or none.");
        const start = span.start_fraction.value, end = span.end_fraction.value;
        if (start < 0 || start >= end || end > 1)
          throw new Error("Fractions require 0 ≤ start < end ≤ 1.");
        const previous = ranges.get(span.pipe_ref) ?? [];
        if (previous.some(([a, b]) => start < b && a < end))
          throw new Error("Partial exposure spans on the same pipe must not overlap.");
        ranges.set(span.pipe_ref, [...previous, [start, end]]);
      }
      await state.queue(
        { object_type: "Load", ref: props.loadId },
        "update_load",
        "equivalent_static.wind.exposure",
        { exposed_pipe_refs: wind.exposed_pipe_refs ?? [], exposed_spans: wind.exposed_spans ?? [] },
        after,
        "Wind exposure"
      );
    }
    catch (e) {
      state.setError(e instanceof Error ? e.message : String(e));
    }
  }
  return <section id="wind-exposure-form" tabIndex={-1} aria-label="Wind exposure">
    <h3>Wind exposure</h3>
    <p>Mark whole pipes or enter partial spans measured from each pipe’s start node. Fractions use unit 1 or none. Empty exposure remains incomplete for wind generation.</p>
    <fieldset disabled={state.busy}>
      <legend>Exposed pipes and spans</legend>
      <fieldset>
        <legend>Whole pipes</legend>{pipeIds.map(id => <label key={id}>
          <input
            type="checkbox"
            aria-label={`Whole pipe ${id}`}
            checked={whole.includes(id)}
            onChange={e => setWhole(e.target.checked ? [...whole, id] : whole.filter(p => p !== id))}
          />
          {id}</label>)}
        {whole.filter(id => !pipeIds.includes(id)).map(id => <p key={id} role="alert">Missing pipe {id} <button type="button" onClick={() => setWhole(whole.filter(p => p !== id))}>Remove missing pipe {id}</button>
        </p>)}</fieldset>
      {spans.map((span, index) => <fieldset key={keys[index]}>
        <legend>Span {index + 1}</legend>
        <TextField
          label={`Span ${index + 1} pipe`}
          value={span.pipe_ref}
          choices={pipeIds}
          onChange={value => update(index, "pipe_ref", value)}
        />
        <QuantityField
          dimension="dimensionless fraction"
          label={`Span ${index + 1} start fraction`}
          value={span.start_fraction}
          onChange={value => update(index, "start_fraction", value)}
        />
        <QuantityField
          dimension="dimensionless fraction"
          label={`Span ${index + 1} end fraction`}
          value={span.end_fraction}
          onChange={value => update(index, "end_fraction", value)}
        />
        <button type="button" onClick={() => {
          setSpans(spans.filter((_, i) => i !== index));
          setKeys(keys.filter((_, i) => i !== index));
        }}>Remove span {index + 1}</button>
      </fieldset>)}<button type="button" onClick={() => {
        setSpans([...spans, {}]);
        setKeys([...keys, nextKey]);
        setNextKey(nextKey + 1);
      }}>Add exposure span</button>
      <button type="button" onClick={() => void submit()}>Queue wind exposure</button>
    </fieldset>
    <QueueFeedback {...state} />
  </section>;
}
