"""I73 S-I1 evaluator mutants: each patch must apply exactly once; each mutant is
killed when `cargo test --lib` fails. Scratch harness; records go to RETURN."""
import json, shutil, subprocess, sys, pathlib, re
WT = pathlib.Path(__import__("os").environ["WT"])  # the T3 host root (placeholder WT)
S = WT / "scratch/i73_s_i1"
SRC = WT / "s-i1/projects/chirality-piping/core/rules/expression_evaluator"
MUTANTS = {
  "M1_outward_step_removed": [(
    "    let (lo, hi) = (lo.next_down(), hi.next_up());\n    if lo.is_finite() && hi.is_finite() {",
    "    let (lo, hi) = (lo, hi);\n    if lo.is_finite() && hi.is_finite() {")],
  "M2_compare_U_arm_to_T": [(
    "            } else {\n                Truth::Indeterminate\n            }\n        }\n        ComparisonOperator::LessThan => {",
    "            } else {\n                Truth::True\n            }\n        }\n        ComparisonOperator::LessThan => {"),
    ("            } else {\n                Truth::Indeterminate\n            }\n        }\n        ComparisonOperator::GreaterThanOrEqual => {",
     "            } else {\n                Truth::True\n            }\n        }\n        ComparisonOperator::GreaterThanOrEqual => {"),
    ("            } else {\n                Truth::Indeterminate\n            }\n        }\n        ComparisonOperator::GreaterThan => {",
     "            } else {\n                Truth::True\n            }\n        }\n        ComparisonOperator::GreaterThan => {"),
    ("            } else {\n                Truth::Indeterminate\n            }\n        }\n        ComparisonOperator::Equal => interval_equal(a, b),",
     "            } else {\n                Truth::True\n            }\n        }\n        ComparisonOperator::Equal => interval_equal(a, b),"),
    ("    } else if a.hi < b.lo || b.hi < a.lo {\n        Truth::False\n    } else {\n        Truth::Indeterminate\n    }",
     "    } else if a.hi < b.lo || b.hi < a.lo {\n        Truth::False\n    } else {\n        Truth::True\n    }")],
  "M4a_multiply_inward": [(
    "    let products = [a.lo * b.lo, a.lo * b.hi, a.hi * b.lo, a.hi * b.hi];",
    "    let products = [a.lo * b.lo, a.hi * b.hi, a.lo * b.lo, a.hi * b.hi];")],
  "M4b_abs_straddle_inward": [(
    "        Enclosure {\n            lo: 0.0,\n            hi: max2(-e.lo, e.hi),\n        }",
    "        Enclosure {\n            lo: min2(-e.lo, e.hi),\n            hi: max2(-e.lo, e.hi),\n        }")],
  "M4c_interpolation_point_hull_d2_literal": [(
    "        let segment = interpolate_segment(low, high, clipped)?;",
    "        let at = |x: f64| if x == low.argument { low.result } else if x == high.argument { high.result } else { low.result + (high.result - low.result) * ((x - low.argument) / (high.argument - low.argument)) };\n        let (p, q) = (at(clipped.lo), at(clipped.hi));\n        let segment = outward(min2(p, q), max2(p, q))?;\n        let _ = interpolate_segment;")],
  "M5_indeterminate_poisoning_removed": [(
    "        value.map(|value| value.into_public(!notes.is_empty()))",
    "        value.map(|value| value.into_public(false))")],
  "M6_divide_zero_range_not_refused": [(
    "                    Some(b) if !interval_contains_zero(b) => match left_enclosure {",
    "                    Some(b) if b.lo != 0.0 && b.hi != 0.0 => match left_enclosure {")],
  "M7_kleene_and_true_with_unknown": [(
    "            (Truth::True, Truth::True) => Truth::True,\n            (Truth::True, Truth::Indeterminate)\n            | (Truth::Indeterminate, Truth::True)\n            | (Truth::Indeterminate, Truth::Indeterminate) => Truth::Indeterminate,",
    "            (Truth::True, Truth::True) | (Truth::True, Truth::Indeterminate) => Truth::True,\n            (Truth::Indeterminate, Truth::True)\n            | (Truth::Indeterminate, Truth::Indeterminate) => Truth::Indeterminate,")],
  "M8_select_unknown_takes_then_branch": [(
    "                            (Some(a), Some(b)) => Some(interval_hull(a, b)),",
    "                            (Some(a), Some(_b)) => Some(a),")],
  "M9_step_lookup_hull_only_first_row": [(
    "    for row in &rows[from + 1..=to] {\n        enclosure = interval_hull(enclosure, Enclosure::point(row.result));\n    }",
    "    for row in &rows[from + 1..from + 1] {\n        enclosure = interval_hull(enclosure, Enclosure::point(row.result));\n    }\n    let _ = to;")],
  "M10_table_range_check_dropped": [(
    "            Some(argument) if first <= argument.lo && argument.hi <= last => {\n                let enclosure = interpolate_enclosure(&table.rows, argument);",
    "            Some(argument) if first <= argument.hi && argument.lo <= last => {\n                let enclosure = interpolate_enclosure(&table.rows, argument);")],
}
only = sys.argv[1:] or list(MUTANTS)
results = {}
for name in only:
    d = S / "mut" / name
    if d.exists(): shutil.rmtree(d)
    shutil.copytree(SRC, d, ignore=shutil.ignore_patterns("target"))
    lib = d / "src/lib.rs"; text = lib.read_text()
    for old, new in MUTANTS[name]:
        n = text.count(old)
        assert n == 1, (name, n, old[:60])
        text = text.replace(old, new)
    lib.write_text(text)
    rc = subprocess.run([str(S / "cargo_run.sh"), f"mut_{name}", str(d), str(WT / "targets/i73-s-i1"),
                         "test", "--locked", "--offline", "--lib"]).returncode
    log = (S / "logs" / f"mut_{name}.log").read_text()
    failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", log, re.M)))
    compiled = "error[" not in log
    results[name] = {"rc": rc, "compiled": compiled, "killed": rc != 0 and compiled, "failed_tests": failed}
    print(name, results[name], flush=True)
out = S / "mut" / ("ee_results_" + "_".join(only) + ".json" if sys.argv[1:] else "ee_results.json")
out.write_text(json.dumps(results, indent=1))
