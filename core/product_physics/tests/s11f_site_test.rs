//! S11 site test, S11-F part (S11_CONTAINMENT revision 5a.2, section 4.3
//! rules 1-8; RV1-N1, RV1-N6 and the sparse_direct item).
//!
//! It reads the product facade (`PP`, `source_recovery`, `source_receipt` and
//! its non-test submodules, `pressure_runtime`, `self_weight`), the kernel
//! seams (`FK`, `SA`, `nonlinear_integration`, `sparse_direct/structural.rs`)
//! and, for rule 8, `SP`, `CB` and `load_case_algebra`. Every check runs on
//! lexed source (comments removed, string and char literal contents blanked)
//! with `#[cfg(test)]` items blanked, so a commented or quoted call is not a
//! call. Each allow-list and constant below is exact: an unlisted hit fails,
//! and so does a listed entry that no longer occurs, so extending a list is a
//! visible edit (rule 7).
//!
//! **Limits.** This is a source scan, not a type check (S11 section 4.3). It
//! backs the types where they cannot reach. It sees only the shapes it names:
//! rule 8 counts compound assignments, `.sum(`, `.sum::<`, `fold(` and RV1-N1's
//! self-assignment folds (`x = x + ...`, `x = checked_value(x - ...)`); rule 3
//! sees a force only through its binding's name or its `let` initializer in
//! the same function. A fold through a helper in an unscanned file, or in a
//! shape not listed here, is invisible to it.
//!
//! **Behavioural backing (RV1's lesson).** Every pin here is backed by a
//! behavioural test that first asserts that the binary64 path and the exact
//! path differ, and only then asserts the outcome:
//! - the ledger at the producers, the typed seams and rules 1, 2, 4 and 5:
//!   `s11f_product_tests::f1_*`, `f2_*`, `f3_*`, `f11_*`, `f12_*` and `f14_*`
//!   (the product publishes the correctly rounded net where the fold differs),
//!   in both modes, which also covers the sparse_direct path;
//! - rule 3 and the E-sites of rule 8: `f2_*` (E5, E7, E12), `f8_*` (E8-E11)
//!   and `f10_*` (E15, E16);
//! - T1's sites: `f4_*`, `f5_*` and `f6_*`; KS1 through the product: `f9_*`;
//! - RV1-N6 (the product never calls the public binary64 residual):
//!   `n6_product_residual_rows_use_the_exact_numerator`.
use std::collections::{BTreeMap, BTreeSet};

struct Source {
    name: &'static str,
    text: &'static str,
}

const PRODUCT: &[Source] = &[
    Source {
        name: "PP/lib.rs",
        text: include_str!("../src/lib.rs"),
    },
    Source {
        name: "PP/source_recovery.rs",
        text: include_str!("../src/source_recovery.rs"),
    },
    Source {
        name: "PP/source_receipt.rs",
        text: include_str!("../src/source_receipt.rs"),
    },
    Source {
        name: "PP/source_receipt/composite.rs",
        text: include_str!("../src/source_receipt/composite.rs"),
    },
    Source {
        name: "PP/source_receipt/endpoint_maximum.rs",
        text: include_str!("../src/source_receipt/endpoint_maximum.rs"),
    },
    Source {
        name: "PP/source_receipt/rows.rs",
        text: include_str!("../src/source_receipt/rows.rs"),
    },
    Source {
        name: "PP/source_receipt/source.rs",
        text: include_str!("../src/source_receipt/source.rs"),
    },
    Source {
        name: "PP/pressure_runtime.rs",
        text: include_str!("../src/pressure_runtime.rs"),
    },
    Source {
        name: "PP/self_weight.rs",
        text: include_str!("../src/self_weight.rs"),
    },
];

const KERNEL: &[Source] = &[
    Source {
        name: "FK/lib.rs",
        text: include_str!("../../solver/frame_kernel/src/lib.rs"),
    },
    Source {
        name: "FK/structural.rs",
        text: include_str!("../../solver/frame_kernel/src/structural.rs"),
    },
    Source {
        name: "FK/structural/exact_boundary.rs",
        text: include_str!("../../solver/frame_kernel/src/structural/exact_boundary.rs"),
    },
    Source {
        name: "FK/load_ledger.rs",
        text: include_str!("../../solver/frame_kernel/src/load_ledger.rs"),
    },
    Source {
        name: "SA/structural_adapter.rs",
        text: include_str!("../../solver/nonlinear_integration/src/structural_adapter.rs"),
    },
    Source {
        name: "nonlinear_integration/lib.rs",
        text: include_str!("../../solver/nonlinear_integration/src/lib.rs"),
    },
    Source {
        name: "sparse_direct/structural.rs",
        text: include_str!("../../solver/sparse_direct/src/structural.rs"),
    },
];

const RECOVERY: &[Source] = &[
    Source {
        name: "SP/lib.rs",
        text: include_str!("../../solver/straight_pipe/src/lib.rs"),
    },
    Source {
        name: "CB/lib.rs",
        text: include_str!("../../solver/curved_bend/src/lib.rs"),
    },
    Source {
        name: "load_case_algebra/lib.rs",
        text: include_str!("../../loads/load_case_algebra/src/lib.rs"),
    },
];

