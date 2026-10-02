from pathlib import Path
import hashlib,json,subprocess,os,shlex,datetime,re
from decimal import Decimal
WT=Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3")
A1=WT/"a1"
REL=Path("projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30")
R=A1/REL
OUT=R/"source_review_RV31/vk_final_01"
CAND="cf7841ced3427bd3692ea39343c6936d3234ef5b"
env=dict(os.environ,GIT_OPTIONAL_LOCKS="0")
def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def bh(b): return hashlib.sha256(b).hexdigest()
def j(p): return json.loads(Path(p).read_text(),parse_float=Decimal)
def git(*args): return subprocess.check_output(["git","-C",str(A1),*args],env=env)
def write(name,v): (OUT/name).write_text(json.dumps(v,indent=2,default=str)+"\n")
def bind(p):
    p=Path(p); rel=p.relative_to(A1).as_posix()
    assert git("show",CAND+":"+rel)==p.read_bytes(),rel
    return sha(p)
def resolve(s): return Path(s.replace("<wt>",str(WT)))
results={"started_utc":"2026-10-02T00:27:09Z","reviewer":"RV31","parent":"ROOT HELP_HUMAN Agent0","mechanism":"delegated-harness-native descendant /root/rv31_a1_final","candidate":CAND,"observed_a1_head":git("rev-parse","HEAD").decode().strip(),"role":"TASK Type2","fresh_identity_not_RV29":True}
results["head_movement_policy"]="Pinned cf7841 candidate governs this evidence review. Observed later HEAD is disclosed and is not implicitly reviewed."
instructions=[]
current=Path("/Users/ryan/.codex/worktrees/92ea/chirality")
for rel in ["AGENTS.md","agents/AGENT_TASK.md","projects/chirality-piping/AGENTS.md",".agents/skills/software-code-review/SKILL.md"]:
    assert (current/rel).read_bytes()==(A1/rel).read_bytes()
    instructions.append({"loaded_origin":str(current/rel),"sha256":sha(current/rel),"candidate_origin":str(A1/rel),"candidate_sha256":bind(A1/rel)})
results["instructions"]=instructions
results["permission_boundary"]="Only additive RV31 packet and its scratch writes; no delegation, runtime/compiler/probe, maintained edit, Git/index mutation, or broad proof restart. Host permits more than brief."
results["unsealed_RV29_vk_final44"]="Preserved untouched and not read or credited."
packets={}
for name,expected,count in [
("I22/vk_runtime_10","4224b381b6482f0f074e49d851a2382549c6008275c84b51119dbba2a5ff292a",48),
("manager/vk_runtime_10","e2287438c6499d7fef170e7fc390e4e3c2ec01165ff96a15170600b7fea39d68",5),
("source_review_RV29/vk_r02_plan_42","a84a27223c26a587ff1c9cdc047be12ab28dc70c4d2f55e28cd3d2b52922c37c",None),
("source_review_RV29/vk_s2_mapping_40",None,None),
("I23/vk_prep_05",None,None)]:
    root=R/name; seal=root/"SHA256SUMS"; actual=sha(seal)
    if expected: assert actual==expected,name
    bind(seal); payloads=[]
    for line in seal.read_text().splitlines():
        h,path=line.split(maxsplit=1); p=root/path.lstrip("*")
        assert sha(p)==h,str(p)
        bind(p); payloads.append({"path":path,"sha256":h})
    if count: assert len(payloads)==count,(name,len(payloads))
    packets[name]={"seal":actual,"payload_count":len(payloads),"all_hashes_and_candidate_bytes_match":True,"payloads":payloads}
results["packets"]=packets
write("PROVENANCE.json",results)
print(json.dumps({"packet_counts":{k:v["payload_count"] for k,v in packets.items()},"all_candidate_bound":True}))

