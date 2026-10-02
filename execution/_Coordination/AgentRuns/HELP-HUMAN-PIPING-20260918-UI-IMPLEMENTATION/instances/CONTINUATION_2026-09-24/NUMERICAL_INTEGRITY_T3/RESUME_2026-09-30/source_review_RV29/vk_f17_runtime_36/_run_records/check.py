from pathlib import Path
import json,tarfile,hashlib,re,collections,datetime,subprocess,os,stat,shlex
W=Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3"); A=W/"a1"
R=Path("projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30")
D=A/R/"source_review_RV29/vk_f17_runtime_36/_run_records";T=tarfile.open(D/"committed_packets.tar");head="0158df21a13de7df138f0f7274339ab0a71dac81";sha=lambda b:hashlib.sha256(b).hexdigest()
read=lambda f:T.extractfile(str(R/f)).read()
j=lambda f:json.loads(read(f))
def git(*args):return subprocess.check_output(["git","-C",str(A),*args],env=dict(os.environ,GIT_OPTIONAL_LOCKS="0"))
def blob(p):return git("show",head+":"+str(p))
def path(s):return Path(s.replace("<wt>",str(W)))
B=W/"scratch/i23/vk_prep/archive";vr="projects/chirality-piping/validation/benchmarks/numerical_robustness"
manifest_b=blob(R/"I23/vk_prep_05/SOURCE_MANIFEST.json");manifest=json.loads(manifest_b)
ready=j("I22/vk_f17_diagnostic_01/READINESS.json");assert sha(manifest_b)==ready["source_manifest_sha256"]
assert not git("diff","--name-only",manifest["revision"],head,"--",*manifest["scopes"])
for f in manifest["files"]:
 p=B/f["path"];assert sha(p.read_bytes())==f["sha256"];assert format(stat.S_IMODE(p.stat().st_mode),"04o")==f["mode"]
assert len(manifest["files"])==189
basis_p=A/R/"source_review_RV29/vk_f17_gap_35/SHA256SUMS";assert sha(basis_p.read_bytes())=="852ef66311a13c2c919c7dbada988b001956ece78c457cd3221e78071a4f2ef6"
sourcebasis=json.loads((basis_p.parent/"_run_records/SOURCE_BASIS.json").read_text())
for c in sourcebasis["cited_sources"]:assert sha(blob(c["path"]))==c["sha256"]
casebytes=blob(vr+"/cases/rf_skew.jsonl");cases=[json.loads(s) for s in casebytes.splitlines()];ids=[c["id"] for c in cases];cid={c["id"]:c for c in cases}
frozenbytes=blob(vr+"/observations/kernel_lane/rf_skew.json");frozen=json.loads(frozenbytes)
art=ready["binary"];assert sha(path(art["path"]).read_bytes())==art["sha256"];fpb=path(art["fingerprint_path"]).read_bytes();assert sha(fpb)==art["fingerprint_sha256"]
fp=json.loads(fpb);assert fp["features"]=='["seeded-faults"]' and fp["rustflags"]==[] and art["profile"]["opt_level"]=="3" and art["profile"]["test"] is False
fps=[{"path":art["fingerprint_path"],"sha256":sha(fpb),"original":fp}]
for dep in ready["dependency_fingerprints"]:
 db=path(dep["path"]).read_bytes();dj=json.loads(db);assert sha(db)==dep["sha256"] and dj["features"]==dep["features"] and dj["rustflags"]==[];fps.append({"path":dep["path"],"sha256":sha(db),"original":dj})
