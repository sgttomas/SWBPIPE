//! DEL-05-01 static-equivalent seismic generation: the component force
//! (the definition of static-equivalent generation, REQ-008, REQ-015, AC-007;
//! LIM-028's repair).
//!
//! Every member not replaced by an objective connector is selected. The
//! self-weight plan's own dispositions (`self_weight::component_dispositions`,
//! REQ-015's order) decide which members a rigid-family component replaces:
//! a mapping conflict blocks (the document's validation names it); a
//! component declared `not_solver_consumed` keeps its member's distributed
//! load, quietly, before its weight or offset is read; a nonzero
//! centre-of-gravity coordinate that normalizes to zero blocks the case by
//! name; the first failed fallback condition keeps the distributed load;
//! otherwise, for each entered global-axis g-factor k, the component force
//! F = fl(k × W) along that axis acts at the component's node, with the
//! moment r × F, each transverse component fl(F × ±r_j) from the rounded
//! force (round to nearest, ties to even), one moment per nonzero transverse
//! coordinate. A force or moment required to be nonzero (k ≠ 0; and r_j ≠ 0
//! for a moment) that is zero or nonfinite after rounding blocks the case by
//! name; a finite nonzero subnormal force passes. A zero g-factor keeps the
//! replacement, with zero force and moments.
//!
//! The generator, the certified nodal producer (`certified_loads::nodal`)
//! and the exact route's generation record each read the factor, W and r
//! from the model through [`plan`], and identify an item only by the exact
//! identifier [`load_id`] forms; nothing is parsed out of an identifier.
//!
//! The certified ledger holds each component force and moment as the exact
//! stored value the rule gives, as it holds the self-weight plan's component
//! wrench: it forms no product of its own. Where the ledger is built, the
//! value is bound to its operands: [`rule_value`] recomputes fl(k × W), or
//! fl(F × ±r_j) from that rounded force, from the converted operands the
//! generator read (W in N, r in m), and a prepared value whose bits differ
//! refuses the case by name ([`VALUE_MISMATCH`]).
use super::{diag, stable_suffix, Diagnostic, PreviewComponent, PreviewLoadCase, PreviewModel};
use crate::self_weight::{component_dispositions, moment_levers, Disposition};
use open_pipe_stress_linear_supports::FrameDof;
use open_pipe_stress_primitive_loads::{LoadDimension, LoadDirection, LoadQuantity, PrimitiveLoad, PrimitiveLoadCategory};
use std::collections::HashSet;

/// The named blocking refusal of a component case: a required force or
/// moment out of range after rounding, or a nonzero centre-of-gravity
/// coordinate that normalizes to zero.
pub(crate) const RANGE_INVALID: &str = "EQUIVALENT_STATIC_COMPONENT_RANGE_INVALID";
/// The named blocking refusal of a case whose prepared component force or
/// moment is not, bit for bit, the rule's value recomputed from its operands
/// where the certified ledger is built.
pub(crate) const VALUE_MISMATCH: &str = "EQUIVALENT_STATIC_COMPONENT_VALUE_MISMATCH";
const AXIS_LABELS: [&str; 3] = ["global-x", "global-y", "global-z"];
const AXES: [&str; 3] = ["x", "y", "z"];

/// A part of one component item: the force along the item's axis, or the
/// moment about global axis `0..3`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Part {
    Force,
    Moment(usize),
}

/// The deterministic identity of one component primitive, in the case's
/// generated seismic namespace: the axis, the literal `component`, the
/// length-prefixed component id, then `force` or `moment:<x|y|z>`.
pub(crate) fn load_id(case: &str, component: &str, axis: usize, part: Part) -> String {
    let tail = match part {
        Part::Force => "force".to_string(),
        Part::Moment(i) => format!("moment:{}", AXES[i]),
    };
    format!(
        "{case}:generated:seismic:{}:component:{}:{component}:{tail}",
        AXIS_LABELS[axis],
        component.len()
    )
}

/// One component item: one per replacing component and entered axis.
#[derive(Debug, Clone)]
pub(crate) struct Item<'a> {
    pub(crate) component: &'a PreviewComponent,
    pub(crate) node_index: usize,
    pub(crate) axis: usize,
    pub(crate) factor: f64,
    pub(crate) weight: f64,
    pub(crate) cog: [f64; 3],
    /// fl(k × W).
    pub(crate) force: f64,
    /// (moment axis, signed lever, fl(force × lever)), one per nonzero
    /// transverse coordinate, in x, y, z order.
    pub(crate) moments: Vec<(usize, f64, f64)>,
}

impl Item<'_> {
    /// The item's primitives: (identifier, part, value), force first.
    pub(crate) fn parts(&self, case: &str) -> Vec<(String, Part, f64)> {
        let mut parts = vec![(load_id(case, &self.component.id, self.axis, Part::Force), Part::Force, self.force)];
        for &(i, _, value) in &self.moments {
            parts.push((load_id(case, &self.component.id, self.axis, Part::Moment(i)), Part::Moment(i), value));
        }
        parts
    }
}

