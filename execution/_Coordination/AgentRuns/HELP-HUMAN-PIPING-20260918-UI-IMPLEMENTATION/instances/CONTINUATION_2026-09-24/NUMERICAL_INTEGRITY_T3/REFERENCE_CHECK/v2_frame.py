"""V2 independent re-derivation engine for the T3 references (standard library only).

Method (deliberately different from R1's closed forms): direct-stiffness assembly of the 12-DOF
Euler-Bernoulli space-frame element written in coordinate-free global form, grounded springs, rigid
restraints by elimination, consistent equivalent nodal loads for uniform member loads, and a sparse
symmetric Gaussian elimination.  Arithmetic is either exact (fractions.Fraction, with pi replaced by a
rational) or high-precision decimal (decimal.Decimal at PREC digits, with pi from a Chudnovsky series).
Neither R1's code nor any product code is imported.

Element (circular section, Iy = Iz = I, J = 2I), for member i -> j with unit axis e, length L,
projector P = I3 - e e^T and cross-product matrix X = [e]_x:
    K_uu(ii) =  EA/L e e^T + 12EI/L^3 P      K_uu(ij) = -K_uu(ii)
    K_ut(ii) = -6EI/L^2 X   K_ut(ij) = -6EI/L^2 X   K_ut(ji) = +6EI/L^2 X   K_ut(jj) = +6EI/L^2 X
    K_tt(ii) =  GJ/L e e^T + 4EI/L P         K_tt(ij) = -GJ/L e e^T + 2EI/L P
(K_tu = K_ut^T).  Uniform global load q per length: nodal forces qL/2 at both ends, moments
+(L^2/12) e x q at i and -(L^2/12) e x q at j.  Section cut moment at station x from end i:
    M(x) = -M_i + x e x F_i + (x^2/2) e x q,    bending magnitude |P M(x)|.
"""
from decimal import Decimal, getcontext, localcontext
from fractions import Fraction
import re

PREC = 110
getcontext().prec = PREC
getcontext().Emax = 999999
getcontext().Emin = -999999

DOFS = ('UX', 'UY', 'UZ', 'RX', 'RY', 'RZ')


