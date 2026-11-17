"""Generate or check the F2 closed-form spectrum references (validation/references/f2_spectrum/closed_form).

  python3 generators/generate.py --check   regenerate in memory and compare byte for byte (default)
  python3 generators/generate.py --write   write the JSON files and SHA256SUMS

Standard library only. The generators import, read and run no product code.
All model data are invented.
"""

import argparse
import hashlib
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
sys.path.insert(0, str(HERE))

import beams_build  # noqa: E402
import chain  # noqa: E402
import directional  # noqa: E402
import screening  # noqa: E402
import sdof  # noqa: E402
import sensitivity  # noqa: E402
import skew  # noqa: E402
import tables  # noqa: E402
from numerics import emit_json  # noqa: E402

SCHEMA = "swbpipe.f2_spectrum.closed_form/v1"
STATUS = "references with a refutation record (REFUTATION.md)"
AUTHORED = ["README.md", "REFUTATION.md"]
GENERATORS = ["generators/numerics.py", "generators/rules.py", "generators/tables.py", "generators/sdof.py",
              "generators/chain.py", "generators/directional.py", "generators/continuum.py", "generators/fe.py",
              "generators/beams.py", "generators/beams_build.py", "generators/sensitivity.py",
              "generators/skew.py", "generators/screening.py", "generators/generate.py"]
NUMBER_FORMAT = ("Inputs are binary64: 'binary64' is the shortest round-trip decimal, 'hex' the exact hex form, "
                 "'exact' the exact rational. Derived reals are decimal strings ('value', 40 significant digits) "
                 "with the nearest binary64; 'exact' on a derived quantity is an exact rational. Deviations and "
                 "cross-check residuals are short diagnostics, not references. SI units.")

FILES = [("spectrum_tables.json", tables.build), ("sdof.json", sdof.build), ("two_mass_chain.json", chain.build),
         ("directional.json", directional.build), ("beams_continuum.json", beams_build.build_continuum),
         ("beams_fe.json", beams_build.build_fe), ("sensitivity.json", sensitivity.build),
         ("beam_skew_and_pairs.json", skew.build), ("screening_basis.json", screening.build)]


def build():
    files, crosses = {}, {}
    for name, fn in FILES:
        cross = []
        body = fn(cross)
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
    args = ap.parse_args()
    t0 = time.time()
    files, crosses = build()
    bad = [(f, c["id"]) for f, cs in crosses.items() for c in cs if not c["agrees"]]
    total = sum(len(cs) for cs in crosses.values())
    print(f"cross-checks: {total} recorded, {len(bad)} disagreeing ({time.time() - t0:.0f} s)")
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