/// The members a component replaces and its items, in generation order:
/// entered axis (x, y, z), then the model's component order.
#[derive(Debug, Clone, Default)]
pub(crate) struct Plan<'a> {
    pub(crate) replaced: HashSet<&'a str>,
    pub(crate) items: Vec<Item<'a>>,
}

/// The case's entered g-factors, (global axis, value), in x, y, z order.
pub(crate) fn entered_factors(case: &PreviewLoadCase) -> Vec<(usize, f64)> {
    let Some(seismic) = case.equivalent_static.as_ref().and_then(|g| g.seismic.as_ref()) else {
        return Vec::new();
    };
    [&seismic.g_factor_x, &seismic.g_factor_y, &seismic.g_factor_z]
        .into_iter()
        .enumerate()
        .filter_map(|(axis, factor)| factor.as_ref().map(|q| (axis, q.value)))
        .collect()
}

/// The component dispositions and items of a seismic case. `connector_replaced`
/// are the spans objective connectors replace (never selected). A blocking
/// finding is pushed to `diagnostics` and the plan is still returned, so the
/// caller generates nothing for the case. A nonfinite factor forms no item:
/// the generator refuses it by name.
pub(crate) fn plan<'a>(
    model: &'a PreviewModel,
    case_id: &str,
    connector_replaced: &HashSet<String>,
    factors: &[(usize, f64)],
    diagnostics: &mut Vec<Diagnostic>,
) -> Plan<'a> {
    let mut plan = Plan::default();
    let selected = |member: &str| {
        !connector_replaced.contains(member) && model.pipe_segments.iter().any(|pipe| pipe.id == member)
    };
    // A selected mapping conflict is refused by name by the document's
    // validation (`rigid_component::append_mapping_conflicts`), before any
    // case is generated; a conflicted member forms no item here.
    let (_conflicts, mapped) = component_dispositions(model, &selected);
    let mut eligible = Vec::new();
    for mapped in mapped {
        match mapped.disposition {
            Disposition::Unresolved | Disposition::NotConsumed | Disposition::Fallback(_) => {}
            Disposition::CogUnderflow => refuse(
                diagnostics,
                case_id,
                mapped.component,
                "cog",
                format!(
                    "rigid/semi-rigid component {} maps member {}, but a nonzero centre-of-gravity coordinate normalizes to zero in model length units; its seismic moments cannot be formed",
                    mapped.component.id, mapped.pipe_ref
                ),
            ),
            Disposition::Eligible { weight, cog } => {
                // The node is looked up before the member is marked replaced:
                // a component whose node does not resolve (a typed model the
                // document's validation has not checked) forms no item, and
                // its member keeps its distributed load.
                let Some(node_index) = model.nodes.iter().position(|node| node.id == mapped.component.node) else {
                    continue;
                };
                plan.replaced.insert(mapped.pipe_ref);
                eligible.push((mapped.component, node_index, weight, cog));
            }
        }
    }
    for &(axis, factor) in factors {
        if !factor.is_finite() {
            continue;
        }
        for &(component, node_index, weight, cog) in &eligible {
            let force = factor * weight;
            let required = factor != 0.0;
            if required && (force == 0.0 || !force.is_finite()) {
                refuse(
                    diagnostics,
                    case_id,
                    component,
                    &format!("{}:force", AXIS_LABELS[axis]),
                    format!(
                        "the seismic force of rigid/semi-rigid component {} along {} (g-factor {factor:e} times weight {weight:e} N) is {force:e} N after rounding, outside the representable range",
                        component.id, AXIS_LABELS[axis]
                    ),
                );
                continue;
            }
            let levers = moment_levers(cog, axis);
            let mut moments = Vec::new();
            let mut in_range = true;
            for (i, lever) in levers.into_iter().enumerate() {
                if i == axis || lever == 0.0 {
                    continue;
                }
                let value = lever * force;
                if required && (value == 0.0 || !value.is_finite()) {
                    refuse(
                        diagnostics,
                        case_id,
                        component,
                        &format!("{}:moment-{}", AXIS_LABELS[axis], AXES[i]),
                        format!(
                            "the seismic moment about global {} of rigid/semi-rigid component {} under its force along {} ({force:e} N times lever {lever:e} m) is {value:e} N*m after rounding, outside the representable range",
                            AXES[i], component.id, AXIS_LABELS[axis]
                        ),
                    );
                    in_range = false;
                }
                moments.push((i, lever, value));
            }
            if in_range {
                plan.items.push(Item { component, node_index, axis, factor, weight, cog, force, moments });
            }
        }
    }
    plan
}

