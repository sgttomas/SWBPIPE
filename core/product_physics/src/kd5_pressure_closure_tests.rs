//! K-D5's load closure on the integrated T4-U2 pressure producer
//! (DEL-04-01 `kd5-member-relative-bound.md` section 1, rows "T4-U2 pressure
//! free strain on arcs" and "T4-U2 pressure operand groups", and section 7
//! "The closure"; DEL-04-07 `pressure-contract.md`, "Batch-4 M3 dependency").
//!
//! - Every exact-pressure factor-group summand (straight Poisson pairs,
//!   terminal caps, the arcs' -c_b and the bend-adjacent kink remainders)
//!   carries its own `PressureOperand`, and the term's manifest is the
//!   producer's complete group enumeration, all pipes meeting the node.
//! - Each family's binding is checked against an independent binary64
//!   reference of its law (p*pi*r_i^2 times the signed coefficient times the
//!   chord or end-tangent direction; eps_p = (1-2nu)pAi/(EA) for the arc's
//!   K_int,arc*u_free term). FK's own family tests check the p re-formation
//!   against 256-bit references; T4-I7's exact references check the
//!   certified rows of the whole integrated system (`t4_u2_pressure_references`).
//! - A drop-one-of-many mutant at a node where several pipes meet, and
//!   duplicate, extra, wrong-pipe and wrong-binding supplied primitives, fail
//!   closed with a named load-closure detail on the ordinary and W2 routes,
//!   in both solver modes; the unmutated system passes with the member bound.
//!
//! Inputs are T4-I7's invented document sketches.
use super::*;
use crate::certified_loads::supplied_mutant;
use serde_json::Value;
use open_pipe_stress_frame_kernel::structural::{
    LoadFormula, LoadPrimitive, PressureCoefficient, PressureDirection, PressureGeometry,
};

const I7_DOCUMENTS: &str =
    include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");
const MODES: [LinearSolveMode; 2] = [LinearSolveMode::DenseScrutiny, LinearSolveMode::SparseInteractive];

/// The frozen sketch's copy, re-authored once to the current 0.7.0 exact form
/// (B7b; the frozen reference itself is unchanged).
fn sketch(id: &str) -> Value {
    let documents: Value = serde_json::from_str(I7_DOCUMENTS).unwrap();
    crate::b7b_admission_tests::reauthor_current_request(
        documents["sketches"][id]["0.3.0"]["document"].clone(),
    )
}

struct Case {
    model: PreviewModel,
    built: BuiltModel,
    exact: pressure_runtime::ExactPressureCase,
    force: AssembledForce,
}

/// The first load case's ledger through the product's own producers
/// (`case_force_ledger`), with any armed supplied-primitive mutant.
fn case(request: &Value) -> Case {
    let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() { model.materials.clone() } else { request.materials };
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).expect("model builds");
    let load_case = model.load_cases[0].clone();
    let exact = pressure_runtime::build_pressure_case_with_members(&model, &built, &materials, &load_case, None, &mut diagnostics)
        .unwrap_or_else(|| panic!("an exact-pressure case: {diagnostics:?}"));
    let loads = build_load_case_primitive_loads(&model, &load_case, &mut diagnostics);
    let application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
    let material_map = materials.iter().map(|m| (m.id.as_str(), m)).collect::<HashMap<_, _>>();
    let pipe_map = model.pipe_segments.iter().enumerate().map(|(i, p)| (p.id.as_str(), i)).collect::<HashMap<_, _>>();
    let thermal = build_thermal_element_loads(&model, &load_case, &material_map, &pipe_map, &built, &mut diagnostics);
    let bends = built.curved_bend_elements.iter().map(|e| (e.pipe_index, e)).collect::<HashMap<_, _>>();
    let force = case_force_ledger(&model, &built, &application, &bends, &thermal, Some(&exact), &load_case.id, &mut diagnostics)
        .finish(built.nodes.len() * DOF_PER_NODE)
        .unwrap();
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    Case { model, built, exact, force }
}

