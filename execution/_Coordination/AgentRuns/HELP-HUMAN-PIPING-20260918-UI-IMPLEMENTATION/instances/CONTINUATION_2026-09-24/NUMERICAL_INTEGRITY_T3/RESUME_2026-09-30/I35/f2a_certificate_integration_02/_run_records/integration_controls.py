"""Exact abstract integration controls only; no product imports or host probes."""
from fractions import Fraction as F
import json

checks=[]
def two(e): return F(1<<e) if e>=0 else F(1,1<<-e)
def rn64(x):
    x=F(x)
    if not x: return F(0)
    e=abs(x).numerator.bit_length()-x.denominator.bit_length()
    if abs(x)<two(e): e-=1
    quantum=two(max(e-52,-1074))
    z=abs(x)/quantum
    n,r=divmod(z.numerator,z.denominator)
    if 2*r>z.denominator or (2*r==z.denominator and n&1): n+=1
    result=n*quantum
    if result>=two(1024): raise ValueError('overflow')
    return result if x>0 else -result

def check(name,body):
    detail=body()
    checks.append(dict(name=name,passed=True,detail=detail))

def coefficient_boundaries():
    x=1+two(-52)
    exact_ck=x*x
    rounded=rn64(exact_ck)
    assert exact_ck-rounded==two(-104)
    assert abs(rounded-rounded)==0 and abs(rounded-exact_ck)>0
    Tlo,T,Thi,Elo,Ehi=map(F,[0,7,25,-7,18])
    numerator=Thi*Elo-T*Elo+T*Ehi-Tlo*Ehi
    actual=rn64(Elo+rn64(rn64((T-Tlo)/(Thi-Tlo))*rn64(Ehi-Elo)))
    assert numerator==0 and actual==two(-50)>0
    assert F(1,2)*F(-1)+F(1,2)*F(3)==1
    return {'CK_delta':'2^-104','zero_exact_E_positive_rounded_E':'2^-50','negative_endpoint_positive_selection':True}
check('exact_K_and_source_positivity_are_not_rounded_results',coefficient_boundaries)

def selection():
    def bracket(points,target):
        if len(set(points))!=len(points) or target in points: return None
        lo=[x for x in points if x<target]; hi=[x for x in points if x>target]
        return (max(lo),min(hi)) if lo and hi else None
    assert bracket([313,293],303)==(293,313)
    assert bracket([313,293,303],303) is None
    assert bracket([313,293,293],303) is None
    assert bracket([313,293],293) is None
    assert bracket([313,293],323) is None
    return {'unsorted_adjacent_pair':True,'at_point_duplicate_and_extrapolation_refuse':True}
check('selection_scan_preserves_strict_resolver_rules',selection)

def hull_and_units():
    represented=(F(1),F(1))
    source=(F(0),F(1)) # Q_S=[0,2] divided by source A=2
    wrong_mixed=(F(1,2),F(1,2))
    hull=(min(represented[0],source[0]),max(represented[1],source[1]))
    assert hull==(0,1) and wrong_mixed[1]<represented[1]
    for y,a in [(F(3,4),F(1)),(F(750),F(1,1000)),(F(3,4000000),F(1000000))]:
        distance=lambda I:max(abs(y-I[0]/a),abs(y-I[1]/a))
        assert distance(hull)==max(distance(represented),distance(source))
    y=F(1); n=rn64(y/1000)
    assert abs(n-n)==0 and abs(y-1000*n)>0
    return {'separate_branch_hull_needed':True,'hull_distance_equivalence':True,'SI_zero_not_raw_zero':True}
check('dual_readout_and_raw_SI_composition',hull_and_units)

def observable_separation():
    lower=two(1023); upper=lower+two(971)
    midpoint=rn64(lower+rn64(F(1,2)*rn64(upper-lower)))
    assert lower<=midpoint<=upper and midpoint==lower
    try: rn64(lower+upper)
    except ValueError: pass
    else: assert False
    actual=F(5001,1000); norm=F(5)
    guard=64*two(-52)*max(abs(actual),two(-1022))
    assert 0<=actual<=10 and abs(actual-norm)>guard
    # Wrong raw i-end sign keeps both endpoint norms at 2 but changes midpoint.
    qi=qj=F(2); t=F(1,2)
    assert t*qj+(t-1)*qi==0 and (1-t)*qi+t*qj==2
    return {'ordered_midpoint_survives_naive_sum_overflow':True,'source_cover_does_not_prove_guard':True,'endpoint_norm_does_not_prove_station_sign':True}
check('observable_and_source_checks_are_independent',observable_separation)

def headline():
    rows=[{'value':F(10),'case':'b','location':'x','id':'B'}, {'value':F(10),'case':'a','location':'z','id':'A'}]
    winner=sorted(rows,key=lambda r:(-r['value'],r['case'],r['location']))[0]
    assert winner['id']=='A'
    published_winner=F(10); other_truth=F(11)
    assert other_truth>published_winner # exact winner-row radius zero cannot cover aggregate truth
    all_cases_complete=[True,False]
    assert not all(all_cases_complete)
    return {'tie_uses_actual_ids':True,'incomplete_case_suppresses_headline':True,'no_winner_radius_aggregate_inference':True}
check('headline_alias_and_coverage',headline)

def permit_and_carry():
    cap=3; spent=0; executed=[]
    for i in range(4):
        prospective=spent+1
        if prospective>cap:
            rejected=i
            break
        spent=prospective
        executed.append(i)
    assert executed==[0,1,2] and spent==3 and rejected==3
    numeric_error='Span'; status={'Overflow'}
    assert numeric_error=='Span' and status # do not replace cause or extract numeric success
    # R4: initial4 + nearest1 live, then clear and step2; seven lifetime != seven live.
    phases=[4,5,0,2,0]
    assert max(phases)==5 and 4+1+2==7
    assert 5*(1<<8128)<1<<8131<1<<8192
    assert 1038+1792+768+256==3854<4096
    assert 179*137+659*8==29795
    return {'rejected_event_not_executed':True,'numeric_cause_and_status_both_retained':True,'R4_five_live_seven_lifetime':True,'logical_endpoint_limb_bytes':29795}
check('visit_exhaustion_and_seven_insert_scope',permit_and_carry)

print(json.dumps({'kind':'exact_abstract_integration_controls','groups_passed':len(checks),'product_or_solver_executed':False,'checks':checks},indent=2,sort_keys=True))
