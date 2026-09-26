//! S11 site test, S11-K part (S11 section 4.3 rule 7, ROOT R3-3, V1 R3B-5).
//!
//! The section 2.5 table is this test's constant. Outside `#[cfg(test)]` items,
//! every floating-point or integer compound assignment (`+=`, `-=`) and every
//! `.sum(` / `.sum::<` / `fold(` in the scanned files is counted per enclosing
//! function. The table below is keyed by (file, function) and records the exact
//! count and the disposition of each match: an E-site / KS-site, or an explicit
//! exemption with its reason. A new accumulation anywhere, including inside an
//! already-listed function, changes a count and fails the test, so extending
//! the table is a visible edit. The E-site and KS-site functions of S11-K are
//! listed with count 0: they sum through `exact_sum::ExactAccumulator`, and a
//! restored binary64 fold there fails this test as well as the numeric tests.
//! The `product_physics` part of the site test comes with S11-F.
//!
//! Limit (S11 section 4.3): this is a source scan, not a type check.
use std::collections::BTreeMap;

struct Source {
    name: &'static str,
    text: &'static str,
}

const SOURCES: &[Source] = &[
    Source {
        name: "FK/lib.rs",
        text: include_str!("../src/lib.rs"),
    },
    Source {
        name: "FK/structural.rs",
        text: include_str!("../src/structural.rs"),
    },
    Source {
        name: "FK/structural/exact_boundary.rs",
        text: include_str!("../src/structural/exact_boundary.rs"),
    },
    Source {
        name: "FK/load_ledger.rs",
        text: include_str!("../src/load_ledger.rs"),
    },
    Source {
        name: "FK/exact_sum.rs",
        text: include_str!("../src/exact_sum.rs"),
    },
    Source {
        name: "SA/structural_adapter.rs",
        text: include_str!("../../nonlinear_integration/src/structural_adapter.rs"),
    },
    Source {
        name: "nonlinear_integration/lib.rs",
        text: include_str!("../../nonlinear_integration/src/lib.rs"),
    },
    Source {
        name: "SP/lib.rs",
        text: include_str!("../../straight_pipe/src/lib.rs"),
    },
    Source {
        name: "CB/lib.rs",
        text: include_str!("../../curved_bend/src/lib.rs"),
    },
    Source {
        name: "load_case_algebra/lib.rs",
        text: include_str!("../../../loads/load_case_algebra/src/lib.rs"),
    },
];