fn node_pair(primitive: &LoadPrimitive) -> Option<(usize, usize)> {
    match &primitive.formula {
        LoadFormula::PressureOperand { geometry: PressureGeometry::Straight { frame }, .. } => {
            Some((frame.node_i.index, frame.node_j.index))
        }
        LoadFormula::PressureOperand { geometry: PressureGeometry::Curved { arc }, .. } => Some((arc.node_i, arc.node_j)),
        _ => None,
    }
}

fn is_arc(primitive: &LoadPrimitive) -> bool {
    matches!(&primitive.formula, LoadFormula::PressureOperand { geometry: PressureGeometry::Curved { .. }, .. })
}

/// A pressure term whose summands come from at least two pipes, one of them
/// a realized arc (a bend-adjacent node where several pipes meet).
fn grouped(term: usize, supplied: &[LoadPrimitive]) -> bool {
    let pairs = supplied.iter().filter_map(node_pair).collect::<HashSet<_>>();
    let _ = term;
    pairs.len() >= 2 && supplied.iter().any(is_arc) && supplied.iter().all(|p| p.identity.family == "pressure_operand")
}

// ------------------------------------------------- independent references

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn unit(a: [f64; 3]) -> [f64; 3] {
    let n = dot(a, a).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}

/// Chord axes (x along the chord, y the y_reference made orthogonal to x).
fn axes(xi: [f64; 3], xj: [f64; 3], yr: [f64; 3]) -> ([f64; 3], [f64; 3], f64) {
    let d = sub(xj, xi);
    let l = dot(d, d).sqrt();
    let x = unit(d);
    let p = dot(yr, x);
    let y = unit([yr[0] - p * x[0], yr[1] - p * x[1], yr[2] - p * x[2]]);
    (x, y, l)
}

/// The binary64 reference value of one `PressureOperand` at its DOF.
fn operand_reference(formula: &LoadFormula) -> f64 {
    let LoadFormula::PressureOperand { geometry, pressure, outer_diameter, wall_thickness, coefficient, direction } = formula else {
        panic!("not a pressure operand");
    };
    let ri = 0.5 * outer_diameter - wall_thickness;
    let force = pressure * std::f64::consts::PI * ri * ri;
    let c = match *coefficient {
        PressureCoefficient::Constant(c) => c,
        PressureCoefficient::Poisson { poisson_ratio, sign } => poisson_ratio * sign,
    };
    let d = match (geometry, *direction) {
        (PressureGeometry::Straight { frame }, PressureDirection::Chord { axis }) => {
            axes(frame.node_i.coordinates, frame.node_j.coordinates, frame.y_reference).0[axis]
        }
        (PressureGeometry::Curved { arc }, PressureDirection::Tangent { end_j, axis }) => {
            let (x, y, l) = axes(arc.coordinates_i, arc.coordinates_j, arc.y_reference);
            let s = l / (2.0 * arc.radius);
            let cos = (1.0 - s * s).sqrt();
            if end_j { cos * x[axis] - s * y[axis] } else { cos * x[axis] + s * y[axis] }
        }
        other => panic!("unexpected pressure geometry/direction {other:?}"),
    };
    force * c * d
}

/// The skew single-arc sketch with its node-B terminal closure separately
/// supported: the region terminal sits at the realized arc's end j and its
/// cap does not transfer to the wall, so the arc's `-c_b` summand at B has no
/// `terminal_cap` (and, being a terminal, no kink remainder) to cancel it.
/// `traverse_region` admits a terminal at an arc end and the terminal
/// validation admits either closure, so this is an admitted configuration in
/// which the arc's end-tangent binding changes the term's value.
fn uncancelled_arc_end() -> Value {
    let mut document = sketch("MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K2-SKEW");
    let terminal = &mut document["model"]["load_cases"][0]["pressure_regions"][0]["terminals"][1];
    assert_eq!(terminal["node_ref"], "node:B");
    terminal["closure_transfer"] = Value::from("separately_supported_or_compensated");
    document
}

