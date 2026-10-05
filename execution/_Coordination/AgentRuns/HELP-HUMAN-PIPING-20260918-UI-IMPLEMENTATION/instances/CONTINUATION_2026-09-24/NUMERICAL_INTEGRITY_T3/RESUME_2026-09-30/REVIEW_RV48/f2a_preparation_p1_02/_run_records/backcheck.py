# RV48-02: immutable repair-delta/source and exact scalar checks only.
import contextlib
import hashlib
import io
import json
import math
import os
from pathlib import Path
import subprocess

ROOT=Path.cwd()
OUT=Path(__file__).resolve().parent
R="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/"
PACKET=R+"I29/f2a_preparation_counts_p1/"
CAND="b1ebe245ecde6a5e27ca9f9bd94a7020e5959422"
ORIGINAL="8eaca35bdc1eedc46e8c3b8fb612f1b304399d4a"
REVIEW="44a6f7d9cfef817d9c66913b19bd76d2c90cfc54"
ENV=dict(os.environ,GIT_OPTIONAL_LOCKS="0")
def git(*args):return subprocess.check_output(["git",*args],cwd=ROOT,env=ENV)
def blob(rev,path):return git("show",rev+":"+path)
def digest(data):return hashlib.sha256(data).hexdigest()
checks=[]
def check(name,condition,evidence):
    assert condition,(name,evidence)
    checks.append({"check":name,"passed":True,"evidence":evidence})
parent=git("rev-parse",CAND+"^").decode().strip()
actual_paths=git("diff-tree","--no-commit-id","--name-only","-r",CAND).decode().splitlines()
expected_paths=[PACKET+p for p in ["COUNTS_AND_OWNERSHIP.md","RETURN.md","WRITE_INVENTORY.json","_run_records/correction_02/CORRECTION.json","_run_records/correction_02/COUNT_CHECKS.json","_run_records/correction_02/count_checks.py"]]
check("actual repair commit changes exactly its six authorized packet paths",set(actual_paths)==set(expected_paths),{"parent":parent,"paths":actual_paths})
original_paths=git("ls-tree","-r","--name-only",ORIGINAL,"--",PACKET).decode().splitlines()
check("repair parent contains original seven packet files unchanged",len(original_paths)==7 and all(blob(parent,p)==blob(ORIGINAL,p) for p in original_paths),{"original_file_count":len(original_paths)})
inv=json.loads(blob(CAND,PACKET+"WRITE_INVENTORY.json"))
for item in inv["files"]:
    data=blob(CAND,PACKET+item["path"])
    check("candidate inventory: "+item["path"],len(data)==item["bytes"] and digest(data)==item["sha256"],{"bytes":len(data),"sha256":digest(data)})
check("candidate has exactly ten packet files",len(git("ls-tree","-r","--name-only",CAND,"--",PACKET).decode().splitlines())==10,{"inventory_payloads":len(inv["files"])})
for name in ["count_checks.py","COUNT_CHECKS.json","ORIGINS.json","SESSION.json"]:
    path=PACKET+"_run_records/"+name
    check("preserved original raw: "+name,blob(CAND,path)==blob(ORIGINAL,path),{"sha256":digest(blob(CAND,path))})
