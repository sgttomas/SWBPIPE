//! The committed case files and everything decided from R1's references alone
//! (no kernel solve): their sha256, the counts, R1's key map, the exact engine
//! against Python's Fractions, the floor check against the committed list, C1,
//! the discrimination of the value controls, and V-K's S* against K4's
//! exported functions.
use open_pipe_stress_frame_kernel::structural::retained_api::{
    body_extent as k4_body_extent, coupled_scales as k4_coupled_scales, FLOOR_RATIO_BITS,
};
use piping_numerical_robustness::cases::{cases_dir, load_all, Case, Target, FAMILY_FILES};
use piping_numerical_robustness::exact::{self, Exact};
use piping_numerical_robustness::floor;
use piping_numerical_robustness::lane::value_controls;
use piping_numerical_robustness::sha256::sha256_hex;
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::OnceLock;

fn cases() -> &'static [Case] {
    static CASES: OnceLock<Vec<Case>> = OnceLock::new();
    CASES.get_or_init(load_all)
}

fn ci() -> impl Iterator<Item = &'static Case> {
    cases().iter().filter(|c| !c.is_large())
}

#[test]
fn the_case_files_match_their_recorded_sha256() {
    let sums = std::fs::read_to_string(cases_dir().join("SHA256SUMS")).unwrap();
    let mut names = Vec::new();
    for line in sums.lines() {
        let (digest, name) = line.split_once("  ").unwrap();
        let data = std::fs::read(cases_dir().join(name)).unwrap();
        assert_eq!(sha256_hex(&data), digest, "{name}");
        names.push(name.to_string());
    }
    for (_, f) in FAMILY_FILES {
        assert!(names.iter().any(|n| n == f), "{f} is not in SHA256SUMS");
    }
    assert_eq!(names.len(), 15);
}

#[test]
fn the_files_hold_r1s_213_kernel_lane_cases_as_counted() {
    let all = cases();
    assert_eq!(all.len(), 213);
    let mut by_family: BTreeMap<&str, (usize, usize, usize)> = BTreeMap::new();
    for c in all {
        let e = by_family.entry(c.family.as_str()).or_default();
        e.0 += 1;
        e.1 += c.rows.len();
        e.2 += c.controls.len();
    }
    let expect = [
        ("RF-CANCEL", (38, 1444, 152)),
        ("RF-CHAIN", (30, 2760, 120)),
        ("RF-FINITE", (6, 748, 12)),
        ("RF-INVARIANCE", (25, 7096, 54)),
        ("RF-LARGE", (24, 11102, 76)),
        ("RF-MECH", (9, 26, 26)),
        ("RF-RANGE", (32, 2720, 92)),
        ("RF-SKEW", (36, 1080, 138)),
        ("RF-WEAK", (9, 618, 36)),
        ("RF-ZERO", (4, 158, 9)),
    ];
    assert_eq!(by_family, expect.into_iter().collect());
    let large: Vec<&Case> = all.iter().filter(|c| c.is_large()).collect();
    assert_eq!(large.len(), 12);
    assert_eq!(large.iter().map(|c| c.rows.len()).sum::<usize>(), 2048);
    assert_eq!(ci().map(|c| c.rows.len()).sum::<usize>(), 25704);
    assert_eq!(ci().map(|c| c.controls.len()).sum::<usize>(), 677);
    assert_eq!(all.iter().filter(|c| c.refuse).count(), 8);
    assert_eq!(
        all.iter().filter(|c| c.needs_directional_spring).count(),
        22
    );
    let represented: Vec<&str> = all
        .iter()
        .filter(|c| c.basis == "represented")
        .map(|c| c.id.as_str())
        .collect();
    assert_eq!(
        represented,
        ["RF-SKEW-A-CANT-AX-122-r1e-12", "RF-FINITE-THIRTIETHS-O1e6"]
    );
    let omitted: Vec<(&str, &Vec<String>)> = ci()
        .filter(|c| !c.model.as_ref().unwrap().omitted_springs.is_empty())
        .map(|c| (c.id.as_str(), &c.model.as_ref().unwrap().omitted_springs))
        .collect();
    assert_eq!(omitted, [("RF-MECH-K0", &vec!["S.N0.0".to_string()])]);
    for c in large {
        assert!(c.model_sha256.is_some() && c.s_full.is_some(), "{}", c.id);
    }
}