// ------------------------------------------------------------------ constants

/// Rule 2: `values()` followed by `.to_vec(`, `.to_owned(`, `.iter_mut(`, or
/// `.iter().copied()` / `.iter().cloned()` with a `.collect` in the same
/// statement. (file, function, disposition).
const ALLOW_VALUES_COPY: &[(&str, &str, &str)] = &[
    ("PP/lib.rs", "legacy_observation_force", "observation lane (S11 section 4.3 limit 2): T1's DEC050/053 observation force; it never reaches a solve seam"),
    ("PP/lib.rs", "append_nonlinear_support_loop_results", "the loop's `input.force` copy; the typed entry `solve_active_set_frame_with_mode_and_springs_assembled` refuses it unless it equals the ledger's values bit for bit"),
];

/// Rule 3: a compound assignment or `.iter_mut()` on a binding whose name
/// contains `force`, `rhs` or `load` (any case), or that a `let` in the same
/// function binds from an expression mentioning `values()`, `force` or `rhs`.
/// (file, function, binding, disposition).
const ALLOW_FORCE_MUTATION: &[(&str, &str, &str, &str)] = &[
    ("PP/lib.rs", "run_linear_static_preview_captured_once", "load_case_solves", "per-case result records, not a force vector"),
    ("PP/lib.rs", "legacy_observation_force", "observation", "observation lane (limit 2): T1's DEC050/053 observation force, K_fc g_c folded on purpose"),
    ("PP/lib.rs", "recover_curved_bend_local_forces", "global_forces", "E8/E9: the row loop writes each row's one exact sum"),
    ("PP/lib.rs", "recover_curved_bend_local_forces", "local_forces", "the chord rotation of the recovered end forces (a formed transform, section 2.2)"),
    ("PP/lib.rs", "recover_curved_bend_local_forces", "local_force", "the chord rotation's element loop (a formed transform, section 2.2)"),
    ("FK/lib.rs", "reduced_right_hand_side", "adjusted_force", "KS2: the legacy row expression, kept only for the option (c) binary64 variant and legacy rows with no nonzero prescribed product (b - (+-0) is exact); ledger rows are always exact"),
    ("FK/lib.rs", "solve_dense", "rhs", "generic linear-algebra kernel (limit 1)"),
    ("FK/lib.rs", "solve_dense", "sum", "generic linear-algebra kernel (limit 1): back substitution"),
    ("FK/structural.rs", "finish_checked_factor", "y", "refinement update of the solution, bound from the prepared right-hand side (not a load sum)"),
    ("nonlinear_integration/lib.rs", "solve_iteration_with_sliding_friction_evidence", "unit_force", "T5's: unit-force influence solves (R3-N1)"),
    ("nonlinear_integration/lib.rs", "add_applied_forces", "force", "T5's: E14 in-loop sliding-friction forces (S11 section 10 item 1)"),
];

/// Rule 4: product calls to the generic linear-algebra kernels (limit 1).
const ALLOW_GENERIC_KERNEL: &[(&str, &str, &str)] = &[
    ("PP/lib.rs", "legacy_dense_observation", "observation lane: DEC050/053 legacy unscaled LU reference on the reduced system; never selects a solution"),
    ("PP/lib.rs", "solve_preview_reduced_system", "observation lane: DEC050/053 raw sparse observation of `observation_force`; never selects a solution"),
    ("PP/lib.rs", "append_sparse_live_path_evidence", "observation lane: DEC050/053 sparse live-path parity evidence"),
];
const GENERIC_KERNELS: &[&str] = &[
    "solve_dense(",
    "solve_symmetric_system(",
    "solve_symmetric_system_from_entries(",
    "factorize_ldlt(",
];

/// Calls the product must never make: the binary64 fold (rule 1), the
/// `&[f64]` seams it switched from, and (RV1-N6) the public binary64 residual.
const FORBIDDEN_PRODUCT_CALLS: &[&str] = &[
    "global_load_vector(",
    "reduce_system(",
    "reduce_system_with_prescribed_displacements(",
    "reduce_system_with_prescribed_displacements_binary64(",
    "solve_structural_dense(",
    "solve_structural_dense_binary64(",
    "solve_structural_sparse(",
    "solve_structural_sparse_binary64(",
    "prepare_structural(",
    "prepare_structural_binary64(",
    "with_force_terms(",
    "solve_active_set_frame(",
    "solve_active_set_frame_with_mode(",
    "solve_active_set_frame_with_mode_and_springs(",
    "evaluate_original_residual(",
    ".solve_binary64(",
    "assembly.solve(",
    "AssemblyEvidence::solve(",
];

