from pathlib import Path
from copy import deepcopy
import json,hashlib,sys
out=Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=True)
prep=json.loads(Path(sys.argv[2]).read_text())
def R(call,span,used,t=0,s=0,n=0,r=0,c=0,check=False,line=None):
 return {'call':call,'exact_operand_range':span,'used_after':used,'delta':dict(term_limbs=t,shift_limbs=s,net_limbs=n,rounded_limbs=r),'context_lme':c,'checkpoint':check,'candidate_line':line}
def finish(rows):
 c=0
 for row in rows:
  row.pop('one_below_prefix_limit',None);row.pop('cumulative_reference_path_lme',None)
  row['new_lme']=sum(row['delta'].values())+row['context_lme'];c+=row['new_lme'];row['cumulative_candidate_local_lme']=c
  if row.get('checkpoint'): row['one_below_room']=c-1
 return c
A=deepcopy(prep['H_reference_path'])
for x in A: x['checkpoint']=False
h=A[:4]
h[-1]['checkpoint']=True;h[-1]['candidate_line']=2978
h += [R('predicate: difference.add_binary64(b=2)','single bit exponent1; trimmed1',2,t=2),R('predicate: difference.add_scaled(H,true,1,0)','H used19 both signs; trimmed17; anchor1 -> -1074, shift1075',19,t=56,s=38),R('predicate: difference.signum; collect/check','2-(1+h)>0; padded used19',19,n=19,check=True,line=3020)]
for ix in [5,6,4,7,8]:
 h.append(A[ix]);h[-1]['checkpoint']=True
h+=A[9:]
for ix in [14,17,20]: h[ix]['checkpoint']=True
# indexes after predicate: 0..3 formation;4..6 comparison;7 sign;8 round;
# 9 den;10 d.round;11 division;12..14 reaches1;15..17 reachesup;18..20 reaches1.
h += [R('round_up final checked(Ok(c))','no additional work; prior reaches already checked identical total',None,check=True,line=2889),R('certify_rows final checked(Accepted)','no additional LME for finite/class/radius/Box writes',None,check=True,line=3296)]
assert finish(h)==18103
assert h[6]['cumulative_candidate_local_lme']==159
assert [h[i]['cumulative_candidate_local_lme'] for i in [7,8,9,10,11,14,17,20]]==[178,267,269,307,17749,17868,17984,18103]
B=deepcopy(prep['small_A1_original_path'])
for x in B:x['checkpoint']=False
b=B[:3]+[B[4],B[5],B[3],B[6],B[7]]+B[8:]
for ix in [2,3,4,5,6,7,10,13]:b[ix]['checkpoint']=True
b += [R('round_up final checked','same total as preceding reaches check',None,check=True,line=2889)]
assert finish(b)==17563
assert [b[i]['cumulative_candidate_local_lme'] for i in [2,3,4,5,6,7,10,13]]==[6,8,46,48,86,17528,17542,17563]
reject=deepcopy(h[:7]);reject[4]['call']='predicate: difference.add_binary64(b=1)';reject[4]['exact_operand_range']='single bit exponent0; trimmed1';reject[5]['exact_operand_range']='H used19; anchor0 -> -1074 shift1074; same padded used19';reject[6]['exact_operand_range']='1-(1+h)=-h<0; numeric rejection after collect/check';assert finish(reject)==159
span=[R('publication_h: add_binary64(1)','single bit0',2,t=2),R('add_wide(v=0,true)','zero',2),R('make_absolute: signum','used2',2,n=2),R('add_wide(W_plus=2^-8128) -> Span; collect Err','proposed span8129; refusal before raw charge; old prefix retained',2,line=2978)]
assert finish(span)==4
exponent=[R('form num.add_integer(1,2^62+1); collect','within i64 anchor; outside Wide exponent range; span1',2,t=2,check=True),R('round_up n.signum; clone_delta','inherited construction2 excluded; sign used2',2,n=2,check=True,line=2850),R('n.round -> Exponent; clone_delta Err','net4 + rounded2 + contextRound32 before checked_exponent refusal',2,n=4,r=2,c=32,line=2865)]
assert finish(exponent)==42
# Exact arithmetic checks, no solver output.
from fractions import Fraction
hh=Fraction(1,1<<1074);assert 1<1+hh<1+Fraction(1,1<<52)<2;assert 3*hh==hh+hh+hh
result={'status':'FROZEN BEFORE NEW COUNTER OBSERVATION','candidate':'3cf296e36645d97e4c657c8ad1a6322bc4163f16','adaptive_sha256':'4e5618a8d809e6ffa4ddd73024c206cc45eac6d5404a2f2034674552e1a95db9','H_success':h,'H_numeric_rejected_b1':reject,'small_A1_bound_once':b,'terminal_Span_publication_h':span,'terminal_Exponent_round_up':exponent,'totals':{'H_success':18103,'H_reject':159,'small_A1_bound_once':17563,'span_h':4,'exponent_round_up_with_collected_formation':42},'expected_mutant_arithmetic':{'PM16_a_remove_all_clone_delta_and_reaches_sumwork':{'H_success':17667,'small_A1_bound':17514},'PM16_b_replace_each_clone_delta_by_full_sumwork':{'H_success':18212,'small_A1_bound':17579},'status':'conditional exact patch definitions; unexecuted'},'limits':'These are one-row algebra/helper paths. Any prior stage/other rows are an external independently known base B. Small-A1 inputs alone do not specify row H, so its row certificate total is not invented. Actual full-case costs remain unobserved.'}
(out/'LEDGER.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'candidate':result['candidate'],'totals':result['totals'],'mutant_arithmetic':result['expected_mutant_arithmetic'],'checks':'passed'},indent=2))
