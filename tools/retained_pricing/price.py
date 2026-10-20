"""Retained pricing: SQ2's TEXT + cap-priced chain under the O-10 rules, as one command (stdlib only).

  run <rev> --out DIR        price a revision: snapshot -> carry the base chain's line-keyed rules from the basis to
                             <rev> (offset, else by function, else by text; deleted keys pruned) -> premise pins ->
                             template inventory -> SQ2 + O-10 rules per route (L, E) -> TEXT + cap-priced points
                             (L at (lc, z) = (3,0), (2,1), (1,2); E at 3) -> o10 checks -> the merged profile tree
  generate <tree> <rs>       write the GENERATED PROFILE block of retained_memory.rs from a profile tree
  compare <tree A> <tree B>  compare two profile trees coefficient by coefficient
  eval <atoms log|-> <tree>  evaluate a tree's phases at in-build atom values (a law-test log's I65_G5_ATOM lines)

Environment (all optional): RP_REPO (the repository to read revisions from; default: this checkout),
RP_OUT (default --out), RP_JOB_WRAPPER (a command each point runs under, e.g. a host's job-slot wrapper; split
like a shell word list). See README.md for inputs, outputs, exit codes and limits.
"""
import argparse, difflib, json, os, re, shlex, shutil, subprocess, sys, tempfile
from lib.b7a_retired_carry import prepare as prepare_b7a_retired_carry
from base.b7a_domain_model import prepare as prepare_b7a_domain
from base.b7a_formation_model import prepare as prepare_b7a_formation
from base.loopscan import off_build_files

H = os.path.dirname(os.path.abspath(__file__))
BASE, LIB, RULES, DATA = (os.path.join(H, d) for d in ("base", "lib", "rules", "data"))
BASIS = "2243be380b"                       # the revision the base chain's rule tables and the decisions are keyed at
P = "projects/chirality-piping"
ROOT_FN = "run_linear_static_preview_value_with_retained_direct"
RULE_FILES = ("callgraph_rules.g4.json", "loop_bounds.g4.json", "text_args.g4.json", "sens.py", "premise_pins.json")
POINTS = (("L", 3, 0), ("L", 2, 1), ("L", 1, 2), ("E", 3, 0))
GIT_ENV = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
MARKER = ".retained_pricing_out"            # written in an output directory this tool created or claimed while empty
OWNED = ("snap", "prepared_rules", "map", "chain", "rcL", "rcE", "runs", "logs", "tree_merged.json", "summary.json", "o10_check_L.json", "o10_check_E.json")
RESOLUTIONS = os.path.join(RULES, "linemap_resolutions.json")


def chain_env(**extra):
    """The chain's scripts call `python3`: this interpreter's directory comes first on PATH."""
    return dict(GIT_ENV, PATH=os.path.dirname(sys.executable) + os.pathsep + os.environ.get("PATH", ""), **extra)


def git(repo, *a, **kw):
    return subprocess.run(["git", "-C", repo, *a], env=GIT_ENV, check=True, capture_output=True, **kw).stdout


def py(args, env=None, cwd=None, out=None):
    """Run one chain step; its stdout goes to `out` (a path) or is returned."""
    r = subprocess.run([sys.executable, *args], env=env or chain_env(), cwd=cwd, capture_output=True, text=True)
    if out:
        open(out, "w").write(r.stdout)
    return r


def stop(code, msg, **kw):
    print(json.dumps({"stopped": msg, "code": code, **kw}, indent=1))
    sys.exit(code)


def default_repo():
    return os.environ.get("RP_REPO") or git(H, "rev-parse", "--show-toplevel", text=True).strip()


# ---- run ---------------------------------------------------------------------------------------------------------
def snapshot(repo, rev, dst):
    """git archive of projects/chirality-piping at rev (execution/ excluded) into dst."""
    os.makedirs(dst)
    arc = git(repo, "archive", rev, P, f":(exclude){P}/execution")
    subprocess.run(["tar", "-x", "-C", dst], input=arc, check=True)
    return os.path.join(dst, P)


CORRECT_NORM = ("core/numerics/correct_norm", "open_pipe_stress_correct_norm")


