"""RV29 finite independent arithmetic/hand-ledger check; no solver imports or runs.
The entries below were enumerated from accepted d01ad98 primitive loops.
They are reference-path expectations, not a simulation of new accounting code.
"""
from fractions import Fraction
from pathlib import Path
import json
import sys

OUT = Path(sys.argv[1])
OUT.mkdir(parents=True, exist_ok=True)
h = Fraction(1, 1 << 1074)
H = 1 + h
up = 1 + Fraction(1, 1 << 52)
assert 1 < H < up
assert H - 1 < Fraction(1, 1 << 1024)  # RN1024(H) = 1
assert 3*h > 2*h and 3*h == h+h+h
assert h/(1 << 64) < h/2 and h/(1 << 53) < h/2
assert ((17*1026) + 2*32) == 17506

# t,s,n,r are NEW SumWork deltas. c is the context LME; zero-priced
# value methods, clones and checks are named in the prose ledger.
def row(call, operand_range, used, t=0, s=0, n=0, r=0, c=0, op=None, span=None):
    return dict(call=call, exact_operand_range=operand_range, used_after=used,
                delta=dict(term_limbs=t, shift_limbs=s, net_limbs=n, rounded_limbs=r),
                context_lme=c, context_operation=op, max_span_evidence=span)

def summarize(rows):
    cum=0
    for item in rows:
        amount=sum(item['delta'].values())+item['context_lme']
        cum+=amount
        item['new_lme']=amount
        item['cumulative_reference_path_lme']=cum
        item['one_below_prefix_limit']=cum-1 if amount else None
    return cum

# Precise reference path: H.add_binary64(1), H.add_wide(0,true),
# H.make_absolute(), H.add_wide(h,false), den.add_binary64(1),
# then the original directed_ratio Up order. Width of add_wide is immaterial
# here: it trims before add_raw and there is no scaling multiplication.
a=[
 row('H.add_binary64(1)', 'single bit exponent 0; trimmed limb count 1',2,t=2,span=1),
 row('H.add_wide(0,true)', 'zero; early return',2,span=1),
 row('H.make_absolute -> signum', 'positive prefix [0,2); value 1',2,n=2,span=1),
 row('H.add_wide(h,false)', 'single bit -1074; anchor 0 -> -1074; new_used=2+16+1=19',19,t=2,s=38,span=1075),
 row('den.add_binary64(1)', 'single bit 0; trimmed limb count 1',2,t=2,span=1),
 row('n=num.clone; n.is_zero', 'inherits H counters (4,38,2,0); signum reads used=19',19,n=19,span=1075),
 row('n.round: net + magnitude + ctx.from_integer', 'exact 1+h; actual occupied indices [0,17); padded used=19; RN1024=1',19,n=38,r=19,c=32,op='Round@L16',span=1075),
 row('d=den.clone; d.round', 'inherits den counters (2,0,0,0); single bit but padded used=2; RN1024=1',2,n=4,r=2,c=32,op='Round@L16',span=1),
 row('ctx.div(nv,dv)', '1/1=1; binary64 conversion is unpriced',None,c=17442,op='Div@L16'),
 row('reaches(1): t.add_scaled(den,2^52,-52)', 'both source prefixes used=2; 4 multiply limbs + 2 raw-add limbs; anchor 0',2,t=6,span=1),
 row('reaches(1): t.add_scaled(H,true,1,0)', 'both source prefixes used=19; 38 multiply limbs + 18 raw-add limbs; span1075; anchor shift1074',19,t=56,s=38,span=1075),
 row('reaches(1): t.signum', '1-(1+h)=-h; used19',19,n=19,span=1075),
 row('reaches(up): t.add_scaled(den,2^52+1,-52)', '53 significant bits [-52,0]; 4 multiply + 2 raw-add; anchor -52',2,t=6,span=53),
 row('reaches(up): t.add_scaled(H,true,1,0)', '1075 significant bits [-1074,0]; anchor shift1022 gives used=2+15+1=18',18,t=56,s=36,span=1075),
 row('reaches(up): t.signum', '2^-52-h > 0; used18',18,n=18,span=1075),
 row('reaches(next_down(up)=1): t.add_scaled(den,2^52,-52)', 'same as first reaches(1), freshly zeroed scratch',2,t=6,span=1),
 row('reaches(next_down(up)=1): t.add_scaled(H,true,1,0)', 'same as first reaches(1)',19,t=56,s=38,span=1075),
 row('reaches(next_down(up)=1): t.signum', '-h < 0; Up correction exits with 1+2^-52',19,n=19,span=1075),
]
assert summarize(a)==17988
assert sum(r['new_lme'] for r in a[5:])==17942
assert sum(sum(r['delta'].values()) for r in a[5:])==436