/// Each formed pressure term's summands sum, in the independent reference,
/// to the producer's binary64 group value; each summand's geometry is bound
/// at the node of its term's DOF (an arc's end tangent is end j exactly when
/// that node is the arc's node j; a straight's frame contains the node); each
/// arc summand's tangent law agrees with CB's end tangents (the producer's own
/// directions); the arc's eps_p law agrees with the represented
/// `arc_pressure_strain`. At a bend-adjacent node an arc's `-c_b` cancels its
/// own kink remainder or transferred terminal cap, so the uncancelled
/// arc-end sketch is the one whose value depends on the end binding.
#[test]
fn kd5_u2_pressure_families_bind_their_own_operands() {
    let mut checked = [0usize; 5];
    let documents = ["U2-L-ANCH-P-K2", "U2-L-ANCH-ALL-K2-SKEW", "U2-L-KINK-ANCH-P-K2", "U2-U-ANCH-ALL-K2-SKEW-X7P3E6"]
        .map(|id| (id, sketch(id)))
        .into_iter()
        .chain([("uncancelled arc end", uncancelled_arc_end())]);
    for (id, document) in documents {
        let Case { built, exact, force, .. } = case(&document);
        let primitives = force.load_primitives();
        for (term, record) in force.closure_records().iter().enumerate() {
            let supplied: Vec<_> = primitives.iter().filter(|p| p.term_index == term).collect();
            if supplied.is_empty() || supplied[0].identity.family != "pressure_operand" {
                continue;
            }
            let node = force.terms()[term].dof / DOF_PER_NODE;
            for p in &supplied {
                match &p.formula {
                    LoadFormula::PressureOperand { geometry: PressureGeometry::Curved { arc }, direction: PressureDirection::Tangent { end_j, .. }, .. } => {
                        assert!([arc.node_i, arc.node_j].contains(&node), "{id} term {term}: the arc does not meet the term's node");
                        assert_eq!(*end_j, node == arc.node_j, "{id} term {term}: the arc's tangent is bound at the other end");
                    }
                    LoadFormula::PressureOperand { geometry: PressureGeometry::Straight { frame }, direction: PressureDirection::Chord { .. }, .. } => {
                        assert!([frame.node_i.index, frame.node_j.index].contains(&node), "{id} term {term}: the straight does not meet the term's node");
                    }
                    other => panic!("{id} term {term}: unexpected pressure binding {other:?}"),
                }
            }
            // A lone arc summand: its term's value is its own end tangent's.
            if supplied.len() == 1 && is_arc(supplied[0]) {
                checked[4] += 1;
            }
            // The manifest is complete, separately installed, and the supplied
            // vector matches it exactly on the unmutated producer.
            let manifest = record.manifest.as_ref().expect("a formed pressure term has its manifest");
            assert!(record.formed);
            assert_eq!(manifest.len(), supplied.len(), "{id} term {term}");
            assert_eq!(manifest, &supplied.iter().map(|p| p.closure_entry()).collect::<Vec<_>>(), "{id} term {term}");
            let open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(value) = force.terms()[term].kind else { panic!("{id}: a pressure operand is a rounded term") };
            let reference: f64 = supplied.iter().map(|p| operand_reference(&p.formula)).sum();
            let scale = supplied.iter().map(|p| operand_reference(&p.formula).abs()).fold(0.0, f64::max).max(value.abs());
            assert!((reference - value).abs() <= 1e-12 * scale, "{id} term {term}: reference {reference:e} against the producer's {value:e}");
            for p in &supplied {
                if let LoadFormula::PressureOperand { geometry: PressureGeometry::Curved { arc }, direction: PressureDirection::Tangent { end_j, axis }, .. } = &p.formula {
                    let bend = built.curved_bend_elements.iter().find(|b| b.macro_element.node_i.index == arc.node_i).unwrap();
                    let tangent = bend.macro_element.end_tangents().unwrap()[usize::from(*end_j)][*axis];
                    let (x, y, l) = axes(arc.coordinates_i, arc.coordinates_j, arc.y_reference);
                    let s = l / (2.0 * arc.radius);
                    let cos = (1.0 - s * s).sqrt();
                    let law = if *end_j { cos * x[*axis] - s * y[*axis] } else { cos * x[*axis] + s * y[*axis] };
                    assert!((law - tangent).abs() <= 1e-12, "{id}: the re-formed tangent law is CB's end tangent ({law:e} against {tangent:e})");
                    checked[1] += 1;
                }
                checked[0] += 1;
            }
        }
        // The arc's member-owned K_int,arc*u_free(eps_p): one primitive per
        // pushed product, its raw law equal to the represented eps_p.
        for (pipe_index, state) in &exact.pipe_states {
            let Some(arc_state) = state.arc else { continue };
            let pipe = &built.pipes[*pipe_index];
            let prefix = certified_loads::member_source(&pipe.element_id, &state.region_id);
            let terms: Vec<_> = primitives.iter().filter(|p| p.identity.family == "pressure_free_strain_curved" && p.identity.source == prefix).collect();
            let bend = built.curved_bend_elements.iter().find(|b| b.pipe_index == *pipe_index).unwrap();
            let nonzero = bend.chord.iter().filter(|c| **c != 0.0).count();
            assert_eq!(terms.len(), 12 * nonzero, "{id} {}", pipe.element_id);
            for p in terms {
                let LoadFormula::PressureFreeStrainCurved { arc, pressure, outer_diameter, wall_thickness, poisson_ratio, column } = &p.formula else { panic!() };
                let ri = 0.5 * outer_diameter - wall_thickness;
                let eps = (1.0 - 2.0 * poisson_ratio) * pressure * std::f64::consts::PI * ri * ri / (arc.elastic_modulus * arc.area);
                assert!((eps - arc_state.strain).abs() <= 1e-12 * arc_state.strain.abs(), "{id}: eps_p law {eps:e} against {:e}", arc_state.strain);
                let col = column.expect("a column per product");
                assert!((DOF_PER_NODE..DOF_PER_NODE + 3).contains(&col) && bend.chord[col - DOF_PER_NODE] != 0.0);
                assert_eq!(p.local_row, p.identity.ordinal / 12);
                checked[2] += 1;
            }
            checked[3] += 1;
        }
    }
    assert!(checked.iter().all(|&n| n > 0), "every family is exercised: {checked:?}");
}