def chain_crates(chain, W):
    """The chain's crate source directories at the snapshot, and the dependency edges they add to the call graph.

    JN1 (retained-pricing-rules.md): the pressure-source join moved frame_kernel's `correct_norm` module, unchanged,
    into its own crate. Where the snapshot has that crate, its source is scanned and each scanned crate whose
    Cargo.toml depends on it gets the dependency edge, so `norm2`/`norm3` calls keep resolving to those functions.
    A snapshot without it prices exactly as before."""
    crates = open(os.path.join(chain, "crate_dirs.txt")).read().split()
    deps = []
    crate, package = CORRECT_NORM
    if os.path.isfile(os.path.join(W, crate, "src", "lib.rs")):
        for c in crates:
            manifest = os.path.join(W, os.path.dirname(c), "Cargo.toml")
            if os.path.isfile(manifest) and re.search(r"^\s*" + re.escape(package) + r"\s*=", open(manifest).read(), re.M):
                deps.append(os.path.dirname(c) + ":" + crate)
        crates = crates + [crate + "/src"]
    return crates, deps


def template_inventory(W, chain, off_build=()):
    """The chain's template inventory regenerated on the snapshot (production .rs files of the chain's crates, less the
    W1R-FG files the registered build does not compile)."""
    files = []
    for c in chain_crates(chain, W)[0]:
        for d, _, fs in os.walk(os.path.join(W, c)):
            if "/tests" in d or "/bin" in d:
                continue
            files += [os.path.relpath(os.path.join(d, f), W) for f in fs if f.endswith(".rs") and not f.endswith("_tests.rs") and f != "tests.rs"
                      and os.path.relpath(os.path.join(d, f), W) not in off_build]
    r = py([os.path.join(chain, "template_inventory.py"), "."] + sorted(files), cwd=W, out=os.path.join(chain, "template_inventory_head.out.json"))
    if r.returncode:
        stop(4, "template inventory failed", stderr=r.stderr[-2000:])


def point(chain, O, caps, W, looplog=True):
    """One TEXT + cap-priced point: call graph, lexicon, sens.py (the whole chain at the caps), then the loop log."""
    if os.path.exists(O):
        shutil.rmtree(O)
    os.makedirs(O)
    crates, extra_deps = chain_crates(chain, W)
    e = chain_env(CG_REPAIR="1", CG_EXTRA_DEPS=",".join(["core/product_physics:core/reporting/result_export"] + extra_deps),
                  CG_RULES=os.path.join(chain, "callgraph_rules.g4.json"),
                  CG_EDGES_OUT=os.path.join(O, "edges.json"), CG_AUDIT_OUT=os.path.join(O, "audit.json"))
    for args, env in (([os.path.join(chain, "callgraph_g5.py"), ".", os.path.join(O, "cg.out.json"), ROOT_FN, *crates], e),
                      ([os.path.join(chain, "text_lexicon.py"), ".", os.path.join(O, "edges.json"), ROOT_FN, os.path.join(O, "lexicon.json")], None)):
        r = py(args, env=env, cwd=W)
        if r.returncode:
            open(os.path.join(O, "step.err"), "w").write(r.stderr)
            return {"error": os.path.basename(args[0]), "rc": r.returncode}
    r = py([os.path.join(chain, "sens.py"), os.path.join(O, "work"), W, os.path.join(O, "edges.json"), os.path.join(O, "lexicon.json"), "2", caps],
           out=os.path.join(O, "summary.json"))
    open(os.path.join(O, "sens.err"), "w").write(r.stderr)
    res = {"sens_rc": r.returncode}
    tbp = os.path.join(O, "work", "text_budget.caps.out.json")
    if not os.path.exists(tbp):
        return res
    t = json.load(open(tbp))
    res.update(complete=t["complete"], D=t["D_diagnostics"], TAV=t["total_text_requested_bytes"], unmapped=len(t["unmapped_loop_headers"]),
               unclassified=len(t["unclassified_args"]), self_recursive=t["self_recursive_ancestors"])
    if looplog:
        wk = os.path.join(O, "work")
        r = py([os.path.join(LIB, "mk_looplog.py"), os.path.join(wk, "text_budget.py"), os.path.join(wk, "text_budget_looplog.py")])
        if r.returncode:
            return dict(res, error="mk_looplog", stderr=r.stderr[-2000:])
        r = py([os.path.join(wk, "text_budget_looplog.py"), W, os.path.join(wk, "template_inventory_head.out.json"), os.path.join(O, "edges.json"),
                os.path.join(wk, "loop_bounds.g4.json"), os.path.join(wk, "text_args.g4.json"), ROOT_FN, "caps"],
               env=chain_env(G4_CAPS=caps, TB_LEXICON=os.path.join(O, "lexicon.json"), TB_COMPOSITE=os.path.join(wk, "composite_text.caps.json"),
                             TB_D=str(t["D_diagnostics"]), TB_LOOPLOG=os.path.join(O, "looplog.json")),
               cwd=W, out=os.path.join(O, "looplog_tb.json"))
        if r.returncode:
            return dict(res, error="looplog", stderr=r.stderr[-2000:])
    if os.path.exists(os.path.join(O, "summary.json")) and os.path.getsize(os.path.join(O, "summary.json")):
        s = json.load(open(os.path.join(O, "summary.json")))
        res.update({m: {k: s[m][k] for k in ("admission_phase", "E_mov_plus_R", "fraction_of_M", "margin_rule", "fits_M", "TAV_W", "TAV_X")} for m in ("sparse", "dense")})
        res.update(text_complete=s["text_complete"], D_converged=s["D_converged"])
    return res


