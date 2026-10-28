"""M3-T comparison of the exact route: routed through the shared M3 judge (refutation round 5).

The reconstructed K-D5 M3 judging that lived here (its rules and their sources, C0-C9 and the information
clauses) is ported unchanged in meaning to ../m3_judge/m3_compare.py, which judges only the canonical evidence view
that ../m3_judge/m3_view.py validates from each raw record (route `exact`: both D-5 bases, `exact_cases` evidence).
The earlier judge searched the whole raw record for evidence while its schema checks read a narrower subtree
(round 5); one judge, built once, now serves this set and the PF M3-T set.

  python3 -I -B ../m3_judge/m3_report.py --route exact <set.jsonl> <raw outputs...> --runner-sha256 <hex64>
          --product-head <hex40> --checkout <checkout> [--ref own|twin|control --controls <controls.jsonl>]
  ../m3_judge/run.sh exact <checkout> <set dir> <run dir> own [twin] [control]   (or ./run.sh, which calls it)

This file keeps its name for kd5t_generate.py `check`, which calls `kd5t_compare.py --selftest <frozen subset>`:
that runs the shared judge's self-test, whose exact view is judged on this directory's frozen subset.
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
        if frozen != os.path.realpath(os.path.join(HERE, "frozen", "kd5t_frozen_subset.jsonl")):
            raise SystemExit("the shared judge's self-test judges the exact view on frozen/kd5t_frozen_subset.jsonl only")
        raise SystemExit(subprocess.run([sys.executable, "-I", "-B", os.path.join(JUDGE, "m3_selftest.py")]).returncode)
    raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