/// (file, function, exact match count, disposition of each match).
const TABLE: &[(&str, &str, usize, &str)] = &[
    // ---- FK/lib.rs
    ("FK/lib.rs", "reduced_right_hand_side", 1, "KS2: legacy row with no nonzero prescribed product; b - (+-0) is exact (keeps today's zero sign); coupled and ledger rows use the exact accumulator"),
    ("FK/lib.rs", "solve_dense", 3, "exempt: generic linear-algebra kernel (S11 section 4.3 limit 1), elimination and back substitution, no case force"),
    ("FK/lib.rs", "add_terms", 1, "exempt: stiffness formation"),
    ("FK/lib.rs", "add_relative_dof_stiffness", 4, "exempt: stiffness formation"),
    ("FK/lib.rs", "assemble_element_contribution", 1, "exempt: stiffness assembly (audited exactly by M03 contribution expansions)"),
    ("FK/lib.rs", "multiply_transpose_left", 2, "exempt: stiffness transform T^T K T"),
    // ---- FK/structural.rs
    ("FK/structural.rs", "radix_scale", 1, "integer: exponent step"),
    ("FK/structural.rs", "add", 1, "integer: expansion operation count"),
    ("FK/structural.rs", "add_product", 1, "integer: expansion operation count"),
    ("FK/structural.rs", "exact_radix", 1, "integer: exponent step"),
    ("FK/structural.rs", "audit_contributions", 3, "exempt: two descriptive ContributionRounding low-part sums of stiffness expansions and one delta-norm of stiffness differences (section 2.5 contribution_differences)"),
    ("FK/structural.rs", "audit_intended_action", 1, "integer: denominator operation count (the audit itself is exact, section 4.3 rule 6)"),
    ("FK/structural.rs", "audit_load_fidelity", 1, "integer: denominator operation count"),
    ("FK/structural.rs", "physical_residual_record", 1, "integer: exponent step"),
    ("FK/structural.rs", "evaluate_original_residual_bound", 1, "integer: product count (KS3 numerator is exact on prescribed-coupled rows)"),
    ("FK/structural.rs", "estimate_rcond", 3, "exempt: condition-estimate probe norms and dot (section 4.3 limit 1)"),
    ("FK/structural.rs", "finish_checked_factor", 2, "exempt: max fold of residual ratios; integer refinement count"),
    ("FK/structural.rs", "verify_negative_direction", 1, "integer: term count"),
    ("FK/structural.rs", "exact_scaled_rhs", 0, "KS1: exact accumulator, scaled before one rounding"),
    ("FK/structural.rs", "prepare_bound", 0, "KS1 dispatch: legacy rows without nonzero prescribed product keep today's checked expression"),
    // ---- FK/structural/exact_boundary.rs
    ("FK/structural/exact_boundary.rs", "approximate_projection", 2, "exempt: proposal quotient, verified exactly afterwards (section 4.1.2)"),
    ("FK/structural/exact_boundary.rs", "ratio_add", 1, "exempt: exact expansion sum (Context::sum)"),
    ("FK/structural/exact_boundary.rs", "new_charged", 7, "coverage checks :361/:387: ordered stiffness/force folds and naive projections are accepted alongside the correctly rounded sum (S11-K); the rest are exact expansion sums and an integer cursor"),
    ("FK/structural/exact_boundary.rs", "solve_charged", 5, "exempt: exact expansion sums (Context::sum)"),
    ("FK/structural/exact_boundary.rs", "project", 3, "exempt: exact expansion sums (Context::sum)"),
    ("FK/structural/exact_boundary.rs", "gap_sign_with_work", 2, "exempt: exact expansion sums (Context::sum)"),
    ("FK/structural/exact_boundary.rs", "verify_candidate_charged", 2, "exempt: min/max folds of interval corners"),
    ("FK/structural/exact_boundary.rs", "contact_charged", 3, "exempt: exact expansion sums (Context::sum)"),
    // ---- FK/exact_sum.rs
    ("FK/exact_sum.rs", "add_word", 1, "integer: limb index"),
    ("FK/exact_sum.rs", "project", 2, "integer: significand rounding and quantum carry"),
    ("FK/exact_sum.rs", "exact_rounded_sum", 0, "the one correctly rounded function"),
    ("FK/load_ledger.rs", "finish", 0, "ledger: one exact sum per DOF"),
    // ---- SA
    ("SA/structural_adapter.rs", "new", 4, "exempt: symmetry-roundoff provenance and integer counts"),
    ("SA/structural_adapter.rs", "element", 2, "exempt: symmetry-roundoff provenance and integer counts"),
    ("SA/structural_adapter.rs", "geometry", 1, "integer: queue index"),
    ("SA/structural_adapter.rs", "scrutinize_gaps", 3, "exempt: max folds of gap-scrutiny observations (T5's in-loop scrutiny)"),
    // ---- nonlinear_integration (signatures only in S11-K; serialized with T5)
    ("nonlinear_integration/lib.rs", "solve_active_set_frame_with_mode_and_springs", 1, "exempt: linear spring stiffness formation"),
    ("nonlinear_integration/lib.rs", "add_curved_bend_stiffness_contributions", 1, "exempt: stiffness assembly"),
    ("nonlinear_integration/lib.rs", "solve_iteration_with_sliding_friction_evidence", 2, "allow-listed as T5's: unit-force influence solves (c61a540ea :1333) and their integer coupling pattern"),
    ("nonlinear_integration/lib.rs", "add_applied_forces", 1, "allow-listed as T5's: E14 in-loop sliding-friction forces (S11 section 10 item 1)"),
    ("nonlinear_integration/lib.rs", "max_abs_free_dof_work_residual", 1, "exempt: max fold of residual observation"),
    ("nonlinear_integration/lib.rs", "max_abs_delta_by_dof_group", 1, "exempt: max fold of delta observation"),
    ("nonlinear_integration/lib.rs", "max_abs_delta", 1, "exempt: max fold of delta observation"),
    ("nonlinear_integration/lib.rs", "max_abs_value", 1, "exempt: max fold (DEC-050/053 sparse parity observation lane)"),
    ("nonlinear_integration/lib.rs", "max_abs_entry_residual", 3, "allow-listed: DEC-050/053 sparse parity observation lane (section 4.3 limit 2)"),
    ("nonlinear_integration/lib.rs", "max_abs_by_dof_group", 1, "exempt: max fold of residual observation"),
    ("nonlinear_integration/lib.rs", "multiply_matrix_vector", 1, "exempt: formed elastic action K*u"),
    // ---- SP
    ("SP/lib.rs", "transform_global_displacements_to_local", 1, "exempt: formed transform (section 2.2 declared formation)"),
    ("SP/lib.rs", "transform_local_element_vector_to_global", 1, "exempt: formed transform (section 2.2 declared formation)"),
    ("SP/lib.rs", "multiply_matrix_vector", 1, "exempt: formed elastic term K_e*u (section 2.2 declared formation)"),
    ("SP/lib.rs", "equivalent_nodal_loads_with_spans", 0, "E1: exact sum of per-load terms"),
    ("SP/lib.rs", "equivalent_local_axial_effect_loads", 0, "E2: exact sum of per-effect pairs"),
    ("SP/lib.rs", "exact_loaded_local_forces", 0, "E3: local - load terms - axial terms, one exact sum"),
    ("SP/lib.rs", "exact_array_sum", 0, "E1-E3 accumulator"),
    ("SP/lib.rs", "station_resultants_from_i_end_with_spans", 0, "E4/E6: one exact sum per station component"),
    ("SP/lib.rs", "exact_terms_sum", 0, "E4/E6 accumulator"),
    // ---- CB
    ("CB/lib.rs", "consistent_uniform_nodal_loads", 2, "exempt: one source's consistent equivalent (section 2.2 declared formation)"),
    ("CB/lib.rs", "consistent_radial_pressure_nodal_loads", 2, "exempt: one source's consistent equivalent (section 2.2 declared formation)"),
    ("CB/lib.rs", "arc_section_resultants_with_radial_pressure", 3, "E11 in S11-F: PP's summed-thrust section (dormant terms API below replaces it at S11-F)"),
    ("CB/lib.rs", "cross_quad", 1, "exempt: flexibility quadrature (stiffness formation)"),
    ("CB/lib.rs", "rotate_to_global", 1, "exempt: formed rotation"),
    ("CB/lib.rs", "quad", 1, "exempt: flexibility quadrature (stiffness formation)"),
    ("CB/lib.rs", "multiply6", 1, "exempt: stiffness formation"),
    ("CB/lib.rs", "multiply6_transpose_right", 1, "exempt: stiffness formation"),
    ("CB/lib.rs", "arc_section_resultant_terms", 0, "E11 terms API: exact sum of single-input section values"),
    // ---- load_case_algebra
    ("load_case_algebra/lib.rs", "evaluate_linear_combination", 1, "E13: the binary64 fold is kept only as today's non-finite value; the published value is the exact sum of exact products"),
];