/// Rule 5: the functions that push into a `LoadLedger` are exactly the
/// section 4.2 producers (plus T1's retained-source fold check, section 4.5).
const PRODUCERS: &[(&str, &str, &str)] = &[
    ("PP/lib.rs", "push_nodal_loads", "nodal loads: one term per load"),
    ("PP/lib.rs", "add_uniform_element_loads", "straight and curved uniform equivalents: one term per (load, DOF)"),
    ("PP/lib.rs", "add_pressure_thrust_loads", "straight thrust pairs: fl(P*x_a) per axis"),
    ("PP/lib.rs", "add_curved_bend_pressure_thrust_load", "curved thrust caps fl(P*t_a) and one term per wall slot"),
    ("PP/lib.rs", "add_thermal_equivalent_loads", "straight thermal pairs and T1 eigen pairs: fl(P*x_a) per axis"),
    ("PP/lib.rs", "add_curved_bend_thermal_equivalent_load", "curved thermal: push_product(K_rc, fl(eps*chord_c)) per nonzero column"),
    ("PP/lib.rs", "push_exact_pressure_operands", "exact pressure: each source group's operand"),
    ("PP/lib.rs", "add_constant_effort_support_loads", "constant effort: one term per application"),
    ("PP/source_recovery.rs", "prepare_sources", "T1 site (section 4.5): the retained nodal terms, in a ledger compared with the actual force"),
    ("PP/source_recovery.rs", "close_load_state", "T1 site (section 4.5): the retained eigen terms, the same terms the product pushes"),
];

/// Rule 6: the functions of `FK`, `SA`, `nonlinear_integration` and
/// `sparse_direct` that touch the case force (its values, its terms, or a
/// `force_rows`/`force_terms` binding). Each is a KS site, the load audit,
/// the exact-context coverage, a dispatch or a read-only use.
const FORCE_FUNCTIONS: &[(&str, &str, &str)] = &[
    ("FK/lib.rs", "reduce_system", "dispatch to KS2 (Values)"),
    (
        "FK/lib.rs",
        "reduce_system_with_prescribed_displacements",
        "dispatch to KS2 (Values)",
    ),
    (
        "FK/lib.rs",
        "reduce_system_with_prescribed_displacements_binary64",
        "dispatch to KS2 (option (c) Binary64)",
    ),
    (
        "FK/lib.rs",
        "reduce_assembled_system",
        "dispatch to KS2 (Assembled)",
    ),
    (
        "FK/lib.rs",
        "reduce_assembled_system_with_prescribed_displacements",
        "dispatch to KS2 (Assembled)",
    ),
    ("FK/lib.rs", "values", "read-only accessor of ForceRows"),
    (
        "FK/lib.rs",
        "reduce_system_for_boundary",
        "KS2 driver: length and finiteness checks, one call per row",
    ),
    (
        "FK/lib.rs",
        "reduced_right_hand_side",
        "KS2: exact f - sum K_ic g_c",
    ),
    (
        "FK/structural.rs",
        "assembled",
        "typed constructor: the force slice is the ledger's values",
    ),
    ("FK/structural.rs", "audit_terms", "binding accessor"),
    (
        "FK/structural.rs",
        "validate",
        "read-only: length and finiteness",
    ),
    ("FK/structural.rs", "contribution_sums", "read-only: length"),
    (
        "FK/structural.rs",
        "audit_intended_action",
        "M03 intended-action audit (already exact)",
    ),
    (
        "FK/structural.rs",
        "audit_load_fidelity",
        "S11 section 5.1 load audit",
    ),
    (
        "FK/structural.rs",
        "audit_load_row",
        "S11 section 5.1 load audit, one row",
    ),
    (
        "FK/structural.rs",
        "unaudited_row",
        "RV1-N5: a flagged unaudited row (read-only)",
    ),
    (
        "FK/structural.rs",
        "prepare_structural_with_force_terms",
        "C3-detect entry point",
    ),
    ("FK/structural.rs", "exact_scaled_rhs", "KS1"),
    ("FK/structural.rs", "prepare_bound", "KS1 dispatch"),
    (
        "FK/structural.rs",
        "evaluate_original_residual_bound",
        "KS3",
    ),
    (
        "FK/structural.rs",
        "finish_checked_factor",
        "read-only: length",
    ),
    (
        "FK/structural.rs",
        "solve_structural_dense_with_force_terms",
        "C3-detect entry point",
    ),
    (
        "FK/structural.rs",
        "verify_negative_direction",
        "read-only: length",
    ),
    (
        "FK/structural/exact_boundary.rs",
        "new_charged",
        "exact-context coverage check (section 4.1.2)",
    ),
    (
        "FK/structural/exact_boundary.rs",
        "force_level",
        "exact-context force basis",
    ),
    (
        "FK/structural/exact_boundary.rs",
        "matches",
        "exact-context binding check",
    ),
    (
        "FK/structural/exact_boundary.rs",
        "force_basis",
        "exact-context force basis",
    ),
    (
        "FK/structural/exact_boundary.rs",
        "copy_charge",
        "exact-context replay binding",
    ),
    (
        "FK/structural/exact_boundary.rs",
        "replay_against",
        "exact-context replay binding",
    ),
    (
        "FK/load_ledger.rs",
        "accumulate_dof",
        "ledger: exact per-DOF accumulation",
    ),
    (
        "SA/structural_adapter.rs",
        "new",
        "stores no force; initializes the C3-detect terms to None",
    ),
    (
        "SA/structural_adapter.rs",
        "with_force_terms",
        "C3-detect binding",
    ),
    (
        "SA/structural_adapter.rs",
        "solve",
        "the pub(crate) `&[f64]` solve (C3-detect when bound)",
    ),
    (
        "SA/structural_adapter.rs",
        "scrutinize_gaps",
        "T5's in-loop scrutiny (E14 friction, allow-listed)",
    ),
    (
        "nonlinear_integration/lib.rs",
        "require_assembled_force",
        "typed siblings: bit-for-bit check",
    ),
    (
        "nonlinear_integration/lib.rs",
        "solve_active_set_frame_with_mode_and_springs",
        "the loop (option (c), T5's)",
    ),
    (
        "nonlinear_integration/lib.rs",
        "product_completion_matches_source",
        "read-only comparison",
    ),
    (
        "nonlinear_integration/lib.rs",
        "validate_input",
        "read-only: length and finiteness",
    ),
    (
        "nonlinear_integration/lib.rs",
        "solve_iteration_with_sliding_friction_evidence",
        "T5's: E14 and the influence solves",
    ),
    (
        "nonlinear_integration/lib.rs",
        "solve_linearized_system_evidence",
        "the loop's binary64 solves (option (c) pins)",
    ),
];
const FORCE_TOKENS: &[&str] = &[
    ".force[",
    ".force.",
    "force.values()",
    "accumulate_dof(",
    "terms_for(",
    "force_terms",
    "ForceRows::",
    "force_rows",
    "force.terms()",
];

