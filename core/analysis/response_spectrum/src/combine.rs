//! The combinations of supplied signed contributions (CLM-001; REQ-003, REQ-004; unit S4).
use crate::contract::{
    ClusterClass, ClusterIndex, ClusterStatement, ClusterSum, Combination, CombinationRequest, CombinedRow, Direction,
    DirectionalPart, DirectionalRule, Magnitude, MissingMass, ModalRule, ModeIndex, RigidRule,
};
use crate::refusal::{Refusal, RefusalCode};
use crate::dyadic::{Dyadic, Interval};
use crate::rules::{self, Certified, CorrelationTable, ModalForm, Uncertain, PRECISION_LADDER};
use crate::table::MAX_SOLVE_DAMPING_RATIO;
use std::collections::{BTreeMap, BTreeSet};

const REQUEST: &str = "request";

fn refuse(code: RefusalCode, object: &str, field: Option<&str>, detail: String) -> Refusal {
    Refusal::new(code, object, field, detail)
}

/// A supplied cluster, with its modes in ascending order.
struct Cluster {
    index: ClusterIndex,
    modes: Vec<usize>,
    standing: f64,
    class: ClusterClass,
}

fn check_choices(request: &CombinationRequest, refusals: &mut Vec<Refusal>) {
    let zeta = request.damping_ratio;
    if !(zeta.is_finite() && (0.0..=MAX_SOLVE_DAMPING_RATIO).contains(&zeta)) {
        refusals.push(refuse(RefusalCode::DampingOutOfRange, REQUEST, Some("damping_ratio"),
            format!("{zeta} is outside 0 ≤ ζ ≤ {MAX_SOLVE_DAMPING_RATIO}")));
    } else if request.modal_rule == ModalRule::Cqc && zeta == 0.0 {
        refusals.push(refuse(RefusalCode::CqcDampingUnsupported, REQUEST, Some("modal_combination"),
            "CQC is not defined at a damping ratio of 0".into()));
    }
    if let MissingMass::Applied { rigid_frequency_hz: fr, .. } = request.missing_mass {
        if !(fr.is_finite() && fr > 0.0) {
            refusals.push(refuse(RefusalCode::RigidFrequencyInvalid, REQUEST, Some("rigid_frequency_hz"),
                format!("{fr} is not a positive finite frequency in Hz")));
        }
    }
    if request.directions.is_empty() {
        refusals.push(refuse(RefusalCode::DirectionInvalid, REQUEST, Some("directions"), "no direction is requested".into()));
    }
    let mut seen = BTreeSet::new();
    for d in &request.directions {
        if !seen.insert(*d) {
            refusals.push(refuse(RefusalCode::DirectionInvalid, d.as_str(), Some("directions"),
                format!("direction {d} is requested more than once")));
        }
    }
}

/// The modal result's structure: modes 1..n in ascending frequency, clusters chained from 1
/// over adjacent modes. Nothing is inferred; a breach is refused.
fn check_modes(request: &CombinationRequest, refusals: &mut Vec<Refusal>) -> bool {
    let before = refusals.len();
    let invalid = |object: String, field: &str, detail: String| {
        refuse(RefusalCode::ModalMetadataInvalid, &object, Some(field), detail)
    };
    // REQ-002 permits an empty extracted set. With correction off its modal sums are zero;
    // the existing missing-mass admission is unchanged by this empty-set repair.
    if request.modes.is_empty() && matches!(request.missing_mass, MissingMass::Applied { .. }) {
        refusals.push(invalid(REQUEST.into(), "modes", "no extracted mode is supplied".into()));
    }
    for (i, m) in request.modes.iter().enumerate() {
        let object = m.mode.to_string();
        if m.mode.0 as usize != i + 1 {
            refusals.push(invalid(object.clone(), "mode", format!("entry {} has mode index {}, not {}", i + 1, m.mode.0, i + 1)));
        }
        if !(m.frequency_hz.is_finite() && m.frequency_hz > 0.0) {
            refusals.push(invalid(object.clone(), "frequency_hz", format!("{} is not a positive finite frequency", m.frequency_hz)));
        } else if i > 0 {
            let previous = request.modes[i - 1].frequency_hz;
            if previous.is_finite() && previous > 0.0 && m.frequency_hz < previous {
                refusals.push(invalid(object.clone(), "frequency_hz", format!("{} Hz is below the previous mode's {previous} Hz", m.frequency_hz)));
            }
        }
        let expected_ok = if i == 0 {
            m.cluster.0 == 1
        } else {
            let previous = request.modes[i - 1].cluster.0;
            m.cluster.0 == previous || m.cluster.0 == previous.wrapping_add(1)
        };
        if !expected_ok {
            refusals.push(invalid(object, "cluster", format!("{} does not chain from the previous mode's cluster (clusters are chained from 1 over adjacent modes)", m.cluster)));
        }
    }
    refusals.len() == before
}

