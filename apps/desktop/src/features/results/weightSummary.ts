/** Weight and centre-of-gravity summary (survey item A9; DEL-07-05 Design
 * `weight-cog-summary.md`). Model data, no solve: the per-entity weights are the
 * self-weight generator's own outputs (`core/product_physics/src/self_weight.rs`,
 * reached through `generateSelfWeightPlan`), run as a plan that is never queued or
 * applied. This module only sums them and forms the centre of gravity
 * Σ(wᵢ·pᵢ)/Σwᵢ over the same entities. It computes no mass and restates no weight
 * rule: a pipe's weight is the generator's uniform intensity (N/m) over its straight
 * span (the resultant of a uniform load, at the span's midpoint), and a component's is
 * the generator's normalized weight at its node plus its centre-of-gravity offset.
 *
 * `planWeightSummary`, `collectSelfWeightEvidence` and `buildWeightSummary` are free
 * of React; only `collectSelfWeightEvidence` awaits (the injected engine call). */

import type { SelfWeightOperationPlan, SelfWeightPlanOutcome } from "../../services/selfWeightPlanService";
import type { PreviewModel, Vec3 } from "../../types";
import { millimetersPerLengthUnit } from "../exportLengthConversion";

/** The generator's evidence method of a member's distributed self-weight (`GENERATED_METHOD`). */
export const PIPE_WEIGHT_METHOD = "pipe_mass_per_length_times_explicit_axis_acceleration/v2";
/** The generator's evidence method of a component's weight (`COMPONENT_WEIGHT_METHOD`). */
export const COMPONENT_WEIGHT_METHOD = "component_weight_at_center_of_gravity_global_offset/v1";
/** The generator's warning when a mapping component's weight is not applied (`COMPONENT_WEIGHT_NOT_APPLIED`). */
export const COMPONENT_WEIGHT_NOT_APPLIED = "SELF_WEIGHT_COMPONENT_WEIGHT_NOT_APPLIED";

/** The request's own fields: a diagnostic naming one is never a pipe's. */
const REQUEST_LEVEL_FIELDS = new Set(["case_id", "label", "provenance", "source_model_hash", "gravity", "gravity.axis", "pipe_refs"]);
/** The allow-list of pipe-local refusals (Design §3.2): the engine codes whose condition is a
 * property of one requested pipe, its section, or its mapped component, so it recurs whenever that
 * pipe is requested, alone or with others. Any other code is never split. */
export const PIPE_LOCAL_CODES: ReadonlySet<string> = new Set([
  "SELF_WEIGHT_INPUT_INVALID",
  "SELF_WEIGHT_RANGE_INVALID",
  "UNIT_CONVERSION_UNAVAILABLE",
  "SECTION_REFERENCE_INVALID",
  "SECTION_BINDING_UNSUPPORTED",
  "SECTION_REFERENCE_CACHE_STALE",
  "PIPE_DIMENSION_INVALID",
  "RIGID_COMPONENT_MAPPING_CONFLICT"
]);
/** The per-pipe section fields the engine names, in place of the pipe, when one of them does not
 * normalize (`normalize` in `self_weight.rs`). */
const SECTION_FIELDS = new Set(["outside_diameter", "wall_thickness", "mill_tolerance", "contents_density",
  "insulation_thickness", "insulation_density", "material_density", "corrosion_allowance"]);
const SECTION_FIELD_CODES = new Set(["SELF_WEIGHT_INPUT_INVALID", "SELF_WEIGHT_RANGE_INVALID", "UNIT_CONVERSION_UNAVAILABLE"]);

export type WeightScopeKind = "model" | "line" | "selection";
export type WeightSummaryScope = Readonly<{
  id: string;
  kind: WeightScopeKind;
  label: string;
  /** The model pipes in scope, in scope order, each once. */
  pipeRefs: readonly string[];
  /** References in the scope's source (a line's `pipe_refs`, the selection) that name no model pipe. */
  unresolved: readonly string[];
}>;

export type WeightGravityInput = Readonly<{ value: string; unit: string; axis: string }>;
export type WeightGravity = Readonly<{ value: number; unit: string; axis: "global_x" | "global_y" | "global_z" }>;

const AXES = ["global_x", "global_y", "global_z"] as const;