# Bind closed source proofs by their sealed candidate bytes; rehash physical runtime bases.
manifest=j(R/"I23/vk_prep_05/SOURCE_MANIFEST.json")
vrbase=WT/"scratch/i23/vk_prep/archive"
fkbase=WT/"scratch/i22/mutations_e1/base"
files=manifest["files"]; assert len(files)==189
source_checks=[]
for kind,base,rows,scopes in [
("VR189",vrbase,files,manifest["scopes"]),
("FK116",fkbase,[x for x in files if "/core/solver/frame_kernel/" in x["path"]],["projects/chirality-piping/core/solver/frame_kernel"])]:
    actual=set()
    for scope in scopes:
        actual.update(str(p.relative_to(base)) for p in (base/scope).rglob("*") if p.is_file())
    expected={x["path"] for x in rows}
    assert actual==expected,(kind,actual-expected,expected-actual)
    checked=[]
    for row in rows:
        p=base/row["path"]
        assert sha(p)==row["sha256"],(kind,row["path"])
        assert p.stat().st_size==row["size"]
        physical_mode=format(p.stat().st_mode & 0o7777,"04o")
        if kind=="VR189": assert physical_mode==row["mode"],(kind,row["path"],physical_mode)
        else: assert physical_mode=="0444",(kind,row["path"],physical_mode)
        checked.append({"path":row["path"],"sha256":row["sha256"],"manifest_mode":row["mode"],"physical_mode":physical_mode})
    if kind=="FK116": assert len(rows)==116 and all(x["same_as40129"] for x in rows)
    source_checks.append({"basis":kind,"physical_root":str(base),"exact_file_set":True,"hashes_match":True,"mode_check":"VR manifest mode0664; FK immutable E1 mode0444", "files":checked})
proof=j(R/"I23/vk_prep_05/SOURCE_DIFFERENCE_PROOF.json")
assert proof["baseline_source"]=="40129a225d73860ac2a53da9a2fa73869df668f3"
assert proof["runtime_archive_source"]=="942572ede4c15aca87b4660c7d570343e14e679e"
assert proof["unchanged_files"]==178 and len(proof["exact_changed_paths"])==11
for x in proof["exact_changed_paths"]: assert "observations/kernel_lane/" in x["path"]
# Closed review citations remain bound to these same physical sources.
citation_counts={}
for p,key in [(R/"source_review_RV29/vk_r02_plan_42/_run_records/SOURCE_ARTIFACT_CHECKS.json","source_hashes"),(R/"source_review_RV29/vk_s2_mapping_40/_run_records/SOURCE_AUDIT_CHECKS.json","source_citations")]:
    rows=j(p)[key]
    for row in rows: assert sha(vrbase/row["path"])==row["sha256"],row["path"]
    citation_counts[str(p.relative_to(R))]=len(rows)
pre=j(R/"manager/vk_runtime_10/PRELAUNCH.json")
assert sha(R/"I23/vk_prep_05/COMMANDS_AND_PROJECTIONS.json")==pre["schedule_sha256"]
artifacts=[]
for b in pre["VR_bindings"]+[pre["scalar_artifact"]]:
    assert sha(resolve(b["path"]))==b["sha256"]
    artifacts.append({"path":b["path"],"sha256":b["sha256"]})
    if "fingerprint_path" in b:
        assert sha(resolve(b["fingerprint_path"]))==b["fingerprint_sha256"]
        fp=j(resolve(b["fingerprint_path"]))
        assert json.loads(fp["features"])==b["cargo_features"]
        assert fp["rustflags"]==[]
        artifacts.append({"path":b["fingerprint_path"],"sha256":b["fingerprint_sha256"],"features":fp["features"],"rustflags":fp["rustflags"]})
    elif "features" in b:
        fp=j(resolve(b["path"])); assert fp["features"]==b["features"] and fp["rustflags"]==[]
        artifacts[-1].update(features=fp["features"],rustflags=fp["rustflags"])
