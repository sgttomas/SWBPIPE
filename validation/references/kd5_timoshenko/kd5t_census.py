"""M3-T premise census (section-bases Design section 4 item 5; K-D5 design section 2, A1 and epsilon_E).

What the product exposes: the K-D5 gate's `reciprocal_condition_estimate` r_hat, its power-of-two
`scale_exponents` D (one per free DOF, in global DOF order) and `global_dof_map`, all inside the
NUMERICAL_INTEGRITY_* diagnostic message. What it does not expose: ||K~_s||_1 (the gate's own
norm), est itself, and epsilon_E (the formation-error triangle bound). So:

- Hager ratio. With est = 1/(r_hat ||K~_s||_1) (K-D5 section 2, step 7), the ratio of the true
  inverse norm to the estimate is ||K~_s^-1||_1 / est = kappa_1(K~_s) * r_hat. kappa_1 is formed
  here from the independent reference stiffness: K~ = D K_ff D with the product's D and the exact
  (110-digit) free-DOF stiffness of kd5t_engine, whose entries differ from the product's binary64
  prepared matrix by input-representation rounding only. A1 is the claim ratio <= 2^10.
- epsilon_E / ||K~_s||_1 cannot be measured from outside the product: it needs the product's own
  binary64 element contributions in assembly order. It is reported as unavailable.

Usage: python3 -I -B kd5t_census.py <set.jsonl> <runs.jsonl> [<runs.jsonl> ...] --out <census.jsonl>
"""

from __future__ import annotations

import json
import os
import re
import sys
from decimal import Decimal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import kd5t_engine as E  # noqa: E402


def parse_gate(message):
    m = re.search(r"reciprocal_condition_estimate: ([0-9.eE+-]+)", message)
    s = re.search(r"scale_exponents: \[([^\]]*)\]", message)
    g = re.search(r"global_dof_map=\[([^\]]*)\]", message)
    if not (m and s and g):
        return None
    exps = [int(x) for x in s.group(1).split(",") if x.strip()]
    dofmap = [x.strip().strip('"') for x in g.group(1).split(",") if x.strip()]
    return float(m.group(1)), exps, dofmap


def inverse(a):
    n = len(a)
    cols = []
    for k in range(n):
        e = [E.ZERO] * n
        e[k] = E.ONE
        cols.append(E.solve_dense(a, e))
    return [[cols[c][r] for c in range(n)] for r in range(n)]


def norm1(a):
    n = len(a)
    return max(sum(abs(a[r][c]) for r in range(n)) for c in range(n))


def census(gen, run, max_n=72):
    ds = run.get("diagnostics") or []
    msg = next((d["message"] for d in ds if d["code"].startswith("NUMERICAL_INTEGRITY")), None)
    rec = {"tag": run["tag"], "entry": run["entry"], "mode": run["mode"], "code": next((d["code"] for d in ds if d["code"].startswith("NUMERICAL_INTEGRITY")), None)}
    if msg is None:
        rec["status"] = "no integrity diagnostic"
        return rec
    gate = parse_gate(msg)
    if gate is None:
        rec["status"] = "gate values not in the message"
        return rec
    rhat, exps, dofmap = gate
    ref = E.Reference(gen["request"])
    case = ref.cases["case:1"]
    names = [f"{n}:{d}" for n in ref.node_order for d in E.DOFS]
    if names != dofmap:
        rec["status"] = "global_dof_map differs from the request node/DOF order"
        return rec
    free = case["free"]
    if len(free) != len(exps):
        rec["status"] = f"scale exponent count {len(exps)} != free DOFs {len(free)}"
        return rec
    if len(free) > max_n:
        rec["status"] = "too large for the exact inverse"
        return rec
    if not free:
        rec["status"] = "no free DOFs"
        return rec
    Dg = [Decimal(2) ** e for e in exps]
    K = case["Kff"]
    Ks = [[Dg[r] * K[r][c] * Dg[c] for c in range(len(free))] for r in range(len(free))]
    diag = [Ks[k][k] for k in range(len(free))]
    nK = norm1(Ks)
    nInv = norm1(inverse(Ks))
    kappa = nK * nInv
    rec.update({"status": "ok", "n_free": len(free), "rhat": rhat, "norm1_Ks": float(nK), "norm1_inv": float(nInv), "kappa1": float(kappa),
                "hager_ratio": float(kappa * Decimal(repr(rhat))), "diag_min": float(min(diag)), "diag_max": float(max(diag)),
                "A1_margin": float(Decimal(1024) / (kappa * Decimal(repr(rhat)))), "epsilon_E_over_norm": None,
                "epsilon_E_status": "not exposed by the product (needs its binary64 element contributions in assembly order)"})
    return rec


def work(job):
    gen, runs = job
    out = []
    for run in runs:
        try:
            out.append(census(gen, run))
        except Exception as exc:  # noqa: BLE001
            out.append({"tag": run["tag"], "entry": run["entry"], "mode": run["mode"], "status": "error: " + repr(exc)[:300]})
    return out


def main():
    args = sys.argv[1:]
    i = args.index("--out")
    out_path = args[i + 1]
    del args[i:i + 2]
    set_path, run_paths = args[0], args[1:]
    gens = {}
    for line in open(set_path):
        x = json.loads(line)
        gens[x["tag"]] = x
    runs = {}
    for p in run_paths:
        for line in open(p):
            r = json.loads(line)
            if r["entry"] != "typed":
                continue  # the gate is entry-independent; the captured entry repeats it
            runs.setdefault(r["tag"], []).append({k: r.get(k) for k in ("tag", "entry", "mode", "diagnostics")})
    import multiprocessing as mp
    jobs = [(gens[t], rs) for t, rs in runs.items()]
    with mp.get_context("fork").Pool(6) as pool, open(out_path, "w") as f:
        for res in pool.imap_unordered(work, jobs, chunksize=2):
            for rec in res:
                f.write(json.dumps(rec) + "\n")


if __name__ == "__main__":
    main()