/** The entered gravity, or why it cannot be used. No value, unit or axis is assumed. */
export function parseWeightGravity(input: WeightGravityInput): { gravity: WeightGravity } | { error: string } {
  const text = input.value.trim();
  const unit = input.unit.trim();
  if (!text || !unit || !input.axis) return { error: "Enter the gravity value, its unit and its direction." };
  const value = Number(text);
  if (!Number.isFinite(value) || value === 0) return { error: "Gravity requires a finite, nonzero number." };
  if (!(AXES as readonly string[]).includes(input.axis)) return { error: "Choose a global gravity direction." };
  return { gravity: Object.freeze({ value, unit, axis: input.axis as WeightGravity["axis"] }) };
}

function uniqueResolved(model: PreviewModel, refs: readonly string[]): { pipeRefs: string[]; unresolved: string[] } {
  const known = new Set(model.pipe_segments.map((pipe) => pipe.id));
  const seen = new Set<string>();
  const pipeRefs: string[] = [];
  const unresolved: string[] = [];
  for (const ref of refs) {
    if (seen.has(ref)) continue;
    seen.add(ref);
    (known.has(ref) ? pipeRefs : unresolved).push(ref);
  }
  return { pipeRefs, unresolved };
}

const plural = (count: number, word: string) => `${count} ${word}${count === 1 ? "" : "s"}`;

/** The scopes the model supports: the whole model, each line (W-13 `lines`), and the
 * session's selected pipes when there are any. */
export function weightSummaryScopes(model: PreviewModel, selectedPipeRefs: readonly string[] = []): WeightSummaryScope[] {
  const scopes: WeightSummaryScope[] = [Object.freeze({
    id: "model",
    kind: "model" as const,
    label: `Whole model (${plural(model.pipe_segments.length, "pipe")})`,
    pipeRefs: Object.freeze(uniqueResolved(model, model.pipe_segments.map((pipe) => pipe.id)).pipeRefs),
    unresolved: Object.freeze([])
  })];
  for (const line of model.lines ?? []) {
    const { pipeRefs, unresolved } = uniqueResolved(model, line.pipe_refs ?? []);
    const name = line.label && line.label !== line.id ? `${line.label} (${line.id})` : line.id;
    scopes.push(Object.freeze({
      id: `line:${line.id}`, kind: "line" as const, label: `Line ${name} (${plural(pipeRefs.length, "pipe")})`,
      pipeRefs: Object.freeze(pipeRefs), unresolved: Object.freeze(unresolved)
    }));
  }
  if (selectedPipeRefs.length > 0) {
    const { pipeRefs, unresolved } = uniqueResolved(model, selectedPipeRefs);
    scopes.push(Object.freeze({
      id: "selection", kind: "selection" as const, label: `Selected pipes (${pipeRefs.length})`,
      pipeRefs: Object.freeze(pipeRefs), unresolved: Object.freeze(unresolved)
    }));
  }
  return scopes;
}

export type BendSpan = Readonly<{ pipeRef: string; componentIds: readonly string[] }>;
export type WeightSummaryRequestPlan = Readonly<{
  /** The scope pipes sent to the generator, in scope order. */
  send: readonly string[];
  /** Scope pipes named as a bend span by a component's `geometry.bend_pipe_ref`; not sent, not counted. */
  bendSpans: readonly BendSpan[];
}>;

/** Splits the scope into the pipes sent to the generator and the bend spans, whose
 * length along a realized arc is the solver's and is not available to the desktop. */
export function planWeightSummary(model: PreviewModel, scope: WeightSummaryScope): WeightSummaryRequestPlan {
  const bends = new Map<string, string[]>();
  for (const component of model.components ?? []) {
    const ref = component.geometry?.bend_pipe_ref;
    if (typeof ref === "string" && ref.trim()) bends.set(ref, [...(bends.get(ref) ?? []), component.id]);
  }
  const send: string[] = [];
  const bendSpans: BendSpan[] = [];
  for (const ref of scope.pipeRefs) {
    const components = bends.get(ref);
    if (components) bendSpans.push(Object.freeze({ pipeRef: ref, componentIds: Object.freeze(components) }));
    else send.push(ref);
  }
  return Object.freeze({ send: Object.freeze(send), bendSpans: Object.freeze(bendSpans) });
}

/** A case id for the generator's request that names no existing case or primitive load.
 * The plan is never queued, so the id appears nowhere else. */
export function weightSummaryCaseId(model: PreviewModel): string {
  const used = new Set<string>();
  for (const loadCase of model.load_cases ?? []) {
    used.add(loadCase.id);
    for (const load of loadCase.primitive_loads ?? []) if (typeof load.id === "string") used.add(load.id);
  }
  const base = "weight-summary-view";
  if (!used.has(base)) return base;
  for (let index = 2; ; index += 1) if (!used.has(`${base}:${index}`)) return `${base}:${index}`;
}