assert sha(resolve(pre["scalar_fingerprint_path"]))==pre["scalar_fingerprint_sha256"]
sfp=j(resolve(pre["scalar_fingerprint_path"]))
assert json.loads(sfp["features"])==[] and sfp["rustflags"]==[]
artifacts.append({"path":pre["scalar_fingerprint_path"],"sha256":pre["scalar_fingerprint_sha256"],"features":sfp["features"],"rustflags":sfp["rustflags"]})
write("SOURCE_BINDING.json",{"checks":source_checks,"closed_40129_942572_proof":str((R/"I23/vk_prep_05/SOURCE_DIFFERENCE_PROOF.json").relative_to(A1)),"proof_sha256":sha(R/"I23/vk_prep_05/SOURCE_DIFFERENCE_PROOF.json"),"source_citation_hash_checks":citation_counts,"artifacts":artifacts,"raw_fingerprints_preserved_without_float_reserialization":True})
print("PASS physical VR189/FK116 exact inventory/hash/mode; source review citations; distinct artifacts/raw fingerprints/dependencies.")

# Independently inspect actual commands and raw byte streams, never launch any.
runtime=R/"I22/vk_runtime_10"
plans={x["process_id"]:x for x in j(R/"I23/vk_prep_05/COMMANDS_AND_PROJECTIONS.json")["processes"]}
scalarplans={x["process_id"]:x for x in pre["scalar_commands"]}
order=["P51","P52","P53","P48","P49","P50","S01","S02","S03"]
faults=["NONE","VK-R28","NONE","NONE","VK-R02","NONE","NONE","VK-R02","NONE"]
calls=[]; records={}; parities={}; reports={}
obsdir=vrbase/"projects/chirality-piping/validation/benchmarks/numerical_robustness/observations/kernel_lane"
frozen={x["id"]:x for x in j(obsdir/"rf_large.json")}
frozenparity=[x for x in j(obsdir/"parity.json") if x["case"].startswith("RF-LARGE-")]
assert len(frozen)==12 and len(frozenparity)==12
decoder=json.JSONDecoder(parse_float=Decimal)
cutoff=datetime.datetime.fromisoformat("2026-10-02T00:09:52+00:00")
last=None
for pid,fault in zip(order,faults):
    e=j(runtime/pid/"EVIDENCE.json")
    p=plans[pid] if pid.startswith("P") else scalarplans[pid]
    expectedargv=list(p["argv"])
    if pid.startswith("P"): expectedargv[0]=pre["VR_bindings"][0]["path"]
    assert e["argv"]==expectedargv,pid
    for k in ["cwd","stdout_path","stderr_path","environment"]:
        assert e[k]==p[k],(pid,k)
    if pid.startswith("P"):
        for k in ["actual_scope","qualifying_condition","record_projections"]:
            assert e[k]==p[k],(pid,k)
        assert e["fresh_process"] is True
        assert e["binary"]==pre["VR_bindings"][0]
    else: assert e["binary"]==pre["scalar_artifact"]
    assert e["environment"]["FK_SEEDED_FAULT"]==fault
    tokens=shlex.split(e["cmd"])
    assert tokens[:3]==["/usr/bin/time","-l","env"]
    i=3; unsets=[]; envactual={}
    while tokens[i]=="-u": unsets.append(tokens[i+1]); i+=2
    while "=" in tokens[i] and not tokens[i].startswith("<wt>"):
        k,v=tokens[i].split("=",1); envactual[k]=v; i+=1
    assert set(unsets)=={"RUSTFLAGS","CARGO_ENCODED_RUSTFLAGS","RUSTC_WRAPPER","RUSTC_WORKSPACE_WRAPPER"}
    expectedenv=dict(p["environment"],GIT_OPTIONAL_LOCKS="0",CARGO_NET_OFFLINE="true")
    assert envactual==expectedenv,(pid,envactual)
    assert tokens[i:i+len(expectedargv)]==expectedargv
    assert tokens[i+len(expectedargv):]==[">",e["stdout_path"],"2>",e["stderr_path"]]
    when=datetime.datetime.fromisoformat(e["launch_utc"].replace("Z","+00:00"))
    assert when<cutoff and (last is None or when>last)
    last=when
    exitcode=e.get("tool_completion",e["tool_initial"])["exit_code"]
    assert exitcode==(101 if pid=="S02" else 0)
    streams=[]
    for stream in ["stdout","stderr"]:
        key=stream+"_path"; original=resolve(e[key]); portable=runtime/pid/(stream+".txt")
        assert sha(original)==e["raw_hashes"][key]==sha(portable),(pid,stream)
        assert original.read_bytes()==portable.read_bytes()
        streams.append({"kind":stream,"original":e[key],"portable":str(portable.relative_to(A1)),"sha256":sha(original)})
    out=(runtime/pid/"stdout.txt").read_text()
    err=(runtime/pid/"stderr.txt").read_text()
    real=re.search(r"([0-9.]+) real",err)
    assert real and Decimal(real[1])<300
    assert "5387" in e["guard_before"]
    call={"id":pid,"fault":fault,"launch_utc":e["launch_utc"],"exit":exitcode,"argv":e["argv"],"cwd":e["cwd"],"environment_and_unsets_match":True,"streams":streams,"time_real_seconds":real[1],"guard_snapshot_pid":5387}
    if pid.startswith("P"):
        rs=[]; remainder=out.lstrip()
        while remainder.startswith("{"):
            record,n=decoder.raw_decode(remainder); rs.append(record); remainder=remainder[n:].lstrip()
        assert rs==j(runtime/pid/"RECORDS.json"),pid
        assert all(x["case_limit"]==18446744073709551615 for x in rs)
        records[pid]=rs
        ps=[json.loads(x[len("RF-LARGE: parity "):],parse_float=Decimal) for x in remainder.splitlines() if x.startswith("RF-LARGE: parity ")]
        assert len(ps)==12 and ps==frozenparity,(pid,"parity")
        if (runtime/pid/"PARITY.json").exists(): assert ps==j(runtime/pid/"PARITY.json")
        parities[pid]=ps
        report_expected="\n".join(x for x in remainder.splitlines() if fault!="NONE" or not x.startswith("RF-LARGE: parity "))
        assert report_expected.strip()==(runtime/pid/"REPORT.txt").read_text().strip()
        reports[pid]=remainder
        assert [x["id"] for x in rs]==e["actual_scope"]["shown_case_ids_only"]
        assert set(x["case"] for x in ps)==set(e["actual_scope"]["case_ids_computed"])
        assert len([x for x in err.splitlines() if " parity " in x])==12
        assert e["actual_scope"]["max_members"]==100 and e["actual_scope"]["show_filters_computation"] is False
        if fault=="NONE":
            assert rs==[frozen[x["id"]] for x in rs],(pid,"complete records")
            assert "rows 9054: passes 9054, absolute-range passes 0, not covered 0 | structural zeros 0, expected unresolved 0, failures 0" in remainder
            assert 'outcomes {"selected Some(128)": 12}' in remainder
            assert "controls discriminated 38, non-discriminating 0, undiscriminated [], unexpectedly failing []" in remainder
            assert "FAILURE" not in remainder
            for row in rs:
                assert [(a["precision"],a["role"],a["outcome"]) for a in row["attempts"]]==[(128,"Candidate","Accepted"),(256,"Verification","Verified")]
                assert row["attempts"][1]["verification"]["data_blocks"]==1 and row["attempts"][1]["verification"]["shift_factorizations"]==1
            call["complete_shown_records_and_full_family_controls_restore"]=True
        call["all12_parity_records_equal_frozen"]=True
        call["original_sensitive_parity_cases"]=[x["case"] for x in ps if x["both_passed"] is False]
        call["full12case_scope_and12dense_parity_retained"]=True
    else:
        assert "running 1 test" in out and "--nocapture" not in e["argv"]
        name=e["argv"][2]
        if fault=="NONE":
            assert "test "+name+" ... ok" in out
            assert "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 390 filtered out" in out
            assert "panicked at" not in out and "panicked at" not in err
            call["all_original_assertions_pass"]=True
        else:
            assert "test "+name+" ... FAILED" in out
            assert "verify_tests.rs:45:5:" in out and "left: [1.0, 8.0]" in out and "right: [4.0, 8.0]" in out
            assert "test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 390 filtered out" in out
            assert "panicked at" not in err
            assert "verify_tests.rs:46" not in out
            call["named_line45_failure_captured_in_stdout"]=True
            call["later_assertions"]="Unexecuted and uncredited"
            call["complementary_only_no_P49_substitution"]=True
    calls.append(call)
