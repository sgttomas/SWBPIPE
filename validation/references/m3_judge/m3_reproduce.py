"""Reproducibility of the committed reference files (Agent 0: "regenerating each set from its committed generator
reproduces the pinned digest. Otherwise the pin only says 'unchanged', not 'reproducible'").

Usage: python3 -I -B m3_reproduce.py <new scratch dir> [--workers N]

For every line of REFERENCE_SHA256SUMS (read from HEAD's blob, as the judge reads it) there is exactly one recipe
below; each regenerates its file into the scratch directory from the committed generator and requires its sha256 to
equal the pin. All six are mandatory: the exact route's three from ../kd5_timoshenko/kd5t_generate.py and the PF
route's three from ../pf_timoshenko_m3t/pft_generate.py.
The PF selftest set is the 20 named lines of the regenerated PF frozen subset, in its order.
"""

from __future__ import annotations

import hashlib
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
import m3_view as MV  # noqa: E402  (only pinned_references: the manifest as the judge reads it)

KD5 = os.path.normpath(os.path.join(HERE, "..", "kd5_timoshenko"))
PF_SELFTEST_TAGS = (
    "pft|CANT03|S3M2|o|m0|eb", "pft|CANT03|S3M2|o|m0|ts", "pft|CANT07|S5M1|utmskew|m0|eb", "pft|CANT07|S5M1|utmskew|m0|ts",
    "pft|CANT09|S5M1|o|cm|eb", "pft|CANT09|S5M1|o|cm|ts", "pft|FF05|S4M2|o|m0|eb", "pft|FF05|S4M2|o|m0|ts", "pft|L13|S5M2|o|m0|eb",
    "pft|L13|S5M2|o|m0|ts", "pft|SPAN04|S5M1|o|m0|eb", "pft|SPAN04|S5M1|o|m0|ts", "pft|NCcodexS8|S8NC|o|m0|eb", "pft|NCcodexS8|S8NC|o|m0|ts",
    "pft|NCxe-17S8|S8NC|o|m0|eb", "pft|NCxe-17S8|S8NC|o|m0|ts", "pft|NCtwoxe-12S8|S8NC|o|m0|eb", "pft|NCtwoxe-12S8|S8NC|o|m0|ts",
    "pft|NCauxfree|S7NC|utmskew|m0|eb", "pft|NCauxfree|S7NC|utmskew|m0|ts")


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def run(argv, cwd):
    p = subprocess.run([sys.executable, "-I", "-B", *argv], cwd=cwd, capture_output=True, text=True)
    if p.returncode != 0:
        raise RuntimeError(f"{argv[0]} exited {p.returncode}: {(p.stderr or p.stdout)[-400:]}")


def frozen(gen_dir, module, out, workers):
    """The generator's frozen subset regenerated in memory (its frozen_bytes, as its `check` does), written to out."""
    run(["-c", f"import sys; sys.path.insert(0, {gen_dir!r}); import {module} as G; open({out!r}, 'wb').write(G.frozen_bytes({workers}))"], gen_dir)
    return out


def full(gen_dir, script, outdir, name, workers):
    if not os.path.exists(os.path.join(outdir, name)):
        run([os.path.join(gen_dir, script), "full", outdir, str(workers)], gen_dir)
    return os.path.join(outdir, name)


def selftest_subset(frozen_path, out):
    by_tag = {}
    for line in open(frozen_path, "rb").read().split(b"\n"):
        if line:
            tag = line.rsplit(b'"tag":"', 1)[1].split(b'"', 1)[0].decode() if b'"tag":"' in line else None   # tag is the last key (sort_keys)
            by_tag[tag] = line
    order = [t for t in by_tag if t in PF_SELFTEST_TAGS]
    if sorted(order) != sorted(PF_SELFTEST_TAGS):
        raise RuntimeError("the regenerated PF frozen subset lacks a selftest model")
    open(out, "wb").write(b"".join(by_tag[t] + b"\n" for t in order))
    return out


def main():
    args = sys.argv[1:]
    pf_dir, workers = os.path.normpath(os.path.join(HERE, "..", "pf_timoshenko_m3t")), 4
    if "--workers" in args:
        i = args.index("--workers")
        workers = int(args[i + 1])
        del args[i:i + 2]
    if len(args) != 1:
        raise SystemExit(__doc__)
    scratch = os.path.abspath(args[0])
    if os.path.exists(scratch) and os.listdir(scratch):
        raise SystemExit(f"{scratch} exists and is not empty")
    os.makedirs(scratch, exist_ok=True)
    pins, probs = MV.pinned_references()
    if probs:
        print("REFERENCE_SHA256SUMS: " + "; ".join(probs))
        print("REPRODUCE: FAIL")
        raise SystemExit(1)
    S = lambda *p: os.path.join(scratch, *p)
    # recipe name (the pin's text, from REFERENCE_SHA256SUMS) -> (route, kind, mandatory, producer)
    recipes = {
        "kd5_timoshenko: kd5t_generate.py full set.jsonl (964 models)":
            ("exact", "set", True, lambda: full(KD5, "kd5t_generate.py", S("kd5-full"), "set.jsonl", workers)),
        "kd5_timoshenko: kd5t_generate.py full controls.jsonl (E perturbed by 2^-20)":
            ("exact", "controls", True, lambda: full(KD5, "kd5t_generate.py", S("kd5-full"), "controls.jsonl", workers)),
        "kd5_timoshenko/frozen/kd5t_frozen_subset.jsonl (64 models)":
            ("exact", "set", True, lambda: frozen(KD5, "kd5t_generate", S("kd5t_frozen_subset.jsonl"), workers)),
        "pf_timoshenko_m3t: pft_generate.py full set.jsonl (906 models; moduli re-authored in GPa where the value entry needs it)":
            ("pf", "set", True, lambda: full(pf_dir, "pft_generate.py", S("pf-full"), "set.jsonl", workers)),
        "pf_timoshenko_m3t/frozen/pft_frozen_subset.jsonl (100 models)":
            ("pf", "set", True, lambda: frozen(pf_dir, "pft_generate", S("pft_frozen_subset.jsonl"), workers)),
        "m3_judge/selftest/pf_set.jsonl (20 models, verbatim lines of the PF frozen subset)":
            ("pf", "set", True, lambda: selftest_subset(S("pft_frozen_subset.jsonl") if os.path.exists(S("pft_frozen_subset.jsonl"))
                                                        else frozen(pf_dir, "pft_generate", S("pft_frozen_subset.jsonl"), workers), S("pf_set.jsonl"))),
    }
    pinned = {(route, kind, name): h for (route, kind), entries in pins.items() for h, name in entries.items()}
    status = 0
    names = {name for _, _, name in pinned}
    for name in sorted(names ^ set(recipes)):
        print(f"FAIL   {name}: {'a pin without a recipe' if name in names else 'a recipe without a pin'}")
        status = 1
    for name, (route, kind, mandatory, make) in recipes.items():
        want = pinned.get((route, kind, name))
        if want is None:
            continue
        try:
            got = sha(make())
        except Exception as e:
            print(f"FAIL   {route} {kind} {name}: not regenerated ({e})")
            status = 1
            continue
        ok = got == want
        status |= 0 if ok else 1
        print(f"{'OK    ' if ok else 'FAIL  '} {route} {kind} {name}: regenerated {got}, pin {want}")
    print(f"REPRODUCE: {'PASS' if status == 0 else 'FAIL'} ({len(pinned)} pinned files, every one mandatory)")
    raise SystemExit(status)


if __name__ == "__main__":
    main()
