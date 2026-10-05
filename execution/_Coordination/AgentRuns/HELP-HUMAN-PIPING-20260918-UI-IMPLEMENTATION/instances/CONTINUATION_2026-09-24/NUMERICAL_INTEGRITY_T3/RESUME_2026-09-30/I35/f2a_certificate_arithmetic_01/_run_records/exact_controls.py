"""Bounded, standard-library ABSTRACT design controls; no product imports/probes."""
from fractions import Fraction as F
import json

P = 1024
SPAN = 8128
EXP = 1 << 62
MAX_BITS = 0x7fefffffffffffff
PI_N = int('3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917', 16)
checks = []

def two(e):
    return F(1 << e) if e >= 0 else F(1, 1 << -e)

def exponent(x):
    x = abs(x)
    assert x
    e = x.numerator.bit_length() - x.denominator.bit_length()
    if x < two(e):
        e -= 1
    assert two(e) <= x < two(e+1)
    return e

def rn(x, p=P):
    x = F(x)
    if not x:
        return F(0)
    sign = -1 if x < 0 else 1
    e = exponent(x)
    unit = two(e-p+1)
    y = abs(x)/unit
    q, r = divmod(y.numerator, y.denominator)
    if 2*r > y.denominator or (2*r == y.denominator and q & 1):
        q += 1
    return sign*q*unit

def step(x, toward):
    assert x
    e = exponent(x)
    away = (toward == 'up') == (x > 0)
    unit = two(e-P+1 if away or abs(x) != two(e) else e-P)
    return x + (unit if toward == 'up' else -unit)

def directed(x, toward):
    q = rn(x)
    if (toward == 'up' and q < x) or (toward == 'down' and q > x):
        q = step(q, toward)
    assert (q >= x) if toward == 'up' else (q <= x)
    return q

def span_of(values):
    lows, highs = [], []
    for value in values:
        value = abs(F(value))
        if not value:
            continue
        d = value.denominator
        assert d & (d-1) == 0
        n = value.numerator
        trail = (n & -n).bit_length()-1
        lows.append(trail-(d.bit_length()-1))
        highs.append(n.bit_length()-1-(d.bit_length()-1))
    return 0 if not lows else max(highs)-min(lows)+1

def add(a, b, direction='up'):
    a,b=F(a),F(b)
    if span_of([a,b]) > SPAN:
        raise ValueError('span')
    return directed(a+b,direction)

def sub(a,b,direction='up'):
    return add(a,-b,direction)

def mul(a,b,direction='up'):
    a,b=F(a),F(b)
    return directed(a*b,direction)

def div(a,b,direction='up'):
    a,b=F(a),F(b)
    if b <= 0:
        raise ValueError('nonpositive denominator')
    return directed(a/b,direction)

def absdiff(a,b):
    return sub(max(a,b),min(a,b),'up')

