"""Narrow RV47-C1 correction confirmation: immutable Git reads and exact math only."""
from fractions import Fraction as Q
from pathlib import Path
import subprocess, os, hashlib, json, io, contextlib
R='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/'
P=R+'I36/f2a_ordinary_coefficients_02/'
OLD='f5ca39a1a2d4c63b0fda3fe2e3c61bbe8afde074'
NEW='7bca4a0dfd88de4fd3f6d1c59de3c98a189ab7ee'
ENV=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def sha(b): return hashlib.sha256(b).hexdigest()
def git(*args): return subprocess.check_output(['git',*args],env=ENV)
def get(v,p): return git('show',v+':'+p)
def replay(b,p):
    scope={}; buf=io.StringIO()
    with contextlib.redirect_stdout(buf): exec(compile(b,p,'exec'),scope)
    return scope,buf.getvalue().encode()
old_script=get(OLD,P+'_run_records/exact_controls.py')
new_script=get(NEW,P+'_run_records/correction_03/exact_controls.py')
old,old_out=replay(old_script,'frozen_original_exact_controls.py')
new,new_out=replay(new_script,'frozen_correction_03_exact_controls.py')
assert old_out==get(OLD,P+'_run_records/exact_controls.json')
assert new_out==get(NEW,P+'_run_records/correction_03/exact_controls.json')
data=json.loads(new_out)
assert len(data['exact_K_correction']['coefficients'])==4
products=[]
for idx,item in enumerate(data['exact_K_correction']['coefficients']):
    left,right=map(Q,[item['left_primitive'],item['right_primitive']])
    # Independent rational multiplication, not the author's constructor.
    exact=Q(left.numerator*right.numerator,left.denominator*right.denominator)
    previous=old['ck'][idx]
    lo,hi=new['cs'][idx]
    delta=max(abs(lo-exact),abs(hi-exact))
    old_delta=max(abs(lo-previous),abs(hi-previous))
    assert exact==new['ck'][idx]==Q(item['corrected_exact_CK'])
    assert previous==Q(item['historical_rounded_CK'])
    assert exact-previous==Q(item['exact_minus_historical_CK'])
    assert delta==Q(item['corrected_delta']) and old_delta==Q(item['historical_delta'])
    assert exact!=previous and delta!=old_delta
    assert item['CK_changed'] is True and item['delta_changed'] is True
    products.append({'coefficient':item['coefficient'],'exact_product_verified':True,'both_delta_bounds_verified':True,'CK_changed':True,'delta_changed':True})
x=Q(1)+Q(1,2**52)
exact=Q(1)+Q(1,2**51)+Q(1,2**104)
rounded=Q(1)+Q(1,2**51)
assert x*x==exact and exact-rounded==Q(1,2**104)
disc=data['exact_K_correction']['rounding_sensitive_control']
assert Q(disc['primitive_operand'])==x and Q(disc['exact_CK'])==exact
assert Q(disc['RN64_CK'])==Q(disc['source_singleton'])==rounded
assert Q(disc['required_delta'])==Q(1,2**104) and Q(disc['historical_faulty_delta'])==0
# Run only the shared-constructor discriminator with the faulty constructor.
block=new_script.decode().split('# Same corrected constructor, with a discriminator that the old boundary fails.')[1].split('# Reviewed bridge, scalar specialization only; no matrix or solve routine.')[0]
mutant=dict(new)
mutant['exact_k_products']=lambda pairs: [new['bits'](a*b) for a,b in pairs]
try:
    exec(compile(block,'isolated_rounding_discriminator_with_faulty_constructor','exec'),mutant)
except AssertionError:
    mutation_rejected=True
else:
    mutation_rejected=False
