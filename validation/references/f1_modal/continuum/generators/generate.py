"""Generate or check the F1 continuum references (validation/references/f1_modal/continuum).

  python3 generators/generate.py --check   regenerate in memory and compare byte for byte (default)
  python3 generators/generate.py --write   write continuum.json, fe_convergence.json,
                                           inertia_witnesses.json and SHA256SUMS
  python3 generators/generate.py --search-round2
                                           re-run the round-2 witness search and report
                                           the first hit (it must equal the frozen one)

Standard library only. The generators import, read and run no product code.
All model data are invented.
"""

import argparse
import hashlib
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
sys.path.insert(0, str(HERE))

import continuum  # noqa: E402
import fe_convergence  # noqa: E402
import inertia_witnesses  # noqa: E402
from numerics import emit_json  # noqa: E402

SCHEMA = "swbpipe.f1_modal.continuum/v1"
STATUS = "candidate references, not yet refuted"
AUTHORED = ["README.md", "REFUTATION.md"]
GENERATORS = ["generators/numerics.py", "generators/continuum.py", "generators/fe_convergence.py",
              "generators/inertia_witnesses.py", "generators/generate.py"]
NUMBER_FORMAT = ("Inputs are binary64: 'binary64' is the shortest round-trip decimal, 'hex' the exact hex form, "
                 "'exact' the exact rational. Derived reals are decimal strings ('value', 40 significant digits "
                 "unless stated) with the nearest binary64. 'exact' on a derived quantity is an exact rational.")


def build():
    files = {}
    crosses = {}
    for name, mod in (("continuum.json", continuum), ("fe_convergence.json", fe_convergence),
                      ("inertia_witnesses.json", inertia_witnesses)):
        cross = []
        body = mod.build(cross)
        doc = {"schema": SCHEMA, "file": name, "status": STATUS, "number_format": NUMBER_FORMAT}
        doc.update(body)
        doc["cross_checks"] = cross
        files[name] = emit_json(doc)
        crosses[name] = cross
    for path in AUTHORED + GENERATORS:
        files[path] = (OUT / path).read_bytes()
    lines = [f"{hashlib.sha256(files[p]).hexdigest()}  {p}\n" for p in sorted(files)]
    files["SHA256SUMS"] = "".join(lines).encode()
    return files, crosses


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    g = ap.add_mutually_exclusive_group()
    g.add_argument("--write", action="store_true")
    g.add_argument("--check", action="store_true")
    g.add_argument("--search-round2", action="store_true")
    args = ap.parse_args()

    if args.search_round2:
        hit = inertia_witnesses.r2_search()
        frozen = (inertia_witnesses.R2_KK, inertia_witnesses.R2_MODE)
        print(f"first hit: kk={hit[0]} mode={hit[1]}; frozen: kk={frozen[0]} mode={frozen[1]}")
        return 0 if hit == frozen else 1

    files, crosses = build()
    bad = [(f, c["id"]) for f, cs in crosses.items() for c in cs if not c["agrees"]]
    total = sum(len(cs) for cs in crosses.values())
    print(f"cross-checks: {total} recorded, {len(bad)} disagreeing")
    for f, cid in bad:
        print(f"  DISAGREES {f}: {cid}")
    generated = [p for p in files if p not in AUTHORED + GENERATORS]
    if args.write:
        for p in generated:
            (OUT / p).write_bytes(files[p])
        print(f"wrote {', '.join(sorted(generated))}")
        return 0 if not bad else 1
    mismatch = [p for p in sorted(files) if not (OUT / p).exists() or (OUT / p).read_bytes() != files[p]]
    present = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(present - set(files))
    if mismatch or extra or bad:
        print("MISMATCH: " + ", ".join(mismatch + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
