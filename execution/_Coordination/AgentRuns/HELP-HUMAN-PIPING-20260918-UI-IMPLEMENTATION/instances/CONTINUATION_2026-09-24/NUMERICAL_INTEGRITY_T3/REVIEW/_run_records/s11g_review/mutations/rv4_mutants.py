#!/usr/bin/env python3
"""RV4 mutation sample for S11-G (standard library).

Reuses I5's driver machinery (s11g_mutants.py, copied unchanged from the
candidate's records) for exact textual patches, byte restore and sha256
verification. Adds RV4's own mutants (RV-*) and re-runs a sample of I5's,
with MR2 also re-run against T6 alone (Addendum 1).

Usage, from the scratch worktree's projects/chirality-piping directory, with
the cargo environment exported:
    python3 rv4_mutants.py <out_dir> [MUTANT ...]
    python3 rv4_mutants.py --check
"""
import sys
import s11g_mutants as i5

FK, SP, PP, FG = i5.FK, i5.SP, i5.PP, i5.FG
SREC = 'core/product_physics/src/source_recovery.rs'
PPT, SPT, SITE, PPM = i5.PPT, i5.SPT, i5.SITE, i5.PPM
FKT = ['cargo', 'test', '--offline', '--locked', '--manifest-path',
       'core/solver/frame_kernel/Cargo.toml', '--lib']

# Every RV mutant runs the whole S11-G surface, so any S11-G test may kill it.
SURFACE = [PPT + ['--', 's11g_tests::'], SITE, FKT + ['--', 's11g'], SPT + ['--', 's11g_tests::']]
# For a survivor: the whole product_physics lib suite and its integration tests.
PP_ALL = [['cargo', 'test', '--offline', '--locked', '--no-fail-fast'] + PPM]

RV = {
    'RV-M1': ([(FG, """    if copy.add_product(sign * 12.0, bound).is_err()""",
                """    if copy.add_product(sign * 12.0, 0.0 * bound).is_err()""")],
              SURFACE, "D21-1: the second test ignores B (|A_net| > 12 T0 instead of |A_net| + 12B > 12 T0)"),
    'RV-M2': ([(PP, """    let relative = product_upward(gamma(20), value.abs());""",
                """    let relative = 0.0 * product_upward(gamma(20), value.abs());""")],
              SURFACE, "exact-pressure operand bound gamma_20 |t| replaced by 0 (family hidden)"),
    'RV-M3': ([(PP, """                        b: bend.chord[local_col - DOF_PER_NODE],""",
                """                        b: bend.chord[(local_col - DOF_PER_NODE + 1) % 3],""")],
              SURFACE, "curved thermal RoundedProduct operand taken from the wrong chord axis"),
    'RV-M4': ([(PP, """            ledger.push_formed(&load.source, i_base + axis, -value, product(-1.0), 0.0, true);
            ledger.push_formed(&load.source, j_base + axis, value, product(1.0), 0.0, true);""",
                """            ledger.push_formed(&load.source, i_base + axis, -value, product(-1.0), 0.0, false);
            ledger.push_formed(&load.source, j_base + axis, value, product(1.0), 0.0, false);""")],
              SURFACE, "straight thermal/eigen pairs tagged not self-equilibrated (floor bypassed)"),
    'RV-M5': ([(PP, """corrected_local_forces[offset + RY].hypot(corrected_local_forces[offset + RZ]),""",
                """corrected_local_forces[offset + RY].hypot(0.0 * corrected_local_forces[offset + RZ]),""")],
              SURFACE, "R-b' q from My only (Mz dropped)"),
    'RV-M6': ([(PP, """        let attempt = if !crate::needs_source_recovery(report_sensitive, attempt_err, None) {""",
                """        let attempt = if !crate::needs_source_recovery(report_sensitive, false, None) {""")],
              SURFACE, "E-1: an ordinary-Err guard-fired case gets the zero-work decline instead of main's real attempt"),
    'RV-M7': ([(PP, """            Err(source_recovery::formation_decline_without_attempt())""",
                """            { source_budget.attempts += 1; Err(source_recovery::formation_decline_without_attempt()) }""")],
              SURFACE, "D22-1: the zero-work decline counts an attempt (ledger differs from main)"),
    'RV-M8': ([(SREC, """        work: exact::WorkReport {
            charged: 0,
            rejected: 0,
            limit: 0,
        },""", """        work: exact::WorkReport {
            charged: 0,
            rejected: 0,
            limit: 4_000_000,
        },""")],
              SURFACE + [['cargo', 'test', '--offline', '--locked', '--manifest-path',
                          'core/runner/headless/Cargo.toml', '--test', 's11g_zero_work_receipt']],
              "D22-1: the zero-work decline carries a nonzero limit"),
    'RV-M9': ([(FK, """            let underflow = *a != 0.0 && *b != 0.0 && p.abs() < FMA_EXACT_PRODUCT_MIN;""",
                """            let underflow = false && *a != 0.0 && *b != 0.0 && p.abs() < FMA_EXACT_PRODUCT_MIN;""")],
              SURFACE, "DN-2: the FMA underflow fallback removed (lo treated as exact)"),
    'RV-M10': ([(FG, """            let (force, moment) = if restrained.contains(&row.dof) {
                (m[2], m[3])
            } else {
                (m[0], m[1])
            };""", """            let (force, moment) = (m[0], m[1]);""")],
               SURFACE, "restrained rows take the free-row S* (section 3.4)"),
    'MR2-T6': ([(FG, """    } else if (row.bound > 0.0 && row.bound >= t0)""", """    } else if (row.bound >= t0)""")],
               i5.pp('t6_collinear_skew_pair_cancels_signed_defects'),
               "ruling 2: restore B >= T0 alone; T6 alone must kill it"),
}

SAMPLE = ['M1', 'M2', 'M5', 'M10', 'M12', 'M13', 'M14a', 'M14b', 'M17', 'M18', 'M23', 'MR2']
i5.MUTANTS = {**{name: i5.MUTANTS[name] for name in SAMPLE}, **RV}

if __name__ == '__main__':
    if len(sys.argv) > 1 and sys.argv[1] == '--survivor':
        # Re-run named survivors against the whole product_physics suite.
        out = sys.argv[2]
        i5.MUTANTS = {name + '-ALL': (i5.MUTANTS[name][0], PP_ALL, i5.MUTANTS[name][2] + ' (whole PP suite)')
                      for name in sys.argv[3:]}
        sys.argv = [sys.argv[0], out] + list(i5.MUTANTS)
    i5.main()
