#!/usr/bin/env python3
"""RV88 (U6c review): schema mutants in a scratch copy of the candidate. Re-runs
every 2nd of I66's 26 (read from I66's committed mutants.py) and RV88's own.
Oracles: I66's tests/test_retained_precision_schema.py, and RV88's three scripts
(document sweep, successor shapes, Y-probes), whose outputs must equal the
unmutated candidate's; any difference is a kill.
Usage: rv88_schema_mutants.py <I66 mutants.py> <out.json>"""
import json, os, re, subprocess, sys, time

WT = "WT"
S = f"{WT}/scratch/rv88_u6/u6c"
CAND = f"{WT}/rv88/c_cand/projects/chirality-piping"
MUT = f"{WT}/rv88/c_mut/projects/chirality-piping"
SLICE = f"{WT}/scratch/rv88_u6/u6a_done/slice"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
ENV = dict(os.environ, PYTHONDONTWRITEBYTECODE="1", TMPDIR=f"{WT}/scratch/rv88_u6/u6d/tmp",
           OPENPIPESTRESS_CHECKED_JSON_BIN=f"{WT}/targets/rv88/canonical_json/release/openpipestress_jcs_ijson",
           OPENPIPESTRESS_UNITS_BIN=f"{WT}/targets/rv88/units/release/openpipestress_units")
ns = {}
exec(open(sys.argv[1]).read().split("def main(")[0].replace('WT = "WT"', f'WT = "{WT}"'), ns)
I66 = [m for k, m in enumerate(ns["M"]) if k % 2 == 0]
RES, AR, SN = ns["RES"], ns["AR"], ns["SN"]
res_branch, ar_branch, sn_branch, drop = ns["res_branch"], ns["ar_branch"], ns["sn_branch"], ns["drop"]
SUCC, PREVIEW = ns["SUCC"], ns["PREVIEW"]
LR = "openpipestress.result_semantics/0.3.0/load-reference-1"
LRS = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
OWN = [
    ("W01_results_successor_receipt_not_required", RES, lambda d: res_branch(d, SUCC)["required"].remove("retained_precision")),
    ("W02_results_preview_admits_receipt", RES, lambda d: res_branch(d, PREVIEW)["properties"].__setitem__("retained_precision", res_branch(d, SUCC)["properties"]["retained_precision"])),
    ("W03_results_load_reference_admits_codes", RES, lambda d: res_branch(d, LR)["allOf"].pop()),
    ("W04_analysis_run_lrs_admits_receipt", AR, lambda d: drop(ar_branch(d, LRS)["not"]["anyOf"], {"required": ["retained_precision"]})),
    ("W05_stress_neutral_lr_admits_receipt", SN, lambda d: drop(sn_branch(d, LR)["not"]["anyOf"], {"required": ["retained_precision"]})),
    ("W06_stress_neutral_successor_annotations_optional", SN, lambda d: sn_branch(d, SUCC)["required"].remove("source_annotations")),
]
SCRIPTS = [("docsweep", "rv88_schema_sweep.py"), ("shapes", "rv88_successor_shapes.py"), ("yprobes", "rv88_y_probes.py")]


def rv88(tag):
    outs = {}
    for name, script in SCRIPTS:
        out = f"{S}/mut_out/{tag}_{name}.tsv"
        subprocess.run([f"{VENV}/bin/python", f"{S}/{script}", MUT, SLICE, out], cwd=MUT, env=ENV, capture_output=True, text=True)
        outs[name] = open(out).read() if os.path.exists(out) else "MISSING"
    return outs


def main():
    os.makedirs(f"{S}/mut_out", exist_ok=True)
    control = rv88("control")
    expected = {name: open(f"{S}/{ {'docsweep': 'docsweep_c_cand.tsv', 'shapes': 'shapes_c_cand.tsv', 'yprobes': 'yprobes_c_cand.tsv'}[name] }").read() for name, _ in SCRIPTS}
    assert control == expected, "control differs from the candidate outputs"
    res = []
    for mid, name, mutate in I66 + OWN:
        if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
            sys.exit("MEMGUARD NOT RUNNING")
        raw = open(f"{CAND}/schemas/{name}", "rb").read()
        data = json.loads(raw)
        before = json.dumps(data, sort_keys=True)
        mutate(data)
        changed = json.dumps(data, sort_keys=True) != before
        open(f"{MUT}/schemas/{name}", "w").write(json.dumps(data, indent=2) + "\n")
        try:
            r = subprocess.run(f"perl -e 'alarm shift; exec @ARGV' 1200 {VENV}/bin/python -m pytest -q -p no:cacheprovider tests/test_retained_precision_schema.py",
                               shell=True, cwd=MUT, env=ENV, capture_output=True, text=True)
            failed = sorted(set(re.findall(r"^FAILED (\S+)", r.stdout, re.M)))
            outs = rv88(mid)
        finally:
            open(f"{MUT}/schemas/{name}", "wb").write(raw)
        mine = [k for k in outs if outs[k] != expected[k]]
        row = {"id": mid, "file": name, "changed": changed,
               "i66_suite": "NO_CHANGE" if not changed else ("KILLED" if failed else ("SURVIVED" if r.returncode == 0 else "ERROR")),
               "rv88": "KILLED" if mine else "SURVIVED", "rv88_by": mine, "i66_failed": [f.split("::")[-1] for f in failed][:4]}
        res.append(row)
        print(mid, row["i66_suite"], row["rv88"], mine, flush=True)
        json.dump(res, open(sys.argv[2], "w"), indent=1)


if __name__ == "__main__":
    main()
