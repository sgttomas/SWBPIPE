#!/usr/bin/env python3
"""RV42 independent fixed exact math, no product/model execution."""
from fractions import Fraction as F
import json,struct

def atan_even(q,n):
    total=F(0)
    power=q
    qsq=q*q
    for k in range(n):
        total+=F(1 if k%2==0 else -1,(2*k+1)*power)
        power*=qsq
    error=F(1,(2*n+1)*power)
    return total,total+error

def floor(q):return q.numerator//q.denominator
def ceil(q):return -floor(-q)
def p2(e):return F(1<<e) if e>=0 else F(1,1<<-e)
def value(a):return a[0]*p2(a[1])
def decode(b):
    e=(b>>52)&2047;m=b&((1<<52)-1)
    assert e<2047 and not b>>63
    return (m,-1074) if e==0 else ((1<<52)+m,e-1075)
def exact_add(a,b,negative=False):
    e=min(a[1],b[1]); m=(a[0]<<(a[1]-e))+(-1 if negative else 1)*(b[0]<<(b[1]-e))
    assert m>0
    return m,e
def exact_mul(a,b):return a[0]*b[0],a[1]+b[1]
def adjust(a,e):return a[0],a[1]+e

def main():
    # Independent identity pi=4(atan(1/2)+atan(1/3)), not author's Machin decomposition.
    # Tangent of the sum is 1; the sum lies in (0,5/6)<(0,pi/2).
    assert (F(1,2)+F(1,3))/(1-F(1,6))==1
    aa=atan_even(2,300);bb=atan_even(3,300)
    pi2=(4*(aa[0]+bb[0]),4*(aa[1]+bb[1]))
    expected_lo=int('3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917',16)
    expected_hi=expected_lo+1
    assert floor(pi2[0]*p2(512))==expected_lo
    assert ceil(pi2[1]*p2(512))==expected_hi
    assert F(expected_lo,1<<512)<pi2[0]<pi2[1]<F(expected_hi,1<<512)
    a5=atan_even(5,128);a239=atan_even(239,128)
    machin=(16*a5[0]-4*a239[1],16*a5[1]-4*a239[0])
    assert machin[1]-machin[0]==F(16,257*5**257)+F(4,257*239**257)
    assert machin[1]-machin[0]<p2(-600)
    assert max(pi2[0],machin[0])<min(pi2[1],machin[1])
    assert floor(machin[0]*p2(512))==expected_lo and ceil(machin[1]*p2(512))==expected_hi
    pi=(F(expected_lo,1<<512),F(expected_hi,1<<512))
    # Independent unnormalized dyadic representation follows the proposed bounded schedule.
    dset=[4,8,0x0010000000000000,0x002fffffffffffff,0x200fffffffffffff,0x3ff0000000000000,0x4010000000000000,0x5fefffffffffffff,0x7fefffffffffffff]
    widths={};emin=0;emax=0;cases=0;formula_checks=0
    for db in dset:
        D=decode(db);d=value(D)
        tset={1,2,3}
        for scale in (F(1,4),F(1,3),F(1,1024)):
            tf=float(d*scale)
            tb=struct.unpack('>Q',struct.pack('>d',tf))[0]
            if tb:tset.add(tb)
        halfbits=struct.unpack('>Q',struct.pack('>d',float(d/2)))[0]
        if halfbits>1:tset.add(halfbits-1)
        for tb in tset:
            t=decode(tb);tv=value(t)
            if not 0<tv<d/2:continue
            c=adjust(D,-1);dt=exact_add(D,t,True);ri=exact_add(c,t,True)
            P=exact_mul(t,dt);c2=exact_mul(c,c);ri2=exact_mul(ri,ri)
            Q=exact_add(c2,ri2);G=exact_mul(P,Q)
            allvalues={'D_minus_t':dt,'ri':ri,'P':P,'Q':Q,'G':G}
            for suffix,p in [('lower',expected_lo),('upper',expected_hi)]:
                pd=(p,-512);av=exact_mul(pd,P);iv=adjust(exact_mul(pd,G),-2);jv=adjust(iv,1)
                # Z has same numerator, exponent iv.exp-c.exp, denominator c.m.
                zv=(iv[0],iv[1]-c[1]);allvalues.update({'A_'+suffix:av,'I_'+suffix:iv,'J_'+suffix:jv,'Z_'+suffix:zv})
                assert value(av)==F(p,1<<512)*(d*d/4-(d/2-tv)**2)
                assert value(iv)==F(p,1<<512)*(d**4/16-(d/2-tv)**4)/4
                assert value(jv)==2*value(iv)
                assert value(zv)/c[0]==value(iv)/(d/2)
                formula_checks+=4
            limits={'D_minus_t':2098,'ri':2099,'P':2151,'Q':4200,'G':6351}
            for name,v in allvalues.items():
                actual=v[0].bit_length();widths[name]=max(widths.get(name,0),actual)
                cap=limits.get(name,2665 if name.startswith('A_') else 6865)
                assert actual<=cap,(db,tb,name,actual,cap)
                emin=min(emin,v[1]);emax=max(emax,v[1]);assert -5782<=v[1]<=4443
            assert c[0].bit_length()<=53
            cases+=1
    products=[1*33,1*1,33*33,34*66,2*(9*34),2*(9*100)]
    assert sum(products)==5779 and 8*128*8==8192
    return {'label':'independent exact mathematics only; no runtime/model/solver or product admission claim','all_passed':True,
      'pi':{'independent_identity':'4*(atan(1/2)+atan(1/3))','independent_terms_each':300,'constants_match':True,'machin_terms_each':128,'machin_width_lt_2_neg_600':True,'lower_numerator_hex':hex(expected_lo),'upper_numerator_hex':hex(expected_hi),'final_machin_numerator_bits':max(v.numerator.bit_length() for v in machin),'final_machin_denominator_bits':max(v.denominator.bit_length() for v in machin)},
      'schedule_samples':{'cases':cases,'formula_endpoint_equalities':formula_checks,'maximum_raw_integer_widths':widths,'observed_exponent_min':emin,'observed_exponent_max':emax,'limit':'sampled sanity checks; general width proof in REVIEW.md'},
      'geometry_work_storage':{'grade_school_limb_products':sum(products),'eight_128_limb_buffers_logical_bytes':8192,'excludes':'actual implementation carries, counters, metadata, call frames, allocator capacities, B1 comparisons and caller overlap'}}
if __name__=='__main__':print(json.dumps(main(),indent=2))
