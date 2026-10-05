from pathlib import Path
import hashlib,json,subprocess,os
root=Path.cwd()
r=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
out=root/r/'REVIEW_RV44/f2a_no_wrap_01'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
source='49034a940f3f8cd3f3da4d4cbc839943b808063d'
paths=[Path('AGENTS.md'),Path('agents/AGENT_TASK.md'),Path('projects/chirality-piping/AGENTS.md'),Path('/Users/ryan/.codex/worktrees/92ea/chirality/.agents/skills/software-code-review/SKILL.md'),r/'BRIEFS/RV44_F2A_NO_WRAP_REVIEW_01.md',r/'REVIEW_RV43/f2a_wire_c1_01/REVIEW.md',r/'I32/f2a_wire_c1/WIRE_CONTRACT.md',r/'REVIEW_RV43/f2a_wire_c1_02/RETURN.md',r.parent/'W1_RESOURCE_POLICY_V1.md']
fk=Path('projects/chirality-piping/core/solver/frame_kernel')
paths += [fk/'src/structural/retained'/x for x in ['wide_sum.rs','wide.rs','wide/multi.rs','adaptive.rs','assemble.rs','factor.rs','bound.rs','verify.rs','recover.rs','source.rs','ledger.rs','directed.rs','combine.rs']]+[fk/'Cargo.toml']
origins=[]
for path in paths:
    p=path if path.is_absolute() else root/path
    data=p.read_bytes()
    row={'path':str(p),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()}
    if str(path).startswith(str(fk)):
        basis=subprocess.check_output(['git','show',f'{source}:{path}'],env=env)
        assert basis==data,str(path)
        row['source_revision']=source
    origins.append(row)
(out/'_run_records/ORIGINS_STAGE0.json').write_text(json.dumps({'source':source,'head':subprocess.check_output(['git','rev-parse','HEAD'],env=env,text=True).strip(),'origins':origins},indent=2)+'\n')
values={'context_max_price':18*(64*16+2),'add_raw':256+256,'add_scaled':2*(128+512),'product_of':128*(2*(128+512))+256,'atom_price':2**18,'max_atoms_strict_u64':((2**64-2)//2**18),'max_raw_terms_under_atom_cap':256*((2**64-2)//2**18),'condition_atom_coefficients':{'E':44,'F':46,'B':33,'A':1,'constant':15}}
assert values['product_of']<values['atom_price']
assert values['max_raw_terms_under_atom_cap']<2**54
(out/'_run_records/ARITHMETIC_STAGE0.json').write_text(json.dumps(values,indent=2)+'\n')
data=(out/'SOURCE_DERIVATION.md').read_bytes()
(out/'_run_records/STAGE0_FREEZE.json').write_text(json.dumps({'stage':'0','source_derivation_sha256':hashlib.sha256(data).hexdigest(),'author_packet_read':False,'limitations':'Partial source/count derivation only; no complete admission theorem.'},indent=2)+'\n')
print(json.dumps({'source_derivation_sha256':hashlib.sha256(data).hexdigest(),'origins':len(origins),'arithmetic':values}))