# ---------------------------------------------------------------- pi
def chudnovsky_pi(digits):
    """pi by the Chudnovsky series, as Decimal with `digits` significant digits (+ guard)."""
    with localcontext() as ctx:
        ctx.prec = digits + 20
        C = 426880 * Decimal(10005).sqrt()
        K, M, L, X, S = 6, 1, 13591409, 1, Decimal(13591409)
        for i in range(1, digits // 14 + 3):
            M = M * (K ** 3 - 16 * K) // (i ** 3)
            L += 545140134
            X *= -262537412640768000
            S += Decimal(M * L) / X
            K += 12
        pi = C / S
    return +pi


PI_D = chudnovsky_pi(260)
_s = format(PI_D, 'f')
PI_Q_V2 = Fraction(int(_s.replace('.', '')[:251]), 10 ** 250)   # |PI_Q_V2 - pi| < 1e-249


# ---------------------------------------------------------------- numbers
def parse(s):
    """Model input string -> exact Fraction ('1.5e-3', 'p/q', 'd*2^k')."""
    s = str(s).strip()
    m = re.fullmatch(r'(.+)\*2\^(-?\d+)', s)
    if m:
        return parse(m.group(1)) * Fraction(2) ** int(m.group(2))
    return Fraction(s)


class Num:
    """Arithmetic mode: exact Fraction or Decimal."""
    def __init__(self, mode, pi=None):
        self.mode = mode
        if mode == 'frac':
            self.pi = Fraction(pi) if pi is not None else PI_Q_V2
        elif mode == 'float':
            # defect model only (v2_defects.py): plain binary64 arithmetic
            import math
            self.pi = math.pi
        else:
            self.pi = PI_D if pi is None else Decimal(pi)
        self.zero = self.conv(Fraction(0))
        self.one = self.conv(Fraction(1))

    def conv(self, q):
        if self.mode == 'frac':
            return Fraction(q)
        if self.mode == 'float':
            return float(q)
        if isinstance(q, Decimal):
            return +q
        q = Fraction(q)
        return Decimal(q.numerator) / Decimal(q.denominator)

    def sqrt(self, x):
        if self.mode == 'frac':
            x = Fraction(x)
            if x == 0:
                return Fraction(0)
            from math import isqrt
            n, d = x.numerator, x.denominator
            rn, rd = isqrt(n), isqrt(d)
            if rn * rn == n and rd * rd == d:
                return Fraction(rn, rd)
            raise ValueError('irrational sqrt in exact mode')
        if self.mode == 'float':
            import math
            return math.sqrt(x) if x > 0 else 0.0
        return x.sqrt() if x > 0 else Decimal(0)


def represented(q):
    """binary64 decode of an exact input (Decimal.from_float(float(x)) as an exact Fraction)."""
    return Fraction(float(Fraction(q)))


# ---------------------------------------------------------------- vectors
def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def scal(s, a):
    return (s * a[0], s * a[1], s * a[2])


# ---------------------------------------------------------------- model
def section_props(sec, R):
    """sec: dict of exact Fractions (E, G or nu, OD, ID).  R: Num.  Returns EA, EI, GJ in R."""
    E = sec['E']
    G = sec['G'] if 'G' in sec else sec['E'] / (2 * (1 + sec['nu']))
    OD, ID = sec['OD'], sec['ID']
    a = (OD ** 2 - ID ** 2) / 4          # A / pi
    i = (OD ** 4 - ID ** 4) / 64         # I / pi
    pi = R.pi
    EA = R.conv(E * a) * pi
    EI = R.conv(E * i) * pi
    GJ = R.conv(G * 2 * i) * pi
    return EA, EI, GJ


def load_model(mj, rep=False):
    """JSON model -> exact-Fraction model dict.  rep=True decodes every scalar input to binary64
    (coordinates, section inputs, spring k, loads, each load contribution, uniform loads); spring
    directions are exact vectors and are left exact."""
    f = (lambda s: represented(parse(s))) if rep else parse
    m = {'nodes': {n: tuple(f(c) for c in xyz) for n, xyz in mj['nodes_m'].items()},
         'sections': {k: {kk: f(vv) for kk, vv in s.items()} for k, s in mj['sections'].items()},
         'members': [tuple(x) for x in mj['members']],
         'supports': {}, 'loads': {}, 'udl': {}}
    for n, s in mj.get('supports', {}).items():
        m['supports'][n] = {'rigid': list(s.get('rigid', [])),
                            'springs': [(sp['kind'], tuple(parse(c) for c in sp['direction']), f(sp['k']))
                                        for sp in s.get('springs', [])]}
    for n, l in mj.get('loads', {}).items():
        F = tuple(f(c) for c in l.get('F', ('0', '0', '0')))
        M = tuple(f(c) for c in l.get('M', ('0', '0', '0')))
        m['loads'][n] = [F, M]
    # separately authored contributions, summed exactly (the exact net)
    for (n, dof, v) in mj.get('load_contributions_in_authored_order', []):
        FM = m['loads'].setdefault(n, [(Fraction(0),) * 3, (Fraction(0),) * 3])
        k = DOFS.index(dof)
        vec = list(FM[k // 3])
        vec[k % 3] += f(v)
        FM[k // 3] = tuple(vec)
    for mem, q in mj.get('member_uniform_loads_N_per_m_global', {}).items():
        m['udl'][mem] = tuple(f(c) for c in q)
    return m


# ---------------------------------------------------------------- ordering
def node_order(model):
    """Reverse Cuthill-McKee order of the node graph (bandwidth reduction)."""
    adj = {n: set() for n in model['nodes']}
    for (_, a, b, _) in model['members']:
        adj[a].add(b)
        adj[b].add(a)
    names = list(model['nodes'])
    seen = set()
    order = []
    for start in sorted(names, key=lambda n: (len(adj[n]), names.index(n))):
        if start in seen:
            continue
        seen.add(start)
        queue = [start]
        qi = 0
        while qi < len(queue):
            v = queue[qi]
            qi += 1
            order.append(v)
            for w in sorted(adj[v] - seen, key=lambda n: (len(adj[n]), names.index(n))):
                seen.add(w)
                queue.append(w)
    return order[::-1]


# ---------------------------------------------------------------- assembly and solve
def element(model, mem, R, props):
    name, a, b, sec = mem
    xa = tuple(R.conv(c) for c in model['nodes'][a])
    xb = tuple(R.conv(c) for c in model['nodes'][b])
    d = sub(xb, xa)
    L = R.sqrt(dot(d, d))
    e = scal(R.one / L, d)
    if name in model.get('axis_override', {}):
        # defect models only (v2_nc.py): constitutive frame built on a wrong unit axis
        e = tuple(R.conv(c) for c in model['axis_override'][name])
    EA, EI, GJ = props[sec]
    ee = [[e[r] * e[c] for c in range(3)] for r in range(3)]
    P = [[(R.one if r == c else R.zero) - ee[r][c] for c in range(3)] for r in range(3)]
    X = [[R.zero, -e[2], e[1]], [e[2], R.zero, -e[0]], [-e[1], e[0], R.zero]]
    a1, b12, c6, d4, d2, g1 = EA / L, 12 * EI / L ** 3, 6 * EI / L ** 2, 4 * EI / L, 2 * EI / L, GJ / L
    Kuu = [[a1 * ee[r][c] + b12 * P[r][c] for c in range(3)] for r in range(3)]
    Kut = [[-c6 * X[r][c] for c in range(3)] for r in range(3)]         # (ii) and (ij)
    Ktt_ii = [[g1 * ee[r][c] + d4 * P[r][c] for c in range(3)] for r in range(3)]
    Ktt_ij = [[-g1 * ee[r][c] + d2 * P[r][c] for c in range(3)] for r in range(3)]
    K = [[R.zero] * 12 for _ in range(12)]

    def put(r0, c0, B, s=1):
        for r in range(3):
            for c in range(3):
                K[r0 + r][c0 + c] += B[r][c] if s == 1 else -B[r][c]
    T = lambda B: [[B[c][r] for c in range(3)] for r in range(3)]
    # blocks: 0 ui, 3 ti, 6 uj, 9 tj
    put(0, 0, Kuu); put(0, 6, Kuu, -1); put(6, 0, Kuu, -1); put(6, 6, Kuu)
    put(0, 3, Kut); put(0, 9, Kut); put(6, 3, Kut, -1); put(6, 9, Kut, -1)
    put(3, 0, T(Kut)); put(9, 0, T(Kut)); put(3, 6, T(Kut), -1); put(9, 6, T(Kut), -1)
    put(3, 3, Ktt_ii); put(9, 9, Ktt_ii); put(3, 9, Ktt_ij); put(9, 3, T(Ktt_ij))
    feq = [R.zero] * 12
    q = None
    if name in model['udl']:
        q = tuple(R.conv(c) for c in model['udl'][name])
        exq = cross(e, q)
        for r in range(3):
            feq[r] = q[r] * L / 2
            feq[6 + r] = q[r] * L / 2
            feq[3 + r] = L * L / 12 * exq[r]
            feq[9 + r] = -L * L / 12 * exq[r]
    return {'name': name, 'a': a, 'b': b, 'L': L, 'e': e, 'K': K, 'feq': feq, 'q': q,
            'EA': EA, 'EI': EI, 'GJ': GJ}


def spring_matrix(direction, k, R):
    d = tuple(R.conv(c) for c in direction)
    dd = dot(d, d)
    kk = R.conv(k)
    return [[kk * d[r] * d[c] / dd for c in range(3)] for r in range(3)], d, dd, kk


class Singular(Exception):
    pass


def solve(model, R, return_system=False, singular_rel=None, singular_global=None):
    props = {k: section_props(s, R) for k, s in model['sections'].items()}
    order = node_order(model)
    pos = {n: i for i, n in enumerate(order)}
    ndof = 6 * len(order)
    rigid = set()
    for n, s in model['supports'].items():
        for dof in s['rigid']:
            rigid.add(6 * pos[n] + DOFS.index(dof))
    free = [g for g in range(ndof) if g not in rigid]
    fidx = {g: i for i, g in enumerate(free)}
    A = [dict() for _ in free]
    b = [R.zero] * len(free)
    elems = []
    for mem in model['members']:
        el = element(model, mem, R, props)
        elems.append(el)
        gi = [6 * pos[el['a']] + r for r in range(6)] + [6 * pos[el['b']] + r for r in range(6)]
        for r in range(12):
            if gi[r] not in fidx:
                continue
            ir = fidx[gi[r]]
            row = A[ir]
            for c in range(12):
                if gi[c] in fidx:
                    jc = fidx[gi[c]]
                    if jc >= ir and el['K'][r][c] != 0:
                        row[jc] = row.get(jc, R.zero) + el['K'][r][c]
            b[ir] += el['feq'][r]
    springs = {}
    for n, s in model['supports'].items():
        for idx, (kind, dvec, k) in enumerate(s['springs']):
            Ks, d, dd, kk = spring_matrix(dvec, k, R)
            off = 0 if kind in ('translation', 't') else 3
            springs.setdefault(n, []).append((kind, off, d, dd, kk))
            g0 = 6 * pos[n] + off
            for r in range(3):
                if g0 + r not in fidx:
                    continue
                ir = fidx[g0 + r]
                for c in range(3):
                    if g0 + c in fidx:
                        jc = fidx[g0 + c]
                        if jc >= ir and Ks[r][c] != 0:
                            A[ir][jc] = A[ir].get(jc, R.zero) + Ks[r][c]
    fnod = {}
    for n, (F, M) in model['loads'].items():
        vec = [R.conv(c) for c in F] + [R.conv(c) for c in M]
        fnod[n] = vec
        for r in range(6):
            g = 6 * pos[n] + r
            if g in fidx:
                b[fidx[g]] += vec[r]
    if return_system:
        return A, b, free, pos, order
    # sparse symmetric Gaussian elimination (upper rows)
    nf = len(free)
    diag_scale = max((abs(A[i].get(i, R.zero)) for i in range(nf)), default=R.one)
    pivots = []
    for i in range(nf):
        row = A[i]
        piv = row.get(i, R.zero)
        ref = abs(piv)
        if piv == 0 or (singular_rel is not None and abs(piv) < singular_rel * max(abs(v) for v in row.values())) or \
                (singular_global is not None and abs(piv) < singular_global * diag_scale):
            raise Singular('zero pivot at free dof %d' % i)
        pivots.append(piv)
        items = [(j, v) for j, v in row.items() if j > i and v != 0]
        bi = b[i]
        for (j, vj) in items:
            f = vj / piv
            Aj = A[j]
            for (k, vk) in items:
                if k >= j:
                    Aj[k] = Aj.get(k, R.zero) - f * vk
            b[j] -= f * bi
    x = [R.zero] * nf
    for i in range(nf - 1, -1, -1):
        s = b[i]
        for j, v in A[i].items():
            if j > i:
                s -= v * x[j]
        x[i] = s / A[i][i]
    U = [R.zero] * ndof
    for g, i in fidx.items():
        U[g] = x[i]
    disp = {n: (tuple(U[6 * pos[n] + r] for r in range(3)), tuple(U[6 * pos[n] + 3 + r] for r in range(3)))
            for n in order}
    return post(model, R, elems, disp, springs, fnod, pos)


def post(model, R, elems, disp, springs, fnod, pos):
    out = {}
    for n in model['nodes']:
        u, th = disp[n]
        for r, lab in enumerate(('UX', 'UY', 'UZ')):
            out['u.%s.%s' % (n, lab)] = u[r]
        for r, lab in enumerate(('RX', 'RY', 'RZ')):
            out['th.%s.%s' % (n, lab)] = th[r]
    # nodal sums of element end actions
    endsum = {n: [R.zero] * 6 for n in model['nodes']}
    for el in elems:
        ue = list(disp[el['a']][0]) + list(disp[el['a']][1]) + list(disp[el['b']][0]) + list(disp[el['b']][1])
        fe = [sum((el['K'][r][c] * ue[c] for c in range(12)), R.zero) - el['feq'][r] for r in range(12)]
        for r in range(6):
            endsum[el['a']][r] += fe[r]
            endsum[el['b']][r] += fe[6 + r]
        e, L = el['e'], el['L']
        ua, ta = disp[el['a']]
        ub, tb = disp[el['b']]
        ext = dot(sub(ub, ua), e)
        tw = dot(sub(tb, ta), e)
        nm = el['name']
        out['N.%s' % nm] = el['EA'] * ext / L
        out['T.%s' % nm] = el['GJ'] * tw / L
        out['tw.%s' % nm] = tw
        out['ext.%s' % nm] = ext
        Fi = tuple(fe[0:3])
        Mi = tuple(fe[3:6])
        q = el['q'] if el['q'] is not None else (R.zero,) * 3
        exF = cross(e, Fi)
        exq = cross(e, q)
        for lab, x in (('i', R.zero), ('mid', L / 2), ('j', L)):
            Mc = add(add(scal(-R.one, Mi), scal(x, exF)), scal(x * x / 2, exq))
            Mp = sub(Mc, scal(dot(Mc, e), e))
            out['Mb.%s.%s' % (nm, lab)] = R.sqrt(dot(Mp, Mp))
        # consistency: torque from end actions equals GJ tw / L
        out['_Tchk.%s' % nm] = -dot(Mi, e) - el['GJ'] * tw / L
    spring_act = {}
    for n, lst in springs.items():
        tot = [R.zero] * 6
        for idx, (kind, off, d, dd, kk) in enumerate(lst):
            v = disp[n][0] if off == 0 else disp[n][1]
            s = -kk * dot(d, v) / dd
            act = scal(s, d)
            labs = ('FX', 'FY', 'FZ') if off == 0 else ('MX', 'MY', 'MZ')
            for r in range(3):
                out['S.%s.%d.%s' % (n, idx, labs[r])] = act[r]
                tot[off + r] += act[r]
        spring_act[n] = tot
    for n, s in model['supports'].items():
        for dof in s['rigid']:
            r = DOFS.index(dof)
            val = endsum[n][r] - (fnod[n][r] if n in fnod else R.zero) - (spring_act[n][r] if n in spring_act else R.zero)
            out['R.%s.%s' % (n, dof)] = val
    return out


def equilibrium_residual(model, R, out):
    """Total force and moment (about the origin) of loads + restraint + spring actions."""
    F = [R.zero] * 3
    M = [R.zero] * 3
    def addfm(n, f, m):
        x = tuple(R.conv(c) for c in model['nodes'][n])
        mm = add(m, cross(x, f))
        for r in range(3):
            F[r] += f[r]
            M[r] += mm[r]
    for n, (Fv, Mv) in model['loads'].items():
        addfm(n, tuple(R.conv(c) for c in Fv), tuple(R.conv(c) for c in Mv))
    for mem, q in model['udl'].items():
        _, a, b, _ = next(m for m in model['members'] if m[0] == mem)
        xa = tuple(R.conv(c) for c in model['nodes'][a]); xb = tuple(R.conv(c) for c in model['nodes'][b])
        d = sub(xb, xa); L = R.sqrt(dot(d, d)); qq = tuple(R.conv(c) for c in q)
        tot = scal(L, qq)
        mid = scal(R.one / 2, add(xa, xb))
        for r in range(3):
            F[r] += tot[r]
        mm = cross(mid, tot)
        for r in range(3):
            M[r] += mm[r]
    for k, v in out.items():
        p = k.split('.')
        if p[0] == 'R':
            n, dof = p[1], p[2]
            r = DOFS.index(dof)
            f = [R.zero] * 3; m = [R.zero] * 3
            (f if r < 3 else m)[r % 3] = v
            addfm(n, tuple(f), tuple(m))
        elif p[0] == 'S':
            n, lab = p[1], p[3]
            r = ('FX', 'FY', 'FZ', 'MX', 'MY', 'MZ').index(lab)
            f = [R.zero] * 3; m = [R.zero] * 3
            (f if r < 3 else m)[r % 3] = v
            addfm(n, tuple(f), tuple(m))
    return F, M
