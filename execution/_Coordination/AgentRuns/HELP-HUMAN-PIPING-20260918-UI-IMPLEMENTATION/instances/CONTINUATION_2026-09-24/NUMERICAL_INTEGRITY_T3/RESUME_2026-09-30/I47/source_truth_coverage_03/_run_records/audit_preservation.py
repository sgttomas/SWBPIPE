#!/usr/bin/env python3
from pathlib import Path
from fractions import Fraction as Q
import json,hashlib,subprocess,os,struct,datetime
O=Path(__file__).resolve().parent;R=O.parents[2];NUM=O.parents[12];CODE=NUM.parent/'f2a';REL=R.relative_to(NUM)
assert NUM.name=='numerics' and (CODE/'AGENTS.md').exists()
def h(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text())
def write(name,x):(O/name).write_text(json.dumps(x,indent=2)+'\n')
env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0'
def git(args,cwd):return subprocess.check_output(['git',*args],cwd=cwd,env=env,text=True)
assert git(['rev-parse','HEAD'],CODE).strip()=='d0daa18717f8243a7232e898c9ef9b4f4d18d9e4'
old=CODE/REL/'I47/selected_material_02';seal=read(old/'SEAL.json');inventory=read(old/'_run_records/INVENTORY.json')
assert h(old/'_run_records/INVENTORY.json')==seal['inventory_sha256'] and h(old/'RETURN.md')==seal['return_sha256']
assert all(h(old/p)==v for p,v in inventory.items())
before=read(O/'SOURCE_PRESERVATION_BEFORE.json');after={p:h(CODE/p) for p in before};assert before==after;write('SOURCE_PRESERVATION_AFTER.json',after)
assert git(['diff','--name-only'],CODE)==''
for path in ['BRIEFS/I47_FULL_TRUTH_COVERAGE_ADDENDUM.md','verification/material_truth_coverage_challenge_01/CHALLENGE.md','verification/material_truth_coverage_challenge_01/ROOT_CHECK.json']:
 assert git(['show','26160e94661ef49db726c3fde5809121a584c10f:'+str(REL/path)],NUM)==(R/path).read_text()
assert h(O/'FROZEN_ORACLE.py')==h(old/'_run_records/independent_oracle.py')
assert h(O/'OLD_RESULTS.json')==h(old/'_run_records/oracle_final_debug_results.json')
assert h(O/'ROOT_CHECK_COPY.json')==h(R/'verification/material_truth_coverage_challenge_01/ROOT_CHECK.json')
first=read(O/'COMMAND.json');final=read(O/'COMMAND_final.json');assert first['exit']==1 and final['exit']==0
assert first['inputs']['full_truth_cover.py']==h(O/'full_truth_cover_initial.py')
assert all(h(O/k)==v for k,v in final['inputs'].items())
# Record all nine captured interpolation input primitives, independently decoding bits.
profiles=[]
for c in read(O/'CAPTURES.json'):
 values=c['selection']['values'];p={}
 for k in ['t_lo','t','t_hi','e_lo','e_hi','g_lo','g_hi','e_hat','g_hat']:
  if k not in values:continue
  q=Q(struct.unpack('>d',bytes.fromhex(values[k]))[0]);n=abs(q.numerator);d=q.denominator
  assert d&(d-1)==0
  significant=n.bit_length()-(n&-n).bit_length()+1 if n else 0
  assert significant<=53
  p[k]={'bits':values[k],'exact':str(q),'significant_bits':significant}
 profiles.append({'case':c['label'],'captured_interpolation_primitives':p})
write('MATERIAL_OPERAND_PROFILES.json',profiles)
result=read(O/'FULL_COVER_RESULTS.json');count=0;passing=0
for c in result['cases']:
 for row in c['rows']:
  for p in row['predicates']:
   count+=1;bound=Q(p['allowance']);upper=Q(p['error_upper']);lower=Q(p['error_lower'])
   assert lower>bound or upper<=bound
   assert p['full_cover_truth_passed']==(upper<=bound)
   if p['candidate_passed']:assert upper<=bound;passing+=1
assert count==562 and passing==390 and result['false_passes']==[]
changed=[(x['case'],x['row'],x['id']) for x in result['changed_rows']]
assert changed==[('interpolated-loaded',10,'result:disp:node-N-DEC092-TIP:ux')]
assert [x['name'] for x in result['changed_rows'][0]['changed_predicates']]==['SharperExact','SharperBinary64']
(O/'FINAL_NUM_STATUS.log').write_text(git(['status','--short'],NUM));(O/'FINAL_CODE_STATUS.log').write_text(git(['status','--short'],CODE))
write('PRESERVATION_AUDIT.json',{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'frozen_source_pin':'d0daa18717f8243a7232e898c9ef9b4f4d18d9e4','old_payloads_unchanged':len(inventory),'source_hashes_unchanged':True,'code_tracked_diff_empty':True,'brief_challenge_pin_verified':'26160e94661ef49db726c3fde5809121a584c10f','frozen_oracle_unchanged':True,'old_results_unchanged':True,'final_command_inputs_verified':True,'exact_predicates':count,'certified_candidate_pass_predicates':passing,'uncertain_comparisons':0,'false_passes':0,'only_changed_truth_row':changed,'runtime_lane':'unused','source_or_runtime_writes':False})
print(json.dumps(read(O/'PRESERVATION_AUDIT.json'),indent=2))
