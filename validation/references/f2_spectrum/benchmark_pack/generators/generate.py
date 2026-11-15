"""Generate or check the F2 benchmark pack (validation/references/f2_spectrum/benchmark_pack).

  python3 generators/generate.py --check   regenerate in memory and compare byte for byte (default)
  python3 generators/generate.py --write   write the JSON files and SHA256SUMS

Standard library only. Imports the closed_form generator modules read-only, pinned by SHA-256
(generators/pinned.py), and reads discrete/c8_pipe_x.json (pinned) for one cross-check. Imports no
product code. Writes no bytecode, so nothing is written next to the pinned modules.
"""

import sys

sys.dont_write_bytecode = True

import argparse  # noqa: E402
import hashlib  # noqa: E402
import time  # noqa: E402
from pathlib import Path  # noqa: E402

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
sys.path.insert(0, str(HERE))

import pinned  # noqa: E402
N_, R_, C_, FE_, B_ = pinned.load()
import pack_c  # noqa: E402
import pack_d  # noqa: E402
import pack_summary  # noqa: E402
import pack_predicted  # noqa: E402

SCHEMA = "swbpipe.f2_spectrum.benchmark_pack/v1"
STATUS = "references with a refutation record (REFUTATION.md)"
AUTHORED = ["README.md", "REFUTATION.md"]
GENERATORS = ["generators/pinned.py", "generators/pack_c.py", "generators/pack_d.py", "generators/pack_summary.py",
              "generators/pack_predicted.py", "generators/generate.py"]
NUMBER_FORMAT = ("Inputs are binary64 ('binary64' shortest round trip, 'hex', 'exact' rational). Derived reals are "
                 "decimal strings with 40 significant digits and the nearest binary64. rel_dev and cross-check "
                 "residuals are short diagnostics. SI units.")
FILES = [("case_c.json", pack_c.build), ("case_d.json", pack_d.build), ("pack_summary.json", pack_summary.build),
         ("predicted_formulation_difference.json", pack_predicted.build)]


def build():
    files, crosses, store = {}, {}, {}
    pins = {"closed_form/generators/" + k: v for k, v in pinned.PINNED.items()}
    pins["discrete/c8_pipe_x.json"] = pinned.C8_SHA256
    for name, fn in FILES:
        cross = []
        body = fn(cross, store)
        doc = dict(body.pop("_head", {}))
        doc.update({"schema": SCHEMA, "file": name, "status": STATUS, "number_format": NUMBER_FORMAT,
                    "pinned_inputs_sha256": pins})
        doc.update(body)
        doc["cross_checks"] = cross
        files[name] = N_.emit_json(doc)
        if name == "pack_summary.json":
            store["summary_bytes"] = files[name]
        crosses[name] = cross
    for path in AUTHORED + GENERATORS:
        files[path] = (OUT / path).read_bytes()
    files["SHA256SUMS"] = "".join(f"{hashlib.sha256(files[p]).hexdigest()}  {p}\n" for p in sorted(files)).encode()
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
    print(f"cross-checks: {sum(len(c) for c in crosses.values())} recorded, {len(bad)} disagreeing "
          f"({time.time() - t0:.0f} s)")
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
