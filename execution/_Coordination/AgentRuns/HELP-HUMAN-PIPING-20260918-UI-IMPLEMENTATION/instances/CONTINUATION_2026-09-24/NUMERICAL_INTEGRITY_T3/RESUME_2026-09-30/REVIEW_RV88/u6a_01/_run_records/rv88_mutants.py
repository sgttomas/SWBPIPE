#!/usr/bin/env python3
"""RV88 (U6a review) mutant runner. Re-runs I66's 62 mutants (their list, read
from their committed mutants.py) and RV88's own, each in a scratch copy of the
candidate archive. Kill attribution is per suite: I66's three test targets,
and RV88's independent tests (zz_rv88_refusals, plus zz_rv88_slice with the
Python comparator for RV88's own mutants). A compile error never counts."""
import json, os, re, shutil, subprocess, sys, time

WT = "WT"
S = f"{WT}/scratch/rv88_u6"
CAND = f"{WT}/rv88/cand/projects/chirality-piping"
MUT = f"{WT}/rv88/mut/projects/chirality-piping"
TGT = f"{WT}/targets/rv88/mut"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
I66_MUTANTS = sys.argv[1]
SC = "core/reporting/result_export/src/semantic_contract.rs"
DV = "core/reporting/result_export/src/derivative.rs"
PY = "core/analysis_runs/retained_precision.py"
text = open(I66_MUTANTS).read()
ns = {}
exec(text.split("def run_rust")[0].replace('WT = "WT"', f'WT = "{WT}"'), ns)
I66 = ns["M"]
assert len(I66) == 62, len(I66)

OWN = [
 # R01: the row guard reads only the first row (a token on any later row slips through).
 ("R01_row_guard_first_row_only", SC,
  "            rows.iter()\n                .any(|row| row[\"recovery_method\"] == crate::retained_precision::METHOD)",
  "            rows.first().into_iter()\n                .any(|row| row[\"recovery_method\"] == crate::retained_precision::METHOD)"),
 # R02: validate_document's downgrade check ignores a null member on a base document.
 ("R02_doc_downgrade_ignores_null", DV,
  "    } else if doc[\"result_envelope\"].get(\"retained_precision\").is_some() {",
  "    } else if doc[\"result_envelope\"].get(\"retained_precision\").is_some_and(|v| !v.is_null()) {"),
 # R03: receipt equality by receipt_sha256 only (a body edit keeping the digest passes).
 ("R03_doc_receipt_sha_only", DV,
  "        if doc[\"result_envelope\"][\"retained_precision\"] != source[\"retained_precision\"] {",
  "        if doc[\"result_envelope\"][\"retained_precision\"][\"receipt_sha256\"] != source[\"retained_precision\"][\"receipt_sha256\"] {"),
 # R04: the disclosed decimal bound is rounded to 4 significant digits (not the receipt's bits).
 ("R04_message_lossy_decimal", DV,
  "verified only to the receipt's absolute bound b = {:e} (binary64",
  "verified only to the receipt's absolute bound b = {:.3e} (binary64"),
 # R05: requested refs compared as a set, not in the receipt's case order.
 ("R05_refs_order_insensitive", SC,
  "        || expected.iter().zip(requested_basis_refs).any(|(a, b)| *a != b)\n",
  "        || expected.iter().any(|a| !requested_basis_refs.contains(*a))\n"),
 # R06: the member guard exempts legacy 0.1.0 sources (no producer).
 ("R06_member_guard_skips_legacy", SC,
  "    if !is_retained(source) && source.get(\"retained_precision\").is_some() {",
  "    if !is_retained(source) && source[\"schema_version\"] != \"0.1.0\" && source.get(\"retained_precision\").is_some() {"),
 # R08: the class disclosure message drops the binary64 bits (only the decimal remains).
 ("R08_message_drops_bits", DV,
  " (binary64 {bound_bits:016x}) in the SI unit",
  " in the SI unit"),
]