def inside_work_tree(path):
    """The first ancestor of path (itself included) holding a .git entry, or None."""
    p = os.path.realpath(path)
    while True:
        if os.path.lexists(os.path.join(p, ".git")):
            return p
        parent = os.path.dirname(p)
        if parent == p:
            return None
        p = parent


def claim_out(out):
    """Make or reuse the output directory: never inside a git work tree; an existing non-empty directory only when it
    carries this tool's marker. Only the tool's own entries (OWNED) are removed before a run."""
    wt = inside_work_tree(out)
    if wt:
        stop(2, "the output directory is inside a git work tree; choose one outside any checkout", out=out, work_tree=wt)
    if os.path.isdir(out) and os.listdir(out) and not os.path.exists(os.path.join(out, MARKER)):
        stop(2, f"the output directory is not empty and was not made by this tool (no {MARKER}); choose a new or empty one", out=out)
    os.makedirs(out, exist_ok=True)
    open(os.path.join(out, MARKER), "w").write("retained_pricing output directory; price.py run replaces its own entries here\n")
    for d in OWNED:
        q = os.path.join(out, d)
        if os.path.isdir(q) and not os.path.islink(q):
            shutil.rmtree(q)
        elif os.path.lexists(q):
            os.remove(q)


def cmd_run(a):
    repo = a.repo or default_repo()
    out = os.path.abspath(a.out or os.environ.get("RP_OUT") or stop(2, "no output directory: --out or RP_OUT"))
    rev = git(repo, "rev-parse", "--verify", a.rev + "^{commit}", text=True).strip()
    short = rev[:10]
    try:
        git(repo, "cat-file", "-e", BASIS + "^{commit}")
    except subprocess.CalledProcessError:
        stop(2, f"the basis {BASIS} is not in {repo}: a full clone (not shallow) is needed")
    claim_out(out)
    os.makedirs(os.path.join(out, "logs"))
    report = {"rev": rev, "basis": BASIS}
    # 1. the snapshot
    W = snapshot(repo, rev, os.path.join(out, "snap"))
    # 2. the base chain's line-keyed rules, basis -> rev (stops on any key it cannot carry)
    try:
        prepared, retirement_report = prepare_b7a_retired_carry(W, BASE, os.path.join(out, "prepared_rules", "retired"))
    except ValueError as error:
        stop(4, str(error))
    report["b7a_retired_carry"] = retirement_report
    # Exact historical/missing-source delegation still enters original strict
    # carry. Actual current B7a source carries the positive widened premises.
    if retirement_report is not None:
        try:
            widened, domain_report = prepare_b7a_domain(W, BASE, os.path.join(out, "prepared_rules", "widened"), prepared)
        except ValueError as error:
            stop(4, str(error))
        prepared.update(widened)
        report["b7a_positive_domain"] = domain_report
        try:
            formed, formation_report = prepare_b7a_formation(W, prepared, os.path.join(out, "prepared_rules", "formed"))
        except ValueError as error:
            stop(4, str(error))
        prepared.update(formed)
        report["b7a_formed_ownership"] = formation_report
    m = os.path.join(out, "map")
    r = py([os.path.join(LIB, "linemap_rules.py"), "--prune-deleted", repo, BASIS, rev, m] + [prepared.get(f, os.path.join(BASE, f)) for f in RULE_FILES],
           env=chain_env(LINEMAP_CRATES=os.path.join(BASE, "crate_dirs.txt"), LINEMAP_ROWS=os.path.join(DATA, "basis_text_rows.json"),
                         LINEMAP_RESOLUTIONS=RESOLUTIONS),
           out=os.path.join(out, "logs", "linemap.json"))
    lm = json.load(open(os.path.join(m, "g7_linemap.out.json")))["summary"] if os.path.exists(os.path.join(m, "g7_linemap.out.json")) else {}
    report["linemap"] = {"remapped": lm.get("remapped"), "unmapped": lm.get("unmapped"), "resolutions_used": lm.get("resolutions_used"),
                         "pruned": [[p["ctx"], p["rule"]] for p in lm.get("pruned_deleted", [])]}
    if r.returncode:
        stop(4, "rule keys not carried from the basis (logs/linemap.json, map/g7_linemap.out.json); a 'fewer_definitions' key "
                "needs a reviewed entry in rules/linemap_resolutions.json", unmapped=lm.get("unmapped"), stderr=r.stderr[-2000:])
    # 3. the premise pins (the dead-branch warrant lines) read as reviewed
    bad = []
    pins = json.load(open(os.path.join(m, "premise_pins.json")))["pins"]
    for p in pins:
        f, ln = p["key"].rsplit(":", 1)
        L = open(os.path.join(W, f), encoding="utf-8").read().split("\n")
        got = L[int(ln) - 1].strip() if int(ln) <= len(L) else ""
        if got != p["text"]:
            bad.append({"key": p["key"], "expected": p["text"], "found": got})
    report["premise_pins"] = {"pins": len(pins), "bad": bad}
    if bad:
        stop(4, "a premise pin's line is not as reviewed", bad=bad)
    # 4. the base chain at rev: the base scripts, the carried rule tables, the regenerated template inventory
    chain = os.path.join(out, "chain")
    shutil.copytree(BASE, chain)
    for f in RULE_FILES:
        shutil.copy(os.path.join(m, f), os.path.join(chain, f))
    # W1R-FG (retained-pricing-rules.md §8.11): the off-feature files and manifest lines, guarded; the scanners blank
    # off-feature items and skip these files, as the registered build (no features) does not compile them.
    try:
        off_build = off_build_files(W)
    except ValueError as error:
        stop(4, str(error))
    report["off_build"] = sorted(off_build)
    template_inventory(W, chain, off_build)
    # 5. SQ2 + O-10 rules per route
    for route in ("L", "E"):
        r = py([os.path.join(LIB, "sq2_rules.py"), chain, os.path.join(out, "rc" + route), os.path.join(RULES, f"sq2_decisions_{route}.py")],
               env=chain_env(SQ2_SNAP=W, SQ2_TARGET=rev, SQ2_REPO=repo, SQ2_DEC_BASIS=BASIS, SQ2_LM=os.path.join(m, "g7_linemap.out.json"),
                             SQ2_RESOLUTIONS=RESOLUTIONS),
               out=os.path.join(out, "logs", f"sq2_rules_{route}.out"))
        if r.returncode:
            stop(4, f"route {route}: the SQ2 rules did not apply (logs/sq2_rules_{route}.err)", stderr=r.stderr[-3000:])
        open(os.path.join(out, "logs", f"sq2_rules_{route}.err"), "w").write(r.stderr)
    # 6. the points, one job each (optionally under RP_JOB_WRAPPER)
    wrap = shlex.split(os.environ.get("RP_JOB_WRAPPER", ""))
    report["points"] = {}
    for route, lc, z in POINTS:
        name = f"{route}_{lc}{z}"
        caps = json.dumps({"l": 128, "c": 3, "lc": lc, "z": z})
        cmd = wrap + [sys.executable, os.path.abspath(__file__), "_point", os.path.join(out, "rc" + route), os.path.join(out, "runs", name), caps, W]
        with open(os.path.join(out, "logs", name + ".log"), "w") as lg:
            rc = subprocess.run(cmd, stdout=lg, stderr=subprocess.STDOUT, env=chain_env()).returncode
        res_p = os.path.join(out, "runs", name, "point.json")
        res = json.load(open(res_p)) if os.path.exists(res_p) else {"error": f"no result (job rc {rc}; logs/{name}.log)"}
        report["points"][name] = res
        print(json.dumps({name: res}))
    # a point passes only when sens.py ran clean, all four TEXT variants are complete (text_complete), the diagnostics
    # fixpoint converged, and no loop header is unmapped and no argument unclassified
    bad = {k: v for k, v in report["points"].items() if v.get("error") or v.get("sens_rc") or not v.get("complete") or not v.get("text_complete")
           or not v.get("D_converged") or v.get("unmapped") or v.get("unclassified")}
    # 7. the O-10 checks: each appended loop rule matched only its functions; the mult-0 sites stayed at 0
    report["o10_check"] = {}
    for route in ("L", "E"):
        runs = [os.path.join(out, "runs", f"{r}_{lc}{z}") for r, lc, z in POINTS if r == route]
        if any(os.path.basename(x) in bad for x in runs):
            report["o10_check"][route] = "not run (an incomplete point)"
            continue
        r = py([os.path.join(LIB, "o10_check.py"), os.path.join(out, "rc" + route), *runs], out=os.path.join(out, f"o10_check_{route}.json"))
        report["o10_check"][route] = r.returncode
    # 8. the merged tree (the maximum over route L's worst shape (3,0) and route E), only when every point passed and both
    # o10 checks held
    if not bad and all(v == 0 for v in report["o10_check"].values()):
        basis = a.basis or (f"SQ2 G5 at {short}: the maximum over routes L (C_eq <= 3: worst shape c = 3, z = 0) and E (c = 3), SQ2 rules; text at l <= 128")
        r = py([os.path.join(LIB, "merge_trees.py"), os.path.join(out, "runs", "L_30", "work", "profile_tree.json"),
                os.path.join(out, "runs", "E_30", "work", "profile_tree.json"), os.path.join(out, "tree_merged.json"), basis])
        report["merge"] = r.returncode if r.returncode else "tree_merged.json"
    json.dump(report, open(os.path.join(out, "summary.json"), "w"), indent=1)
    code = 5 if bad else (3 if any(v != 0 for v in report["o10_check"].values()) else (6 if report.get("merge") != "tree_merged.json" else 0))
    print(json.dumps({"rev": short, "incomplete": sorted(bad), "o10_check": report["o10_check"], "merge": report.get("merge"), "code": code}))
    sys.exit(code)