fn check_rows(request: &CombinationRequest, refusals: &mut Vec<Refusal>) {
    if request.rows.is_empty() {
        refusals.push(refuse(RefusalCode::RowInvalid, REQUEST, Some("rows"), "no row is supplied".into()));
    }
    let mut seen = BTreeSet::new();
    for row in &request.rows {
        let object = row.object();
        if object.is_empty() || object.contains('.') || object.chars().any(char::is_whitespace) {
            refusals.push(refuse(RefusalCode::RowInvalid, &row.key(), Some("object"),
                format!("`{object}` is not an object id (empty, or containing `.` or white space)")));
        }
        if !seen.insert(row.key()) {
            refusals.push(refuse(RefusalCode::RowInvalid, &row.key(), Some("rows"), "the row is supplied more than once".into()));
        }
    }
}

fn clusters(request: &CombinationRequest) -> Vec<Cluster> {
    let mut out: Vec<Cluster> = Vec::new();
    for (i, m) in request.modes.iter().enumerate() {
        match out.last_mut() {
            Some(c) if c.index == m.cluster => c.modes.push(i),
            _ => out.push(Cluster { index: m.cluster, modes: vec![i], standing: m.frequency_hz, class: ClusterClass::Periodic }),
        }
    }
    if let MissingMass::Applied { rigid_frequency_hz: fr, .. } = request.missing_mass {
        for c in &mut out {
            // Periodic when the cluster's lowest frequency is below f_r (CLM-001, Rigid response).
            c.class = if c.standing < fr { ClusterClass::Periodic } else { ClusterClass::Rigid };
        }
    }
    out
}

fn check_values(values: &[f64], rows: usize, object: &str, code: RefusalCode, request: &CombinationRequest,
    refusals: &mut Vec<Refusal>) -> bool {
    if values.len() != rows {
        refusals.push(refuse(code, object, Some("values"), format!("{} values for {rows} rows", values.len())));
        return false;
    }
    let mut ok = true;
    for (v, row) in values.iter().zip(&request.rows) {
        if !v.is_finite() {
            refusals.push(refuse(code, object, Some(&row.key()), format!("{v} is not a finite value")));
            ok = false;
        }
    }
    ok
}