export type EngineDiagnostic = SelfWeightPlanOutcome["diagnostics"][number];
export type SelfWeightEngineRun = (pipeRefs: readonly string[]) => Promise<SelfWeightPlanOutcome>;
export type SelfWeightEvidenceCollection =
  | Readonly<{
      state: "collected";
      plans: readonly Readonly<{ pipeRefs: readonly string[]; plan: SelfWeightOperationPlan }>[];
      refused: readonly Readonly<{ pipeRef: string; diagnostics: readonly EngineDiagnostic[] }>[];
      calls: number;
    }>
  | Readonly<{
      state: "refused";
      diagnostics: readonly EngineDiagnostic[];
      calls: number;
      /** True when the generator refused, taken together, pipes it had accepted in parts. */
      combined?: boolean;
    }>;

/** Whether one diagnostic is known to belong to one requested pipe (Design §3.2). Fail closed:
 * an unlisted code, a request field, no reference, or references to two requested pipes are not
 * pipe-local. A component reference is pipe-local only for the engine's per-component range
 * checks (`SELF_WEIGHT_RANGE_INVALID`); an input refusal naming a component is its generated
 * primitive identity collision, which depends on the other components requested with it. */
export function isPipeLocalDiagnostic(item: EngineDiagnostic, requested: ReadonlySet<string>, componentIds: ReadonlySet<string> = new Set()): boolean {
  if (!PIPE_LOCAL_CODES.has(item.code)) return false;
  const refs = Array.isArray(item.affected_objects) ? item.affected_objects : [];
  if (refs.length === 0 || refs.some((ref) => REQUEST_LEVEL_FIELDS.has(ref))) return false;
  // An input refusal naming a component is its generated identity collision, even when a pipe shares the id.
  if (item.code === "SELF_WEIGHT_INPUT_INVALID" && refs.some((ref) => componentIds.has(ref))) return false;
  const pipes = refs.filter((ref) => requested.has(ref));
  if (pipes.length > 0) return pipes.length === 1;
  if (refs.length !== 1) return false;
  if (SECTION_FIELDS.has(refs[0])) return SECTION_FIELD_CODES.has(item.code);
  return item.code === "SELF_WEIGHT_RANGE_INVALID" && componentIds.has(refs[0]);
}

const PLAN_ABSENT: EngineDiagnostic = { code: "SELF-WEIGHT-PLAN-ABSENT", severity: "blocking",
  message: "The self-weight generator returned no plan and no diagnostic.", remediation: "Compute again.", affected_objects: [] };

/** Runs the generator over the pipes. One call when every pipe is ready. A refusal whose every
 * diagnostic is pipe-local (`isPipeLocalDiagnostic`) is split in halves until each refusal is held
 * by a single pipe, so one pipe's missing input does not hide the others' weights. Any other
 * refusal, at any level, is a refusal of the whole request and stops the collection. When pipes
 * were accepted in more than one part, one last call requests them together: the engine may have
 * returned a pipe-local refusal first and not yet reached a conflict between pipes, and a
 * refusal of that call is a refusal of the whole request. */