def cmd_point(a):
    res = point(a.chain, a.run_dir, a.caps, a.snap)
    json.dump(res, open(os.path.join(a.run_dir, "point.json"), "w"), indent=1)
    print(json.dumps(res))


# ---- generate ----------------------------------------------------------------------------------------------------
def block(src):
    i = src.index("// ---- BEGIN GENERATED PROFILE")
    j = src.index("// ---- END GENERATED PROFILE ----", i)
    return src[i:src.index("\n", j) + 1]


def cmd_generate(a):
    tree = json.load(open(a.tree))
    if a.basis is not None:
        tree["basis"] = a.basis
    with tempfile.TemporaryDirectory() as td:
        tp, rp = os.path.join(td, "profile_tree.json"), os.path.join(td, "retained_memory.rs")
        json.dump(tree, open(tp, "w"))
        shutil.copy(a.rs, rp)
        r = py([os.path.join(BASE, "g5_profile.py"), tp, rp])
        if r.returncode:
            stop(6, "the generator failed", stderr=r.stderr[-2000:])
        new, old = open(rp).read(), open(a.rs).read()
    if a.check:
        bn, bo = block(new), block(old)
        diff = list(difflib.unified_diff(bo.split("\n"), bn.split("\n"), "committed", "generated", lineterm="", n=0))
        print(json.dumps({"equal": bn == bo, "block_lines": bn.count("\n"), "diff": diff[:40]}, indent=1))
        sys.exit(0 if bn == bo else 6)
    open(a.rs, "w").write(new)
    print(r.stdout.strip())


