#!/usr/bin/env python3
"""Mutation runner for the reviewed material.rs tests, on a scratch crate copy.
Reviewed bytes are the source; each mutant replaces one snippet; the original
ten material tests are run. Reviewed files are never edited."""
import pathlib, re, subprocess, sys, os
SRC = pathlib.Path(sys.argv[1]); CRATE = pathlib.Path(sys.argv[2]); TARGET = sys.argv[3]
text = SRC.read_text()
M = [
 ("MM1_source_order_bracket_no_sort", "            table.sort_by(|a, b| a.0.total_cmp(&b.0));\n", ""),
 ("MM2_interpolation_skips_actual_T_check", "            check_temperature_basis(material, actual, Some(requested), analysis_basis_override)?;\n", ""),
 ("MM3_nu_not_interpolated", "                    linear(low.pair.poisson_ratio(), high.pair.poisson_ratio()),", "                    low.pair.poisson_ratio(),"),
 ("MM4_exact_point_ignores_actual_T", "            check_temperature_basis(\n                material,\n                actual,\n                selected.temperature_k,\n                analysis_basis_override,\n            )?;\n", ""),
]
env = dict(os.environ, CARGO_TARGET_DIR=TARGET)
dest = CRATE / "src/case_state/material.rs"
for name, old, new in M:
    if text.count(old) != 1:
        print(name, "SNIPPET COUNT", text.count(old)); continue
    dest.write_text(text.replace(old, new))
    r = subprocess.run(["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "1", "--lib", "case_state::material::tests"],
                       cwd=CRATE, capture_output=True, text=True, env=env)
    out = r.stdout + r.stderr
    m = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", out)
    failed = re.findall(r"^test (\S+) \.\.\. FAILED", out, re.M)
    if not m:
        print(name, "BUILD_FAILED", out[-800:]); continue
    print(f"{name}: {m.group(1)} passed, {m.group(2)} failed {failed} => {'KILLED' if m.group(2) != '0' else 'SURVIVED'}")
    sys.stdout.flush()
dest.write_text(text)
