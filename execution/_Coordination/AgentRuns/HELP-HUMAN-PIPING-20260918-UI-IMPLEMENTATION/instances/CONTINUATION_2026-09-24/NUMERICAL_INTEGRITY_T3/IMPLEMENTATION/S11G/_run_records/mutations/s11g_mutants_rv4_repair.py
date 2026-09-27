#!/usr/bin/env python3
"""T3 S11-G (I5) mutation driver (standard library).

Usage, from the worktree's projects/chirality-piping directory, with the cargo
environment already exported (RUSTUP_TOOLCHAIN, RUSTUP_AUTO_INSTALL=0,
CARGO_INCREMENTAL=0, CARGO_TARGET_DIR=<s11g-target>):
    python3 s11g_mutants.py <out_dir> [MUTANT ...]
    python3 s11g_mutants.py --check        (anchor check only; no cargo)

Each mutant is one or more exact textual replacements (each anchor must occur
exactly once). The driver saves the file bytes, applies the patch, runs the
named killing tests with `cargo test --offline --locked`, records the exit
status and log, and restores the saved bytes, verifying the sha256. No Git
operation is used. A mutant is KILLED when its killing command fails, and
COMPILE_ERROR (not counted as a kill) when the mutated tree does not compile.
"""
import hashlib
import json
import os
import subprocess
import sys

FK = 'core/solver/frame_kernel/src/load_ledger.rs'
SP = 'core/solver/straight_pipe/src/lib.rs'
PP = 'core/product_physics/src/lib.rs'
FG = 'core/product_physics/src/formation_guard.rs'

PPM = ['--manifest-path', 'core/product_physics/Cargo.toml']
PPT = ['cargo', 'test', '--offline', '--locked'] + PPM + ['--lib']
SPT = ['cargo', 'test', '--offline', '--locked', '--manifest-path', 'core/solver/straight_pipe/Cargo.toml', '--lib']
SITE = ['cargo', 'test', '--offline', '--locked'] + PPM + ['--test', 's11f_site_test']


def pp(*names):
    return [PPT + ['--', '--exact'] + ['s11g_tests::' + n for n in names]]


