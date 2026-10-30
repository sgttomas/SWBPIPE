//! The kernel lane (T3 D1 §4.10; plan §5): each R1 case through W1a's public
//! entry, `solve_case`, with no budget limit reachable (u64::MAX for the case
//! and the invocation, recorded), then every R1 row judged (plan §6), the
//! floor check (§7), the discrimination check (§8) and the class
//! correspondence of the not-covered rows (§7.2).
use crate::cases::{Case, ControlKind, Model, Target};
use crate::compare::{judge, ControlTally, Observed, Tally, Verdict};
use crate::exact::{self, Exact};
use crate::floor;
use crate::records;
use crate::sha256::sha256_hex;
use open_pipe_stress_frame_kernel::structural::retained_api::{
    solve_case, Binary64Outcome, CaseLimit, CaseOutcome, Component, End, InvocationMeter,
    PrimitiveSource, PublishedRow, QuantityId, Refusal, RowClass, SourceParts,
};
use serde_json::Value;
use std::collections::BTreeMap;

/// One case's run.
#[derive(Clone, Debug)]
pub struct CaseRun {
    pub id: String,
    pub family: String,
    /// `selected`, `refused`, `unresolved` or `source_refused`.
    pub outcome: String,
    pub selected_precision: Option<u32>,
    pub tally: Tally,
    /// Every failure, with its reason (a failure on a covered row is a stop).
    pub failures: Vec<String>,
    /// The not-covered keys the floor check found, in row order.
    pub not_covered: Vec<String>,
    /// Not-covered rows whose published row is not `absolute_verified` or
    /// unpublishable (plan §7.2).
    pub class_mismatches: Vec<String>,
    /// Not-covered rows whose published row is `input_derived`: a displacement
    /// at a restrained DOF, published as its exact prescription (D1 §4.1.6.1
    /// rule 2a). Listed apart from the mismatches (A1 finding for ROOT).
    pub input_derived_not_covered: Vec<String>,
    pub controls: ControlTally,
    /// The deterministic per-case record (plan §12).
    pub record: Option<Value>,
    /// The published rows, for the invariance checks.
    pub published: BTreeMap<QuantityId, PublishedRow>,
}

/// The discrimination check of a case's value controls, against its reference
/// rows (plan §8). Outcome controls are decided by `run_case`.
pub fn value_controls(case: &Case) -> ControlTally {
    let rows: BTreeMap<&str, &crate::cases::Row> =
        case.rows.iter().map(|r| (r.key.as_str(), r)).collect();
    let mut t = ControlTally::default();
    for c in &case.controls {
        let ControlKind::Value(values) = &c.kind else {
            continue;
        };
        let fails = values.iter().any(|(key, v)| {
            let row = rows
                .get(key.as_str())
                .unwrap_or_else(|| panic!("{}: control {} names {key}", case.id, c.id));
            let exp = Exact::parse_decimal(&row.expected).unwrap();
            let scale = Exact::parse_decimal(case.scale_of(row)).unwrap();
            // A control's value (a magnitude too) is compared as a value.
            let obs = Exact::parse_decimal(v).unwrap();
            !exact::predicate(&obs, &exp, &scale)
        });
        let name = format!("{}:{}", case.id, c.id);
        match (c.discriminates, fails) {
            (true, true) => t.discriminated += 1,
            (true, false) => t.undiscriminated.push(name),
            (false, false) => t.non_discriminating += 1,
            (false, true) => {
                t.non_discriminating += 1;
                t.unexpectedly_failing.push(name);
            }
        }
    }
    t
}

fn value_of(rows: &BTreeMap<QuantityId, PublishedRow>, id: QuantityId) -> Result<f64, String> {
    match rows.get(&id).map(|r| r.value) {
        Some(Binary64Outcome::Normal(v)) => Ok(v),
        Some(Binary64Outcome::Subnormal { value, .. }) => Ok(value),
        Some(Binary64Outcome::Underflow { negative }) => Ok(if negative { -0.0 } else { 0.0 }),
        Some(Binary64Outcome::Overflow { .. }) => Err(format!("{id:?} overflowed")),
        None => Err(format!("{id:?} not published")),
    }
}

fn end(member: u32, j_end: bool, component: Component) -> QuantityId {
    QuantityId::EndAction {
        member,
        end: if j_end { End::J } else { End::I },
        component,
    }
}