// --------------------------------------------------------------- mutants

#[derive(Clone, Copy, Debug)]
enum Mutant {
    Drop,
    Duplicate,
    Extra,
    WrongPipe,
    WrongEnd,
    WrongSign,
}

fn arm(mutant: Mutant) {
    supplied_mutant::arm(Box::new(move |term, supplied: &mut Vec<LoadPrimitive>| {
        if !grouped(term, supplied) {
            return false;
        }
        let arc = supplied.iter().position(is_arc).unwrap();
        match mutant {
            Mutant::Drop => {
                supplied.remove(arc);
            }
            Mutant::Duplicate => supplied.push(supplied[arc].clone()),
            Mutant::Extra => {
                let mut extra = supplied[arc].clone();
                extra.identity.ordinal = supplied.len();
                supplied.push(extra);
            }
            Mutant::WrongPipe => {
                // The arc summand's identity attached to a straight pipe's geometry.
                let straight = supplied.iter().position(|p| !is_arc(p)).unwrap();
                let LoadFormula::PressureOperand { geometry, .. } = supplied[straight].formula.clone() else { unreachable!() };
                let LoadFormula::PressureOperand { geometry: target, direction, .. } = &mut supplied[arc].formula else { unreachable!() };
                let axis = match *direction { PressureDirection::Chord { axis } | PressureDirection::Tangent { axis, .. } => axis };
                *target = geometry;
                *direction = PressureDirection::Chord { axis };
            }
            Mutant::WrongEnd => {
                let LoadFormula::PressureOperand { direction: PressureDirection::Tangent { end_j, .. }, .. } = &mut supplied[arc].formula else { unreachable!() };
                *end_j = !*end_j;
            }
            Mutant::WrongSign => {
                let LoadFormula::PressureOperand { coefficient, .. } = &mut supplied[arc].formula else { unreachable!() };
                *coefficient = match *coefficient {
                    PressureCoefficient::Constant(c) => PressureCoefficient::Constant(-c),
                    PressureCoefficient::Poisson { poisson_ratio, sign } => PressureCoefficient::Poisson { poisson_ratio, sign: -sign },
                };
            }
        }
        true
    }));
}