assert (runtime/"P51/stdout.txt").read_bytes()==(runtime/"P53/stdout.txt").read_bytes()
assert (runtime/"P48/stdout.txt").read_bytes()==(runtime/"P50/stdout.txt").read_bytes()
assert (runtime/"S01/stdout.txt").read_bytes()==(runtime/"S03/stdout.txt").read_bytes()
r28=[]
expectedr28=[
[(128,"Candidate","Rejected(Uc { body: 0 })"),(256,"VerificationThenCandidate","Rejected(Charge { quantity: EndAction { member: 1, end: I, component: Ux }, body: 0, kind: Force })"),(512,"VerificationThenCandidate","Accepted"),(1024,"Verification","Verified")],
[(128,"Candidate","Rejected(Charge { quantity: EndAction { member: 2, end: I, component: Ux }, body: 0, kind: Force })"),(256,"VerificationThenCandidate","Accepted"),(512,"Verification","Verified")]]
for row,seq in zip(records["P52"],expectedr28):
    actual=[(a["precision"],a["role"],a["outcome"]) for a in row["attempts"]]
    assert actual==seq
    a256=[a for a in row["attempts"] if a["precision"]==256][0]
    assert a256["verification"] is not None
    assert a256["verification"]["data_blocks"]==1 and a256["verification"]["shift_factorizations"]==0
    r28.append({"case":row["id"],"sequence":actual,"actual_nonnull_p256_report":a256["verification"],"baseline_nonnull_p256_report":frozen[row["id"]]["attempts"][1]["verification"]})