#[test]
fn every_r1_key_resolves_and_the_structural_zeros_are_exact_zeros() {
    let mut zeros: BTreeMap<&str, usize> = BTreeMap::new();
    for c in ci() {
        let model = c.model.as_ref().unwrap();
        for row in &c.rows {
            let target = model.resolve(&row.key).unwrap();
            if target == Target::StructuralZero {
                assert_eq!(row.expected, "0", "{} {}", c.id, row.key);
                *zeros.entry(c.family.as_str()).or_default() += 1;
            }
        }
    }
    let expect = [
        ("RF-CHAIN", 60),
        ("RF-INVARIANCE", 40),
        ("RF-RANGE", 80),
        ("RF-SKEW", 96),
        ("RF-WEAK", 6),
    ];
    assert_eq!(zeros, expect.into_iter().collect());
}

/// The engine against Python's Fractions (plain rows) and 400-digit Decimal
/// (magnitudes) on sampled rows (`engine_vectors.jsonl`).
#[test]
fn the_exact_engine_agrees_with_the_generators_fractions_on_every_vector() {
    let text = std::fs::read_to_string(cases_dir().join("engine_vectors.jsonl")).unwrap();
    let hex = |v: &Value| f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap());
    let (mut plain, mut magnitude, mut held) = (0, 0, 0);
    for line in text.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let exp = Exact::parse_decimal(v["e"].as_str().unwrap()).unwrap();
        let scale = Exact::parse_decimal(v["s"].as_str().unwrap()).unwrap();
        let got = match v["k"].as_str().unwrap() {
            "p" => {
                plain += 1;
                exact::predicate(&Exact::from_f64(hex(&v["o"])), &exp, &scale)
            }
            _ => {
                magnitude += 1;
                exact::magnitude_predicate(hex(&v["y"]), hex(&v["z"]), &exp, &scale)
            }
        };
        assert_eq!(got, v["v"].as_bool().unwrap(), "{line}");
        held += usize::from(got);
    }
    println!("vectors: {plain} plain, {magnitude} magnitude, {held} hold");
    assert!(plain > 1000 && magnitude > 100 && held > 0 && held < plain + magnitude);
}

