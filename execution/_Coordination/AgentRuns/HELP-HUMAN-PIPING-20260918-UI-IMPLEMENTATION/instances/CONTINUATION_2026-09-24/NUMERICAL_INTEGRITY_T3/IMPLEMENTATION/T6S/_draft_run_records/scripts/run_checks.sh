#!/bin/bash
# I80: the citation dry runs and checks for T6S's package. Read-only Git (GIT_OPTIONAL_LOCKS=0).
# Usage: run_checks.sh <WT> <python>. Outputs go to T6S/_draft_run_records/outputs/, with WT replaced by "WT".
set -u
WT="$1"; PY="$2"
NUM="$WT/numerics"
IMP="$NUM/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION"
PKG="$IMP/T6S"; OUT="$PKG/_draft_run_records/outputs"; TOOL="$IMP/F2A_D1/check_citations.py"
BRIEF_MAIN=f8ed4f055126cf19315d2d8b06d7d11796786a82; MAIN=$(GIT_OPTIONAL_LOCKS=0 git -C "$NUM" rev-parse origin/main); BASE=c1bfc460fc8bedd2a0080e0aa3cfb74bf15df7df
HEAD=fdcdb5e024c09e87c4df7e37302112f21a837e57; NUMH=$(GIT_OPTIONAL_LOCKS=0 git -C "$NUM" rev-parse HEAD)
export GIT_OPTIONAL_LOCKS=0
mkdir -p "$OUT/negative_controls"
san() { sed -e "s#$WT#WT#g"; }
run() { # name base head [extra args]
  local name=$1 b=$2 h=$3; shift 3
  { echo "\$ check_citations.py --repo NUM --base $b --head $h --index T6S/citations.json --package T6S $*"
    echo "# S: $(git -C "$NUM" diff --name-only "$b" "$h" -- . ':!projects/chirality-piping/execution' ':!execution' | grep -c .) maintained paths in the two-dot diff"
    "$PY" "$TOOL" --repo "$NUM" --base "$b" --head "$h" --index "$PKG/citations.json" --package "$PKG" "$@"; echo "rc=$?"; } 2>&1 | san > "$OUT/$name.out"
  tail -2 "$OUT/$name.out" | head -1; tail -1 "$OUT/$name.out"
}
echo "NUM head $NUMH" | tee "$OUT/heads.txt"; echo "main (origin/main) $MAIN; main named in the brief $BRIEF_MAIN; slice base $BASE; slice head $HEAD" | tee -a "$OUT/heads.txt"
echo "== 1. the brief's form: --base f8ed4f0551 --head the slice head"; run check_briefmain_slicehead $BRIEF_MAIN $HEAD --list
echo "== 2. the slice's own diff: --base c1bfc460fc --head the slice head"; run check_slice $BASE $HEAD --list --out "$OUT/resolved.md"
echo "== 3. NUM with T6S merged (its maintained diff from current main is the 19 files): --base main --head NUM"; run check_main_num $MAIN $NUMH --list
sed -i '' -e "s#$WT#WT#g" "$OUT/resolved.md"
echo "== source-equality pre-check (git reads only)"
{ P=projects/chirality-piping
  echo "I80 source-equality pre-check (git reads only; not source_equality.py)"
  echo "NUM head $NUMH"; echo "main $MAIN"; echo "slice head $HEAD"
  echo "merge-base(NUM, main) $(git -C "$NUM" merge-base "$NUMH" "$MAIN")"
  git -C "$NUM" merge-base --is-ancestor "$MAIN" "$NUMH" && echo "main is an ancestor of NUM: yes" || echo "main is an ancestor of NUM: no"
  SN=$(git -C "$NUM" diff --name-only "$MAIN" "$NUMH" -- . ":!$P/execution" ":!execution"); SS=$(git -C "$NUM" diff --name-only "$BASE" "$HEAD")
  echo "S = diff --name-only main NUM (maintained): $(echo "$SN" | grep -c .) paths; slice paths $BASE..$HEAD: $(echo "$SS" | grep -c .)"
  [ "$SN" = "$SS" ] && echo "S equals the slice's paths: yes" || echo "S equals the slice's paths: NO"
  n=0; m=0
  for f in $SN; do
    [ "$(git -C "$NUM" ls-tree "$NUMH" -- "$f")" = "$(git -C "$NUM" ls-tree "$HEAD" -- "$f")" ] || { echo "DIFF $f"; n=$((n+1)); }
    [ "$(git -C "$NUM" rev-parse --verify -q "$MAIN:$f")" = "$(git -C "$NUM" rev-parse --verify -q "$BASE:$f")" ] || { echo "MAIN CHANGED $f"; m=$((m+1)); }
  done
  echo "S paths whose blob or mode differ between NUM and the slice head: $n"
  echo "S paths main changed since the slice base: $m"; } 2>&1 | san > "$OUT/source_equality_precheck.txt"; tail -4 "$OUT/source_equality_precheck.txt"
echo "== named_references"; { "$PY" "$PKG/_draft_run_records/scripts/verify_named_t6s.py" --repo "$NUM" --index "$PKG/citations.json" --main $MAIN; echo "rc=$?"; } 2>&1 | san > "$OUT/verify_named.out"; tail -2 "$OUT/verify_named.out"
echo "== negative controls"
"$PY" - "$TOOL" "$NUM" "$PKG/citations.json" "$OUT/negative_controls" "$MAIN" "$HEAD" "$PKG" <<'PYEOF' 2>&1 | san | tee "$OUT/negative_controls.out"
import copy, json, os, subprocess, sys
tool, repo, index, out, main, head, pkg = sys.argv[1:8]
base = json.load(open(index, encoding="utf-8"))
def drop_inputs(d): d["citations"] = [c for c in d["citations"] if c["token"] != "R/I76/t6s_01/inputs"]
def pin_before_rv101(d): d["num_commit"] = "f8ed4f055126cf19315d2d8b06d7d11796786a82"  # main before #1103 lacks RV101's evidence
def wrong_path(d): d["citations"][1]["num_paths"] = ["REVIEW_RV101/t6s_01/evidence/oracle/exp_difference.txt"]
controls = [("nc1_missing_entry", drop_inputs), ("nc2_pin_before_rv101_records", pin_before_rv101), ("nc3_wrong_num_path", wrong_path)]
ok = True
for name, alter in controls:
    d = copy.deepcopy(base); alter(d)
    p = os.path.join(out, name + ".json"); json.dump(d, open(p, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    r = subprocess.run([sys.executable, tool, "--repo", repo, "--base", "c1bfc460fc", "--head", head, "--index", p, "--package", pkg], capture_output=True, text=True)
    open(os.path.join(out, name + ".out"), "w", encoding="utf-8").write(r.stdout + r.stderr)
    last = [l for l in (r.stdout + r.stderr).splitlines() if l.startswith(("RESULT", "UNRESOLVED", "FAILED"))]
    good = r.returncode == 1
    ok &= good
    print(f"{name}: rc={r.returncode} (want 1) {'as expected' if good else 'UNEXPECTED'}; " + " | ".join(last))
    os.remove(p)
print("NEGATIVE CONTROLS", "ALL AS EXPECTED" if ok else "UNEXPECTED")
PYEOF
