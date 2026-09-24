"""Independent scalar-chain arithmetic probes, no production imports.

PFloat is exact Fraction arithmetic rounded after EACH operation to p binary
significand bits, ties-even, with unbounded exponent. It is NOT an MPFR run.
DD uses actual binary64 TwoSum/FMA product residual and high/low arithmetic.
DD is a bounded experimental transcription, NOT a qualified QD library.
"""
from fractions import Fraction as F
from decimal import Decimal as D, localcontext
import math
import json
import platform


def two_sum(a, b):
    s = a + b
    v = s - a
    return s, (a - (s - v)) + (b - v)


class DD:
    def __init__(self, hi, lo=0.0):
        self.hi, self.lo = two_sum(float(hi), float(lo))

    def exact(self):
        return F(self.hi) + F(self.lo)

    def __neg__(self):
        return DD(-self.hi, -self.lo)

    def __add__(self, b):
        s, e = two_sum(self.hi, b.hi)
        t, f = two_sum(self.lo, b.lo)
        s, e = two_sum(s, e + t)
        return DD(s, e + f)

    def __sub__(self, b):
        return self + (-b)

    def __mul__(self, b):
        p = self.hi * b.hi
        e = math.fma(self.hi, b.hi, -p)
        e += self.hi * b.lo + self.lo * b.hi
        p, e = two_sum(p, e)
        return DD(p, e + self.lo * b.lo)

    def __truediv__(self, b):
        q1 = self.hi / b.hi
        r = self - b * DD(q1)
        q2 = r.hi / b.hi
        r = r - b * DD(q2)
        q3 = r.hi / b.hi
        return DD(q1) + DD(q2) + DD(q3)


def pow2(n):
    return F(2**n) if n >= 0 else F(1, 2**(-n))


def round_binary(x, p):
    x = F(x)
    if not x:
        return x
    sign = 1 if x > 0 else -1
    x = abs(x)
    e = x.numerator.bit_length() - x.denominator.bit_length()
    if x < pow2(e):
        e -= 1
    step = pow2(e - p + 1)
    scaled = x / step
    q, r = divmod(scaled.numerator, scaled.denominator)
    if 2*r > scaled.denominator or (2*r == scaled.denominator and q % 2):
        q += 1
    return sign * q * step


class PFloat:
    def __init__(self, value, precision):
        self.p = precision
        self.v = round_binary(value, precision)

    def exact(self):
        return self.v

    def __neg__(self):
        return PFloat(-self.v, self.p)

    def __add__(self, b):
        return PFloat(self.v + b.v, self.p)

    def __sub__(self, b):
        return PFloat(self.v - b.v, self.p)

    def __mul__(self, b):
        return PFloat(self.v * b.v, self.p)

    def __truediv__(self, b):
        return PFloat(self.v / b.v, self.p)


def ldlt(a, rhs, make):
    # General dense SPD LDL, with no fixture-specific arithmetic or branch.
    n = len(rhs)
    ell = [[make(0) for _ in range(n)] for _ in range(n)]
    diag = [make(0) for _ in range(n)]
    for i in range(n):
        ell[i][i] = make(1)
        for j in range(i):
            v = a[i][j]
            for h in range(j):
                v = v - ell[i][h] * diag[h] * ell[j][h]
            ell[i][j] = v / diag[j]
        v = a[i][i]
        for h in range(i):
            v = v - ell[i][h] * diag[h] * ell[i][h]
        if v.exact() <= 0:
            raise ArithmeticError('nonpositive computed pivot')
        diag[i] = v
    y = []
    for i in range(n):
        v = rhs[i]
        for j in range(i):
            v = v - ell[i][j] * y[j]
        y.append(v)
    x = [y[i] / diag[i] for i in range(n)]
    for i in reversed(range(n)):
        for j in range(i+1, n):
            x[i] = x[i] - ell[j][i] * x[j]
    return x


def assembled(springs, ground, make):
    n = len(springs)+1
    a = [[make(0) for _ in range(n)] for _ in range(n)]
    for i, stiffness in enumerate(springs):
        s = make(stiffness)
        for r, c, sign in [(i,i,1),(i+1,i+1,1),(i,i+1,-1),(i+1,i,-1)]:
            a[r][c] = a[r][c] + (s if sign == 1 else -s)
    a[0][0] = a[0][0] + make(ground)
    return a


def exact_chain(springs, ground, torque):
    x = [torque / ground]
    for a in springs:
        x.append(x[-1] + torque / a)
    return x


def describe(x):
    with localcontext() as ctx:
        ctx.prec = 40
        x = F(x)
        return str(D(x.numerator)/D(x.denominator))


def relative(x, reference):
    return describe(abs((x-reference)/reference))


def result(springs, ground, torque, make, rounded=False):
    a = assembled(springs, ground, make)
    if rounded:
        a = [[make(float(v.exact())) for v in row] for row in a]
    try:
        x = ldlt(a, [make(0)]*len(springs) + [make(torque)], make)
    except (ArithmeticError, ZeroDivisionError) as err:
        return {'status':str(err)}
    exact = exact_chain(springs, ground, torque)
    represented = [v.exact() for v in x]
    actions = [(x[i+1]-x[i])*make(s) for i,s in enumerate(springs)]
    pub = [float(v) for v in represented]
    lost_recovery = [(F(pub[i+1])-F(pub[i]))*s for i,s in enumerate(springs)]
    return {
        'status':'positive factor',
        'max_displacement_relative_error':describe(max(abs((r-t)/t) for r,t in zip(represented,exact))),
        'max_internal_action_relative_error':describe(max(abs((v.exact()-torque)/torque) for v in actions)),
        'spring_relative_error':relative((make(ground)*x[0]).exact(),torque),
        'max_action_error_after_f64_publication':describe(max(abs((v-torque)/torque) for v in lost_recovery)),
        'root':describe(represented[0]),
        'first_relative_rotation':describe((x[1]-x[0]).exact()),
        'published_root_hex':pub[0].hex(),
        'published_tip_hex':pub[-1].hex(),
    }