/// Validates the contributions and rigid responses; returns the values indexed by
/// (direction, mode position) and by direction.
fn check_contributions<'r>(request: &'r CombinationRequest, clusters: &[Cluster], refusals: &mut Vec<Refusal>)
    -> (BTreeMap<(Direction, usize), &'r [f64]>, BTreeMap<Direction, &'r [f64]>) {
    let rows = request.rows.len();
    let requested: BTreeSet<Direction> = request.directions.iter().copied().collect();
    let mut class_of_mode = vec![ClusterClass::Periodic; request.modes.len()];
    for c in clusters {
        for &m in &c.modes {
            class_of_mode[m] = c.class;
        }
    }
    let mut periodic = BTreeMap::new();
    for entry in &request.periodic {
        let object = format!("{} {}", entry.mode, entry.direction);
        let position = (entry.mode.0 as usize).checked_sub(1).filter(|&p| p < request.modes.len());
        let Some(position) = position else {
            refusals.push(refuse(RefusalCode::ContributionInvalid, &object, Some("mode"),
                format!("{} is not an extracted mode", entry.mode)));
            continue;
        };
        if !requested.contains(&entry.direction) {
            refusals.push(refuse(RefusalCode::ContributionInvalid, &object, Some("direction"),
                format!("direction {} is not requested", entry.direction)));
            continue;
        }
        if class_of_mode[position] == ClusterClass::Rigid {
            refusals.push(refuse(RefusalCode::ContributionNotPeriodic, &object, Some("mode"),
                format!("{} belongs to rigid {}; an extracted rigid mode is never combined as periodic",
                    entry.mode, request.modes[position].cluster)));
            continue;
        }
        if periodic.contains_key(&(entry.direction, position)) {
            refusals.push(refuse(RefusalCode::ContributionInvalid, &object, Some("mode"), "supplied more than once".into()));
            continue;
        }
        if check_values(&entry.values, rows, &object, RefusalCode::ContributionInvalid, request, refusals) {
            periodic.insert((entry.direction, position), entry.values.as_slice());
        } else {
            periodic.insert((entry.direction, position), &[][..]);
        }
    }
    for d in &request.directions {
        for (position, m) in request.modes.iter().enumerate() {
            if class_of_mode[position] == ClusterClass::Periodic && !periodic.contains_key(&(*d, position)) {
                refusals.push(refuse(RefusalCode::ContributionInvalid, &format!("{} {d}", m.mode), Some("mode"),
                    format!("no contribution is supplied for periodic {} in direction {d}", m.mode)));
            }
        }
    }
    let mut rigid = BTreeMap::new();
    match request.missing_mass {
        MissingMass::NotApplied => {
            for entry in &request.rigid {
                refusals.push(refuse(RefusalCode::RigidResponseInvalid, entry.direction.as_str(), Some("rigid_responses"),
                    "a rigid response is supplied without the missing-mass correction".into()));
            }
        }
        MissingMass::Applied { .. } => {
            let periodic_clusters: Vec<ClusterIndex> =
                clusters.iter().filter(|c| c.class == ClusterClass::Periodic).map(|c| c.index).collect();
            for entry in &request.rigid {
                let object = entry.direction.as_str();
                if !requested.contains(&entry.direction) {
                    refusals.push(refuse(RefusalCode::RigidResponseInvalid, object, Some("direction"),
                        format!("direction {object} is not requested")));
                    continue;
                }
                if rigid.contains_key(&entry.direction) {
                    refusals.push(refuse(RefusalCode::RigidResponseInvalid, object, Some("direction"), "supplied more than once".into()));
                    continue;
                }
                let mut left_out = entry.periodic_clusters_left_out.clone();
                left_out.sort();
                let repeated = left_out.windows(2).any(|w| w[0] == w[1]);
                if repeated || left_out != periodic_clusters {
                    let list = |v: &[ClusterIndex]| v.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
                    refusals.push(refuse(RefusalCode::RigidResponsePremiseMismatch, object, Some("periodic_clusters_left_out"),
                        format!("the rigid load left out [{}] as periodic; the request classifies [{}] as periodic",
                            list(&entry.periodic_clusters_left_out), list(&periodic_clusters))));
                }
                if check_values(&entry.values, rows, object, RefusalCode::RigidResponseInvalid, request, refusals) {
                    rigid.insert(entry.direction, entry.values.as_slice());
                } else {
                    rigid.insert(entry.direction, &[][..]);
                }
            }
            for d in &request.directions {
                if !rigid.contains_key(d) {
                    refusals.push(refuse(RefusalCode::RigidResponseInvalid, d.as_str(), Some("rigid_responses"),
                        format!("no rigid response is supplied for direction {d}")));
                }
            }
        }
    }
    (periodic, rigid)
}

/// Why one row could not be published.
enum RowFail {
    /// Not certified at the working precision; another precision may succeed.
    Uncertain { field: String, cause: Uncertain },
    /// The enclosure's lower end is beyond the binary64 range.
    Nonfinite { field: String },
}

fn take(c: Certified, field: &str) -> Result<f64, RowFail> {
    match c {
        Certified::Value(v) => Ok(v),
        Certified::Uncertain(cause) => Err(RowFail::Uncertain { field: field.to_string(), cause }),
        Certified::Nonfinite => Err(RowFail::Nonfinite { field: field.to_string() }),
    }
}

/// The exact inputs of one row: the cluster sums per direction (in X, Y, Z order) and the rigid
/// responses.
struct RowInputs<'a> {
    sums: Vec<Vec<&'a Dyadic>>,
    rigid: Option<Vec<Dyadic>>,
}

