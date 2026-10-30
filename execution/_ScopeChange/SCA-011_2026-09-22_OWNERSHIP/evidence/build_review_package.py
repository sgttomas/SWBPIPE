#!/usr/bin/env python3
"""Build review-only exact patches/manifests. Never writes canonical targets."""
import csv, difflib, hashlib, json, subprocess
from pathlib import Path
REPO = Path(__file__).resolve().parents[6]
RUN = Path(__file__).resolve().parents[1]
# Resolve repository by its known source root rather than depending on cwd.
while not (REPO / "AGENTS.md").exists():
    REPO = REPO.parent
    if REPO == REPO.parent: raise RuntimeError("repository root not found")
def sha(b): return hashlib.sha256(b).hexdigest()
def load(p): return json.loads((RUN/p).read_text())
meta=load("evidence/SOURCES_DECOMPOSITION.json")
promotions={x["target"]:x for x in meta["application_metadata"]["files"]}
extra=RUN/"interfaces/INTERFACE_PROMOTION.json"
if extra.exists():
    data=json.loads(extra.read_text())
    entries=data if isinstance(data,list) else data["files"]
    promotions.update({x["target"]:x for x in entries})
rows=[]; patches=[]; errors=[]
for candidate in sorted((RUN/"candidate").rglob("*")):
    if not candidate.is_file(): continue
    target=str(candidate.relative_to(RUN/"candidate"))
    before=(REPO/target).read_bytes() if (REPO/target).exists() else b""
    source=candidate.read_bytes(); applied=source
    m=promotions.get(target)
    if m:
        assert sha(source)==m["candidate_sha256"],target
        s=source.decode()
        for op in m["operations"]:
            assert op["operation"]=="replace_exact"
            assert s.count(op["old"])==op["expected_occurrences"],(target,op)
            s=s.replace(op["old"],op["new"])
        applied=s.encode(); assert sha(applied)==m["applied_sha256"],target
    rows.append([target,str(candidate.relative_to(REPO)),sha(before) if (REPO/target).exists() else "ABSENT",sha(source),sha(applied),"GROUP2_APPLICATION", "evidence/SOURCES_DECOMPOSITION.json#/application_metadata" if target in {x['target'] for x in meta['application_metadata']['files']} else ("interfaces/INTERFACE_PROMOTION.json" if m else "NONE")])
    patches.extend(difflib.unified_diff(before.decode().splitlines(True),applied.decode().splitlines(True),fromfile="a/"+target if (REPO/target).exists() else "/dev/null",tofile="b/"+target))
# Exact local dependency mirrors have separate owning writer but same group-2 authorization.
mirrors=load("dependencies/LOCAL_MIRRORS_MANIFEST.json")
local_promotion_path=RUN/"dependencies/LOCAL_MIRRORS_PROMOTION.json"
local_promotions={x["target"]:x for x in json.loads(local_promotion_path.read_text())["files"]} if local_promotion_path.exists() else {}
for m in mirrors["files"]:
    target="projects/chirality-piping/"+m["proposed_landing"]
    candidate=REPO/"projects/chirality-piping"/m["candidate"]
    before=(REPO/target).read_bytes() if (REPO/target).exists() else b""
    after=candidate.read_bytes()
    assert sha(after)==m["sha256"],target
    assert (sha(before) if (REPO/target).exists() else None)==m["baseline_sha256"],target
    applied=after
    pm=local_promotions.get(target)
    if pm:
        assert sha(after)==pm["candidate_sha256"],target
        body=after.decode()
        for op in pm["operations"]:
            assert body.count(op["old"])==op["expected_occurrences"],(target,op)
            body=body.replace(op["old"],op["new"])
        applied=body.encode();assert sha(applied)==pm["applied_sha256"],target
    rows.append([target,str(candidate.relative_to(REPO)),sha(before) if (REPO/target).exists() else "ABSENT",sha(after),sha(applied),"GROUP2_DEPENDENCY_TASK","dependencies/LOCAL_MIRRORS_PROMOTION.json" if pm else "NONE"])
    patches.extend(difflib.unified_diff(before.decode().splitlines(True),applied.decode().splitlines(True),fromfile="a/"+target if (REPO/target).exists() else "/dev/null",tofile="b/"+target))
assert len({x[0] for x in rows})==len(rows),"duplicate canonical target"
with (RUN/"APPLY_MANIFEST.csv").open("w",newline="") as f:
    w=csv.writer(f);w.writerow(["CanonicalTarget","CandidatePath","BeforeSHA256","CandidateSHA256","AppliedSHA256","ApplicationLane","TransformationReference"]);w.writerows(rows)
(RUN/"AMENDMENT.patch").write_text("".join(patches))
# New dependency snapshot stays a distinct projection until the same group-3 adoption.
graph=[]
for name in ["DependencyEdges.csv","DeliverableNodes.csv"]:
    p=RUN/"dependencies"/name
    if p.exists(): graph.append({"candidate":str(p.relative_to(REPO)),"sha256":sha(p.read_bytes()),"proposed_target":"projects/chirality-piping/execution/_DAG/DAG-011/"+name,"application":"GROUP2_STAGE_GROUP3_ADOPT"})
(RUN/"DEPENDENCY_SNAPSHOT_MANIFEST.json").write_text(json.dumps({"status":"EXACT_CANDIDATE_NOT_ADOPTED","files":graph,"current_pointer":"DAG-010; change only on group-3 accepted actual poststate","approval_record":"Prepared by graph owner with actual group-3 decision and final artifact manifest; no fabricated decision bytes."},indent=2)+"\n")
changed=subprocess.check_output(["git","diff","--name-only"],cwd=REPO,text=True).splitlines()
allowed="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/WORK_GRAPH.json"
assert all(x==allowed for x in changed),changed
checks={"status":"PASS","canonical_tracked_changes":changed,"canonical_amendment_applied":False,"candidate_files":len(rows),"new_canonical_paths":sum(x[2]=="ABSENT" for x in rows),"modified_canonical_paths":sum(x[2]!="ABSENT" for x in rows),"unique_canonical_targets":True,"preimage_and_transformation_hashes":"PASS","manifest_sha256":sha((RUN/"APPLY_MANIFEST.csv").read_bytes()),"patch_sha256":sha((RUN/"AMENDMENT.patch").read_bytes()),"schema_drafts_excluded_from_application":True,"limits":["Application preview only; exact transforms assert no human acceptance before its actual act.","Graph acceptance and final snapshot pointer remain group-3 acts; new contract maturity remains pending.","Product/native/engineering verification not run or claimed."]}
(RUN/"evidence/INTEGRATION_VALIDATION.json").write_text(json.dumps(checks,indent=2)+"\n")
print(json.dumps(checks,indent=2))