MUTANTS = {
    # The unmutated tree: every killing command passes (so a kill is the mutant's).
    'BASELINE': ([], [PPT + ['--', 's11g_tests::'], SPT + ['--', 's11g_tests::'], SITE], 'no mutation'),
    'M1': ([(FK, """            target
                .add_product(12.0, value)
                .map_err(|_| "12-scaled value is out of range")?;""", """            target
                .add_product(0.0, value)
                .map_err(|_| "12-scaled value is out of range")?;"""),
            (FK, """                target
                    .add_product(-factor, component)""", """                target
                    .add_product(0.0 * factor, component)""")],
           pp('t1_udl_w1e8_is_demoted_on_both_entries_and_modes', 't2_udl_w1e80_is_demoted_on_the_typed_entry'),
           'drop a formed term defect (Exact defect = 0)'),
    'M2': ([(PP, """                    ledger.push_formed(
                        &load.load_id,
                        *global,
                        equivalent[slot],
                        formation,
                        operand_bound,
                        false,
                    );""", """                    let _ = (formation, operand_bound);
                    ledger.push(&load.load_id, *global, equivalent[slot]);""")],
           pp('t1_udl_w1e8_is_demoted_on_both_entries_and_modes', 't2_udl_w1e80_is_demoted_on_the_typed_entry')
           + [SITE + ['--', '--exact', 't8_every_case_force_producer_is_classified']],
           'tag the straight uniform formed term as input (plain push)'),
    'M3': ([(FK, """            // An input term: its intended value is its value.
            if add_twelve_value(term, &mut row.twelve_intended_net).is_err() {""", """            // An input term: its intended value is its value.
            let _ = bounds.add(UNIT_ROUNDOFF * match term.kind {
                ForceTermKind::Term(v) => v.abs(),
                ForceTermKind::Product(a, b) => (a * b).abs(),
            });
            if add_twelve_value(term, &mut row.twelve_intended_net).is_err() {""")],
           pp('t4b_exact_formed_terms_cancelled_by_inputs_stay_silent'),
           'give input terms a bound of u|t| (T4 alone: equivalent, no formed row)'),
    'M4': ([(SP, """        let formations = match exact_scaled_intended(&orientation, length, uniform_loads) {""",
             """        let formations = match None::<Vec<Vec<f64>>>.or_else(|| { let _ = exact_scaled_intended; None }) {""")],
           pp('t3_udl_w1e5_stays_checks_passed', 't5_probe_a_same_expression_defects_cancel'),
           "replace SP's exact defect with the a-priori gamma_16 sum|monomials| bound"),
    'M5': ([(FG, """            let (force, moment) = if restrained.contains(&row.dof) {
                (m[2], m[3])
            } else {
                (m[0], m[1])
            };""", """            let (force, moment) = (m[2], m[3]);""")],
           pp('t1_udl_w1e8_is_demoted_on_both_entries_and_modes', 't2_udl_w1e80_is_demoted_on_the_typed_entry'),
           'take the free-row S* over all rows'),
    'M6': ([(FK, """        let mut cannot = false;
        if let Err(reason) = add_defect(
            term,
            record,
            target,
            &mut row.twelve_intended_net,
            &mut bounds,
            &mut cannot,
        ) {
            failure = failure.or(Some(reason));
        }""", """        let mut cannot = false;
        let mut single = ExactAccumulator::new();
        if let Err(reason) = add_defect(
            term,
            record,
            &mut single,
            &mut row.twelve_intended_net,
            &mut bounds,
            &mut cannot,
        ) {
            failure = failure.or(Some(reason));
        }
        let _ = target.add(single.round().unwrap_or(0.0).abs());""")],
           pp('t5_probe_a_same_expression_defects_cancel'),
           'sum |eps| instead of the signed sum'),
    'M8': ([(PP, """    let ordinary_attempt = match &attempted_linear {""", """    let mut attempted_linear = attempted_linear;
    if load_row_finding.is_some() {
        if let Ok(solve) = attempted_linear.as_mut() {
            solve.structural_report.quality = SolveQuality::Sensitive;
        }
    }
    let ordinary_attempt = match &attempted_linear {""")],
           pp('t1_udl_w1e8_is_demoted_on_both_entries_and_modes'),
           'demote through report.quality before routing'),
    # Revision 2.2 routing (G-1, G-2, G-3) and erratum E-1 (ROOT's D22-1 condition).
    # M7 (drop revision 2.1's source_eligible gate) is withdrawn: revision 2.2 G-1 removed the gate.
    'M19': ([(PP, """    if source_eligible && needs_source_recovery {""",
              """    if source_eligible && needs_source_recovery && load_row_finding.is_none() {""")],
            pp('t18_path2_invocation_is_not_refused', 't19_path1_load_row_variant_is_not_refused'),
            "reintroduce revision 2.1's load-row routing gate"),
    'M20': ([(PP, """    report_sensitive || attempt_err || load_row_finding.is_some()""",
              """    report_sensitive || attempt_err || (load_row_finding.is_some() && false)""")],
            pp('t10_routing_predicates', 't19_path1_load_row_variant_is_not_refused'),
            'the routing predicate ignores the load-row finding (G-2)'),
    'M21': ([(PP, """            integrity_diagnostic_id(&load_case.id),
            load_row_finding.is_some(),
        ),""", """            integrity_diagnostic_id(&load_case.id),
            false,
        ),""")],
            pp('t19_path1_load_row_variant_is_not_refused')
            + [SITE + ['--', '--exact', 't10b_routing_site_calls_the_tested_predicates']],
            'the ordinary attempt ignores the finding: passed(..., false) (G-2)'),
    'M22': ([(PP, """    if load_row_finding.is_some() {
        Err(recovery.decline_formation())
    } else {
        Ok(recovery)
    }""", """    let _ = load_row_finding;
    Ok(recovery)""")],
            pp('t21_selection_is_declined_for_a_formation_finding'),
            'a guard-fired selection is not declined (G-3)'),
    'M23': ([(PP, """        let attempt = if !crate::needs_source_recovery(report_sensitive, attempt_err, None) {""",
              """        let attempt = if false && !crate::needs_source_recovery(report_sensitive, attempt_err, None) {""")],
            pp('t22_invocation_budget_equals_main_on_a_passed_guard_fired_case'),
            'always run the charged attempt on a guard-fired case (E-1 / D22-1)'),
    'M9': ([(FG, """    diagnostic.severity = "warning".to_string();""", """    diagnostic.severity = "blocking".to_string();""")],
           pp('t1_udl_w1e8_is_demoted_on_both_entries_and_modes'),
           'map a fired case to a blocking diagnostic'),
    'M10': ([(FK, """        let target = if record.self_equilibrated {""", """        let binary64_family = !record.self_equilibrated
            && matches!(record.formation, Formation::RoundedProduct { .. });
        let target = if binary64_family {
            if let Formation::RoundedProduct { k, a, b } = record.formation {
                let p = a * b;
                let _ = rp_unscaled.add_product(-k, a.mul_add(b, -p));
                let _ = add_twelve_value(term, &mut row.twelve_intended_net);
                let _ = row.twelve_intended_net.add_product(4.0 * k, a.mul_add(b, -p));
                let _ = row.twelve_intended_net.add_product(4.0 * k, a.mul_add(b, -p));
                let _ = row.twelve_intended_net.add_product(4.0 * k, a.mul_add(b, -p));
            }
            continue;
        } else if record.self_equilibrated {"""),
             (FK, """    let mut bounds = ExactAccumulator::new();
    let mut magnitude = ExactAccumulator::new();
    let mut failure = None;
    for &index in indices {""", """    let mut bounds = ExactAccumulator::new();
    let mut magnitude = ExactAccumulator::new();
    let mut failure = None;
    let mut rp_unscaled = ExactAccumulator::new();
    for &index in indices {"""),
             (FK, """    row.bound = round_upward(&bounds).unwrap_or(f64::INFINITY);""", """    let combined = row.net_defect.round().unwrap_or(0.0) / 12.0 + rp_unscaled.round().unwrap_or(0.0);
    row.net_defect = ExactAccumulator::new();
    let _ = row.net_defect.add_product(12.0, combined);
    row.bound = round_upward(&bounds).unwrap_or(f64::INFINITY);""")],
            pp('m10_families_are_combined_exactly'),
            "combine the Exact and RoundedProduct families in binary64 (revision 1)"),
    'M11': ([(FG, """    let floor = product_downward(row.self_equilibrated_magnitude, FLOOR_FACTOR);""",
              """    let floor = 0.0 * product_downward(row.self_equilibrated_magnitude, FLOOR_FACTOR);""")],
            pp('t6a_collinear_runs_are_silent_with_the_floor'), 'drop the floor'),
    'M12': ([(FK, """        let target = if record.self_equilibrated {
            let added = match term.kind {
                ForceTermKind::Term(value) => magnitude.add(value.abs()),
                ForceTermKind::Product(a, b) => magnitude.add_product(a.abs(), b.abs()),
            };
            if added.is_err() {
                failure = failure.or(Some("self-equilibrated magnitude is out of range"));
            }
            &mut row.self_equilibrated_defect""", """        let _ = match term.kind {
            ForceTermKind::Term(value) => magnitude.add(value.abs()),
            ForceTermKind::Product(a, b) => magnitude.add_product(a.abs(), b.abs()),
        };
        let target = if record.self_equilibrated {
            &mut row.self_equilibrated_defect""")],
            pp('t1_udl_w1e8_is_demoted_on_both_entries_and_modes', 't2_udl_w1e80_is_demoted_on_the_typed_entry'),
            'take the floor from all formed terms'),
    'M13': ([(FG, """    let above_floor = s_star_moment >= DESIGN_FLOOR_MIN_SCALE && q >= DESIGN_FLOOR * s_star_moment;""",
              """    let above_floor = true || (s_star_moment >= DESIGN_FLOOR_MIN_SCALE && q >= DESIGN_FLOOR * s_star_moment);""")],
            pp('t12_accurate_small_moment_rows_below_the_floor_stay_passed'), "drop R-b''s floor clause (R-b ships)"),
    'M14a': ([(SP, """                    .add_product(transform[k][c].abs(), displacement.abs())""", """                    .add_product(transform[k][c], *displacement)"""),
              (SP, """                    .add_product(stiffness[ry][k].abs(), *magnitude)""", """                    .add_product(stiffness[ry][k], *magnitude)"""),
              (SP, """                    .add_product(stiffness[rz][k].abs(), *magnitude)""", """                    .add_product(stiffness[rz][k], *magnitude)""")],
             pp('t11_inplane_rows_are_demoted_by_rb_prime_alone'), 'signed sums in B'),
    'M14b': ([(SP, """                    .add_product(transform[k][c].abs(), displacement.abs())""", """                    .add_product(transform[k][c], *displacement)"""),
              (SP, """            *magnitude = round_upward(&accumulator).map_err(|_| bound_error(f64::INFINITY))?;""",
               """            *magnitude = accumulator.round().map_err(|_| bound_error(f64::INFINITY))?.abs();""")],
             [SPT + ['--', '--exact', 's11g_tests::t17_bending_formation_bound_on_a_skew_member']], '|T u| in B'),
    'M15': ([(FG, """    if diagnostic.code != PASSED {
        return false;
    }""", """    let _ = PASSED;""")],
            pp('t13b_already_sensitive_case_is_left_untouched', 'no_op_rule_leaves_a_non_passed_record_untouched'),
            'demote an already-Sensitive case again and append text'),
    'M16': ([(FK, """            add_twelve_value(term, intended).map_err(|_| "12-scaled value is out of range")?;
            *cannot_bound = true;""", """            add_twelve_value(term, intended).map_err(|_| "12-scaled value is out of range")?;
            let _ = cannot_bound;""")],
            pp('t15_curved_uniform_load_is_cannot_bound'), 'treat CannotBound as a zero defect'),
    'M17': ([(FG, """    } else if (row.bound > 0.0 && row.bound >= t0)
        || exceeds(&row.net_defect, row.bound, t0)""", """    } else if (row.bound > 0.0 && row.bound >= tf)
        || exceeds(&row.net_defect, row.bound, tf)""")],
            pp('t6b_floor_never_hides_a_net_defect'), "apply the floor to the whole row (revision 2's rule)"),
    'M18': ([(FG, """pub(crate) const CRITERION: f64 = f64::from_bits(1e-9_f64.to_bits() - 1);""",
              """pub(crate) const CRITERION: f64 = 1e-9;""")],
            pp('m18_threshold_constant_is_rounded_down'), 'the binary64 literal 1e-9'),
    'MR2': ([(FG, """    } else if (row.bound > 0.0 && row.bound >= t0)""", """    } else if (row.bound >= t0)""")],
            pp('t6_collinear_skew_pair_cancels_signed_defects', 'ruling2_boundary_of_the_first_clause'),
            "ruling 2: restore revision 2.1's first clause B >= T0 alone"),
    'X1': ([(SP, """            *bound = product_upward(gamma(16), row_sum);""", """            *bound = row_sum;""")],
           pp('t11_inplane_rows_are_demoted_by_rb_prime_alone')
           + [SPT + ['--', '--exact', 's11g_tests::t17_bending_formation_bound_on_a_skew_member']], "drop B's gamma_16"),
    'X2': ([(SP, """            *bound = product_upward(gamma(16), row_sum);""", """            *bound = product_upward(gamma(1), row_sum);""")],
           [SPT + ['--', '--exact', 's11g_tests::t17_bending_formation_bound_on_a_skew_member']], 'gamma_1 instead of gamma_16 in B'),
    'X3': ([(SP, """                accumulator
                    .add_product(stiffness[rz][k].abs(), *magnitude)
                    .map_err(|_| bound_error(*magnitude))?;""", """                let _ = rz;""")],
           pp('t11_inplane_rows_are_demoted_by_rb_prime_alone'), "drop the RZ row from B"),
    'X4': ([(FG, """    let resolved = q > RESOLUTION_FACTOR * bound;""", """    let resolved = RESOLUTION_FACTOR > 0.0;""")],
           pp('t11_inplane_rows_are_demoted_by_rb_prime_alone', 't12_accurate_small_moment_rows_below_the_floor_stay_passed',
              't16_formation_rows_are_published_non_passed_on_both_entries', 'rb_prime_clauses_at_their_boundaries'),
           "drop R-b's resolution clause q > 2^10 B"),
    'X5': ([(FG, """            let mo = if extent > 0.0 {
                moment.max(extent * force)
            } else {
                moment
            };""", """            let mo = if extent > 0.0 && force < 0.0 { moment.max(extent * force) } else { moment };""")],
           pp('t12_accurate_small_moment_rows_below_the_floor_stay_passed', 't11_inplane_rows_are_demoted_by_rb_prime_alone'),
           'S*_moment without the L_b S(force) coupling'),
    'X6': ([(PP, """            formation_guard::amend_integrity_report(
                diagnostics,
                &integrity_diagnostic_id(&load_case.id),
                &finding,
            );""", """            let _ = finding;""")],
           pp('t11_inplane_rows_are_demoted_by_rb_prime_alone'), "drop the R-b' amendment"),
    'X7': ([(FG, """        first.add(bound).is_ok() && first.add_product(-CRITERION, q).is_ok() && first.signum() > 0;""",
             """        first.add(bound).is_ok() && first.add_product(-CRITERION * 1e3, q).is_ok() && first.signum() > 0;""")],
           pp('t11_inplane_rows_are_demoted_by_rb_prime_alone', 't12_accurate_small_moment_rows_below_the_floor_stay_passed',
              'rb_prime_clauses_at_their_boundaries'),
           "R-b's first clause at 1e-6 instead of 1e-9"),
    # RV4's review mutants (REVIEW/_run_records/s11g_review/mutations/patches), with the
    # repair's killing tests.
    'RV-M1': ([(FG, """    if copy.add_product(sign * 12.0, bound).is_err()""",
                """    if copy.add_product(sign * 12.0, 0.0 * bound).is_err()""")],
              pp('d21_1_second_test_adds_the_bound_exactly'), "D21-1: the second test ignores B"),
    'RV-M2': ([(PP, """    let relative = product_upward(gamma(20), value.abs());""",
                """    let relative = 0.0 * product_upward(gamma(20), value.abs());""")],
              pp('exact_pressure_operand_bound_is_gamma_20', 'exact_pressure_operand_rows_carry_their_bound'),
              'exact-pressure operand bound gamma_20 |t| replaced by 0'),
    'RV-M3': ([(PP, """                        b: bend.chord[local_col - DOF_PER_NODE],""",
                """                        b: bend.chord[(local_col - DOF_PER_NODE + 1) % 3],""")],
              pp('curved_thermal_records_name_the_pushed_products'),
              'curved thermal RoundedProduct operand taken from the wrong chord axis'),
    'RV-M6': ([(PP, """        let attempt = if !crate::needs_source_recovery(report_sensitive, attempt_err, None) {""",
                """        let attempt = if !crate::needs_source_recovery(report_sensitive, false, None) {""")],
              pp('e1_ordinary_err_guard_fired_case_keeps_mains_attempt'),
              "E-1: an ordinary-Err guard-fired case gets the zero-work decline"),
    'RV-M10': ([(FG, """            let (force, moment) = if restrained.contains(&row.dof) {
                (m[2], m[3])
            } else {
                (m[0], m[1])
            };""", """            let (force, moment) = (m[0], m[1]);""")],
               pp('restrained_rows_take_the_all_rows_scale'), 'restrained rows take the free-row S*'),
}


