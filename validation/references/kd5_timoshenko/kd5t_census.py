"""M3-T premise census (section-bases Design section 4 item 5; K-D5 design section 2, A1 and epsilon_E).

A1 is the claim that the Hager estimate is within 2^10 of the true inverse norm of the gate's prepared
matrix K~_s; the epsilon_E go/no-go is epsilon_E <= 2^-40 ||K~_s||_1 on every case-mode (no-go above
2^-36) [K-D5 section 2.1].

What the product publishes: the gate's reciprocal_condition_estimate r_hat, its power-of-two scale
exponents D (one per free DOF, global DOF order) and global_dof_map, inside the NUMERICAL_INTEGRITY
diagnostic. It does not publish est, ||K~_s||_1 or epsilon_E. Hence three modes:

  hager      <set.jsonl> <runs.jsonl>... --out <f>   Hager ratio from published values only:
             ||K~_s^-1||_1 / est = kappa_1(D K D) * r_hat, with the exact free-DOF stiffness of
             kd5t_engine (straight members). diag(D K D) in [1, 4) confirms the mapping.
  probe      <dump.jsonl> [--workers N] [--requests <in.jsonl>...] --out <f>
             consumes the dump of an instrumented product build (format below): epsilon_E/||K~_s||_1
             against 2^-40 and 2^-36, 2^10 est epsilon_E, and the Hager ratio ||K~_s^-1||_1/est and the A1
             margin 2^10 est/||K~_s^-1||_1, with the inverse of the dumped K~_s (binary64 entries taken
             exactly; one Gauss-Jordan elimination in 110-digit decimal, which the self-test checks against
             the exact rational inverse), or, without a dumped matrix, of D K D from the engine for
             straight-only requests.
  standings  <runs.jsonl>... [--out <f>]            Passed / Sensitive / fail-closed / refused counts per
             corpus and shear model, paired by tag stem (the "|reauth-eb" / "|reauth-ts" suffix or the
             set's twin tags).
  --selftest <set.jsonl> <runs.jsonl>               validates the probe arithmetic on synthetic dumps.

Probe dump format (one JSON line per K-D5 evaluation; the runner sets KD5_ACTIVE_TAG, KD5_ACTIVE_ENTRY
and KD5_ACTIVE_MODE so the probe can label lines):
  {"tag", "entry", "mode", "step7": whether the evaluation reached step 7, "r_hat", "est", "norm1_Ks",
   "epsilon_E" (the product's own at step 7; formed by the probe with the same function otherwise),
   "epsilon_E_parts": [elements, assembly, projection] (each the larger of its row- and column-sum norms),
   "neumann_2p10_est_epsE", "outcome": "bound" (step 7 completed: the member-bound inputs exist; the
   standing is then PP's) | "sensitive:estimate" (D5C-1) | "fail_closed:<detail>",
   optional "case", "attempt", "contraction", "Ks": [[...]] (the prepared scaled symmetric matrix the
   factor sees) and "scale_exponents"}. The README records the probe used for the census.
"""

from __future__ import annotations

import collections
import json
import os
import re
import sys
from decimal import Decimal
from fractions import Fraction as Q

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import kd5t_engine as E  # noqa: E402

GO = Q(1, 2 ** 40)
NO_GO = Q(1, 2 ** 36)


def parse_gate(message):
    m = re.search(r"reciprocal_condition_estimate: ([0-9.eE+-]+)", message)
    s = re.search(r"scale_exponents: \[([^\]]*)\]", message)
    g = re.search(r"global_dof_map=\[([^\]]*)\]", message)
    if not (m and s and g):
        return None
    exps = [int(x) for x in s.group(1).split(",") if x.strip()]
    dofmap = [x.strip().strip('"') for x in g.group(1).split(",") if x.strip()]
    return float(m.group(1)), exps, dofmap


def inverse_norm1(a):
    """||a^-1||_1 for a square matrix of Decimal or Fraction entries (exact for Fractions)."""
    n = len(a)
    best = None
    for k in range(n):
        e = [0 * a[0][0]] * n
        e[k] = 1 + 0 * a[0][0]
        col = E.solve_dense(a, e)
        s = sum(abs(x) for x in col)
        best = s if best is None or s > best else best
    return best