/// The case's checked solves: (ordinary, W2 at 2^b) per mode, each as
/// (quality, formation-check text, member bound present).
fn checked_solves(case: &Case, b: i32) -> Vec<(String, SolveQuality, String, bool)> {
    let built = &case.built;
    let n = built.nodes.len() * DOF_PER_NODE;
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let free = (0..n).filter(|d| !boundary.restrained_dofs.contains(d)).collect::<Vec<_>>();
    let prescribed = boundary.restrained_dofs.iter().map(|&d| (d, 0.0)).collect::<Vec<_>>();
    let BasisStiffness::Formed(formed) = form_basis_stiffness(built, &[], true).unwrap() else { panic!("formed") };
    let dense = formed.to_dense();
    let curved = built
        .curved_bend_elements
        .iter()
        .map(|e| CurvedBendStiffnessElement::from_macro_element(e.component_id.clone(), &e.macro_element).unwrap())
        .collect::<Vec<_>>();
    let macros = built.curved_bend_elements.iter().map(|e| e.macro_element).collect::<Vec<_>>();
    let mut out = Vec::new();
    // T4-U8 (Design §4 item 2): the product's source names each frame's member.
    let members = crate::frame_member_identities(built);
    for mode in MODES {
        let assembly = AssemblyEvidence::new(built.nodes.len(), &built.frame_elements, &built.connectors, &curved, &[])
            .unwrap()
            .with_frame_members(&members);
        let ordinary = assembly.solve_assembled_with_formation_check(&dense, &case.force, &free, &prescribed, mode, &macros, true).unwrap();
        out.push((format!("ordinary {mode:?}"), ordinary.report.quality, format!("{:?}", ordinary.formation_check), ordinary.member_bound.is_some()));
        let scale = ForceScale::new(b).unwrap();
        let factor = 2f64.powi(b);
        let scaled = dense.iter().map(|row| row.iter().map(|v| v * factor).collect()).collect::<Vec<Vec<f64>>>();
        let assembly = AssemblyEvidence::new_force_scaled(built.nodes.len(), &built.frame_elements, &built.connectors, &curved, &[], scale)
            .unwrap()
            .with_frame_members(&members);
        let w2 =assembly.solve_force_scaled_with_formation_check(&scaled, &case.force, &free, &prescribed, mode, &macros, true).unwrap();
        out.push((format!("W2 2^{b} {mode:?}"), w2.solution.report.quality, format!("{:?}", w2.solution.formation_check), w2.solution.member_bound.is_some()));
    }
    out
}