assert "rows 9054: passes 9054" in reports["P52"]
assert 'outcomes {"selected Some(128)": 7, "selected Some(256)": 2, "selected Some(512)": 3}' in reports["P52"]
named="Rejected(VerificationEstimate { quantity: EndAction { member: 1, end: I, component: Ry }, body: 0, kind: Moment })"
r02=records["P49"][0]
assert r02["id"]=="RF-LARGE-TREE-n00100-AX"
assert [(a["precision"],a["role"],a["outcome"]) for a in r02["attempts"]]==[(128,"Candidate",named),(256,"VerificationThenCandidate",named),(512,"VerificationThenCandidate",named),(1024,"Verification","Solved")]
assert r02["outcome"]=="Unresolved Ceiling [Restrained]" and r02.get("selected_precision") is None
failures=[x for x in reports["P49"].splitlines() if x.startswith("RF-LARGE: FAILURE ")]
assert len(failures)==4
assert 'outcomes {"selected Some(128)": 7, "selected Some(512)": 1, "unresolved None": 4}' in reports["P49"]
assert "rows 9054: passes 4976" in reports["P49"] and "failures 4078" in reports["P49"]
end=j(R/"manager/vk_runtime_10/FINAL_CHECK.json")["verified_utc"]
assert datetime.datetime.fromisoformat(end)<datetime.datetime.fromisoformat("2026-10-02T00:12:52+00:00")
write("OBSERVATIONS.json",{"actual_order":order,"calls":calls,"R28":r28,"R02":{"case":r02["id"],"named_rejections":[{"precision":a["precision"],"role":a["role"],"outcome":a["outcome"]} for a in r02["attempts"]],"outcome":r02["outcome"],"all_failure_lines":failures,"other3Ceilings":"Retained without independent fault credit or invented causes","no_unprinted_numeric_ratio_work_or_certificate_inference":True},"scalar":{"exact_line":45,"input":[1,8],"extent":2,"expected":[4,8],"observed":[1,8],"stream":"stdout","all_later_assertions_unexecuted_uncredited_in_fault":True,"NONE_controls":"Both exactly one test pass, all original assertions","not_P49_substitution":True},"timing":{"all_launches_before_cutoff":True,"cutoff":"2026-10-02T00:09:52Z","manager_seal_observation":end,"manager_before_hard_end":True,"hard_end":"2026-10-02T00:12:52Z"},"exact_integer_and_decimal_parse":True})
print("PASS all nine commands/env/streams/timing; normal complete records and family/parity restoration; exact R28/R02/scalar witnesses.")