review_path=R+"REVIEW_RV48/f2a_preparation_p1_01/"
review_files=git("ls-tree","-r","--name-only",REVIEW,"--",review_path).decode().splitlines()
check("all seven original review files preserved",len(review_files)==7 and all(blob(CAND,p)==blob(REVIEW,p) for p in review_files),{"review_revision":REVIEW,"file_count":len(review_files)})
# Preserve the original immutable 16-control reference exactly where the inspected
# repair script expects it. Only the correction output is generated below it.
replay=OUT/"replay"
(replay/"correction_02").mkdir(parents=True,exist_ok=True)
original_reference=blob(ORIGINAL,PACKET+"_run_records/COUNT_CHECKS.json")
(replay/"COUNT_CHECKS.json").write_bytes(original_reference)
code=blob(CAND,PACKET+"_run_records/correction_02/count_checks.py")
namespace={"__file__":str(replay/"correction_02/count_checks.py"),"__name__":"__main__"}
stdout=io.StringIO()
with contextlib.redirect_stdout(stdout):exec(compile(code,"immutable-P1-correction-checker","exec"),namespace)
produced=(replay/"correction_02/COUNT_CHECKS.json").read_bytes()
frozen=blob(CAND,PACKET+"_run_records/correction_02/COUNT_CHECKS.json")
result=json.loads(produced)
old=json.loads(original_reference)
check("51-check repair output reproduces byte-for-byte",produced==frozen and len(result["checks"])==51 and all(x["passed"] for x in result["checks"]),{"stdout":stdout.getvalue().strip(),"sha256":digest(produced),"checker_sha256":digest(code)})
check("original 16 control records and reference bytes preserved",result["checks"][:16]==old["checks"] and (replay/"COUNT_CHECKS.json").read_bytes()==original_reference,{"original_controls":16,"reference_sha256":digest(original_reference)})
# Independent malformed-input probes, all bounded scalar/list examples.
rejections=[]
def rejected(label,fn):
    try:fn()
    except ValueError:rejections.append(label)
    else:raise AssertionError("malformed input accepted: "+label)
for at in range(8):
    for bad in [True,-1,0.5]:
        args=[2,1,1,6,3,1,12,1];args[at]=bad
        rejected("validated[%d]=%r"%(at,bad),lambda args=args:namespace["validated"](*args))
for at in range(6):
    for bad in [True,-1,0.5]:
        args=[2,1,1,6,3,1,[4,4,4],[1]];args[at]=bad
        rejected("raw[%d]=%r"%(at,bad),lambda args=args:namespace["raw"](*args))
for bad in [True,-1,0.5]:
    rejected("raw id element %r"%bad,lambda bad=bad:namespace["raw"](1,0,0,0,1,0,[bad],[]))
    rejected("raw child element %r"%bad,lambda bad=bad:namespace["raw"](1,0,0,0,0,1,[],[bad]))
    rejected("combo k %r"%bad,lambda bad=bad:namespace["combo"]([38,38],[0,0],bad))
    rejected("combo encoding element %r"%bad,lambda bad=bad:namespace["combo"]([bad,39],[0,0],0))
    rejected("combo load element %r"%bad,lambda bad=bad:namespace["combo"]([38,38],[bad,1],0))
for at in range(3):
    for bad in [True,-1,0.5]:
        args=[8,8,2,64];args[at]=bad
        rejected("layout[%d]=%r"%(at,bad),lambda args=args:namespace["layout_count"](*args))
for bad in [True,-1,0.5,0]:
    for name,args in [("raw",[2,1,1,6,3,1,[4,4,4],[1]]),("validated",[2,1,1,6,3,1,12,1]),("combo",[[38],[1],1])]:
        rejected(name+" width "+repr(bad),lambda name=name,args=args,bad=bad:namespace[name](*args,bits=bad))
    rejected("checked width "+repr(bad),lambda bad=bad:namespace["checked"](0,bad))
    rejected("layout width "+repr(bad),lambda bad=bad:namespace["layout_count"](8,8,2,bad))
check("independent per-input malformed-count rejection",len(rejections)==86,{"probe_count":len(rejections),"probes":rejections})
U32,U64=(1<<32)-1,(1<<64)-1
max_product_F=(math.isqrt(1+4*U64)-1)//2
F=U32+1
check("sentinel clarification remains conditional and exact",max_product_F==U32 and F*(F+1)//2<=U64<F*(F+1),{"max_F_with_unreduced_product":max_product_F,"reduced_only_example_F":F,"final_count_U32_last_id":U32-1,"sentinel":U32})
report={"scope":"Exact actual repair commit only; immutable source trace plus finite integer checks. No model, native source, compiler, solver, runtime or host-profile probe.","candidate":CAND,"parent":parent,"original":ORIGINAL,"review":REVIEW,"checks":checks,"all_passed":True}
(OUT/"BACKCHECK_CHECKS.json").write_text(json.dumps(report,indent=2)+"\n")
print(json.dumps({"checks":len(checks),"all_passed":True,"reproduced_author_checks":51,"preserved_original_controls":16,"independent_malformed_probes":len(rejections)}))
