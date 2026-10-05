#!/usr/bin/env python3
"""RV88 (U6b review): Python carrier mutants in a scratch copy of the candidate.
Re-runs every 4th of I66's 60 (read from I66's committed mutants.py) and RV88's
own. Oracles: I66's tests (its TESTS list), and RV88's rv88_u6b_checks.py, which
kills when a check fails or its recorded facts differ from the candidate's.
Usage: rv88_py_mutants.py <I66 mutants.py> <out.json>"""
import json, os, py_compile, subprocess, sys

WT = "WT"
S = f"{WT}/scratch/rv88_u6/u6b"
CAND = f"{WT}/rv88/b_cand/projects/chirality-piping"
MUT = f"{WT}/rv88/b_mut/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
ENV = dict(os.environ, PYTHONDONTWRITEBYTECODE="1", TMPDIR=f"{WT}/scratch/rv88_u6/u6d/tmp",
           OPENPIPESTRESS_CHECKED_JSON_BIN=f"{WT}/targets/rv88/canonical_json/release/openpipestress_jcs_ijson",
           OPENPIPESTRESS_UNITS_BIN=f"{WT}/targets/rv88/units/release/openpipestress_units")
ns = {}
exec(open(sys.argv[1]).read().split("def main(")[0].replace('WT = "WT"', f'WT = "{WT}"'), ns)
I66, TESTS, CO, RE = [m for k, m in enumerate(ns["M"]) if k % 4 == 0], ns["TESTS"], ns["CO"], ns["RE"]
OWN = [
    ("Q01_member_guard_null_slips", CO, '    if isinstance(source, Mapping) and "retained_precision" in source:\n        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)',
     '    if isinstance(source, Mapping) and source.get("retained_precision") is not None:\n        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)'),
    ("Q02_token_guard_first_row_only", CO, 'row.get("recovery_method") == RETAINED_METHOD for row in source["results"]):', 'row.get("recovery_method") == RETAINED_METHOD for row in source["results"][:1]):'),
    ("Q03_ar_equality_by_sha_only", CO, '        if run.get("retained_precision") != source["retained_precision"]:',
     '        if (run.get("retained_precision") or {}).get("receipt_sha256") != source["retained_precision"]["receipt_sha256"]:'),
    ("Q04_refused_statement_binds", CO, "    except (ValueError, KeyError, TypeError, AttributeError):\n        return RULE_QUANTITY_NOT_COVERED", "    except (ValueError, KeyError, TypeError, AttributeError):\n        return None"),
    ("Q05_wrapper_null_slips", RE, 'if isinstance(mechanics_result, Mapping) and "retained_precision" in mechanics_result:', 'if isinstance(mechanics_result, Mapping) and mechanics_result.get("retained_precision"):'),
    ("Q06_refs_order_insensitive", CO, "list(requested_basis_refs) != expected", "(len(requested_basis_refs) != len(expected) or any(ref not in expected for ref in requested_basis_refs))"),
]


def rv88_checks(tag):
    out = f"{S}/mut_out/{tag}_facts.tsv"
    r = subprocess.run([f"{VENV}/bin/python", f"{S}/rv88_u6b_checks.py", MUT, out], cwd=MUT, env=ENV, capture_output=True, text=True)
    facts = open(out).read() if os.path.exists(out) else "MISSING"
    return r.stdout, facts


def main():
    os.makedirs(f"{S}/mut_out", exist_ok=True)
    expected = open(f"{S}/u6b_facts.tsv").read()
    ctl_out, ctl_facts = rv88_checks("control")
    assert "497 of 497" in ctl_out and ctl_facts == expected, ctl_out[-300:]
    res = []
    for mid, name, old, new, *more in I66 + OWN:
        if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
            sys.exit("MEMGUARD NOT RUNNING")
        raw = open(f"{CAND}/{name}").read()
        pairs = [(old, new), *more]
        if any(raw.count(o) != 1 for o, _ in pairs):
            res.append({"id": mid, "status": "NOT_APPLIED"}); continue
        mutated = raw
        for o, n in pairs:
            mutated = mutated.replace(o, n)
        open(f"{MUT}/{name}", "w").write(mutated)
        try:
            py_compile.compile(f"{MUT}/{name}", cfile=f"{S}/mut_out/compile.pyc", doraise=True)
            r = subprocess.run(f"perl -e 'alarm shift; exec @ARGV' 1200 {VENV}/bin/python -m pytest -q -p no:cacheprovider {TESTS}", shell=True, cwd=MUT, env=ENV, capture_output=True, text=True)
            failed = [line.split(" ")[1] for line in r.stdout.splitlines() if line.startswith("FAILED ")]
            out, facts = rv88_checks(mid)
        except py_compile.PyCompileError as e:
            failed, out, facts, r = None, "", "", None
        finally:
            open(f"{MUT}/{name}", "w").write(raw)
        mine = [line for line in out.splitlines() if line.startswith("FAIL")]
        row = {"id": mid, "file": name,
               "i66_suite": "SYNTAX_ERROR" if failed is None else ("KILLED" if failed else "SURVIVED"),
               "rv88": "KILLED" if (mine or (facts and facts != expected)) else "SURVIVED",
               "rv88_by": (mine[:2] or (["facts differ"] if facts != expected else [])), "i66_failed": (failed or [])[:3]}
        res.append(row)
        print(mid, row["i66_suite"], row["rv88"], row["rv88_by"][:1], flush=True)
        json.dump(res, open(sys.argv[2], "w"), indent=1)
    for name in (CO, RE):
        assert open(f"{CAND}/{name}").read() == open(f"{MUT}/{name}").read(), name


if __name__ == "__main__":
    main()
