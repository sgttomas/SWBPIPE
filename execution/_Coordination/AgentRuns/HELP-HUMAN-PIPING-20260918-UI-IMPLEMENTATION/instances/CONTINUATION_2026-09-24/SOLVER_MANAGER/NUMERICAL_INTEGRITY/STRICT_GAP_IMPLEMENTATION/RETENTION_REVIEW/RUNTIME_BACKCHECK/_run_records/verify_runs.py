"""Verify retained parent-run evidence only. No subprocess/Rust/build/Git execution."""
from pathlib import Path
import json,hashlib,re,sys,platform
OUT=Path(__file__).parent
ROOT=next(p for p in OUT.parents if (p/"projects/chirality-piping/core/solver/frame_kernel").is_dir())
NUM=OUT.parents[3]
RUN=NUM/"SOLVER_FIRST_CHECKS/_run_records"
if not RUN.exists():
    RUN=next(ROOT.glob("projects/chirality-piping/execution/_Coordination/AgentRuns/*/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/NUMERICAL_INTEGRITY/SOLVER_FIRST_CHECKS/_run_records"))
spec={
 "frame_exact_boundary":(20,61),
 "nonlinear_compile":(None,None),
 "nonlinear_ret01":(1,57),
 "nonlinear_retained":(1,57),
 "nonlinear_strict":(6,52),
 "nonlinear_coupled":(1,57),
 "nonlinear_decimal":(1,57),
 "nonlinear_affine":(1,57),
}
kernel="projects/chirality-piping/core/solver/frame_kernel/src/structural/exact_boundary.rs"
adapter="projects/chirality-piping/core/solver/nonlinear_integration/src/structural_adapter.rs"
lib="projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs"
reviewed={kernel:"6801610d0bb515eab33603163423812b81bf94946d8c23b99c127b655d996a7f",
 adapter:"f7600925cd44169ce6d04a03ae5c0e622a8f27aabee831377d297dce68ec51aa",
 lib:"a5a3f7104b8a77c9d9709a9d4abd5123e14aedd4fa31f2d3763cc0101677034f"}