/// The published rows a target is formed from.
fn sources(target: Target) -> Vec<QuantityId> {
    match target {
        Target::Displacement(d) => vec![QuantityId::Displacement(d)],
        Target::Reaction(d) => vec![QuantityId::Reaction(d)],
        Target::Spring { id, component } => vec![QuantityId::SpringAction {
            spring: id,
            component,
        }],
        Target::DirectionalSpring { id, component } => {
            vec![QuantityId::DirectionalSpringAction {
                spring: id,
                component,
            }]
        }
        Target::StructuralZero => vec![],
        Target::Axial(m) | Target::Extension(m) => vec![end(m, true, Component::Ux)],
        Target::Torque(m) | Target::Twist(m) => vec![end(m, true, Component::Rx)],
        Target::BendingEnd { member, j_end } => vec![
            end(member, j_end, Component::Ry),
            end(member, j_end, Component::Rz),
        ],
        Target::BendingStation(s) => vec![
            QuantityId::StationAction {
                station: s,
                component: Component::Ry,
            },
            QuantityId::StationAction {
                station: s,
                component: Component::Rz,
            },
        ],
    }
}

/// k_t and k_a of a member (DESIGN.md §4.10, with C1's pre-scaling).
pub fn member_coefficients(model: &Model, member: u32) -> (f64, f64) {
    let m = model.members.iter().find(|m| m.id == member).unwrap();
    let l = floor::member_length(
        model.nodes[m.node_i as usize],
        model.nodes[m.node_j as usize],
    );
    (
        floor::coefficient(m.shear_modulus, m.torsion_constant, l),
        floor::coefficient(m.elastic_modulus, m.area, l),
    )
}

/// What the kernel published for a target.
pub fn observe(
    target: Target,
    rows: &BTreeMap<QuantityId, PublishedRow>,
    model: &Model,
) -> Observed {
    let unavailable = |e: String| Observed::Unavailable(e);
    match target {
        Target::StructuralZero => Observed::StructuralZero,
        Target::BendingEnd { .. } | Target::BendingStation(_) => {
            let ids = sources(target);
            match (value_of(rows, ids[0]), value_of(rows, ids[1])) {
                (Ok(y), Ok(z)) => Observed::Magnitude(y, z),
                (Err(e), _) | (_, Err(e)) => unavailable(e),
            }
        }
        Target::Twist(m) | Target::Extension(m) => {
            let (kt, ka) = member_coefficients(model, m);
            match value_of(rows, sources(target)[0]) {
                Ok(v) => Observed::Value(if matches!(target, Target::Twist(_)) {
                    v / kt
                } else {
                    v / ka
                }),
                Err(e) => unavailable(e),
            }
        }
        _ => match value_of(rows, sources(target)[0]) {
            Ok(v) => Observed::Value(v),
            Err(e) => unavailable(e),
        },
    }
}

/// Runs one case with a model (the committed one, or a large model file).
pub fn run_case_with(case: &Case, model: &Model) -> CaseRun {
    run_parts(case, model, model.source_parts())
}

pub fn run_case(case: &Case) -> CaseRun {
    let model = case.model.as_ref().expect("a CI-scale case");
    run_case_with(case, model)
}

