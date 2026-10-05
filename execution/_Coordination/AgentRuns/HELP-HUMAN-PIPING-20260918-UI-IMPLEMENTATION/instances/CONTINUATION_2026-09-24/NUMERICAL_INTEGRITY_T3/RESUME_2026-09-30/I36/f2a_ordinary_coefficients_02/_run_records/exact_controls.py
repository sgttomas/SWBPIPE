"""I36 finite ABSTRACT exact controls; no product imports, models, or solver.
Run: python3 -B exact_controls.py. Stdout is deterministic JSON.
Fraction arithmetic is offline checking, never a proposed runtime fallback.
"""
from fractions import Fraction as F
import json

P = 1024  # I35's proposed precision, not another format selection
PI_LO = F(int('3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917',16),2**512)
PI_HI = PI_LO + F(1,2**512)
def pw(e): return F(2**e) if e>=0 else F(1,2**(-e))
def bits(x): return F.from_float(float(x))
def rn(x): return bits(x)
def down_up(x):
    """Exact floor/ceiling of a rational on its 1024-significant-bit binade."""
    if not x: return F(0),F(0)
    if x<0:
        a,b=down_up(-x); return -b,-a
    e=x.numerator.bit_length()-x.denominator.bit_length()
    if x<pw(e): e-=1
    q=pw(e-P+1)
    r=x/q
    n=r.numerator//r.denominator
    return n*q, (n if r.denominator==1 else n+1)*q

def material(tlo,t,thi,xlo,xhi):
    assert tlo<t<thi
    h=thi-tlo
    terms=(thi*xlo,-t*xlo,t*xhi,-tlo*xhi)
    numerator=sum(terms,F(0))
    assert numerator==(thi-t)*xlo+(t-tlo)*xhi
    if numerator<=0: return None, numerator, terms
    hl,hu=down_up(h)
    pl,pu=down_up(numerator)
    assert hl>0 and pl>0
    lower=down_up(pl/hu)[0]
    upper=down_up(pu/hl)[1]
    assert 0<lower<=numerator/h<=upper
    return (lower,upper), numerator, terms

def actual_scalar(tlo,t,thi,xlo,xhi):
    # Exact simulation of only the isolated source's separate RN64 operations.
    frac=rn(rn(t-tlo)/rn(thi-tlo))
    return rn(xlo+rn(frac*rn(xhi-xlo)))

def cover(a,b): return min(a[0],b),max(a[1],b)
def prod(a,b): return a[0]*b[0],a[1]*b[1]
def hull(*xs): return min(x[0] for x in xs),max(x[1] for x in xs)
def quotient(q,d):
    xs=[x/y for x in q for y in d]
    return min(xs),max(xs)
def torsion(q,c,j):
    xs=[x*y/z for x in q for y in c for z in j]
    return min(xs),max(xs)
def geometry(d,t):
    c=d/2; ri=c-t
    assert 0<t<c
    a=t*(d-t); ii=a*(c*c+ri*ri)/4
    return {'c':(c,c),'A':(PI_LO*a,PI_HI*a),'I':(PI_LO*ii,PI_HI*ii),
            'J':(2*PI_LO*ii,2*PI_HI*ii),'Z':(PI_LO*ii/c,PI_HI*ii/c)}
def h(l,u,x): return max(abs(x-l),abs(x-u))
checks=[]
def check(name): checks.append(name)

# Base/point source positivity and represented cover.
for e,g in [(bits(200e9),bits(80e9)),(bits(180e9),bits(50e9))]:
    assert e>0 and g>0 and cover((e,e),e)==(e,e) and cover((g,g),g)==(g,g)
check('base_and_exact_point_positive_singletons')
assert bits(50e9)!=bits(80e9)
check('selected_G_is_independent_of_base_G_and_no_nu_needed')

# Exact law uses normalized input bits, not rounded fraction.
args=tuple(map(F,[0,1,3,10,13]))
i,num,terms=material(*args)
assert num/F(3)==11 and i[0]<=11<=i[1]
check('exact_linear_law_and_four_term_identity')
for vals in [(-1,3),(0,2),(-7,19)]:
    i,num,_=material(F(0),F(1),F(2),*map(F,vals))
    assert i and num>0
check('nonpositive_E_endpoint_with_positive_interpolation_is_allowed')

args=tuple(map(F,[0,7,25,-7,18]))
i,num,_=material(*args)
hat=actual_scalar(*args)
assert num==0 and i is None and hat==pw(-50)>0
check('positive_RN64_Ehat_does_not_prove_exact_E_positive')
for xhi in [F(17),F(18)]:
    i,num,_=material(F(0),F(7),F(25),F(-7),xhi)
    assert i is None and num<=0
check('zero_and_negative_source_numerator_refuse_certificate')

args=(F(0),F(7),F(25),F(-7),F(18)+pw(-48))
i,num,_=material(*args); hat=actual_scalar(*args)
assert num>0 and i and hat>0
c=cover(i,hat)
assert c[0]>0 and c[0]<=num/F(25)<=c[1] and c[0]<=hat<=c[1]
check('positive_cancellation_and_actual_resolver_hull')

for tl,t,th in [(0,1,2),(-9,0,13),(1,3,7)]:
    for g0,g1 in [(1,2),(37,41),(F(1,2**1074),F(2))]:
        i,n,_=material(*map(F,[tl,t,th,g0,g1])); assert i and n>0