# ---- compare / eval ----------------------------------------------------------------------------------------------
def cmd_compare(a):
    A, B = json.load(open(a.a)), json.load(open(a.b))
    higher, lower, same = [], [], 0
    def cmp(kind, name, x, y):
        nonlocal same
        if x > y: higher.append([kind, name, x, y])
        elif x < y: lower.append([kind, name, x, y])
        else: same += 1
    for n in sorted(set(A["forms"]) | set(B["forms"])):
        fa, fb = A["forms"].get(n, {}), B["forms"].get(n, {})
        for k in sorted(set(fa) | set(fb)):
            cmp("form", f"{n}[{k}]", fa.get(k, 0), fb.get(k, 0))
    for fam in ("text_atoms", "L"):
        for k in sorted(set(A[fam]) | set(B[fam])):
            cmp(fam, k, A[fam].get(k, 0), B[fam].get(k, 0))
    strip = lambda t: {k: v for k, v in t.items() if k != "basis"}
    out = {"identical": A == B, "identical_but_basis": strip(A) == strip(B),
           "structure_equal": all(A.get(k) == B.get(k) for k in ("exprs", "phases", "EPS", "M", "R", "which")),
           "a_le_b": not higher, "a_ge_b": not lower, "coefficients_equal": same, "higher_in_a": len(higher), "lower_in_a": len(lower),
           "check": {m: [A["check"][m]["max"], B["check"][m]["max"]] for m in ("sparse", "dense")},
           "higher": higher[:a.limit], "lower": lower[:a.limit]}
    print(json.dumps(out, indent=1))


