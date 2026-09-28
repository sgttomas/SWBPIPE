//! S11 site test, S11-K part (S11 section 4.3 rule 7, ROOT R3-3, V1 R3B-5;
//! RV1-N1 in S11-F).
//!
//! The section 2.5 table is this test's constant. Outside `#[cfg(test)]` items,
//! the scanner counts, per enclosing function:
//! - every compound assignment (`+=`, `-=`), floating-point or integer;
//! - every `.sum(`, `.sum::<` and `fold(`;
//! - (RV1-N1, S11-F) every self-assignment fold written `x = x + ...`,
//!   `x = x - ...`, `x = checked_value(x + ...)` or `x = checked_value(x - ...)`,
//!   where `x` is a place expression (an identifier, possibly with `let`,
//!   `mut`, a type, indexing, field access or a leading `*`) repeated verbatim
//!   at the start of the right-hand side.
//!
//! The table below is keyed by (file, function) and records the exact count
//! and the disposition of each match: an E-site / KS-site, or an explicit
//! exemption with its reason. A new accumulation of these shapes anywhere,
//! including inside an already-listed function, changes a count and fails the
//! test, so extending the table is a visible edit. The E-site functions of
//! S11-K are listed with count 0: they sum through
//! `exact_sum::ExactAccumulator`.
//!
//! Limits (what the scan cannot see; S11 section 4.3, RV1-N1). It is a source
//! scan, not a type check. It does not see a fold written in any other shape:
//! `s = v + s`, `s = t + v` with `t` a copy of `s`, a fold through a helper
//! function or closure, `iter().product()`, `map(..).collect()` followed by a
//! later sum, or a sum formed in another file. KS1 and KS3 keep their legacy
//! binary64 expressions (option (c) and uncoupled rows), so their rows are
//! nonzero and a restored fold on the exact rows would not change the count:
//! the numeric K4/K11 tests and the option (c) pins, not this table, kill
//! those mutants. The `product_physics` part of the site test is
//! `product_physics/tests/s11f_site_test.rs` (S11-F).
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
    // K1 (manager-approved additive extension): the sparse representation.
    Source {
        name: "FK/structural/sparse.rs",
        text: include_str!("../src/structural/sparse.rs"),
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
    // K-D5's formation check (ROOT, on K1's PR, a separate item).
    Source {
        name: "FK/structural/formation_check.rs",
        text: include_str!("../src/structural/formation_check.rs"),
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
    ("FK/structural.rs", "audit_contributions", 7, "exempt: two descriptive ContributionRounding low-part sums of stiffness expansions and one delta-norm of stiffness differences (section 2.5 contribution_differences); RV1-N1: the column magnitude/delta norms and the rhs magnitude norms (self-assignment), stiffness and |rhs| norms of the perturbation estimate, not load sums"),
    ("FK/structural.rs", "audit_intended_action", 2, "integer: denominator operation count (the audit itself is exact, section 4.3 rule 6); RV1-N1: the same-sign denominator |f|+sum|K||u| (a magnitude bound, not a load sum)"),
    ("FK/structural.rs", "audit_load_row", 2, "integer: denominator operation count; RV1-N1: the same-sign denominator |f_exact|+sum|K||u| (the load term is the exact net)"),
    ("FK/structural.rs", "cholesky", 2, "RV1-N1: exempt, generic factorization (section 4.3 limit 1): pivot sum and its magnitude scale"),
    ("FK/structural.rs", "factor_structural_profile", 3, "RV1-N1: exempt, generic skyline factorization (section 4.3 limit 1)"),
    ("FK/structural.rs", "solve", 4, "RV1-N1: exempt, the dense and profile factors' forward/back substitution (section 4.3 limit 1)"),
    ("FK/structural.rs", "transform_roundoff", 4, "RV1-N1: exempt, transform roundoff bound (stiffness formation evidence)"),
    ("FK/structural.rs", "physical_residual_record", 1, "integer: exponent step"),
    ("FK/structural.rs", "evaluate_original_residual_bound", 3, "integer: product count; RV1-N1: KS3's legacy binary64 numerator `r = r + p` (option (c) Binary64 binding and rows with no nonzero prescribed coupling, whose only load operand is the one force value) and the same-sign denominator d; the numerator is exact on prescribed-coupled rows"),
    ("FK/structural.rs", "estimate_rcond", 4, "exempt: condition-estimate probe norms and dot (section 4.3 limit 1); RV1-N1: a row 1-norm"),
    ("FK/structural.rs", "finish_checked_factor", 3, "exempt: max fold of residual ratios; integer refinement count; RV1-N1: the refinement update y = y + delta (a correction of the solution, not a load sum)"),
    ("FK/structural.rs", "verify_negative_direction", 3, "integer: term count; RV1-N1: energy and magnitude sums of the witness direction (stiffness quadratic form)"),
    ("FK/structural.rs", "exact_scaled_rhs", 0, "KS1: exact accumulator, scaled before one rounding"),
    ("FK/structural.rs", "prepare_bound", 1, "KS1 dispatch; RV1-N1: the legacy expression `b = checked_value(b - K*u)` kept for the option (c) Binary64 binding and for legacy rows without a nonzero prescribed product (b - (+-0) is exact); ledger and coupled rows use exact_scaled_rhs"),
    // ---- FK/structural/sparse.rs (K1: the sparse representation; the
    // residual, intended-action and load audits, KS1 and the completion are
    // FK/structural.rs's functions above, run on it)
    ("FK/structural/sparse.rs", "from_pattern_and_contributions", 1, "exempt: stiffness assembly, one coalesced value per pattern entry summed in contribution order (audited exactly by M03 contribution expansions)"),
    ("FK/structural/sparse.rs", "scatter_block", 1, "exempt: stiffness assembly, one addition per element entry (the dense assemble_element_contribution; audited exactly by M03 contribution expansions)"),
    ("FK/structural/sparse.rs", "assemble_sparse_stiffness", 1, "exempt: stiffness assembly (spring diagonal, as the product adds it)"),
    ("FK/structural/sparse.rs", "lower_entry_count", 1, "integer: storage count"),
    ("FK/structural/sparse.rs", "multiply", 1, "exempt: formed elastic action K*u (bit-identical to the product's multiply_matrix_vector, E12's formed term)"),
    ("FK/structural/sparse.rs", "sparse_audit_contributions", 7, "exempt, as audit_contributions: two descriptive ContributionRounding low-part sums of stiffness expansions and one delta-norm of stiffness differences; the column magnitude/delta norms and the |rhs| magnitude norms (self-assignment) of the perturbation estimate, not load sums"),
    ("FK/structural/sparse.rs", "verify_sparse_negative_direction", 3, "integer: term count; energy and magnitude sums of the witness direction (stiffness quadratic form), as verify_negative_direction"),
    ("FK/structural/sparse.rs", "pair_energy", 3, "integer: term count; energy and magnitude sums of a pair direction (stiffness quadratic form), as verify_negative_direction"),
    ("FK/structural/sparse.rs", "sparse_negative_pair_witness_counted", 1, "integer: visited-pair count"),
    ("FK/structural/sparse.rs", "reactions", 0, "E12 from sparse rows: the formed K*u and the DOF's ledger terms in one exact sum"),
    ("FK/structural/sparse.rs", "reduce_assembled_sparse_system", 0, "KS2 over the pattern: one exact sum per free row"),
    ("FK/structural/sparse.rs", "legacy_zero_product_fold", 0, "KS1 legacy rows (no nonzero prescribed product): the dense b - (+-0) fold reduced to its range check and zero-sign rule; no accumulation"),
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
    // ---- FK/structural/formation_check.rs (K-D5; its residual rho is one
    // ExactAccumulator sum per free row, so the check has no load fold)
    ("FK/structural/formation_check.rs", "pow2", 1, "integer: exponent step"),
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
        if b[i] == b'=' && self_assignment_at(&code, i) {
            let name = stack.last().map_or("<module>".to_string(), |s| s.0.clone());
            *counts.entry(name).or_insert(0) += 1;
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

/// RV1-N1: whether the `=` at `i` is a self-assignment fold `x = x + ...`,
/// `x = x - ...`, `x = checked_value(x + ...)` or `x = checked_value(x - ...)`.
fn self_assignment_at(code: &str, i: usize) -> bool {
    let b = code.as_bytes();
    let prev = if i > 0 { b[i - 1] } else { b' ' };
    let next = b.get(i + 1).copied().unwrap_or(b' ');
    if b"=!<>+-*/%&|^".contains(&prev) || next == b'=' || next == b'>' {
        return false;
    }
    let start = code[..i]
        .rfind(|c| c == ';' || c == '{' || c == '}' || c == '\n')
        .map_or(0, |k| k + 1);
    let mut lhs = code[start..i].trim();
    if let Some(rest) = lhs.strip_prefix("let ") {
        lhs = rest.trim_start();
        if let Some(rest) = lhs.strip_prefix("mut ") {
            lhs = rest.trim_start();
        }
        if let Some(colon) = lhs.find(':') {
            lhs = lhs[..colon].trim_end();
        }
    }
    if lhs.is_empty()
        || !lhs
            .chars()
            .all(|c| c.is_alphanumeric() || "_.[]* ".contains(c))
    {
        return false;
    }
    let mut rhs = code[i + 1..].trim_start();
    if let Some(rest) = rhs.strip_prefix("checked_value(") {
        rhs = rest.trim_start();
    }
    let Some(after) = rhs.strip_prefix(lhs) else {
        return false;
    };
    if after
        .chars()
        .next()
        .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '[' || c == '.')
    {
        return false;
    }
    let after = after.trim_start();
    after.starts_with('+') || (after.starts_with('-') && !after.starts_with("->"))
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

/// RV1-N1: RV1's mutants RV-M1a, RV-M1c and RV-M1d wrote the fold as a
/// self-assignment; the scanner now counts that shape, and nothing else.
#[test]
fn scanner_counts_self_assignment_folds() {
    let src = r#"
        fn e1(a: &mut [f64; 12], t: &[f64; 12]) { for d in 0..12 { a[d] = a[d] + t[d]; } }
        fn e3(b: f64, k: f64) -> f64 { let mut b = b; b = checked_value(b - k)?; b }
        fn e4(mut s: f64, v: f64) -> f64 { s = s + v; let s = s - v; *p = *p + 1.0; s }
        fn controls(s: f64, v: f64) -> bool { let t = s + v; let u = v + s; let w = s.max(v); let q = s_other + v; s == s + v || s <= s - v }
        fn arrow() -> impl Fn(f64) -> f64 { |x| x }
    "#;
    let counts = scan(src);
    assert_eq!(counts.get("e1"), Some(&1));
    assert_eq!(counts.get("e3"), Some(&1));
    assert_eq!(counts.get("e4"), Some(&3));
    assert_eq!(counts.get("controls"), None);
    assert_eq!(counts.get("arrow"), None);
}