/// Evaluates and certifies one row with the ρ enclosures of one precision level.
fn evaluate_row(request: &CombinationRequest, directions: &[Direction], form: &ModalForm,
    table: Option<&CorrelationTable>, inputs: &RowInputs)
    -> Result<(Vec<DirectionalPart>, Magnitude, Option<Direction>), RowFail> {
    let mut parts = Vec::with_capacity(directions.len());
    let mut enclosures = Vec::with_capacity(directions.len());
    for (k, &d) in directions.iter().enumerate() {
        let field = d.as_str();
        let values: Vec<Dyadic> = inputs.sums[k].iter().map(|v| (*v).clone()).collect();
        let periodic = rules::modal(form, table, &values)
            .map_err(|cause| RowFail::Uncertain { field: field.to_string(), cause })?;
        let periodic_value = take(rules::certify(&periodic), field)?;
        let (rigid_value, combined) = match request.missing_mass {
            MissingMass::NotApplied => (None, periodic),
            MissingMass::Applied { rule, .. } => {
                let r = &inputs.rigid.as_ref().expect("rigid responses under the correction")[k];
                let rigid_value = take(rules::certify(&Interval::exact(r.abs())), field)?;
                let combined = match rule {
                    RigidRule::Srss => rules::with_rigid_srss(&periodic, r),
                    RigidRule::AbsoluteSum => rules::with_rigid_absolute_sum(&periodic, r),
                };
                (Some(Magnitude::new(rigid_value)), combined)
            }
        };
        let combined_value = take(rules::certify(&combined), field)?;
        parts.push(DirectionalPart {
            direction: d,
            periodic: Magnitude::new(periodic_value),
            rigid: rigid_value,
            combined: Magnitude::new(combined_value),
        });
        enclosures.push(combined);
    }
    let over = match request.directional_rule {
        DirectionalRule::Srss => rules::directional_srss(&enclosures),
        DirectionalRule::AbsoluteSum => rules::directional_absolute_sum(&enclosures),
        DirectionalRule::HundredFortyForty => rules::hundred_forty_forty(&enclosures),
    };
    let value = take(rules::certify(&over), "directions")?;
    // The direction carrying the full value: the largest published directional magnitude, the
    // first in X, Y, Z order on a tie (the value does not depend on which one governs).
    let governing = (request.directional_rule == DirectionalRule::HundredFortyForty).then(|| {
        let mut g = 0;
        for (i, part) in parts.iter().enumerate() {
            if part.combined.value() > parts[g].combined.value() {
                g = i;
            }
        }
        directions[g]
    });
    Ok((parts, Magnitude::new(value), governing))
}

fn uncertain_detail(cause: Uncertain, cap: u64) -> String {
    match cause {
        Uncertain::RadicandNotPositive => format!(
            "at the {cap}-bit cap the enclosure of the CQC radicand still reaches zero or below (strong cancellation); \
the value is not exactly zero by exact arithmetic, and no value is published"),
        Uncertain::NotWithinTolerance { tight: true } => "the value is enclosed to within a quarter of 1e-9, but no binary64 \
lies within 1e-9 relative of it; no value is published".into(),
        Uncertain::NotWithinTolerance { tight: false } => format!(
            "at the {cap}-bit cap the enclosure is still wider than 1e-9 relative allows (strong cancellation); \
no value is published"),
    }
}