assert mutation_rejected
# Verify unaffected statements around the replaced coefficient block and output.
a=old_script.decode(); b=new_script.decode()
old_start=a.index('ck=[bits(eh*bits(')
new_start=b.index('# RV47-C1 correction:')
end_marker='# Reviewed bridge, scalar specialization only; no matrix or solve routine.'
assert a[:old_start]==b[:new_start]
old_tail=a[a.index(end_marker):]
new_tail=b[b.index(end_marker):]
insert_start=new_tail.index(" 'exact_K_correction':")
insert_end=new_tail.index(" 'limits':",insert_start)
assert new_tail[:insert_start]+new_tail[insert_end:]==old_tail
original_json=json.loads(old_out)
assert data['groups']==original_json['groups'][:11]+['exact_K_product_rounding_sensitive_negative_control']+original_json['groups'][11:]
for key in ['status','Ehat_positive_source_zero','limits']: assert data[key]==original_json[key]
correction=json.loads(get(NEW,P+'_run_records/correction_03/CORRECTION.json'))
for name in correction['comparison']['unaffected_exact_value_state_names']:
    assert old[name]==new[name]
# Precise author write scope and preserved raw/theorem/dependency bytes.
expected={'RETURN.md':'M','_run_records/INVENTORY.json':'M','_run_records/correction_03/CORRECTION.json':'A','_run_records/correction_03/exact_controls.py':'A','_run_records/correction_03/exact_controls.json':'A'}
changes=git('diff-tree','--no-commit-id','--name-status','-r',NEW).decode().splitlines()
assert {line.split('\t')[1]:line.split('\t')[0] for line in changes}=={P+k:v for k,v in expected.items()}
assert set(correction['write_fence'])==set(expected)
preserved=[]
for item in correction['preserved_original_payloads']:
    path=P+item['path']; original=get(OLD,path); current=get(NEW,path)
    assert current==original and len(original)==item['bytes'] and sha(original)==item['sha256']
    preserved.append({'path':item['path'],'sha256':sha(original)})
old_return=get(OLD,P+'RETURN.md').decode()
new_return=get(NEW,P+'RETURN.md').decode()
start='## 1. Source values and positivity'; end='## 4. Private interface, controls and return'
assert old_return.split(start)[1].split(end)[0]==new_return.split(start)[1].split(end)[0]
assert sha(get(OLD,P+'_run_records/INVENTORY.json'))==correction['original_inventory_sha256']
for item in correction['origins']:
    raw=get(item['revision'],item['path']); assert len(raw)==item['bytes'] and sha(raw)==item['sha256']
for command in correction['commands']:
    assert sha(command['command'].encode())==command['command_utf8_sha256']
inv=json.loads(get(NEW,P+'_run_records/INVENTORY.json'))
for item in inv['files']:
    raw=get(NEW,P+item['path']); assert len(raw)==item['bytes'] and sha(raw)==item['sha256']
actual=set(git('ls-tree','-r','--name-only',NEW,'--',P).decode().splitlines())
assert actual=={P+item['path'] for item in inv['files']}|{P+'_run_records/INVENTORY.json'}
review=R+'REVIEW_RV47/f2a_ordinary_coefficients_02/'
review_files=git('ls-tree','-r','--name-only','dfd7a0771122d061dfa4bcec8a0aac4358c5da2f','--',review).decode().splitlines()
for path in review_files: assert get('dfd7a0771122d061dfa4bcec8a0aac4358c5da2f',path)==get(NEW,path)
print(json.dumps({'status':'RV47-C1 correction confirmed; no new finding','old_groups':original_json['group_count'],'corrected_groups':data['group_count'],'old_output_sha256':sha(old_out),'corrected_output_sha256':sha(new_out),'corrected_script_sha256':sha(new_script),'products':products,'shared_constructor_mutation_rejected':mutation_rejected,'required_delta':'2^-104','faulty_delta':'0','unaffected_group_statements_preserved':16,'unaffected_named_states_equal':len(correction['comparison']['unaffected_exact_value_state_names']),'exact_five_file_author_scope':changes,'preserved_original_payloads':preserved,'theorem_sections_1_to_3_preserved':True,'author_inventory_payloads_verified':len(inv['files']),'correction_origin_hashes_verified':len(correction['origins']),'recorded_command_hashes_verified':len(correction['commands']),'previous_review_files_preserved':len(review_files),'limits':'Exact abstract correction backcheck only; no product/source implementation, runtime, wider qualification or availability claim.'},indent=2))