def cargo(tests):
    args = " ".join(f"--test {t}" for t in tests)
    cmd = (f"cd {MUT}/core/reporting/result_export && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 "
           f"RV88_REFUSALS_OUT={S}/mut_refusals.tsv RV88_LANE=cand RV88_FIXTURES={MUT}/fixtures/results RV88_SLICE_OUT={S}/mut_slice "
           f"perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --target-dir {TGT} {args}")
    return subprocess.run(cmd, shell=True, capture_output=True, text=True)


def pytest():
    cmd = (f"cd {MUT} && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/rv88/canonical_json/release/openpipestress_jcs_ijson "
           f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/rv88/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1200 "
           f"{VENV}/bin/python -m pytest -q -p no:cacheprovider tests/test_retained_precision_contract.py -k 'public_entry or not_qualification'")
    return subprocess.run(cmd, shell=True, capture_output=True, text=True)


I66_TESTS = ["retained_precision_carriers", "preview_physics_contract", "derivative_contract"]
I66_NAMES = None


def main(which):
    results = []
    todo = (I66 if "i66" in which else []) + (OWN if "own" in which else [])
    for mid, rel, old, new in todo:
        if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
            print("MEMGUARD NOT RUNNING"); sys.exit(9)
        src = open(f"{CAND}/{rel}").read()
        assert src.count(old) == 1, (mid, src.count(old))
        open(f"{MUT}/{rel}", "w").write(src.replace(old, new))
        t = time.time()
        own = mid.startswith("R")
        try:
            if rel == PY:
                r = pytest(); out = r.stdout + r.stderr
                compile_error = "SyntaxError" in out or "IndentationError" in out
                failed = sorted(set(re.findall(r"^FAILED (\S+)", out, re.M)))
                i66_failed, rv88_failed = failed, []
                rc = r.returncode
            else:
                tests = I66_TESTS + ["zz_rv88_refusals"] + (["zz_rv88_slice"] if own else [])
                shutil.rmtree(f"{S}/mut_slice", ignore_errors=True)
                r = cargo(tests); out = r.stdout + r.stderr
                compile_error = "error[E" in out or "could not compile" in out
                # attribute failures by test binary
                # stdout/stderr are captured separately, so attribute by test name:
                # RV88's tests are all named rv88_*, I66's never are.
                i66_failed, rv88_failed = [], []
                for name in re.findall(r"^test (\S+) \.\.\. FAILED", out, re.M):
                    (rv88_failed if name.startswith("rv88_") else i66_failed).append(name)
                if own and not compile_error:
                    for f in os.listdir(f"{S}/slice"):
                        if f.startswith("base_") and f.endswith("_projection.json"):
                            shutil.copyfile(f"{S}/slice/{f}", f"{S}/mut_slice/{f}")
                    cmp = subprocess.run(["python3", f"{S}/rv88_slice_compare.py", f"{S}/mut_slice", f"{MUT}/fixtures/results"], capture_output=True, text=True)
                    n = re.search(r"FAILURES: (\d+)", cmp.stdout)
                    if cmp.returncode != 0 or not n or int(n.group(1)) > 0:
                        rv88_failed.append(f"rv88_slice_compare({n.group(1) if n else 'error'})")
                rc = r.returncode
        finally:
            shutil.copyfile(f"{CAND}/{rel}", f"{MUT}/{rel}")
        status_i66 = "COMPILE_ERROR" if compile_error else ("KILLED" if i66_failed else ("SURVIVED" if rc == 0 or rv88_failed else "ERROR"))
        status_rv88 = "COMPILE_ERROR" if compile_error else ("KILLED" if rv88_failed else ("n/a" if rel == PY else "SURVIVED"))
        results.append({"id": mid, "file": rel, "i66_suite": status_i66, "rv88_suite": status_rv88,
                        "i66_failed": sorted(set(i66_failed))[:6], "rv88_failed": sorted(set(rv88_failed))[:6], "seconds": round(time.time() - t)})
        print(f"{mid}\tI66:{status_i66}\tRV88:{status_rv88}\t{sorted(set(i66_failed))[:2]}\t{sorted(set(rv88_failed))[:2]}", flush=True)
        with open(f"{S}/mutants_{'_'.join(which)}.json", "w") as fh:
            json.dump(results, fh, indent=1)


if __name__ == "__main__":
    main(sys.argv[2:])
