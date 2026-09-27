"""V3 (RF-ELOAD refutation): independent exact solver. Standard library only.

Third route, different from the author's route A (tree integration plus force method) and route B
(closed-form direct stiffness with consistent Hermite loads and recovery K_e u_e - f_eq, then statics
from end i):

* Element stiffness: the 6x6 end-j flexibility of the member clamped at i (closed-form cantilever
  compliances, in coordinate-free global form), inverted exactly; the other blocks by the
  contragredient equilibrium transformation Gamma (K = [[G K G^T, G K], [K G^T, K]]).
* Element loads: the fixed-fixed state is obtained by solving the Euler-Bernoulli ODEs
  EI v'''' = q_perp and EA w'' = -q_ax exactly (piecewise polynomials with Macaulay brackets and the
  four clamped boundary conditions), not by shape-function integration. Its end reactions are the
  fixed-end forces; eigenstrain and thrust enter the same fixed state as a constant axial force.
* Recovery: superposition of the fixed-fixed internal fields (from the ODE solution) and the
  displacement-state fields obtained by statics from END J (not end i).
* pi: Chudnovsky series (binary splitting), 170 digits, as a rational.

Inputs are taken from each case's `inputs` strings (the authored decimals), with the topology from
`model`. Every derived magnitude (sections, local-to-global intensities, eps*, F_p, generated
intensities) is recomputed here from the inputs and cross-checked against the model's derived fields.
"""
import math
from fractions import Fraction as Fr
from decimal import Decimal, localcontext

DOFS = ('UX', 'UY', 'UZ', 'RX', 'RY', 'RZ')
STATIONS = (('i', Fr(0)), ('q1', Fr(1, 4)), ('mid', Fr(1, 2)), ('q3', Fr(3, 4)), ('j', Fr(1)))


# ---------------------------------------------------------------- pi (Chudnovsky, binary splitting)
def _bs(a, b):
    if b - a == 1:
        if a == 0:
            p = q = 1
        else:
            p = (6 * a - 5) * (2 * a - 1) * (6 * a - 1)
            q = a * a * a * 10939058860032000
        t = p * (13591409 + 545140134 * a)
        return p, q, (-t if a & 1 else t)
    m = (a + b) // 2
    p1, q1, t1 = _bs(a, m)
    p2, q2, t2 = _bs(m, b)
    return p1 * p2, q1 * q2, t1 * q2 + p1 * t2


def pi_scaled(d):
    """floor-ish of pi * 10^d (error < 2 units in the last place)."""
    n = d // 14 + 3
    _, q, t = _bs(0, n)
    s = math.isqrt(10005 * 10 ** (2 * d))
    return (q * 426880 * s) // t


PI_DIGITS = 170
PI = Fr(pi_scaled(PI_DIGITS), 10 ** PI_DIGITS)


