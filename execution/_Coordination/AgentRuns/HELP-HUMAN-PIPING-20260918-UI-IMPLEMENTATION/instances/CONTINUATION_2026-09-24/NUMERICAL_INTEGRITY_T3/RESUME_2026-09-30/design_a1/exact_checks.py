"""Bounded A1 design arithmetic, stdlib only; no solver/oracle imports."""
from fractions import Fraction as F
import json
import struct

def p2(n):
    return F(2**n) if n >= 0 else F(1, 2**-n)

def bits(x):
    return struct.unpack('>Q', struct.pack('>d', x))[0]

def rn(x):
    return F.from_float(float(x))

def ru(x):
    nearest = float(x)
    if F.from_float(nearest) < x:
        nearest = struct.unpack('>d', struct.pack('>Q', bits(nearest)+1))[0]
    return F.from_float(nearest)

def text(x):
    return str(x.numerator) + '/' + str(x.denominator)

def coupled(a, length, rounded=False):
    t,r,f,m = a
    op = rn if rounded else lambda x:x
    if length == 0:
        return a
    return [max(t,op(length*r)), max(r,op(t/length)),
            max(f,op(m/length)), max(m,op(length*f))]

def bound(x,s):
    base = ru(p2(-64)*s)
    return ru(base + ru(p2(-53)*abs(x)) + p2(-1074)) if 0<s<p2(-988) else base

h=p2(-1074)
checks=[]
def check(name, condition, **values):
    assert condition, name
    checks.append({'name':name,'pass':True,**{k:text(v) if isinstance(v,F) else v for k,v in values.items()}})

# Same lost h/4 operand in each of the four scale directions.
for label,index,dest,length in [('rotation_to_translation',1,0,p2(100)),
                              ('translation_to_rotation',0,1,p2(-100)),
                              ('moment_to_force',3,2,p2(-100)),
                              ('force_to_moment',2,3,p2(100))]:
    v=[F(0)]*4;v[index]=5*h/4
    x=[rn(y) for y in v]
    sv=coupled(v,length);sp=coupled(x,length,True)
    gap=sv[dest]-sp[dest]
    check(label, gap==p2(-976) and gap/sv[dest]==F(1,5),gap=gap,relative_gap=gap/sv[dest])

truth=9*p2(-1041); b=p2(-1038)
check('C17_frozen_force_balance',9*h/p2(-33)==truth,truth=truth)
check('C17_false_qualified_claim',truth/b==F(9,8) and truth>b*(1+p2(-22)),error_over_b=truth/b)
check('direct_certificate_rejects_C17_even_with_exact_verification',abs(F(0)-truth)>b,certificate=truth,bound=b)

# Actual retained verification may be different: any E that encloses truth
# still makes |x-v|+E at least |x-truth|. Exercise both signs and cancellation.
for v in [F(0),truth,2*truth,-truth]:
    e=abs(v-truth)
    check('C17_triangle_'+text(v),abs(v)+e>=truth and abs(v)+e>b,certificate=abs(v)+e)

v=[F(0),5*h/4,F(0),F(0)]
sv=coupled(v,p2(-100));sp=coupled([rn(y) for y in v],p2(-100),True)
check('zero_published_coupled_scale',sv[0]>0 and sp[0]==0,verification_scale=sv[0],published_scale=sp[0])
check('zero_bound_positive_certificate_refuses',bound(F(0),F(0))==0 and p2(-2000)>0,certificate=p2(-2000))
check('zero_bound_zero_certificate_passes',bound(F(0),F(0))==0)
check('small_scale_own_rounding',bound(h,h)==3*h,bound=bound(h,h),direct_rounding_error=h/4)
for exponent in [-989,-988]:
    s=p2(exponent)
    check('branch_'+str(exponent),bound(F(0),s)==(ru(p2(-64)*s)+h if exponent < -988 else ru(p2(-64)*s)),scale=s,bound=bound(F(0),s))

ehat=p2(100)
phi=ru(p2(-438)*ehat)
bp=ru(p2(-64)*phi)
check('p512_floor_budget',bp==p2(-502)*ehat and 69*p2(-1024)*ehat<bp,phi=phi,bound=bp,recovery_term=69*p2(-1024)*ehat)
check('p512_old_charge_can_exceed_bare_bound',bp*(1+p2(-22))>bp,qualified_candidate=bp*(1+p2(-22)),bare_bound=bp)

# An illustrative ordinary nonzero absolute row with direct publication
# rounding and conservative verification error; not a solver selection.
s=F(1);v=p2(-35)+p2(-90);x=rn(v);e=p2(-128)
check('ordinary_absolute_arithmetic_can_pass',abs(x-v)+e<=bound(x,s),certificate=abs(x-v)+e,bound=bound(x,s))

# A nonzero value rounded away after a unit conversion cannot be called exact.
conversion_scale=p2(-100);v=h;converted=rn(conversion_scale*v)
conversion_error=abs(converted-conversion_scale*v)
check('converted_zero_requires_positive_radius',converted==0 and conversion_error>0,conversion_error=conversion_error)

print(json.dumps({'status':'PASS','scope':'exact arithmetic design checks only; no solver or product run',
                  'checks':checks},indent=2))
