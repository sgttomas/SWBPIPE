//! S11 site test, S11-F part (S11_CONTAINMENT revision 5a.2, section 4.3
//! rules 1-8; RV1-N1, RV1-N6 and the sparse_direct item).
//!
//! It reads the product facade (`PP`, `source_recovery`, `source_receipt` and
//! its non-test submodules, `pressure_runtime`, `self_weight`, and K-D5's
//! `certified_loads` and `certified_publication`), the kernel seams (`FK`,
//! including K-D5's `formation_check` and `certified_recovery`, `SA`,
//! `nonlinear_integration`, `sparse_direct/structural.rs`) and, for rule 8,
//! `SP`, `CB` and `load_case_algebra`. Every check runs on
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
//! - rule 3 and the E-sites of rule 8: `f2_*` (E5, E7, E12) and `f8_*`
//!   (E8-E11); E15 and E16, the legacy pressure sums, were retired with
//!   legacy pressure (U3);
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
        name: "PP/retained_product.rs",
        text: include_str!("../src/retained_product.rs"),
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
    Source {
        name: "PP/formation_guard.rs",
        text: include_str!("../src/formation_guard.rs"),
    },
    // K-D5 (DEL-04-01 kd5-member-relative-bound.md §§1, 3, 6): the product's
    // load-closure producers and its certified recovery publication. Appended
    // so the indices above are unchanged.
    Source {
        name: "PP/certified_loads.rs",
        text: include_str!("../src/certified_loads.rs"),
    },
    Source {
        name: "PP/certified_publication.rs",
        text: include_str!("../src/certified_publication.rs"),
    },
    // T4-U2: the arc pressure recovery (no ledger term; its producer is in
    // lib.rs); appended so the indices above are unchanged.
    Source {
        name: "PP/bend_pressure.rs",
        text: include_str!("../src/bend_pressure.rs"),
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
    // K-D5: the D-5 formation check reads the case force (the folded values
    // or the ledger terms) to form its residual; appended so the indices
    // above are unchanged.
    Source {
        name: "FK/structural/formation_check.rs",
        text: include_str!("../../solver/frame_kernel/src/structural/formation_check.rs"),
    },
    // K1: the sparse representation's gate reads the case force (KS1, KS2,
    // E12 from sparse rows, the load audit); appended so the indices above
    // are unchanged.
    Source {
        name: "FK/structural/sparse.rs",
        text: include_str!("../../solver/frame_kernel/src/structural/sparse.rs"),
    },
    // K-D5 (DEL-04-01 kd5-member-relative-bound.md §§3, 6): the certified
    // recovery at p reads the ledger's classification and re-forms the load
    // primitives; appended so the indices above are unchanged.
    Source {
        name: "FK/structural/certified_recovery.rs",
        text: include_str!("../../solver/frame_kernel/src/structural/certified_recovery.rs"),
    },
    // Nonlinear K-D5 (DEL-04-01 kd5-nonlinear-active-set.md §1.6): the
    // confirmation appends the sliding-friction terms to the case ledger;
    // appended so the indices above are unchanged.
    Source {
        name: "nonlinear_integration/confirmation.rs",
        text: include_str!("../../solver/nonlinear_integration/src/confirmation.rs"),
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
    // T4-U1 (T4-RV12 N-1): CB's stable small-angle series, called by
    // CB/lib.rs's formation and load functions.
    Source {
        name: "CB/arc_integrals.rs",
        text: include_str!("../../solver/curved_bend/src/arc_integrals.rs"),
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
    ("PP/lib.rs", "append_nonlinear_support_loop_results", "the loop's `input.force` copy; the typed entry `solve_active_set_frame_with_mode_and_springs_assembled` refuses it unless it equals the ledger's values bit for bit"),
];

/// Rule 3: a compound assignment or `.iter_mut()` on a binding whose name
/// contains `force`, `rhs` or `load` (any case), or that a `let` in the same
/// function binds from an expression mentioning `values()`, `force` or `rhs`.
/// (file, function, binding, disposition).
const ALLOW_FORCE_MUTATION: &[(&str, &str, &str, &str)] = &[
    ("PP/lib.rs", "run_linear_static_preview_observed_inner", "load_case_solves", "per-case result records, not a force vector"),
    ("PP/lib.rs", "recover_curved_bend_local_forces", "global_forces", "E8/E9: the row loop writes each row's one exact sum"),
    ("PP/lib.rs", "recover_curved_bend_local_forces", "local_forces", "the end tangent rotation of the recovered global end forces (one formed transform, section 2.2)"),
    ("FK/lib.rs", "reduced_right_hand_side", "adjusted_force", "KS2: the legacy row expression, kept only for the option (c) binary64 variant and legacy rows with no nonzero prescribed product (b - (+-0) is exact); ledger rows are always exact"),
    ("FK/lib.rs", "solve_dense", "rhs", "generic linear-algebra kernel (limit 1)"),
    ("FK/lib.rs", "solve_dense", "sum", "generic linear-algebra kernel (limit 1): back substitution"),
    ("FK/structural.rs", "finish_checked_factor", "y", "refinement update of the solution, bound from the prepared right-hand side (not a load sum)"),
    ("nonlinear_integration/lib.rs", "solve_iteration_with_sliding_friction_evidence", "unit_force", "T5's: unit-force influence solves (R3-N1)"),
    ("nonlinear_integration/lib.rs", "add_applied_forces", "force", "T5's: E14 in-loop sliding-friction forces (S11 section 10 item 1)"),
];

/// Rule 4: product calls to the generic linear-algebra kernels (limit 1). E14
/// retired the DEC-050/053 observation lane, their only product callers.
const ALLOW_GENERIC_KERNEL: &[(&str, &str, &str)] = &[];
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

/// Rule 5: the functions that write into a `LoadLedger` (a push, or K-D5's
/// closure manifest and supplied primitives, directly or by forwarding a
/// ledger parameter) are exactly the section 4.2 producers (plus T1's
/// retained-source fold check, section 4.5, and K-D5's closure writers).
const PRODUCERS: &[(&str, &str, &str)] = &[
    ("PP/lib.rs", "push_nodal_loads", "nodal loads: one term per load"),
    ("PP/lib.rs", "add_uniform_element_loads", "straight and curved uniform equivalents: one term per (load, DOF)"),
    ("PP/lib.rs", "add_thermal_equivalent_loads", "straight thermal pairs and T1 eigen pairs: fl(P*x_a) per axis"),
    ("PP/lib.rs", "add_curved_bend_thermal_equivalent_load", "curved thermal: push_formed_product(K_rc, fl(eps*chord_c)) per nonzero column (S11-G: the same exact product term)"),
    ("PP/lib.rs", "add_curved_bend_pressure_equivalent_load", "T4-U2 (H-2): each arc's K_b*u_free(eps_p), push_formed_product(K_rc, fl(eps_p*chord_c)) per nonzero column"),
    ("PP/lib.rs", "push_exact_pressure_operands", "exact pressure: each source group's operand"),
    ("PP/lib.rs", "add_constant_effort_support_loads", "constant effort: one term per application"),
    ("PP/lib.rs", "add_connector_reference_loads", "T4-U3 (S13): each connector's +B^T K q_ref, one formed term per nonzero DOF"),
    ("PP/lib.rs", "add_tie_rod_reference_loads", "T4-U5b (DEL-04-07 u4b-u5b-implementation.md §3.2): each tie rod's +C^T k e_ref, one formed term per DOF (none when e_ref = 0)"),
    ("PP/source_recovery.rs", "prepare_sources", "T1 site (section 4.5): the retained nodal terms, in a ledger compared with the actual force"),
    ("PP/source_recovery.rs", "close_load_state", "T1 site (section 4.5): the retained eigen terms, the same terms the product pushes"),
    // K-D5 (DEL-04-01 kd5-member-relative-bound.md §1): the live nodal
    // producer and the closure metadata writers. `push_nodal_loads` above now
    // serves only the retained-source replay (`nodal_and_eigen_case_force`).
    ("PP/certified_loads.rs", "nodal", "live nodal producer (`case_force_ledger`): an unscaled authored load is a plain input term; a load-source-factored load is a formed RoundedProduct term with its NodalProduct summand"),
    ("PP/certified_loads.rs", "spring_base_motion", "W-16 R-c (DEL-04-03 pf-nonlinear-imposed-motion.md §11.1; called from `solve_prepared_load_case`): each moved spring's fl(k*g), a formed RoundedProduct term with its NodalProduct summand; no term when g compares equal to zero"),
    ("PP/certified_loads.rs", "attach", "closure metadata (§1 complete summands): installs the term's source-enumerated manifest, then attaches the supplied primitives; pushes no term"),
    ("PP/certified_loads.rs", "attach_last", "closure metadata: `attach` on the last pushed term (forwards its ledger)"),
    // Nonlinear K-D5 (DEL-04-01 kd5-nonlinear-active-set.md §1.4, §1.6): the
    // confirmation's appended ledgers, never the case force the loop solves.
    ("nonlinear_integration/confirmation.rs", "appended_ledger", "the sliding-friction terms appended after the case ledger: a formed RoundedProduct (explicit) or Bounded (coupled Stage-B ball) term with its source-enumerated manifest, or the Stage-A input φ⁰"),
    ("nonlinear_integration/confirmation.rs", "stage_a", "the influence system's unit right-hand side e_d, an input term of its own ledger"),
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
        "build",
        "stores no force; initializes the C3-detect terms to None (T4-U3: `new`'s body, moved into the shared `build` of the connector entries)",
    ),
    (
        "SA/structural_adapter.rs",
        "with_force_terms",
        "C3-detect binding",
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
    // K-D5's closure (DEL-04-01 kd5-member-relative-bound.md §1): `check` and
    // the sparse `formation_check` now pass the whole ledger (`AssembledForce`)
    // through untouched and are no longer force sites; with no ledger the
    // evaluation fails closed instead of folding the force.
    (
        "FK/structural/formation_check.rs",
        "evaluate",
        "K-D5: the residual rho = f_int - K_int u starts from the ledger's input terms and the closure's re-formed primitives (no ledger fails closed), in one exact sum per free row; read-only",
    ),
    (
        "FK/structural/sparse.rs",
        "assembled",
        "typed constructor: the force slice is the ledger's values",
    ),
    (
        "FK/structural/sparse.rs",
        "validate_sparse",
        "read-only: length and finiteness",
    ),
    (
        "FK/structural/sparse.rs",
        "sparse_contribution_sums",
        "read-only: length",
    ),
    (
        "FK/structural/sparse.rs",
        "prepare_sparse_structural_with_force_terms",
        "C3-detect entry point",
    ),
    (
        "FK/structural/sparse.rs",
        "prepare_sparse_bound",
        "KS1 dispatch (exact on ledger and coupled rows)",
    ),
    (
        "FK/structural/sparse.rs",
        "legacy_zero_product_fold",
        "KS1 legacy rows with no nonzero prescribed product: reads the one force value; b - (+-0) reduced to its range check and zero-sign rule, no fold",
    ),
    (
        "FK/structural/sparse.rs",
        "audit_sparse_load_fidelity",
        "S11 section 5.1 load audit over the pattern (rows through FK/structural.rs audit_load_row)",
    ),
    (
        "FK/structural/sparse.rs",
        "negative_verdict",
        "read-only: length",
    ),
    (
        "FK/structural/sparse.rs",
        "reactions",
        "E12 from sparse rows: the formed K*u and the DOF's ledger terms in one exact sum",
    ),
    (
        "FK/structural/sparse.rs",
        "reduce_assembled_sparse_system",
        "KS2 over the pattern: one exact sum per free row",
    ),
    // K2b (ROOT's K2b ruling 5: a declared, additive extension, tests only):
    // the force-scaled functions that read the case force.
    (
        "FK/structural.rs",
        "unscale_structural_solution",
        "K2b: republishes a flagged load-audit row's exact net and actual force bits from the unscaled ledger (one exact sum of the DOF's terms); read-only",
    ),
    (
        "FK/structural/sparse.rs",
        "force_scaled_reactions",
        "K2b: E12 at 2^b from sparse rows, the formed K*u and the DOF's ledger terms at 2^b in one exact sum, rounded once at 2^-b",
    ),
    (
        "SA/structural_adapter.rs",
        "force_scale_census",
        "K2b: reads each ledger term's exponent for the b-rule's census; read-only",
    ),
    // K-D5 (DEL-04-01 kd5-member-relative-bound.md §§1-3, 6): every function
    // that forms a load at p, binds its closure, or recovers at p from the
    // ledger's classification and primitives. None reads a folded ledger value
    // (`rule_6b_kd5_takes_no_folded_ledger_value`).
    (
        "FK/load_ledger.rs",
        "set_closure_manifest",
        "K-D5 §1: stores a formed term's source-enumerated manifest; refuses an input term",
    ),
    (
        "FK/load_ledger.rs",
        "attach_load_primitive",
        "K-D5 §1: stores one supplied primitive; the manifest comparison happens in FK's load_closure",
    ),
    ("FK/load_ledger.rs", "load_primitives", "K-D5: read-only accessor of the supplied primitives"),
    ("FK/load_ledger.rs", "closure_records", "K-D5: read-only accessor of the trusted input/formed classification and manifests"),
    (
        "FK/structural/formation_check.rs",
        "load_closure",
        "K-D5 §1: compares each formed term's manifest with its supplied primitive multiset and bindings (terms and records read for classification and identity only); fails closed",
    ),
    (
        "FK/structural/formation_check.rs",
        "load_reformation",
        "K-D5 §1: re-forms one primitive's 12 element rows at p from its binary64 operands (with its stack helpers uniform_straight_balls, curved_strain_balls and pressure_force, which take geometry and balls only); no ledger value",
    ),
    ("FK/structural/formation_check.rs", "intensity_reformation", "K-D5 §1: a uniform primitive's intensity at p"),
    ("FK/structural/formation_check.rs", "intensity_balls", "K-D5 §1: authored, self-weight, product, seismic and wind intensities at p"),
    ("FK/structural/formation_check.rs", "mass_reformation", "K-D5 §1: self-weight mass per length at p from the section's authored operands"),
    ("FK/structural/formation_check.rs", "eigenstrain_reformation", "K-D5 §1: the resolved total eigenstrain at p from its thermal and fit primitives"),
    ("FK/structural/formation_check.rs", "eigenstrain_is_exact_zero", "K-D5 §1 replaced-span source zero: raw exact-zero test of a primitive; read-only"),
    ("FK/structural/formation_check.rs", "table_value", "K-D5 §1: a tabulated thermal measure at p"),
    ("FK/structural/formation_check.rs", "table_integral", "K-D5 §1: a tabulated thermal integral at p"),
    ("FK/structural/formation_check.rs", "closure_entry", "K-D5 §1: a primitive's identity and exact operand binding; read-only"),
    ("FK/structural/formation_check.rs", "intensity_binding", "K-D5 §1: exact operand bits of an intensity; read-only"),
    ("FK/structural/formation_check.rs", "mass_binding", "K-D5 §1: exact operand bits of a mass primitive; read-only"),
    ("FK/structural/formation_check.rs", "points_binding", "K-D5 §1: exact operand bits of a thermal table; read-only"),
    ("FK/structural/formation_check.rs", "strain_binding", "K-D5 §1: exact operand bits of an eigenstrain; read-only"),
    // T4-U8 (DEL-05-03 Design §4 item 2): the frame-binding check of the
    // closure. Each names a load primitive or formula in its signature and
    // reads only its frame's bits and its source identity; none forms, folds or
    // reads a force value.
    ("FK/structural/formation_check.rs", "straight_primitive_frame", "T4-U8 §4 item 2: a straight primitive's frame (uniform, free strain, straight pressure operand); read-only"),
    ("FK/structural/formation_check.rs", "unbound_straight_primitive", "T4-U8 §4 item 2: the first straight primitive whose frame is not its member's FormationSource frame; read-only"),
    ("FK/structural/formation_check.rs", "primitive_frame_bound", "T4-U8 §4 item 2: a straight primitive's frame bit-equal, beam-theory leaf included, to its own member's source frame at its force exponent (keyed by FormationSource::frame_members); read-only"),
    (
        "FK/structural/formation_check.rs",
        "binding",
        "K-D5 §1: LoadFormula::binding, exact operand bits of a primitive's formula for its closure entry; read-only",
    ),
    (
        "FK/structural/formation_check.rs",
        "force_scaled",
        "K-D5 §2.2: LoadPrimitive::force_scaled, rebinds one primitive's force exponent with its term's 2^b (W2); authored operands unchanged, no ledger value",
    ),
    (
        "FK/structural/certified_recovery.rs",
        "from_primitives",
        "K-D5 §3.1: unqualified member-load constructor (delegates to from_member_primitives)",
    ),
    (
        "FK/structural/certified_recovery.rs",
        "from_member_primitives",
        "K-D5 §3.1: the member's intended end loads, each primitive re-formed at p and added to its row's exact sum",
    ),
    ("FK/structural/certified_recovery.rs", "member_pair", "K-D5 §3.1: a primitive's member nodes; read-only"),
    (
        "FK/structural/certified_recovery.rs",
        "default",
        "K-D5 §3.1: Default for CertifiedMemberLoads, the unloaded member (zero end loads, no primitives)",
    ),
    (
        "FK/structural/certified_recovery.rs",
        "add",
        "K-D5 §3.1: RowLoad::add, one load contribution entered as its own exact addend of the row's sum, radius added upward",
    ),
    (
        "FK/structural/certified_recovery.rs",
        "single",
        "K-D5 §3.1: RowLoad::single, one load ball as the row's only (subtracted) load addend",
    ),
    (
        "FK/structural/certified_recovery.rs",
        "evaluate_row",
        "K-D5 §3.1: a row A x - load with one load addend (RowLoad::single), through evaluate_row_inner",
    ),
    (
        "FK/structural/certified_recovery.rs",
        "evaluate_row_inner",
        "K-D5 §3.1: the row's exact sum starts from its RowLoad addends (moved, not copied) and the stiffness products of u, w and w2 join it; one rounding",
    ),
    (
        "FK/structural/certified_recovery.rs",
        "evaluate_ends",
        "K-D5 §3.1: the 12 end rows, each with its own end-load addend (evaluate_row)",
    ),
    ("FK/structural/certified_recovery.rs", "uniform_loads", "K-D5 §3.1: each uniform primitive's intensity re-formed at p for the station transfer"),
    ("FK/structural/certified_recovery.rs", "local_loads", "K-D5 §3.1: the member's intended end loads in its chord frame"),
    ("FK/structural/certified_recovery.rs", "straight_context", "K-D5 §3.1: straight recovery context (K_int,m and the chord-frame loads)"),
    ("FK/structural/certified_recovery.rs", "straight_section_rows", "K-D5 §3.1: section rows at p; each load contribution is its own exact addend"),
    ("FK/structural/certified_recovery.rs", "straight_section_evaluation_envelopes", "K-D5 §3.2: positive majorant of the section recipe's evaluation error"),
    ("FK/structural/certified_recovery.rs", "certify_straight_recovery", "K-D5 §3.1: straight end and station rows at u + w + w2"),
    ("FK/structural/certified_recovery.rs", "certify_straight_section", "K-D5 §3.1: one straight section by the same recipe"),
    ("FK/structural/certified_recovery.rs", "certify_straight_stress_spans", "K-D5 §3.2: Bernstein stress controls from the certified recovery and the raw uniform primitives"),
    ("FK/structural/certified_recovery.rs", "certify_arc_recovery", "K-D5 §3.1: arc end and station rows through K_int,arc"),
    ("FK/structural/certified_recovery.rs", "tangent_end_operator_and_loads", "U4a: rotate intended end loads in ball arithmetic before evaluating and publishing the certified tangent rows"),
    ("FK/structural/certified_recovery.rs", "certify_connector_recovery", "K-D5 §3.1: connector rows through connector_matrix"),
    ("FK/structural/certified_recovery.rs", "straight_member_frames_bound", "T4-U8 §4 item 2: the straight recovery frame bit-equal to its member's FormationSource frame (CertifiedMemberLoads::with_member_source) and to every straight primitive frame of the member; reads frame bits and source identities only; read-only"),
    (
        "FK/structural/certified_recovery.rs",
        "certify_spring_reaction",
        "K-D5 §3.2, W-16 R-c: spring row -k (u + w + w2 - g); a moved base's exact product +k*g is the row's own load addend (no binary64 fold of k*g or u - g); g equal to zero is the fixed-base row",
    ),
    (
        "FK/structural/certified_recovery.rs",
        "certify_support_reaction",
        "K-D5 §3.2: restrained reaction K_int,r (u + w + w2) - f_int,r; a formed term enters as its re-formed primitives, an input term as its exact binary64 value (term.accumulate), each its own exact addend",
    ),
    (
        "SA/structural_adapter.rs",
        "solve_assembled_with_formation_check",
        "K-D5 §1: attaches the ledger's supplied primitives to FormationSource::loads; read-only",
    ),
    (
        "SA/structural_adapter.rs",
        "solve_force_scaled_with_formation_check",
        "K-D5 §2.2: the W2 entry; attaches the primitives (rescaled with their terms by force_scaled) to FormationSource::loads; read-only",
    ),
    // Nonlinear K-D5 (DEL-04-01 kd5-nonlinear-active-set.md §1.5, §1.6).
    (
        "SA/structural_adapter.rs",
        "confirm_with_formation_check",
        "nonlinear K-D5 §1.5: the confirmation entry; prepares the gate exactly as solve_assembled and attaches each evaluated ledger's primitives to FormationSource::loads; read-only",
    ),
    (
        "FK/structural.rs",
        "evaluate_at",
        "nonlinear K-D5 §1.5: hands the evaluated system's ledger to the closed check (formation_check::check_at); read-only",
    ),
    ("nonlinear_integration/confirmation.rs", "friction_primitive", "nonlinear K-D5 §1.6: one sliding-friction summand's identity and row; forms no value"),
    ("nonlinear_integration/confirmation.rs", "explicit_formula", "nonlinear K-D5 §1.6: the explicit-normal friction formula's operands; forms no value"),
    ("nonlinear_integration/confirmation.rs", "coupled_formula", "nonlinear K-D5 §1.6: the coupled friction formula's operands (the Stage-A ball); forms no value"),
    (
        "FK/structural/certified_recovery.rs",
        "certify_load_primitive",
        "nonlinear K-D5 §3.1: one sliding-friction primitive re-formed at p and rounded once; read-only",
    ),
    ("PP/certified_loads.rs", "constant_alpha", "K-D5 §1: a constant-alpha eigenstrain primitive from its authored operands"),
    ("PP/certified_loads.rs", "eigenstrain", "K-D5 §1: the resolved eigenstrain primitive from the case's thermal and fit inputs"),
    ("PP/certified_loads.rs", "member_eigenstrain", "K-D5 §1: binds a natural-length fit to its member's raw geometry"),
    ("PP/certified_loads.rs", "intensity", "K-D5 §1: a uniform load's intensity primitive (authored, source-factored or generated)"),
    ("PP/certified_loads.rs", "mass", "K-D5 §1: self-weight mass primitive from the section's authored operands"),
    ("PP/certified_loads.rs", "summand", "K-D5 §1: one source-enumerated summand identity and formula"),
    (
        "PP/certified_publication.rs",
        "source",
        "K-D5 §3: the recovery's FormationSource, with the ledger's supplied primitives; read-only",
    ),
    (
        "PP/certified_publication.rs",
        "member",
        "K-D5 §3.1: one member's certified rows from its re-formed primitives; read-only",
    ),
    (
        "PP/certified_publication.rs",
        "fallback_section_errors",
        "K-D5 §3.2: whole-member error replay of the binary64 E4/E6 section recipe from the held end actions and the member's local uniform intensities (SpannedUniformLocalLoad); no ledger value",
    ),
    (
        "PP/certified_publication.rs",
        "fallback_circular_maximum",
        "K-D5 §3.2: the fallback circular stress maximum over the member's local uniform intensities (through fallback_section_errors); no ledger value",
    ),
    (
        "PP/certified_publication.rs",
        "straight_stress_spans",
        "K-D5 §3.2: certified stress controls from the member's re-formed primitives; read-only",
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
    // K-D5: the ledger bound under its own name, the closure metadata, the
    // re-formation of a load at p and an input term's exact entry.
    "ledger.values()",
    "ledger.terms()",
    ".closure_records()",
    ".load_primitives()",
    "load_reformation(",
    "term.accumulate(",
    // K-D5 §3.1: a recovered row's load addends (see FORCE_SIGNATURE_TOKENS).
    "RowLoad",
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
    ("PP/lib.rs", "append_nonlinear_residual_observation_results", 3, "max folds of residual observations"),
    ("PP/lib.rs", "append_spring_hanger_user_input_results", 3, "integer: appended count"),
    ("PP/lib.rs", "assemble_case_stiffness", 1, "stiffness assembly (spring diagonal)"),
    ("PP/lib.rs", "compute_pipe_mass_per_length", 2, "declared formation: same-sign mass terms (section 2.2, as self_weight)"),
    ("PP/lib.rs", "curved_bend_section_resultants", 1, "formed transform: chord frame back to global (section 2.2); E11 is the terms API call"),
    ("PP/lib.rs", "debit", 2, "integer: budget charge"),
    ("PP/lib.rs", "recover_curved_bend_local_forces", 1, "formed transform: the end tangent rotation after the E8/E9 exact sums"),
    ("PP/lib.rs", "reserve_publication", 2, "integer: budget charge"),
    ("PP/lib.rs", "run_linear_static_preview_observed_inner", 1, "integer per-case component-stress-modifier count; H-4 C1–C3 deleted the retired, always-zero pressure-thrust count; F1b: the spring diagonals moved into the kernel's sparse assembly (`assemble_basis_stiffness`)"),
    ("PP/lib.rs", "solve_load_case_observed", 0, "validated pressure preparation and delegation to the private prepared solve; no force accumulation"),
    ("PP/lib.rs", "solve_prepared_load_case", 2, "integer: source-budget attempts and component modifier count, moved unchanged from solve_load_case_observed"),
    ("PP/lib.rs", "straight_local_uniform_loads", 1, "declared formation: one load's local transform (section 2.5 PP:7479)"),
    ("PP/lib.rs", "straight_summary_extrema", 1, "declared: the stress bound's same-sign pattern sums (section 2.2)"),
    ("PP/lib.rs", "exact_straight_end_forces", 0, "E5"),
    ("PP/lib.rs", "exact_straight_summary_extrema", 0, "E7"),
    ("PP/lib.rs", "restrained_reactions", 0, "E12 (F1b: `SparseStiffness::reactions`, whose formed row is K1's `multiply`)"),
    ("PP/lib.rs", "curved_bend_uniform_intensities_by_pipe", 0, "E10 removed: one intensity per load"),
    ("PP/lib.rs", "push_nodal_loads", 0, "producer"),
    ("PP/lib.rs", "add_uniform_element_loads", 0, "producer"),
    ("PP/lib.rs", "add_thermal_equivalent_loads", 0, "producer"),
    ("PP/lib.rs", "add_curved_bend_thermal_equivalent_load", 0, "producer"),
    ("PP/lib.rs", "add_curved_bend_pressure_equivalent_load", 0, "producer"),
    ("PP/lib.rs", "push_exact_pressure_operands", 0, "producer"),
    ("PP/lib.rs", "add_constant_effort_support_loads", 0, "producer"),
    // ---- PP/pressure_runtime.rs
    ("PP/pressure_runtime.rs", "finish_source_groups", 1, "max fold of the pressure-RHS screen magnitude"),
    ("PP/pressure_runtime.rs", "traverse_region", 2, "integer: reindex the two arc-junction member positions by subtracting counts of removed virtual joint edges; no load formation"),
    ("PP/pressure_runtime.rs", "collinear_run", 1, "geometry: chord projection (section 2.5; T4-U2 moved it from traverse_region, unchanged)"),
    // ---- PP/retained_product.rs (B1 SQ, RR "RV109 passes SP in RV-P round 2; …", ruling 3: E-12)
    ("PP/retained_product.rs", "check_support_maps", 1, "integer: one support's spring-map count"),
    // B4/D1 (b): the scan attributes this fold (and `load_case_solves` to the `_inner`
    // function that holds it) since the legacy-source records left the scanned files.
    ("PP/retained_product.rs", "pending_error_bytes", 1, "integer: the parked association errors' byte capacity"),
    // B2-P's combination Call (lane P), met by B1 SQ's table at J0a's merge.
    ("PP/retained_product.rs", "combination_call", 1, "integer: the next attempt ordinal"),
    ("PP/retained_product.rs", "selected_attempts", 1, "integer: the selected attempts' bit set (T-12's detail)"),
    // T4-U6 (D-5): W1-E binds nominal rows only; these count the reduced twins
    // passed, mapping a published row to its bound row index.
    ("PP/retained_product.rs", "bound_row_indices", 1, "integer: the reduced twins passed (D-5 published-to-bound row index)"),
    ("PP/retained_product.rs", "apply_prepared_overlay", 1, "integer: the reduced twins passed (D-5 published-to-bound row index)"),
    // T4-U4b S4 (§4.1): the not-covered derived rows' formation; no f64 accumulation.
    ("PP/retained_product.rs", "prepared_not_covered", 2, "integer: the pending not-covered rows' count and the published-to-bound row index"),
    // O4 (DEL-04-08 Design `o4-source-extension.md` §4.5): locating a recipe's nth bound row
    // (a distinct-basis member's maximum on its reduced section); no f64 accumulation.
    ("PP/retained_product.rs", "prepared_row_index_nth", 1, "integer: the matching recipes passed (the nth bound row of a recipe)"),
    // ---- PP/self_weight.rs
    ("PP/self_weight.rs", "stable_mass_per_length", 2, "declared formation: same-sign mass (section 2.2)"),
    // ---- PP/source_recovery.rs
    ("PP/source_recovery.rs", "prepare_sources", 1, "stiffness coverage fold (stiffness, not loads; section 2.5 :761)"),
    ("PP/source_recovery.rs", "solve", 2, "max folds of retained-source observations"),
    // ---- PP/source_receipt.rs
    ("PP/source_receipt.rs", "check_input_with_physical", 1, "stiffness assembly (spring diagonal) of the replay"),
    ("PP/source_receipt.rs", "finalization_size", 2, "integer: byte sizes"),
    ("PP/source_receipt.rs", "finalize_for", 3, "integer: byte sizes and qualified count"),
    // ---- SP, CB, load_case_algebra (also in frame_kernel/tests/s11_site_table.rs)
    ("SP/lib.rs", "transform_global_displacements_to_local", 1, "formed transform (section 2.2)"),
    ("SP/lib.rs", "transform_local_element_vector_to_global", 1, "formed transform (section 2.2)"),
    ("SP/lib.rs", "multiply_matrix_vector", 1, "formed elastic term K_e*u (section 2.2)"),
    ("SP/lib.rs", "equivalent_nodal_load_terms_with_spans", 0, "E1 per-load terms (E5's input)"),
    ("CB/lib.rs", "consistent_uniform_nodal_loads", 2, "one source's consistent equivalent (section 2.2)"),
    ("CB/lib.rs", "cross_quad", 1, "flexibility quadrature (stiffness formation)"),
    ("CB/lib.rs", "rotate_to_global", 1, "formed rotation"),
    ("CB/lib.rs", "multiply6", 1, "stiffness formation"),
    ("CB/lib.rs", "multiply6_transpose_right", 1, "stiffness formation"),
    ("CB/lib.rs", "flexibility_scale", 1, "max fold of the flexibility diagonal (one exact power-of-two scale)"),
    ("CB/lib.rs", "arc_section_resultant_terms", 0, "E11"),
    ("load_case_algebra/lib.rs", "evaluate_linear_combination", 1, "E13: the fold is kept only as today's non-finite value"),
];
const RULE8_FILES: &[&str] = &[
    "PP/lib.rs",
    "PP/pressure_runtime.rs",
    "PP/retained_product.rs",
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
    let parameters = parameter_ledgers(sig);
    receivers
        .iter()
        .map(|r| {
            let direct: usize = LEDGER_WRITES
                .iter()
                .map(|w| calls(&item.body, &format!("{r}.{w}(")).count())
                .sum();
            // K-D5 (attach_last): a function handed a ledger that passes that
            // ledger on, as a bare argument, writes into it through its callee.
            // A locally created ledger handed to the producers is the case
            // force's assembly, not a producer.
            let forwarded = if parameters.contains(r) {
                forwarded_receiver(&item.body, r)
            } else {
                0
            };
            direct + forwarded
        })
        .sum()
}

/// Every `&mut self` writer of `FK/load_ledger.rs`'s `LoadLedger`: the four
/// pushes and K-D5's closure metadata (manifest and supplied primitives).
const LEDGER_WRITES: &[&str] = &[
    "push",
    "push_product",
    "push_formed",
    "push_formed_product",
    "set_closure_manifest",
    "attach_load_primitive",
];

/// The `&mut LoadLedger` parameter names of a signature.
fn parameter_ledgers(sig: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
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
            out.insert(name);
        }
    }
    out
}

/// Bare uses of `receiver` as a call argument (`f(ledger, ...)`, `f(x,
/// ledger)`, `&mut *ledger`), not as a method receiver.
fn forwarded_receiver(body: &str, receiver: &str) -> usize {
    calls(body, receiver)
        .filter(|&at| {
            let after = body[at + receiver.len()..].trim_start();
            let before = body[..at].trim_end();
            let before = before
                .strip_suffix("&mut *")
                .or_else(|| before.strip_suffix("&mut"))
                .unwrap_or(before)
                .trim_end();
            let ident_end = !body[at + receiver.len()..]
                .starts_with(|c: char| c.is_alphanumeric() || c == '_');
            ident_end
                && (before.ends_with('(') || before.ends_with(','))
                && (after.starts_with(',') || after.starts_with(')'))
        })
        .count()
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
    // K-D5: the typed linear solve now goes through the formation-checked
    // typed entry (`solve_assembled` plus the D-5 check; `selected` false runs
    // `solve_assembled` unchanged). Behavioural backing: the product tests in
    // `tests/formation_check_runtime.rs` (122 demotes on both entries, the
    // controls and the nonlinear-support case do not) and SA's
    // `kd5_not_selected_invocation_runs_the_unchanged_solve_assembled`.
    let linear = body("solve_preview_reduced_system");
    assert!(linear.contains("assembly.solve_assembled_with_formation_check("));
    assert!(linear.contains("built.nonlinear_supports.is_empty()"));
    assert!(!linear.contains("assembly.solve_assembled("));
    // U5a's dormant composition extracted the mechanics body after the
    // document route's pressure preparation. Follow that one private call,
    // preserving the validation gate as well as the typed seams below.
    let wrapper = body("solve_load_case_observed");
    assert_eq!(calls(&wrapper, "build_pressure_case_with_members(").count(), 1);
    let prepared = call_arguments(&wrapper, "solve_prepared_load_case(");
    assert_eq!(prepared.len(), 1);
    assert!(prepared[0].contains("exact_pressure"));
    assert!(wrapper.find("build_pressure_case_with_members(").unwrap()
        < wrapper.find("solve_prepared_load_case(").unwrap());
    let prepared_callers = items(&pp).into_iter().filter_map(|item| {
        let count = calls(&item.body, "solve_prepared_load_case(").count();
        (count > 0).then_some((item.name, count))
    }).collect::<Vec<_>>();
    assert_eq!(prepared_callers, vec![("solve_load_case_observed".to_string(), 1)]);
    let case = body("solve_prepared_load_case");
    assert_eq!(calls(&case, "build_pressure_case_with_members(").count(), 0);
    assert!(!case.contains("reduce_assembled_system_with_prescribed_displacements("));
    assert!(!case.contains("reduce_assembled_system("));
    // E14 retired the DEC-050/053 LU lane and its dense reductions.
    // The ordinary route retains the pattern's partition (F1b §4.8).
    assert!(case.contains("reduce_assembled_sparse_system("));
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
fn rule_4_the_product_calls_no_generic_kernel() {
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
    assert_eq!(
        actual,
        expected,
        "rule 5 producer list; unlisted {:?}; stale {:?}",
        actual.difference(&expected).collect::<Vec<_>>(),
        expected.difference(&actual).collect::<Vec<_>>()
    );
    // Section 4.2 granularity where the push kind matters (M5): the curved
    // thermal producer pushes exact products K_rc * fl(eps * chord_c), never
    // a pre-summed row value; the exact-pressure producer pushes each group
    // operand, never the pre-summed `assembled_loads` total.
    let pp = code_of(PRODUCT[0].text);
    let item = |name: &str| items(&pp).into_iter().find(|i| i.name == name).unwrap();
    let curved = item("add_curved_bend_thermal_equivalent_load");
    assert!(calls(&curved.body, "ledger.push_formed_product(")
        .next()
        .is_some());
    assert!(calls(&curved.body, "ledger.push(").next().is_none());
    assert!(calls(&curved.body, "ledger.push_formed(").next().is_none());
    let pressure = item("push_exact_pressure_operands");
    assert!(pressure.body.contains("assembled_operands"));
    assert!(!pressure.body.contains("assembled_loads"));
}

/// Rule 6 also reads K-D5's product-side load and recovery files (DEL-04-01
/// kd5-member-relative-bound.md §6: every function that forms a load at p,
/// and the recovery at p, is a force site).
const RULE6_PRODUCT_FILES: &[&str] = &["PP/certified_loads.rs", "PP/certified_publication.rs"];

/// Rule 6, K-D5: a function whose parameters or return type name a load
/// primitive or its closure metadata forms, binds or re-forms a load at p
/// even where its body names no ledger binding. (`AssembledForce` alone in a
/// signature is a pass-through, judged by the body tokens above.)
const FORCE_SIGNATURE_TOKENS: &[&str] = &[
    "LoadPrimitive",
    "LoadFormula",
    "LoadClosure",
    "IntensityPrimitive",
    "MassPrimitive",
    "EigenstrainPrimitive",
    "ThermalPointPrimitive",
    "CertifiedMemberLoads",
    "CertifiedUniformLoad",
    // K-D5 §3.1 (9ef0cbc78a): a recovered row's intended load part, each load
    // contribution its own exact addend, joined by the row's stiffness
    // products in `evaluate_row_inner`.
    "RowLoad",
];

/// Rule 6, K-D5 §3.1: a parameter of these names carries a recovered row's
/// load addend(s) into the row's exact sum (`evaluate_row`'s `load`,
/// `evaluate_ends`'s `loads`), so the function is a force site whatever the
/// parameter's type.
const FORCE_PARAMETER_NAMES: &[&str] = &["load:", "loads:"];

fn force_site(item: &Item) -> bool {
    FORCE_TOKENS.iter().any(|t| item.body.contains(t))
        || FORCE_SIGNATURE_TOKENS
            .iter()
            .chain(FORCE_PARAMETER_NAMES)
            .any(|t| calls(&item.signature, t).next().is_some())
}

/// The bodies of the `impl` blocks (inherent or trait) whose implementing type
/// is `ty`: their methods are force sites although a `self` receiver names no
/// type (`RowLoad::add`, `RowLoad::single`).
fn impl_bodies(code: &str, ty: &str) -> Vec<String> {
    let mut out = Vec::new();
    for at in calls(code, "impl") {
        let rest = &code[at + 4..];
        if !rest.starts_with(|c: char| c.is_whitespace() || c == '<') {
            continue;
        }
        let Some(open) = rest.find(['{', ';']) else {
            continue;
        };
        if !rest[open..].starts_with('{') {
            continue;
        }
        let header = rest[..open].split(" where ").next().unwrap_or("");
        let implementing = header.rsplit(" for ").next().unwrap_or(header);
        let path = implementing.split('<').next().unwrap_or("").trim();
        if path.rsplit("::").next() != Some(ty) {
            continue;
        }
        let start = at + 4 + open;
        let mut depth = 0usize;
        for (offset, c) in code[start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        out.push(code[start + 1..start + offset].to_string());
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    out
}

/// Rule 6's force sites of one lexed file.
fn force_sites(code: &str) -> BTreeSet<String> {
    let mut out: BTreeSet<String> = items(code)
        .into_iter()
        .filter(force_site)
        .map(|item| item.name)
        .collect();
    for ty in FORCE_SIGNATURE_TOKENS {
        for body in impl_bodies(code, ty) {
            out.extend(items(&body).into_iter().map(|item| item.name));
        }
    }
    out
}

#[test]
fn rule_6_kernel_functions_that_touch_the_force_are_the_listed_sites() {
    let mut actual = BTreeSet::new();
    let product = PRODUCT
        .iter()
        .filter(|s| RULE6_PRODUCT_FILES.contains(&s.name));
    for source in KERNEL.iter().chain(product) {
        for name in force_sites(&code_of(source.text)) {
            actual.insert((source.name.to_string(), name));
        }
    }
    let expected: BTreeSet<_> = FORCE_FUNCTIONS
        .iter()
        .map(|(f, n, _)| (f.to_string(), n.to_string()))
        .collect();
    assert_eq!(
        actual,
        expected,
        "rule 6 force-function list; unlisted {:?}; stale {:?}",
        actual.difference(&expected).collect::<Vec<_>>(),
        expected.difference(&actual).collect::<Vec<_>>()
    );
}

/// Rule 6b, K-D5 (DEL-04-01 kd5-member-relative-bound.md §1, "No ledger value
/// of a formed load term enters any of them"): K-D5's residual, closure and
/// recovery files never read the ledger's folded binary64 values. A formed
/// term enters only as its re-formed primitives and an input term only as its
/// own exact term (`term.accumulate`), and only on a branch guarded as an
/// input term. Exact: no function may match.
const KD5_FILES: &[&str] = &[
    "FK/structural/formation_check.rs",
    "FK/structural/certified_recovery.rs",
    "PP/certified_loads.rs",
    "PP/certified_publication.rs",
];

/// Folded-value reads: `values()`, an indexed or element read of a force or
/// ledger, the `ForceRows` binding, any `.force.` use other than its length,
/// a per-DOF ledger read or sum (`terms_for`, `accumulate_dof`), the S11-G
/// formation rows (net defects of the binary64 terms), a term's binary64
/// operands (`.kind`, `ForceTermKind`), and a ledger-term accumulation outside
/// an input-term guard (`unguarded_term_accumulations`).
fn folded_value_reads(body: &str) -> usize {
    let tokens = [
        "values()",
        ".force[",
        "force.get(",
        "ledger.get(",
        "force.iter(",
        "ledger.iter(",
        "ForceRows",
        "force_rows",
        "accumulate_dof(",
        "terms_for(",
        "formation_rows(",
        "ForceTermKind",
    ];
    let field = |name: &str| {
        body.match_indices(name)
            .filter(|(at, _)| {
                !body[at + name.len()..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
            })
            .count()
    };
    tokens.iter().map(|t| body.matches(t).count()).sum::<usize>()
        + body.matches(".force.").count()
        - body.matches(".force.len()").count()
        + field(".kind")
        + unguarded_term_accumulations(body)
}

/// Ledger-term accumulations (`ForceTerm::accumulate(&self, &mut acc,
/// negate)`: `.accumulate(` with two or more arguments, or the path form
/// `ForceTerm::accumulate(`; the product's one-argument
/// `FallbackError::accumulate` is not one) that no enclosing block guards as
/// an input term. Only these two guards count, each an `if` whose whole
/// condition is a plain field path ending in `.formed`:
/// - the then-block of `if !<path>.formed { .. }`;
/// - the `else` block of `if <path>.formed { .. } else { .. }`.
/// Any other position (a formed branch, an `else if`, a compound condition,
/// a closure, a `match` arm) counts. Limit: the guard is matched by shape,
/// not by proving that its record is the accumulated term's own.
fn unguarded_term_accumulations(body: &str) -> usize {
    let method = calls(body, ".accumulate(").filter(|&at| {
        let arguments = call_arguments(&body[at..], ".accumulate(")[0].clone();
        let mut depth = 0i32;
        let commas = arguments
            .trim_end()
            .trim_end_matches(',')
            .chars()
            .filter(|&c| {
                match c {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth -= 1,
                    _ => {}
                }
                c == ',' && depth == 0
            })
            .count();
        commas >= 1
    });
    let path = calls(body, "ForceTerm::accumulate(");
    method
        .chain(path)
        .filter(|&at| !input_term_guarded(body, at))
        .count()
}

/// Whether an enclosing block of `at` is the then-block of
/// `if !<path>.formed` or the `else` block of `if <path>.formed`.
fn input_term_guarded(body: &str, at: usize) -> bool {
    let b = body.as_bytes();
    let mut open = Vec::new();
    for (i, &c) in b[..at].iter().enumerate() {
        match c {
            b'{' => open.push(i),
            b'}' => {
                open.pop();
            }
            _ => {}
        }
    }
    // The header of the block opened at `o`: the text back to the previous
    // `;`, `{` or `}`, and that delimiter's position.
    let header = |o: usize| {
        let start = b[..o]
            .iter()
            .rposition(|&c| matches!(c, b';' | b'{' | b'}'))
            .map_or(0, |p| p + 1);
        (body[start..o].trim(), start.checked_sub(1))
    };
    // A condition that is exactly a field path ending in `.formed`.
    let formed_path = |cond: &str| {
        cond.ends_with(".formed")
            && cond
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '_' | '.' | '[' | ']' | '(' | ')'))
    };
    open.iter().rev().any(|&o| {
        let (head, delimiter) = header(o);
        if let Some(cond) = head.strip_prefix("if ") {
            return cond
                .trim()
                .strip_prefix('!')
                .is_some_and(|path| formed_path(path.trim()));
        }
        if head != "else" {
            return false;
        }
        // Match the then-block's `}` back to its `{`.
        let Some(close) = delimiter.filter(|&d| b[d] == b'}') else {
            return false;
        };
        let mut depth = 0usize;
        let then_open = (0..close).rev().find(|&i| match b[i] {
            b'}' => {
                depth += 1;
                false
            }
            b'{' if depth == 0 => true,
            b'{' => {
                depth -= 1;
                false
            }
            _ => false,
        });
        then_open.is_some_and(|t| {
            header(t)
                .0
                .strip_prefix("if ")
                .is_some_and(|cond| formed_path(cond.trim()))
        })
    })
}

#[test]
fn rule_6b_kd5_takes_no_folded_ledger_value() {
    let mut hits = Vec::new();
    for source in all_sources().filter(|s| KD5_FILES.contains(&s.name)) {
        for item in items(&code_of(source.text)) {
            if folded_value_reads(&item.body) > 0 {
                hits.push(format!("{}: {}", source.name, item.name));
            }
        }
    }
    assert_eq!(
        all_sources().filter(|s| KD5_FILES.contains(&s.name)).count(),
        KD5_FILES.len()
    );
    assert!(
        hits.is_empty(),
        "K-D5 reads a folded ledger value:\n{}",
        hits.join("\n")
    );
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
    // K-D5: closure writers count, a forwarded ledger parameter counts, and a
    // local ledger handed to producers does not.
    let code = code_of(
        "fn g(ledger: &mut LoadLedger) { attach(ledger, ledger.term_count() - 1, s); }\nfn h(ledger: &mut LoadLedger) { ledger.set_closure_manifest(0, e); ledger.attach_load_primitive(p); }\nfn k() { let mut ledger = LoadLedger::new(); producer(&mut ledger, 1); ledger.finish(3) }",
    );
    let found = items(&code);
    assert_eq!(ledger_pushes(&found[0]), 1);
    assert_eq!(ledger_pushes(&found[1]), 2);
    assert_eq!(ledger_pushes(&found[2]), 0);
    // Rule 6: a load-primitive signature is a site; rule 6b sees folded reads
    // but not a length.
    let code = code_of(
        "fn m(p: &LoadPrimitive) -> u8 { 0 }\nfn n(ledger: &AssembledForce) { let _ = ledger.values(); }\nfn o(system: &S) { let n = system.force.len(); }",
    );
    let found = items(&code);
    assert!(force_site(&found[0]));
    assert_eq!(folded_value_reads(&found[1].body), 1);
    assert_eq!(folded_value_reads(&found[2].body), 0);
    // Rule 6: a load-addend parameter is a site, and so is every method of an
    // impl of a signature type (a `self` receiver names no type).
    let code = code_of(
        "fn p(op: &[Ball; 3], load: Ball) -> u8 { 0 }\nimpl Default for RowLoad { fn default() -> Self { z() } }\nimpl RowLoad { fn add(&mut self) {} }\nimpl Other { fn q(&self) {} }",
    );
    let sites = force_sites(&code);
    assert_eq!(
        sites,
        ["add", "default", "p"].into_iter().map(String::from).collect()
    );
    // Rule 6b (N1): per-DOF ledger reads and sums and a term's binary64
    // operands are folded reads; `.kinds` is not `.kind`.
    let code = code_of(
        "fn a(l: &AssembledForce, s: &mut A) { l.accumulate_dof(0, s, false); }\nfn b(l: &AssembledForce) { for t in l.terms_for(0) {} }\nfn c(t: &ForceTerm) { match t.kind { _ => {} } }\nfn d(x: &X) { x.kinds(); }",
    );
    let found = items(&code);
    let reads: Vec<_> = found.iter().map(|i| folded_value_reads(&i.body)).collect();
    assert_eq!(reads, [1, 1, 1, 0]);
    // Rule 6b (N1): a ledger-term accumulation counts unless an enclosing
    // block is the input branch of a plain `formed` guard; the product's
    // one-argument accumulation is not a ledger-term accumulation.
    let code = code_of(
        "fn e(t: &ForceTerm, r: &R) { if !r.records()[0].formed { if ok { t.accumulate(&mut s, false)?; } } }\n\
         fn f(t: &ForceTerm, r: &R) { if r.formed { g(); } else { t.accumulate(&mut s, false)?; } }\n\
         fn g(t: &ForceTerm, r: &R) { if r.formed { t.accumulate(&mut s, false)?; } else { h(); } }\n\
         fn h(t: &ForceTerm, r: &R) { if !r.formed && x { t.accumulate(&mut s, false)?; } }\n\
         fn i(t: &ForceTerm, r: &R) { if r.formed { g(); } else if y { t.accumulate(&mut s, false)?; } }\n\
         fn j(t: &ForceTerm) { t.accumulate(&mut s, false)?; ForceTerm::accumulate(t, &mut s, true)?; }\n\
         fn k(a: F, b: F) { let c = a.accumulate(b.mul(x)?)?; }",
    );
    let found = items(&code);
    let reads: Vec<_> = found.iter().map(|i| folded_value_reads(&i.body)).collect();
    assert_eq!(reads, [0, 0, 1, 1, 1, 2, 0]);
    // Rule 8 counts RV1-N1's self-assignment folds.
    let counts = scan(
        "fn f(a: &mut [f64], t: &[f64]) { a[0] = a[0] + t[0]; let b = checked_value(b - t[1])?; }",
    );
    assert_eq!(counts.get("f"), Some(&2));
}

// ------------------------------------------------------------------ S11-G

/// T8 (S11-G section 3.2): every case-force producer is classified. Input
/// producers push plain terms and no formation record; formed producers push
/// only `push_formed`/`push_formed_product` terms, with the named formation
/// families and the self-equilibrated flag of their family. A formed site
/// using plain `push` fails (M2). (function, class, formation tokens that
/// must occur, self-equilibrated flag of every formed push.)
const FORMATION_SITES: &[(&str, &str, &[&str], Option<&[&str]>)] = &[
    (
        "push_nodal_loads",
        "input: nodal loads carry no formation defect",
        &[],
        None,
    ),
    (
        "add_constant_effort_support_loads",
        "input: constant effort carries no formation defect",
        &[],
        None,
    ),
    (
        "add_uniform_element_loads",
        "formed: straight Exact (SP formula), curved Exact with T4-U1b's certificate (its operand bound), CannotBound where the certificate is refused; not self-equilibrated",
        &[
            "equivalent_global_nodal_loads_with_spans_formed(",
            "certify_curved_uniform_load(",
            "Formation::Exact",
            "Formation::CannotBound",
        ],
        Some(&["false"]),
    ),
    (
        "add_thermal_equivalent_loads",
        "formed: straight thermal and eigen RoundedProduct fl(N*x_a), self-equilibrated pair",
        &["Formation::RoundedProduct"],
        Some(&["true"]),
    ),
    (
        "add_curved_bend_thermal_equivalent_load",
        "formed: K_rc * fl(eps*chord_c), scaled RoundedProduct, self-equilibrated",
        &["Formation::RoundedProduct"],
        Some(&["true"]),
    ),
    (
        "add_curved_bend_pressure_equivalent_load",
        "formed: T4-U2 K_rc * fl(eps_p*chord_c), scaled RoundedProduct with eps_p's formation bound as operand bound, self-equilibrated",
        &["Formation::RoundedProduct", "bend_pressure::strain_operand_bound("],
        Some(&["true"]),
    ),
    (
        "push_exact_pressure_operands",
        "formed: exact-pressure group operand, Bounded by its producer-owned gamma_20 bound or the nonzero joint product's absolute underflow reserve",
        &["Formation::Bounded", "exact.assembled_operand_bounds[group_index]"],
        Some(&["false"]),
    ),
    (
        "add_connector_reference_loads",
        "formed: T4-U3 connector +B^T K q_ref, Bounded from the held operands (S13), self-equilibrated",
        &["Formation::Bounded", "connector_reference_load_bound("],
        Some(&["true"]),
    ),
    (
        "add_tie_rod_reference_loads",
        "formed: T4-U5b tie rod +C^T k e_ref, Bounded by K-D5's own enclosure of the authored operands (TieRod::reference_load), self-equilibrated",
        &["Formation::Bounded", "load.bounds[index]"],
        Some(&["true"]),
    ),
];

/// The argument text of each call of `needle` in `body` (balanced parens).
fn call_arguments(body: &str, needle: &str) -> Vec<String> {
    calls(body, needle)
        .map(|at| {
            let start = at + needle.len();
            let mut depth = 1;
            let mut end = start;
            for (offset, c) in body[start..].char_indices() {
                match c {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth -= 1,
                    _ => {}
                }
                if depth == 0 {
                    end = start + offset;
                    break;
                }
            }
            body[start..end].to_string()
        })
        .collect()
}

fn last_argument(arguments: &str) -> String {
    // rustfmt may leave a trailing comma after the last argument.
    let arguments = arguments.trim_end().trim_end_matches(',');
    let mut depth = 0;
    let mut last = 0;
    for (i, c) in arguments.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => last = i + 1,
            _ => {}
        }
    }
    arguments[last..]
        .trim()
        .trim_end_matches(',')
        .trim()
        .to_string()
}

#[test]
fn t8_every_case_force_producer_is_classified() {
    let pp = code_of(PRODUCT[0].text);
    let pp_items = items(&pp);
    for (name, class, tokens, flags) in FORMATION_SITES {
        assert!(!class.is_empty());
        let item = pp_items
            .iter()
            .find(|i| i.name == *name)
            .unwrap_or_else(|| panic!("{name}"));
        let plain = calls(&item.body, "ledger.push(").count()
            + calls(&item.body, "ledger.push_product(").count();
        let formed: Vec<String> = call_arguments(&item.body, "ledger.push_formed(")
            .into_iter()
            .chain(call_arguments(&item.body, "ledger.push_formed_product("))
            .collect();
        match flags {
            None => {
                assert!(plain > 0, "{name}: an input producer pushes plain terms");
                assert!(
                    formed.is_empty(),
                    "{name}: an input producer attaches no formation"
                );
            }
            Some(flags) => {
                assert_eq!(plain, 0, "{name}: a formed site uses plain push (M2)");
                assert!(!formed.is_empty(), "{name}: no formed push");
                for token in *tokens {
                    assert!(item.body.contains(token), "{name}: {token}");
                }
                let seen: BTreeSet<String> = formed.iter().map(|a| last_argument(a)).collect();
                let allowed: BTreeSet<String> = flags.iter().map(|f| f.to_string()).collect();
                assert_eq!(seen, allowed, "{name}: self-equilibrated flags");
            }
        }
    }
    // K-D5 (DEL-04-01 kd5-member-relative-bound.md §1): the live nodal
    // producer is mixed. An unscaled authored load is a plain input term; a
    // load-source-factored load is a formed RoundedProduct term (not
    // self-equilibrated) carrying its NodalProduct summand, and only that
    // branch attaches closure metadata.
    let certified = code_of(PRODUCT[11].text);
    assert_eq!(PRODUCT[11].name, "PP/certified_loads.rs");
    let nodal = items(&certified)
        .into_iter()
        .find(|i| i.name == "nodal")
        .expect("certified_loads::nodal");
    assert_eq!(calls(&nodal.body, "ledger.push(").count(), 1);
    assert_eq!(calls(&nodal.body, "ledger.push_product(").count(), 0);
    let formed = call_arguments(&nodal.body, "ledger.push_formed(");
    assert_eq!(formed.len(), 1);
    assert!(formed[0].contains("Formation::RoundedProduct"), "{}", formed[0]);
    assert_eq!(last_argument(&formed[0]), "false");
    assert!(nodal.body.contains("LoadFormula::NodalProduct"));
    let factored = nodal.body.find("source_factor(").expect("factor branch");
    let plain = nodal.body.find("ledger.push(").unwrap();
    let else_at = nodal.body[factored..].find("} else {").map(|k| k + factored).unwrap();
    assert!(nodal.body.find("ledger.push_formed(").unwrap() < else_at && else_at < plain);
    assert!(nodal.body[..else_at].contains("attach_last("));
    assert!(!nodal.body[else_at..].contains("attach"));
    // W-16 R-c (DEL-04-03 pf-nonlinear-imposed-motion.md §11.1): one formed
    // RoundedProduct term (not self-equilibrated) with its NodalProduct
    // summand per moved spring, after the zero test; no input push.
    let spring = items(&certified)
        .into_iter()
        .find(|i| i.name == "spring_base_motion")
        .expect("certified_loads::spring_base_motion");
    assert_eq!(calls(&spring.body, "ledger.push(").count(), 0);
    assert_eq!(calls(&spring.body, "ledger.push_product(").count(), 0);
    let formed = call_arguments(&spring.body, "ledger.push_formed(");
    assert_eq!(formed.len(), 1);
    assert!(formed[0].contains("Formation::RoundedProduct { k: 1.0, a: k, b: g }"), "{}", formed[0]);
    assert_eq!(last_argument(&formed[0]), "false");
    assert!(spring.body.contains("LoadFormula::NodalProduct { global_dof: dof, magnitude: k, factor: g }"));
    let zero = spring.body.find("if g == 0.0 {").expect("the zero test");
    assert!(zero < spring.body.find("ledger.push_formed(").unwrap());
    assert!(spring.body.contains("attach_last("));
    // The live ledger takes its nodal loads from it; `push_nodal_loads` (an
    // input producer) remains only for the retained-source replay.
    let ledger_body = &pp_items
        .iter()
        .find(|i| i.name == "case_force_ledger")
        .unwrap()
        .body;
    assert!(ledger_body.contains("certified_loads::nodal("));
    assert!(!ledger_body.contains("push_nodal_loads("));
    // U5a moved pressure formation-bound selection into the source-group
    // producer. The ledger must pair each operand with that same group's
    // bound and closure, never with a pre-summed per-DOF force. Source-known
    // zero retains the old zero bound; only a nonzero authored joint product
    // whose binary64 result underflows to zero receives one Tiny reserve.
    // Behavioural backing: joint_pressure_tests' source-underflow and
    // closure controls, and formation_check_tests' exact three-factor control.
    let pressure = pp_items.iter().find(|i| i.name == "push_exact_pressure_operands").unwrap();
    assert!(pressure.body.contains("exact.assembled_operands.iter().enumerate()"));
    assert!(pressure.body.contains("exact.closure_sources[group_index]"));
    assert!(!pressure.body.contains("assembled_loads"));
    let pressure_runtime = PRODUCT.iter().find(|s| s.name == "PP/pressure_runtime.rs").unwrap();
    let source_items = items(&code_of(pressure_runtime.text));
    let source = &source_items.iter().find(|i| i.name == "finish_source_groups").unwrap().body;
    assert_eq!(calls(source, "output.assembled_operands.push(").count(), 1);
    assert_eq!(calls(source, "output.assembled_operand_bounds.push(").count(), 1);
    assert!(source.contains("output.assembled_operands.push((dof, value, source))"));
    assert!(source.contains("output.assembled_operand_bounds.push(bound)"));
    assert!(source.contains("pressure_group_value(group.pressure, coefficient, group.direction)"));
    assert_eq!(calls(source, "super::exact_pressure_operand_bound(").count(), 1);
    let compact = source.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    assert!(compact.contains("letbound=ifmatches!(area,FactorArea::Joint(_))&&value==0.0&&group.pressure.pressure_pa()!=0.0&&coefficient!=0.0&&group.direction!=0.0{f64::from_bits(1)}else{super::exact_pressure_operand_bound(value)}"));
    let bound = &pp_items.iter().find(|i| i.name == "exact_pressure_operand_bound").unwrap().body;
    assert!(bound.contains("product_upward(gamma(20), value.abs())"));
    assert!(bound.contains("value != 0.0 && value.abs() < f64::MIN_POSITIVE"));
    assert!(bound.contains("(relative + f64::from_bits(1)).next_up()"));
    assert!(bound.contains("} else {\n        relative\n"));
    // Formed pushes occur nowhere else in the product or the kernel seams.
    let mut formed_sites = BTreeSet::new();
    for source in PRODUCT.iter().chain(KERNEL) {
        for item in items(&code_of(source.text)) {
            if calls(&item.body, ".push_formed(").next().is_some()
                || calls(&item.body, ".push_formed_product(").next().is_some()
            {
                formed_sites.insert((source.name.to_string(), item.name));
            }
        }
    }
    let expected: BTreeSet<_> = FORMATION_SITES
        .iter()
        .filter(|s| s.3.is_some())
        .map(|s| ("PP/lib.rs".to_string(), s.0.to_string()))
        .chain([("PP/certified_loads.rs".to_string(), "nodal".to_string())])
        // W-16 R-c: the spring-base producer, a formed product per moved spring.
        .chain([("PP/certified_loads.rs".to_string(), "spring_base_motion".to_string())])
        // Nonlinear K-D5 (kd5-nonlinear-active-set.md §1.6): the confirmation's
        // appended sliding-friction terms, outside the case force.
        .chain([("nonlinear_integration/confirmation.rs".to_string(), "appended_ledger".to_string())])
        .collect();
    assert_eq!(formed_sites, expected, "formed push sites");
}

/// T10b (V1 DN-5; S11G_GUARD revision 2.2 with erratum E-1): the routing
/// site in the private prepared body of `solve_load_case` binds
/// `source_eligible` by one call to main's
/// predicate (no load-row finding in it: M19), binds `needs_source_recovery`
/// by the tested predicate with the finding (M20), forms the receipt's
/// ordinary attempt with the finding (M21), declines a Passed guard-fired case
/// without an attempt (E-1, M23) and applies the G-3 decline before the
/// 0.4.0 replay reservation (M22).
#[test]
fn t10b_routing_site_calls_the_tested_predicates() {
    let pp = code_of(PRODUCT[0].text);
    let pp_items = items(&pp);
    let body = |name: &str| {
        pp_items
            .iter()
            .find(|i| i.name == name)
            .unwrap_or_else(|| panic!("{name}"))
            .body
            .clone()
    };
    // The document entry still prepares pressure once and enters this
    // private body once; Rule 1 pins that call and its order. All routing
    // predicates, declines and receipt formation remain in this body.
    let wrapper = body("solve_load_case_observed");
    assert_eq!(calls(&wrapper, "solve_prepared_load_case(").count(), 1);
    assert_eq!(calls(&wrapper, "source_eligible(").count(), 0);
    let case = body("solve_prepared_load_case");
    let statement = |binding: &str| {
        let at = case.find(binding).unwrap_or_else(|| panic!("{binding}"));
        let rest = &case[at + binding.len()..];
        rest[..rest.find(';').unwrap()].trim().to_string()
    };
    // G-1: main's predicate, one call, three arguments, no finding.
    assert_eq!(calls(&case, "source_eligible(").count(), 1);
    let eligible = statement("let source_eligible =");
    assert!(
        eligible.starts_with("source_eligible(") && eligible.ends_with(')'),
        "{eligible}"
    );
    assert!(!eligible.contains("load_row_finding"), "{eligible}");
    assert!(!body("source_eligible").contains("load_row_finding"));
    // G-2: the tested routing predicate carries the finding.
    let needs = statement("let needs_source_recovery =");
    assert!(
        needs.starts_with("needs_source_recovery(") && needs.contains("load_row_finding"),
        "{needs}"
    );
    assert!(body("needs_source_recovery").contains("load_row_finding.is_some()"));
    assert!(case.contains("if source_eligible && needs_source_recovery"));
    // G-2: the ordinary attempt records the finding.
    let passed = call_arguments(&case, "OrdinaryAttempt::passed(");
    assert_eq!(passed.len(), 1);
    assert!(
        passed[0].contains("load_row_finding.is_some()"),
        "{}",
        passed[0]
    );
    // E-1: a case the ordinary route would not attempt is declined without an
    // attempt, before any retained-source solve.
    let zero = case
        .find("formation_decline_without_attempt()")
        .expect("E-1 decline");
    // F1b (ROOT OQ3): the attempt enters through `solve_ordinary`, with the
    // ordinary evidence's scale `ForceScale::UNSCALED`, and no other non-test
    // product code calls `source_recovery::solve(` directly.
    let solve = case
        .find("source_recovery::solve_ordinary(")
        .expect("the attempt");
    let ordinary = call_arguments(&case, "source_recovery::solve_ordinary(");
    assert_eq!(ordinary.len(), 1);
    assert_eq!(last_argument(&ordinary[0]), "ForceScale::UNSCALED");
    for source in PRODUCT.iter().filter(|s| s.name != "PP/source_recovery.rs") {
        assert_eq!(
            calls(&code_of(source.text), "source_recovery::solve(").count(),
            0,
            "{}",
            source.name
        );
    }
    assert!(zero < solve);
    assert!(case[..zero].contains("needs_source_recovery(report_sensitive, attempt_err, None)"));
    // G-3: the decline precedes the 0.4.0 replay reservation.
    let decline = case.find("decline_for_formation(").expect("G-3 decline");
    let reserve = case
        .find("reserve_captured_replay(")
        .expect("replay reservation");
    assert!(solve < decline && decline < reserve);
    assert!(body("decline_for_formation").contains("decline_formation()"));
}