def h(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def closure(manifest):
    todo=[manifest.resolve()];seen=set();files=set()
    while todo:
        m=todo.pop()
        if m in seen:continue
        seen.add(m);files.add(m)
        for named in ["Cargo.lock","build.rs"]:
            f=m.parent/named
            if f.is_file():files.add(f)
        for dirname in ["src","tests","examples","benches"]:
            d=m.parent/dirname
            if d.is_dir():files.update(d.rglob("*.rs"))
        # Observed manifests use literal path entries. Lib/bin source paths are
        # excluded because they do not name a directory containing Cargo.toml.
        manifest_text="\n".join(line for line in m.read_text().splitlines() if not line.lstrip().startswith("#"))
        for path in re.findall(r'path\s*=\s*"([^"]+)"',manifest_text):
            dependency=(m.parent/path/"Cargo.toml").resolve()
            if dependency.is_file():todo.append(dependency)
    for f in list(files):
        if f.suffix==".rs":
            for rel in re.findall(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*,?\s*\)',f.read_text()):
                q=(f.parent/rel).resolve()
                if q.exists():files.add(q)
    return {str(p.relative_to(ROOT)) for p in seen},{str(p.relative_to(ROOT)) for p in files}
stages=[];union={};inputs={}
for name,(passed,filtered) in spec.items():
    stage=RUN/name
    before=json.loads((stage/"BEFORE.json").read_text())
    after=json.loads((stage/"AFTER.json").read_text())
    log=(stage/"cargo.log").read_text()
    assert after["exit_code"]==0 and after["unchanged"] is True
    assert before["snapshot"]==after["after"]
    assert h(stage/"cargo.log")==after["log_sha256"]
    command=before["command"]
    assert command[:5]==["cargo","test","--offline","--locked","-j2"]
    assert "--lib" in command
    assert before["snapshot"]["hashes"][kernel]==reviewed[kernel]
    if name!="frame_exact_boundary":
        assert before["snapshot"]["hashes"][adapter]==reviewed[adapter]
        assert before["snapshot"]["hashes"][lib]==reviewed[lib]
    if passed is None:
        assert "--no-run" in command
        assert not re.search(r"^test result:",log,re.M)
        assert "Executable unittests" in log
        testnames=[]
    else:
        assert "--test-threads=2" in command and "--nocapture" in command
        matches=re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;",log)
        assert matches==[(str(passed),"0","0","0",str(filtered))]
        testnames=re.findall(r"^test (.*?) \.\.\. ok$",log,re.M)
        assert len(testnames)==passed
    manifest=ROOT/command[command.index("--manifest-path")+1]
    manifests,files=closure(manifest)
    snap=before["snapshot"]
    assert manifests==set(snap["manifests"])
    assert files==set(snap["hashes"])
    for p,want in snap["hashes"].items():
        assert h(ROOT/p)==want
        if p in union:assert union[p]==want
        union[p]=want
    for file in ["BEFORE.json","AFTER.json","cargo.log"]:
        inputs[f"{name}/{file}"]=h(stage/file)
    stages.append({"stage":name,"exit_code":0,"passed_tests":passed,"filtered_tests":filtered,
        "test_names":testnames,"sources_lock_includes_unchanged":True,"local_manifests":len(manifests),
        "captured_files":len(files),"command":command,"raw_cwd":before["cwd"],"raw_target":before["target"]})
allnames={n for s in stages for n in s["test_names"]}
for name in ["structural::exact_boundary::tests::retained_adjacent_witness_replays_after_context_drop",
 "structural::exact_boundary::tests::retained_mutations_and_different_consistent_source_reject_original_binding",
 "structural::exact_boundary::tests::retained_force_tail_and_projection_rebinding_are_checked",
 "structural_adapter::retention_tests::ret01_long_id_and_nested_error_are_reserved_before_summary_comparison",
 "structural_adapter::retention_tests::actual_adapter_retains_adjacent_response_and_separate_ordinary_projected_states"]:
    assert name in allnames
inputs["run_checks.py"]=h(RUN/"run_checks.py")
result={"kind":"Independent read-only backcheck of executed frame/nonlinear stages only",
 "reviewed_identities":reviewed,"stages":stages,"all_before_after_snapshots_identical":True,
 "all_log_hashes_verified":True,"recursive_local_and_literal_include_coverage_verified":True,
 "current_source_lock_include_bytes_match":True,"union_file_count":len(union),"input_sha256":inputs,
 "limits":["No product stages included.","Compile-only stage is not a test pass.",
 "Nested fixture-loop counts and actual numerical charge totals are not printed; no inferred runtime counts reported.",
 "No persisted artifact/wire/currentness/native/performance qualification."],
 "executor":"/root/solver_manager/kernel_review","parent":"/root/solver_manager",
 "mechanism":"same delegated-harness-native TASK reviewer; no descendants","dispatch":"unchanged gpt-6-astra xhigh",
 "runtime":{"python":sys.version,"platform":platform.platform(),"raw_root":str(ROOT)},
 "operations":"Read/hash/parse only; no Rust, build, Git or source edits.",
 "discovery_note":"Initial lookup expected top-level BEFORE/AFTER; actual evidence is per-stage. The first read-only verifier stopped before inspection because this review Python lacks tomllib; it was replaced with observed literal-path traversal. No Rust/test process ran in either attempt."}
(OUT/"VERIFIED_RUNS.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({"stages":[{k:s[k] for k in ["stage","exit_code","passed_tests","filtered_tests","local_manifests","captured_files"]} for s in stages],
 "union_file_count":len(union),"all_hashes_and_coverage_verified":True,"product_stages_excluded":True},indent=2))
