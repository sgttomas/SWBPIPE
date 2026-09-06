import pathlib, subprocess, hashlib, json, os, base64, time
root=pathlib.Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip()); w=root/'projects/chirality-piping'; e=w/'execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/F1'; t=pathlib.Path('/tmp/piping-physics-audit-f1'); t.mkdir(exist_ok=True)
h=lambda b:hashlib.sha256(b).hexdigest()
source=w/'core/product_physics/src/lib.rs'; assert h(source.read_bytes())=='93d182ee3504db7114058ff7de0aaa6f096728398869ca72a39d6ad78b18b2cc'
brief=w/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/briefs/F1_FIXTURE_REGENERATION_V1.md'; assert h(brief.read_bytes())=='fb11575751ab20f4691e65cf55e5f874e44b039946caace3d04ecc0272314389'
f=w/'fixtures/product_preview/invented_mechanics_result.json'; e.joinpath('before.json').write_bytes(f.read_bytes())
env=dict(os.environ,CARGO_TARGET_DIR=str(t/'target'),CARGO_NET_OFFLINE='true'); records=[]
def run(name,cmd,cwd=w):
 start=time.time(); r=subprocess.run(cmd,cwd=cwd,env=env,capture_output=True); records.append(dict(name=name,command=cmd,cwd=str(cwd.relative_to(root)),returncode=r.returncode,seconds=time.time()-start,stdout_sha256=h(r.stdout),stderr_sha256=h(r.stderr)))
 e.joinpath(name+'_transport.json').write_text(json.dumps(dict(stdout_base64=base64.b64encode(r.stdout).decode(),stderr_base64=base64.b64encode(r.stderr).decode(),stdout_sha256=h(r.stdout),stderr_sha256=h(r.stderr)),indent=2)+'\n'); e.joinpath('EXECUTIONS.json').write_text(json.dumps(records,indent=2)+'\n'); print(name,r.returncode,flush=True); return r
cmd=['cargo','run','--quiet','--manifest-path','core/product_physics/Cargo.toml','--example','preview_result']
a=run('generate_1',cmd); assert a.returncode==0; json.loads(a.stdout); t.joinpath('generated_1.json').write_bytes(a.stdout)
b=run('generate_2',cmd); assert b.returncode==0 and a.stdout==b.stdout; assert h(source.read_bytes())=='93d182ee3504db7114058ff7de0aaa6f096728398869ca72a39d6ad78b18b2cc'
t.joinpath('generated_atomic.json').write_bytes(a.stdout); os.replace(t/'generated_atomic.json',f); e.joinpath('after.json').write_bytes(a.stdout)
inputs={str(p.relative_to(root)):h(p.read_bytes()) for p in [source,brief,w/'package.json',w/'core/product_physics/examples/preview_result.rs',w/'fixtures/product_preview/invented_preview_model.json']}
e.joinpath('BINDING.json').write_text(json.dumps(dict(inputs=inputs,before_sha256=h(e.joinpath('before.json').read_bytes()),after_sha256=h(a.stdout),deterministic=True,environment={k:env[k] for k in ['CARGO_TARGET_DIR','CARGO_NET_OFFLINE']},temporary_scope=str(t),model='unknown',role='Agent 2 ephemeral generalist',parent='/root',runtime='/root/fixture_regeneration',enforcement='instruction+config asserted',script_equivalence='Existing generator command, redirected to isolated temporary bytes and atomic replacement only after both successful identical runs.'),indent=2)+'\n')
run('product_tests',['cargo','test','--manifest-path','core/product_physics/Cargo.toml'])
run('consumer_tests',['npm','test','--','src/services/previewService.test.ts'],w/'apps/desktop')
assert h(source.read_bytes())==inputs[str(source.relative_to(root))]