def sha(path):
    return hashlib.sha256(open(path, 'rb').read()).hexdigest()


def check():
    for name, (patches, _, _) in MUTANTS.items():
        for path, old, _ in patches:
            count = open(path).read().count(old)
            print(name, path, count, 'OK' if count == 1 else 'BAD')


def main():
    if sys.argv[1] == '--check':
        return check()
    out = sys.argv[1]
    names = sys.argv[2:] or list(MUTANTS)
    os.makedirs(out, exist_ok=True)
    results = {}
    results_path = os.path.join(out, 'results.json')
    if os.path.exists(results_path):
        results = json.load(open(results_path))
    for name in names:
        patches, commands, what = MUTANTS[name]
        saved = {}
        for path, _, _ in patches:
            if path not in saved:
                saved[path] = (open(path, 'rb').read(), sha(path))
        try:
            for path, old, new in patches:
                text = open(path).read()
                count = text.count(old)
                assert count == 1, f'{name}: anchor occurs {count} times in {path}'
                open(path, 'w').write(text.replace(old, new))
            log_path = os.path.join(out, f'{name}.log')
            exits = []
            with open(log_path, 'w') as log:
                for command in commands:
                    log.write('$ ' + ' '.join(command) + '\n')
                    log.flush()
                    exits.append(subprocess.run(command, stdout=log, stderr=subprocess.STDOUT).returncode)
            killed = any(code != 0 for code in exits)
            log_text = open(log_path).read()
            compile_error = 'could not compile' in log_text
            verdict = 'COMPILE_ERROR' if compile_error else ('KILLED' if killed else 'SURVIVED')
            if 'running 0 tests' in log_text:
                verdict += ' (a command ran 0 tests)'
            results[name] = {'what': what, 'commands': [' '.join(c) for c in commands], 'exits': exits,
                             'verdict': verdict}
        finally:
            for path, (data, digest) in saved.items():
                open(path, 'wb').write(data)
                assert sha(path) == digest, f'{path} not restored'
        json.dump(results, open(results_path, 'w'), indent=1, sort_keys=True)
        print(name, results[name]['verdict'], results[name]['exits'], flush=True)


if __name__ == '__main__':
    main()