export async function collectSelfWeightEvidence(
  pipeRefs: readonly string[],
  run: SelfWeightEngineRun,
  options: Readonly<{ componentIds?: Iterable<string> }> = {}
): Promise<SelfWeightEvidenceCollection> {
  const componentIds = new Set(options.componentIds ?? []);
  let plans: { pipeRefs: readonly string[]; plan: SelfWeightOperationPlan }[] = [];
  const refused: { pipeRef: string; diagnostics: readonly EngineDiagnostic[] }[] = [];
  let calls = 0;
  let stop: readonly EngineDiagnostic[] | null = null;
  async function call(refs: readonly string[]): Promise<SelfWeightPlanOutcome | null> {
    calls += 1;
    try {
      return await run(refs);
    } catch (error) {
      stop = [{ code: "SELF-WEIGHT-ENGINE-UNAVAILABLE", severity: "blocking", message: error instanceof Error ? error.message : String(error),
        remediation: "Make the self-weight engine available and compute again.", affected_objects: [] }];
      return null;
    }
  }
  async function visit(refs: readonly string[]): Promise<void> {
    if (refs.length === 0 || stop) return;
    const outcome = await call(refs);
    if (!outcome) return;
    if (outcome.status === "ready" && outcome.plan) {
      plans.push({ pipeRefs: Object.freeze([...refs]), plan: outcome.plan });
      return;
    }
    const diagnostics = Array.isArray(outcome.diagnostics) ? outcome.diagnostics : [];
    const requested = new Set(refs);
    if (diagnostics.length === 0 || !diagnostics.every((item) => isPipeLocalDiagnostic(item, requested, componentIds))) {
      stop = diagnostics.length > 0 ? diagnostics : [PLAN_ABSENT];
      return;
    }
    if (refs.length === 1) {
      refused.push({ pipeRef: refs[0], diagnostics: Object.freeze([...diagnostics]) });
      return;
    }
    const middle = Math.ceil(refs.length / 2);
    await visit(refs.slice(0, middle));
    await visit(refs.slice(middle));
  }
  await visit(pipeRefs);
  let combined = false;
  if (!stop && plans.length > 1) {
    const accepted = new Set(plans.flatMap((item) => item.pipeRefs));
    const together = pipeRefs.filter((ref) => accepted.has(ref));
    const outcome = await call(together);
    if (outcome?.status === "ready" && outcome.plan) {
      plans = [{ pipeRefs: Object.freeze(together), plan: outcome.plan }];
    } else if (outcome) {
      const diagnostics = Array.isArray(outcome.diagnostics) ? outcome.diagnostics : [];
      stop = diagnostics.length > 0 ? diagnostics : [PLAN_ABSENT];
      combined = true;
    }
  }
  if (stop) {
    return Object.freeze({ state: "refused" as const, diagnostics: Object.freeze([...(stop as readonly EngineDiagnostic[])]), calls,
      ...(combined ? { combined: true } : {}) });
  }
  return Object.freeze({ state: "collected" as const, plans: Object.freeze(plans.map((item) => Object.freeze(item))), refused: Object.freeze(refused), calls });
}

export type WeightEntity = Readonly<{
  kind: "pipe" | "component";
  /** The scope pipe this weight stands for (a component's weight stands in for its mapped member's). */
  pipeRef: string;
  /** The component, for a component weight. */
  componentId: string | null;
  weight: number;
  unit: string;
  /** Where the weight acts, in the model's length unit. */
  position: Vec3;
  /** Pipes only: the generator's statement that contents or insulation entered the mass. */
  contents: boolean | null;
  insulation: boolean | null;
}>;
export type NotCountedPipe = Readonly<{ pipeRef: string; reason: string }>;
export type NotIncludedComponent = Readonly<{ componentId: string; reason: string }>;
export type CategoryTotal = Readonly<{ count: number; totals: readonly Readonly<{ unit: string; weight: number }>[] }>;

export type WeightSummary =
  | Readonly<{ state: "unavailable"; scope: WeightSummaryScope; reason: string; diagnostics: readonly EngineDiagnostic[] }>
  | Readonly<{
      state: "available";
      scope: WeightSummaryScope;
      gravity: WeightGravity;
      lengthUnit: string;
      entities: readonly WeightEntity[];
      pipes: CategoryTotal;
      components: CategoryTotal;
      /** One unit's total over every counted entity, or null when units differ or nothing is counted. */
      total: Readonly<{ weight: number; unit: string }> | null;
      /** In the model's length unit; null when no total is formed or it is not positive. */
      centreOfGravity: Vec3 | null;
      /** Why no total or no centre of gravity is formed. */
      note: string | null;
      notCounted: readonly NotCountedPipe[];
      componentsNotIncluded: readonly NotIncludedComponent[];
      /** Scope pipes whose weight (their own or their component's) is counted. */
      countedPipeCount: number;
      /** True when a scope pipe, a component weight or a scope reference is left out: the total
       * and centre of gravity are then those of the counted weights only. */
      partial: boolean;
      contentsCount: number;
      insulationCount: number;
      engineCalls: number;
    }>;

const record = (value: unknown): Record<string, unknown> | null =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : null;

function finiteVector(value: unknown): [number, number, number] | null {
  return Array.isArray(value) && value.length === 3 && value.every((item) => typeof item === "number" && Number.isFinite(item))
    ? [value[0], value[1], value[2]] : null;
}

function finitePosition(position: Vec3): boolean {
  return Number.isFinite(position.x) && Number.isFinite(position.y) && Number.isFinite(position.z);
}

/** (a + b) / 2, or a / 2 + b / 2 when the sum leaves the binary64 range. */
function midpoint(a: number, b: number): number {
  const sum = (a + b) / 2;
  return Number.isFinite(sum) ? sum : a / 2 + b / 2;
}

