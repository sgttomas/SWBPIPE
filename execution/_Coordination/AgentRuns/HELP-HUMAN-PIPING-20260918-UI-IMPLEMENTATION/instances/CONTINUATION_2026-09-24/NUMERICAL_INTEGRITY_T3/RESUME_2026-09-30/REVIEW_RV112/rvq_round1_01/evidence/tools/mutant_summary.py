#!/usr/bin/env python3
"""RV112: summarize the mutant runs (libtest output of PP's lib suite, one log per mutant).

Usage: mutant_summary.py <mutants dir> <mutants.tsv>   (prints JSON and a table)
A mutant is killed when a test other than the known Mac `t13` (failing in the pristine control
too) fails at an assertion (a panic with an assertion or explicit message), not at compilation.
"""
import json, re, sys

D, TSV = sys.argv[1], sys.argv[2]
KNOWN = {"s11g_tests::t13_committed_fallback_uz_is_byte_identical"}
TEST = re.compile(r"^test (\S+) \.\.\. (ok|FAILED|ignored)")
PANIC = re.compile(r"^thread '([^']+)' \(\d+\) panicked at ([^:]+:\d+:\d+):$")


def parse(path):
    outcomes, panics, lines = {}, {}, open(path, encoding="utf-8", errors="replace").read().splitlines()
    for i, line in enumerate(lines):
        m = TEST.match(line)
        if m:
            outcomes[m.group(1)] = m.group(2)
        m = PANIC.match(line)
        if m and m.group(1) not in panics:
            msg = []
            for nxt in lines[i + 1:i + 8]:
                if nxt.startswith("note:") or nxt.startswith("thread '") or nxt.startswith("test ") or nxt == "":
                    break
                msg.append(nxt.strip())
            panics[m.group(1)] = {"at": m.group(2), "message": " | ".join(msg)[:400]}
    result = re.findall(r"^test result: .*$", "\n".join(lines), re.M)
    return outcomes, panics, result


def main():
    pristine, _, presult = parse(f"{D}/pristine.log")
    pfail = sorted(t for t, o in pristine.items() if o == "FAILED")
    out = {"pristine": {"result": presult, "failed": pfail, "tests": len(pristine)}, "mutants": {}}
    rows = [l.rstrip("\n").split("\t") for l in open(TSV, encoding="utf-8") if l.strip()]
    rows.append(["H01", "RV112", "B-6: NOTICE_RESERVE_BYTES without ×C (const; its own build)"])
    for mid, kind, desc in rows:
        path = f"{D}/{mid}.log"
        import os
        if os.path.exists(path) and not os.path.exists(f"{D}/{mid}.rc"):
            out["mutants"][mid] = {"kind": kind, "desc": desc, "status": "RUNNING"}
            continue
        try:
            outcomes, panics, result = parse(path)
        except FileNotFoundError:
            out["mutants"][mid] = {"kind": kind, "desc": desc, "status": "NOT RUN"}
            continue
        failed = sorted(t for t, o in outcomes.items() if o == "FAILED" and t not in KNOWN)
        compiled = bool(outcomes)
        killers = {t: panics.get(t, {"at": "?", "message": "(no panic text found)"}) for t in failed}
        status = "KILLED" if failed and all(k["at"] != "?" for k in killers.values()) else ("NOT COMPILED" if not compiled else "SURVIVED" if not failed else "KILLED?")
        out["mutants"][mid] = {"kind": kind, "desc": desc, "status": status, "result": result, "killed_by": killers,
                               "tests_seen": len(outcomes)}
    print(json.dumps(out, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
