"""API-02 finite specification arithmetic only; no product imports or Rust witness."""
from pathlib import Path
import json
M=(1<<64)-1
E,O,I=0,1,2
rows=[]
def check(name, predicates, result):
    assert all(predicates), name
    rows.append({'id':name,'pass':True,'result':result})

def delta(a,b,after_stream,before_stream,sa=0,sb=0):
    s=sa|sb|(I if after_stream!=before_stream else 0)
    if not s and a<b:s|=I
    return (None,s) if s else (a-b,E)
check('A01_stream_identity',[delta(7,7,'a','b')==(None,I),delta(7,7,'a','a')==(0,E),delta(M,M,'a','a',O,O)==(None,O)],{'foreign_equal':'I','exact_same_equal':0,'lost_equal':'O'})
# Persistent CloneWork: sequential observations on the same inherited clone.
base=M-500
sign=128
rounded=384
check('A02_persistent_clone',[base+sign<=M,base+sign+rounded>M,base+rounded<=M],{'before':base,'sign_delta':sign,'round_delta':rounded,'persistent_clone_round':'overflow','incorrect_fresh_reclone_would_fit':True})
# A pre-reservation changes no charged payload and reserves the pending base.
limit=31;t=25;base_charge=4;shift_charge=1
assert t+base_charge+shift_charge<=limit
t+=shift_charge
assert t+1+base_charge<=limit
t+=1
next_allowed=t+1+base_charge<=limit
check('A03_pending_base',[not next_allowed,t==27],{'committed_prefix':t,'pending_base_uncommitted':base_charge,'next_carry_performed':False,'exact_total_claim':False})
# Poisoned clear selects full reset, preserving all prior work flags.
poisoned=True;used=1;magnitude=[1,2,3,4];state=O
if poisoned:magnitude=[0]*len(magnitude);poisoned=False
check('A04_poisoned_clear',[magnitude==[0,0,0,0],not poisoned,state==O],{'value_reset':'full','work_flag':'O','next_numeric_operation':'WorkAccounting'})
# Combined primitive: first insertion succeeds; second fails pre-mutation.
first_mutated=True;second_mutated=False;second_error=True
compound_poison=second_error and (first_mutated or second_mutated)
check('A05_compound_poison',[compound_poison],{'second_failed_before_write':True,'destination_poisoned':True})
# Existing numerical prior is retained while accounting blocks escalation.
def terminal(prior,status):
    return {'kind':'work_accounting','fault':status,'prior':prior if prior!='work_accounting' else None} if status else {'kind':prior}
check('A06_prior_and_no_escalation',[terminal('Condition',O)['prior']=='Condition',terminal('Span',I)['prior']=='Span',terminal('work_accounting',O)['prior'] is None],{'condition_with_O':terminal('Condition',O),'span_with_I':terminal('Span',I),'new_precision_started':False})
# Cached work is not charged anew but invalid dependency state transfers.
case=7;increment=0;incoming=12;origin_state=O
check('A07_zero_price_status',[case==7,increment==0,(E|origin_state)==O],{'case_charge_diagnostic':case,'actual_new_increment':increment,'incoming_prefix':incoming,'invocation_receipt_state':'O'})
# C2 source refusal has no fabricated Run and carries actual snapshots.
call={'variant':'PreSourceRefusal','before':(12,O),'after':(12,O),'run':None,'source':None}
check('A08_c2_pre_source',[call['before']==call['after'],call['run'] is None,call['source'] is None,call['before'][1]!=E],{'call':call,'successor_emitted':False})
helpers={
'chord_d':2*512+128,
'chord_c':2*1280+128,
'determinant_sum':24*512+128,
'determinant_c128':6*32,
'determinant_c192':12*32,
'intensified_sum':3*512+128,
'row_bound_num_clone':3*512+128+384,
'row_bound_den_clone':512+384,
'row_bound_context':2*32+17*(64*16+2),
'fresh_correction_scratch':2*1280+128,
'binary64_up':2*512+128}
check('A09_closed_fresh_helpers',[all(v<(1<<18) for v in helpers.values()),helpers['row_bound_context']==17506],helpers)
# These are old exact formula prices, not an additional instrumentation price.
price_rows=[]
for l in (4,8,16):
    p=[2*l,2*l,l*l,(l+1)*(64*l+2),(l+2)*(64*l+2),2*l,12*l,l*l+4*l]
    price_rows.append({'L':l,'prices':p})
check('A10_pinned_prices',[price_rows[0]['prices']==[8,8,16,1290,1548,8,48,32],price_rows[-1]['prices']==[32,32,256,17442,18468,32,192,320]],price_rows)
check('A11_view_gate',[not bool(O==E),bool(E==E)],{'H_work_lines_on_non_E':False,'VR_record_on_non_E':None,'legacy_Refused_fields_changed':False,'E_serialized_status_field':False})
check('A12_exact_range',[M>(1<<53)-1],{'native_exact_MAX':'E','JSON_successor':'work_counter_range','unknown':'work_counter_unknown','OI':'work_counter_inconsistent','O':'work_counter_overflow','automatic_lower_bound':False})
out={'scope':'selected API specification controls only; not compiled/source-edge/runtime witnesses','passed':len(rows),'controls':rows}
Path(__file__).with_name('CONTROLS.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({'passed':len(rows),'helper_bounds':len(helpers),'price_widths':3}))