function diagnosticText(diagnostics: readonly EngineDiagnostic[]): string {
  return diagnostics.map((item) => `${item.code}: ${item.message}`).join(" ") || "The self-weight generator refused this pipe.";
}

function sumByUnit(entities: readonly WeightEntity[]): Readonly<{ unit: string; weight: number }>[] {
  const totals = new Map<string, number>();
  for (const entity of entities) totals.set(entity.unit, (totals.get(entity.unit) ?? 0) + entity.weight);
  return [...totals].map(([unit, weight]) => Object.freeze({ unit, weight }));
}

const NORMAL_MIN = 2 ** -1022;
const CENTRE_OVERFLOW = "The centre of gravity is outside the representable range; none is formed.";
const CENTRE_UNDERFLOW = "A weighted position falls below the normal binary64 range, where it would lose precision; no centre of gravity is formed.";
const CENTRE_WEIGHT_SPREAD = "The counted weights are about 2^1022 apart or more, so the smaller cannot be scaled exactly within the normal binary64 range; no centre of gravity is formed.";

/** value × 2^exponent, in steps that stay representable. Exact when the exact result lies in the
 * normal range; below it the result is rounded (`weightedCentre` refuses that case). */
function scaleByPowerOfTwo(value: number, exponent: number): number {
  let scaled = value;
  let rest = exponent;
  while (rest > 1000) { scaled *= 2 ** 1000; rest -= 1000; }
  while (rest < -1000) { scaled *= 2 ** -1000; rest += 1000; }
  return scaled * 2 ** rest;
}

/** The e with 2^e ≤ value < 2^(e+1), for a positive finite value (subnormal included). */
function binaryExponent(value: number): number {
  let exponent = Math.floor(Math.log2(value));
  if (scaleByPowerOfTwo(value, -exponent) >= 2) exponent += 1;
  else if (scaleByPowerOfTwo(value, -exponent) < 1) exponent -= 1;
  return exponent;
}

/** Σ(wᵢ·pᵢ)/Σwᵢ (Design §3.4), range-safe. Every weight is first scaled by the power of two 2^s
 * that brings the largest into [1, 2), so Σwᵢ cannot overflow and a subnormal weight does not
 * underflow its products. Each step is guarded so that no rounding happens below the normal range:
 * - Scaling a weight is guaranteed exact when its exact scaled value is normal. Otherwise it may
 *   round (it may round up to 2^-1022 and pass a range test), so it is refused unless the scaled weight is
 *   normal and scales back to the stated weight. That holds exactly when wᵢ·2^s ≥ 2^-1022 (a
 *   subnormal scaled weight is refused even when its scaling is exact).
 * - A product with a nonzero coordinate, and a nonzero moment's quotient, is refused when its
 *   magnitude is at most 2^-1022: rounding is monotone, so a larger rounded value had a normal exact
 *   value, while one equal to 2^-1022 may have been rounded up from below (refused even if exact).
 * - Sums of normal products round in the normal range; a sum whose exact value is subnormal is exact.
 * Accepted steps are then ordinary binary64 operations on exactly scaled data, and a power of two
 * commutes with rounding in the normal range. So when the unscaled formula's products, sums and
 * quotients are all zero or normal and finite, a returned result is bit-for-bit the unscaled one
 * (a centre may still be refused by the normal-range policy); otherwise it is the scaled formula's
 * result or a stated refusal. Also refused when a moment or the result is
 * not finite. */
function weightedCentre(entities: readonly WeightEntity[]): { centre: Vec3 } | { reason: string } {
  let largest = 0;
  for (const entity of entities) largest = Math.max(largest, entity.weight);
  const shift = -binaryExponent(largest);
  let weights = 0;
  const moments = { x: 0, y: 0, z: 0 };
  for (const entity of entities) {
    const weight = scaleByPowerOfTwo(entity.weight, shift);
    if (!(weight >= NORMAL_MIN) || scaleByPowerOfTwo(weight, -shift) !== entity.weight) return { reason: CENTRE_WEIGHT_SPREAD };
    weights += weight;
    for (const axis of ["x", "y", "z"] as const) {
      const coordinate = entity.position[axis];
      const product = weight * coordinate;
      if (!Number.isFinite(product)) return { reason: CENTRE_OVERFLOW };
      if (coordinate !== 0 && Math.abs(product) <= NORMAL_MIN) return { reason: CENTRE_UNDERFLOW };
      moments[axis] += product;
    }
  }
  const centre = { x: moments.x / weights, y: moments.y / weights, z: moments.z / weights };
  for (const axis of ["x", "y", "z"] as const) {
    if (!Number.isFinite(moments[axis]) || !Number.isFinite(centre[axis])) return { reason: CENTRE_OVERFLOW };
    if (moments[axis] !== 0 && Math.abs(centre[axis]) <= NORMAL_MIN) return { reason: CENTRE_UNDERFLOW };
  }
  return { centre: Object.freeze(centre) };
}