# Original row_bound(h,h): absolute_bound(h)=h, rounding=h; three h terms.
b=[
 row('num.add_binary64(b=h)', 'single bit -1074; padded used2',2,t=2,span=1),
 row('num.add_binary64(rounding=h)', 'same single-bit add; accumulated value2h; recorded highest TERM stays -1074',2,t=2,span=1),
 row('num.add_binary64(h)', 'same single-bit add; value3h (2 significant bits), used2',2,t=2,span=1),
 row('den.add_binary64(1)', 'single bit0; padded used2',2,t=2,span=1),
 row('n=num.clone; n.is_zero', 'inherits num counters (6,0,0,0); value3h; used2',2,n=2,span=1),
 row('n.round: net + magnitude + ctx.from_integer', 'exact3h, occupied one limb; used2; RN1024 exact',2,n=4,r=2,c=32,op='Round@L16',span=1),
 row('d=den.clone; d.round', 'inherits den counters (2,0,0,0); used2',2,n=4,r=2,c=32,op='Round@L16',span=1),
 row('ctx.div(nv,dv)', '3h/1=3h; binary64 conversion exact, unpriced',None,c=17442,op='Div@L16'),
 row('reaches(3h): t.add_scaled(den,3,-1074)', 'both used2 prefixes; value3h; 2 significant bits; anchor -1074',2,t=6,span=2),
 row('reaches(3h): t.add_scaled(num,true,1,0)', 'both used2 prefixes; 4 multiply + 2 raw-add; no anchor shift',2,t=6,span=2),
 row('reaches(3h): t.signum', 'exact zero from two nonempty magnitudes; used2',2,n=2,span=2),
 row('reaches(2h): t.add_scaled(den,2,-1074)', 'trim low zero; anchor -1073; significant one bit',2,t=6,span=1),
 row('reaches(2h): t.add_scaled(num,true,1,0)', '3h lowers anchor by1; new_used=2+0+1=3; 2*3 shifts',3,t=6,s=6,span=2),
 row('reaches(2h): t.signum', '-h < 0; used3; exits with3h',3,n=3,span=2),
]
assert summarize(b)==17563
assert sum(r['new_lme'] for r in b[4:])==17555
assert sum(sum(r['delta'].values()) for r in b[4:])==49

terminal={
 'span_raw': [
  row('s.add_integer(false,[1],0)', 'single bit0; used2',2,t=2,span=1),
  row('s.add_integer(false,[1],-8128) -> Span', 'proposed span8129 exceeds8128 BEFORE mutation/charge; existing sum unchanged',2,span=1),
 ],
 'span_scaled_width4': [
  row('s.add_integer(false,[1],0)', 'single bit0; used2',2,t=2,span=1),
  row('s.add_wide_scaled(Wide4(2^-8128),false,1,0) -> Span', '4 source-width multiply limbs charged BEFORE add_raw refuses span8129',2,t=4,span=1),
 ],
 'exponent_round': [
  row('s.add_integer(false,[1],2^62+1)', 'finite i64 anchor; add_raw accepts span1',2,t=2,span=1),
  row('s.round(ctx16) -> Exponent', 'net4 then rounded2 then charged Round32; checked_exponent rejects leading exponent 2^62+1',2,n=4,r=2,c=32,op='Round@L16',span=1),
 ],
}
assert {k:summarize(v) for k,v in terminal.items()}=={'span_raw':2,'span_scaled_width4':6,'exponent_round':40}

report={
 'status':'PREPARATORY_REFERENCE_PATHS_FROZEN; NOT_FINAL_CANDIDATE_LEDGER',
 'basis':'d01ad98a754698631f927709d08284c272de85e8 accepted primitives, exact fixed operands; no observed helper counters',
 'H_reference_path':a,'small_A1_original_path':b,'terminal_reference_paths':terminal,
 'clone_inheritance':{
  'H':dict(num=[4,38,2,0],den=[2,0,0,0],new_num=[0,0,57,19],new_den=[0,0,4,2]),
  'small_A1':dict(num=[6,0,0,0],den=[2,0,0,0],new_num=[0,0,6,2],new_den=[0,0,4,2])},
 'conditional_mutant_discrimination':{
  'H_conversion_correct':17942,'H_conversion_omitting_all_new_local_sumwork':17506,'H_conversion_duplicating_inherited_num_den_once':17988,
  'small_A1_conversion_correct':17555,'small_A1_conversion_omitting_all_new_local_sumwork':17506,'small_A1_conversion_duplicating_inherited_num_den_once':17563},
 'stop_limit_note':'Each one_below_prefix_limit is hypothetical until frozen source reveals an actual checkpoint there. If a checkpoint exists, use external base B plus prefix-1; no finer-grained checkpoint or new order is required by this table.',
 'new_candidate_total':'pending exact call order, comparison construction, report-term handling and budget/checkpoint placement in frozen implementation',
 'executed':'standard-library arithmetic and static-ledger arithmetic assertions only; no Rust/solver/mutant',
}
(OUT/'LEDGER.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'H_setup':46,'H_conversion':17942,'H_reference_total':17988,'small_A1_setup':8,'small_A1_conversion':17555,'small_A1_total':17563,'terminal':{k:v[-1]['cumulative_reference_path_lme'] for k,v in terminal.items()},'assertions':'passed','candidate_total':'pending frozen implementation'},indent=2))