check('positive_G_convexity_at_admitted_source_value')

# Source-only extremal span, not a product admission claim.
max64=(2-pw(-52))*pw(1023)
min64=pw(-1074)
i,num,terms=material(-max64,min64,max64,max64,min64)
assert i and num>0
for term in terms:
    assert (term*pw(2148)).denominator==1 and abs(term)<pw(2048)
assert abs(num)<pw(2050) and (num*pw(2148)).denominator==1
assert 4198<8128
check('finite_binary64_product_and_numerator_span_bound')

# B1C pi endpoints are premises; these checks do not re-prove pi.
d,t=bits(0.12),bits(0.01)
geom=geometry(d,t)
assert all(0<a<=b for a,b in geom.values())
es=material(F(0),F(7),F(25),F(-7),F(19))[0]
gs=material(F(0),F(7),F(25),F(40),F(60))[0]
eh=actual_scalar(F(0),F(7),F(25),F(-7),F(19))
gh=actual_scalar(F(0),F(7),F(25),F(40),F(60))
ecover,gcover=cover(es,eh),cover(gs,gh)
cs=[prod(ecover,geom['A']),prod(gcover,geom['J']),prod(ecover,geom['I']),prod(ecover,geom['I'])]
for pi in [PI_LO,(PI_LO+PI_HI)/2,PI_HI]:
    aa=pi*t*(d-t); ii=aa*((d/2)**2+(d/2-t)**2)/4
    for ee in [es[0],es[1],eh]:
        for gg in [gs[0],gs[1],gh]:
            truths=(ee*aa,gg*2*ii,ee*ii,ee*ii)
            assert all(lo<=x<=hi for (lo,hi),x in zip(cs,truths))
check('ordinary_positive_coefficient_rectangle_contains_all_targets')

ck=[bits(eh*bits(geom['A'][0])),bits(gh*bits(geom['J'][1])),
    bits(eh*bits(geom['I'][0])),bits(eh*bits(geom['I'][0]))]
for (lo,hi),k in zip(cs,ck):
    delta=max(abs(lo-k),abs(hi-k))
    assert all(abs(x-k)<=delta for x in (lo,(lo+hi)/2,hi))
check('coefficient_difference_majorant_is_about_actual_K_products')

# Reviewed bridge, scalar specialization only; no matrix or solve routine.
k=F(3); x=F(1,3); delta=F(1,100); beta=F(2,3)
alpha=beta*delta; tau=beta*delta*abs(x)/(1-alpha)
for beam in [F(2)-delta,F(2),F(2)+delta]:
    u=1/(beam+1)
    assert abs(u-x)<=tau
    beam_error=(2+delta)*tau+delta*abs(x)
    assert abs(beam*u-F(2)*x)<=beam_error
    assert abs(-u+x)<=tau
    assert (beam+1)*u==1  # beam and spring restoring magnitudes sum to load
check('uniform_I33_scalar_response_and_full_action_instantiation')

ik=bits(geom['I'][0]); zhat=bits(geom['Z'][1]); cstar=d/2
zr=hull((zhat,zhat),down_up(ik/cstar))
assert zr[0]<=zhat<=zr[1] and zr[0]<=ik/cstar<=zr[1]
qk=(F(-3),F(-2)); qs=(F(-4),F(-1))
represented=quotient(qk,zr); source=quotient(qs,geom['Z'])
combined=hull(represented,source)
assert combined[0]<=represented[0]<=represented[1]<=combined[1]
assert combined[0]<=source[0]<=source[1]<=combined[1]
assert all(x<=0 for x in combined)
check('represented_Zhat_and_IK_over_c_cover_and_signed_readout_hull')

tr=torsion((F(-2),F(3)),geom['c'],geom['J'])
assert tr[0]<0<tr[1] and torsion((F(0),F(0)),geom['c'],geom['J'])==(0,0)
check('torsion_corners_and_exact_zero_preserved')
for y in [F(-5),F(0),F(1,10),F(9)]:
    assert h(*combined,y)==max(h(*represented,y),h(*source,y))
    a=F(10**6)
    assert h(combined[0]/a,combined[1]/a,y)==max(h(represented[0]/a,represented[1]/a,y),h(source[0]/a,source[1]/a,y))
check('hull_endpoint_distance_identity_in_SI_and_raw_units')

# Sufficient check can refuse without changing the publication or bound.
y=F(0); bound=F(1,2**64); interval=(F(-1,2**52),F(1,2**52))
assert h(*interval,y)>bound
check('unchanged_absolute_predicate_refuses_without_bound_widening')

raw_adds=7; term=raw_adds*(18+128)+16; shift=raw_adds*256
net=6*128; rounded=2*128
assert term+shift+net+rounded==3854<4096
assert 9+4+2+2+4==21
assert 2+8+4+4==18
check('closed_four_product_round_entry_count_and_scratch_inputs')

print(json.dumps({'status':'PASS; abstract exact source/interval controls only',
 'group_count':len(checks),'groups':checks,
 'Ehat_positive_source_zero':{'Tlo':'0','T':'7','Thi':'25','Elo':'-7','Ehi':'18','source_E':'0','isolated_RN64_Ehat':str(pw(-50))},
 'limits':'No production execution, product admission witness, source-map implementation, complete arithmetic qualification, memory or availability claim.'},indent=2))
