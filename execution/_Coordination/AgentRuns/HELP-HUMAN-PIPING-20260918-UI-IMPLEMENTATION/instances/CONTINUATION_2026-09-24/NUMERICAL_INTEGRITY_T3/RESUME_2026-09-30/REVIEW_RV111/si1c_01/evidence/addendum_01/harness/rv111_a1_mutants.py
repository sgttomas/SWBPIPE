#!/usr/bin/env python3
"""RV111 addendum 01: mutants of the repaired head f5665f8862 (copy WT/rv111/mut2, target WT/targets/rv111-mut2).
SF-1's N06 and N09 re-applied with their round-0 patch strings, and ruling 2's note form (replace, prepend,
separator, existing note only). Killing tests: the runner crate's whole `cargo test --offline --locked
--no-fail-fast`, through WT/tools/t3_cargo.sh. Results in a1/mutants.json; logs in a1/mutants/."""
import json
import os
import re
import shutil
import subprocess

WT = "WT"
S = f"{WT}/scratch/rv111_si1c_01"
P = "projects/chirality-piping"
HEAD = f"{WT}/rv111/head2/{P}"
MUT = f"{WT}/rv111/mut2/{P}"
RCR = "core/rules/rule_check_runner/src/lib.rs"
ENV = dict(os.environ, TMPDIR=f"{S}/tmp", CARGO_TARGET_DIR=f"{WT}/targets/rv111-mut2")
APPEND = '                (true, Some(existing)) => Some(format!("{existing}; {NON_FINITE_INPUT_NOTE}")),'
MUTANTS = [
    ("N06_dedup_removed", "                if !non_finite_inputs.contains(&id) {", "                if true {"),
    ("N09_trim_dropped", "if !v.is_finite() && u.trim() != unit_ref.trim()", "if !v.is_finite() && u != unit_ref"),
    ("A1_note_replaces_existing", APPEND,
     '                (true, Some(_existing)) => Some(NON_FINITE_INPUT_NOTE.to_string()),'),
    ("A1_note_prepended", APPEND,
     '                (true, Some(existing)) => Some(format!("{NON_FINITE_INPUT_NOTE}; {existing}")),'),
    ("A1_note_separator_space", APPEND,
     '                (true, Some(existing)) => Some(format!("{existing} {NON_FINITE_INPUT_NOTE}")),'),
    ("A1_existing_note_only", APPEND, '                (true, Some(existing)) => Some(existing),'),
]


def main():
    os.makedirs(f"{S}/a1/mutants", exist_ok=True)
    results = {}
    for mid, old, new in MUTANTS:
        shutil.copyfile(f"{HEAD}/{RCR}", f"{MUT}/{RCR}")
        src = open(f"{MUT}/{RCR}", encoding="utf-8").read()
        n = src.count(old)
        if n != 1:
            results[mid] = {"error": f"anchor occurs {n} times"}
            continue
        open(f"{MUT}/{RCR}", "w", encoding="utf-8").write(src.replace(old, new))
        log = f"{S}/a1/mutants/{mid}.log"
        with open(log, "w") as fh:
            rc = subprocess.call([f"{WT}/tools/t3_cargo.sh", "test", "--offline", "--locked", "--no-fail-fast"],
                                 cwd=f"{MUT}/core/rules/rule_check_runner", env=ENV, stdout=fh, stderr=subprocess.STDOUT)
        text = open(log, errors="replace").read()
        failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", text, re.M)))
        # Each failing test's panic: an assertion (assert_eq!/assert!) or something else.
        panics = re.findall(r"^thread '([^']+)' panicked at [^\n]*\n([^\n]*)", text, re.M)
        kinds = {name: ("assertion" if msg.startswith("assertion") else msg[:80]) for name, msg in panics}
        results[mid] = {"rc": rc, "compile_error": "error[E" in text, "failed": failed, "panic_kinds": kinds,
                        "killed": bool(failed)}
        json.dump(results, open(f"{S}/a1/mutants.json", "w"), indent=1, sort_keys=True)
        print(mid, json.dumps(results[mid])[:400], flush=True)
    shutil.copyfile(f"{HEAD}/{RCR}", f"{MUT}/{RCR}")


if __name__ == "__main__":
    main()