/// Rule 8 (R3-3, R3B-5, RV1-N1): every accumulation of the rule-8 shapes in
/// the listed files, keyed by (file, function) with its exact count. The
/// S11-F E-site functions are listed with count 0: they sum through
/// `exact_sum::ExactAccumulator`.
const TABLE: &[(&str, &str, usize, &str)] = &[
    // ---- PP/lib.rs
    ("PP/lib.rs", "add_curved_bend_stiffness_contributions", 1, "stiffness assembly"),
    ("PP/lib.rs", "append_component_stress_multiplier_results", 1, "integer: appended count"),
    ("PP/lib.rs", "append_curved_bend_macro_element_results", 1, "integer: appended count"),
    ("PP/lib.rs", "append_expansion_joint_pressure_thrust_results", 1, "integer: appended count (E16 itself is the exact sum below)"),
    ("PP/lib.rs", "append_expansion_joint_user_stiffness_results", 1, "integer: appended count"),
    ("PP/lib.rs", "append_nonlinear_residual_observation_results", 3, "max folds of residual observations"),
    ("PP/lib.rs", "append_spring_hanger_user_input_results", 3, "integer: appended count"),
    ("PP/lib.rs", "assemble_case_stiffness", 1, "stiffness assembly (spring diagonal)"),
    ("PP/lib.rs", "compute_pipe_mass_per_length", 2, "declared formation: same-sign mass terms (section 2.2, as self_weight)"),
    ("PP/lib.rs", "curved_bend_section_resultants", 1, "formed transform: chord frame back to global (section 2.2); E11 is the terms API call"),
    ("PP/lib.rs", "debit", 2, "integer: budget charge"),
    ("PP/lib.rs", "legacy_observation_force", 1, "allow-listed observation lane (limit 2)"),
    ("PP/lib.rs", "max_abs_delta", 1, "max fold of an observation"),
    ("PP/lib.rs", "max_abs_entry_residual", 3, "allow-listed DEC050/053 sparse parity observation lane (limit 2)"),
    ("PP/lib.rs", "max_abs_value", 1, "max fold of an observation"),
    ("PP/lib.rs", "multiply_matrix_vector", 1, "formed elastic term K*u of E12 (section 2.5)"),
    ("PP/lib.rs", "recover_curved_bend_local_forces", 1, "formed transform: the chord rotation after the E8/E9 exact sums"),
    ("PP/lib.rs", "reserve_publication", 2, "integer: budget charge"),
    ("PP/lib.rs", "run_linear_static_preview_captured_once", 4, "stiffness assembly (spring diagonals, 2) and integer per-case counts (2)"),
    ("PP/lib.rs", "solve_load_case", 2, "integer: source-budget attempts and component modifier count"),
    ("PP/lib.rs", "straight_local_uniform_loads", 1, "declared formation: one load's local transform (section 2.5 PP:7479)"),
    ("PP/lib.rs", "straight_summary_extrema", 1, "declared: the stress bound's same-sign pattern sums (section 2.2)"),
    ("PP/lib.rs", "exact_straight_end_forces", 0, "E5"),
    ("PP/lib.rs", "exact_straight_summary_extrema", 0, "E7"),
    ("PP/lib.rs", "restrained_reactions", 0, "E12"),
    ("PP/lib.rs", "pressure_for_pipe", 0, "E15"),
    ("PP/lib.rs", "pressure_thrusts_for_pipe", 0, "E5/E9/E11 inputs: one entry per thrust load"),
    ("PP/lib.rs", "curved_bend_uniform_intensities_by_pipe", 0, "E10 removed: one intensity per load"),
    ("PP/lib.rs", "push_nodal_loads", 0, "producer"),
    ("PP/lib.rs", "add_uniform_element_loads", 0, "producer"),
    ("PP/lib.rs", "add_pressure_thrust_loads", 0, "producer"),
    ("PP/lib.rs", "add_curved_bend_pressure_thrust_load", 0, "producer"),
    ("PP/lib.rs", "add_thermal_equivalent_loads", 0, "producer"),
    ("PP/lib.rs", "add_curved_bend_thermal_equivalent_load", 0, "producer"),
    ("PP/lib.rs", "push_exact_pressure_operands", 0, "producer"),
    ("PP/lib.rs", "add_constant_effort_support_loads", 0, "producer"),
    // ---- PP/pressure_runtime.rs
    ("PP/pressure_runtime.rs", "finish_source_groups", 1, "max fold of the pressure-RHS screen magnitude"),
    ("PP/pressure_runtime.rs", "traverse_region", 1, "geometry: chord projection (section 2.5)"),
    // ---- PP/self_weight.rs
    ("PP/self_weight.rs", "stable_mass_per_length", 2, "declared formation: same-sign mass (section 2.2)"),
    // ---- PP/source_recovery.rs
    ("PP/source_recovery.rs", "prepare_sources", 1, "stiffness coverage fold (stiffness, not loads; section 2.5 :761)"),
    ("PP/source_recovery.rs", "solve", 2, "max folds of retained-source observations"),
    // ---- PP/source_receipt.rs
    ("PP/source_receipt.rs", "check_input_with_physical", 1, "stiffness assembly (spring diagonal) of the replay"),
    ("PP/source_receipt.rs", "finalization_size", 2, "integer: byte sizes"),
    ("PP/source_receipt.rs", "finalize_for", 3, "integer: byte sizes and qualified count"),
    ("PP/source_receipt.rs", "scaled_norm", 1, "max fold"),
    // ---- SP, CB, load_case_algebra (also in frame_kernel/tests/s11_site_table.rs)
    ("SP/lib.rs", "transform_global_displacements_to_local", 1, "formed transform (section 2.2)"),
    ("SP/lib.rs", "transform_local_element_vector_to_global", 1, "formed transform (section 2.2)"),
    ("SP/lib.rs", "multiply_matrix_vector", 1, "formed elastic term K_e*u (section 2.2)"),
    ("SP/lib.rs", "equivalent_nodal_load_terms_with_spans", 0, "E1 per-load terms (E5's input)"),
    ("CB/lib.rs", "consistent_uniform_nodal_loads", 2, "one source's consistent equivalent (section 2.2)"),
    ("CB/lib.rs", "consistent_radial_pressure_nodal_loads", 2, "one source's consistent equivalent (section 2.2)"),
    ("CB/lib.rs", "arc_section_resultants_with_radial_pressure", 3, "no product caller since S11-F (E11 uses arc_section_resultant_terms); kept for CB's callers and tests"),
    ("CB/lib.rs", "cross_quad", 1, "flexibility quadrature (stiffness formation)"),
    ("CB/lib.rs", "rotate_to_global", 1, "formed rotation"),
    ("CB/lib.rs", "quad", 1, "flexibility quadrature (stiffness formation)"),
    ("CB/lib.rs", "multiply6", 1, "stiffness formation"),
    ("CB/lib.rs", "multiply6_transpose_right", 1, "stiffness formation"),
    ("CB/lib.rs", "arc_section_resultant_terms", 0, "E11"),
    ("load_case_algebra/lib.rs", "evaluate_linear_combination", 1, "E13: the fold is kept only as today's non-finite value"),
];
const RULE8_FILES: &[&str] = &[
    "PP/lib.rs",
    "PP/pressure_runtime.rs",
    "PP/self_weight.rs",
    "PP/source_recovery.rs",
    "PP/source_receipt.rs",
    "SP/lib.rs",
    "CB/lib.rs",
    "load_case_algebra/lib.rs",
];