/** The components whose stated weight the summary does not include, in every scope (Design §3.3):
 * components mapped (`geometry.rigid_pipe_ref`) onto a scope pipe or naming a scope bend span
 * (`geometry.bend_pipe_ref`), every component in the whole-model scope, and every component the
 * generator's warnings name. Read from the model, so a refused pipe or a bend span keeps its
 * components in the list. A component counted as an entity is not listed. */
function omittedComponents(
  model: PreviewModel,
  scope: WeightSummaryScope,
  request: WeightSummaryRequestPlan,
  entities: readonly WeightEntity[],
  notCounted: readonly NotCountedPipe[],
  warned: ReadonlyMap<string, string>
): readonly NotIncludedComponent[] {
  const inScope = new Set(scope.pipeRefs);
  const bendSpans = new Set(request.bendSpans.map((item) => item.pipeRef));
  const skipped = new Set(notCounted.map((item) => item.pipeRef));
  const pipeCounted = new Set(entities.filter((entity) => entity.kind === "pipe").map((entity) => entity.pipeRef));
  const componentCounted = new Set(entities.filter((entity) => entity.kind === "component").map((entity) => `${entity.componentId}\u0000${entity.pipeRef}`));
  const componentCountedAnywhere = new Set(entities.filter((entity) => entity.kind === "component").map((entity) => entity.componentId));
  const text = (value: unknown) => typeof value === "string" && value.trim() ? value : null;
  const out: NotIncludedComponent[] = [];
  const listed = new Set<string>();
  for (const component of model.components ?? []) {
    const mapped = text(component.geometry?.rigid_pipe_ref);
    const bend = text(component.geometry?.bend_pipe_ref);
    const counted = mapped === null ? componentCountedAnywhere.has(component.id) : componentCounted.has(`${component.id}\u0000${mapped}`);
    const warning = warned.get(component.id);
    if (counted || (component.geometry?.weight === undefined && warning === undefined)) continue;
    const mappedInScope = mapped !== null && inScope.has(mapped);
    const bendInScope = bend !== null && inScope.has(bend);
    if (scope.kind !== "model" && !mappedInScope && !bendInScope && warning === undefined) continue;
    let reason: string;
    if (warning !== undefined) reason = warning;
    else if (mappedInScope && bendSpans.has(mapped)) reason = `Its mapped pipe ${mapped} is a bend span, which is not counted; its weight is not included.`;
    else if (bendInScope && bendSpans.has(bend)) reason = `It names bend span ${bend}, which is not counted; its weight is not included.`;
    else if (mappedInScope && skipped.has(mapped)) reason = `Its mapped pipe ${mapped} is not counted (listed with the pipes not counted); its weight is not included.`;
    else if (mappedInScope && pipeCounted.has(mapped)) reason = `The self-weight generator does not apply its stated weight; the distributed weight of its mapped pipe ${mapped} is counted instead.`;
    else reason = "Its stated weight is applied by the self-weight generator to no counted member, so it is not included.";
    const key = `${component.id}\u0000${mapped ?? ""}`;
    if (listed.has(key)) continue;
    listed.add(key);
    out.push(Object.freeze({ componentId: component.id, reason }));
  }
  return Object.freeze(out);
}

/** Sums the generator's weights over the scope and forms the centre of gravity. Pure and
 * deterministic for its inputs; every output is frozen. */
