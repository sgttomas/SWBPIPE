from pathlib import Path
import hashlib,json,os,re,subprocess,time,runpy,sys
r=Path('/private/tmp/piping-numerical-integrity-20260924');n=r/'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/NUMERICAL_INTEGRITY';d=n/'PRODUCT_FULL_DIAGNOSIS/_run_records/EXECUTION_01';d.mkdir(parents=True,exist_ok=False)
oldargv=sys.argv;sys.argv=['snapshot'];module=runpy.run_path('/private/tmp/solver_ready_checks.py');sys.argv=oldargv;snapshot=module['snapshot']
p=r/'projects/chirality-piping/core/product_physics/src/lib.rs';m=p.parents[1]/'Cargo.toml';h=lambda b:hashlib.sha256(b).hexdigest()
before=p.read_bytes();assert h(before)=='37d19221af9900e40d500370c30c156e49f5469a905f20acb3b2e3db6ae54d80'
probe=(n/'PRODUCT_FULL_DIAGNOSIS/_run_records/lib.probe_candidate.rs').read_bytes();assert h(probe)=='738d4aabda61453d83daee23d7f0df85ffe959a3f34b88e872da70d0e631b783'
original=snapshot(m);cmd=['cargo','test','--offline','--locked','-j2','--manifest-path',str(m.relative_to(r)),'--lib','tests::diagnosis_probe_product_full_five_failures','--','--exact','--nocapture','--test-threads=1']
env=os.environ.copy();env['CARGO_BUILD_JOBS']='2';env['CARGO_TARGET_DIR']='/private/tmp/piping-product-rotational-gap-target'
p.write_bytes(probe);instrumented=snapshot(m)
(d/'BEFORE.json').write_text(json.dumps({'original':original,'instrumented':instrumented,'command':cmd,'cwd':str(r),'target':env['CARGO_TARGET_DIR'],'started_unix':time.time()},indent=2)+'\n')
result=None
try:
 with (d/'cargo.log').open('w') as log:result=subprocess.run(cmd,cwd=r,env=env,stdout=log,stderr=subprocess.STDOUT)
finally:
 current=p.read_bytes();unchanged_probe=h(current)==h(probe)
 if unchanged_probe:p.write_bytes(before)
 restored=snapshot(m)
 (d/'AFTER.json').write_text(json.dumps({'exit_code':None if result is None else result.returncode,'instrumented_file_unchanged':unchanged_probe,'restored_original_source_and_locks':restored==original,'after':restored,'log_sha256':h((d/'cargo.log').read_bytes()),'ended_unix':time.time()},indent=2)+'\n')
text=(d/'cargo.log').read_text();rows=[]
for block in text.split('DIAGPROBE OUTPUT ')[1:]:
 block=block.split('DIAGPROBE INPUT ')[0]
 label=block.split(' status=',1)[0]
 status=re.search(r'mechanics: "([^"]+)"',block)
 rows.append({'label':label,'mechanics':status.group(1) if status else None,'diagnostic_codes':re.findall(r'code: "([^"]+)"',block),'empty_rows':bool(re.search(r'rows=\[\s*\]',block))})
summary={'exit_code':result.returncode,'restored':restored==original,'public_runs':rows,'mixed_headlines':[line for line in text.splitlines() if line.startswith('DIAGPROBE FINAL_QUALIFICATION') or line.startswith('DIAGPROBE ITERATION')]}
(d/'SUMMARY.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if result.returncode:print(text[-3000:])
sys.exit(result.returncode or (0 if restored==original else 1))
