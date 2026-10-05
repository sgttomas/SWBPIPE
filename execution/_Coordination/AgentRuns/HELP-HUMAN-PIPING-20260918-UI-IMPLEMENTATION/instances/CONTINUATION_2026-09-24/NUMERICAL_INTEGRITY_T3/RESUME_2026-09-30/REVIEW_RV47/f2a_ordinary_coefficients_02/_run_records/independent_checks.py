"""RV47 bounded exact ABSTRACT checks. No product import, solver or host probe.
Only standard-library integer/Fraction arithmetic; not a runtime proposal.
"""
from fractions import Fraction as Q
import json

def two(e):
    return Q(1 << e) if e >= 0 else Q(1, 1 << -e)

def binade(x):
    assert x > 0
    e = x.numerator.bit_length() - x.denominator.bit_length()
    return e - (x < two(e))

def directed(x, precision=1024):
    if x == 0:
        return Q(0), Q(0)
    if x < 0:
        a, b = directed(-x, precision)
        return -b, -a
    unit = two(binade(x) - precision + 1)
    scaled = x / unit
    n, r = divmod(scaled.numerator, scaled.denominator)
    return n * unit, (n + (r != 0)) * unit

def nearest(x, precision=53):
    # The fixed discriminator stays normal; no binary64 subnormal emulation claimed.
    if x == 0:
        return Q(0)
    if x < 0:
        return -nearest(-x, precision)
    unit = two(binade(x) - precision + 1)
    v = x / unit
    q, r = divmod(v.numerator, v.denominator)
    q += (2*r > v.denominator) or (2*r == v.denominator and q % 2 == 1)
    return q * unit

def source(t0, t, t1, x0, x1):
    assert t0 < t < t1
    terms = (t1*x0, -t*x0, t*x1, -t0*x1)
    n = sum(terms, Q(0))
    assert n == (t1-t)*x0 + (t-t0)*x1
    return n, t1-t0, terms

def enclosure(t0, t, t1, x0, x1):
    n, h, terms = source(t0,t,t1,x0,x1)
    if n <= 0:
        return None
    lo_n, hi_n = directed(n)
    lo_h, hi_h = directed(h)
    lo = directed(lo_n/hi_h)[0]
    hi = directed(hi_n/lo_h)[1]
    assert 0 < lo <= n/h <= hi
    return lo, hi

def resolved(t0,t,t1,x0,x1):
    f = nearest(nearest(t-t0)/nearest(t1-t0))
    return nearest(x0 + nearest(f * nearest(x1-x0)))

checks=[]
count=0
for t0 in map(Q,[-8,0,1]):
    for h in map(Q,[3,17,49]):
        for step in [Q(1),h-Q(1)]:
            for x0 in [Q(-7),Q(-1),Q(0),Q(1),two(-1074)]:
                for x1 in [Q(-2),Q(0),Q(3),Q(18),two(1023)]:
                    n,den,_=source(t0,t0+step,t0+h,x0,x1)
                    got=enclosure(t0,t0+step,t0+h,x0,x1)
                    assert (got is not None) == (n>0)
                    count += 1
checks.append({'name':'four_product_identity_sign_and_positive_outward_quotient','cases':count})
assert enclosure(Q(0),Q(1),Q(2),Q(-1),Q(3)) == (Q(1),Q(1))
checks.append({'name':'negative_E_endpoint_with_positive_selected_E_is_not_refused'})
args=tuple(map(Q,[0,7,25,-7,18]))
n,h,_=source(*args)
hat=resolved(*args)
assert n == 0 and hat == two(-50) and enclosure(*args) is None
checks.append({'name':'independent_integer_RN64_source_zero_discriminator','source':'0','rounded':str(hat)})
maximum=two(1024)-two(971)
minimum=two(-1074)
n,h,terms=source(-maximum,minimum,maximum,maximum,minimum)
assert all((x/two(-2148)).denominator==1 and abs(x)<two(2048) for x in terms)
assert n>0 and n<two(2050) and (n/two(-2148)).denominator==1
assert two(-1074)<=h<two(1025)
assert enclosure(-maximum,minimum,maximum,maximum,minimum) is not None
assert 2048+2148==4196 and 2050+2148==4198 and 2051+2148==4199<8128
checks.append({'name':'extreme_abstract_dyadic_span_and_local_positivity','span_upper':4199})
Eargs=tuple(map(Q,[0,7,25,-7,19]))
Gargs=tuple(map(Q,[0,7,25,40,60]))
HE=enclosure(*Eargs); HG=enclosure(*Gargs)
eh=resolved(*Eargs); gh=resolved(*Gargs)
HE=(min(HE[0],eh),max(HE[1],eh)); HG=(min(HG[0],gh),max(HG[1],gh))
assert 0<HE[0]<=eh<=HE[1] and 0<HG[0]<=gh<=HG[1]
A=(Q(3),Q(4)); I=(Q(5),Q(6)); J=(Q(10),Q(12))
products=[]
for M,S in [(HE,A),(HG,J),(HE,I),(HE,I)]:
    bound=(directed(M[0]*S[0])[0],directed(M[1]*S[1])[1])
    for m in [M[0],sum(M)/2,M[1]]:
        for s in [S[0],sum(S)/2,S[1]]:
            assert bound[0]<=m*s<=bound[1]
    products.append(bound)
checks.append({'name':'resolved_value_hulls_and_positive_coefficient_rectangles'})
k=Q(3); x=Q(1,3); d=Q(1,100); beta=Q(2,3)
tau=beta*d*abs(x)/(1-beta*d)
for beam in [Q(2)-d,Q(2),Q(2)+d]:
    u=1/(beam+1)
    assert abs(u-x)<=tau
    assert abs(beam*u-2*x)<=(2+d)*tau+d*abs(x)
checks.append({'name':'closed_scalar_I33_response_and_recovery_change'})
operand=1+two(-52)
exact=operand*operand
rounded=nearest(exact)
assert exact-rounded==two(-104)
assert max(abs(rounded-rounded),abs(rounded-rounded))==0
assert max(abs(rounded-exact),abs(rounded-exact))==two(-104)
checks.append({'name':'RV47_C1_exact_K_product_rounding_sensitive_counterexample','exact_minus_rounded':str(two(-104)),'wrong_delta':'0'})
R=(Q(-5),Q(-2)); S=(Q(-7),Q(1)); H=(min(R[0],S[0]),max(R[1],S[1]))
for y in [Q(-9),Q(-1),Q(0),Q(4)]:
    for factor in [Q(1),Q(1000),Q(1,10**6)]:
        distance=lambda interval: max(abs(y-factor*z) for z in interval)
        assert distance(H)==max(distance(R),distance(S))
checks.append({'name':'represented_source_hull_distances_preserve_constituent_bounds'})
assert 2+8+4+4==18 and 3*(18-4)+2==44
assert 1182+44==1226 and 2+32==34
assert 7*(18+128)+16==1038
assert 7*256==1792 and 6*128==768 and 2*128==256
assert 1038+1792+768+256==3854<4096
assert 9+4+2+2+4==21 and (21+2+2)*137==3425
checks.append({'name':'operation_sumwork_and_logical_scratch_arithmetic'})
print(json.dumps({'status':'PASS for independent abstract implications; RV47-C1 evidence defect confirmed','groups':checks,'limits':'No product admission, complete arithmetic implementation, layout, tariff or availability evidence.'},indent=2))