def inverse_norm1_gj(a):
    """||a^-1||_1 by one Gauss-Jordan elimination with partial pivoting on [a | I], O(n^3) against the n
    eliminations of inverse_norm1: exact for Fraction entries; for Decimal entries (each binary64 entry
    converted exactly) the arithmetic carries kd5t_engine's decimal context (110 digits)."""
    n = len(a)
    zero, one = 0 * a[0][0], 1 + 0 * a[0][0]
    m = [list(row) + [one if c == r else zero for c in range(n)] for r, row in enumerate(a)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(m[r][col]))
        if m[piv][col] == 0:
            raise ZeroDivisionError("singular matrix")
        m[col], m[piv] = m[piv], m[col]
        pr = m[col]
        inv = one / pr[col]
        for c in range(col, 2 * n):
            pr[c] = pr[c] * inv
        for r in range(n):
            if r == col:
                continue
            mr = m[r]
            f = mr[col]
            if f != 0:
                for c in range(col, 2 * n):
                    if pr[c] != 0:
                        mr[c] -= f * pr[c]
    return max(sum(abs(m[r][n + c]) for r in range(n)) for c in range(n))


def norm1(a):
    n = len(a)
    return max(sum(abs(a[r][c]) for r in range(n)) for c in range(n))


def engine_scaled(request, exps):
    ref = E.Reference(request)
    case = ref.cases[next(iter(ref.cases))]
    free = case["free"]
    if len(free) != len(exps):
        raise ValueError(f"scale exponent count {len(exps)} != free DOFs {len(free)}")
    Dg = [Decimal(2) ** e for e in exps]
    K = case["Kff"]
    return ref, [[Dg[r] * K[r][c] * Dg[c] for c in range(len(free))] for r in range(len(free))]


def hager_record(gen, run, max_n=72):
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
    names = [f"{n['id']}:{d}" for n in gen["request"]["model"]["nodes"] for d in E.DOFS]
    if names != dofmap:
        rec["status"] = "global_dof_map differs from the request node/DOF order"
        return rec
    if not exps:
        rec["status"] = "no free DOFs"
        return rec
    if len(exps) > max_n:
        rec["status"] = "too large for the exact inverse"
        return rec
    _, Ks = engine_scaled(gen["request"], exps)
    nK, nInv = norm1(Ks), inverse_norm1(Ks)
    kappa = nK * nInv
    diag = [Ks[k][k] for k in range(len(Ks))]
    rec.update({"status": "ok", "n_free": len(Ks), "rhat": rhat, "norm1_Ks": float(nK), "norm1_inv": float(nInv), "kappa1": float(kappa),
                "hager_ratio": float(kappa * Decimal(repr(rhat))), "diag_min": float(min(diag)), "diag_max": float(max(diag)),
                "A1_margin": float(Decimal(1024) / (kappa * Decimal(repr(rhat))))})
    return rec


def probe_record(line, requests, exact=False):
    rec = {k: line.get(k) for k in ("tag", "entry", "mode", "case", "attempt", "outcome", "step7", "n_free", "r_hat")}
    nK = Q(line["norm1_Ks"]) if isinstance(line.get("norm1_Ks"), (int, float)) else None
    eps = Q(line["epsilon_E"]) if isinstance(line.get("epsilon_E"), (int, float)) else None
    est = Q(line["est"]) if isinstance(line.get("est"), (int, float)) else None
    if nK and eps is not None:
        r = eps / nK
        rec["epsilon_E_over_norm"] = float(r)
        rec["go"] = r <= GO
        rec["no_go"] = r > NO_GO
    if est is not None and eps is not None:
        rec["neumann"] = float(1024 * est * eps)
    if est is not None:
        inv = None
        if line.get("Ks"):
            Ks = [[Q(v) for v in row] for row in line["Ks"]]
            if nK is not None and norm1(Ks) != nK:
                rec["norm_mismatch"] = float(norm1(Ks) / nK)
            if exact:
                inv = Q(inverse_norm1_gj(Ks))
                rec["hager_basis"] = "dumped K~_s (exact rational inverse of its binary64 entries)"
            else:
                inv = Q(inverse_norm1_gj([[Decimal(v) for v in row] for row in line["Ks"]]))
                rec["hager_basis"] = "dumped K~_s (binary64 entries taken exactly; inverse in 110-digit decimal)"
        elif line.get("scale_exponents") and line["tag"] in requests:
            _, Ks = engine_scaled(requests[line["tag"]], line["scale_exponents"])
            inv = Q(inverse_norm1(Ks))
            rec["hager_basis"] = "engine D K D (straight members)"
        if inv is not None:
            rec["hager_ratio"] = float(inv / est)
            rec["A1"] = inv <= 1024 * est
            rec["A1_margin"] = float(1024 * est / inv)
    return rec