fn refuse(diagnostics: &mut Vec<Diagnostic>, case_id: &str, component: &PreviewComponent, part: &str, message: String) {
    diagnostics.push(diag(
        &format!(
            "diagnostic:equivalent-static:{}:seismic:component:{}:{part}",
            stable_suffix(case_id),
            stable_suffix(&component.id)
        ),
        RANGE_INVALID,
        "blocking",
        message,
        vec![case_id.to_string(), component.id.clone()],
    ));
}

/// The plan's nodal primitives, in generation order.
pub(crate) fn primitives(plan: &Plan, case_id: &str) -> Vec<PrimitiveLoad> {
    let mut loads = Vec::new();
    for item in &plan.items {
        for (id, part, value) in item.parts(case_id) {
            let (direction, dimension) = match part {
                Part::Force => (
                    [LoadDirection::GlobalX, LoadDirection::GlobalY, LoadDirection::GlobalZ][item.axis],
                    LoadDimension::Force,
                ),
                Part::Moment(i) => (LoadDirection::Dof([FrameDof::Rx, FrameDof::Ry, FrameDof::Rz][i]), LoadDimension::Moment),
            };
            #[cfg(test)]
            let value = prepared_mutant::apply(&id, value);
            let quantity = LoadQuantity::new(value, dimension).expect("a formed component term is finite");
            loads.push(PrimitiveLoad::nodal_force(id, PrimitiveLoadCategory::Seismic, item.node_index, direction, quantity));
        }
    }
    loads
}

/// The 16 hexadecimal digits of a binary64's bits.
fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

const RECORD_AXES: [&str; 3] = ["global_x", "global_y", "global_z"];

