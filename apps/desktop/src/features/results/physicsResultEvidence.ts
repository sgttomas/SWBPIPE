import type { MechanicsResult, PreviewModel } from "../../types";

const PROFILE = "exact_straight_pressure_v2";
type RecordValue = Record<string, any>;
function demand(ok: unknown, reason: string): asserts ok {
  if (!ok) throw new Error(`PHYSICS_EVIDENCE_${reason}`);
}
function shape(value: unknown, fields: string[], reason: string): asserts value is RecordValue {
  demand(value && typeof value === "object" && !Array.isArray(value)
    && Object.keys(value).length === fields.length && fields.every(key => Object.hasOwn(value, key)), reason);
}
const text = (v: unknown): v is string => typeof v === "string" && v.length > 0;
const finite = (v: unknown): v is number => typeof v === "number" && Number.isFinite(v);
function finiteTree(v: unknown, seen = new Set<object>()): boolean {
  if (v === null || typeof v === "string" || typeof v === "boolean") return true;
  if (typeof v === "number") return Number.isFinite(v);
  if (!v || typeof v !== "object" || seen.has(v)) return false;
  seen.add(v); const valid = Object.values(v).every(x => finiteTree(x, seen)); seen.delete(v); return valid;
}
function unique(values: unknown): values is string[] {
  return Array.isArray(values) && values.every(text) && new Set(values).size === values.length;
}
const sameSet = (a: string[], b: string[]) => a.length === b.length && a.every(x => b.includes(x));
function binary64Word(value: number): bigint {
  const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, value, false);
  return view.getBigUint64(0, false);
}
function binary64Bits(value: number): string { return binary64Word(value).toString(16).padStart(16, "0"); }
function coherentShearModulus(actual: number, expected: number): boolean {
  if (!(Number.isFinite(expected) && expected > 0)) return false;
  const delta = binary64Word(actual) - binary64Word(expected);
  return delta >= -2n && delta <= 2n;
}
function sourceBoreBits(section: RecordValue): [string, string] {
  const a = section.outside_diameter_m * 0.5, b = section.effective_wall_thickness_m;
  const x = a - b, bv = a - x, av = x + bv, low = (a - av) + (bv - b);
  return [binary64Bits(x), binary64Bits(low === 0 ? 0 : low)];
}
const vector = (v: unknown, n: number) => Array.isArray(v) && v.length === n && v.every(finite);
const equalFields = (a: RecordValue, b: RecordValue, fields: string[]) => fields.every(k => a[k] === b[k]);
const materialFields = ["pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "thermal_consumed", "alpha_per_kelvin", "provenance"];
const sectionFields = ["pipe_id", "geometry_basis", "outside_diameter_m", "effective_wall_thickness_m", "ri_m", "ro_m", "Ai_m2", "As_m2", "I_m4", "J_m4", "Z_m3"];
const caseFields = ["load_case_id", "profile_mode", "material_basis", "pipe_materials", "pipe_sections", "pipe_stress_extrema", "stress_maximum_coverage", "pressure_rhs_assembly"];
const pressureFields = ["load_case_id", "region_id", "profile_version", "profile_mode", "pressure_basis", "member_pipe_ids", "p_pa", "external_pressure_increment_pa", "geometry", "materials", "applied_loads", "terminals", "provenance", "approximation", "geometry_representation_guard", "result_ids"];
const physicalKinds: Record<string, [string[], string, string, string, string[]]> = {
  pipe_wall_endpoint_action_v2: [["wall_axial_end_action"], "N", "element_local", "recovered_from_local_element_stiffness", ["end_i", "end_j"]],
  pipe_wall_axial_force_v2: [["wall_axial_force"], "N", "element_local", "recovered_from_local_element_stiffness", ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"]],
  pipe_effective_axial_force_v2: [["effective_axial_force"], "N", "element_local", "recovered_from_local_element_stiffness", ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"]],
  pipe_axial_membrane_stress_v2: [["axial_membrane_stress"], "Pa", "pipe_section", "recovered_from_open_mechanics_stress_components", ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"]],
  pipe_lame_radial_stress_v2: [["lame_inner_radial_stress", "lame_outer_radial_stress"], "Pa", "pipe_section", "recovered_from_open_mechanics_stress_components", ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"]],
  pipe_lame_hoop_stress_v2: [["lame_inner_hoop_stress", "lame_outer_hoop_stress"], "Pa", "pipe_section", "recovered_from_open_mechanics_stress_components", ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"]],
  pipe_elastic_normal_stress_maximum_v2: [["maximum_absolute_normal_stress"], "Pa", "pipe_section", "recovered_from_open_mechanics_stress_components", ["governing_station"]],
  support_reaction_component_v2: [["Fx", "Fy", "Fz", "Mx", "My", "Mz"], "", "global", "recovered_from_assembled_support_law", ["node"]],
  support_reaction_force_magnitude_v2: [["force_magnitude"], "N", "global", "recovered_from_assembled_support_law", ["node"]],
  support_reaction_moment_magnitude_v2: [["moment_magnitude"], "N*m", "global", "recovered_from_assembled_support_law", ["node"]],
};
const physicalSigns: Record<string, string> = {
  "support_reaction_component_v2": "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate",
  "support_reaction_force_magnitude_v2": "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate",
  "support_reaction_moment_magnitude_v2": "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate",
  "pipe_elastic_normal_stress_maximum_v2": "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim",
  "pipe_wall_endpoint_action_v2": "node-on-element wall action, positive along authored local x toward end j; cap transfer is not subtracted from wall recovery",
  "pipe_wall_axial_force_v2": "tension-positive material wall section resultant Nw",
  "pipe_effective_axial_force_v2": "effective wall-fluid resultant S=Nw-pAi; not material stress or a support reaction",
  "pipe_axial_membrane_stress_v2": "tension-positive axial wall membrane stress Nw/As; no added longitudinal pressure scalar",
  "pipe_lame_radial_stress_v2": "tension-positive radial stress at named surface, inner traction -p and zero external pressure increment",
  "pipe_lame_hoop_stress_v2": "tension-positive circumferential stress at named surface for long straight annulus, zero external pressure increment"
};
function material(m: unknown, region = false): asserts m is RecordValue {
  shape(m, region ? [...materialFields, "temperature_basis"] : materialFields, "MATERIAL_SHAPE");
  demand(text(m.pipe_id) && text(m.material_id) && text(m.provenance) && m.constitutive_basis === "homogeneous_isotropic_E_nu_v1"
    && finite(m.E_pa) && m.E_pa > 0 && finite(m.nu) && m.nu > -1 && m.nu < 0.5 && finite(m.G_pa) && m.G_pa > 0
    && coherentShearModulus(m.G_pa, m.E_pa / (2 * (1 + m.nu)))
    && typeof m.thermal_consumed === "boolean" && (m.thermal_consumed ? finite(m.alpha_per_kelvin) : m.alpha_per_kelvin === null), "MATERIAL_INVALID");
  if (region) {
    const t = m.temperature_basis;
    if (t?.selection === "base_material") shape(t, ["selection"], "TEMPERATURE_BASIS");
    else if (t?.selection === "exact_point") { shape(t, ["selection", "point_id"], "TEMPERATURE_BASIS"); demand(text(t.point_id), "TEMPERATURE_BASIS"); }
    else { shape(t, ["selection", "temperature_value", "temperature_unit"], "TEMPERATURE_BASIS"); demand(t.selection === "interpolation" && finite(t.temperature_value) && ["K", "degC", "degF"].includes(t.temperature_unit), "TEMPERATURE_BASIS"); }
  }
}
function section(s: unknown, region = false): asserts s is RecordValue {
  shape(s, region ? [...sectionFields, "traversal_forward"] : sectionFields, "SECTION_SHAPE");
  demand(text(s.pipe_id) && s.geometry_basis === "authored_normalized_od_wall_v1"
    && sectionFields.slice(2).every(k => finite(s[k]) && s[k] > 0)
    && s.effective_wall_thickness_m < s.outside_diameter_m / 2 && s.ri_m < s.ro_m
    && s.ro_m === s.outside_diameter_m * 0.5 && s.ri_m === s.outside_diameter_m * 0.5 - s.effective_wall_thickness_m
    && (!region || typeof s.traversal_forward === "boolean"), "SECTION_INVALID");
}

/** Validate the closed supported physical evidence and its actual source bindings.
 * This never recomputes membrane/effective actions from rounded published rows,
 * authenticates a producer, or substitutes for the numerical quality gate. */
export function validatePhysicsEvidence(source: MechanicsResult, model?: Pick<PreviewModel, "load_cases"> & Partial<Pick<PreviewModel, "pipe_segments" | "supports">>): void {
  demand(!Object.hasOwn(source, "source_block_recovery") && !Object.hasOwn(source, "carrier_evidence"), "UNSUPPORTED_SOURCE_NAMESPACE");
  const evidence = source.contract_evidence;
  shape(evidence, ["pressure", "connector", "exact_cases"], "SHAPE");
  demand(finiteTree(evidence) && source.results.every(row => finite(row.value)), "NONFINITE");
  demand(Array.isArray(evidence.pressure) && Array.isArray(evidence.exact_cases) && Array.isArray(evidence.connector) && evidence.connector.length === 0, "UNSUPPORTED_COMPOSITION");
  const cases = evidence.exact_cases as RecordValue[], regions = evidence.pressure as RecordValue[];
  for (const c of cases) {
    shape(c, caseFields, "CASE_SHAPE");
    demand(Array.isArray(c.pipe_materials) && Array.isArray(c.pipe_sections) && Array.isArray(c.pipe_stress_extrema), "CASE_INVALID");
  }
  demand(unique(cases.map(c => c.load_case_id)), "CASE_ID_AMBIGUOUS");
  const caseIds = cases.map(c => c.load_case_id);
  const numericalCases = source.numerical_quality?.cases ?? [];
  demand(numericalCases.every(c => c.basis_ref.ref_type === "load_case") && unique(numericalCases.map(c => c.basis_ref.ref_id)), "NUMERICAL_CASE_SCOPE");
  if (source.status.mechanics === "MECHANICS_SOLVED") demand(sameSet(caseIds, numericalCases.map(c => c.basis_ref.ref_id)) && cases.length > 0, "CASE_COVERAGE");
  else demand(caseIds.every(id => numericalCases.some(c => c.basis_ref.ref_id === id)), "CASE_COVERAGE");
  if (model && source.status.mechanics === "MECHANICS_SOLVED") demand(sameSet(caseIds, model.load_cases.map(c => c.id)), "MODEL_CASE_COVERAGE");
  demand(unique(source.results.map(row => row.id)), "RESULT_ID_AMBIGUOUS");
  const rows = new Map(source.results.map(row => [row.id, row]));
  demand(unique([...source.results.map(row => row.id), ...source.diagnostics.map(d => d.id)]), "NUMERICAL_EVIDENCE_ID_AMBIGUOUS");
  for (const c of numericalCases) {
    demand(unique(c.evidence_refs), "NUMERICAL_EVIDENCE_ID_AMBIGUOUS");
    for (const id of c.evidence_refs) {
      const row = rows.get(id), diagnostic = source.diagnostics.find(d => d.id === id);
      demand(row ? row.basis_ref?.ref_type === "load_case" && row.basis_ref.ref_id === c.basis_ref.ref_id
        : diagnostic?.affected_refs?.includes(c.basis_ref.ref_id), "NUMERICAL_EVIDENCE_CASE_SCOPE");
    }
  }
  const regionMembers = new Set<string>(), boundRows = new Set<string>(), regionKeys = new Set<string>();
  for (const r of regions) {
    shape(r, pressureFields, "PRESSURE_SHAPE");
    demand(text(r.region_id) && caseIds.includes(r.load_case_id) && r.profile_version === "2.0.0" && r.profile_mode === PROFILE
      && r.pressure_basis === "internal_differential_zero_external_v1" && finite(r.p_pa) && r.p_pa >= 0 && r.external_pressure_increment_pa === 0
      && r.approximation === "long_straight_annulus_small_strain_v2" && text(r.provenance)
      && unique(r.member_pipe_ids) && r.member_pipe_ids.length > 0 && unique(r.result_ids), "PRESSURE_INVALID");
    const key = JSON.stringify([r.load_case_id, r.region_id]); demand(!regionKeys.has(key), "REGION_ID_AMBIGUOUS"); regionKeys.add(key);
    shape(r.geometry_representation_guard, ["epsilon_multiplier", "meaning"], "GEOMETRY_GUARD");
    demand(r.geometry_representation_guard.epsilon_multiplier === 64 && r.geometry_representation_guard.meaning === "arithmetic_representation_only", "GEOMETRY_GUARD");
    const c = cases.find(c => c.load_case_id === r.load_case_id)!;
    for (const [field, validate] of [["geometry", section], ["materials", material]] as const) {
      demand(Array.isArray(r[field]), "MEMBER_SCOPE");
      r[field].forEach((v: unknown) => validate(v, true));
      demand(unique(r[field].map((v: RecordValue) => v.pipe_id)) && sameSet(r.member_pipe_ids, r[field].map((v: RecordValue) => v.pipe_id)), "MEMBER_SCOPE");
    }
    for (const m of r.materials) {
      const temperature = m.temperature_basis;
      if (temperature.selection === "base_material") demand(c.material_basis === "base_material_common_E_nu", "TEMPERATURE_CASE_BINDING");
      else if (temperature.selection === "exact_point") demand(c.material_basis.includes(`material=${m.material_id};common_E_nu_basis=point:${temperature.point_id};G=E/[2(1+nu)];alpha_same_basis=`), "TEMPERATURE_CASE_BINDING");
      else demand(c.material_basis.includes(`material=${m.material_id};common_E_nu_basis=interpolated:`) && c.material_basis.includes(";temperature_kelvin="), "TEMPERATURE_CASE_BINDING");
    }
    for (const id of r.member_pipe_ids) {
      const mk = JSON.stringify([r.load_case_id, id]); demand(!regionMembers.has(mk), "MEMBER_REGION_AMBIGUOUS"); regionMembers.add(mk);
      const cs = c.pipe_sections?.find((s: RecordValue) => s.pipe_id === id), cm = c.pipe_materials?.find((m: RecordValue) => m.pipe_id === id);
      demand(cs && cm && equalFields(cs, r.geometry.find((s: RecordValue) => s.pipe_id === id), sectionFields)
        && equalFields(cm, r.materials.find((m: RecordValue) => m.pipe_id === id), materialFields), "CASE_REGION_CONTRADICTION");
    }
    demand(Array.isArray(r.applied_loads) && unique(r.applied_loads.map((l: RecordValue) => l.pipe_id)) && sameSet(r.member_pipe_ids, r.applied_loads.map((l: RecordValue) => l.pipe_id)), "APPLIED_LOADS");
    for (const l of r.applied_loads) {
      shape(l, ["pipe_id", "eigenload_pair_local_n", "local_x_global", "mathematical_cap_pair_local_n", "thermal_included"], "APPLIED_LOADS");
      demand(vector(l.eigenload_pair_local_n, 2) && vector(l.local_x_global, 3) && vector(l.mathematical_cap_pair_local_n, 2)
        // This ledger carries only pressure cap/Poisson loads. Thermal mechanics
        // consume the selected material separately, even in a mixed load case.
        && l.thermal_included === false, "APPLIED_LOADS");
    }
    demand(Array.isArray(r.terminals) && r.terminals.length === 2 && unique(r.terminals.map((t: RecordValue) => t.node_ref)), "TERMINALS");
    for (const t of r.terminals) {
      shape(t, ["node_ref", "closure_transfer", "provenance", "closure_pressure_load_global_n", "pipe_cap_transfer_global_n", "remote_closure_excluded_from_pipe_solve", "remote_closure_support_reaction_global_n"], "TERMINAL_SHAPE");
      demand(text(t.provenance) && vector(t.closure_pressure_load_global_n, 3) && vector(t.pipe_cap_transfer_global_n, 3), "TERMINALS");
      if (t.closure_transfer === "transfers_to_wall") demand(t.remote_closure_excluded_from_pipe_solve === false && t.remote_closure_support_reaction_global_n === null && t.closure_pressure_load_global_n.every((v: number, i: number) => v === t.pipe_cap_transfer_global_n[i]), "TERMINALS");
      else demand(t.closure_transfer === "separately_supported_or_compensated" && t.remote_closure_excluded_from_pipe_solve === true && vector(t.remote_closure_support_reaction_global_n, 3) && t.pipe_cap_transfer_global_n.every((v: number) => v === 0) && t.closure_pressure_load_global_n.every((v: number, i: number) => -v === t.remote_closure_support_reaction_global_n[i]), "TERMINALS");
    }
    for (const id of r.result_ids) {
      const row = rows.get(id); demand(row && !boundRows.has(id) && row.kind.startsWith("pipe_") && Object.hasOwn(physicalKinds, row.kind)
        && row.basis_ref?.ref_type === "load_case" && row.basis_ref.ref_id === r.load_case_id && r.member_pipe_ids.includes(row.entity_ref), "REGION_RESULT_BINDING"); boundRows.add(id);
    }
  }
  for (const c of cases) {
    demand(c.profile_mode === PROFILE && text(c.material_basis) && Array.isArray(c.pipe_materials) && Array.isArray(c.pipe_sections) && Array.isArray(c.pipe_stress_extrema), "CASE_INVALID");
    c.pipe_materials.forEach((m: unknown) => material(m)); c.pipe_sections.forEach((s: unknown) => section(s));
    const members = c.pipe_materials.map((m: RecordValue) => m.pipe_id);
    demand(unique(members) && members.length > 0 && unique(c.pipe_sections.map((s: RecordValue) => s.pipe_id)) && sameSet(members, c.pipe_sections.map((s: RecordValue) => s.pipe_id)), "CASE_MEMBER_SCOPE");
    demand(sameSet(members, cases[0].pipe_materials.map((m: RecordValue) => m.pipe_id)), "CASE_MEMBER_COVERAGE");
    if (model?.pipe_segments) demand(sameSet(members, model.pipe_segments.map(p => p.id)), "MODEL_MEMBER_COVERAGE");
    shape(c.stress_maximum_coverage, ["complete", "unavailable_pipe_ids"], "EXTREMA_COVERAGE");
    const unavailable = c.stress_maximum_coverage.unavailable_pipe_ids;
    demand(unique(unavailable) && unavailable.every(id => members.includes(id)) && c.stress_maximum_coverage.complete === (unavailable.length === 0)
      && unique(c.pipe_stress_extrema.map((x: RecordValue) => x.pipe_id)) && sameSet(c.pipe_stress_extrema.map((x: RecordValue) => x.pipe_id), members.filter((id: string) => !unavailable.includes(id))), "EXTREMA_COVERAGE");
    for (const x of c.pipe_stress_extrema) {
      shape(x, ["pipe_id", "result_id", "approximation", "station_fraction", "span_index", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa", "subdivisions", "coefficient_basis", "enclosure_scope"], "EXTREMA_SHAPE");
      const row = rows.get(x.result_id);
      demand(row && row.kind === "pipe_elastic_normal_stress_maximum_v2" && row.entity_ref === x.pipe_id && row.basis_ref?.ref_id === c.load_case_id
        && x.approximation === "piecewise_quadratic_straight_section_statics" && x.coefficient_basis === "j_side_section_equilibrium_binary64"
        && x.enclosure_scope === "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate"
        && ["station_fraction", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa"].every(k => finite(x[k]))
        && x.station_fraction >= 0 && x.station_fraction <= 1 && x.local_fraction >= 0 && x.local_fraction <= 1
        && Number.isSafeInteger(x.span_index) && x.span_index >= 0 && Number.isSafeInteger(x.subdivisions) && x.subdivisions >= 0 && x.subdivisions <= 131072
        && x.value_lower_pa >= 0 && x.value_upper_pa >= x.value_lower_pa && x.global_upper_bound_pa >= x.value_upper_pa
        && x.certified_gap_pa >= 0 && x.certified_gap_pa <= 1e-12 + 1e-12 * x.value_lower_pa
        && row.value === x.value_lower_pa + 0.5 * (x.value_upper_pa - x.value_lower_pa)
        && x.global_upper_bound_pa - x.value_lower_pa <= x.certified_gap_pa, "EXTREMA_BINDING");
    }
    validateRhs(c.pressure_rhs_assembly, c.load_case_id, regions);
  }
  const physicalSignatures = new Set<string>();
  for (const row of source.results) {
    shape(row.basis_ref, ["ref_type", "ref_id"], "ROW_CASE_REFERENCE");
    demand(row.basis_ref.ref_type === "load_case" && caseIds.includes(row.basis_ref.ref_id)
      && (!Object.hasOwn(row, "source_result_refs") || (Array.isArray(row.source_result_refs) && row.source_result_refs.length === 0)), "UNSUPPORTED_DERIVATION");
    demand(text(row.kind) && text(row.entity_ref) && text(row.unit), "ROW_IDENTITY");
    demand(!row.kind.endsWith("_v2") || Object.hasOwn(physicalKinds, row.kind), "UNKNOWN_PHYSICAL_KIND");
    const rule = Object.hasOwn(physicalKinds, row.kind) ? physicalKinds[row.kind] : undefined; if (!rule) continue;
    const md = row.metadata;
    shape(md, ["component", "coordinate_system", "location", "basis", "sign_convention"], "ROW_METADATA_SHAPE");
    const unit = row.kind === "support_reaction_component_v2" ? (md?.component.startsWith("M") ? "N*m" : "N") : rule[1];
    demand(md && rule[0].includes(md.component) && row.unit === unit && md.coordinate_system === rule[2] && md.basis === rule[3] && rule[4].includes(md.location) && md.sign_convention === physicalSigns[row.kind], "ROW_SEMANTICS");
    if (row.kind.endsWith("_magnitude_v2")) demand(row.value >= 0, "ROW_SEMANTICS");
    const signature = JSON.stringify([row.basis_ref.ref_id, row.entity_ref, row.kind, md.component, md.location]);
    demand(!physicalSignatures.has(signature), "ROW_SEMANTIC_DUPLICATE"); physicalSignatures.add(signature);
    const c = cases.find(c => c.load_case_id === row.basis_ref!.ref_id)!;
    if (row.kind.startsWith("pipe_")) {
      demand(c.pipe_materials.some((m: RecordValue) => m.pipe_id === row.entity_ref), "ROW_MEMBER_BINDING");
      const pressurized = regionMembers.has(JSON.stringify([c.load_case_id, row.entity_ref]));
      demand(pressurized === boundRows.has(row.id), "REGION_RESULT_COVERAGE");
      if (row.kind === "pipe_elastic_normal_stress_maximum_v2") demand(c.pipe_stress_extrema.some((x: RecordValue) => x.result_id === row.id), "EXTREMA_BINDING");
      else demand(pressurized, "UNPRESSURIZED_PRESSURE_ROW");
    } else if (model?.supports) demand(model.supports.some(s => s.id === row.entity_ref), "SUPPORT_BINDING");
  }
  for (const region of regions) for (const member of region.member_pipe_ids) {
    for (const [kind, [components, , , , locations]] of Object.entries(physicalKinds)) {
      if (!kind.startsWith("pipe_") || kind === "pipe_elastic_normal_stress_maximum_v2") continue;
      for (const component of components) for (const location of locations)
        demand(physicalSignatures.has(JSON.stringify([region.load_case_id, member, kind, component, location])), "PRESSURE_ROW_COVERAGE");
    }
  }
  for (const row of source.results.filter(r => r.kind.startsWith("support_reaction_") && r.kind.endsWith("_v2"))) {
    for (const [kind, [components, , , , locations]] of Object.entries(physicalKinds)) {
      if (!kind.startsWith("support_reaction_")) continue;
      for (const component of components) for (const location of locations)
        demand(physicalSignatures.has(JSON.stringify([row.basis_ref!.ref_id, row.entity_ref, kind, component, location])), "SUPPORT_ROW_COVERAGE");
    }
  }
  const headline = source.summary.max_open_formula_stress;
  if (cases.some(c => !c.stress_maximum_coverage.complete)) demand(!headline, "INCOMPLETE_MAXIMUM_HEADLINE");
  if (headline) {
    const row = rows.get(headline.result_ref);
    demand(row && row.kind === "pipe_elastic_normal_stress_maximum_v2" && row.entity_ref === headline.location_ref && row.value === headline.value && row.unit === headline.unit
      && source.results.filter(r => r.kind === row.kind).every(r => r.value <= row.value), "MAXIMUM_HEADLINE_BINDING");
  }
}

function validateRhs(rhs: unknown, caseId: string, regions: RecordValue[]): void {
  shape(rhs, ["method", "load_case_id", "node_order", "dof_order", "dof_units", "assembled_pressure_rhs_global", "groups", "rounded_cap_rhs_global", "rounded_poisson_rhs_global", "rounded_cap_and_eigen_ledgers_are_observational", "cancellation_screen", "screen_limit", "screen_roundoff_multiplier", "screen_is_not_numerical_qualification"], "RHS_SHAPE");
  demand(rhs.method === "source_factor_grouped_pressure_rhs_v1" && rhs.load_case_id === caseId && unique(rhs.node_order) && rhs.node_order.length > 0
    && JSON.stringify(rhs.dof_order) === JSON.stringify(["Fx", "Fy", "Fz", "Mx", "My", "Mz"])
    && JSON.stringify(rhs.dof_units) === JSON.stringify(["N", "N", "N", "N*m", "N*m", "N*m"])
    && [rhs.assembled_pressure_rhs_global, rhs.rounded_cap_rhs_global, rhs.rounded_poisson_rhs_global].every(v => vector(v, 6 * rhs.node_order.length))
    && rhs.rounded_cap_and_eigen_ledgers_are_observational === true && rhs.screen_is_not_numerical_qualification === true
    && rhs.screen_limit === 1e-9 && rhs.screen_roundoff_multiplier === 32 && finite(rhs.cancellation_screen) && rhs.cancellation_screen >= 0 && rhs.cancellation_screen <= rhs.screen_limit && Array.isArray(rhs.groups), "RHS_INVALID");
  for (const values of [rhs.assembled_pressure_rhs_global, rhs.rounded_cap_rhs_global, rhs.rounded_poisson_rhs_global])
    demand(values.every((value: number, index: number) => index % 6 < 3 || value === 0), "RHS_PRESSURE_MOMENT");
  const groups = new Set<string>();
  for (const group of rhs.groups) {
    shape(group, ["node_ref", "component", "pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits", "direction_component_magnitude", "coefficient_sum", "assembled_force_n", "terms"], "RHS_GROUP");
    demand(rhs.node_order.includes(group.node_ref) && ["Fx", "Fy", "Fz"].includes(group.component)
      && [group.pressure_bits, group.source_inner_radius_hi_bits, group.source_inner_radius_lo_bits].every(s => typeof s === "string" && /^[0-9a-f]{16}$/.test(s))
      && finite(group.direction_component_magnitude) && group.direction_component_magnitude > 0 && group.direction_component_magnitude <= 1
      && finite(group.coefficient_sum) && finite(group.assembled_force_n) && Array.isArray(group.terms) && group.terms.length > 0, "RHS_GROUP");
    const identity = JSON.stringify([group.node_ref, group.component, group.pressure_bits, group.source_inner_radius_hi_bits, group.source_inner_radius_lo_bits, group.direction_component_magnitude]);
    demand(!groups.has(identity), "RHS_GROUP_DUPLICATE"); groups.add(identity);
    for (const term of group.terms) {
      shape(term, ["coefficient", "region_id", "pipe_id", "kind"], "RHS_TERM");
      const region = regions.find(r => r.load_case_id === caseId && r.region_id === term.region_id);
      const geometry = region?.geometry.find((g: RecordValue) => g.pipe_id === term.pipe_id);
      const bore = geometry ? sourceBoreBits(geometry) : null;
      demand(region && region.member_pipe_ids.includes(term.pipe_id) && group.pressure_bits === binary64Bits(region.p_pa)
        && bore && group.source_inner_radius_hi_bits === bore[0] && group.source_inner_radius_lo_bits === bore[1] && finite(term.coefficient)
        && ["poisson_eigen", "terminal_cap"].includes(term.kind), "RHS_TERM");
      const material = region.materials.find((m: RecordValue) => m.pipe_id === term.pipe_id);
      demand(material && Math.abs(term.coefficient) === (term.kind === "terminal_cap" ? 1 : Math.abs(2 * material.nu)), "RHS_TERM_COEFFICIENT");
    }
  }
}