_REQUESTS = {}


def _probe_job(line):
    return probe_record(json.loads(line), _REQUESTS)


def classify(run):
    ds = run.get("diagnostics") or []
    if run.get("error"):
        return "error"
    if any(d.get("severity") == "blocking" for d in ds):
        return "refused:" + ",".join(sorted({d["code"] for d in ds if d.get("severity") == "blocking"}))
    codes = [d for d in ds if d["code"].startswith("NUMERICAL_INTEGRITY")]
    if not codes:
        return "no-integrity"
    if codes[0]["code"] == "NUMERICAL_INTEGRITY_CHECKS_PASSED":
        return "passed"
    msg = codes[0].get("message", "")
    if "reason=formation_check_unavailable" in msg or "member-relative formation check is not available" in msg:
        return "fail-closed"
    if "S11-G recovery guard (R-b'): load case" in msg:
        return "sensitive-recovery-guard"
    if "reason=estimate" in msg:
        return "sensitive-d5c1"
    if "S11-G formation-noise guard" in msg:
        return "sensitive-load-row"
    return "sensitive-gate-or-solve"


def stem(tag):
    for suf in ("|reauth-eb", "|reauth-ts", "|eb", "|ts"):
        if tag.endswith(suf):
            return tag[: -len(suf)], suf.split("-")[-1].strip("|")
    return tag, "?"


def corpus_of(tag):
    t = tag.split("|")[0]
    if t.startswith("invented:rv30") or tag[:2] in ("A|", "B|", "C|", "D|"):
        return "RV30"
    if "i43w1" in t:
        return "W1"
    if "rv29" in t:
        return "RV29"
    if t.startswith("m3t"):
        return "M3-T"
    return "realistic"


def standings(paths):
    c = collections.defaultdict(collections.Counter)
    pairs = collections.defaultdict(dict)
    for p in paths:
        for line in open(p):
            r = json.loads(line)
            s, th = stem(r["tag"])
            cls = classify(r)
            c[(corpus_of(r["tag"]), th)][cls] += 1
            pairs[(s, r["entry"], r["mode"])][th] = cls
    moves = collections.Counter()
    for k, v in pairs.items():
        if "eb" in v and "ts" in v:
            moves[(v["eb"], v["ts"])] += 1
    return c, moves


