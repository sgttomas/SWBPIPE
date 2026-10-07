#!/usr/bin/env python3
"""RV109: summarize the mutant runs. For each mutant: did it compile; which tests failed (t13, the
known Mac failure, excluded); and each failing test's panic message (the first failing assertion).

Usage: mutant_summary.py <mutants dir> <mutants.py list output>
"""
import os
import re
import sys

KNOWN = {"s11g_tests::t13_committed_fallback_uz_is_byte_identical"}


def summarize(path):
    text = open(path, encoding="utf-8", errors="replace").read()
    compiled = "Running unittests" in text
    failed = re.findall(r"^test (\S+) \.\.\. FAILED$", text, re.M)
    # Tests whose name line was interleaved with output still appear in the failures list.
    m = re.search(r"^failures:\n((?:    \S+\n)+)", text, re.M)
    if m:
        failed = sorted(set(failed) | {x.strip() for x in m.group(1).splitlines()})
    failed = [t for t in failed if t not in KNOWN]
    panics = {}
    for pm in re.finditer(r"^thread '([^']+)' \(\d+\) panicked at ([^\n]+):\n(.*?)(?=\n(?:note:|thread '|\n|---- |failures:))", text, re.M | re.S):
        name, at, msg = pm.group(1), pm.group(2), pm.group(3)
        if name in KNOWN:
            continue
        msg = msg.strip().splitlines()
        first = msg[0] if msg else ""
        extra = next((l for l in msg[1:] if l.strip() and not l.lstrip().startswith(("left:", "right:"))), "")
        panics.setdefault(name, f"{at}: {first[:300]}" + (f" | {extra[:200]}" if extra else ""))
    result = re.findall(r"^test result: .*$", text, re.M)
    return compiled, failed, panics, result


def main():
    d = sys.argv[1]
    why = {}
    for line in open(sys.argv[2], encoding="utf-8"):
        k, _, v = line.rstrip("\n").partition("\t")
        why[k] = v
    out = ["# RV109 mutant results", "", "| Mutant | What it changes | Compiled | Killed? | Failing tests (t13 excluded) and first failing assertion |", "|---|---|---|---|---|"]
    for mid, desc in why.items():
        log = os.path.join(d, f"{mid}.log")
        if not os.path.exists(log):
            out.append(f"| {mid} | {desc} | not run | – | – |")
            continue
        compiled, failed, panics, result = summarize(log)
        killed = "yes" if failed else ("BY COMPILATION" if not compiled else "**SURVIVED**")
        cells = "<br>".join(f"`{t}`: {panics.get(t, '(no panic text captured)').replace('|', '/')}" for t in failed) or "none"
        out.append(f"| {mid} | {desc} | {'yes' if compiled else 'no'} | {killed} | {cells} |")
    pristine = os.path.join(d, "pristine.log")
    if os.path.exists(pristine):
        compiled, failed, panics, result = summarize(pristine)
        out += ["", f"Pristine control (same target, same flags): compiled={compiled}; failures other than t13: {failed or 'none'}; {result}"]
    print("\n".join(out))


if __name__ == "__main__":
    main()
