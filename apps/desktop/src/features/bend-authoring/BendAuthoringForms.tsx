import { useEffect, useState } from "react";
import { QueueFeedback, QuantityField, requireText, TextField, useRichQueue, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";
import { isPressureV3 } from "../pressure-authoring/pressureContract";
import type { PreviewComponent, PreviewModel } from "../../types";

/** T4-U2 (T4-I11 D-A, D-G): bend authoring for 3.0.0/exact_pressure_v3. A
 * realized bend carries the user's flexibility factor k; k is never suggested
 * or defaulted, and k = 1 means no extra flexibility beyond the curved beam.
 * A bend at a corner is inserted with exact tangent points (Insert bend at
 * corner); a bend with an arc span is realized in place. T4-U4b: a realized
 * bend may carry the user's directional SIF pair (both or neither, under the
 * same source); none is suggested. */
export const K_HINT = "k multiplies the bend's bending flexibility; k = 1 means no extra flexibility beyond the curved beam. Enter your own value; none is suggested.";
export const SIF_HINT = "Optional directional SIFs: i_ip multiplies the in-plane bending moment M_z and i_op the out-of-plane M_y of the arc's tangent frame (intensified bending rows only; never k). Enter both or neither, with their source above; none is suggested.";
export const GEOMETRY_ONLY_BANNER = "Geometry-only bend: analysed as a straight chord with no flexibility. The exact pressure contract refuses it. Realize this bend.";
const CURVED = "curved_bend_macro_element";

const isBend = (component: PreviewComponent) => component.kind === "bend" || component.kind === "elbow";
const realized = (component: PreviewComponent) => isBend(component) && component.mechanics_interface?.solver_consumption === CURVED;

/** The projection `bend_realization` replaces (the engine's), or undefined. */
export function realizationProjection(component: PreviewComponent): DraftRecord | undefined {
  const out: DraftRecord = {};
  if (component.mechanics_interface?.solver_consumption !== undefined) out.solver_consumption = component.mechanics_interface.solver_consumption;
  if (component.modifiers?.flexibility_factor_user_value !== undefined) out.flexibility_factor_user_value = component.modifiers.flexibility_factor_user_value;
  if (component.modifiers?.source_reference !== undefined) out.source_reference = component.modifiers.source_reference;
  if (component.modifiers?.sif_in_plane_user_value !== undefined) out.sif_in_plane_user_value = component.modifiers.sif_in_plane_user_value;
  if (component.modifiers?.sif_out_of_plane_user_value !== undefined) out.sif_out_of_plane_user_value = component.modifiers.sif_out_of_plane_user_value;
  return Object.keys(out).length ? out : undefined;
}

/** The canonical corner context `insert_bend_at_corner` names (the engine's). */
export function cornerContext(model: PreviewModel, node: string) {
  return {
    node: model.nodes.find(item => item.id === node) ?? null,
    pipes: model.pipe_segments.filter(pipe => pipe.from === node || pipe.to === node),
    markers: model.components.filter(component => isBend(component) && component.node === node)
  };
}

function flexibilityValue(text: string): number {
  const k = Number(text);
  if (text.trim() === "" || !Number.isFinite(k) || k <= 0) throw new Error("Enter the bend's flexibility factor k (finite and positive).");
  return k;
}

/** The directional SIF pair as entered: both blank is none; otherwise both finite and positive. */
export function directionalPair(inPlane: string, outOfPlane: string): DraftRecord {
  if (inPlane.trim() === "" && outOfPlane.trim() === "") return {};
  const factor = (text: string) => {
    const value = Number(text);
    if (text.trim() === "" || !Number.isFinite(value) || value <= 0) throw new Error("Enter both directional SIFs (finite and positive), or neither.");
    return { value, unit: "1" };
  };
  return { sif_in_plane_user_value: factor(inPlane), sif_out_of_plane_user_value: factor(outOfPlane) };
}

export function BendRealizationForm(props: RichFormProps & { component: PreviewComponent }) {
  const { component } = props;
  const [k, setK] = useState(() => String(component.modifiers?.flexibility_factor_user_value?.value ?? ""));
  const [source, setSource] = useState(() => component.modifiers?.source_reference ?? "");
  const [ip, setIp] = useState(() => String(component.modifiers?.sif_in_plane_user_value?.value ?? ""));
  const [op, setOp] = useState(() => String(component.modifiers?.sif_out_of_plane_user_value?.value ?? ""));
  useEffect(() => {
    setK(String(component.modifiers?.flexibility_factor_user_value?.value ?? "")); setSource(component.modifiers?.source_reference ?? "");
    setIp(String(component.modifiers?.sif_in_plane_user_value?.value ?? "")); setOp(String(component.modifiers?.sif_out_of_plane_user_value?.value ?? ""));
  }, [component]);
  const state = useRichQueue(props);
  const before = realizationProjection(component);
  const queue = (after: DraftRecord, label: string) => state.queue({ object_type: "Component", ref: component.id }, "set_field", "bend_realization", before, after, label, before === undefined);
  async function realize() { try {
    await queue({ solver_consumption: CURVED, flexibility_factor_user_value: { value: flexibilityValue(k), unit: "none" }, source_reference: requireText(source, "Source of k"), ...directionalPair(ip, op) }, "Bend realization");
  } catch (e) { state.setError(e instanceof Error ? e.message : String(e)); } }
  const isRealized = realized(component);
  return <section aria-label="Bend realization" data-testid="bend-realization-form"><h3>Bend realization</h3>
    {!isRealized && isPressureV3(props.model) && <p role="note" data-testid="bend-geometry-only-banner">{GEOMETRY_ONLY_BANNER}</p>}
    <p data-testid="bend-realization-state">{isRealized
      ? `Realized curved bend over ${component.geometry?.bend_pipe_ref ?? "(no span)"}: k = ${component.modifiers?.flexibility_factor_user_value?.value ?? "not entered"} (${component.modifiers?.source_reference ?? "no source"}). k is in the stiffness.${component.modifiers?.sif_in_plane_user_value ? ` Directional SIFs i_ip = ${component.modifiers.sif_in_plane_user_value.value}, i_op = ${component.modifiers.sif_out_of_plane_user_value?.value ?? "not entered"}.` : ""}`
      : component.geometry?.bend_pipe_ref
        ? `Geometry-only bend over ${component.geometry.bend_pipe_ref}. Realizing it forms the arc from the span's chord, the radius and the span's y_reference; both ends must be tangent to their neighbours.`
        : "This bend names no arc span: it marks a corner. Select its node and use Insert bend at corner, which places the tangent points."}</p>
    <fieldset disabled={state.busy}>
      <label>Flexibility factor k<input aria-label="Bend flexibility factor k" inputMode="decimal" value={k} onChange={e => setK(e.target.value)} /></label>
      <p>{K_HINT}</p>
      <TextField label="Source of k" value={source} onChange={setSource} />
      <label>In-plane SIF i_ip<input aria-label="Bend in-plane SIF i_ip" inputMode="decimal" value={ip} onChange={e => setIp(e.target.value)} /></label>
      <label>Out-of-plane SIF i_op<input aria-label="Bend out-of-plane SIF i_op" inputMode="decimal" value={op} onChange={e => setOp(e.target.value)} /></label>
      <p>{SIF_HINT}</p>
      <button type="button" onClick={() => void realize()}>{isRealized ? "Queue bend realization update" : "Queue realize bend"}</button>
      {isRealized && <button type="button" onClick={() => void queue({ solver_consumption: "mechanics_geometry_only" }, "Bend realization")}>Queue return to geometry-only</button>}
    </fieldset><QueueFeedback {...state} />
  </section>;
}

export function InsertBendAtCornerForm(props: RichFormProps & { node: string }) {
  const { model, node } = props;
  const context = cornerContext(model, node);
  const marker = context.markers[0];
  const token = node.replace(/^[^:]*:/, "");
  const [draft, setDraft] = useState<DraftRecord>(() => ({
    radius: { unit: model.project.units.length ?? "" },
    start: `node:${token}-T1`, end: `node:${token}-T2`, arc: `pipe:${token}-bend`, component: `component:${token}-bend`,
  }));
  const [convert, setConvert] = useState(Boolean(marker));
  const update = (key: string, value: unknown) => setDraft(current => ({ ...current, [key]: value }));
  const state = useRichQueue(props);
  async function submit() { try {
    if (context.pipes.length !== 2) throw new Error("A corner joins exactly two pipes.");
    const radius = draft.radius as DraftRecord;
    const value = Number(radius.value);
    if (String(radius.value ?? "").trim() === "" || !Number.isFinite(value) || value <= 0) throw new Error("Enter the bend radius R (finite and positive).");
    const provenance = requireText(draft.provenance, "Provenance");
    const record = (id: unknown, label: string) => ({ id: requireText(id, "New ID"), label, provenance });
    const after = {
      radius: { value, unit: requireText(radius.unit, "Radius unit") },
      flexibility_factor: { value: flexibilityValue(String(draft.k ?? "")), unit: "none" },
      flexibility_source_reference: requireText(draft.kSource, "Source of k"),
      new_nodes: [record(draft.start, "Bend start"), record(draft.end, "Bend end")],
      new_pipe: record(draft.arc, "Bend arc"),
      component: convert && marker
        ? { convert: marker.id, ...(typeof draft.geometrySource === "string" && draft.geometrySource.trim() ? { geometry_source_reference: draft.geometrySource } : {}) }
        : { ...record(draft.component, "Bend"), geometry_source_reference: requireText(draft.geometrySource, "Geometry source reference") },
    };
    await state.queue({ object_type: "Node", ref: node }, "insert_bend_at_corner", "bend_at_corner", context, after, "Insert bend at corner");
  } catch (e) { state.setError(e instanceof Error ? e.message : String(e)); } }
  return <section aria-label="Insert bend at corner" data-testid="insert-bend-at-corner-form"><h3>Insert bend at corner</h3>
    <p>Replaces corner {node} by a realized bend of radius R between its two straights: the tangent points are placed exactly, both pipes are shortened, the arc span joins them and the bend carries your k. Supports, loads and regions on the corner or either pipe must be removed first; nothing is repartitioned.</p>
    <fieldset disabled={state.busy}>
      <QuantityField label="Bend radius R" dimension="length" value={draft.radius} onChange={v => update("radius", v)} />
      <label>Flexibility factor k<input aria-label="Corner bend flexibility factor k" inputMode="decimal" value={String(draft.k ?? "")} onChange={e => update("k", e.target.value)} /></label>
      <p>{K_HINT}</p>
      <TextField label="Corner bend source of k" value={draft.kSource} onChange={v => update("kSource", v)} />
      <TextField label="Bend start node ID" value={draft.start} onChange={v => update("start", v)} />
      <TextField label="Bend end node ID" value={draft.end} onChange={v => update("end", v)} />
      <TextField label="Bend arc pipe ID" value={draft.arc} onChange={v => update("arc", v)} />
      {marker
        ? <label><input type="checkbox" aria-label={`Convert marker ${marker.id}`} checked={convert} onChange={e => setConvert(e.target.checked)} />Convert the bend marker {marker.id} in place (keeps its identity and other data)</label>
        : <TextField label="Bend component ID" value={draft.component} onChange={v => update("component", v)} />}
      <TextField label="Geometry source reference" value={draft.geometrySource} onChange={v => update("geometrySource", v)} />
      <TextField label="Corner bend provenance" value={draft.provenance} onChange={v => update("provenance", v)} />
      <button type="button" onClick={() => void submit()}>Queue insert bend at corner</button>
    </fieldset><QueueFeedback {...state} />
  </section>;
}

/** Mounted by the inspector: realization for a selected bend; insertion for a
 * selected node joining exactly two pipes. */
export function BendAuthoringForms(props: RichFormProps) {
  const { model, selection } = props;
  if (selection.type === "component") {
    const component = model.components.find(item => item.id === selection.id);
    return component && isBend(component) ? <BendRealizationForm key={component.id} {...props} component={component} /> : null;
  }
  if (selection.type === "node" && model.pipe_segments.filter(pipe => pipe.from === selection.id || pipe.to === selection.id).length === 2) {
    return <InsertBendAtCornerForm key={selection.id} {...props} node={selection.id} />;
  }
  return null;
}