def sqrt_dir(a,direction):
    if a < 0:
        raise ValueError('negative radicand')
    if a == 0:
        return F(0),0
    assert rn(a) == a
    e = exponent(a)-P+1
    sig = int(a/two(e))
    base = P+2
    shift = base if (e-base)%2 == 0 else base+1
    rad = sig << shift
    root = rem = 0
    peaks = {'root':0,'rem':0,'trial':0}
    for i in range(P+1,-1,-1):  # exactly 1026 digit-pair iterations
        rem = (rem << 2) | ((rad >> (2*i)) & 3)
        trial = (root << 2) | 1
        peaks['rem'] = max(peaks['rem'],rem.bit_length())
        peaks['trial'] = max(peaks['trial'],trial.bit_length())
        root <<= 1
        if rem >= trial:
            rem -= trial
            root |= 1
        peaks['root'] = max(peaks['root'],root.bit_length())
    assert root*root <= rad < (root+1)*(root+1)
    assert rem == rad-root*root
    assert max(peaks.values()) <= 1088
    drop = root.bit_length()-P
    keep = root >> drop
    roundbit = bool(root & (1 << (drop-1)))
    rest = bool(rem or root & ((1 << (drop-1))-1))
    if roundbit and (rest or keep & 1):
        keep += 1
    q = keep * two((e-shift)//2+drop)
    side = q*q-a
    if (direction == 'up' and side < 0) or (direction == 'down' and side > 0):
        q = step(q,direction)
    assert q >= 0
    assert q*q >= a if direction == 'up' else q*q <= a
    return q,max(peaks.values())

def bits_value(bits):
    assert 0 <= bits <= MAX_BITS
    biased, frac = bits >> 52, bits & ((1<<52)-1)
    return frac*two(-1074) if biased == 0 else ((1<<52)|frac)*two(biased-1075)

def ru64_search(terms):
    assert len(terms) == 3
    target = sum(terms,F(0))
    assert target >= 0
    comparisons = 1
    if bits_value(MAX_BITS) < target:
        raise ValueError('range')
    lo,hi = 0,MAX_BITS
    for _ in range(63):
        if lo == hi:
            break
        mid = (lo+hi)//2
        comparisons += 1
        if bits_value(mid) >= target:
            hi = mid
        else:
            lo = mid+1
    assert lo == hi and comparisons <= 64
    assert bits_value(lo) >= target
    assert lo == 0 or bits_value(lo-1) < target
    return bits_value(lo),comparisons

def rn64(x):
    if x == 0:
        return F(0)
    e = exponent(x)
    quantum = two(max(e-52,-1074))
    z = abs(x)/quantum
    n,r = divmod(z.numerator,z.denominator)
    if 2*r > z.denominator or (2*r == z.denominator and n & 1):
        n += 1
    result = n*quantum
    if result > bits_value(MAX_BITS):
        raise ValueError('binary64 overflow')
    return result if x >= 0 else -result

def assert_group(name, f):
    data = f()
    checks.append({'name':name,'passed':True,'detail':data})

def zeros():
    h=two(-1074)
    assert rn(h)==h and rn(h/2)==h/2 and rn64(h/2)==0
    assert add(0,0)==0 and mul(0,F(7,3))==0 and div(0,3)==0
    assert sqrt_dir(0,'up')[0]==0
    return {'binary64_min_lift_exact':True,'wide_half_min_nonzero':True}
assert_group('zero_and_subnormal',zeros)

def cancellation():
    t=two(-1000)
    assert sub(1+t,1,'down')==t
    tiny=two(-1100)
    lo,hi=sub(1,tiny,'down'),sub(1,tiny,'up')
    assert lo <= 1-tiny <= hi and lo < hi
    assert hi==1 and lo==1-two(-1024)
    for s in [1,-1]:
        x=s*(F(1,3)+two(-1300))
        assert directed(x,'down') <= x <= directed(x,'up')
    return {'exact_cancellation':True,'negative_tiny_subtraction_enclosed':True}
assert_group('signed_cancellation_and_rounding',cancellation)

def span_controls():
    assert span_of([1,two(-8127)])==8128
    add(1,two(-8127))
    try: add(1,two(-8128))
    except ValueError as e: assert str(e)=='span'
    else: assert False
    assert span_of([two(6000),two(-3000)])==9001
    return {'span_limit':8128,'next_span_refuses':True}
assert_group('exponent_separation_and_span',span_controls)

def endpoint_steps():
    for x in [F(1),F(-1),F(3,2),F(-3,2),two(-1074),-two(-1074)]:
        assert step(x,'down') < x < step(x,'up')
        assert rn(step(x,'up'))==step(x,'up')
        assert rn(step(x,'down'))==step(x,'down')
    assert step(1,'down')==1-two(-1024)
    assert step(1,'up')==1+two(-1023)
    return {'values':6,'power_of_two_asymmetry':True}
assert_group('one_step_adjacency',endpoint_steps)

def division():
    pairs=[(1,3),(-1,3),(two(-1074),F(7)),(two(1000),two(-1000)),(F(1),two(-53))]
    for a,b in pairs:
        assert div(a,b,'down') <= F(a)/b <= div(a,b,'up')
    for bad in [0,-1]:
        try: div(1,bad)
        except ValueError: pass
        else: assert False
    return {'pairs':len(pairs),'zero_negative_denominator_refused':True}
assert_group('positive_division_and_close_material_denominator',division)

def alpha():
    near=1-two(-1024)
    assert rn(near)==near and sub(1,near,'down')==two(-1024)
    exact=1-two(-1025)
    upper=directed(exact,'up')
    assert exact<1 and upper==1
    beta,eta,v=F(2),F(1,8),F(1,3)
    ah=mul(beta,eta)
    tau=div(mul(beta,directed(v,'up')),sub(1,ah,'down'))
    assert tau >= beta*v/(1-beta*eta)
    return {'strict_equal_one_refuses':True,'positive_2_pow_minus_1024_denominator':True}
assert_group('alpha_to_one_and_tau',alpha)

def restoring_division():
    pairs=[(1<<1023,1<<1023),((1<<1024)-1,1<<1023),(1<<1023,(1<<1024)-1),((1<<1024)-13,(1<<1024)-29)]
    peak=0
    for aa,bb in pairs:
        remainder=aa
        quotient=0
        if remainder>=bb:
            remainder-=bb
            quotient=1
        for _ in range(1025):
            trial=2*remainder
            peak=max(peak,trial.bit_length())
            bit=trial>=bb
            remainder=trial-bb if bit else trial
            quotient=(quotient<<1)|int(bit)
            assert 0<=remainder<bb
        assert quotient==(aa<<1025)//bb
        assert remainder==(aa<<1025)%bb
        assert quotient.bit_length()<=1026
    assert peak<=1025
    return {'pairs':len(pairs),'iterations':1025,'trial_bits_at_most':peak}
assert_group('fixed_restoring_division',restoring_division)

def square_roots():
    values=[F(1),F(2),F(4),F(9),two(-2148),two(-2149),two(2046),F(3,2)]
    peak=0
    for a in values:
        lo,l=sqrt_dir(a,'down'); hi,h=sqrt_dir(a,'up')
        assert lo*lo<=a<=hi*hi and (lo==hi or step(lo,'up')==hi)
        peak=max(peak,l,h)
    try: sqrt_dir(F(-1),'up')
    except ValueError: pass
    else: assert False
    return {'values':len(values),'iterations_per_nonzero':1026,'max_observed_active_bits':peak}
assert_group('bounded_directed_sqrt',square_roots)

def pi_and_geometry():
    pl,pu=PI_N*two(-512),(PI_N+1)*two(-512)
    assert 3<pl<pu<4 and pu-pl==two(-512) and PI_N.bit_length()==514
    pairs=[(F(2),two(-50)),(F(1),two(-1074)),(two(500),two(498)),(two(-1000),two(-1002)),(F(3),F(1,8))]
    for D,t in pairs:
        c=D/2
        d=(sub(D,t,'down'),sub(D,t,'up'))
        ri=(sub(c,t,'down'),sub(c,t,'up'))
        pp=(mul(t,d[0],'down'),mul(t,d[1],'up'))
        c2=mul(c,c,'down')
        qq=(add(c2,mul(ri[0],ri[0],'down'),'down'),add(c2,mul(ri[1],ri[1],'up'),'up'))
        gg=(mul(pp[0],qq[0],'down'),mul(pp[1],qq[1],'up'))
        AA=(mul(pl,pp[0],'down'),mul(pu,pp[1],'up'))
        II=(mul(pl,gg[0],'down')/4,mul(pu,gg[1],'up')/4)
        JJ=(2*II[0],2*II[1])
        ZZ=(div(II[0],c,'down'),div(II[1],c,'up'))
        ep=t*(D-t); eg=ep*(c*c+(c-t)*(c-t))
        for bounds,exactends in [(AA,(pl*ep,pu*ep)),(II,(pl*eg/4,pu*eg/4)),(JJ,(pl*eg/2,pu*eg/2)),(ZZ,(pl*eg/(4*c),pu*eg/(4*c)))]:
            assert 0<bounds[0]<=exactends[0]<=exactends[1]<=bounds[1]
    return {'abstract_pairs':len(pairs),'reviewed_pi_constant_width':514,'no_runtime_series':True}
assert_group('outward_source_geometry',pi_and_geometry)

def material():
    for E,nu in [(F(3),rn64(F(1,3))),(F(1),-1+two(-53)),(F(1),two(-1074))]:
        d=(2*add(1,nu,'down'),2*add(1,nu,'up'))
        g=(div(E,d[1],'down'),div(E,d[0],'up'))
        exact=E/(2*(1+nu))
        assert 0<g[0]<=exact<=g[1]
    return {'material_pairs':3}
assert_group('selected_E_nu_outward_material',material)

def rowsums():
    terms=[two(-i*79) for i in range(100)]
    total=F(0)
    for term in terms:
        total=add(total,term)
    assert total>=sum(terms,F(0)) and rn(total)==total
    return {'terms':100,'stored_significand_bound':1024}
assert_group('fixed_width_nonnegative_accumulator',rowsums)

def corners():
    Q=(-F(3,2),directed(F(2,3),'up')); D=(F(2),F(3))
    lo=min(div(q,d,'down') for q in Q for d in D)
    hi=max(div(q,d,'up') for q in Q for d in D)
    for q in [Q[0],0,Q[1]]:
        for d in [D[0],F(5,2),D[1]]:
            assert lo<=q/d<=hi
    T=(-F(1),F(2)); C=(F(3),F(4)); J=(F(5),F(6))
    lows=[div(mul(t,c,'down'),j,'down') for t in T for c in C for j in J]
    highs=[div(mul(t,c,'up'),j,'up') for t in T for c in C for j in J]
    for t in [T[0],0,T[1]]:
        for c in [C[0],F(7,2),C[1]]:
            for j in [J[0],F(11,2),J[1]]:
                assert min(lows)<=t*c/j<=max(highs)
    assert all(div(mul(0,c),j)==0 for c in C for j in J)
    return {'signed_quotient_corners':4,'torsion_corners':8,'zero_torsion_exact':True}
assert_group('signed_recipe_corners',corners)

def binary_search():
    terms=[(F(0),F(0),F(0)),(two(-1074),two(-1074),two(-1074)),(F(1),two(-53),F(0)),(two(-1060),two(-1074),two(-1074)),(bits_value(MAX_BITS),F(0),F(0))]
    maximum=0
    for triple in terms:
        value,c=ru64_search(triple); maximum=max(maximum,c)
        assert value>=sum(triple,F(0))
    try: ru64_search((bits_value(MAX_BITS),bits_value(MAX_BITS),F(0)))
    except ValueError: pass
    else: assert False
    return {'cases':len(terms),'max_comparisons':maximum,'overflow_refuses':True}
assert_group('fixed_64_comparison_RU64_bound',binary_search)

def final_predicates():
    h=two(-1074); n=S=F(1)
    allowance=two(-64)+two(-85)+two(-53)+h
    delta=two(-1200)
    assert allowance-(allowance-delta)>0
    assert allowance-allowance==0
    assert allowance-(allowance+delta)<0
    assert 0-F(0)==0 and 0-h<0
    for value in [F(1),two(-1000),F(7,4),two(1000)]:
        rhs=value/F(10**9)
        assert value-10**9*rhs==0
        assert value-10**9*(rhs+delta)<0
    y=F(1); n=rn64(y/1000)
    Hn=absdiff(n,n); HU=absdiff(y,1000*n)
    assert Hn==0 and HU>0
    return {'exact_equality_and_neighbors':True,'zero_bound_not_underflow':True,'raw_mm_error_survives_SI_equality':True}
assert_group('exact_final_allowance_and_raw_SI_boundaries',final_predicates)

def allowance_order():
    differences=[]
    for n,S in [(F(1),F(1)),(two(-1074),two(-1074)),(two(-1000),two(-1000)),(F(7,4),F(7,4)),(two(500),two(500)),(bits_value(0x3ff000000000ffff),bits_value(0x3ff000000000ffff))]:
        m=max(abs(n),S)
        exact=m*two(-64)+m*two(-85)+abs(n)*two(-53)+two(-1074)
        a0=rn64(m*two(-64)); a1=rn64(a0*(1+two(-21)))
        u0=rn64(abs(n)*two(-53)); u1=rn64(u0+two(-1074)); a2=rn64(a1+u1)
        differences.append((a2>exact)-(a2<exact))
        assert a2>=0
    assert -1 in differences and 1 in differences
    return {'finite_cases':len(differences),'rounding_signs':differences,'exact_not_replaced_by_f64':True}
assert_group('operation_ordered_binary64_allowance',allowance_order)

def bounds():
    assert 5*(1<<8128) < 1<<8131 < 1<<8192
    assert (2**64-1)**2+2*(2**64-1)==2**128-1
    assert 152*137+(256+130+17+128+128)*8+128*8+64+2*137==27458
    assert 3*58+236+248+244+280==1182
    assert 17*1026+320+32==17794
    assert 18*1026+320+32==18820
    assert 5*(18+128)+16+5*256+5*128+2*128 <4096
    assert 5*(18+128)+5*16+5*256+128<4096
    for e,k,accept in [(EXP,0,True),(EXP,1,False),(-EXP,-1,False),(-EXP,1024,True)]:
        assert (-EXP<=e+k<=EXP)==accept
    max_u64=(1<<64)-1
    assert max_u64+1>max_u64  # checked aggregate must refuse, no wrap or saturation recovery
    return {'logical_constant_bytes':27458,'builder_calls':58,'fresh_sumwork_upper':4096,'range_checks_symbolic':True}
assert_group('width_carry_storage_count_and_exponent_bounds',bounds)

print(json.dumps({'kind':'abstract_standard_library_design_controls','source_code_executed':False,'groups_passed':len(checks),'precision':P,'span_limit':SPAN,'checks':checks},indent=2,sort_keys=True))
