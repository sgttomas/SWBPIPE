import json,tarfile,hashlib,pathlib,re,datetime,stat,subprocess,os,collections
W=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3")
R=pathlib.Path("projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30")
A=W/"a1"; D=A/R/"source_review_RV29/vk_completed_34/_run_records"; T=tarfile.open(D/"committed_packets.tar")
B=W/"scratch/i23/vk_prep/archive"; VR=B/"projects/chirality-piping/validation/benchmarks/numerical_robustness"
sha=lambda b:hashlib.sha256(b).hexdigest()
def read(n):return T.extractfile(str(R/n)).read()
def physical(s):return pathlib.Path(s.replace("<wt>",str(W)))
manifest=json.loads((A/R/"I23/vk_prep_05/SOURCE_MANIFEST.json").read_text()); source=[]
for f in manifest["files"]:
 p=B/f["path"]; assert sha(p.read_bytes())==f["sha256"]; assert format(stat.S_IMODE(p.stat().st_mode),"04o")==f["mode"]
 source.append({"path":f["path"],"sha256":f["sha256"]})
assert len(source)==189
ids=[14,16,20,21,22,23,24,25,26]; faults={21:"VK-F08",23:"VK-F10",25:"VK-F13"}; checks=[]; binaries={}
prev=None
for n in ids:
 i=f"P{n}"; packet="03" if n<20 else "04"; prefix=f"I22/vk_runtime_{packet}/{i}/"; e=json.loads(read(prefix+"EVIDENCE.json"))
 out=physical(e["stdout_path"]).read_bytes();err=physical(e["stderr_path"]).read_bytes()
 for k,b in [("stdout",out),("stderr",err)]:
  assert sha(b)==e["raw_hashes"][k]
  matches=[m for m in T.getmembers() if m.isfile() and m.name.startswith(str(R/prefix)) and m.name.endswith(k+".txt")]
  if not matches: matches=[m for m in T.getmembers() if m.isfile() and m.name.startswith(str(R/prefix)) and k in m.name and not m.name.endswith("json")]
  assert len(matches)==1,(i,k,[m.name for m in matches])
  assert b.replace(str(W).encode(),b"<wt>")==T.extractfile(matches[0]).read(),(i,k)
 ob=out.decode(); eb=err.decode(); assert "running 1 test" in ob
 expected_fault=faults.get(n,"NONE");assert e["environment"]["FK_SEEDED_FAULT"]==expected_fault;assert e["fresh_process"] is True
 for token in ["env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER","RUSTUP_TOOLCHAIN=1.97.1","RUSTUP_AUTO_INSTALL=0","CARGO_INCREMENTAL=0","RUST_TEST_THREADS=2","CARGO_NET_OFFLINE=true","FK_SEEDED_FAULT="+expected_fault]: assert token in e["cmd"]
 assert e["argv"][1]=="--exact" and e["argv"][-2:]==["--test-threads=2","--nocapture"]
 assert "5387" in e["guard_before"] and "guard/memguard.sh" in e["guard_before"]
 exitcode=(e.get("tool_completion") or e["tool_initial"])["exit_code"];assert exitcode==(101 if n in faults else 0)
 assert ("test result: FAILED. 0 passed; 1 failed" if n in faults else "test result: ok. 1 passed; 0 failed") in ob
 dt=lambda s:datetime.datetime.fromisoformat(s.replace("Z","+00:00"))
 launch,end=dt(e["launch_utc"]),dt(e["completed_utc"]);assert dt(e["runtime_authority"]["at"])<launch<=end<dt(e["hard_end"])
 assert dt(e["hard_end"])==dt("2026-10-01T20:30:30Z"); assert prev is None or prev<launch;prev=end
 release=read(f"manager/vk_runtime_{packet}/SCHEDULE_RELEASE.md");assert sha(release)==e["runtime_authority"]["sha256"]
 b=e["binary"];bb=physical(b["path"]).read_bytes();fpb=physical(b["fingerprint_path"]).read_bytes();assert sha(bb)==b["sha256"] and sha(fpb)==b["fingerprint_sha256"]
 fp=json.loads(fpb);assert fp["features"]=='["seeded-faults"]' and fp["rustflags"]==[]
 assert b["profile"]["test"] and b["profile"]["opt_level"]=="0"
 binaries[b["key"]]={**b,"original_fingerprint":fp}
 if "test_source" in e: assert sha((B/e["test_source"]["path"]).read_bytes())==e["test_source"]["sha256"]
 checks.append({"process":i,"phase":e["phase"],"returns_for":e.get("returns_for"),"fault":expected_fault,"argv":e["argv"],"cwd":e["cwd"],"launch":e["launch_utc"],"completion_observed":e["completed_utc"],"exit":exitcode,"raw_hashes":e["raw_hashes"],"release_sha256":sha(release),"qualifying_condition_original":e["qualifying_condition"],"result":re.search(r"test result:.*",ob).group()})
