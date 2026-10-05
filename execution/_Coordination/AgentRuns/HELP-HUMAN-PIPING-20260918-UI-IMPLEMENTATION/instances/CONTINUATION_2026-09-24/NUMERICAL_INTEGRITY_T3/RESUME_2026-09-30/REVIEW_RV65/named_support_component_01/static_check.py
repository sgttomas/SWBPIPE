"""RV65 independent static checks only; no product/model/compiler execution."""
from pathlib import Path
from fractions import Fraction
import hashlib, json, os, platform, re, subprocess, sys

NUM = Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics')
CODE = NUM.parent / 'f2a'
R = Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
OUT = Path(__file__).resolve().parent
RP = 'f58de3f5cccc44200a2fc021a7c6d447618791e7'
SP = 'd0daa18717f8243a7232e898c9ef9b4f4d18d9e4'
IP = '848981f43349e62173b0a91850f369c3a040a1a0'
I = R / 'I50/first_publishing_component_01'
P = Path('projects/chirality-piping/core')
env = dict(os.environ, GIT_OPTIONAL_LOCKS='0')
def blob(root, pin, path):
    return subprocess.check_output(['git', 'show', pin+':'+str(path)], cwd=str(root), env=env)
def digest(b):
    return hashlib.sha256(b).hexdigest()
origins = []
def origin(root, pin, path, scope, compare=True):
    b = blob(root, pin, path)
    if compare:
        assert b == (root/path).read_bytes(), str(path)
    origins.append(dict(origin=str(root/path), revision=pin, sha256=digest(b), bytes=len(b), scope_read=scope))
    return b

for p in ['AGENTS.md', 'agents/AGENT_TASK.md', 'projects/chirality-piping/AGENTS.md', '.agents/skills/software-code-review/SKILL.md']:
    origin(NUM, RP, Path(p), 'full active instruction/selected skill')
origin(NUM, RP, R/'BRIEFS/RV65_NAMED_SUPPORT_COMPONENT.md', 'full active brief')
origin(NUM, RP, R/'ROOT_CURRENT.md', 'full initial supplied snapshot; later changes not adopted', False)
for p in ['RETURN.md','SOURCE_CHECK.json','source_check.py','EXECUTION.json','ORIGINS.json','NAMED_REQUEST.json','SEAL.json','SEAL_HISTORY_ADDENDUM.json','ADDENDUM_SEAL.json']:
    origin(NUM, IP, I/p, 'full subject evidence; named payload parsed independently' if p=='NAMED_REQUEST.json' else 'subject source/design and provenance evidence')
for p, scope in [
    ('I31/f2a_certificate_b1/SOURCE_OPERANDS.md','operand/row table and identity context'),
    ('I31/f2a_certificate_b1/RETURN.md','125-180: finite identified support sum and units'),
    ('I33/f2a_source_action_bridge_01/RETURN.md','1-240: premises, source bridge, support law, unchanged predicates'),
    ('I36/f2a_preview_truth_01/RETURN.md','65-114: ordinary section/E/G/source warrant')]:
    origin(NUM, RP, R/p, scope)
origin(NUM, RP, R.parent/'ROOT_RULINGS_V1.md', 'targeted selection search and sections 5038-5080,5180-5260,5290-5345,5500-5545,5750-5825', False)

code_paths = {
    'product_physics/src/retained_product.rs':'capture/source, binding/metadata, observables, G5a, checked adapter work',
    'product_physics/src/retained_product_tests.rs':'base/selected callers, existing identities, exact G5a and failure-prefix expectations',
    'product_physics/tests/formation_check_runtime.rs':'exact named literal, entry/mode helpers and protected test',
    'product_physics/src/lib.rs':'actual support builder/recovery, nodal/support/station/stress/mode producer loops',
    'product_physics/src/preview_physics.rs':'same-DOF ambiguity, support replacement, maximum/readout context',
    'solver/linear_supports/src/lib.rs':'prepare_boundary, rigid/spring families and scalar SpringEntry',
    'solver/frame_kernel/src/structural/retained/source.rs':'primitive shapes, counts, canonicalization and support checks',
    'solver/frame_kernel/src/structural/retained/recover.rs':'native layout, signed end/station/spring/reaction and group recovery',
    'solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs':'recipe, scales, coverage, gate, work/status/capacity',
    'solver/frame_kernel/src/structural/retained/product_certificate/source_residual.rs':'998-1090 spring, reaction and group source recovery',
    'solver/frame_kernel/src/structural/retained/product_certificate/bridge.rs':'605-680 spring/group bridge bounds',
    'solver/frame_kernel/src/structural/retained/origins.rs':'779-806 private certificate facade caller and owner identity',
    'solver/frame_kernel/tests/retained_k4/product_final_case_tests.rs':'full existing focused tests; only isolated NonQuantity gate rows'
}
for p, scope in code_paths.items():
    b = origin(CODE, SP, P/p, scope)
    assert b == (NUM/P/p).read_bytes(), 'NUM/CODE mismatch: '+p