export function buildWeightSummary(input: {
  model: PreviewModel;
  scope: WeightSummaryScope;
  gravity: WeightGravity;
  request: WeightSummaryRequestPlan;
  collection: SelfWeightEvidenceCollection;
}): WeightSummary {
  const { model, scope, gravity, request, collection } = input;
  if (collection.state === "refused") {
    return Object.freeze({ state: "unavailable" as const, scope,
      reason: collection.combined
        ? "The self-weight generator refused the pipes in scope taken together, though it accepted them in parts; a refusal that does not belong to one pipe is not split, and no weight is summed."
        : "The self-weight generator refused the request; no weight is summed.",
      diagnostics: collection.diagnostics });
  }
  const lengthUnit = model.project?.units?.length;
  const millimetres = millimetersPerLengthUnit(lengthUnit);
  if (millimetres === null) {
    return Object.freeze({ state: "unavailable" as const, scope,
      reason: `The model's length unit ${typeof lengthUnit === "string" && lengthUnit ? `"${lengthUnit}"` : "(missing)"} has no routine conversion to metres; no weight is summed.`,
      diagnostics: Object.freeze([]) });
  }
  const metresPerUnit = millimetres / 1000;
  const unitsPerMetre = 1000 / millimetres;
  const nodes = new Map(model.nodes.map((node) => [node.id, node.position]));
  const pipes = new Map(model.pipe_segments.map((pipe) => [pipe.id, pipe]));

  const pipeEvidence = new Map<string, Record<string, unknown>>();
  const componentEvidence = new Map<string, Record<string, unknown>>();
  const warned = new Map<string, string>();
  for (const { plan } of collection.plans) {
    for (const item of plan.source_evidence ?? []) {
      const evidence = record(item);
      if (!evidence) continue;
      if (evidence.method === PIPE_WEIGHT_METHOD && typeof evidence.pipe_id === "string") pipeEvidence.set(evidence.pipe_id, evidence);
      if (evidence.method === COMPONENT_WEIGHT_METHOD && typeof evidence.omitted_pipe_id === "string" && evidence.wrench_part === "force") {
        componentEvidence.set(evidence.omitted_pipe_id, evidence);
      }
    }
    for (const warning of plan.warnings ?? []) {
      if (warning.code !== COMPONENT_WEIGHT_NOT_APPLIED) continue;
      const componentId = warning.affected_refs?.[0];
      if (componentId) warned.set(componentId, warning.message);
    }
  }
  const refused = new Map(collection.refused.map((item) => [item.pipeRef, item.diagnostics]));

  const entities: WeightEntity[] = [];
  const notCounted: NotCountedPipe[] = [];
  const skip = (pipeRef: string, reason: string) => notCounted.push(Object.freeze({ pipeRef, reason }));
  for (const bend of request.bendSpans) {
    skip(bend.pipeRef, `Named as a bend span by ${bend.componentIds.join(", ")}; the length along a realized bend's arc is the solver's and is not available here.`);
  }
  for (const pipeRef of request.send) {
    const component = componentEvidence.get(pipeRef);
    const evidence = pipeEvidence.get(pipeRef);
    if (component) {
      const componentId = typeof component.component_id === "string" ? component.component_id : "(unnamed component)";
      const node = typeof component.component_node === "string" ? nodes.get(component.component_node) : undefined;
      const offset = finiteVector(component.center_of_gravity_m);
      const weight = component.weight_N;
      if (typeof weight !== "number" || !Number.isFinite(weight) || weight <= 0) { skip(pipeRef, `The generator's weight of ${componentId} is not a finite positive value.`); continue; }
      if (!node || !offset) { skip(pipeRef, `The node or centre of gravity of ${componentId} is not available.`); continue; }
      const position = { x: node.x + offset[0] * unitsPerMetre, y: node.y + offset[1] * unitsPerMetre, z: node.z + offset[2] * unitsPerMetre };
      if (!finitePosition(position)) { skip(pipeRef, `The position of ${componentId}'s centre of gravity is outside the representable range.`); continue; }
      entities.push(Object.freeze({
        kind: "component" as const, pipeRef, componentId, weight, unit: "N",
        position: Object.freeze(position),
        contents: null, insulation: null
      }));
      continue;
    }
    if (evidence) {
      const payload = record(evidence.generated_payload);
      const magnitude = record(payload?.magnitude);
      const value = magnitude?.value;
      const perLength = typeof magnitude?.unit === "string" ? /^(.+)\/m$/.exec(magnitude.unit) : null;
      const pipe = pipes.get(pipeRef);
      const from = pipe ? nodes.get(pipe.from) : undefined;
      const to = pipe ? nodes.get(pipe.to) : undefined;
      if (typeof value !== "number" || !Number.isFinite(value)) { skip(pipeRef, "The generator's intensity is not a finite number."); continue; }
      if (!perLength) { skip(pipeRef, `The generator's intensity unit ${String(magnitude?.unit)} is not per metre.`); continue; }
      if (!from || !to) { skip(pipeRef, "An end node of the pipe is not in the model."); continue; }
      const length = Math.hypot(to.x - from.x, to.y - from.y, to.z - from.z) * metresPerUnit;
      // The intensity carries the entered gravity's sign; the weight is its magnitude over the span.
      const weight = Math.abs(value) * length;
      if (!Number.isFinite(weight) || weight <= 0) { skip(pipeRef, "The pipe's weight is not a finite positive value."); continue; }
      entities.push(Object.freeze({
        kind: "pipe" as const, pipeRef, componentId: null, weight, unit: perLength[1],
        position: Object.freeze({ x: midpoint(from.x, to.x), y: midpoint(from.y, to.y), z: midpoint(from.z, to.z) }),
        contents: evidence.contents_absent === false, insulation: evidence.insulation_absent === false
      }));
      continue;
    }
    const diagnostics = refused.get(pipeRef);
    skip(pipeRef, diagnostics ? diagnosticText(diagnostics) : "The self-weight generator returned no weight for this pipe.");
  }

  const componentsNotIncluded = omittedComponents(model, scope, request, entities, notCounted, warned);

  const pipeEntities = entities.filter((entity) => entity.kind === "pipe");
  const componentEntities = entities.filter((entity) => entity.kind === "component");
  const all = sumByUnit(entities);
  const pipeTotals = sumByUnit(pipeEntities);
  const componentTotals = sumByUnit(componentEntities);
  if ([...all, ...pipeTotals, ...componentTotals].some((item) => !Number.isFinite(item.weight))) {
    return Object.freeze({ state: "unavailable" as const, scope,
      reason: "The summed weight is outside the representable range; no total or centre of gravity is formed.", diagnostics: Object.freeze([]) });
  }
  let total: { weight: number; unit: string } | null = null;
  let centreOfGravity: Vec3 | null = null;
  let note: string | null = null;
  if (all.length > 1) {
    note = `Units differ (${all.map((item) => item.unit).join(", ")}); no total or centre of gravity is formed.`;
  } else if (all.length === 0) {
    note = "No weight is counted; no total or centre of gravity is formed.";
  } else {
    total = Object.freeze({ weight: all[0].weight, unit: all[0].unit });
    const centre = weightedCentre(entities);
    if ("centre" in centre) centreOfGravity = centre.centre;
    else note = centre.reason;
  }
  return Object.freeze({
    state: "available" as const,
    scope,
    gravity,
    lengthUnit: lengthUnit as string,
    entities: Object.freeze(entities),
    pipes: Object.freeze({ count: pipeEntities.length, totals: Object.freeze(pipeTotals) }),
    components: Object.freeze({ count: componentEntities.length, totals: Object.freeze(componentTotals) }),
    total,
    centreOfGravity,
    note,
    notCounted: Object.freeze(notCounted),
    componentsNotIncluded,
    countedPipeCount: scope.pipeRefs.length - notCounted.length,
    partial: notCounted.length > 0 || componentsNotIncluded.length > 0 || scope.unresolved.length > 0,
    contentsCount: pipeEntities.filter((entity) => entity.contents).length,
    insulationCount: pipeEntities.filter((entity) => entity.insulation).length,
    engineCalls: collection.calls
  });
}