/// Runs a case on given parts (the list-permutation checks pass permuted parts).
pub fn run_parts(case: &Case, model: &Model, parts: SourceParts) -> CaseRun {
    let mut failures = Vec::new();
    let mut tally = Tally::default();
    let mut controls = value_controls(case);
    let mut published = BTreeMap::new();
    let mut not_covered = Vec::new();
    let mut class_mismatches = Vec::new();
    let mut input_derived_not_covered = Vec::new();
    let source = match PrimitiveSource::new(parts) {
        Ok(s) => s,
        Err(e) => {
            // No kernel outcome: every row fails; for a mechanism the named
            // source refusal is recorded (C10).
            for row in &case.rows {
                tally.add(&Verdict::Fail(format!("source refused: {e:?}")));
                failures.push(format!("{}: {} source refused: {e:?}", case.id, row.key));
            }
            if !case.refuse {
                failures.push(format!("{}: source refused: {e:?}", case.id));
            }
            decide_outcome_controls(case, false, &mut controls);
            let record = Some(records::source_refused_record(
                case,
                &format!("{e:?}"),
                &tally,
            ));
            return CaseRun {
                id: case.id.clone(),
                family: case.family.clone(),
                outcome: "source_refused".into(),
                selected_precision: None,
                tally,
                failures,
                not_covered,
                class_mismatches,
                input_derived_not_covered,
                controls,
                record,
                published,
            };
        }
    };
    let encoding = source.encoding();
    if sha256_hex(&encoding) != case.k4src_sha256 {
        failures.push(format!(
            "{}: K4SRC sha256 differs from the generator's --model path",
            case.id
        ));
    }
    let mut meter = InvocationMeter::new(u64::MAX);
    let outcome = solve_case(source, CaseLimit::new(u64::MAX), &mut meter);
    let (label, selected_precision) = match &outcome {
        CaseOutcome::Selected(s) => ("selected", Some(s.selected_precision())),
        CaseOutcome::Refused { .. } => ("refused", None),
        CaseOutcome::Unresolved { .. } => ("unresolved", None),
    };
    if meter.exhausted() {
        failures.push(format!("{}: the invocation meter was exhausted", case.id));
    }
    let selected = matches!(outcome, CaseOutcome::Selected(_));
    decide_outcome_controls(case, selected, &mut controls);
    // ROOT's ruling on I17's A1 stop: a listed case must end unresolved with
    // no rows; leaving the list is a failure, and so is joining it.
    let listed = crate::cases::expected_unresolved().contains(&case.id);
    if listed && !matches!(outcome, CaseOutcome::Unresolved { .. }) {
        failures.push(format!(
            "{}: on the expected-unresolved list, but {}",
            case.id,
            records::outcome_text(&outcome)
        ));
    }
    if case.refuse {
        if let CaseOutcome::Selected(s) = &outcome {
            failures.push(format!(
                "{}: a mechanism was published ({} rows)",
                case.id,
                s.publish().rows.len()
            ));
        }
    } else if let CaseOutcome::Selected(s) = &outcome {
        for r in &s.publish().rows {
            published.insert(r.id, r.clone());
        }
        let bodies = model.bodies();
        if bodies.iter().any(|&b| b != 0) {
            failures.push(format!("{}: more than one body", case.id));
        }
        let scales = floor::scales(case, model);
        for row in &case.rows {
            let exp = Exact::parse_decimal(&row.expected).unwrap();
            let scale = Exact::parse_decimal(case.scale_of(row)).unwrap();
            let covered = exact::covered(&exp, &scale, floor::row_scale(row, &scales));
            let verdict = match model.resolve(&row.key) {
                Ok(target) => {
                    let v = judge(&observe(target, &published, model), &exp, &scale, covered);
                    if !covered {
                        not_covered.push(row.key.clone());
                        for id in sources(target) {
                            let restrained = matches!(id, QuantityId::Displacement(d)
                                if model.constraints.contains(&(d.node, d.component.index())));
                            match published.get(&id).map(|r| &r.class) {
                                Some(RowClass::AbsoluteVerified { .. })
                                | Some(RowClass::Unpublishable) => {}
                                Some(RowClass::InputDerived) if restrained => {
                                    input_derived_not_covered
                                        .push(format!("{}:{}", case.id, row.key))
                                }
                                other => class_mismatches.push(format!(
                                    "{}: {} ({id:?} is {other:?})",
                                    case.id, row.key
                                )),
                            }
                        }
                    }
                    v
                }
                Err(e) => Verdict::Fail(e),
            };
            if let Verdict::Fail(why) = &verdict {
                failures.push(format!("{}: {} ({}) {why}", case.id, row.key, row.expected));
            }
            tally.add(&verdict);
        }
    } else if listed && matches!(outcome, CaseOutcome::Unresolved { .. }) {
        for _row in &case.rows {
            tally.add(&Verdict::ExpectedUnresolved);
        }
    } else {
        for _row in &case.rows {
            tally.add(&Verdict::Fail(format!("case {label}")));
        }
        failures.push(format!(
            "{}: not selected: {}",
            case.id,
            records::outcome_text(&outcome)
        ));
    }
    let record = match records::case_record(case, &outcome, &tally, &meter) {
        Ok(record) => Some(record),
        Err(fault) => {
            failures.push(format!("{}: retained work accounting {fault}", case.id));
            None
        }
    };
    CaseRun {
        id: case.id.clone(),
        family: case.family.clone(),
        outcome: label.into(),
        selected_precision,
        tally,
        failures,
        not_covered,
        class_mismatches,
        input_derived_not_covered,
        controls,
        record,
        published,
    }
}

/// Outcome controls (plan §8): on a mechanism, each describes publishing a
/// solution, so the refusal discriminates it; on a stable model (the RF-MECH
/// companion), NC-FALSE-MECHANISM is a refusal, so the selection does.
fn decide_outcome_controls(case: &Case, selected: bool, t: &mut ControlTally) {
    for c in &case.controls {
        if let ControlKind::Outcome(_) = c.kind {
            let discriminated = if case.refuse { !selected } else { selected };
            match (c.discriminates, discriminated) {
                (true, true) => t.discriminated += 1,
                (true, false) => t.undiscriminated.push(format!("{}:{}", case.id, c.id)),
                (false, _) => t.non_discriminating += 1,
            }
        }
    }
}

/// A refusal's description without platform-dependent binary64 values (the
/// witness's rigid parameters come from FK's geometry, which uses `hypot`).
pub fn refusal_text(r: &Refusal) -> String {
    match r {
        Refusal::MechanismWitnessed { body, .. } => {
            format!("MechanismWitnessed {{ body: {body} }}")
        }
        other => format!("{other:?}"),
    }
}