/// The unmutated integrated producer closes on both routes; each mutant of
/// the supplied primitives at a grouped bend-adjacent term fails closed with
/// the named load-closure detail, while the independently installed manifest
/// keeps every original summand.
#[test]
fn kd5_u2_grouped_pressure_mutants_fail_closed_on_ordinary_and_w2_routes() {
    let request = sketch("U2-L-ANCH-P-K2");
    let control = case(&request);
    // W2's classification and complete scaled manifest are preserved.
    let scaled = control.force.force_scaled(ForceScale::new(4).unwrap()).unwrap();
    assert_eq!(scaled.closure_records().len(), control.force.closure_records().len());
    for (original, rescaled) in control.force.closure_records().iter().zip(scaled.closure_records()) {
        assert_eq!(original.formed, rescaled.formed);
        assert_eq!(original.manifest.is_some(), rescaled.manifest.is_some());
        let (Some(original), Some(rescaled)) = (&original.manifest, &rescaled.manifest) else { continue };
        // The same entries, each rebound with the force exponent (its last
        // operand word) and otherwise bit-identical.
        assert_eq!(original.len(), rescaled.len());
        for (a, b) in original.iter().zip(rescaled) {
            assert_eq!(a.identity, b.identity);
            let (a_exponent, a_operands) = a.operand_bits.split_last().unwrap();
            let (b_exponent, b_operands) = b.operand_bits.split_last().unwrap();
            assert_eq!(a_operands, b_operands, "{:?}", a.identity);
            assert_eq!(*b_exponent as i64, *a_exponent as i64 + 4, "{:?}", a.identity);
        }
    }
    assert!(scaled.load_primitives().iter().all(|p| p.force_scale_exponent == 4));
    for (route, quality, check, bound) in checked_solves(&control, 4) {
        assert_eq!(quality, SolveQuality::Passed, "{route}: {check}");
        assert!(bound, "{route}: the closed system carries the member bound");
    }
    for mutant in [Mutant::Drop, Mutant::Duplicate, Mutant::Extra, Mutant::WrongPipe, Mutant::WrongEnd, Mutant::WrongSign] {
        arm(mutant);
        let mutated = case(&request);
        let term = supplied_mutant::disarm().unwrap_or_else(|| panic!("{mutant:?}: precondition, a grouped arc term exists"));
        let manifest = mutated.force.closure_records()[term].manifest.as_ref().unwrap();
        assert_eq!(manifest, control.force.closure_records()[term].manifest.as_ref().unwrap(), "{mutant:?}: the manifest is independent of the supplied vector");
        let pipes = control.force.load_primitives().iter().filter(|p| p.term_index == term).filter_map(node_pair).collect::<HashSet<_>>();
        assert!(pipes.len() >= 2, "{mutant:?}: several pipes meet at the term's node");
        for (route, quality, check, bound) in checked_solves(&mutated, 4) {
            assert_eq!(quality, SolveQuality::Sensitive, "{mutant:?} {route}");
            assert!(check.contains("load closure"), "{mutant:?} {route}: {check}");
            assert!(!bound, "{mutant:?} {route}");
        }
    }
}