assert sha((W/"guard/memguard.sh").read_bytes())==ready["guard_source_sha256"]
managerpre=j("manager/vk_f17_diagnostic_01/PRELAUNCH.json");assert managerpre["permission"].startswith("RELEASE_EXISTING_I22")
grant=read("I22/vk_f17_diagnostic_01/GRANT.md");assert sha(grant)==managerpre["brief_sha256"]
numgrant=subprocess.check_output(["git","-C",str(W/"numerics"),"show",managerpre["grant"]+":"+str(R/"BRIEFS/A1_VK_F17_DIAGNOSTIC.md")],env=dict(os.environ,GIT_OPTIONAL_LOCKS="0"));assert numgrant==grant
allrecords={};calls=[];reports={};last=None; dt=lambda s:datetime.datetime.fromisoformat(s.replace("Z","+00:00"))
for name,fault in [("D01_NONE_BEFORE","NONE"),("D02_F17","VK-F17"),("D03_NONE_RETURN","NONE")]:
 p="I22/vk_f17_diagnostic_01/"+name+"/";e=j(p+"EVIDENCE.json");assert e["environment"]["FK_SEEDED_FAULT"]==fault
 assert e["argv"]==[art["path"],"RF-SKEW"]+["--show="+c for c in ids]
 assert e["cwd"]=="<wt>/scratch/i23/vk_prep/archive/"+vr
 assert e["fresh_process_required"] and e["executed"]
 assert e["unset_environment"]==["RUSTFLAGS","CARGO_ENCODED_RUSTFLAGS","RUSTC_WRAPPER","RUSTC_WORKSPACE_WRAPPER"]
 toks=shlex.split(e["cmd"]);assert toks[:3]==["/usr/bin/time","-l","env"]
 for k,v in e["environment"].items():assert k+"="+v in toks
 for z in e["unset_environment"]:assert toks[toks.index(z)-1]=="-u"
 assert toks[toks.index(art["path"]):toks.index(">")]==e["argv"]
 assert e["tool_initial"]["exit_code"]==0 and "5387" in e["preflight"]
 launch=dt(e["launch_utc"]);assert dt(managerpre["utc"])<launch<dt(ready["hard_end_utc"]);assert last is None or last<launch;last=launch
 raw={}
 for stream in ["stdout","stderr"]:
  b=path(e[stream]).read_bytes();assert sha(b)==e["raw_hashes"][stream];assert b.replace(str(W).encode(),b"<wt>")==read(p+stream+".txt");raw[stream]=b.decode()
 decoder=json.JSONDecoder();s=raw["stdout"];index=0;records=[]
 while True:
  while index<len(s) and s[index].isspace():index+=1
  if index==len(s) or s[index]!="{":break
  obj,index=decoder.raw_decode(s,index);records.append(obj)
 tail=s[index:];assert len(records)==36 and [r["id"] for r in records]==ids and records==j(p+"RECORDS.json")
 for record in records:
  assert record["family"]=="RF-SKEW" and record["case_limit"]==2**64-1 and type(record["case_limit"]) is int
  assert record["k4src_sha256"]==cid[record["id"]]["k4src_sha256"]
  assert record["report"]["rows"]==len(cid[record["id"]]["rows"])
 if fault=="NONE":
  assert records==frozen
  for token in ["rows 1080: passes 982", "not covered 2", "structural zeros 96", "failures 0", "selected Some(128)", "controls discriminated 105, non-discriminating 33, undiscriminated [], unexpectedly failing []"]:assert token in tail,token
  assert "FAILURE" not in tail
 else:
  assert "CLASS" not in tail and '"unresolved None": 36' in tail
  for token in ["passes 0", "not covered 0", "failures 1080", "controls discriminated 105, non-discriminating 33, undiscriminated [], unexpectedly failing []"]:assert token in tail
 allrecords[name]=records;reports[name]=tail
 duration=float(re.search(r"([0-9]+\.[0-9]+) real",raw["stderr"]).group(1));assert duration<300
 calls.append({"id":name,"argv":e["argv"],"fault":fault,"launch":e["launch_utc"],"real_seconds":duration,"raw_hashes":e["raw_hashes"],"record_count":len(records),"limit_exact":True})
assert allrecords["D01_NONE_BEFORE"]==allrecords["D03_NONE_RETURN"]
seqs=[];group=collections.Counter();protected=[]
for r in allrecords["D02_F17"]:
 assert r["outcome"].startswith("Unresolved Ceiling ") and len(r["attempts"])==4
 assert [(a["precision"],a["role"]) for a in r["attempts"]]==[(128,"Candidate"),(256,"VerificationThenCandidate"),(512,"VerificationThenCandidate"),(1024,"Verification")]
 reasons=[a["outcome"] for a in r["attempts"]]
 for reason in reasons[:3]:assert re.fullmatch(r"Rejected\(PublicationEnclosure \{ quantity: .+, body: 0, kind: (Translation|Rotation|Force|Moment), predicate: PublicRelative \}\)",reason)
 assert len(set(reasons[:3]))==1 and reasons[3]=="Solved";group[reasons[0]]+=1
 assert r["report"]["fail"]==r["report"]["rows"] and all(r["report"][k]==0 for k in ["pass","pass_absolute_range","not_covered","structural_zero","expected_unresolved"])
 seqs.append({"id":r["id"],"k4src_sha256":r["k4src_sha256"],"outcome":r["outcome"],"sequence":[[a["precision"],a["role"],a["outcome"]] for a in r["attempts"]]})
 if cid[r["id"]]["not_covered"]:
  assert cid[r["id"]]["not_covered"]==["tw.M1"]
  assert reasons[0]=='Rejected(PublicationEnclosure { quantity: EndAction { member: 1, end: I, component: Ux }, body: 0, kind: Force, predicate: PublicRelative })'
  protected.append({"id":r["id"],"actual":reasons[0],"protected":"EndAction { member: 1, end: J, component: Rx }, body: 0, kind: Moment","same_quantity":False,"same_kind":False,"same_body":True,"class_observed":False})
assert len(protected)==2 and len(group)==7 and sum(group.values())==36
for f in ["I22/vk_f17_diagnostic_01/FINAL_CHECK.json","manager/vk_f17_diagnostic_01/FINAL_CHECK.json"]:
 fcheck=j(f);at=fcheck.get("checked_utc",fcheck.get("verified_utc"));assert dt(last.isoformat())<dt(at)<dt(ready["hard_end_utc"])
(D/"CHECKS.json").write_text(json.dumps({"candidate":head,"source_count":189,"source_manifest_sha256":sha(manifest_b),"source_basis35_seal":sha(basis_p.read_bytes()),"frozen_records_sha256":sha(frozenbytes),"binary":art,"original_fingerprints":fps,"grant_sha256":sha(grant),"calls":calls,"full_records_per_process":36,"both_none_exact_full_equal":True,"fault_sequences":seqs,"rejection_groups":dict(group),"protected_quantity_relation":protected,"reports":reports,"diagnosis":"Named relative-certificate prevention after R7; no CLASS credit; P29 remains unqualified.","limits":["No draft values/classes/H exposed","Source/control/provenance checks are bounded; recorded guard snapshots are not continuous monitoring","No P30 inspected pending seal"]},indent=2)+"\n")
print(json.dumps({"calls":len(calls),"controls_exact":True,"source_files":189,"named_rejections":108,"groups":dict(group),"protected":protected},indent=2))
