//! The certificate's indexed lookups answer exactly as the scans they replace:
//! the operand group find (first match), the support magnitude candidates and
//! the pressure region of a member. Each reference below is the replaced scan,
//! kept here verbatim as the test's oracle.
use super::*;
use crate::{ResultBasisRef, ResultItem};

fn group(entity: &str, location: &str, kind: &str, bound: Option<f64>) -> OperandGroup {
    OperandGroup { entity: entity.into(), location: location.into(), group: kind.into(), moment: false, q: 1.0, bound, floor_scale: 1.0 }
}

/// The replaced scan: the first group of the triple.
fn find_scan<'o>(operand: &'o Operand, entity: &str, location: &str, kind: &str) -> Option<&'o OperandGroup> {
    operand.groups.iter().find(|g| g.entity == entity && g.location == location && g.group == kind)
}

fn find_indexed<'o>(operand: &'o Operand, entity: &str, location: &str, kind: &str) -> Option<&'o OperandGroup> {
    first_positions(operand).get(&(entity, location, kind)).map(|&i| &operand.groups[i])
}

/// Duplicate triples (two unavailable records of one entity; a connector with two
/// tie rods of one identifier): the first wins, so publish reaches the same
/// `break` on the first's unavailable bound; a missing triple misses.
#[test]
fn the_group_index_finds_the_first_of_duplicate_triples_and_misses_as_the_scan() {
    let operand = Operand {
        groups: vec![
            group("pipe:a", "end_i", "axial", Some(1.0)),
            group("pipe:u", "end_i", "axial", None),
            group("pipe:a", "end_j", "axial", Some(2.0)),
            group("pipe:u", "end_i", "axial", Some(3.0)),
            group("rod:t", "node", "support_force", Some(4.0)),
            group("rod:t", "node", "support_force", Some(5.0)),
        ],
    };
    // The first duplicate's bound is unavailable: publish's `let Some(bound) = own.bound else { break }`.
    let own = find_indexed(&operand, "pipe:u", "end_i", "axial").expect("found");
    assert!(own.bound.is_none(), "the first of the duplicates, whose bound breaks");
    assert_eq!(find_indexed(&operand, "rod:t", "node", "support_force").and_then(|g| g.bound), Some(4.0));
    let mut triples: Vec<(&str, &str, &str)> = operand.groups.iter().map(|g| (g.entity.as_str(), g.location.as_str(), g.group.as_str())).collect();
    triples.extend([("pipe:a", "midspan", "axial"), ("pipe:z", "end_i", "axial"), ("pipe:a", "end_i", "shear")]);
    for (e, l, k) in triples {
        let (scan, indexed) = (find_scan(&operand, e, l, k), find_indexed(&operand, e, l, k));
        assert_eq!(scan.map(|g| g as *const _), indexed.map(|g| g as *const _), "{e} {l} {k}");
    }
    assert!(find_indexed(&operand, "pipe:z", "end_i", "axial").is_none());
}

fn row(id: &str, kind: &str, entity: &str, value: f64, combination: &str) -> ResultItem {
    ResultItem {
        id: id.into(),
        kind: kind.into(),
        value,
        unit: "N".into(),
        entity_ref: entity.into(),
        basis_ref: Some(ResultBasisRef { ref_type: "combination".into(), ref_id: combination.into() }),
        source_result_refs: Vec::new(),
        metadata: None,
        component_body: None,
    }
}

/// publish's row loop, restated: (combination rows, magnitude index).
fn indexes(results: &[ResultItem]) -> (HashMap<&str, Vec<usize>>, MagnitudeRows<'_>) {
    let mut combination_rows: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut magnitude_rows: MagnitudeRows<'_> = HashMap::new();
    for (i, row) in results.iter().enumerate() {
        let Some(basis) = row.basis_ref.as_ref().filter(|b| b.ref_type == "combination") else { continue };
        if row.kind == MODULUS_RECORD {
            continue;
        }
        combination_rows.entry(basis.ref_id.as_str()).or_default().push(i);
        if row.kind == SUPPORT_FORCE || row.kind == SUPPORT_MOMENT {
            magnitude_rows.entry((basis.ref_id.as_str(), row.kind.as_str(), row.entity_ref.as_str())).or_default().push(i);
        }
    }
    (combination_rows, magnitude_rows)
}

/// The replaced scan of `magnitude_group`.
fn candidates_scan(results: &[ResultItem], combination_rows: &HashMap<&str, Vec<usize>>, combination: &str, kind: &str, entity: &str) -> Option<Vec<usize>> {
    Some(combination_rows.get(combination)?.iter().copied().filter(|&r| results[r].kind == kind && results[r].entity_ref == entity).collect())
}

