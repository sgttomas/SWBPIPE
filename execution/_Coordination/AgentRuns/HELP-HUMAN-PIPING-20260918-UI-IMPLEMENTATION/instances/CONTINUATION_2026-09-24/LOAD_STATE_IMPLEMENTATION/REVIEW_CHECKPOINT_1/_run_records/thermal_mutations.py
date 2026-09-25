#!/usr/bin/env python3
"""Mutation runner for the reviewed thermal.rs tests (scratch copies only).

Each mutant replaces exactly one kernel snippet (lines 1-689) in a scratch copy,
builds the reviewer probe crate with --features mutant, and runs the ORIGINAL
eight thermal tests, then the reviewer probes. Reviewed bytes are never edited.
"""
import pathlib, re, subprocess, sys, os
SRC = pathlib.Path(sys.argv[1])       # reviewed thermal.rs
CRATE = pathlib.Path(sys.argv[2])     # probe crate dir
TARGET = sys.argv[3]
text = SRC.read_text()
kernel_end = text.index("#[cfg(test)]\nmod tests")
M = [
 ("T1_integral_last_segment_only", "terms.extend(product(width, average)?.map(|x| sign * x));", "terms = product(width, average)?.map(|x| sign * x).to_vec();"),
 ("T2_dilation_last_segment_only", "terms.extend(product(delta, ratio)?.map(|v| sign * v));", "terms = product(delta, ratio)?.map(|v| sign * v).to_vec();"),
 ("T3_secant_no_install_ratio", '(datum, "engineering_secant", li, lo, divide(delta, li)?)', '(datum, "engineering_secant", li, lo, delta)'),
 ("T4_log_as_linear", 'let strain = published(delta.exp_m1(), "logarithmic thermal strain")?;', 'let strain = published(delta, "logarithmic thermal strain")?;'),
 ("T5_reversal_by_negation_integral", "(to, from, -1.0)\n    };\n    let mut terms = Vec::new();\n    let first = points\n        .partition_point(|p| p.temperature_kelvin <= lo)\n        .saturating_sub(1);\n    for (i, w) in points.windows(2).enumerate().skip(first) {\n        if w[0].temperature_kelvin >= hi {\n            break;\n        }\n        let a = lo.max(w[0].temperature_kelvin);\n        let b = hi.min(w[1].temperature_kelvin);\n        if a >= b {\n            continue;\n        }\n        let va", "(to, from, 1.0)\n    };\n    let mut terms = Vec::new();\n    let first = points\n        .partition_point(|p| p.temperature_kelvin <= lo)\n        .saturating_sub(1);\n    for (i, w) in points.windows(2).enumerate().skip(first) {\n        if w[0].temperature_kelvin >= hi {\n            break;\n        }\n        let a = lo.max(w[0].temperature_kelvin);\n        let b = hi.min(w[1].temperature_kelvin);\n        if a >= b {\n            continue;\n        }\n        let va"),
 ("T6_additive_fit_no_cross_term", "[fit, thermal.strain, cross[0], cross[1]],", "[fit, thermal.strain],"),
 ("T7_fit_sign_flip", "divide(*change_m, *reference_length_m)?", "divide(-*change_m, *reference_length_m)?"),
 ("T8_no_path_positivity", "    candidates.sort_by(f64::total_cmp);", "    candidates.clear();"),
 ("T10_secant_alpha_hot_times_interval", 'sum([po[0], po[1], -pi[0], -pi[1]], "secant dilation difference")?', 'sum(product(ao, sum([operate, -install], "x")?)?, "secant dilation difference")?'),
 ("T11_naive_rounded_stretch_difference", '            let delta = integral(points, install, operate, &mut used)?;\n            (\n                datum,\n                "differential_per_datum_length",', '            let delta = sum([lo, -li], "naive")?;\n            (\n                datum,\n                "differential_per_datum_length",'),
 ("T12_datum_integral_ignores_install_datum", '                "differential_per_datum_length",\n                li,\n                lo,\n                divide(delta, li)?,', '                "differential_per_datum_length",\n                li,\n                lo,\n                delta,'),
 ("T13_dilation_datum_zero_unchecked", "if sample(points, datum, &mut used)? != 0.0 {", "if false {"),
 ("T14_secant_datum_coverage_dropped", "                    covered(points, datum)?;\n                    check_path(points, datum, install, operate, PathKind::Secant, &mut used)?;", "                    check_path(points, datum.max(points[0].temperature_kelvin), install, operate, PathKind::Secant, &mut used)?;"),
]
env = dict(os.environ, CARGO_TARGET_DIR=TARGET)
def run(filter_):
    r = subprocess.run(["cargo", "+1.97.1", "test", "--offline", "-j", "1", "--features", "mutant", "--", filter_],
                       cwd=CRATE, capture_output=True, text=True, env=env)
    out = r.stdout + r.stderr
    m = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", out)
    failed = re.findall(r"^test (\S+) \.\.\. FAILED", out, re.M)
    if not m:
        return ("BUILD_FAILED", out[-1500:])
    return (f"{m.group(1)} passed, {m.group(2)} failed", failed)
for name, old, new in M:
    kernel = text[:kernel_end]
    n = kernel.count(old)
    if n != 1:
        print(name, "SNIPPET COUNT", n); continue
    (CRATE / "src/mutant_thermal.rs").write_text(kernel.replace(old, new) + text[kernel_end:])
    orig = run("thermal::tests")
    probes = run("probes::")
    verdict = "KILLED by original tests" if orig[0] != "BUILD_FAILED" and not orig[0].endswith(" 0 failed") else ("SURVIVED original tests" if orig[0] != "BUILD_FAILED" else "BUILD_FAILED")
    print(f"{name}: original thermal tests: {orig[0]} {orig[1] if isinstance(orig[1], list) else ''} | reviewer probes: {probes[0]} {probes[1] if isinstance(probes[1], list) else ''} => {verdict}")
    sys.stdout.flush()