/// K-D5 "One ε" (Design section 1): a producer may omit only a source-known
/// exactly zero contribution, never a formed one because its binary64 fold
/// is zero. The quarter arc from (1.5, 0, 0) to (0, 1.5, 0) with
/// y_reference (1, 1, 0) and R one ulp below 1.5 has end tangents whose
/// binary64 components t_i,x and t_j,y fold to exactly zero, while at p they
/// are +-(s - c)/sqrt2 = +-2(a - R)(a + R)/(sqrt2 R (L + sqrt(4R^2 - L^2)))
/// with a - R = 2^-52 * 1.5 exact and nonzero (an 80-digit evaluation gives
/// 1.4802973661668757e-16). Node B's closure is separately supported, so
/// the arc's -c_b at B is the lone summand of its term. Both components keep
/// their arc operand at the term of their DOF, and the unmutated system
/// closes on both routes with the member bound.
#[test]
fn kd5_u2_arc_tangent_folded_to_zero_keeps_its_formed_operand() {
    let mut document = sketch("MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K2");
    let model = &mut document["model"];
    let (a, radius) = (1.5_f64, f64::from_bits(1.5_f64.to_bits() - 1));
    model["nodes"][0]["position"] = serde_json::json!({"x": a, "y": 0.0, "z": 0.0});
    model["nodes"][1]["position"] = serde_json::json!({"x": 0.0, "y": a, "z": 0.0});
    model["components"][0]["geometry"]["bend_radius"]["value"] = Value::from(radius);
    let terminal = &mut model["load_cases"][0]["pressure_regions"][0]["terminals"][1];
    assert_eq!(terminal["node_ref"], "node:B");
    terminal["closure_transfer"] = Value::from("separately_supported_or_compensated");
    let control = case(&document);
    let bend = &control.built.curved_bend_elements[0];
    let tangents = bend.macro_element.end_tangents().unwrap();
    let (node_i, node_j) = (bend.macro_element.node_i.index, bend.macro_element.node_j.index);
    // Preconditions: binary64 zero, not source-known zero, nonzero at p.
    assert_eq!((tangents[0][0], tangents[1][1]), (0.0, 0.0), "{tangents:?}");
    let chord = [0, 1].map(|k| bend.macro_element.node_j.coordinates[k] - bend.macro_element.node_i.coordinates[k]);
    assert!(chord.iter().all(|d| *d != 0.0));
    let l = (2.0 * a * a).sqrt();
    let formed = 2.0 * (a - radius) * (a + radius) / (2.0_f64.sqrt() * radius * (l + (4.0 * radius * radius - l * l).sqrt()));
    assert!((formed / 1.4802973661668757e-16 - 1.0).abs() < 1e-3, "{formed:e}");
    let primitives = control.force.load_primitives();
    for (node, axis, end_j) in [(node_i, 0, false), (node_j, 1, true)] {
        let dof = node * DOF_PER_NODE + axis;
        let terms: Vec<_> = (0..control.force.terms().len())
            .filter(|&term| control.force.terms()[term].dof == dof)
            .filter(|&term| primitives.iter().any(|p| p.term_index == term && p.identity.family == "pressure_operand"))
            .collect();
        assert_eq!(terms.len(), 1, "one pressure term at node {node} axis {axis}");
        let term = terms[0];
        let manifest = control.force.closure_records()[term].manifest.as_ref().expect("its manifest");
        let supplied: Vec<_> = primitives.iter().filter(|p| p.term_index == term).collect();
        assert_eq!(manifest, &supplied.iter().map(|p| p.closure_entry()).collect::<Vec<_>>());
        let arc_operands = supplied
            .iter()
            .filter(|p| matches!(&p.formula, LoadFormula::PressureOperand { direction: PressureDirection::Tangent { end_j: e, axis: x }, .. } if *e == end_j && *x == axis))
            .count();
        assert!(arc_operands >= 1, "node {node} axis {axis}: the arc's formed tangent operand is kept");
        let open_pipe_stress_frame_kernel::load_ledger::ForceTermKind::Term(value) = control.force.terms()[term].kind else { panic!() };
        assert_eq!(value, 0.0, "the ordinary force bits are the binary64 fold");
        if end_j {
            assert_eq!(supplied.len(), 1, "node B: the arc's -c_b is the lone summand");
        }
    }
    // The published observation of the binary64 RHS lists only groups with
    // a nonzero binary64 direction (the reader's RHS_DIRECTION); the formed
    // zero-fold groups are ledger terms with closure operands beyond it.
    let groups = control.exact.assembly_evidence["groups"].as_array().unwrap();
    assert!(groups.iter().all(|g| g["direction_component_magnitude"].as_f64().unwrap() > 0.0));
    assert!(control.exact.assembled_operands.len() > groups.len());
    for (route, quality, check, bound) in checked_solves(&control, 4) {
        assert_eq!(quality, SolveQuality::Passed, "{route}: {check}");
        assert!(bound, "{route}: the closed system carries the member bound");
    }
}

