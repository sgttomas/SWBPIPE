#!/usr/bin/env python3
"""Finite B1C exact design arithmetic. No product imports or model/solver calls.
Fixed Machin proof, source-geometry intervals, and algebraic bridge discriminator.
All output is stdout; source fixture is read only. Standard library only.
"""
from fractions import Fraction as F
from pathlib import Path
import json
import struct

def p2(e):
    return F(1 << e) if e >= 0 else F(1,1 << -e)
def bits(x):
    return struct.unpack(">Q",struct.pack(">d",x))[0]
def dec(b):
    s=-1 if b>>63 else 1
    e=(b>>52)&2047; m=b&((1<<52)-1)
    if e==2047: raise ValueError("nonfinite")
    return s*(m*p2(-1074) if e==0 else ((1<<52)+m)*p2(e-1075))
def rndint(q):
    n,d=q.numerator,q.denominator
    a,r=divmod(n,d)
    return a+(2*r>d or (2*r==d and a&1))
def rn_bits(q):
    if not q: return 0
    s=(1<<63) if q<0 else 0; q=abs(q)
    e=q.numerator.bit_length()-q.denominator.bit_length()
    if q<p2(e): e-=1
    k=max(e-52,-1074); m=rndint(q/p2(k))
    if not m: return s
    if m==1<<53: m>>=1; k+=1
    if m<1<<52: return s|m
    biased=k+1075
    if biased>=2047: raise OverflowError("nonfinite")
    return s|(biased<<52)|(m-(1<<52))
def rn(q): return dec(rn_bits(q))
def floor(q): return q.numerator//q.denominator
def ceil(q): return -floor(-q)
def fstr(q):
    return str(q.numerator)+"/"+str(q.denominator)
def interval(q): return [fstr(q[0]),fstr(q[1])]

N=128
def atan_bounds(q):
    # N even: partial sum below atan(1/q); next term bounds positive tail.
    assert N%2==0 and q>1
    s=sum((F((-1)**k,(2*k+1)*q**(2*k+1)) for k in range(N)),F())
    e=F(1,(2*N+1)*q**(2*N+1))
    return s,s+e
a,b=atan_bounds(5),atan_bounds(239)
pi_series=(16*a[0]-4*b[1],16*a[1]-4*b[0])
assert 0<pi_series[1]-pi_series[0]<p2(-600)
P_LO=floor(pi_series[0]*p2(512))
P_HI=ceil(pi_series[1]*p2(512))
PI=(F(P_LO,1<<512),F(P_HI,1<<512))
assert PI[0]<=pi_series[0]<pi_series[1]<=PI[1]
assert P_HI==P_LO+1
assert F(3)<PI[0]<PI[1]<F(4)
# Exact tangent identity: tan(4 atan(1/5)-atan(1/239))=1.
t2=(2*F(1,5))/(1-F(1,25))
t4=(2*t2)/(1-t2*t2)
assert t2==F(5,12) and t4==F(120,119)
assert (t4-F(1,239))/(1+t4/F(239))==1

def geom(D,t):
    assert D>0 and 0<t<D/2
    c=D/2; ri=c-t; P=t*(D-t); Q=c*c+ri*ri
    A=(PI[0]*P,PI[1]*P)
    I=(A[0]*Q/4,A[1]*Q/4)
    J=(2*I[0],2*I[1])
    Z=(I[0]/c,I[1]/c)
    assert c>0 and ri>0 and all(0<x[0]<=x[1] for x in (A,I,J,Z))
    for pi in PI:
        # Equivalent section integral; proves algebraic formula identity.
        assert pi*P==pi*(c*c-ri*ri)
        assert pi*P*Q/4==pi*(c**4-ri**4)/4
    return {"c":(c,c),"ri":(ri,ri),"A":A,"I":I,"J":J,"Z":Z}
def quotient(X,D):
    assert D[0]>0
    c=[x/d for x in X for d in D]
    return min(c),max(c)
def tor(X,c,J):
    return min(x*r/j for x in X for r in c for j in J),max(x*r/j for x in X for r in c for j in J)