#[test]
fn the_rows_outside_binary64s_range_are_cont_n10000s_five() {
    let mut found = Vec::new();
    for c in cases() {
        for row in &c.rows {
            let exp = Exact::parse_decimal(&row.expected).unwrap();
            if exact::outside_binary64(&exp) {
                found.push(format!("{}:{}", c.id, row.key));
            }
        }
    }
    assert_eq!(
        found,
        [
            "RF-LARGE-CONT-n10000-AX:u.C2500.UZ",
            "RF-LARGE-CONT-n10000-AX:u.C4999.UZ",
            "RF-LARGE-CONT-n10000-AX:R.S1250.UZ",
            "RF-LARGE-CONT-n10000-AX:Mb.A3750.i",
            "RF-LARGE-CONT-n10000-ROT:Mb.A3750.i",
        ]
    );
    let committed: Value = serde_json::from_str(
        &std::fs::read_to_string(cases_dir().join("absolute_range_rows.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(committed.as_array().unwrap().len(), 5);
}

#[test]
fn the_floor_check_reproduces_the_committed_not_covered_list() {
    assert_eq!(floor::R_BITS, FLOOR_RATIO_BITS);
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for c in ci().filter(|c| !c.rows.is_empty()) {
        let model = c.model.as_ref().unwrap();
        assert!(model.bodies().iter().all(|&b| b == 0), "{}: one body", c.id);
        let scales = floor::scales(c, model);
        let mine = floor::not_covered(c, model, &scales);
        assert_eq!(mine, c.not_covered, "{}", c.id);
        *counts.entry(c.family.as_str()).or_default() += mine.len();
    }
    counts.retain(|_, n| *n > 0);
    assert_eq!(
        counts,
        [("RF-CANCEL", 3), ("RF-SKEW", 2), ("RF-WEAK", 46)]
            .into_iter()
            .collect()
    );
    let listed: Value = serde_json::from_str(
        &std::fs::read_to_string(cases_dir().join("not_covered.json")).unwrap(),
    )
    .unwrap();
    let entries = listed["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 51);
    for e in entries {
        let (family, id, key) = (
            e[0].as_str().unwrap(),
            e[1].as_str().unwrap(),
            e[2].as_str().unwrap(),
        );
        let c = cases().iter().find(|c| c.id == id).unwrap();
        assert_eq!(c.family, family);
        assert!(c.not_covered.iter().any(|k| k == key), "{id} {key}");
    }
}

#[test]
fn v_ks_s_star_equals_k4s_exported_functions_bit_for_bit() {
    for c in ci().filter(|c| !c.rows.is_empty()) {
        let model = c.model.as_ref().unwrap();
        let ext = floor::body_extent(&model.nodes);
        assert_eq!(
            ext.to_bits(),
            k4_body_extent(&model.nodes).to_bits(),
            "{}",
            c.id
        );
        let s = floor::maxima(c.rows.iter().map(|r| (r.kind(), r.expected.as_str())));
        let mine = floor::couple(s, ext);
        let theirs = k4_coupled_scales(s, ext);
        assert_eq!(mine.map(f64::to_bits), theirs.map(f64::to_bits), "{}", c.id);
    }
}

/// ROOT's C1 ruling: the pre-scaled k_t and k_a equal §4.10's formula bit for
/// bit wherever that formula's intermediates are normal, and the formula is
/// undefined only on RF-RANGE's LEF cases.
#[test]
fn c1_the_prescaled_coefficients_equal_the_designs_wherever_it_is_defined() {
    let mut undefined = Vec::new();
    let mut compared = 0;
    for c in ci() {
        let model = c.model.as_ref().unwrap();
        for m in &model.members {
            let l = floor::member_length(
                model.nodes[m.node_i as usize],
                model.nodes[m.node_j as usize],
            );
            for (a, b, what) in [
                (m.shear_modulus, m.torsion_constant, "k_t"),
                (m.elastic_modulus, m.area, "k_a"),
            ] {
                let pre = floor::prescaled_coefficient(a, b, l);
                assert!(pre.is_normal(), "{} {} {what}", c.id, m.name);
                match floor::design_coefficient(a, b, l) {
                    Some(k) => {
                        assert_eq!(pre.to_bits(), k.to_bits(), "{} {} {what}", c.id, m.name);
                        compared += 1;
                    }
                    None => undefined.push(format!("{} {what}", c.id)),
                }
            }
        }
    }
    undefined.sort();
    undefined.dedup();
    assert_eq!(
        undefined,
        [
            "RF-RANGE-CHAIN-LEF-large k_t",
            "RF-RANGE-CHAIN-LEF-small k_t",
            "RF-RANGE-CONT-LEF-large k_t",
            "RF-RANGE-CONT-LEF-small k_t",
            "RF-RANGE-SKEW-LEF-large k_t",
            "RF-RANGE-SKEW-LEF-small k_t",
        ]
    );
    // 2,977 members in the CI models, two coefficients each; k_t is undefined
    // on the 16 members of each LEF vector (5 + 1 + 10).
    assert_eq!(compared, 2 * 2977 - 32);
}

#[test]
fn every_discriminating_value_control_fails_the_predicate() {
    let mut discriminated = 0;
    let mut non = 0;
    for c in ci() {
        let t = value_controls(c);
        assert!(t.undiscriminated.is_empty(), "{:?}", t.undiscriminated);
        assert!(
            t.unexpectedly_failing.is_empty(),
            "{:?}",
            t.unexpectedly_failing
        );
        discriminated += t.discriminated;
        non += t.non_discriminating;
    }
    // 506 discriminating controls in CI, of which 25 are outcome controls
    // (decided by the lane).
    assert_eq!((discriminated, non), (481, 171));
    let large: usize = cases()
        .iter()
        .filter(|c| c.is_large())
        .map(|c| value_controls(c).discriminated)
        .sum();
    assert_eq!(large, 38);
}

/// ROOT's ruling on I17's A1 stop: the expected-unresolved list is THIN-A and
/// THIN-B, with the stiffness ratio that puts them beyond W1a's reach.
#[test]
fn the_expected_unresolved_list_is_thin_a_and_thin_b() {
    use piping_numerical_robustness::cases::expected_unresolved;
    assert_eq!(
        expected_unresolved(),
        ["RF-RANGE-THIN-A", "RF-RANGE-THIN-B"]
    );
    let v: Value = serde_json::from_str(
        &std::fs::read_to_string(cases_dir().join("expected_unresolved.json")).unwrap(),
    )
    .unwrap();
    for e in v["entries"].as_array().unwrap() {
        let log2: f64 = e["log2_ea_over_12ei_l3"].as_str().unwrap().parse().unwrap();
        assert!((507.0..508.0).contains(&log2), "{e}");
        assert_eq!(e["family"], "RF-RANGE");
    }
}