/// The exact route's `equivalent_static.component_forces` (issue 450 item
/// 4): one entry per component and entered axis, in generation order, each
/// read from the model through [`plan`]: the component, its node, the axis,
/// the factor, the weight, the three offset coordinates, the force with its
/// load identifier, and one moment with its load identifier and axis per
/// nonzero transverse coordinate; every binary64 as the 16 hexadecimal digits
/// of its bits. `None` when the case forms no component item, so that its
/// record keeps its earlier bytes. Only items whose force the case generated
/// (`generated`) are recorded.
pub(crate) fn record(model: &PreviewModel, case: &PreviewLoadCase, generated: &[&str]) -> Option<serde_json::Value> {
    case.equivalent_static.as_ref()?.seismic.as_ref()?;
    let replaced = crate::joint::replaced_span_ids(model);
    let plan = plan(model, &case.id, &replaced, &entered_factors(case), &mut Vec::new());
    let entries: Vec<serde_json::Value> = plan
        .items
        .iter()
        .filter(|item| generated.contains(&load_id(&case.id, &item.component.id, item.axis, Part::Force).as_str()))
        .map(|item| {
            serde_json::json!({
                "component_ref": item.component.id,
                "node_ref": item.component.node,
                "axis": RECORD_AXES[item.axis],
                "g_factor": bits(item.factor),
                "weight_N": bits(item.weight),
                "center_of_gravity_m": item.cog.map(bits),
                "force": {"load_id": load_id(&case.id, &item.component.id, item.axis, Part::Force), "value_N": bits(item.force)},
                "moments": item.moments.iter().map(|&(i, _, value)| serde_json::json!({
                    "load_id": load_id(&case.id, &item.component.id, item.axis, Part::Moment(i)),
                    "axis": RECORD_AXES[i],
                    "value_N_m": bits(value),
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    (!entries.is_empty()).then(|| serde_json::Value::Array(entries))
}

/// Test-only replacement of one prepared component value (never of the
/// operands the certified producer reads from the model): issue 450's first
/// test of the nodal branch.
#[cfg(test)]
pub(crate) mod prepared_mutant {
    use std::cell::RefCell;
    pub(crate) type Mutant = Box<dyn Fn(&str, f64) -> Option<f64>>;
    thread_local! {
        static MUTANT: RefCell<Option<Mutant>> = const { RefCell::new(None) };
    }
    pub(crate) fn arm(mutant: Mutant) {
        MUTANT.with(|m| *m.borrow_mut() = Some(mutant));
    }
    pub(crate) fn disarm() {
        MUTANT.with(|m| *m.borrow_mut() = None);
    }
    pub(super) fn apply(id: &str, value: f64) -> f64 {
        MUTANT.with(|m| m.borrow().as_ref().and_then(|mutant| mutant(id, value))).unwrap_or(value)
    }
}

/// The rule's value of one generated component primitive, recomputed where
/// the certified ledger is built: for a load the case's generation produced
/// (the three-condition predicate, judged on the effective case by
/// `is_generated_in_effective_case`: on a load and reference-state document a
/// stored primitive the ledger lists, one a lowering appended included, is
/// authored; issue 452 decision 10) that is a component primitive of the
/// case's own plan, by exact identifier, fl(k × W) for a force and
/// fl(fl(k × W) × ±r_j) for a moment, from the plan's operands (the factor as
/// entered, W normalized to N and r to m by the component's disposition),
/// with the formula's text for a refusal. `None` for every other load.
pub(crate) fn rule_value(model: &PreviewModel, case_id: &str, load_id: &str) -> Option<(f64, String)> {
    let case = model.load_cases.iter().find(|case| case.id == case_id)?;
    if !crate::is_generated_in_effective_case(model, case, load_id, Some("seismic")) {
        return None;
    }
    let replaced = crate::joint::replaced_span_ids(model);
    let plan = plan(model, case_id, &replaced, &entered_factors(case), &mut Vec::new());
    plan.items.iter().find_map(|item| {
        item.parts(case_id).into_iter().find_map(|(id, part, _)| {
            (id == load_id).then(|| match part {
                Part::Force => {
                    (item.factor * item.weight, format!("fl(k*W) = fl({:e} * {:e} N)", item.factor, item.weight))
                }
                Part::Moment(i) => {
                    let (_, lever, _) = item.moments.iter().find(|(axis, _, _)| *axis == i).expect("a formed moment");
                    let force = item.factor * item.weight;
                    (force * lever, format!("fl(F*r) = fl({force:e} N * {lever:e} m)"))
                }
            })
        })
    })
}

/// The binding of one prepared component value to its rule (`rule_value`):
/// `None` when the bits agree, else the blocking refusal naming the load.
pub(crate) fn value_mismatch(case_id: &str, load_id: &str, prepared: f64, rule: &(f64, String)) -> Option<Diagnostic> {
    (prepared.to_bits() != rule.0.to_bits()).then(|| {
        diag(
            &format!("diagnostic:equivalent-static:{}:seismic:load:{}:value", stable_suffix(case_id), stable_suffix(load_id)),
            VALUE_MISMATCH,
            "blocking",
            format!(
                "the generated seismic component load {load_id} of case {case_id} is {prepared:e}, but its rule {} gives {:e}; the certified load would not be the rule's value",
                rule.1, rule.0
            ),
            vec![case_id.to_string(), load_id.to_string()],
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::self_weight::{generate_self_weight_operations, SelfWeightGravity, SelfWeightRequest};
    use crate::{build_load_case_primitive_loads, run_linear_static_preview_value_with_mode, PreviewSolverMode};
    use serde_json::{json, Value};

    const PROV: &str = "seismic component unit test invented input";
    const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

    fn q(value: f64, unit: &str) -> Value {
        json!({"value": value, "unit": unit})
    }

    /// A(0) - B(2.25) - C(2.85) - D(4.6) along X, anchored at A (and D when
    /// `fixed`), on the pressure-free route; a valve at C maps BC with weight
    /// `weight` N and centre of gravity `cog` m; one seismic case `case:s`.
    fn document(weight: f64, cog: [f64; 3], factors: [Option<f64>; 3], fixed: bool) -> Value {
        let section = json!({"outside_diameter": q(0.26, "m"), "wall_thickness": q(0.012, "m"), "material_density": q(7810.0, "kg/m^3")});
        let mut seismic = json!({"gravity_acceleration": q(9.79, "m/s^2")});
        for (axis, k) in ["x", "y", "z"].iter().zip(factors) {
            if let Some(k) = k {
                seismic[format!("g_factor_{axis}")] = q(k, "1");
            }
        }
        let anchor = |id: &str, node: &str| json!({"id": id, "node": node, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV});
        let mut supports = vec![anchor("support:A", "A")];
        if fixed {
            supports.push(anchor("support:D", "D"));
        }
        let nodes: Vec<Value> = [("A", 0.0), ("B", 2.25), ("C", 2.85), ("D", 4.6)].iter()
            .map(|(id, x)| json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV})).collect();
        let pipes: Vec<Value> = [("AB", "A", "B"), ("BC", "B", "C"), ("CD", "C", "D")].iter()
            .map(|(id, from, to)| json!({"id": id, "from": from, "to": to, "section": section, "material": "m",
                "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV})).collect();
        let model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
            "project": {"id": "project:u", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
            "nodes": nodes, "pipe_segments": pipes,
            "materials": [{"id": "m", "elastic_modulus": q(197e9, "Pa"), "shear_modulus": q(76.953125e9, "Pa"), "provenance": PROV}],
            "supports": supports,
            "components": [{"id": "component:valve", "kind": "valve", "node": "C",
                "geometry": {"rigid_pipe_ref": "BC", "weight": q(weight, "N"), "center_of_gravity": {"x": cog[0], "y": cog[1], "z": cog[2], "unit": "m"}},
                "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV}],
            "load_cases": [{"id": "case:s", "primitive_loads": [], "equivalent_static": {"seismic": seismic, "provenance": PROV}, "provenance": PROV}],
            "combinations": []});
        json!({"model": model, "materials": []})
    }
    fn typed(wire: &Value) -> PreviewModel {
        serde_json::from_value(wire["model"].clone()).expect("typed model")
    }
    /// The case's generated loads and findings through the product's own
    /// load builder (the position where generation is formed in each solve).
    fn generated(wire: &Value) -> (Vec<PrimitiveLoad>, Vec<Diagnostic>) {
        let model = typed(wire);
        let mut diagnostics = Vec::new();
        let loads = build_load_case_primitive_loads(&model, &model.load_cases[0], &mut diagnostics);
        (loads, diagnostics)
    }
    fn range_refusals(diagnostics: &[Diagnostic]) -> Vec<String> {
        diagnostics.iter().filter(|d| d.code == RANGE_INVALID && d.severity == "blocking").map(|d| d.id.clone()).collect()
    }
    fn component_values(loads: &[PrimitiveLoad]) -> Vec<(String, f64)> {
        loads.iter().filter(|l| l.load_id.contains(":component:")).map(|l| (l.load_id.clone(), l.magnitude.unwrap().value)).collect()
    }
    fn self_weight_range_refused(wire: &Value) -> bool {
        let request = SelfWeightRequest {
            case_id: "case:weight".into(),
            label: "invented".into(),
            pipe_refs: vec!["AB".into(), "BC".into(), "CD".into()],
            gravity: SelfWeightGravity { value: -9.79, unit: "m/s^2".into(), axis: "global_z".into() },
            provenance: PROV.into(),
            source_model_hash: format!("sha256:{}", "a".repeat(64)),
        };
        match generate_self_weight_operations(&typed(wire), &request) {
            Ok(_) => false,
            Err(d) => {
                assert!(d.iter().all(|d| d.code == "SELF_WEIGHT_RANGE_INVALID"), "{d:?}");
                true
            }
        }
    }

    /// An eligible component whose node does not resolve (a typed model; the
    /// document's validation refuses it earlier) forms no item, and its member
    /// is not marked replaced, so it keeps its distributed load.
    #[test]
    fn an_unresolved_node_does_not_replace_its_member() {
        let mut wire = document(2350.0, [0.0, 0.0, 0.0], [Some(0.3), None, None], false);
        wire["model"]["components"][0]["node"] = json!("Z");
        wire["model"]["pipe_segments"][1]["to"] = json!("Z");
        let model = typed(&wire);
        let mut diagnostics = Vec::new();
        let plan = plan(&model, "case:s", &HashSet::new(), &[(0, 0.3)], &mut diagnostics);
        assert!(plan.replaced.is_empty(), "{:?}", plan.replaced);
        assert!(plan.items.is_empty());
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        // Control: with the node present, the member is replaced.
        let model = typed(&document(2350.0, [0.0, 0.0, 0.0], [Some(0.3), None, None], false));
        let plan = super::plan(&model, "case:s", &HashSet::new(), &[(0, 0.3)], &mut diagnostics);
        assert!(plan.replaced.contains("BC") && plan.items.len() == 1);
    }

    /// The law on the reference values: F = fl(k W) at C, M = r × F from the
    /// rounded force, one moment per nonzero transverse coordinate, BC's
    /// member loads absent, AB and CD's present; generation order is axis,
    /// then member loads before component items.
    #[test]
    fn the_component_force_replaces_the_member_load() {
        let (w, r, k) = (2350.0, [-0.175, 0.12, 0.125], [0.3, 0.35, 0.4]);
        let (loads, diagnostics) = generated(&document(w, r, k.map(Some), false));
        assert!(diagnostics.iter().all(|d| d.severity != "blocking"), "{diagnostics:?}");
        let ids: Vec<&str> = loads.iter().map(|l| l.load_id.as_str()).collect();
        assert!(!ids.iter().any(|id| id.ends_with("element-1")), "BC carries no member load: {ids:?}");
        assert_eq!(ids.iter().filter(|id| id.contains(":element-")).count(), 6);
        let mut expected = Vec::new();
        for axis in 0..3 {
            let f = k[axis] * w;
            expected.push((load_id("case:s", "component:valve", axis, Part::Force), f));
            for (i, lever) in moment_levers(r, axis).into_iter().enumerate() {
                if i != axis && lever != 0.0 {
                    expected.push((load_id("case:s", "component:valve", axis, Part::Moment(i)), lever * f));
                }
            }
        }
        let observed = component_values(&loads);
        assert_eq!(observed.len(), 9);
        for ((oid, ov), (eid, ev)) in observed.iter().zip(&expected) {
            assert_eq!(oid, eid);
            assert_eq!(ov.to_bits(), ev.to_bits(), "{oid}");
        }
        // The README's exact wrench, (705, 822.5, 940) N; M_x = 799/80 N m in XYZ.
        assert_eq!(observed[0].1, 705.0);
        // Member loads first, then the component items.
        let first_component = ids.iter().position(|id| id.contains(":component:")).unwrap();
        assert!(ids[..first_component].iter().all(|id| id.contains(":element-")));
    }

    /// A zero g-factor keeps the replacement: zero force and moments, and no
    /// member load on BC; no range check applies.
    #[test]
    fn a_zero_factor_keeps_the_replacement() {
        let (loads, diagnostics) = generated(&document(2350.0, [-0.175, 0.12, 0.125], [Some(0.0), None, None], false));
        assert!(range_refusals(&diagnostics).is_empty());
        let values = component_values(&loads);
        assert_eq!(values.len(), 3, "the force and one moment per nonzero transverse coordinate: {values:?}");
        assert!(values.iter().all(|(_, v)| *v == 0.0));
        assert!(!loads.iter().any(|l| l.load_id.ends_with("element-1")));
    }

    /// The range refusals by name, with nothing generated; a finite nonzero
    /// subnormal force passes its check.
    #[test]
    fn range_refusals_and_the_subnormal_force() {
        let r = [-0.175, 0.12, 0.125];
        for (what, w, cog, k, part) in [
            ("force overflow", 1.0e300, r, 1.0e10, ":global-x:force"),
            ("moment overflow", 1.0e300, [0.0, 1.0e20, 0.0], 1.0e-2, ":global-x:moment-z"),
            ("force underflow", 1.0e-300, r, 1.0e-30, ":global-x:force"),
            ("moment underflow", 1.0e-300, [0.0, 1.0e-30, 0.0], 0.3, ":global-x:moment-z"),
        ] {
            let (loads, diagnostics) = generated(&document(w, cog, [Some(k), None, None], false));
            let refusals = range_refusals(&diagnostics);
            assert_eq!(refusals.len(), 1, "{what}: {diagnostics:?}");
            assert!(refusals[0].ends_with(part), "{what}: {refusals:?}");
            assert!(!loads.iter().any(|l| l.load_id.contains(":generated:")), "{what}: nothing generated");
        }
        // W = 1e-310 N (subnormal), k = 0.3: F = fl(3e-311), subnormal and nonzero.
        let (loads, diagnostics) = generated(&document(1.0e-310, r, [Some(0.3), None, None], false));
        assert!(range_refusals(&diagnostics).is_empty(), "{diagnostics:?}");
        let values = component_values(&loads);
        assert!(values[0].1 != 0.0 && !values[0].1.is_normal(), "{values:?}");
    }

    /// Decision 2: a nonzero centre-of-gravity coordinate that normalizes to
    /// zero blocks before fallback (here the weight is absent, which would
    /// otherwise select fallback), read through the self-weight plan's own
    /// normalization of the entered value.
    #[test]
    fn a_centre_of_gravity_that_normalizes_to_zero_blocks() {
        let mut wire = document(2350.0, [0.0, 1.0e-321, 0.0], [Some(0.3), None, None], false);
        wire["model"]["components"][0]["geometry"]["center_of_gravity"]["unit"] = json!("mm");
        wire["model"]["components"][0]["geometry"].as_object_mut().unwrap().remove("weight");
        let (loads, diagnostics) = generated(&wire);
        let refusals = range_refusals(&diagnostics);
        assert_eq!(refusals.len(), 1, "{diagnostics:?}");
        assert!(refusals[0].ends_with(":cog"));
        assert!(!loads.iter().any(|l| l.load_id.contains(":generated:")));
    }

    /// VER-005's paired cases: the self-weight wrench and the scaled seismic
    /// terms differ in representability, in both directions, by overflow and
    /// by underflow. Where the seismic terms are in range the component load
    /// is generated; where they are not, the case is refused by name with no
    /// generated load.
    #[test]
    fn paired_representability_in_both_directions() {
        // (what, W, r, k_x, self-weight refused, seismic refused)
        let pairs: [(&str, f64, [f64; 3], f64, bool, bool); 4] = [
            ("overflow: weight in range, seismic out", 1.0e300, [0.0, 0.12, 0.125], 1.0e10, false, true),
            ("overflow: weight out, seismic in", 1.0e300, [0.0, 1.0e10, 0.125], 1.0e-20, true, false),
            ("underflow: weight in range, seismic out", 1.0e-300, [0.0, 1.0e-20, 0.125], 1.0e-10, false, true),
            ("underflow: weight out, seismic in", 1.0e-300, [0.0, 1.0e-30, 0.125], 1.0e10, true, false),
        ];
        for (what, w, r, k, weight_refused, seismic_refused) in pairs {
            let wire = document(w, r, [Some(k), None, None], false);
            assert_eq!(self_weight_range_refused(&wire), weight_refused, "{what}: self-weight plan");
            let (loads, diagnostics) = generated(&wire);
            assert_eq!(!range_refusals(&diagnostics).is_empty(), seismic_refused, "{what}: {diagnostics:?}");
            let components = component_values(&loads);
            if seismic_refused {
                assert!(loads.iter().all(|l| !l.load_id.contains(":generated:")), "{what}");
            } else {
                // The force, and the moment about z from r_y (r_x = 0 forms none).
                assert_eq!(components.len(), 3, "{what}: {components:?}");
                assert_eq!(components[0].1.to_bits(), (k * w).to_bits(), "{what}");
                assert!(components.iter().all(|(_, v)| v.is_finite() && *v != 0.0), "{what}: {components:?}");
            }
        }
    }

    /// Issue 450's first test of the nodal branch: a deliberately wrong
    /// prepared value at the right degree of freedom, with the operands the
    /// certified producer reads from the model unchanged. In both solver
    /// modes, on the indeterminate model (A and D anchored), a doubled force
    /// or moment, one a single unit in the last place high and one so large
    /// that assembly would overflow are each refused by name
    /// (EQUIVALENT_STATIC_COMPONENT_VALUE_MISMATCH) as the only blocking
    /// finding, with no rows and no SOLVER_SYSTEM_BLOCKED: the binding
    /// compares the prepared value's bits with the rule's, and the case stops
    /// before its ledger is finished.
    #[test]
    fn a_wrong_prepared_value_is_refused_by_name() {
        let force = load_id("case:s", "component:valve", 0, Part::Force);
        let moment = load_id("case:s", "component:valve", 0, Part::Moment(2));
        let wire = document(2350.0, [-0.175, 0.12, 0.125], [Some(0.3), None, None], true);
        for mode in MODES {
            prepared_mutant::disarm();
            let plain = serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap()).unwrap();
            assert_eq!(plain["status"]["mechanics"], "MECHANICS_SOLVED");
            assert_eq!(plain["numerical_quality"]["status"], "checks_passed", "{mode:?}");
            assert!(plain["diagnostics"].as_array().unwrap().iter().all(|d| d["code"] != VALUE_MISMATCH));
            type Change = fn(f64) -> f64;
            let doubled: Change = |v| 2.0 * v;
            let ulp: Change = |v| f64::from_bits(v.to_bits() + 1);
            let huge: Change = |v| f64::MAX.copysign(v);
            for (what, target, change) in [
                ("force doubled", &force, doubled),
                ("force one ulp high", &force, ulp),
                ("force at the largest finite value", &force, huge),
                ("moment doubled", &moment, doubled),
                ("moment one ulp high", &moment, ulp),
                ("moment at the largest finite value", &moment, huge),
            ] {
                let id = target.clone();
                prepared_mutant::arm(Box::new(move |load, value| (load == id).then(|| change(value))));
                let out = serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap()).unwrap();
                prepared_mutant::disarm();
                assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{what} {mode:?}");
                assert!(out["results"].as_array().unwrap().is_empty(), "{what} {mode:?}");
                let found: Vec<&Value> = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == VALUE_MISMATCH).collect();
                assert_eq!(found.len(), 1, "{what} {mode:?}");
                assert_eq!(found[0]["severity"], "blocking");
                assert_eq!(found[0]["affected_refs"], json!(["case:s", target]), "{what}");
                let blocking: Vec<&Value> = out["diagnostics"].as_array().unwrap().iter()
                    .filter(|d| d["severity"] == "blocking" || d["severity"] == "failure").collect();
                assert_eq!(blocking.len(), 1, "{what} {mode:?}: the mismatch is the only blocking finding: {blocking:?}");
                assert!(out["diagnostics"].as_array().unwrap().iter().all(|d| d["code"] != "SOLVER_SYSTEM_BLOCKED"), "{what} {mode:?}");
            }
        }
    }

    /// Where the ledger is built: the component's force and moments are exact
    /// input terms carrying the rule's value at the component node's degrees of
    /// freedom (C: the force at global X, the moments at RY and RZ), with no
    /// formation record. A one-ulp prepared value is not pushed: the refusal
    /// names it and the ledger holds no term of it. The rule is recomputed
    /// from the plan's converted operands.
    #[test]
    fn the_ledger_holds_the_rule_s_value_as_an_exact_term() {
        use open_pipe_stress_frame_kernel::load_ledger::{ForceTermKind, LoadLedger};
        let force = load_id("case:s", "component:valve", 0, Part::Force);
        // The offset entered in millimetres: the rule reads the converted
        // lever (m), as the generator does.
        let mut wire = document(2350.0, [-0.175, 0.12, 0.125], [Some(0.3), None, None], true);
        wire["model"]["components"][0]["geometry"]["center_of_gravity"] = json!({"x": -175.0, "y": 120.0, "z": 125.0, "unit": "mm"});
        let model = typed(&wire);
        let ledger_of = |loads: &[PrimitiveLoad]| {
            let application = open_pipe_stress_primitive_loads::prepare_loads(model.nodes.len(), model.pipe_segments.len(), loads);
            let mut ledger = LoadLedger::new();
            let mut diagnostics = Vec::new();
            crate::certified_loads::nodal(&mut ledger, &model, &application, "case:s", model.nodes.len(), &mut diagnostics);
            (ledger, diagnostics)
        };
        let (loads, generation) = generated(&wire);
        assert!(generation.iter().all(|d| d.severity != "blocking"), "{generation:?}");
        let (ledger, diagnostics) = ledger_of(&loads);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        let components = component_values(&loads);
        assert_eq!(components.len(), 3, "the force and two moments: {components:?}");
        // Node C is the third node: its degrees of freedom are 12..18.
        let dofs = [(force.clone(), 2 * 6), (load_id("case:s", "component:valve", 0, Part::Moment(1)), 2 * 6 + 4),
            (load_id("case:s", "component:valve", 0, Part::Moment(2)), 2 * 6 + 5)];
        for (id, value) in &components {
            let rule = rule_value(&model, "case:s", id).expect("a component primitive");
            assert_eq!(rule.0.to_bits(), value.to_bits(), "{id}");
            let terms: Vec<_> = ledger.terms().iter().filter(|t| &t.source == id).collect();
            assert_eq!(terms.len(), 1, "{id}");
            let dof = dofs.iter().find(|(d, _)| d == id).unwrap_or_else(|| panic!("{id}")).1;
            assert_eq!(terms[0].dof, dof, "{id}: at node C");
            assert!(matches!(terms[0].kind, ForceTermKind::Term(v) if v.to_bits() == value.to_bits()), "{id}: {:?}", terms[0].kind);
        }
        let assembled = ledger.finish(model.nodes.len() * 6).unwrap();
        assert!(assembled.load_primitives().is_empty(), "no formed product for a component term");
        assert_eq!(rule_value(&model, "case:s", &force).unwrap().0.to_bits(), (0.3f64 * 2350.0).to_bits());
        // A one-ulp prepared force is not pushed, and the refusal names it; the
        // moments, which are the rule's, are still pushed.
        let target = force.clone();
        prepared_mutant::arm(Box::new(move |load, value| (load == target).then(|| f64::from_bits(value.to_bits() + 1))));
        let (loads, _) = generated(&wire);
        prepared_mutant::disarm();
        assert!(loads.iter().any(|l| l.load_id == force && l.magnitude.unwrap().value.to_bits() == (0.3f64 * 2350.0).to_bits() + 1),
            "the prepared value carries the mutant");
        let (ledger, diagnostics) = ledger_of(&loads);
        assert!(ledger.terms().iter().all(|t| t.source != force), "the mismatched term is not in the ledger");
        assert_eq!(ledger.terms().iter().filter(|t| t.source.contains(":component:")).count(), 2);
        assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
        assert_eq!((diagnostics[0].code.as_str(), diagnostics[0].severity.as_str()), (VALUE_MISMATCH, "blocking"));
        assert_eq!(diagnostics[0].affected_refs, vec!["case:s".to_string(), force.clone()]);
        // Every other load: no rule.
        assert!(rule_value(&model, "case:s", "case:s:generated:seismic:global-x:element-0").is_none());
        assert!(rule_value(&model, "case:other", &force).is_none());
    }

    #[test]
    fn identifiers_are_distinct_from_member_loads_and_length_prefixed() {
        assert_eq!(load_id("case:s", "component:v", 0, Part::Force), "case:s:generated:seismic:global-x:component:11:component:v:force");
        assert_eq!(load_id("case:s", "component:v", 2, Part::Moment(1)), "case:s:generated:seismic:global-z:component:11:component:v:moment:y");
        // A member load of the same case and axis never has the `component` tag.
        assert!(!"case:s:generated:seismic:global-x:element-0".contains(":component:"));
        // Two components whose ids share a prefix stay distinct.
        assert_ne!(load_id("c", "a:b", 0, Part::Force), load_id("c", "a", 0, Part::Force));
    }

    #[test]
    fn levers_give_r_cross_f_for_each_axis() {
        // r × (f e_x) = (0, z f, -y f); r × (f e_y) = (-z f, 0, x f); r × (f e_z) = (y f, -x f, 0).
        let r = [2.0, 3.0, 5.0];
        assert_eq!(moment_levers(r, 0), [0.0, 5.0, -3.0]);
        assert_eq!(moment_levers(r, 1), [-5.0, 0.0, 2.0]);
        assert_eq!(moment_levers(r, 2), [3.0, -2.0, 0.0]);
        // Negation is exact, so fl(-(y f)) == fl((-y) f) bit for bit, signed zeros included.
        for (y, f) in [(0.1, 0.7), (0.0, -1.0), (-0.0, 1.0), (1e-300, 1e-300)] {
            assert_eq!((-(y * f) as f64).to_bits(), ((-y) * f as f64).to_bits());
        }
    }
}
