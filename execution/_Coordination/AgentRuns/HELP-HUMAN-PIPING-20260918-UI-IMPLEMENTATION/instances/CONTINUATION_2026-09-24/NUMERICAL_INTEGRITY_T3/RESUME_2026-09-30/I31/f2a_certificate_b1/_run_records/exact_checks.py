#!/usr/bin/env python3
"""B1 finite design arithmetic only. No product import, solver, model or runtime.
Pure integer/Fraction binary64 reference operations; all control sets are finite.
Run from any cwd: python3 <this-file>; stdout is the only output.
"""
from fractions import Fraction as F
import json

SIGN = 1 << 63
INF = 0x7ff0000000000000
MAX = 0x7fefffffffffffff
def p2(e):
    return F(1 << e) if e >= 0 else F(1, 1 << -e)
def dec(b):
    assert 0 <= b < (1 << 64) and (b & INF) != INF
    s = -1 if b & SIGN else 1
    e, m = (b >> 52) & 2047, b & ((1 << 52)-1)
    return s * (m * p2(-1074) if e == 0 else ((1 << 52) + m) * p2(e-1075))
def rni(q):
    n, d = q.numerator, q.denominator
    a, r = divmod(n, d)
    return a + (2*r > d or (2*r == d and a & 1))
def rn_bits(q):
    q = F(q)
    if not q:
        return 0
    sign = SIGN if q < 0 else 0
    q = abs(q)
    e = q.numerator.bit_length() - q.denominator.bit_length()
    if q < p2(e):
        e -= 1
    k = max(e-52, -1074)
    m = rni(q / p2(k))
    if m == 0:
        return sign
    if m == 1 << 53:
        m >>= 1
        k += 1
    if m < 1 << 52:
        assert k == -1074
        return sign | m
    biased = k+1075
    if biased >= 2047:
        raise OverflowError("nonfinite RN64")
    return sign | (biased << 52) | (m-(1 << 52))
def rn(q):
    return dec(rn_bits(q))
def ru(q):
    assert q >= 0
    b = rn_bits(q)
    if dec(b) < q:
        b += 1
    if b >= INF:
        raise OverflowError("no finite upper binary64")
    return dec(b)
def down(q):
    b = rn_bits(q)
    if dec(b) > q:
        b = (b+1) if b & SIGN else (SIGN|1 if b == 0 else b-1)
    return dec(b)
def up(q):
    b = rn_bits(q)
    if dec(b) < q:
        b = (b-1) if b & SIGN else b+1
    if (b & INF) == INF:
        raise OverflowError("no finite upper endpoint")
    return dec(b)
def interval(x, r):
    assert r >= 0
    return x-r, x+r
def quotient(Q, A):
    assert 0 < A[0] <= A[1]
    c = [q/a for q in Q for a in A]
    return min(c), max(c)
def torsion(T, R, J):
    assert 0 < R[0] <= R[1] and 0 < J[0] <= J[1]
    c = [t*r/j for t in T for r in R for j in J]
    return min(c), max(c)
def hs(y, a, I):
    assert a > 0
    n = rn(y*a)  # exact map; mm uses division by 1000 (= exact rational map)
    hn = max(abs(n-s) for s in I)
    hu = max(abs(y-s/a) for s in I)
    return n, hn, hu
eps, u, h, small = p2(-64), p2(-53), p2(-1074), p2(-988)
def allowance(z, S):
    ae = eps*max(abs(z),S)*(1+p2(-21)) + u*abs(z) + h
    af = rn(rn(rn(eps*max(abs(z),S))*(1+p2(-21))) + rn(rn(u*abs(z))+h))
    return ae, af
def bound(z, S):
    b = ru(eps*S)
    if 0 < S < small:
        b = ru(b+ru(u*abs(z))+h)
    return b
records = []
def record(name, **facts):
    records.append({"name":name, **facts})
def fmt(q):
    q=F(q)
    return {"numerator":str(q.numerator), "denominator":str(q.denominator)}
def hx(q):
    return f"{rn_bits(q):016x}"