// ------------------------------------------------------------- scanner

/// Removes comments and blanks string and char literal contents, keeping
/// line structure.
fn lex(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let n = b.len();
    let mut out = String::with_capacity(n);
    let mut i = 0;
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    while i < n {
        let c = b[i];
        if c == '/' && i + 1 < n && b[i + 1] == '/' {
            while i < n && b[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && i + 1 < n && b[i + 1] == '*' {
            let mut depth = 1;
            i += 2;
            while i < n && depth > 0 {
                if b[i] == '/' && i + 1 < n && b[i + 1] == '*' {
                    depth += 1;
                    i += 2;
                } else if b[i] == '*' && i + 1 < n && b[i + 1] == '/' {
                    depth -= 1;
                    i += 2;
                } else {
                    if b[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
            }
            continue;
        }
        let prev_ident = i > 0 && ident(b[i - 1]);
        // Raw strings r"..", r#".."#, br"..".
        let raw_start = if !prev_ident && c == 'r' {
            Some(i + 1)
        } else if !prev_ident && c == 'b' && i + 1 < n && b[i + 1] == 'r' {
            Some(i + 2)
        } else {
            None
        };
        if let Some(mut j) = raw_start {
            let mut hashes = 0;
            while j < n && b[j] == '#' {
                hashes += 1;
                j += 1;
            }
            if j < n && b[j] == '"' {
                j += 1;
                loop {
                    if j >= n {
                        break;
                    }
                    if b[j] == '"' && (0..hashes).all(|h| j + 1 + h < n && b[j + 1 + h] == '#') {
                        j += 1 + hashes;
                        break;
                    }
                    if b[j] == '\n' {
                        out.push('\n');
                    }
                    j += 1;
                }
                out.push_str("\"\"");
                i = j;
                continue;
            }
        }
        if c == '"' || (!prev_ident && c == 'b' && i + 1 < n && b[i + 1] == '"') {
            let mut j = if c == 'b' { i + 2 } else { i + 1 };
            while j < n && b[j] != '"' {
                if b[j] == '\\' {
                    j += 1;
                }
                if j < n && b[j] == '\n' {
                    out.push('\n');
                }
                j += 1;
            }
            out.push_str("\"\"");
            i = j + 1;
            continue;
        }
        if c == '\'' {
            // Char literal 'x' or '\..'; otherwise a lifetime.
            if i + 2 < n && b[i + 1] != '\\' && b[i + 2] == '\'' {
                out.push_str("' '");
                i += 3;
                continue;
            }
            if i + 1 < n && b[i + 1] == '\\' {
                let mut j = i + 2;
                while j < n && b[j] != '\'' {
                    j += 1;
                }
                out.push_str("' '");
                i = j + 1;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Blanks every `#[cfg(test)]` item (a braced block, or up to `;`).
fn strip_cfg_test(code: &str) -> String {
    const ATTR: &str = "#[cfg(test)]";
    let mut out = String::with_capacity(code.len());
    let mut rest = code;
    while let Some(at) = rest.find(ATTR) {
        out.push_str(&rest[..at]);
        let item = &rest[at..];
        let mut depth = 0usize;
        let mut started = false;
        let mut end = item.len();
        for (k, ch) in item.char_indices().skip(ATTR.len()) {
            match ch {
                '{' => {
                    depth += 1;
                    started = true;
                }
                '}' => {
                    depth -= 1;
                    if started && depth == 0 {
                        end = k + 1;
                        break;
                    }
                }
                ';' if !started => {
                    end = k + 1;
                    break;
                }
                _ => {}
            }
        }
        out.extend(item[..end].chars().filter(|&c| c == '\n'));
        rest = &item[end..];
    }
    out.push_str(rest);
    out
}

/// Per-function counts of accumulation matches.
fn scan(src: &str) -> BTreeMap<String, usize> {
    let code = strip_cfg_test(&lex(src));
    let b = code.as_bytes();
    let mut counts = BTreeMap::new();
    let mut stack: Vec<(String, usize)> = Vec::new();
    let mut pending: Option<String> = None;
    let (mut depth, mut nest) = (0usize, 0usize);
    let ident = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = 0;
    while i < b.len() {
        let rest = &code[i..];
        let boundary = i == 0 || !ident(b[i - 1]);
        if boundary
            && rest.starts_with("fn")
            && rest.len() > 2
            && (b[i + 2] as char).is_whitespace()
        {
            let name: String = rest[2..]
                .trim_start()
                .chars()
                .take_while(|&c| c.is_alphanumeric() || c == '_')
                .collect();
            if !name.is_empty() {
                pending = Some(name);
            }
            i += 2;
            continue;
        }
        let matched = ["+=", "-=", ".sum(", ".sum::<"]
            .iter()
            .find(|p| rest.starts_with(**p))
            .map(|p| p.len())
            .or_else(|| (boundary && rest.starts_with("fold(")).then_some(5));
        if let Some(len) = matched {
            let name = stack.last().map_or("<module>".to_string(), |s| s.0.clone());
            *counts.entry(name).or_insert(0) += 1;
            if rest[..len].ends_with('(') {
                nest += 1;
            }
            i += len;
            continue;
        }
        match b[i] {
            b'(' | b'[' => nest += 1,
            b')' | b']' => nest = nest.saturating_sub(1),
            b';' if nest == 0 => pending = None,
            b'{' => {
                depth += 1;
                if let Some(name) = pending.take() {
                    stack.push((name, depth));
                }
            }
            b'}' => {
                if stack.last().is_some_and(|s| s.1 == depth) {
                    stack.pop();
                }
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
        i += 1;
    }
    counts
}

fn functions(src: &str) -> Vec<String> {
    let code = strip_cfg_test(&lex(src));
    code.split("fn ")
        .skip(1)
        .map(|s| {
            s.chars()
                .take_while(|&c| c.is_alphanumeric() || c == '_')
                .collect()
        })
        .collect()
}

#[test]
fn site_table_is_exactly_the_accumulations_of_the_scanned_files() {
    let mut actual: BTreeMap<(String, String), usize> = BTreeMap::new();
    for source in SOURCES {
        for (function, count) in scan(source.text) {
            actual.insert((source.name.to_string(), function), count);
        }
    }
    let mut expected: BTreeMap<(String, String), usize> = BTreeMap::new();
    for &(file, function, count, disposition) in TABLE {
        assert!(!disposition.is_empty());
        assert!(
            SOURCES.iter().any(|s| s.name == file),
            "unknown file {file}"
        );
        let previous = expected.insert((file.to_string(), function.to_string()), count);
        assert!(previous.is_none(), "duplicate table row {file} {function}");
        if count == 0 {
            // A zero row names an exact site: the function must exist.
            let source = SOURCES.iter().find(|s| s.name == file).unwrap();
            assert!(
                functions(source.text).iter().any(|f| f == function),
                "{file}: {function} not found"
            );
        }
    }
    let nonzero_expected: BTreeMap<_, _> = expected
        .iter()
        .filter(|(_, &c)| c > 0)
        .map(|(k, &c)| (k.clone(), c))
        .collect();
    let mut problems = Vec::new();
    for (key, count) in &actual {
        match nonzero_expected.get(key) {
            Some(expected_count) if expected_count == count => {}
            Some(expected_count) => {
                problems.push(format!("{key:?}: table {expected_count}, source {count}"))
            }
            None => problems.push(format!("{key:?}: {count} unlisted accumulation(s)")),
        }
    }
    for (key, count) in &nonzero_expected {
        if !actual.contains_key(key) {
            problems.push(format!("{key:?}: table {count}, source 0"));
        }
    }
    for (key, &count) in &expected {
        if count == 0 && actual.contains_key(key) {
            problems.push(format!(
                "{key:?}: exact site now accumulates {}",
                actual[key]
            ));
        }
    }
    assert!(
        problems.is_empty(),
        "site table mismatch:\n{}",
        problems.join("\n")
    );
}

#[test]
fn scanner_counts_what_it_claims() {
    let src = r#"
        fn a(x: &mut [f64; 3]) { x[0] += 1.0; let s: f64 = x.iter().sum(); x.iter().fold(0.0, |p, q| p + q); }
        // fn commented() { y += 1.0; }
        fn b() -> [f64; 2] { let t = "+= inside a string"; let c = '{'; [0.0; 2] }
        #[cfg(test)]
        mod tests { fn c() { z -= 1.0; } }
        fn d() { w -= 2.0; w.iter().sum::<f64>(); }
    "#;
    let counts = scan(src);
    assert_eq!(counts.get("a"), Some(&3));
    assert_eq!(counts.get("b"), None);
    assert_eq!(counts.get("c"), None);
    assert_eq!(counts.get("d"), Some(&2));
    assert_eq!(counts.len(), 2);
}