assert checks[1]["phase"]=="FRESH_NONE_BASELINE" and checks[1]["returns_for"] is None
for a,b in [(21,22),(23,24),(25,26)]:
 x=next(c for c in checks if c["process"]==f"P{a}"); y=next(c for c in checks if c["process"]==f"P{b}");assert x["argv"]==y["argv"]
err21=(W/"scratch/i23/vk_prep/logs/P21.stderr").read_text();assert "tests/parity.rs:21:9" in err21 and "RF-LARGE-CHAIN-n00010-AX: bitwise K" in err21 and 'pattern Some(0.0), dense -3.9793506945470715e8' in err21
err23=(W/"scratch/i23/vk_prep/logs/P23.stderr").read_text();assert "tests/adapter.rs:73:9" in err23 and "RF-CHAIN-T-n03-r1e-04" in err23
left=bytes(json.loads(re.search(r"left: (\[.*\])",err23).group(1)));right=bytes(json.loads(re.search(r"right: (\[.*\])",err23).group(1)));assert left!=right
cases={c["id"]:c for c in map(json.loads,(VR/"cases/rf_cancel.jsonl").read_text().splitlines())}
out25=(W/"scratch/i23/vk_prep/logs/P25.stdout").read_text();rows=re.findall(r"RF-CANCEL: FAILURE (\S+): (\S+) \(([^)]+)\) predicate",out25);assert len(rows)==58 and len(set(rows))==58
for cid,key,ref in rows:
 assert "G1e80" in cid
 matches=[r for r in cases[cid]["rows"] if r[0]==key];assert len(matches)==1 and matches[0][1]==ref
assert len({c for c,_,_ in rows})==8
assert '"selected": 38' in out25 and 'failures 58' in out25 and 'CLASS' not in out25 and 'unresolved' not in out25.lower().replace('expected unresolved 0','')
output={"candidate":"cb60b0861f756d3d37a0bf5325c206d74f5f6ec1","source_manifest_revision":manifest["revision"],"source_count":len(source),"source_bindings":source,"processes":checks,"binaries":binaries,"F08":{"site":"tests/parity.rs:21","case":"RF-LARGE-CHAIN-n00010-AX","matrix_entry":"(0,6)","pattern":"Some(0.0)","dense":"-3.9793506945470715e8"},"F10":{"site":"tests/adapter.rs:73","case":"RF-CHAIN-T-n03-r1e-04","left_size":len(left),"right_size":len(right),"left_sha256":sha(left),"right_sha256":sha(right),"different_byte_positions":sum(a!=b for a,b in zip(left,right))},"F13":{"site":"tests/lane.rs:75","reference_rows":rows,"case_counts":dict(collections.Counter(c for c,_,_ in rows)),"all_58_exact_reference_strings_match":True},"limitations":["Completion stamps are observation times, not raw process elapsed times.","P16 inherited qualifying_condition retains original return wording, explicitly superseded by phase/returns_for and grant/release.","No P28 or later call inspected or credited."]}
(D/"CHECKS.json").write_text(json.dumps(output,indent=2)+"\n")
print(json.dumps({"checked_calls":len(checks),"fault_assertions":3,"none_passes":6,"source_files":189,"binaries":list(binaries),"F10":output["F10"],"F13cases":output["F13"]["case_counts"]},indent=2))