def dyadic_width(q):
    if not q: return 0
    assert q.denominator&(q.denominator-1)==0
    m=abs(q.numerator)
    while not(m&1): m>>=1
    return m.bit_length()
def geometry_widths(D,t,g):
    c=D/2; ri=c-t; P=t*(D-t); Q=c*c+ri*ri; G=P*Q
    return {name:dyadic_width(q) for name,q in {
        "D_minus_t":D-t,"ri":ri,"P":P,"Q":Q,"G":G,
        "A_lower":g["A"][0],"A_upper":g["A"][1],
        "I_lower":g["I"][0],"I_upper":g["I"][1]
    }.items()}

# Find repository through fixed ancestor relation, not an environment/tool probe.
root=Path(__file__).resolve()
while root.name!="projects": root=root.parent
repo=root.parent
fixture=repo/"projects/chirality-piping/core/product_physics/tests/fixtures/pressure_reference/SOURCE_ODWALL_EXPECTATIONS.json"
ref=json.loads(fixture.read_text())
controls=[]
for case in ref["cases"]+[{"id":"representable_range_control",**ref["representable_range_control"]}]:
    D=F.from_float(case["inputs"]["OD_m"]); t=F.from_float(case["inputs"]["wall_m"])
    g=geom(D,t)
    # Fixture uses a finite Machin approximation, not exact transcendental pi.
    ref_A=F(case["A_m2"]["exact_rational_with_bounded_pi"])
    pi_ref=ref_A/(t*(D-t))
    fixture_error=F(3,10**142)
    assert PI[0]>=pi_ref-fixture_error and PI[1]<=pi_ref+fixture_error
    row={"name":"source_fixture_"+case["id"],"D_bits":f"{bits(case['inputs']['OD_m']):016x}",
         "t_bits":f"{bits(case['inputs']['wall_m']):016x}",
         "positive":True,"fixture_pi_error_warrant_contains_new_bracket":True,
         "max_actual_integer_width":max(geometry_widths(D,t,g).values()),
         "property_interval_relative_width":fstr((g["A"][1]-g["A"][0])/g["A"][0])}
    # Every property is pi times a positive fixed rational, so same relative width.
    assert all((g[k][1]-g[k][0])/g[k][0]==(PI[1]-PI[0])/PI[0] for k in ("A","I","J","Z"))
    # Stress inputs are abstract action intervals, not solver-generated rows.
    actions=(F(-1),F(2))
    for name,den in [("N/A","A"),("M/Z","Z")]:
        enc=quotient(actions,g[den])
        for q in (F(-1),F(0),F(2)):
            for pi in (PI[0],sum(PI)/2,PI[1]):
                property_q=g[den][0]*pi/PI[0]
                assert enc[0]<=q/property_q<=enc[1]
    tors=tor(actions,g["c"],g["J"])
    for q in (F(-1),F(0),F(2)):
        for pi in (PI[0],sum(PI)/2,PI[1]):
            J=g["J"][0]*pi/PI[0]
            assert tors[0]<=q*g["c"][0]/J<=tors[1]
    controls.append(row)

# Abstract numerical boundaries: these are not claimed product-admitted cases.
h=p2(-1074)
abstract=[("subnormal_dimensions",4*h,h),("near_solid",F(2),1-p2(-53)),
          ("max_span",dec(0x7fefffffffffffff),h),("tiny_wall",F(1),p2(-1000))]
max_width=0
for name,D,t in abstract:
    g=geom(D,t); widths=geometry_widths(D,t,g)
    max_width=max(max_width,max(widths.values()))
    assert widths["P"]<=2151 and widths["Q"]<=4200 and widths["G"]<=6351
    assert max(widths["A_lower"],widths["A_upper"])<=2665
    assert max(widths["I_lower"],widths["I_upper"])<=6865
    controls.append({"name":name,"product_admission_claim":False,"integer_widths":widths})

