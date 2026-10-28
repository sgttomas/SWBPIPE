"""M3-T comparison of the PF route: routed through the shared M3 judge.

The PF judging that lived here (kd5t_compare's reconstructed judge with a single-section adapter, the strict schemas,
the request binding, the inventory, the design-conformance clause C10 and the derived-row clauses C11, C11tau_b and
C11ops, refuted in rounds 1-4) is ported unchanged in meaning to ../m3_judge/m3_compare.py, which judges only the
canonical evidence view that ../m3_judge/m3_view.py validates from each raw record (route `pf`). The view matches each
record's rows against the semantic contract its request's shear model selects: preview-physics-2 for
Euler-Bernoulli, preview-physics-timoshenko-1 for Timoshenko (DEL-05-03 Design/pf-timoshenko.md). One judge, built
once, serves this set and the exact route's (../kd5_timoshenko/).

  python3 -I -B ../m3_judge/m3_report.py --route pf <set.jsonl> <raw outputs...> --runner-sha256 <hex64>
          --product-head <hex40> --checkout <checkout> [--ref own|twin]
  ../m3_judge/run.sh pf <checkout> <set dir> <run dir> own [twin]   (or ./run.sh, which calls it)

This file keeps its name for pft_generate.py `check`, which calls `pft_compare.py --selftest <frozen subset>`: that
runs the shared judge's self-test, whose PF view is judged on its selftest set, 20 verbatim lines of this directory's
frozen subset (../m3_judge/selftest/pf_set.jsonl, pinned in ../m3_judge/REFERENCE_SHA256SUMS).
"""

from __future__ import annotations

import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
JUDGE = os.path.join(HERE, "..", "m3_judge")


def main():
    args = sys.argv[1:]
    if len(args) == 2 and args[0] == "--selftest":
        frozen = os.path.realpath(args[1])
        if frozen != os.path.realpath(os.path.join(HERE, "frozen", "pft_frozen_subset.jsonl")):
            raise SystemExit("the shared judge's self-test judges the PF view on lines of frozen/pft_frozen_subset.jsonl only")
        raise SystemExit(subprocess.run([sys.executable, "-I", "-B", os.path.join(JUDGE, "m3_selftest.py")]).returncode)
    raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