# RN64 self-controls at ties, subnormal boundaries and powers, independently exact.
assert rn(p2(-1075)) == 0
assert rn(3*p2(-1075)) == 2*h
assert rn(1+p2(-53)) == 1
assert rn(1+3*p2(-53)) == 1+p2(-51)
assert rn(dec(0x0010000000000000)) == p2(-1022)
assert ru(h/4) == h and ru(F(0)) == 0
record("binary64_reference_controls", checks=7)

# Identity certificates; all operations and truth are design values, no solve reach.
x = dec(0x3fb999999999999a)
for label, a in [("mm", F(1,1000)), ("kN",F(1000)), ("MPa",F(1000000))]:
    y = rn(x/a)
    n, hn, hu = hs(y,a,(x,x))
    ep, en = abs(a*y-x), abs(n-a*y)
    assert hn == abs(n-x)
    assert hu == ep/a
    assert hn <= ep+en
    assert hu >= abs(y-x/a)
    record("identity_"+label, x=hx(x),y=hx(y),n=hx(n),
           e_pub_SI=fmt(ep),e_norm_SI=fmt(en),H_n=fmt(hn),H_U=fmt(hu))
assert rn(rn(x*1000)/1000) == x
assert abs(F(100,1000)-x) == F(1,180143985094819840)
assert abs(F(100)-1000*x) == F(25,4503599627370496)
assert abs(F(100)-1000*x) > 0

# A zero-SI-radius point may need a raw interval: x/0.001 is nonrepresentable.
lo, hi = down(x*1000), up(x*1000)
assert lo < x*1000 < hi
record("zero_bound_raw_point_needs_interval", lo=hx(lo),hi=hx(hi))

# Signed map and support-style exact linear combination.
xp, rp = F(7), F(1,8)
I = (-xp-rp,-xp+rp)
n, hn, hu = hs(F(-7),F(1),I)
assert hn == rp and hu == rp
terms = [(F(1),F(2),F(1,8)),(F(-1),F(3),F(1,16))]
center=sum(c*q for c,q,r in terms); radius=sum(abs(c)*r for c,q,r in terms)
assert center == -1 and radius == F(3,16)
record("sign_and_exact_sum",center=fmt(center),radius=fmt(radius))

# Direct ratio witnesses include negative numerator, zero-crossing and denominator uncertainty.
for name,Q,A in [
    ("axial_positive",interval(F(7),F(1,8)),(F(2),F(2))),
    ("bending_negative",interval(F(-5),F(1,4)),(F(3),F(3))),
    ("ratio_zero_crossing",(F(-1),F(1)),(F(2),F(4))),
    ("area_formation_uncertainty",(F(1),F(1)),(F(1),1+p2(-52))),
]:
    I=quotient(Q,A)
    for q in [Q[0],sum(Q)/2,Q[1]]:
        for a in [A[0],sum(A)/2,A[1]]:
            assert I[0] <= q/a <= I[1]
    y=rn((sum(Q)/2)/(sum(A)/2)/1000000)
    n,hn,hu=hs(y,F(1000000),I)
    for s in I:
        assert abs(n-s)<=hn and abs(y-s/1000000)<=hu
    record(name,lo=fmt(I[0]),hi=fmt(I[1]),H_n=fmt(hn),H_U=fmt(hu))
assert quotient((F(-1),F(1)),(F(2),F(4))) == (F(-1,2),F(1,2))
assert quotient((F(0),F(0)),(F(1),F(2))) == (0,0)
uncertain = quotient((F(1),F(1)),(F(1),1+p2(-52)))
assert max(abs(1-v) for v in uncertain)>0 # a false exact-denominator H=0 is killed
try:
    quotient((F(0),F(1)),(F(0),F(1)))
    raise AssertionError("zero denominator admitted")
except AssertionError as e:
    assert str(e) != "zero denominator admitted"
record("zero_and_zero_denominator_controls", checks=3)