# Preserve represented effective-wall boundary; do not strengthen to exact nominal minus tolerance.
D,tnom,m=F(4),F(1),p2(-55)
t_hat=rn(tnom-m)
assert t_hat==1 and t_hat != tnom-m
g_hat=geom(D,t_hat); g_unrounded=geom(D,tnom-m)
assert g_hat["A"][0]>g_unrounded["A"][1]
controls.append({"name":"effective_wall_is_actual_normalized_effective_bit",
                 "normalized_nominal":"1","normalized_tolerance":fstr(m),
                 "effective_bit":f"{rn_bits(t_hat):016x}",
                 "unrounded_subtraction":fstr(tnom-m),
                 "area_truths_disjoint":True})

# Stored-bit singleton cannot be an exact real-pi geometric area.
ga=geom(F(4),F(1))
ahat=rn(sum(ga["A"])/2)
assert not(ga["A"][0]<=ahat<=ga["A"][1])
controls.append({"name":"rounded_area_is_not_source_area",
                 "rounded_reference_area_bits":f"{rn_bits(ahat):016x}",
                 "rounded_value_outside_source_interval":True,
                 "actual_SourceAnnulus_execution":False})

# Analytic stiffness-partition discriminator, not a product model or solver.
# Equal E/L, geometric areas 3*pi and p*pi; finite coefficient controls only.
g1=geom(F(4),F(1)); a1=rn(sum(g1["A"])/2)
bridge_witness=None
for p in range(3,34):
    g2=geom(F(p+1),F(1)); a2=rn(sum(g2["A"])/2)
    qk=a1/(a1+a2); qg=F(3,3+p)
    if qk != qg:
        bridge_witness=(p,a2,qk,qg)
        break
assert bridge_witness is not None
p,a2,qk,qg=bridge_witness
controls.append({"name":"action_bridge_not_given_by_denominator_enclosure",
                 "scope":"exact scalar coefficient identity only, no product model",
                 "q_K":fstr(qk),"q_G":fstr(qg),"difference":fstr(qk-qg),
                 "geometric_area_coefficients":[3,p],
                 "Ahat1_bits":f"{rn_bits(a1):016x}","Ahat2_bits":f"{rn_bits(a2):016x}"})

# Geometric zero action remains exact without demanding a zero-width pi bracket.
assert quotient((F(0),F(0)),ga["A"])==(0,0)
assert tor((F(0),F(0)),ga["c"],ga["J"])==(0,0)
controls.append({"name":"zero_action_geometry_uncertainty_does_not_create_stress","checks":2})

bad=[(F(0),F(1)),(F(1),F(0)),(F(2),F(1)),(F(2),F(3,2))]
for D,t in bad:
    try:
        geom(D,t)
    except AssertionError:
        pass
    else:
        raise AssertionError("invalid annulus admitted by design formula")
controls.append({"name":"positive_annulus_preconditions","refused_pairs":4})

print(json.dumps({
    "scope":"finite standard-library exact design arithmetic; no runtime/product/solver execution",
    "all_assertions_passed":True,
    "pi_proof":{"identity":"pi=16 atan(1/5)-4 atan(1/239)","terms_each":N,
       "series_width_less_than":"2^-600","constant_denominator_power":512,
       "lower_numerator_hex":hex(P_LO),"upper_numerator_hex":hex(P_HI),
       "constant_width":"2^-512",
       "proof_fraction_max_bits":max(x.numerator.bit_length() for x in pi_series)+0,
       "proof_denominator_max_bits":max(x.denominator.bit_length() for x in pi_series)},
    "geometry_symbolic_significand_bounds":{"P":2151,"Q":4200,"G":6351,"A":2665,"I_J":6865,"Z_denominator":53},
    "geometry_schedule":{"subtractions":2,"additions":1,"multiplications":8,
       "power_of_two_exponent_adjustments":"fixed count, no numeric rounding",
       "grade_school_u64_limb_product_ceiling":5779,
       "runtime_series_or_refinement_iterations":0,
       "counts_are_not_LME_prices":True},
    "control_groups":len(controls),"controls":controls,
    "source_action_bridge_proved":False
},indent=2))