/** One summary through the generator: the A6 entry point. `run` takes the request the
 * panel would send; `hash` is the session's canonical model hash function. */
export async function computeWeightSummary(input: {
  model: PreviewModel;
  scope: WeightSummaryScope;
  gravity: WeightGravity;
  hash: (model: PreviewModel) => Promise<{ value: string } | null>;
  generate: (model: PreviewModel, request: {
    case_id: string; label: string; pipe_refs: string[];
    gravity: { value: number; unit: string; axis: string }; provenance: string; source_model_hash: string;
  }) => Promise<SelfWeightPlanOutcome>;
}): Promise<WeightSummary> {
  const { model, scope, gravity } = input;
  const request = planWeightSummary(model, scope);
  const hash = await input.hash(model);
  if (!hash) return Object.freeze({ state: "unavailable" as const, scope, reason: "The model hash is unavailable; no weight is summed.", diagnostics: Object.freeze([]) });
  const caseId = weightSummaryCaseId(model);
  const collection = await collectSelfWeightEvidence(request.send, (refs) => input.generate(model, {
    case_id: caseId,
    label: "Weight summary view (not applied)",
    pipe_refs: [...refs],
    gravity: { value: gravity.value, unit: gravity.unit, axis: gravity.axis },
    provenance: "Weight and centre-of-gravity summary view; the plan is read, never queued or applied.",
    source_model_hash: hash.value
  }), { componentIds: (model.components ?? []).map((component) => component.id) });
  return buildWeightSummary({ model, scope, gravity, request, collection });
}
