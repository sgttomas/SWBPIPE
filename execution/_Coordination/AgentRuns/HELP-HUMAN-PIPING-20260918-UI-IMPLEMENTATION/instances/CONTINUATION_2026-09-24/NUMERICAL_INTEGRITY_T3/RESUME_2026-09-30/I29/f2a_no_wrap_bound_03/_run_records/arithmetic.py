"""Run-specific exact-integer usefulness check of LOCAL no-wrap components only.
Reads immutable source/storage counts; never executes a model, solver or comparator.
Usage: python3 arithmetic.py NUM_ROOT OUTPUT_DIRECTORY
"""
import hashlib,json,sys
from pathlib import Path
root,out=map(Path,sys.argv[1:])
U64_MAX=(1<<64)-1
origins=[]
def read(rel):
    b=(root/rel).read_bytes()
    origins.append({'path':str(rel),'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b)})
    return b
def local(F,E,W,Z,B,L=16):
    # F,E,W,Z,B are upper bounds; do not substitute F_upper into H-F.
    assert min(F,E,W,Z,B)>=0 and L in (4,8,16)
    add=2*L;mul=L*L;div=(L+1)*(64*L+2);product=mul+4*L
    Q=E*max(W-1,0)//2
    pivot=3*(L+256)+3*256+128
    negative=6*(L+256)+6*256+128
    factor_wide=Q*(mul+add)+E*(div+mul+2*add)
    factor_sum=F*pivot+Z*negative
    row_wide=(W*max(W-1,0)//2)*(mul+add)+W*(div+mul+2*add)
    row_failure=row_wide+pivot+Z*negative
    R=F+11+22*B
    term=(L+256)*Z+38*F*256
    shift=256*Z+38*F*256
    net=256*R
    rounded=128*R
    condition_wide=11*(2*E*(mul+add)+F*div)+add*R+5*F*product+(11*B+F+3)*div+mul
    if F==0:
        condition_wide=term=shift=net=rounded=0
    condition_sum=term+shift+net+rounded
    # At most one approximate and one exact evaluation per offered pivot row.
    margin=F*((4*4+(4+1)*(64*4+2))+(4*16+(16+1)*(64*16+2)))
    total=factor_wide+factor_sum+condition_wide+condition_sum+margin
    return {'Q_upper':Q,'factor_wide_LME':factor_wide,'factor_SumWork_upper':factor_sum,
      'factor_row_with_failure_upper':row_failure,'condition_component_upper':{'term':term,'shift':shift,'net':net,'rounded':rounded},
      'condition_wide_LME':condition_wide,'condition_total_upper':condition_wide+condition_sum,
      'pivot_margin_charged_upper':margin,'covered_local_sum_upper':total,
      'strictly_below_u64_MAX':total<U64_MAX,'global_admission_established':False}
hrel=Path('projects/chirality-piping/core/solver/performance_harness/observations/k6b/counts.jsonl')
hs=[]
for x in map(json.loads,read(hrel).splitlines()):
    F=x['w1_free_dofs'];H=x['w1_profile_entries'];W=x['w1_half_bandwidth'];Z=x['w1_pattern_entries'];B=x['w1_blocks']
    assert H>=F and 0<=B<=F and 0<=W<=max(F-1,0)
    hs.append({'id':x['model'],'family':x['family'],'members':x['w1_members'],
      'count_fields':{'F':F,'H':H,'E':H-F,'W':W,'Z':Z,'B':B},
      'L16':local(F,H-F,W,Z,B)})
vs=[];missing=[]
vdir=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness/observations/kernel_lane')
for p in sorted((root/vdir).glob('rf_*.json')):
    rel=p.relative_to(root)
    for case in json.loads(read(rel)):
        storage=[a['storage'] for a in case['attempts']]
        if not storage:
            missing.append({'id':case['id'],'reason':'no stored attempt counts; no zero-work or admission inference'})
            continue
        pairs={(x['pattern_entries'],x['profile_entries']) for x in storage}
        assert len(pairs)==1
        Z,H=next(iter(pairs))
        # Count-only monotone projection: F<=H, E<=H, W<=H, B<=H.
        vs.append({'id':case['id'],'family':case['family'],'record':str(rel),
          'stored_counts':{'Z':Z,'H':H},'projection':{'F_upper':H,'E_upper':H,'W_upper':H,'B_upper':H},
          'L16':local(H,H,H,Z,H)})
result={'scope':'LOCAL primitive/factor/condition/margin arithmetic usefulness only; not complete no-wrap admission',
 'integer_limit':U64_MAX,'ordinary_build_widths':[4,8,16],
 'atomic_bounds':{'raw_term':256,'raw_shift':256,'wide_scaled_term_at16':272,
 'scaled_sum_term':768,'scaled_sum_shift':512,'product_of_sums_dest_term':98304,
 'product_of_sums_dest_shift':65536,'product_of_sums_other_net':256,
 'round_net':256,'round_rounded':128,'sign_net':128},
 'H':hs,'VR_storage_only':vs,'VR_without_counts':missing}
out.mkdir(parents=True,exist_ok=True)
(out/'ARITHMETIC.json').write_text(json.dumps(result,indent=2)+'\n')
(out/'ARITHMETIC_ORIGINS.json').write_text(json.dumps(origins,indent=2)+'\n')
print(json.dumps({'H_rows':len(hs),'H_local_max':max(x['L16']['covered_local_sum_upper'] for x in hs),
 'H_all_local_bounds_below_u64_MAX':all(x['L16']['strictly_below_u64_MAX'] for x in hs),
 'VR_storage_rows':len(vs),'VR_without_counts':len(missing),
 'VR_local_max':max(x['L16']['covered_local_sum_upper'] for x in vs),
 'VR_all_local_bounds_below_u64_MAX':all(x['L16']['strictly_below_u64_MAX'] for x in vs)},indent=2))