def selftest(set_path, runs_path):
    """Synthetic dumps from the engine and the gate's published r_hat and D: the probe arithmetic must
    reproduce the hager-mode ratio, and the go/no-go and A1 flags must flip at their thresholds."""
    gens = {}
    for line in open(set_path):
        x = json.loads(line)
        gens[x["tag"]] = x
    n = 0
    bad = []
    for line in open(runs_path):
        run = json.loads(line)
        if run["entry"] != "typed" or n >= 6:
            continue
        h = hager_record(gens[run["tag"]], run)
        if h.get("status") != "ok":
            continue
        n += 1
        msg = next(d["message"] for d in run["diagnostics"] if d["code"].startswith("NUMERICAL_INTEGRITY"))
        rhat, exps, _ = parse_gate(msg)
        _, Ks = engine_scaled(gens[run["tag"]]["request"], exps)
        nK = Q(norm1(Ks))
        est = 1 / (Q(rhat) * nK)
        for eps_ratio, go, nogo in ((Q(1, 2 ** 41), True, False), (Q(1, 2 ** 38), False, False), (Q(1, 2 ** 35), False, True)):
            dump = {"tag": run["tag"], "entry": "typed", "mode": run["mode"], "case": "case:1", "r_hat": rhat, "est": float(est),
                    "norm1_Ks": float(nK), "epsilon_E": float(eps_ratio * nK), "scale_exponents": exps}
            rec = probe_record(dump, {run["tag"]: gens[run["tag"]]["request"]})
            if rec["go"] != go or rec["no_go"] != nogo:
                bad.append((run["tag"], "go/no-go", float(eps_ratio)))
            if abs(rec["hager_ratio"] / h["hager_ratio"] - 1) > 1e-12:
                bad.append((run["tag"], "hager ratio", rec["hager_ratio"], h["hager_ratio"]))
        dump = {"tag": run["tag"], "entry": "typed", "mode": run["mode"], "est": float(est) / 2000, "norm1_Ks": float(nK), "epsilon_E": 0.0,
                "scale_exponents": exps}
        if probe_record(dump, {run["tag"]: gens[run["tag"]]["request"]})["A1"]:
            bad.append((run["tag"], "A1 not refuted by an estimate 2000x too small"))
        # a dumped matrix (binary64 entries of D K D) gives the same Hager ratio to rounding
        dump = {"tag": run["tag"], "entry": "typed", "mode": run["mode"], "est": float(est), "norm1_Ks": float(nK), "epsilon_E": 0.0,
                "Ks": [[float(v) for v in row] for row in Ks]}
        rec = probe_record(dump, {})
        if abs(rec["hager_ratio"] / h["hager_ratio"] - 1) > 1e-9:
            bad.append((run["tag"], "dumped-matrix hager ratio", rec["hager_ratio"], h["hager_ratio"]))
        # the dumped matrix's 110-digit inverse norm agrees with its exact rational inverse norm, and the one
        # Gauss-Jordan elimination agrees with the column-by-column eliminations
        exact = Q(inverse_norm1_gj([[Q(float(v)) for v in row] for row in Ks]))
        dec = Q(inverse_norm1_gj([[Decimal(float(v)) for v in row] for row in Ks]))
        if abs(dec / exact - 1) > Q(1, 10 ** 80):
            bad.append((run["tag"], "110-digit against exact inverse norm", float(dec / exact - 1)))
        if probe_record(dump, {}, exact=True)["A1"] != rec["A1"]:
            bad.append((run["tag"], "exact and 110-digit A1 differ"))
        if abs(Q(inverse_norm1_gj(Ks)) / Q(inverse_norm1(Ks)) - 1) > Q(1, 10 ** 80):
            bad.append((run["tag"], "Gauss-Jordan against column eliminations"))
    print(f"census self-test: {n} models, {len(bad)} failures {bad[:3]}")
    if bad or n == 0:
        raise SystemExit(1)


def main():
    args = sys.argv[1:]
    if args[0] == "--selftest":
        selftest(args[1], args[2])
        return
    mode = args.pop(0)
    out_path = None
    if "--out" in args:
        i = args.index("--out")
        out_path = args[i + 1]
        del args[i:i + 2]
    if mode == "hager":
        set_path, run_paths = args[0], args[1:]
        gens = {}
        for line in open(set_path):
            x = json.loads(line)
            gens[x["tag"]] = x
        jobs = []
        for p in run_paths:
            for line in open(p):
                r = json.loads(line)
                if r["entry"] == "typed":  # the gate is entry-independent
                    jobs.append((gens[r["tag"]], {k: r.get(k) for k in ("tag", "entry", "mode", "diagnostics")}))
        import multiprocessing as mp
        with mp.get_context("fork").Pool(6) as pool, open(out_path, "w") as f:
            for rec in pool.starmap(hager_record, jobs, chunksize=4):
                f.write(json.dumps(rec) + "\n")
    elif mode == "probe":
        workers = 6
        if "--workers" in args:
            i = args.index("--workers")
            workers = int(args[i + 1])
            del args[i:i + 2]
        if "--requests" in args:
            i = args.index("--requests")
            for p in args[i + 1:]:
                for line in open(p):
                    x = json.loads(line)
                    _REQUESTS[x["tag"]] = x["request"]
            del args[i:]
        import multiprocessing as mp
        with mp.get_context("fork").Pool(workers) as pool, open(out_path, "w") as f:
            for rec in pool.imap(_probe_job, open(args[0]), chunksize=8):
                f.write(json.dumps(rec) + "\n")
    elif mode == "standings":
        c, moves = standings(args)
        for k in sorted(c):
            print(k, dict(sorted(c[k].items())))
        print("EB -> TS moves:", dict(moves))
        if out_path:
            json.dump({"standings": {f"{a}|{b}": dict(v) for (a, b), v in c.items()}, "moves": {f"{a} -> {b}": n for (a, b), n in moves.items()}},
                      open(out_path, "w"), indent=1, sort_keys=True)
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