/// Combines the supplied signed contributions by the request's rules (CLM-001).
///
/// The modes of each supplied cluster are summed with their signs first, exactly; the modal rule
/// is applied across the periodic clusters, each represented by its lowest frequency in ρ and in
/// the closeness tests; the rigid response, when the correction is applied, is combined with the
/// periodic part by the rigid rule; then the directions by the directional rule, in X, Y, Z order
/// whatever order they were requested in. Every output magnitude is unsigned.
///
/// **Accuracy.** Every published magnitude is the binary64 that an enclosure of its exact value on
/// the represented operands shows within 1e-9 relative of that value, or an exact zero proved by
/// exact arithmetic (`rules`). A row is evaluated at the precisions of
/// [`crate::PRECISION_LADDER`] in turn; a magnitude not certified at the cap refuses the request
/// (`SPECTRUM_COMBINATION_ACCURACY_UNCERTIFIED`), naming the row and where it occurred. A cluster
/// sum is published as the binary64 nearest its exact value.
///
/// A request that breaks the contract is refused with every refusal found, in a fixed order; no
/// partial result is published.
pub fn combine(request: &CombinationRequest) -> Result<Combination, Vec<Refusal>> {
    let mut refusals = Vec::new();
    check_choices(request, &mut refusals);
    let modes_ok = check_modes(request, &mut refusals);
    check_rows(request, &mut refusals);
    if !modes_ok {
        return Err(refusals);
    }
    let clusters = clusters(request);
    let (periodic, rigid) = check_contributions(request, &clusters, &mut refusals);
    if !refusals.is_empty() {
        return Err(refusals);
    }

    let directions: Vec<Direction> = Direction::ALL.into_iter().filter(|d| request.directions.contains(d)).collect();
    let units: Vec<&Cluster> = clusters.iter().filter(|c| c.class == ClusterClass::Periodic).collect();
    let frequencies: Vec<f64> = units.iter().map(|c| c.standing).collect();
    let (form, groups, close_pairs) = match request.modal_rule {
        ModalRule::Srss => (ModalForm::Srss, None, None),
        ModalRule::AbsoluteSum => (ModalForm::AbsoluteSum, None, None),
        ModalRule::Cqc => (ModalForm::Cqc, None, None),
        ModalRule::Grouping => {
            let g = rules::groups(&frequencies);
            let named = g.iter().map(|group| group.iter().map(|&i| units[i].index).collect()).collect();
            (ModalForm::Pairs(rules::group_pairs(&g)), Some(named), None)
        }
        ModalRule::TenPercent => {
            let pairs = rules::ten_percent_pairs(&frequencies);
            let named = pairs.iter().map(|&(i, j)| (units[i].index, units[j].index)).collect();
            (ModalForm::Pairs(pairs), None, Some(named))
        }
    };

    // Exact signed cluster sums, per direction, periodic cluster and row.
    let rows = request.rows.len();
    let mut exact: Vec<Vec<Vec<Dyadic>>> = Vec::with_capacity(directions.len());
    let mut cluster_sums = Vec::new();
    for &d in &directions {
        let mut per_unit = Vec::with_capacity(units.len());
        for c in &units {
            let mut sums = vec![Dyadic::zero(); rows];
            for &m in &c.modes {
                for (sum, v) in sums.iter_mut().zip(periodic[&(d, m)]) {
                    *sum = sum.add(&Dyadic::from_f64(*v));
                }
            }
            let values: Vec<f64> = sums.iter().map(Dyadic::to_f64).collect();
            for (value, row) in values.iter().zip(&request.rows) {
                if !value.is_finite() {
                    refusals.push(refuse(RefusalCode::CombinationNonfinite, &row.key(), Some(d.as_str()),
                        format!("the exact signed sum of {} is beyond the binary64 range", c.index)));
                }
            }
            cluster_sums.push(ClusterSum { cluster: c.index, direction: d, values });
            per_unit.push(sums);
        }
        exact.push(per_unit);
    }
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let rigid_exact: Option<Vec<Vec<Dyadic>>> = match request.missing_mass {
        MissingMass::NotApplied => None,
        MissingMass::Applied { .. } => Some(directions.iter().map(|d| rigid[d].iter().map(|&v| Dyadic::from_f64(v)).collect()).collect()),
    };

    // ρ tables, built once per precision when CQC needs them.
    let mut tables: Vec<Option<CorrelationTable>> = PRECISION_LADDER.iter().map(|_| None).collect();
    let cap = *PRECISION_LADDER.last().expect("a ladder");
    let mut combined_rows = Vec::with_capacity(rows);
    for (r, quantity) in request.rows.iter().enumerate() {
        let inputs = RowInputs {
            sums: exact.iter().map(|per_unit| per_unit.iter().map(|sums| &sums[r]).collect()).collect(),
            rigid: rigid_exact.as_ref().map(|by_direction| by_direction.iter().map(|values| values[r].clone()).collect()),
        };
        for (level, &p) in PRECISION_LADDER.iter().enumerate() {
            if request.modal_rule == ModalRule::Cqc && tables[level].is_none() {
                tables[level] = Some(CorrelationTable::new(&frequencies, request.damping_ratio, p));
            }
            match evaluate_row(request, &directions, &form, tables[level].as_ref(), &inputs) {
                Ok((per_direction, combined, governing_direction)) => {
                    combined_rows.push(CombinedRow { quantity: quantity.clone(), per_direction, combined, governing_direction });
                    break;
                }
                Err(RowFail::Nonfinite { field }) => {
                    refusals.push(refuse(RefusalCode::CombinationNonfinite, &quantity.key(), Some(&field),
                        "the combined value is beyond the binary64 range".into()));
                    break;
                }
                Err(RowFail::Uncertain { field, cause }) => {
                    if p == cap {
                        refusals.push(refuse(RefusalCode::CombinationAccuracyUncertified, &quantity.key(), Some(&field),
                            uncertain_detail(cause, cap)));
                    }
                }
            }
        }
    }
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let correlation = (request.modal_rule == ModalRule::Cqc)
        .then(|| tables[1].get_or_insert_with(|| CorrelationTable::new(&frequencies, request.damping_ratio, PRECISION_LADDER[1])).nearest(units.len()));

    let statements = clusters
        .iter()
        .map(|c| ClusterStatement {
            cluster: c.index,
            modes: c.modes.iter().map(|&m| ModeIndex(request.modes[m].mode.0)).collect(),
            frequencies_hz: c.modes.iter().map(|&m| request.modes[m].frequency_hz).collect(),
            standing_frequency_hz: c.standing,
            class: c.class,
        })
        .collect();
    Ok(Combination {
        damping_ratio: request.damping_ratio,
        modal_rule: request.modal_rule,
        directional_rule: request.directional_rule,
        missing_mass: request.missing_mass,
        directions,
        clusters: statements,
        correlation,
        groups,
        close_pairs,
        cluster_sums,
        rows: combined_rows,
    })
}
