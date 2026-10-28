"""PF M3-T premise census (pf-timoshenko.md section 5; K-D5 design section 2, A1 and epsilon_E).

kd5t_census.py (../kd5_timoshenko, read-only) unchanged, with its engine bound to pft_engine, so its exact
free-DOF stiffness is the PF reference's (PF's single section, kappa_E from E/(2G)). Its docstring states
the modes, the probe-dump format and the arithmetic. What PF publishes for the census is what the exact
route publishes: the gate's reciprocal_condition_estimate, scale_exponents and global_dof_map inside the
NUMERICAL_INTEGRITY diagnostic (established on the format probes). It does not publish est, ||K~_s||_1 or
epsilon_E, so the epsilon_E go/no-go needs an instrumented product build (`probe`), as on the exact route.

  python3 -I -B pft_census.py hager <set.jsonl> <runs.jsonl>... --out <f>
  python3 -I -B pft_census.py probe <dump.jsonl> [--workers N] [--requests <in.jsonl>...] --out <f>
  python3 -I -B pft_census.py standings <runs.jsonl>... [--out <f>]
  python3 -I -B pft_census.py --selftest <set.jsonl> <runs.jsonl>
"""

from __future__ import annotations

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "kd5_timoshenko"))
import pft_engine  # noqa: E402
import kd5t_census as KCE  # noqa: E402  (read-only reuse)

KCE.E = pft_engine            # the census's exact engine is the PF reference
_corpus_of = KCE.corpus_of


def corpus_of(tag):
    return "PF M3-T" if tag.startswith(("pft|", "probe|")) else _corpus_of(tag)


KCE.corpus_of = corpus_of

if __name__ == "__main__":
    KCE.main()