# Rounded Z does not create a proof of Z*=I/r. Actual T*r/J has its own expression.
I0, r0 = F(1), F(3)
zhat = rn(I0/r0)
assert zhat != I0/r0
assert F(1)/zhat != F(1)/(I0/r0)
found = None
for ti in range(1,17):
    if found: break
    for ii in range(1,17):
        if found: break
        for ri in range(1,17):
            t,iiq,r = F(ti),F(ii),F(ri)
            j=2*iiq; z=rn(iiq/r)
            actual=rn(rn(t*r)/j)
            shortcut=rn(t/rn(2*z))
            if actual != shortcut:
                found=(t,iiq,r,j,z,actual,shortcut)
                break
assert found
t,iiq,r,j,z,actual,shortcut=found
assert torsion((t,t),(r,r),(j,j)) == (t*r/j,t*r/j)
record("torsion_not_T_over_2Zhat",T=fmt(t),I=fmt(iiq),r=fmt(r),J=fmt(j),
       Zhat=hx(z),actual_T_r_J=hx(actual),shortcut=hx(shortcut))
TI=torsion((F(-2),F(3)),(F(1),F(2)),(F(4),F(8)))
assert TI == (F(-1),F(3,2))
for t in [F(-2),F(0),F(3)]:
    for r in [F(1),F(3,2),F(2)]:
        for j in [F(4),F(6),F(8)]:
            assert TI[0] <= t*r/j <= TI[1]
record("torsion_uncertain_operands",lo=fmt(TI[0]),hi=fmt(TI[1]),sampled_exact_points=27)

# Zero/nonzero-underflow and final A1 regimes.
assert bound(F(0),F(0)) == 0
assert h/4 > bound(F(0),F(0))
assert bound(F(0),h) == 2*h
assert bound(small,small) == p2(-1052)
assert bound(small/2,small/2) > ru(eps*small/2)
floor=ru(p2(-438)*h)
assert floor == h and bound(F(0),floor)==2*h
record("zero_subnormal_A1_floor",zero_bound=hx(0),tiny_bound=hx(2*h),
       p512_floor=hx(floor),at_threshold_bound=hx(bound(small,small)))

# Opposite allowance rounding directions; acceptance must use their exact minimum.
directions={}
for i in range(1024):
    b=0x3ff0000000000000 + ((i*0x9e3779b97f4a) & ((1<<52)-1))
    z=dec(b)
    ae,af=allowance(z,z)
    if ae != af:
        key="exact_larger" if ae>af else "f64_larger"
        if key not in directions:
            H=(ae+af)/2
            assert H<=max(ae,af) and H>min(ae,af)
            assert 1000000000*H<=abs(z)
            directions[key]={"z":f"{b:016x}","A_exact":fmt(ae),
                             "A_f64":fmt(af),"rejected_midpoint":fmt(H)}
assert set(directions)=={"exact_larger","f64_larger"}
record("both_sharper_predicates_required", **directions)

# A final class can differ from the kernel class because the actual row set differs.
z=F(1)
assert abs(z)>=rn(p2(-34)*F(1))
assert abs(z)<rn(p2(-34)*p2(40))
record("actual_final_row_set_class_change",value=hx(z),
       scale_before=hx(1),scale_after=hx(p2(40)))

# Directed representability and the proposed bounded rational guard are refusals.
try:
    ru(dec(MAX)+p2(971))
    raise AssertionError("overflow admitted")
except OverflowError:
    pass
def capped_fraction(n,d,cap=8128):
    if not d or max(abs(n).bit_length(),abs(d).bit_length())>cap:
        raise ValueError("design span/refusal")
    return F(n,d)
assert capped_fraction(1,1)==1
for n,d in [(1 << 8128,1),(1,0)]:
    try:
        capped_fraction(n,d)
        raise AssertionError("invalid span/denominator admitted")
    except ValueError:
        pass
record("range_span_refusal_controls",checks=4,proposed_significand_cap_bits=8128)

print(json.dumps({"scope":"finite exact design arithmetic, no production execution",
                  "product_reach_or_availability_claim":False,
                  "all_assertions_passed":True,
                  "control_count":len(records),"controls":records},indent=2))
