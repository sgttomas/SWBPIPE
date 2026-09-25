"""Read-only retained-run backcheck; does not execute Rust, build, or mutate sources."""
from pathlib import Path
import json,re,hashlib,collections,sys,platform
OUT=Path(__file__).parent
BASE=OUT.parents[1]
ROOT=next(p for p in BASE.parents if (p/"projects/chirality-piping/core/product_physics").is_dir())
post=BASE/"_run_records/POSTFIX"
before=json.loads((post/"BEFORE.json").read_text())
after=json.loads((post/"AFTER.json").read_text())
stages=json.loads((post/"STAGES.json").read_text())
candidate=json.loads((BASE/"CANDIDATE_FILES.json").read_text())
assert after["sources_and_lock_unchanged"] is True
assert stages==after["stages"] and {x["stage"] for x in stages}=={"rotational","translational","moment_alias"}
assert all(x["exit_code"]==0 for x in stages)
for p,want in candidate.items():assert before["sources"][p]==want
current={}
for p,want in before["sources"].items():
    actual=hashlib.sha256((ROOT/p).read_bytes()).hexdigest()
    current[p]={"expected":want,"actual":actual,"match":actual==want}
assert all(x["match"] for x in current.values())
# Independently inspect recursive local path-dependency manifests and Rust sources.
pending=[ROOT/"projects/chirality-piping/core/product_physics/Cargo.toml"];seen=set()
while pending:
    manifest=pending.pop().resolve()
    if manifest in seen:continue
    seen.add(manifest)
    text=manifest.read_text()
    for path in re.findall(r'path\s*=\s*"([^"]+)"',text):
        other=(manifest.parent/path/"Cargo.toml").resolve()
        if other.is_file():pending.append(other)
assert len(seen)==before["local_manifest_count"]==13
expected_files=set()
for m in seen:
    expected_files.add(str(m.relative_to(ROOT)))
    expected_files.update(str(s.relative_to(ROOT)) for s in (m.parent/"src").rglob("*.rs"))
assert expected_files<=set(before["sources"])
def rows(path):
    found={}
    for line in path.read_text().splitlines():
        if not line.startswith("rotational-gap unit contract:"):continue
        m=re.match(r'rotational-gap unit contract: mode=(\w+), dof=(\w+), gap=([^ ]+) (\w+), mechanics=(\w+), rotation=(.*), diagnostics=(\[.*\])$',line)
        assert m,line
        mode,dof,value,unit,status,rotation,codes=m.groups()
        key=(mode,dof,value,unit);assert key not in found
        found[key]={"status":status,"rotation":rotation,"codes":json.loads(codes)}
    assert len(found)==36
    return found
baseline=rows(BASE/"_run_records/BASELINE/baseline.log")
result=rows(post/"rotational.log")
assert baseline.keys()==result.keys()
oldcount=collections.Counter();newcount=collections.Counter();comparisons=[]
for key,old in baseline.items():
    mode,dof,value,unit=key;new=result[key]
    if dof.startswith("rotation_"):
        oldcount["invalid_support_token"]+=1;newcount["preserved_invalid_support_token"]+=1
        assert new==old
        expected="UNIT_CONVERSION_UNAVAILABLE" if unit=="rad" else "NONLINEAR_SUPPORT_DOF_INVALID"
        assert new["codes"]==[expected]
    else:
        if unit=="rad":
            oldcount["legacy_rad_unit_rejected"]+=1
            assert old["codes"]==["UNIT_CONVERSION_UNAVAILABLE"]
        else:
            oldcount["unsafe_solved"]+=1
            assert old["status"]=="MECHANICS_SOLVED" and old["rotation"]=='Some((5e-5, "rad"))'
        newcount["explicit_rotational_block"]+=1
        assert new["codes"]==["NONLINEAR_ROTATIONAL_GAP_UNSUPPORTED"]
    assert new["status"]=="MODEL_INCOMPLETE" and new["rotation"]=="None"
    comparisons.append({"case":key,"before":old,"after":new})
assert dict(oldcount)=={"invalid_support_token":18,"unsafe_solved":12,"legacy_rad_unit_rejected":6}
assert dict(newcount)=={"preserved_invalid_support_token":18,"explicit_rotational_block":18}
testnames={"rotational":"unit_contract_rotational_gap_cannot_consume_a_length_as_an_angle",
 "translational":"gap_preview_closes_to_explicit_clearance_through_dense_loop",
 "moment_alias":"input_contract_authored_moment_axes_reach_live_solve"}
for stage,name in testnames.items():
    text=(post/f"{stage}.log").read_text()
    assert "running 1 test" in text
    assert f"test tests::{name} ... ok" in text
    assert "test result: ok. 1 passed; 0 failed;" in text
lib=(ROOT/"projects/chirality-piping/core/product_physics/src/lib.rs").read_text()
test=lib.split("fn unit_contract_rotational_gap_cannot_consume_a_length_as_an_angle()",1)[1].split("    #[test]",1)[0]
assert "assert!(!output.accepted_model_state_mutated);" in test
assert 'd.severity == "blocking"' in test
assert 'output.status.mechanics == "MECHANICS_SOLVED"' in test
hashes={}
for p in ["_run_records/POSTFIX/BEFORE.json","_run_records/POSTFIX/AFTER.json","_run_records/POSTFIX/STAGES.json",
          "_run_records/POSTFIX/rotational.log","_run_records/POSTFIX/translational.log","_run_records/POSTFIX/moment_alias.log",
          "_run_records/BASELINE/baseline.log","_run_records/BASELINE_TEST.patch","_run_records/REPAIR/test_expectation.patch",
          "_run_records/REPAIR/validation.patch","CANDIDATE_FILES.json","REVIEW/RETURN.md"]:
    hashes[p]=hashlib.sha256((BASE/p).read_bytes()).hexdigest()
out={"kind":"Independent read-only backcheck of parent executed logs; no new Rust/build run",
 "reviewed_two_file_hashes":candidate,"all_recorded_current_source_lock_hashes_match":True,
 "snapshot_file_count":len(current),"recursive_local_manifest_count":len(seen),
 "dependency_manifest_and_src_rs_coverage_verified":True,"stages":stages,
 "baseline_counts":dict(oldcount),"postfix_counts":dict(newcount),"all36_postfix_no_solved_no_rotation":True,
 "blocking_severity_and_no_mutation_assertions_executed":"All36 loop observations followed by test PASS; frozen body asserts blocking and accepted_model_state_mutated=false per case",
 "case_comparison":comparisons,"input_sha256":hashes,
 "runtime":{"python":sys.version,"platform":platform.platform(),"raw_root":str(ROOT)},
 "executor":"/root/solver_manager/kernel_review","parent":"/root/solver_manager",
 "mechanism":"delegated-harness-native resumed TASK, no descendants","dispatch":"unchanged gpt-6-astra xhigh",
 "instruction_basis":"Prior REVIEW/_run_records/ORIGINS.json; same frozen assignment and source review",
 "limits":"Containment backcheck only; no Current/general angular/whole product/native qualification."}
(OUT/"RUNTIME_BACKCHECK.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k not in ["case_comparison","input_sha256","runtime"]},indent=2))

