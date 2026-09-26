/** Closed preview-physics-1 reader checks (T0R S1_INTERFACE §9); never producer
 * authentication. The checks bind the published rows, diagnostics, headlines and
 * the closed `contract_evidence` statement to one another. They do not recompute
 * the solve, the enclosures or the support laws; numerical standing still needs
 * the caller's authentic source, model, input and build binding. Same meaning as
 * the Rust and Python readers. */
import table from "../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json";
import type { MechanicsResult, PreviewModel } from "../../types";

export const PREVIEW_PHYSICS_EVIDENCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1";
const PROFILE = "product_preview_mechanics_v1";
const LIMITATIONS: readonly string[] = table.supported_profile_limitations;
const RETIRED_KINDS: readonly string[] = table.retired_source_kinds;
export const PREVIEW_RETIRED_DIAGNOSTIC_CODES: readonly string[] = ["COMPONENT_STRESS_MULTIPLIER_APPLIED", "COMBINATION_STRESS_SUMMARY_SKIPPED"];

const MAXIMUM_KIND = "pipe_elastic_normal_stress_maximum_v2";
const INTENSIFIED_KIND = "component_equal_factor_intensified_bending_stress_v1";
const SUPPORT_COMPONENT_KIND = "support_reaction_component_v2";
const SUPPORT_FORCE_KIND = "support_reaction_force_magnitude_v2";
const SUPPORT_MOMENT_KIND = "support_reaction_moment_magnitude_v2";
const SUPPORT_KINDS = [SUPPORT_COMPONENT_KIND, SUPPORT_FORCE_KIND, SUPPORT_MOMENT_KIND];
const SUPPORT_COMPONENTS = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"] as const;
const SUPPORT_ROW_KINDS: Record<string, [string, string]> = {
  Fx: [SUPPORT_COMPONENT_KIND, "N"], Fy: [SUPPORT_COMPONENT_KIND, "N"], Fz: [SUPPORT_COMPONENT_KIND, "N"],
  Mx: [SUPPORT_COMPONENT_KIND, "N*m"], My: [SUPPORT_COMPONENT_KIND, "N*m"], Mz: [SUPPORT_COMPONENT_KIND, "N*m"],
  force_magnitude: [SUPPORT_FORCE_KIND, "N"], moment_magnitude: [SUPPORT_MOMENT_KIND, "N*m"],
};
const SUPPORT_SIGN = "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate";
const MAXIMUM_METADATA = {
  component: "maximum_absolute_normal_stress", coordinate_system: "pipe_section",
  location: "governing_station", basis: "recovered_from_open_mechanics_stress_components",
  sign_convention: "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim",
};
const INTENSIFIED_SIGN_PREFIX = "nonnegative i*hypot(My,Mz)/Z at the member end; i=";
const INTENSIFIED_SOURCE_KINDS = ["element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z"];
const EXTREMA_KEYS = ["pipe_id", "result_id", "station_fraction", "span_index", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa", "subdivisions", "approximation", "coefficient_basis", "enclosure_scope"];
const EXTREMA_CONSTANTS: Record<string, string> = {
  approximation: "piecewise_quadratic_straight_section_statics",
  coefficient_basis: "j_side_section_equilibrium_binary64",
  enclosure_scope: "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate",
};
const CASE_KEYS = ["load_case_id", "pipe_stress_extrema", "stress_maximum_coverage", "support_attribution", "intensified_measures"];
const INTENSIFIED_KEYS = ["result_id", "component_id", "pipe_id", "location", "factor_role", "sif", "sif_source_reference", "section_modulus_m3", "bending_moment_y_n_m", "bending_moment_z_n_m"];
const WITHHELD_REASONS = ["SUPPORT_ACTION_ATTRIBUTION_WITHHELD", "CONSTANT_EFFORT_NOT_CONSUMED"];
const PER_SUPPORT_NONLINEAR_KINDS = ["nonlinear_support_final_reaction", "nonlinear_support_final_displacement", "nonlinear_support_active_set_state_code"];
export const PREVIEW_COMBINATION_GATE_CODES = ["NONLINEAR_COMBINATION_REQUIRES_SOLVE", "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE", "COMBINATION_MODULUS_BASIS_MIXED"] as const;
const LINEAR_BASES = ["explicit_user_linear_combination", "explicit_user_result_state_subtraction"];
const RANGE_BASIS = "explicit_user_range_envelope";
// Representation guard only (S1 §9.7); not an engineering tolerance.
const GUARD = 64 * Number.EPSILON;
const TINY = 2.2250738585072014e-308; // binary64 minimum positive normal

type Row = MechanicsResult["results"][number];
type Json = Record<string, any>;

function demand(ok: unknown, detail: string): asserts ok {
  if (!ok) throw new Error(`SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: ${detail}`);
}
const text = (v: unknown): v is string => typeof v === "string" && v.length > 0;
const finite = (v: unknown): v is number => typeof v === "number" && Number.isFinite(v);
const isObject = (v: unknown): v is Json => !!v && typeof v === "object" && !Array.isArray(v);
function strings(v: unknown): v is string[] {
  return Array.isArray(v) && v.every(text) && new Set(v).size === v.length;
}
function shape(v: unknown, keys: readonly string[], detail: string): asserts v is Json {
  demand(isObject(v) && Object.keys(v).length === keys.length && keys.every(k => Object.hasOwn(v, k)), detail);
}
function finiteTree(v: unknown, seen = new Set<object>()): void {
  if (typeof v === "number") { demand(Number.isFinite(v), "nonfinite evidence"); return; }
  if (!v || typeof v !== "object") return;
  demand(!seen.has(v), "cyclic evidence"); seen.add(v);
  for (const item of Object.values(v)) finiteTree(item, seen);
  seen.delete(v);
}
const sameJson = (a: unknown, b: unknown): boolean => {
  if (a === b) return true;
  if (!isObject(a) || !isObject(b)) return false;
  const ka = Object.keys(a);
  return ka.length === Object.keys(b).length && ka.every(k => Object.hasOwn(b, k) && sameJson(a[k], b[k]));
};
const utf8 = new TextEncoder();
/** The producer's exact_source_identity ID(a, b, …): `{UTF-8 byte length}:{part}` per part, concatenated. */
export function lengthPrefixed(...parts: string[]): string {
  return parts.map(part => `${utf8.encode(part).length}:${part}`).join("");
}
/** Row-id segment `{L(a)}:{a}:{L(b)}:{b}` (S1 §3): parts joined by ':'; not ID(). */
export function lengthPrefixedSegments(...parts: string[]): string {
  return parts.map(part => `${utf8.encode(part).length}:${part}`).join(":");
}
/** Code-point (byte-order equivalent) comparison; never UTF-16 unit order. */
export function compareCodePoints(a: string, b: string): number {
  const x = Array.from(a), y = Array.from(b);
  for (let i = 0; i < Math.min(x.length, y.length); i++) {
    const d = x[i].codePointAt(0)! - y[i].codePointAt(0)!;
    if (d !== 0) return d < 0 ? -1 : 1;
  }
  return x.length === y.length ? 0 : x.length < y.length ? -1 : 1;
}
function consistentNorm(magnitude: number, [x, y, z]: number[]): boolean {
  const expected = Math.hypot(Math.hypot(x, y), z);
  return magnitude >= 0 && Math.abs(magnitude - expected) <= GUARD * Math.max(Math.abs(magnitude), TINY);
}
const basisOf = (row: Row) => row.basis_ref;
const sameCaseBasis = (row: Row, caseId: string) => sameJson(row.basis_ref, { ref_type: "load_case", ref_id: caseId });

/** First table variant, in table order, whose `source_basis` is absent or equal (S1 §1). */
export function previewPhysicsSignature(row: Row): (typeof table.rows)[number] | null {
  demand(!RETIRED_KINDS.includes(row.kind), `retired kind ${row.kind}`);
  const candidates = table.rows.filter(entry => entry.kind === row.kind);
  demand(candidates.length, `kind without signature ${row.kind}`);
  const units = candidates.filter(entry => entry.unit === row.unit);
  demand(units.length, `unit contradiction ${row.kind}/${row.unit}`);
  const metadata: Json = isObject(row.metadata) ? row.metadata : {};
  const component = metadata.component;
  for (const pool of [units.filter(e => e.component === component), units.filter(e => e.component === null)]) {
    if (pool.length) {
      const exact = pool.filter(e => !("source_basis" in e) || (e as { source_basis?: string }).source_basis === metadata.basis);
      demand(exact.length, `basis contradiction ${row.kind}`);
      return exact[0];
    }
  }
  demand(component === undefined || component === null, `component contradiction ${row.kind}/${component}`);
  return null;
}

function header(source: MechanicsResult): void {
  demand(!Object.hasOwn(source, "source_block_recovery") && !Object.hasOwn(source, "carrier_evidence"), "unsupported source namespace");
  demand(source.schema_version === "0.2.0", "schema version");
  demand(sameJson(source.producer, { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: PREVIEW_PHYSICS_EVIDENCE_CONTRACT_ID }), "producer");
  const f = source.formulation_basis;
  shape(f, ["profile_id", "limitations"], "formulation basis");
  demand(f.profile_id === PROFILE && Array.isArray(f.limitations) && f.limitations.length === LIMITATIONS.length && f.limitations.every((l: unknown, i: number) => l === LIMITATIONS[i]), "formulation profile or limitations");
  const evidence = source.contract_evidence;
  shape(evidence, ["preview_cases", "combination_gates"], "contract evidence namespace");
  demand(Array.isArray(evidence.preview_cases) && Array.isArray(evidence.combination_gates), "contract evidence lists");
  finiteTree(evidence);
}

function readCases(evidence: Json): Map<string, Json> {
  const cases = new Map<string, Json>();
  for (const c of evidence.preview_cases as unknown[]) {
    shape(c, CASE_KEYS, "preview case shape");
    const id = c.load_case_id;
    demand(text(id) && !cases.has(id), "preview case identity");
    cases.set(id, c);
    demand(Array.isArray(c.pipe_stress_extrema) && Array.isArray(c.intensified_measures), "preview case lists");
    const coverage = c.stress_maximum_coverage;
    shape(coverage, ["complete", "unavailable_pipe_ids", "outside_domain_pipe_ids"], "maximum coverage shape");
    demand(typeof coverage.complete === "boolean" && strings(coverage.unavailable_pipe_ids) && strings(coverage.outside_domain_pipe_ids), "maximum coverage values");
    demand(!coverage.unavailable_pipe_ids.some((p: string) => coverage.outside_domain_pipe_ids.includes(p)), "maximum coverage overlap");
    demand(coverage.complete === (coverage.unavailable_pipe_ids.length === 0 && coverage.outside_domain_pipe_ids.length === 0), "maximum coverage completeness");
    const attribution = c.support_attribution;
    shape(attribution, ["attributed_support_ids", "withheld"], "support attribution shape");
    demand(strings(attribution.attributed_support_ids) && Array.isArray(attribution.withheld), "support attribution values");
    for (const record of attribution.withheld as unknown[]) {
      shape(record, ["support_id", "reason"], "withheld support shape");
      demand(text(record.support_id) && WITHHELD_REASONS.includes(record.reason), "withheld support values");
    }
    const supports = [...attribution.attributed_support_ids, ...attribution.withheld.map((r: Json) => r.support_id)];
    demand(new Set(supports).size === supports.length, "support attributed or withheld more than once");
    for (const x of c.pipe_stress_extrema as unknown[]) {
      shape(x, EXTREMA_KEYS, "extrema shape");
      demand(text(x.pipe_id) && text(x.result_id) && Object.entries(EXTREMA_CONSTANTS).every(([k, v]) => x[k] === v), "extrema identity or basis");
      // A1 d: exactly the listed checks; no certified-gap bound (finiteness is item 1).
      demand(["value_lower_pa", "value_upper_pa"].every(k => finite(x[k])) && x.value_lower_pa >= 0 && x.value_lower_pa <= x.value_upper_pa, "extrema bounds");
    }
    const pipes: string[] = c.pipe_stress_extrema.map((x: Json) => x.pipe_id);
    demand(new Set(pipes).size === pipes.length && !pipes.some(p => coverage.unavailable_pipe_ids.includes(p) || coverage.outside_domain_pipe_ids.includes(p)), "extrema member partition");
    for (const m of c.intensified_measures as unknown[]) {
      shape(m, INTENSIFIED_KEYS, "intensified measure shape");
      demand(text(m.result_id) && text(m.component_id) && text(m.pipe_id) && ["end_i", "end_j"].includes(m.location) && ["bend", "branch_header", "branch_branch"].includes(m.factor_role) && typeof m.sif_source_reference === "string", "intensified measure identity");
      demand(["sif", "section_modulus_m3", "bending_moment_y_n_m", "bending_moment_z_n_m"].every(k => finite(m[k])) && m.sif > 0 && m.section_modulus_m3 > 0, "intensified measure inputs");
    }
  }
  const supportSets = new Set([...cases.values()].map(c => JSON.stringify([...c.support_attribution.attributed_support_ids, ...c.support_attribution.withheld.map((r: Json) => r.support_id)].sort())));
  demand(supportSets.size <= 1, "support set differs between cases");
  const extremaIds = [...cases.values()].flatMap(c => c.pipe_stress_extrema.map((x: Json) => x.result_id));
  const measureIds = [...cases.values()].flatMap(c => c.intensified_measures.map((m: Json) => m.result_id));
  demand(new Set(extremaIds).size === extremaIds.length && new Set(measureIds).size === measureIds.length, "duplicate evidence result binding");
  return cases;
}

function readGates(evidence: Json): Map<string, Json> {
  const gates = new Map<string, Json>();
  for (const gate of evidence.combination_gates as unknown[]) {
    shape(gate, ["combination_id", "withheld", "reason"], "combination gate shape");
    demand(text(gate.combination_id) && !gates.has(gate.combination_id), "combination gate identity");
    demand(typeof gate.withheld === "boolean" && (gate.withheld ? (PREVIEW_COMBINATION_GATE_CODES as readonly string[]).includes(gate.reason) : gate.reason === null), "combination gate reason");
    gates.set(gate.combination_id, gate);
  }
  return gates;
}

/** Throws `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: …` on the first failed check.
 * The model is accepted for call-site symmetry; the checks are the same with or without it. */
export function validatePreviewPhysicsEvidence(source: MechanicsResult, _model?: Partial<Pick<PreviewModel, "load_cases" | "supports">>): void {
  try { validate(source); }
  catch (error) {
    if (error instanceof Error && error.message.startsWith("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID")) throw error;
    throw new Error("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: malformed evidence");
  }
}

function validate(source: MechanicsResult): void {
  // 1. Header, closed evidence key sets, finite numbers.
  header(source);
  const evidence = source.contract_evidence as Json;
  const cases = readCases(evidence);
  const gates = readGates(evidence);
  const results = source.results, diagnostics = source.diagnostics;
  demand(Array.isArray(results) && Array.isArray(diagnostics), "result collections");
  demand(results.every(r => isObject(r) && text(r.id)) && diagnostics.every(d => isObject(d) && text(d.id)), "evidence identities");
  const rows = new Map(results.map(r => [r.id, r]));
  const diagnosticIds = diagnostics.map(d => d.id as string);
  demand(rows.size === results.length && new Set(diagnosticIds).size === diagnosticIds.length && !diagnosticIds.some(id => rows.has(id)), "duplicate or ambiguous evidence ID");

  // 2. Every row kind has a signature; no retired kind or diagnostic code.
  for (const row of results) {
    demand(finite(row.value) && text(row.kind) && text(row.unit) && text(row.entity_ref), "source row fields");
    demand(row.metadata === undefined || row.metadata === null || isObject(row.metadata), "source row metadata");
    previewPhysicsSignature(row);
    const basis = basisOf(row);
    if (basis !== undefined && basis !== null) {
      shape(basis, ["ref_type", "ref_id"], "row basis reference");
      demand(["load_case", "combination"].includes(basis.ref_type) && text(basis.ref_id), "row basis type");
    }
  }
  for (const item of diagnostics) demand(!PREVIEW_RETIRED_DIAGNOSTIC_CODES.includes(item.code), `retired diagnostic code ${item.code}`);

  // 3. Completeness (F-1): result-namespace references only.
  for (const item of diagnostics) {
    const refs = item.affected_refs ?? [];
    demand(Array.isArray(refs) && refs.every(ref => typeof ref === "string"), "diagnostic reference list");
    for (const ref of refs) demand(!ref.startsWith("result:") || rows.has(ref), `dangling result reference ${ref}`);
  }
  const summary = source.summary as Json;
  demand(isObject(summary), "summary");
  for (const [field, headline] of Object.entries(summary)) {
    if (isObject(headline) && headline.result_ref !== undefined && headline.result_ref !== null) demand(rows.has(headline.result_ref), `dangling summary result reference ${field}`);
  }

  // A1 e: a blocked envelope has empty evidence, no rows and null headlines
  // (items 2-3 above already refused retired codes and result: references).
  if (source.status?.mechanics !== "MECHANICS_SOLVED") {
    demand(cases.size === 0 && gates.size === 0 && results.length === 0
      && (summary.max_open_formula_stress ?? null) === null && (summary.max_displacement ?? null) === null, "blocked envelope carries evidence, rows or headlines");
    return;
  }
  // 4. Case scope.
  const qualityIds = (source.numerical_quality?.cases ?? []).map(c => c?.basis_ref?.ref_id);
  const caseIds = [...cases.keys()];
  demand(caseIds.length === qualityIds.length && caseIds.every((id, i) => id === qualityIds[i]), "preview case coverage");
  for (const row of results) {
    const basis = basisOf(row);
    if (basis === undefined || basis === null) continue;
    if (basis.ref_type === "load_case") demand(cases.has(basis.ref_id) || cases.size === 0, "row case scope");
    else {
      const gate = gates.get(basis.ref_id);
      // 9. Combination rows only for admitted combinations.
      demand(gate !== undefined && gate.withheld === false, "combination row without an admitting gate");
      demand(row.kind !== MAXIMUM_KIND && row.kind !== INTENSIFIED_KIND, "maximum or intensified row for a combination");
      const refs = row.source_result_refs ?? [];
      demand(Array.isArray(refs) && refs.every(ref => typeof ref === "string" && rows.has(ref)), "combination source reference");
    }
  }
  const caseRows = (caseId: string, kind?: string) => results.filter(r => sameCaseBasis(r, caseId) && (kind === undefined || r.kind === kind));

  // 5. Maxima bind to their enclosure; coverage partitions the case members.
  const maximumRows = results.filter(r => r.kind === MAXIMUM_KIND);
  const bound = new Set<string>();
  for (const row of maximumRows) {
    demand(row.unit === "Pa" && sameJson(row.metadata, MAXIMUM_METADATA) && row.basis_ref?.ref_type === "load_case", "maximum row semantics");
    demand(row.id === `result:elastic-maximum:${lengthPrefixedSegments(row.basis_ref!.ref_id, row.entity_ref)}`, "maximum row identity");
  }
  for (const [caseId, c] of cases) {
    const coverage = c.stress_maximum_coverage;
    const members = new Set(caseRows(caseId, "element_local_axial_force").map(r => r.entity_ref));
    const partition = new Set<string>([...c.pipe_stress_extrema.map((x: Json) => x.pipe_id), ...coverage.unavailable_pipe_ids, ...coverage.outside_domain_pipe_ids]);
    demand(partition.size === members.size && [...members].every(m => partition.has(m)), "maximum coverage does not partition the case members");
    for (const x of c.pipe_stress_extrema as Json[]) {
      const row = rows.get(x.result_id);
      demand(row && row.kind === MAXIMUM_KIND && row.entity_ref === x.pipe_id && sameCaseBasis(row, caseId), "extrema result binding");
      const lower = x.value_lower_pa, upper = x.value_upper_pa, value = row.value;
      demand(lower <= value && value <= upper && value === lower + 0.5 * (upper - lower), "maximum outside or off its enclosure");
      bound.add(row.id);
    }
  }
  demand(bound.size === maximumRows.length && maximumRows.every(r => bound.has(r.id)), "maximum row without exactly one enclosure");

  // 6. Headlines over all load cases, with identity ties.
  const governing = (candidates: Row[]) => candidates.reduce((best, row) => {
    if (row.value !== best.value) return row.value > best.value ? row : best;
    const byCase = compareCodePoints(row.basis_ref!.ref_id, best.basis_ref!.ref_id);
    if (byCase !== 0) return byCase < 0 ? row : best;
    return compareCodePoints(row.entity_ref, best.entity_ref) < 0 ? row : best;
  });
  const complete = cases.size > 0 && [...cases.values()].every(c => c.stress_maximum_coverage.complete);
  const stress = summary.max_open_formula_stress ?? null;
  if (complete && maximumRows.length) {
    const selected = governing(maximumRows);
    demand(sameJson(stress, { value: selected.value, unit: "Pa", location_ref: selected.entity_ref, result_ref: selected.id }), "stress headline does not govern");
  } else demand(stress === null, "stress headline while coverage is incomplete or no maximum exists");
  const displacementRows = results.filter(r => r.kind === "displacement_magnitude" && r.basis_ref?.ref_type === "load_case");
  const displacement = summary.max_displacement ?? null;
  if (displacementRows.length) {
    const selected = governing(displacementRows);
    demand(sameJson(displacement, { value: selected.value, unit: "mm", location_ref: selected.entity_ref, result_ref: selected.id }), "displacement headline does not govern");
  } else demand(displacement === null, "displacement headline without displacement rows");
  const intensifiedRows = results.filter(r => r.kind === INTENSIFIED_KIND);
  if (Object.hasOwn(summary, "component_stress_modifier_count")) demand(summary.component_stress_modifier_count === intensifiedRows.length, "intensified row count");

  // 7. Support actions: attributed => exactly eight rows; withheld => none.
  const withheldDiagnostics = new Set<string>();
  for (const item of diagnostics) {
    if (!WITHHELD_REASONS.includes(item.code)) continue;
    const support = item.affected_refs?.[0];
    const expectedId = `diagnostic:preview-physics:${item.code === "SUPPORT_ACTION_ATTRIBUTION_WITHHELD" ? "attribution" : "constant-effort-not-consumed"}:${text(support) ? lengthPrefixed(support) : ""}`;
    demand(text(support) && item.id === expectedId, "withheld diagnostic identity");
    const key = JSON.stringify([support, item.code]);
    demand(!withheldDiagnostics.has(key), "duplicate withheld diagnostic");
    withheldDiagnostics.add(key);
  }
  for (const row of results) {
    if (SUPPORT_KINDS.includes(row.kind) && row.basis_ref?.ref_type === "load_case") demand(cases.has(row.basis_ref.ref_id), "support action outside a preview case");
  }
  for (const [caseId, c] of cases) {
    const attribution = c.support_attribution;
    const grouped = new Map<string, Map<string, Row>>();
    for (const row of caseRows(caseId)) {
      if (!SUPPORT_KINDS.includes(row.kind)) continue;
      const component = row.metadata?.component as string;
      const slots = grouped.get(row.entity_ref) ?? new Map<string, Row>();
      grouped.set(row.entity_ref, slots);
      demand(!slots.has(component), "duplicate support action component");
      slots.set(component, row);
    }
    const attributed: string[] = attribution.attributed_support_ids;
    demand(grouped.size === attributed.length && attributed.every(id => grouped.has(id)), "support action rows do not match attribution");
    for (const [support, slots] of grouped) {
      const names = Object.keys(SUPPORT_ROW_KINDS);
      demand(slots.size === names.length && names.every(n => slots.has(n)), "support action component coverage");
      for (const [component, row] of slots) {
        const [kind, unit] = SUPPORT_ROW_KINDS[component];
        demand(row.kind === kind && row.unit === unit && row.id === `result:support-action:${lengthPrefixedSegments(caseId, support)}:${component}`, "support action identity");
        demand(sameJson(row.metadata, { component, coordinate_system: "global", location: "node", basis: "recovered_from_assembled_support_law", sign_convention: SUPPORT_SIGN }), "support action semantics");
      }
      demand(consistentNorm(slots.get("force_magnitude")!.value, SUPPORT_COMPONENTS.slice(0, 3).map(n => slots.get(n)!.value)), "support force magnitude inconsistent with components");
      demand(consistentNorm(slots.get("moment_magnitude")!.value, SUPPORT_COMPONENTS.slice(3).map(n => slots.get(n)!.value)), "support moment magnitude inconsistent with components");
    }
    // A1 b: withheld records <-> withheld diagnostics, both directions (support = affected_refs[0], reason = code, §8 ids).
    const recordKeys = new Set<string>(attribution.withheld.map((r: Json) => JSON.stringify([r.support_id, r.reason])));
    demand(recordKeys.size === withheldDiagnostics.size && [...recordKeys].every(k => withheldDiagnostics.has(k)), "withheld records do not match withheld diagnostics");
    const listed = new Set<string>([...attributed, ...attribution.withheld.map((r: Json) => r.support_id)]);
    demand(caseRows(caseId).filter(r => PER_SUPPORT_NONLINEAR_KINDS.includes(r.kind)).every(r => listed.has(r.entity_ref)), "per-support nonlinear row names an unlisted support");
    const attributionWithheld = new Set<string>(attribution.withheld.filter((r: Json) => r.reason === "SUPPORT_ACTION_ATTRIBUTION_WITHHELD").map((r: Json) => r.support_id));
    demand(!caseRows(caseId, "nonlinear_support_final_reaction").some(r => attributionWithheld.has(r.entity_ref)), "withheld support still publishes a final reaction");
  }

  // 8. Intensified rows <=> intensified_measures.
  const measured = new Set<string>();
  for (const [caseId, c] of cases) {
    for (const m of c.intensified_measures as Json[]) {
      const row = rows.get(m.result_id);
      demand(row && row.kind === INTENSIFIED_KIND && row.unit === "Pa" && sameCaseBasis(row, caseId) && row.entity_ref === m.component_id, "intensified result binding");
      const md = row.metadata as Json | undefined;
      demand(isObject(md) && Object.keys(md).length === 5 && md.component === "equal_factor_intensified_bending_stress" && md.coordinate_system === "pipe_section" && md.location === m.location
        && md.basis === "user_sif_times_member_section_bending_stress_v1" && typeof md.sign_convention === "string" && md.sign_convention.startsWith(INTENSIFIED_SIGN_PREFIX), "intensified row semantics");
      const expected = m.sif * (Math.hypot(m.bending_moment_y_n_m, m.bending_moment_z_n_m) / m.section_modulus_m3);
      demand(row.value >= 0 && Math.abs(row.value - expected) <= GUARD * Math.max(Math.abs(row.value), TINY), "intensified value inconsistent with its inputs");
      const refs = row.source_result_refs;
      demand(Array.isArray(refs) && refs.length === 2 && refs.every(ref => typeof ref === "string" && rows.has(ref)), "intensified source reference");
      const sources = refs.map(ref => rows.get(ref)!);
      const kinds = new Set(sources.map(s => s.kind));
      demand(kinds.size === 2 && INTENSIFIED_SOURCE_KINDS.every(k => kinds.has(k))
        && sources.every(s => sameJson(s.basis_ref, row.basis_ref) && s.entity_ref === m.pipe_id && s.metadata?.location === m.location), "intensified source rows are not the same-case member end");
      measured.add(row.id);
    }
  }
  demand(measured.size === intensifiedRows.length && intensifiedRows.every(r => measured.has(r.id)), "intensified row without exactly one measure");

  // 9. Admitted mechanics/subtraction magnitudes follow their components.
  const combinationRows = new Map<string, Row[]>();
  for (const row of results) {
    if (row.basis_ref?.ref_type === "combination") {
      const list = combinationRows.get(row.basis_ref.ref_id) ?? [];
      list.push(row); combinationRows.set(row.basis_ref.ref_id, list);
    }
  }
  for (const members of combinationRows.values()) {
    const bases = new Set(members.map(r => r.metadata?.basis).filter((b): b is string => typeof b === "string"));
    if (bases.has(RANGE_BASIS) || !LINEAR_BASES.some(b => bases.has(b))) continue;
    const nodal = new Map(members.filter(r => r.kind.startsWith("global_nodal_displacement_")).map(r => [JSON.stringify([r.entity_ref, r.kind]), r]));
    const supports = new Map<string, Row>();
    for (const row of members) {
      if (!SUPPORT_KINDS.includes(row.kind)) continue;
      const key = JSON.stringify([row.entity_ref, row.metadata?.component]);
      demand(!supports.has(key), "duplicate combined support component");
      supports.set(key, row);
    }
    for (const row of members) {
      if (row.kind === "displacement_magnitude") {
        const parts = ["x", "y", "z"].map(axis => nodal.get(JSON.stringify([row.entity_ref, `global_nodal_displacement_${axis}`])));
        demand(parts.every(Boolean) && consistentNorm(row.value, parts.map(p => p!.value)), "combined displacement magnitude inconsistent with components");
      } else if (row.kind === SUPPORT_FORCE_KIND || row.kind === SUPPORT_MOMENT_KIND) {
        const names = row.kind === SUPPORT_FORCE_KIND ? SUPPORT_COMPONENTS.slice(0, 3) : SUPPORT_COMPONENTS.slice(3);
        const parts = names.map(n => supports.get(JSON.stringify([row.entity_ref, n])));
        demand(parts.every(Boolean) && consistentNorm(row.value, parts.map(p => p!.value)), "combined support magnitude inconsistent with components");
      }
    }
  }
}

/** Closed statement shape and internal consistency only; creates no row evidence.
 * A retained package carries the statement without the raw rows, so the row joins
 * of `validatePreviewPhysicsEvidence` are checked only when the supplied raw
 * envelope is validated separately (same split as the Python reader). */
export function validatePreviewPhysicsTransportMetadata(source: Pick<MechanicsResult, "formulation_basis" | "contract_evidence">): void {
  try {
    demand(!Object.hasOwn(source, "source_block_recovery") && !Object.hasOwn(source, "carrier_evidence"), "unsupported source namespace");
    const f = source.formulation_basis;
    shape(f, ["profile_id", "limitations"], "formulation basis");
    demand(f.profile_id === PROFILE && Array.isArray(f.limitations) && f.limitations.length === LIMITATIONS.length && f.limitations.every((l: unknown, i: number) => l === LIMITATIONS[i]), "formulation profile or limitations");
    const evidence = source.contract_evidence;
    shape(evidence, ["preview_cases", "combination_gates"], "transport evidence shape");
    demand(Array.isArray(evidence.preview_cases) && Array.isArray(evidence.combination_gates), "transport evidence shape");
    finiteTree(evidence);
    readCases(evidence);
    readGates(evidence);
  } catch (error) {
    if (error instanceof Error && error.message.startsWith("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID")) throw error;
    throw new Error("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: malformed evidence");
  }
}