// ------------------------------------------------------------- scanner
// `lex`, `strip_cfg_test` and the rule-8 counter are copied from
// `frame_kernel/tests/s11_site_table.rs` (another crate's integration test,
// which cannot be imported), with RV1-N1's self-assignment rule.

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

fn code_of(text: &str) -> String {
    strip_cfg_test(&lex(text))
}

/// RV1-N1: whether the `=` at `i` is a self-assignment fold.
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

/// A function item outside `#[cfg(test)]`: its name, parameter list and body.
struct Item {
    name: String,
    signature: String,
    body: String,
}

/// Every `fn` item with its signature and body (nested items separately).
fn items(code: &str) -> Vec<Item> {
    let b = code.as_bytes();
    let ident = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut out = Vec::new();
    let mut stack: Vec<(String, String, usize, usize)> = Vec::new();
    let mut pending: Option<(String, usize)> = None;
    let (mut depth, mut nest) = (0usize, 0usize);
    let mut i = 0;
    while i < b.len() {
        let boundary = i == 0 || !ident(b[i - 1]);
        if boundary
            && code[i..].starts_with("fn")
            && i + 2 < b.len()
            && (b[i + 2] as char).is_whitespace()
        {
            let name: String = code[i + 2..]
                .trim_start()
                .chars()
                .take_while(|&c| c.is_alphanumeric() || c == '_')
                .collect();
            if !name.is_empty() {
                pending = Some((name, i));
            }
            i += 2;
            continue;
        }
        match b[i] {
            b'(' | b'[' => nest += 1,
            b')' | b']' => nest = nest.saturating_sub(1),
            b';' if nest == 0 => pending = None,
            b'{' => {
                depth += 1;
                if let Some((name, at)) = pending.take() {
                    stack.push((name, code[at..i].to_string(), depth, i));
                }
            }
            b'}' => {
                if stack.last().is_some_and(|s| s.2 == depth) {
                    let (name, signature, _, start) = stack.pop().unwrap();
                    out.push(Item {
                        name,
                        signature,
                        body: code[start..=i].to_string(),
                    });
                }
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// Rule-8 counts per function.
fn scan(src: &str) -> BTreeMap<String, usize> {
    let code = code_of(src);
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

/// Positions of `needle` not preceded by an identifier character.
fn calls<'a>(body: &'a str, needle: &'a str) -> impl Iterator<Item = usize> + 'a {
    body.match_indices(needle).filter_map(move |(at, _)| {
        let starts_ident = needle.starts_with(|c: char| c.is_alphanumeric() || c == '_');
        let prev = body[..at].chars().next_back();
        (!starts_ident || !prev.is_some_and(|c| c.is_alphanumeric() || c == '_')).then_some(at)
    })
}

/// The first identifier of a place expression (`*x[i].y` -> `x`).
fn base_ident(place: &str) -> String {
    let place = place.trim().trim_start_matches('*');
    let place = place.strip_prefix("self.").unwrap_or(place);
    place
        .chars()
        .skip_while(|c| !(c.is_alphabetic() || *c == '_'))
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// `let [mut] NAME[: T] = EXPR;` bindings whose EXPR mentions a force.
fn tainted_bindings(body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for at in calls(body, "let ") {
        let rest = &body[at + 4..];
        let rest = rest.trim_start();
        let rest = rest.strip_prefix("mut ").unwrap_or(rest).trim_start();
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() {
            continue;
        }
        let Some(end) = rest.find(';') else { continue };
        let statement = &rest[..end];
        let Some(eq) = statement.find('=') else {
            continue;
        };
        let expression = &statement[eq + 1..];
        if expression.contains("values()")
            || expression.contains("force")
            || expression.contains("rhs")
        {
            out.insert(name);
        }
    }
    out
}

fn force_named(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("force") || lower.contains("rhs") || lower.contains("load")
}

/// Rule 3 hits in one body: (binding, what).
fn force_mutations(body: &str) -> Vec<(String, &'static str)> {
    let tainted = tainted_bindings(body);
    let bad = |name: &str| !name.is_empty() && (tainted.contains(name) || force_named(name));
    let mut out = Vec::new();
    for op in ["+=", "-="] {
        for (at, _) in body.match_indices(op) {
            let start = body[..at]
                .rfind(|c| c == ';' || c == '{' || c == '}' || c == '\n')
                .map_or(0, |k| k + 1);
            let name = base_ident(&body[start..at]);
            if bad(&name) {
                out.push((name, "compound assignment"));
            }
        }
    }
    for (at, _) in body.match_indices(".iter_mut(") {
        let start = body[..at]
            .rfind(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.'))
            .map_or(0, |k| k + 1);
        let name = base_ident(&body[start..at]);
        if bad(&name) {
            out.push((name, "iter_mut"));
        }
    }
    out
}

/// Rule 2 hits in one body.
fn values_copies(body: &str) -> usize {
    let mut hits = 0;
    for (at, needle) in body.match_indices("values()") {
        let rest = body[at + needle.len()..].trim_start();
        let statement = rest.split(';').next().unwrap_or("");
        if rest.starts_with(".to_vec(")
            || rest.starts_with(".to_owned(")
            || rest.starts_with(".iter_mut(")
            || ((rest.starts_with(".iter().copied()") || rest.starts_with(".iter().cloned()"))
                && statement.contains(".collect"))
        {
            hits += 1;
        }
    }
    hits
}

/// Rule 5: the `LoadLedger` bindings a function pushes into.
fn ledger_pushes(item: &Item) -> usize {
    let mut receivers = BTreeSet::new();
    let sig = &item.signature;
    for (at, _) in sig.match_indices("&mut LoadLedger") {
        let before = sig[..at].trim_end().trim_end_matches(':').trim_end();
        let name: String = before
            .chars()
            .rev()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        if !name.is_empty() {
            receivers.insert(name);
        }
    }
    for at in calls(&item.body, "let mut ") {
        let rest = &item.body[at + 8..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if rest[name.len()..]
            .trim_start()
            .trim_start_matches('=')
            .trim_start()
            .starts_with("LoadLedger::new()")
        {
            receivers.insert(name);
        }
    }
    receivers
        .iter()
        .map(|r| {
            calls(&item.body, &format!("{r}.push(")).count()
                + calls(&item.body, &format!("{r}.push_product(")).count()
        })
        .sum()
}

fn all_sources() -> impl Iterator<Item = &'static Source> {
    PRODUCT.iter().chain(KERNEL).chain(RECOVERY)
}

// ------------------------------------------------------------------ tests

#[test]
fn rule_1_and_forbidden_calls_the_product_never_folds_or_uses_untyped_seams() {
    let mut problems = Vec::new();
    for source in PRODUCT {
        let code = code_of(source.text);
        for needle in FORBIDDEN_PRODUCT_CALLS {
            for item in items(&code) {
                if calls(&item.body, needle).next().is_some() {
                    problems.push(format!("{}: {} calls {needle}", source.name, item.name));
                }
            }
        }
    }
    assert!(
        problems.is_empty(),
        "forbidden product calls:\n{}",
        problems.join("\n")
    );
    // The typed seams are reached: the linear solve, the reductions, the loop.
    let pp = code_of(PRODUCT[0].text);
    let body = |name: &str| {
        items(&pp)
            .into_iter()
            .find(|item| item.name == name)
            .unwrap_or_else(|| panic!("{name} present"))
            .body
    };
    assert!(body("solve_preview_reduced_system").contains("assembly.solve_assembled("));
    let case = body("solve_load_case");
    assert!(case.contains("reduce_assembled_system_with_prescribed_displacements("));
    assert!(case.contains("reduce_assembled_system("));
    assert!(case.contains("finish_case_ledger("));
    assert!(body("append_nonlinear_support_loop_results")
        .contains("solve_active_set_frame_with_mode_and_springs_assembled("));
}

#[test]
fn rule_2_values_are_copied_only_in_allow_listed_functions() {
    let mut actual = BTreeSet::new();
    for source in PRODUCT.iter().chain(KERNEL) {
        for item in items(&code_of(source.text)) {
            if values_copies(&item.body) > 0 {
                actual.insert((source.name.to_string(), item.name));
            }
        }
    }
    let expected: BTreeSet<_> = ALLOW_VALUES_COPY
        .iter()
        .map(|(f, n, d)| {
            assert!(!d.is_empty());
            (f.to_string(), n.to_string())
        })
        .collect();
    assert_eq!(actual, expected, "rule 2 allow-list");
}

#[test]
fn rule_3_force_bindings_are_mutated_only_in_allow_listed_functions() {
    let mut actual = BTreeSet::new();
    for source in PRODUCT.iter().chain(KERNEL) {
        for item in items(&code_of(source.text)) {
            for (binding, _) in force_mutations(&item.body) {
                actual.insert((source.name.to_string(), item.name.clone(), binding));
            }
        }
    }
    let expected: BTreeSet<_> = ALLOW_FORCE_MUTATION
        .iter()
        .map(|(f, n, b, d)| {
            assert!(!d.is_empty());
            (f.to_string(), n.to_string(), b.to_string())
        })
        .collect();
    assert_eq!(actual, expected, "rule 3 allow-list");
}

#[test]
fn rule_4_generic_kernels_are_called_only_by_observation_lanes() {
    let mut actual = BTreeSet::new();
    for source in PRODUCT {
        for item in items(&code_of(source.text)) {
            if GENERIC_KERNELS
                .iter()
                .any(|k| calls(&item.body, k).next().is_some())
            {
                actual.insert((source.name.to_string(), item.name));
            }
        }
    }
    let expected: BTreeSet<_> = ALLOW_GENERIC_KERNEL
        .iter()
        .map(|(f, n, _)| (f.to_string(), n.to_string()))
        .collect();
    assert_eq!(actual, expected, "rule 4 allow-list");
}

#[test]
fn rule_5_the_ledger_producers_are_exactly_the_section_4_2_list() {
    let mut actual = BTreeSet::new();
    for source in PRODUCT.iter().chain(KERNEL) {
        for item in items(&code_of(source.text)) {
            if ledger_pushes(&item) > 0 {
                actual.insert((source.name.to_string(), item.name));
            }
        }
    }
    let expected: BTreeSet<_> = PRODUCERS
        .iter()
        .map(|(f, n, _)| (f.to_string(), n.to_string()))
        .collect();
    assert_eq!(actual, expected, "rule 5 producer list");
    // Section 4.2 granularity where the push kind matters (M5): the curved
    // thermal producer pushes exact products K_rc * fl(eps * chord_c), never
    // a pre-summed row value; the exact-pressure producer pushes each group
    // operand, never the pre-summed `assembled_loads` total.
    let pp = code_of(PRODUCT[0].text);
    let item = |name: &str| items(&pp).into_iter().find(|i| i.name == name).unwrap();
    let curved = item("add_curved_bend_thermal_equivalent_load");
    assert!(calls(&curved.body, "ledger.push_product(").next().is_some());
    assert!(calls(&curved.body, "ledger.push(").next().is_none());
    let pressure = item("push_exact_pressure_operands");
    assert!(pressure.body.contains("assembled_operands"));
    assert!(!pressure.body.contains("assembled_loads"));
}

#[test]
fn rule_6_kernel_functions_that_touch_the_force_are_the_listed_sites() {
    let mut actual = BTreeSet::new();
    for source in KERNEL {
        for item in items(&code_of(source.text)) {
            if FORCE_TOKENS.iter().any(|t| item.body.contains(t)) {
                actual.insert((source.name.to_string(), item.name));
            }
        }
    }
    let expected: BTreeSet<_> = FORCE_FUNCTIONS
        .iter()
        .map(|(f, n, _)| (f.to_string(), n.to_string()))
        .collect();
    assert_eq!(actual, expected, "rule 6 force-function list");
}

/// sparse_direct (carried item): no typed sibling is needed. SA's typed
/// `solve_assembled` prepares through `prepare_assembled_structural`, and the
/// sparse factor takes that `PreparedSystem`, which carries the ledger
/// binding (KS1, KS3 and the load audit). The sparse file itself never reads a
/// force outside its tests. Behaviourally backed by the sparse-mode F-tests.
#[test]
fn sparse_direct_factor_inherits_the_prepared_ledger_binding() {
    let sparse = code_of(KERNEL[6].text);
    let factor = items(&sparse)
        .into_iter()
        .find(|item| item.name == "factor_structural_ldlt")
        .expect("factor_structural_ldlt present");
    assert!(factor.signature.contains("PreparedSystem"));
    assert!(
        !sparse.contains("force"),
        "sparse_direct/structural.rs reads a force"
    );
    let adapter = code_of(KERNEL[4].text);
    let typed = items(&adapter)
        .into_iter()
        .find(|item| item.name == "solve_assembled")
        .expect("SA solve_assembled present");
    assert!(typed.body.contains("prepare_assembled_structural("));
    let prepared = items(&adapter)
        .into_iter()
        .find(|item| item.name == "solve_prepared")
        .expect("SA solve_prepared present");
    assert!(prepared.body.contains("factor_structural_ldlt(&prepared)"));
}

#[test]
fn rule_8_every_accumulation_in_the_listed_files_is_in_the_table() {
    let mut actual: BTreeMap<(String, String), usize> = BTreeMap::new();
    for source in all_sources().filter(|s| RULE8_FILES.contains(&s.name)) {
        for (function, count) in scan(source.text) {
            actual.insert((source.name.to_string(), function), count);
        }
    }
    let mut expected: BTreeMap<(String, String), usize> = BTreeMap::new();
    for &(file, function, count, disposition) in TABLE {
        assert!(!disposition.is_empty());
        assert!(RULE8_FILES.contains(&file), "unknown file {file}");
        let previous = expected.insert((file.to_string(), function.to_string()), count);
        assert!(previous.is_none(), "duplicate table row {file} {function}");
        if count == 0 {
            let source = all_sources().find(|s| s.name == file).unwrap();
            assert!(
                items(&code_of(source.text))
                    .iter()
                    .any(|i| i.name == function),
                "{file}: {function} not found"
            );
        }
    }
    let mut problems = Vec::new();
    for (key, count) in &actual {
        match expected.get(key) {
            Some(e) if e == count => {}
            Some(e) => problems.push(format!("{key:?}: table {e}, source {count}")),
            None => problems.push(format!("{key:?}: {count} unlisted accumulation(s)")),
        }
    }
    for (key, &count) in &expected {
        if count > 0 && !actual.contains_key(key) {
            problems.push(format!("{key:?}: table {count}, source 0"));
        }
    }
    assert!(
        problems.is_empty(),
        "site table mismatch:\n{}",
        problems.join("\n")
    );
}

#[test]
fn scanner_self_checks() {
    // A commented or quoted forbidden call is not a call.
    let code =
        code_of("fn a() { // global_load_vector(\n let s = \"reduce_system(\"; b.push(1); }");
    let item = &items(&code)[0];
    assert!(calls(&item.body, "global_load_vector(").next().is_none());
    assert!(calls(&item.body, "reduce_system(").next().is_none());
    // `reduce_system(` does not match `reduce_assembled_system(` or
    // `xreduce_system(`.
    let code = code_of("fn b() { reduce_assembled_system(k); xreduce_system(k); }");
    assert!(calls(&items(&code)[0].body, "reduce_system(")
        .next()
        .is_none());
    // Rule 3 sees names and `let` initializers; rule 2 sees copies.
    let code = code_of(
        "fn c(f: &AssembledForce) { let mut v = f.values().to_vec(); v[0] += 1.0; let mut load_sum = 0.0; load_sum -= 2.0; let mut n = 0; n += 1; }",
    );
    let item = &items(&code)[0];
    let hits: BTreeSet<_> = force_mutations(&item.body)
        .into_iter()
        .map(|h| h.0)
        .collect();
    assert_eq!(
        hits,
        ["load_sum".to_string(), "v".to_string()]
            .into_iter()
            .collect()
    );
    assert_eq!(values_copies(&item.body), 1);
    // Rule 5 finds pushes on a `&mut LoadLedger` parameter or a local ledger.
    let code = code_of(
        "fn d(ledger: &mut LoadLedger, v: &mut Vec<f64>) { ledger.push(\"a\", 0, 1.0); v.push(2.0); }\nfn e() { let mut l = LoadLedger::new(); l.push_product(\"a\", 0, 1.0, 2.0); }",
    );
    let found = items(&code);
    assert_eq!(ledger_pushes(&found[0]), 1);
    assert_eq!(ledger_pushes(&found[1]), 1);
    // Rule 8 counts RV1-N1's self-assignment folds.
    let counts = scan(
        "fn f(a: &mut [f64], t: &[f64]) { a[0] = a[0] + t[0]; let b = checked_value(b - t[1])?; }",
    );
    assert_eq!(counts.get("f"), Some(&2));
}