# Verify the delivered seal chain and the preserved final subject, never rewrite it.
for seal_name in ['SEAL.json','ADDENDUM_SEAL.json']:
    seal = json.loads((NUM/I/seal_name).read_text())
    for item in seal['files']:
        b = (NUM/I/item['path']).read_bytes()
        assert len(b)==item['bytes'] and digest(b)==item['sha256'], item['path']
add = json.loads((NUM/I/'ADDENDUM_SEAL.json').read_text())
base = (NUM/I/'SEAL.json').read_bytes()
assert len(base)==add['base_seal']['bytes'] and digest(base)==add['base_seal']['sha256']

# Use the owning literal independently, not the author's generated expectation.
test = (CODE/P/'product_physics/tests/formation_check_runtime.rs').read_bytes()
matches = re.findall(rb'const RF_SKEW_T_CANT_OFF_122_R1E_04: &str = r#"(.*?)"#;', test)
assert len(matches)==1
raw = matches[0]
assert raw == (NUM/I/'NAMED_REQUEST.json').read_bytes()
j = json.loads(raw)
m = j['model']
assert not j['materials'] and not m['combinations'] and not m.get('components',[])
assert len(m['load_cases'])==1 and len(m['materials'])==1
assert m['pipe_segments'][0]['y_reference']==dict(x=1,y=0,z=0)
nodes = m['nodes']; supports=m['supports']; loads=m['load_cases'][0]['primitive_loads']
assert all(s['node']=='N0' for s in supports)
assert supports[0]['restraints']==['UX','UY','UZ'] and 'family' not in supports[0]
springs = [s for s in supports if s.get('family')=='spring']
assert [(s['restraints'],s['stiffness']['dof'],s['stiffness']['value']['value'],s['stiffness']['value']['unit']) for s in springs]==[
    (['RX'],'RX',144,'N*m/rad'),(['RY'],'RY',1000000,'N*m/rad'),(['RZ'],'RZ',1000000,'N*m/rad')]
assert [(v['direction'],v['target'],v['magnitude']['unit']) for v in loads]==[(c,{'type':'node','node':'N1'},'N*m') for c in ['RX','RY','RZ']]
f = lambda x: Fraction.from_float(float(x))
d = [f(nodes[1]['position'][a])-f(nodes[0]['position'][a]) for a in 'xyz']
M = [f(l['magnitude']['value']) for l in loads]
assert d==[1,2,2] and M==[M[0],2*M[0],2*M[0]]
assert sum(x*x for x in d)==9
n=len(nodes); members=len(m['pipe_segments']); stations=3*members; g=len(supports); s=len(springs)
k=sum(len(x['restraints']) for x in supports if x.get('family')!='spring')
q=7*n+12*members+6*stations+s+k+2*g
final=7*n+12*members+6*stations+6*g+2*g+4*(2*members+stations)+members+1
assert (n,members,stations,s,k,g,q,final)==(2,1,3,3,3,4,58,98)
assert b'NUMERICAL_INTEGRITY_SENSITIVE' in test and b'for (entry, envelope) in both_entries(&value, mode)' in test
result=dict(kind='independent static source/fixture/seal check', runtime_executed=False,
    request_bytes=len(raw), request_sha256=digest(raw), preserved_subject=IP,
    final_seals_verified=True, overwritten_preliminary_bytes_claimed_preserved=False,
    focused_source_files_equal_to_pin_and_NUM=len(code_paths),
    expected_census=dict(n=n,m=members,t=stations,s=s,k=k,g=g,Q=q,final=final,mechanical=final-1,
        support_components=6*g,support_magnitudes=2*g,reaction_components=k,spring_components=s,empty_components=6*g-k-s),
    exact_input_only=dict(chord=[str(x) for x in d],moment_parallel=True,length_squared='9'),
    census_warrant='Review of recover::layout, source_counts, producer loops and PP complete coverage; source-derived, not executed counts',
    host=dict(node=platform.node(),platform=platform.platform(),python=sys.version),
    pass_all_assertions=True)
(OUT/'CHECK.json').write_text(json.dumps(result,indent=2)+'\n')
(OUT/'ORIGINS.json').write_text(json.dumps(dict(records_pin=RP,source_pin=SP,subject_pin=IP,
    mechanism='delegated-harness-native',parent='/root',task='/root/rv65_named_support_component',
    selected_role='TASK Type 2',selected_workflow=None,selected_skill=str(NUM/'.agents/skills/software-code-review/SKILL.md'),inputs=origins),indent=2)+'\n')
print(json.dumps(result,indent=2))