/// A support with zero, one and two candidate magnitude rows: one forms the group,
/// zero and two form none, as the scan decided; candidates are in row order.
#[test]
fn support_magnitude_candidates_match_the_scan_for_zero_one_and_two_rows() {
    let c = "comb:x";
    let mut results = vec![
        row("r:fx", SUPPORT_COMPONENT, "support:a", 3.0, c),
        row("r:fy", SUPPORT_COMPONENT, "support:a", 4.0, c),
        row("r:fz", SUPPORT_COMPONENT, "support:a", 12.0, c),
        row("r:mod", MODULUS_RECORD, "support:a", 0.0, c),
        row("r:other", SUPPORT_FORCE, "support:a", 13.0, "comb:y"),
        row("r:mag", SUPPORT_FORCE, "support:a", 13.0, c),
    ];
    let derived = vec![Derived { json: Value::Null, rows: vec![0, 1, 2], bound: 0.0, floor_scale: 1.0, verdict: "established" }];
    let formed: HashMap<(String, String, String, usize), usize> =
        ["support:a", "support:b"].into_iter().map(|s| ((s.to_string(), "node".to_string(), "support_force".to_string(), 0), 0)).collect();
    let magnitude = |results: &[ResultItem], entity: &str| {
        let (combination_rows, magnitude_rows) = indexes(results);
        let scan = candidates_scan(results, &combination_rows, c, SUPPORT_FORCE, entity).unwrap_or_default();
        let indexed = magnitude_rows.get(&(c, SUPPORT_FORCE, entity)).cloned().unwrap_or_default();
        assert_eq!(scan, indexed, "{entity}: candidates in row order");
        magnitude_group(results, c, &group(entity, "node", "support_force_magnitude", Some(0.0)), "support_force", &formed, &derived, &magnitude_rows)
            .map(|d| d.rows)
    };
    assert_eq!(magnitude(&results, "support:a"), Some(vec![5]), "one candidate");
    assert_eq!(magnitude(&results, "support:b"), None, "zero candidates");
    results.push(row("r:mag2", SUPPORT_FORCE, "support:a", 13.0, c));
    assert_eq!(magnitude(&results, "support:a"), None, "two candidates");
    let (_, magnitude_rows) = indexes(&results);
    assert_eq!(magnitude_rows[&(c, SUPPORT_FORCE, "support:a")], vec![5, 6]);
    assert!(magnitude_rows.keys().all(|(_, kind, _)| *kind != MODULUS_RECORD));
}

/// The replaced scans of D-6's premise.
fn region_scan<'v>(pressure: &'v [Value], case: &str, member: &str) -> Option<&'v Value> {
    let mut hits = pressure.iter().filter(|r| r["load_case_id"] == case && r["member_pipe_ids"].as_array().is_some_and(|m| m.iter().any(|p| p == member)));
    let first = hits.next()?;
    hits.next().is_none().then_some(first)
}

fn equal_pressure_scan(pressure: &[Value], minuend: &str, subtrahend: &str, member: &str) -> bool {
    let (Some(a), Some(b)) = (region_scan(pressure, minuend, member), region_scan(pressure, subtrahend, member)) else { return false };
    let entry = |region: &Value, field: &str| -> Option<String> { region[field].as_array()?.iter().find(|e| e["pipe_id"] == member).map(bits_text) };
    ["p_pa", "external_pressure_increment_pa", "pressure_basis"].iter().all(|f| a.get(*f).is_some() && bits_text(&a[*f]) == bits_text(&b[*f]))
        && ["geometry", "materials"].iter().all(|f| entry(a, f).is_some() && entry(a, f) == entry(b, f))
}

fn region(case: &str, members: Value, p: f64, geometry: Value) -> Value {
    json!({"load_case_id": case, "member_pipe_ids": members, "p_pa": p, "external_pressure_increment_pa": 0.0, "pressure_basis": "b",
        "geometry": geometry, "materials": [{"pipe_id": "pipe:a", "e": 1}, {"pipe_id": "pipe:b", "e": 1}, {"pipe_id": "pipe:c", "e": 1}, {"pipe_id": "pipe:d", "e": 1}]})
}

/// A member in two regions of one case has no region (`None`, so no D-6 statement);
/// a member listed twice in one record has that record; a miss has none; the
/// first geometry entry of a pipe is the one compared.
#[test]
fn the_pressure_index_answers_as_the_region_scan() {
    let geometry = json!([{"pipe_id": "pipe:a", "g": 1}, {"pipe_id": "pipe:b", "g": 1}, {"pipe_id": "pipe:a", "g": 2}, {"pipe_id": "pipe:d", "g": 1}]);
    let pressure = vec![
        region("case:O", json!(["pipe:a", "pipe:b", "pipe:b"]), 2.0e6, geometry.clone()),
        region("case:O", json!(["pipe:c"]), 2.0e6, geometry.clone()),
        region("case:O", json!(["pipe:c", "pipe:d"]), 2.0e6, geometry.clone()),
        region("case:S", json!(["pipe:a", "pipe:b", "pipe:c", "pipe:d", 7]), 2.0e6, json!([{"pipe_id": "pipe:a", "g": 1}, {"pipe_id": "pipe:b", "g": 9}, {"pipe_id": "pipe:d", "g": 1}])),
        json!({"load_case_id": 1, "member_pipe_ids": ["pipe:a"]}),
    ];
    let index = PressureIndex::new(&pressure);
    for case in ["case:O", "case:S", "case:X"] {
        for member in ["pipe:a", "pipe:b", "pipe:c", "pipe:d", "pipe:z", "7"] {
            let scan = region_scan(&pressure, case, member).map(|r| r as *const Value);
            let indexed = index.region_of(case, member).map(|i| &pressure[i] as *const Value);
            assert_eq!(scan, indexed, "{case} {member}");
        }
    }
    assert_eq!(index.region_of("case:O", "pipe:c"), None, "two regions of one case: none");
    assert_eq!(index.region_of("case:O", "pipe:b"), Some(0), "listed twice in one record: that record");
    for member in ["pipe:a", "pipe:b", "pipe:c", "pipe:d", "pipe:z"] {
        for (m, s) in [("case:O", "case:S"), ("case:S", "case:O"), ("case:S", "case:S")] {
            assert_eq!(equal_pressure(&pressure, &index, m, s, member), equal_pressure_scan(&pressure, m, s, member), "{m} {s} {member}");
        }
    }
    assert!(equal_pressure(&pressure, &index, "case:O", "case:S", "pipe:a"), "the first geometry entry of pipe:a matches");
    assert!(!equal_pressure(&pressure, &index, "case:O", "case:S", "pipe:b"), "pipe:b's geometry differs");
    assert!(!equal_pressure(&pressure, &index, "case:O", "case:S", "pipe:c"), "pipe:c is in two regions of case:O");
}