def cmd_eval(a):
    r = subprocess.run([sys.executable, os.path.join(LIB, "tree_eval.py"), a.atoms, a.tree] + ([a.other] if a.other else []))
    sys.exit(r.returncode)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sp = ap.add_subparsers(dest="cmd", required=True)
    p = sp.add_parser("run", help="price a revision")
    p.add_argument("rev"); p.add_argument("--out"); p.add_argument("--repo")
    p.add_argument("--basis", help="the merged tree's basis text (default: 'SQ2 G5 at <rev>: ...')")
    p.set_defaults(f=cmd_run)
    p = sp.add_parser("_point"); p.add_argument("chain"); p.add_argument("run_dir"); p.add_argument("caps"); p.add_argument("snap"); p.set_defaults(f=cmd_point)
    p = sp.add_parser("generate", help="write the GENERATED PROFILE block from a tree")
    p.add_argument("tree"); p.add_argument("rs")
    p.add_argument("--basis", help="replace the tree's basis text; a text starting with ',' is wrapped after 'corrections,'")
    p.add_argument("--check", action="store_true", help="compare with the committed block instead of writing (exit 6 on a difference)")
    p.set_defaults(f=cmd_generate)
    p = sp.add_parser("compare", help="compare two trees coefficient by coefficient")
    p.add_argument("a"); p.add_argument("b"); p.add_argument("--limit", type=int, default=40); p.set_defaults(f=cmd_compare)
    p = sp.add_parser("eval", help="evaluate a tree at in-build atom values")
    p.add_argument("atoms"); p.add_argument("tree"); p.add_argument("other", nargs="?"); p.set_defaults(f=cmd_eval)
    a = ap.parse_args()
    a.f(a)


if __name__ == "__main__":
    main()