# ---------------------------------------------------------------- small exact linear algebra
def vadd(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vsub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def vscl(s, a):
    return tuple(s * x for x in a)


def vdot(a, b):
    return sum(x * y for x, y in zip(a, b))


def vcross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


ZERO3 = (Fr(0), Fr(0), Fr(0))
AXES = {'X': (Fr(1), Fr(0), Fr(0)), 'Y': (Fr(0), Fr(1), Fr(0)), 'Z': (Fr(0), Fr(0), Fr(1))}


def fsqrt_exact(q):
    q = Fr(q)
    n, d = math.isqrt(q.numerator), math.isqrt(q.denominator)
    if n * n != q.numerator or d * d != q.denominator:
        raise ValueError('not a rational square: %s' % q)
    return Fr(n, d)


def mat_inv(A):
    n = len(A)
    M = [list(r) + [Fr(int(i == j)) for j in range(n)] for i, r in enumerate(A)]
    for c in range(n):
        p = next(r for r in range(c, n) if M[r][c] != 0)
        M[c], M[p] = M[p], M[c]
        pv = M[c][c]
        M[c] = [x / pv for x in M[c]]
        for r in range(n):
            if r != c and M[r][c] != 0:
                f = M[r][c]
                M[r] = [x - f * y for x, y in zip(M[r], M[c])]
    return [r[n:] for r in M]


def solve(A, b):
    n = len(A)
    M = [list(r) + [b[i]] for i, r in enumerate(A)]
    for c in range(n):
        p = next(r for r in range(c, n) if M[r][c] != 0)
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            if M[r][c] != 0:
                f = M[r][c] / M[c][c]
                M[r] = [x - f * y for x, y in zip(M[r], M[c])]
    x = [Fr(0)] * n
    for r in range(n - 1, -1, -1):
        x[r] = (M[r][n] - sum(M[r][k] * x[k] for k in range(r + 1, n))) / M[r][r]
    return x


def matmul(A, B):
    Bt = list(zip(*B))
    return [[sum(a * b for a, b in zip(r, c)) for c in Bt] for r in A]


def transpose(A):
    return [list(r) for r in zip(*A)]


# ---------------------------------------------------------------- element: flexibility-inverted stiffness
def _compliance(e, v, EI, GJ):
    ev = vdot(e, v)
    return vadd(vscl(ev / GJ, e), vscl(1 / EI, vsub(v, vscl(ev, e))))


def element_stiffness(L, e, EA, EI, GJ):
    """Exact inverse of the clamped-at-i end-j flexibility, plus Gamma transforms. 12x12, global."""
    F = [[Fr(0)] * 6 for _ in range(6)]
    for k in range(6):
        unit = [Fr(int(k == m)) for m in range(6)]
        P, M = tuple(unit[:3]), tuple(unit[3:])
        eP = vcross(e, P)
        # M(s) = M + (L - s) e x P ;  int M ds = L M + L^2/2 e x P ; int (L-s) M ds = L^2/2 M + L^3/3 e x P
        th = _compliance(e, vadd(vscl(L, M), vscl(L * L / 2, eP)), EI, GJ)
        k2 = _compliance(e, vadd(vscl(L * L / 2, M), vscl(L ** 3 / 3, eP)), EI, GJ)
        u = vadd(vcross(k2, e), vscl(vdot(e, P) * L / EA, e))
        for r in range(3):
            F[r][k] = u[r]
            F[3 + r][k] = th[r]
    Kjj = mat_inv(F)
    Le = vscl(L, e)
    # Gamma: end-j actions -> end-i actions that equilibrate them: P_i = -P_j, M_i = -M_j - (L e) x P_j
    G = [[Fr(0)] * 6 for _ in range(6)]
    for r in range(3):
        G[r][r] = Fr(-1)
        G[3 + r][3 + r] = Fr(-1)
    # -(Le) x P = -[Le]x P
    sk = [[Fr(0), -Le[2], Le[1]], [Le[2], Fr(0), -Le[0]], [-Le[1], Le[0], Fr(0)]]
    for r in range(3):
        for c in range(3):
            G[3 + r][c] = -sk[r][c]
    GK = matmul(G, Kjj)
    GKGt = matmul(GK, transpose(G))
    KGt = matmul(Kjj, transpose(G))
    K = [[Fr(0)] * 12 for _ in range(12)]
    for r in range(6):
        for c in range(6):
            K[r][c] = GKGt[r][c]
            K[r][6 + c] = GK[r][c]
            K[6 + r][c] = KGt[r][c]
            K[6 + r][6 + c] = Kjj[r][c]
    return K


# ---------------------------------------------------------------- element: fixed-fixed state by exact ODE solution
def _pp(x, a, n):
    """Macaulay bracket (x - a)_+^n."""
    return (x - a) ** n if x > a else Fr(0)


class FixedState:
    """Fixed-fixed element state from span loads (exact ODE solution) and a constant axial force N0.

    Fields are the action of the outer part [x, L] on the inner part [0, x]: F(x) (force), M(x) (moment).
    """

    def __init__(self, L, e, EA, EI):
        self.L, self.e, self.EA, self.EI = L, e, EA, EI
        self.loads = []   # (q_ax, q_perp, A, B, c2, c3, Q2L)
        self.N0 = Fr(0)

    def add_span_load(self, q, a, b):
        L, e, EI = self.L, self.e, self.EI
        A, B = a * L, b * L
        q_ax = vdot(q, e)
        q_perp = vsub(q, vscl(q_ax, e))
        P4 = (_pp(L, A, 4) - _pp(L, B, 4)) / 24
        P3 = (_pp(L, A, 3) - _pp(L, B, 3)) / 6
        # c2 L^2 + c3 L^3 = -P4/EI ; 2 c2 L + 3 c3 L^2 = -P3/EI
        det = L * L * 3 * L * L - L ** 3 * 2 * L
        r1, r2 = -P4 / EI, -P3 / EI
        c2 = (r1 * 3 * L * L - L ** 3 * r2) / det
        c3 = (L * L * r2 - 2 * L * r1) / det
        Q2L = (_pp(L, A, 2) - _pp(L, B, 2)) / 2
        self.loads.append((q_ax, q_perp, A, B, c2, c3, Q2L))

    def add_axial(self, N0):
        self.N0 += N0

    def F(self, x):
        e, EI = self.e, self.EI
        tot = vscl(self.N0, e)
        for q_ax, q_perp, A, B, c2, c3, Q2L in self.loads:
            Q1 = _pp(x, A, 1) - _pp(x, B, 1)
            N = q_ax * (Q2L / self.L - Q1)
            phi3 = 6 * c3 + (_pp(x, A, 1) - _pp(x, B, 1)) / EI
            tot = vadd(tot, vadd(vscl(N, e), vscl(-EI * phi3, q_perp)))
        return tot

    def M(self, x):
        e, EI = self.e, self.EI
        tot = ZERO3
        for q_ax, q_perp, A, B, c2, c3, Q2L in self.loads:
            phi2 = 2 * c2 + 6 * c3 * x + (_pp(x, A, 2) - _pp(x, B, 2)) / (2 * EI)
            tot = vadd(tot, vscl(EI * phi2, vcross(e, q_perp)))
        return tot

    def end_reactions(self):
        """Support-on-element actions in the fixed state: (P_i, M_i, P_j, M_j)."""
        L = self.L
        F0, M0, FL, ML = self.F(Fr(0)), self.M(Fr(0)), self.F(L), self.M(L)
        return tuple(-x for x in F0) + tuple(-x for x in M0) + FL + ML


# ---------------------------------------------------------------- sections and model building
class Section:
    def __init__(self, E, G, OD, wall, mill):
        self.E, self.G, self.OD, self.wall, self.mill = E, G, OD, wall, mill
        self.t = wall - mill
        self.ID = OD - 2 * self.t
        self.A_over_pi = (OD ** 2 - self.ID ** 2) / 4
        self.I_over_pi = (OD ** 4 - self.ID ** 4) / 64
        self.A = PI * self.A_over_pi
        self.I = PI * self.I_over_pi
        self.J = 2 * self.I
        self.EA, self.EI, self.GJ = E * self.A, E * self.I, G * self.J


def mass_per_length(sec, rho_m, rho_c, rho_i, t_ins):
    """m' over pi, exact: metal + contents + insulation, one effective wall."""
    OD, ID = sec.OD, sec.ID
    return (rho_m * (OD ** 2 - ID ** 2) + rho_c * ID ** 2 + rho_i * ((OD + 2 * t_ins) ** 2 - OD ** 2)) / 4


class Problem:
    pass


def build(case, basis='int', mod=None):
    """Build the exact problem for a case. basis 'int' = authored decimals; 'rep' = binary64-decoded inputs."""
    inp = case['inputs']
    model = case['model']
    if basis == 'int':
        dec = Fr
    else:
        dec = lambda s: Fr(float(s))
    g = lambda k, default=None: dec(inp[k]) if k in inp else default
    pb = Problem()
    pb.case_id = case['id']
    pb.basis = basis
    pb.nodes = {}
    for n, xyz in model['nodes'].items():
        c = (g('X.' + n), g('Y.' + n), g('Z.' + n))
        assert c == tuple(Fr(v) for v in xyz) or basis == 'rep', (n, c, xyz)
        pb.nodes[n] = c
    secs = {}
    for m in model['members']:
        s = m['section']
        if s not in secs:
            secs[s] = Section(g('sec.%s.E' % s), g('sec.%s.G' % s), g('sec.%s.OD' % s), g('sec.%s.wall' % s),
                              g('sec.%s.mill_tolerance' % s, Fr(0)))
    pb.sections = secs
    pb.members = []
    for m in model['members']:
        d = vsub(pb.nodes[m['j']], pb.nodes[m['i']])
        L = fsqrt_exact(vdot(d, d))
        e = vscl(1 / L, d)
        sec = secs[m['section']]
        if basis == 'int':
            assert L == Fr(m['L_m'])
            assert sec.EA / PI == Fr(m['EA_over_pi']) and sec.EI / PI == Fr(m['EI_over_pi']) and sec.GJ / PI == Fr(m['GJ_over_pi']), m['name']
        pb.members.append(dict(name=m['name'], i=m['i'], j=m['j'], L=L, e=e, sec=sec,
                               yref=tuple(Fr(v) for v in m['y_reference'])))
    pb.mem = {m['name']: m for m in pb.members}
    pb.restraints = {}
    for n, dd in model['restraints'].items():
        pb.restraints[n] = {}
        for dof, v in dd.items():
            val = g('presc.%s.%s' % (n, dof), Fr(0))
            if basis == 'int':
                assert val == Fr(v), (n, dof)
            pb.restraints[n][dof] = val
    pb.springs = []
    for s in model['springs']:
        k = g('k.%s.%s' % (s['node'], s['dof']))
        if basis == 'int':
            assert k == Fr(s['k'])
        pb.springs.append((s['node'], s['dof'], k))
    pb.nodal = []
    seen = {}
    for nl in model['nodal_loads_in_authored_order']:
        base = 'P.%s.%s' % (nl['node'], nl['dof'])
        seen[base] = seen.get(base, 0) + 1
        key = base if seen[base] == 1 else '%s#%d' % (base, seen[base])
        v = g(key)
        if basis == 'int':
            assert v == Fr(nl['value']), key
        pb.nodal.append((nl['node'], nl['dof'], v, key))
    pb.efforts = []
    for ce in model['constant_efforts']:
        v = g('CE.%s.%s' % (ce['node'], ce['dof']))
        if basis == 'int':
            assert v == Fr(ce['value'])
        assert v > 0
        pb.efforts.append((ce['node'], ce['dof'], v))
    # generated-load inputs
    gen = {k[4:]: dec(v) for k, v in inp.items() if k.startswith('gen.')}
    pb.gen = gen
    gi = {}
    for rec in case.get('generated_intensities', []):
        gi.setdefault((rec['member'], rec['kind']), []).append(rec)
    pb.eloads = []
    for el in model['element_loads']:
        mem = pb.mem[el['member']]
        key = el['key']
        if el['source'] == 'authored':
            a = g(key + '.a', Fr(0))
            b = g(key + '.b', Fr(1))
            comp = (g(key + '.x', Fr(0)), g(key + '.y', Fr(0)), g(key + '.z', Fr(0)))
            if basis == 'int':
                assert comp == tuple(Fr(v) for v in el['authored']), key
            if el['frame'] == 'local':
                yr = tuple(Fr(v) for v in el['y_reference'])
                assert yr == mem['yref']
                q = local_to_global(mem['e'], yr, comp)
            else:
                q = comp
            kind = 'authored'
        elif el['source'] == 'generated seismic':
            a, b = Fr(el['extent'][0]), Fr(el['extent'][1])
            mp = mass_per_length(mem['sec'], gen['rho_metal'], gen['rho_contents'], gen['rho_insulation'],
                                 gen['t_insulation']) * PI
            q = tuple(gen.get('g_factor.' + ax, Fr(0)) * gen['g'] * mp for ax in 'XYZ')
            kind = 'seismic'
        elif el['source'] == 'generated wind':
            ext = el['extent']
            a, b = ((g(key + '.a', Fr(0)), g(key + '.b', Fr(1))) if (key + '.a') in inp or (key + '.b') in inp
                    else (Fr(ext[0]), Fr(ext[1])))
            axis = gi[(el['member'], 'wind')][0]['axis']
            w = gen['wind_pressure'] * gen['shape_factor'] * (mem['sec'].OD + 2 * gen['t_insulation'])
            q = vscl(w, AXES[axis])
            kind = 'wind'
        else:
            raise ValueError(el['source'])
        if basis == 'int':
            assert (a, b) == (Fr(el['extent'][0]), Fr(el['extent'][1])), key
        pb.eloads.append(dict(member=el['member'], key=key, q=q, a=a, b=b, kind=kind, frame=el['frame'],
                              comp=None if kind != 'authored' else comp))
    pb.eigen = {}
    for ei in model['eigen']:
        m = ei['member']
        pb.eigen[m] = eps_star(inp, m, dec, pb.mem[m]['L'])
        if basis == 'int':
            assert pb.eigen[m] == Fr(ei['eps_exact']), m
    pb.thrust = {}
    for t in model['thrust']:
        m = t['member']
        p = g('p.' + m)
        sec = pb.mem[m]['sec']
        pb.thrust[m] = p * PI * sec.ID ** 2 / 4
        if basis == 'int':
            assert pb.thrust[m] / PI == Fr(t['F_p_over_pi'])
    if mod:
        mod(pb)
    return pb


def local_to_global(e, yref, comp):
    y = vsub(yref, vscl(vdot(yref, e), e))
    ny = fsqrt_exact(vdot(y, y))
    y = vscl(1 / ny, y)
    z = vcross(e, y)
    return vadd(vadd(vscl(comp[0], e), vscl(comp[1], y)), vscl(comp[2], z))


def local_frame(e, yref):
    y = vsub(yref, vscl(vdot(yref, e), e))
    ny = fsqrt_exact(vdot(y, y))
    y = vscl(1 / ny, y)
    return e, y, vcross(e, y)


def eps_star(inp, m, dec, L):
    if ('th.%s.alpha' % m) in inp:
        return dec(inp['th.%s.alpha' % m]) * dec(inp['th.%s.DeltaT' % m])
    Td = dec(inp['th.%s.T_datum' % m])
    Ti = dec(inp['th.%s.T_install' % m])
    To = dec(inp['th.%s.T_operating' % m])
    ai = dec(inp['th.%s.alpha_sec(T_install)' % m])
    ao = dec(inp['th.%s.alpha_sec(T_operating)' % m])
    lam = lambda a, T: 1 + a * (T - Td)
    lth = lam(ao, To) / lam(ai, Ti)
    fk = 'fit.%s.delta_L' % m
    lfit = 1 + dec(inp[fk]) / L if fk in inp else Fr(1)
    return lfit * lth - 1


# ---------------------------------------------------------------- global solve
class Solution:
    pass


def solve_problem(pb, flags=None):
    """Direct stiffness solve. flags: optional defect switches used by the negative-control models."""
    flags = flags or {}
    names = list(pb.nodes)
    idx = {n: k for k, n in enumerate(names)}
    ndof = 6 * len(names)
    K = [[Fr(0)] * ndof for _ in range(ndof)]
    f = [Fr(0)] * ndof
    elem = {}
    for m in pb.members:
        sec = m['sec']
        L, e = m['L'], m['e']
        Ke = element_stiffness(L, e, sec.EA, sec.EI, sec.GJ)
        fs = FixedState(L, e, sec.EA, sec.EI)
        for el in pb.eloads:
            if el['member'] == m['name']:
                fs.add_span_load(el['q'], el['a'], el['b'])
        if m['name'] in pb.eigen:
            fs.add_axial(-sec.EA * pb.eigen[m['name']])
        if m['name'] in pb.thrust:
            fs.add_axial(-pb.thrust[m['name']])
        Pfix = fs.end_reactions()
        dmap = [6 * idx[m['i']] + k for k in range(6)] + [6 * idx[m['j']] + k for k in range(6)]
        for r in range(12):
            f[dmap[r]] -= Pfix[r]
            for c in range(12):
                K[dmap[r]][dmap[c]] += Ke[r][c]
        elem[m['name']] = (Ke, fs, Pfix, dmap)
    for n, dof, v, _ in pb.nodal:
        f[6 * idx[n] + DOFS.index(dof)] += v
    for n, dof, v in pb.efforts:
        f[6 * idx[n] + DOFS.index(dof)] += v
    for n, dof, k in pb.springs:
        K[6 * idx[n] + DOFS.index(dof)][6 * idx[n] + DOFS.index(dof)] += k
    presc = {}
    for n, dd in pb.restraints.items():
        for dof, v in dd.items():
            presc[6 * idx[n] + DOFS.index(dof)] = v
    free = [i for i in range(ndof) if i not in presc]
    rhs = [f[i] - sum(K[i][j] * v for j, v in presc.items()) for i in free]
    uf = solve([[K[i][j] for j in free] for i in free], rhs) if free else []
    u = [Fr(0)] * ndof
    for i, v in zip(free, uf):
        u[i] = v
    for i, v in presc.items():
        u[i] = v
    sol = Solution()
    sol.u = {n: tuple(u[6 * idx[n]:6 * idx[n] + 6]) for n in names}
    sol.R = {}
    for i in presc:
        n, dof = names[i // 6], DOFS[i % 6]
        sol.R[(n, dof)] = sum(K[i][j] * u[j] for j in range(ndof)) - f[i]
    sol.S = {(n, dof): -k * u[6 * idx[n] + DOFS.index(dof)] for n, dof, k in pb.springs}
    sol.members = {}
    for m in pb.members:
        Ke, fs, Pfix, dmap = elem[m['name']]
        ue = [u[d] for d in dmap]
        Pd = [sum(Ke[r][c] * ue[c] for c in range(12)) for r in range(12)]
        Ptot = [a + b for a, b in zip(Pd, Pfix)]
        L, e = m['L'], m['e']
        Pdj, Mdj = tuple(Pd[6:9]), tuple(Pd[9:12])
        rec = {'Pend': Ptot}
        st = {}
        for sname, frac in STATIONS:
            x = frac * L
            Fx = vadd(fs.F(x), Pdj)
            Mx = vadd(fs.M(x), vadd(Mdj, vscl(L - x, vcross(e, Pdj))))
            if flags.get('no_eigen_recovery') and m['name'] in pb.eigen:
                Fx = vadd(Fx, vscl(m['sec'].EA * pb.eigen[m['name']], e))
            if flags.get('thrust_no_fec') and m['name'] in pb.thrust:
                Fx = vadd(Fx, vscl(pb.thrust[m['name']], e))
            st[sname] = (Fx, Mx)
        # consistency: statics from end j must reproduce end-i actions
        F0, M0 = st['i']
        if not flags:
            assert vadd(F0, tuple(Ptot[0:3])) == ZERO3 and vadd(M0, tuple(Ptot[3:6])) == ZERO3, m['name']
        ui, ti = sol.u[m['i']][:3], sol.u[m['i']][3:]
        uj, tj = sol.u[m['j']][:3], sol.u[m['j']][3:]
        rec['stations'] = st
        rec['tw'] = vdot(vsub(tj, ti), e)
        rec['ext'] = vdot(vsub(uj, ui), e)
        sol.members[m['name']] = rec
    sol.K, sol.f, sol.uvec, sol.idx, sol.names = K, f, u, idx, names
    check_equilibrium(pb, sol)
    return sol


def check_equilibrium(pb, sol):
    """Global force and moment balance about the origin, exactly."""
    Ft, Mt = ZERO3, ZERO3

    def add(pt, F, M=ZERO3):
        nonlocal Ft, Mt
        Ft = vadd(Ft, F)
        Mt = vadd(Mt, vadd(M, vcross(pt, F)))

    def dvec(dof, v):
        k = DOFS.index(dof)
        z = [Fr(0)] * 6
        z[k] = v
        return tuple(z[:3]), tuple(z[3:])

    for (n, dof), v in list(sol.R.items()) + list(sol.S.items()):
        F, M = dvec(dof, v)
        add(pb.nodes[n], F, M)
    for n, dof, v, _ in pb.nodal:
        F, M = dvec(dof, v)
        add(pb.nodes[n], F, M)
    for n, dof, v in pb.efforts:
        F, M = dvec(dof, v)
        add(pb.nodes[n], F, M)
    for el in pb.eloads:
        m = pb.mem[el['member']]
        xi = pb.nodes[m['i']]
        W = vscl((el['b'] - el['a']) * m['L'], el['q'])
        c = vadd(xi, vscl((el['a'] + el['b']) / 2 * m['L'], m['e']))
        add(c, W)
    # thrust caps and eigen pairs are self-equilibrated on each member
    assert Ft == ZERO3 and Mt == ZERO3, (pb.case_id, Ft, Mt)


# ---------------------------------------------------------------- publication (the package's quantity set)
def publish(pb, sol):
    out = {}
    for n in pb.nodes:
        u = sol.u[n]
        for k, d in enumerate(('UX', 'UY', 'UZ')):
            out['u.%s.%s' % (n, d)] = ('v', u[k])
        for k, d in enumerate(('RX', 'RY', 'RZ')):
            out['th.%s.%s' % (n, d)] = ('v', u[3 + k])
    for (n, dof), v in sol.R.items():
        out['R.%s.%s' % (n, dof)] = ('v', v)
    for (n, dof), v in sol.S.items():
        out['S.%s.%s' % (n, 'F' + dof[1].lower() if dof[0] == 'U' else 'M' + dof[1].lower())] = ('v', v)
    for m in pb.members:
        rec = sol.members[m['name']]
        e = m['e']
        for sname, _ in STATIONS:
            Fx, Mx = rec['stations'][sname]
            T = vdot(Mx, e)
            Mbv = vsub(Mx, vscl(T, e))
            if sname in ('i', 'mid', 'j'):
                out['N.%s.%s' % (m['name'], sname)] = ('v', vdot(Fx, e))
                out['T.%s.%s' % (m['name'], sname)] = ('v', T)
            out['Mb.%s.%s' % (m['name'], sname)] = ('sq', vdot(Mbv, Mbv), Mbv)
        out['tw.%s' % m['name']] = ('v', rec['tw'])
        out['ext.%s' % m['name']] = ('v', rec['ext'])
    return out


def to_dec(entry, prec=80):
    with localcontext() as c:
        c.prec = prec
        if entry[0] == 'v':
            q = entry[1]
            return Decimal(q.numerator) / Decimal(q.denominator)
        q = entry[1]
        return (Decimal(q.numerator) / Decimal(q.denominator)).sqrt()


def run(case, basis='int', mod=None, flags=None):
    pb = build(case, basis, mod)
    sol = solve_problem(pb, flags)
    return pb, sol, publish(pb, sol)