/// Repaired arc wall publication under pressure (Design section 3.2,
/// pipe-wall rows; H-2): on a Passed case the arc's wall force at both ends
/// and the three stations is its effective force plus pAi, the effective
/// force is the arc's elastic axial row, and the wall end actions are -N_w at
/// end i and +N_w at end j, in both modes.
#[test]
fn kd5_u2_arc_wall_rows_keep_nw_equal_s_plus_p_ai() {
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let request = sketch("U2-L-ANCH-PTW-K2");
        let control = case(&request);
        let state = control.exact.pipe_states.values().find(|s| s.arc.is_some()).unwrap();
        let p_ai = state.pressure.pressure_pa() * state.sections.pressure_bore_area_m2();
        let output = run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap();
        let case_id = &control.model.load_cases[0].id;
        let report = output.diagnostics.iter().find(|d| d.id == integrity_diagnostic_id(case_id)).unwrap();
        assert_eq!(report.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{mode:?}: {}", report.message);
        let row = |kind: &str, location: &str| {
            output.results.iter().find(|r| r.entity_ref == "pipe:BEND" && r.kind == kind && r.metadata.as_ref().is_some_and(|m| m.location == location))
                .unwrap_or_else(|| panic!("{kind} {location}")).value
        };
        for location in ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"] {
            let (wall, effective) = (row("pipe_wall_axial_force_v2", location), row("pipe_effective_axial_force_v2", location));
            assert!((wall - effective - p_ai).abs() <= 1e-12 * wall.abs().max(p_ai), "{mode:?} {location}: Nw {wall:e}, S {effective:e}, pAi {p_ai:e}");
        }
        assert_eq!(row("pipe_wall_endpoint_action_v2", "end_i"), -row("pipe_wall_axial_force_v2", "end_i"));
        assert_eq!(row("pipe_wall_endpoint_action_v2", "end_j"), row("pipe_wall_axial_force_v2", "end_j"));
        assert!(!output.results.iter().any(|r| r.entity_ref == "pipe:BEND" && r.kind == "element_local_axial_force"), "{mode:?}");
    }
}

/// The certified arc wall rows read the certified recovery's section by its
/// location name, and an unknown name fails closed with a named blocking
/// diagnostic instead of being read as end_j (Design section 3.2, pipe-wall
/// rows).
#[test]
fn kd5_u2_arc_wall_rows_refuse_an_unknown_section_location() {
    use open_pipe_stress_frame_kernel::structural::{CertifiedMemberRecovery, CertifiedRow};
    let names = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];
    for (index, name) in names.into_iter().enumerate() {
        assert_eq!(bend_pressure::certified_section_index(name), Some(index));
    }
    for name in ["end", "quarter_2", "END_J", ""] {
        assert_eq!(bend_pressure::certified_section_index(name), None, "{name:?}");
    }
    let control = case(&sketch("U2-L-ANCH-PTW-K2"));
    let state = control.exact.pipe_states.values().find(|s| s.arc.is_some()).unwrap();
    let load_case = &control.model.load_cases[0];
    let rows = CertifiedMemberRecovery { ends: [CertifiedRow::ZERO; 12], sections: [[CertifiedRow::ZERO; 6]; 5] };
    let run = |locations: [&'static str; 3]| {
        let stations = locations.map(|location| StationResultants { location, resultants: [0.0; 6] });
        let (mut results, mut diagnostics) = (Vec::new(), Vec::new());
        bend_pressure::append_arc_pressure_results(
            &mut results, &mut diagnostics, load_case, "pipe:BEND", state, &[[0.0; 6]; 2], &stations, Some((&rows, &rows)),
        );
        (results, diagnostics)
    };
    let (results, diagnostics) = run(["quarter_1", "midspan", "quarter_3"]);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    // Two wall end actions, then at each of the five locations the wall and
    // effective forces and the membrane stress, which D-5 (DEL-05-03 Design
    // §2.1) publishes on the nominal section and then on the reduced one.
    assert_eq!(results.len(), 2 + 5 * (2 + 2));
    assert_eq!(results.iter().filter(|r| r.kind == "pipe_axial_membrane_stress_v2").count(), 5 * 2);
    let (results, diagnostics) = run(["quarter_1", "quarter_2", "quarter_3"]);
    assert!(results.is_empty(), "no row is published from an unknown section");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code, bend_pressure::ARC_SECTION_UNKNOWN);
    assert_eq!(diagnostics[0].severity, "blocking");
    assert!(diagnostics[0].message.contains("quarter_2"), "{}", diagnostics[0].message);
}