def machin_pi():
    def atan_inv(q):
        q = D(q)
        x = 1/q
        term, total, j = x, x, 1
        while abs(term) > D('1e-115'):
            term = -term/(q*q)
            total += term/(2*j+1)
            j += 1
        return total
    return 16*atan_inv(5)-4*atan_inv(239)


def oblique_energy_probe(a):
    # Generic rank-one energy terms with exactly orthogonal represented vectors.
    # This is a 2-coordinate numerical analogue, not a deployed frame rotation.
    c,s=F(.6),F(.8)
    v,w=[c,s],[-s,c]
    k,t=F(1e-12),F(1e-16)
    alpha=c*c+s*s
    make=lambda x:PFloat(x,128)
    rhs=[make(t)*make(wi) for wi in w]
    expected=[t*wi/(k*alpha) for wi in w]
    rows=[]
    for rounded in [False,True]:
        mat=[]
        for i in range(2):
            row=[]
            for j in range(2):
                if rounded:
                    # Exact lifts of individually rounded global energy entries.
                    h=make(float(a)*float(v[i])*float(v[j]))
                    soft=make(float(k)*float(w[i])*float(w[j]))
                else:
                    h=make(a)*make(v[i])*make(v[j])
                    soft=make(k)*make(w[i])*make(w[j])
                row.append(h+soft)
            mat.append(row)
        label='rounded_global_element_terms' if rounded else 'source_energy_factors_at_128'
        try:
            u=ldlt(mat,rhs,make)
            q=make(0)
            hard=make(0)
            for wi,vi,ui in zip(w,v,u):
                q=q+make(wi)*ui
                hard=hard+make(vi)*ui
            rows.append({'path':label,'max_u_relative_error':describe(max(abs((ui.exact()-e)/e) for ui,e in zip(u,expected))),
                         'soft_action_relative_error':relative((make(k)*q).exact(),t),
                         'hard_action_over_applied_T':describe(abs((make(a)*hard).exact()/t))})
        except ArithmeticError as err:
            rows.append({'path':label,'status':str(err)})
    return rows


def main():
    with localcontext() as ctx:
        ctx.prec = 120
        a_decimal = D(687800)*machin_pi()
    a = F(float(a_decimal))
    out = {'environment':{'python':platform.python_version(),'platform':platform.platform(),
                          'math_fma':hasattr(math,'fma')},
           'a_decimal_120':str(a_decimal),'a_f64_exact':describe(a),
           'rounding_self_checks':{},'cases':[]}
    for x in [F(1),F(1,10), F(-1,10), F(2**53+1), F(2**53+3),F(1,2**100),a]:
        assert round_binary(x,53)==F(float(x))
    for x,y in [(float(a),1e-12),(1e16,1.0),(1.25,-1.25),(1e-4,1e-22)]:
        hi,lo=two_sum(x,y)
        assert F(hi)+F(lo)==F(x)+F(y)
        hi=x*y;lo=math.fma(x,y,-hi)
        assert F(hi)+F(lo)==F(x)*F(y)
    out['rounding_self_checks']={'53_bit_ties_even':7,'two_sum_and_fma_product_exact_checks':8}
    for name,ks,ts,n in [
        ('ordinary_small_angle','100','.01',1),
        ('N05','1e-4','1e-8',1),('N06','1e-12','1e-16',1),
        ('supplemental_large_angle_algebra_only','1e-10','.1',1),
        ('supplemental_small_angle','1e-10','1e-14',1),
        ('N06_8span_scalar_chain','1e-12','1e-16',8),
        ('adaptive_stress_small_angle','1e-28','1e-32',1),
    ]:
        k,t=F(float(ks)),F(float(ts))
        springs=[a] if n==1 else [F(float(a*(i+1)/(i+2))) for i in range(n)]
        theta=t/k;delta=t/springs[0]
        q={'id':name,'spans':n,'k_decimal':ks,'T_decimal':ts,
           'theta0_from_exact_f64_inputs':describe(theta),
           'first_twist':describe(delta),
           'theta0_f64_ulp':describe(F(math.ulp(float(theta)))),
           'twist_over_global_ulp':describe(delta/F(math.ulp(float(theta)))),
           'kappa_approx_2dof':describe(4*a/k),
           'results':{}}
        for p in [53,106,128,192,256,512]:
            q['results']['binary_'+str(p)]=result(springs,k,t,lambda x:PFloat(x,p))
        q['results']['genuine_two_word_DD']=result(springs,k,t,DD)
        q['results']['rounded_global_then_binary128']=result(springs,k,t,lambda x:PFloat(x,128),rounded=True)
        # Separate relative-coordinate state: 1/2 k q0^2 + sum 1/2 a_i qi^2.
        rel=[t/k]+[t/s for s in springs]
        rel_f64=[F(float(v)) for v in rel]
        q['relative_coordinate_direct_f64']={
            'max_action_relative_error':describe(max([abs((k*rel_f64[0]-t)/t)]+[abs((s*qv-t)/t) for s,qv in zip(springs,rel_f64[1:])])),
            'note':'analytical diagonal relative-energy formulation; not generic frame implementation'}
        out['cases'].append(q)
    out['oblique_energy_probe']=oblique_energy_probe(a)
    print(json.dumps(out,indent=2))


if __name__=='__main__':
    main()
