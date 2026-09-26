"""V3 step 4: negative controls (non-cancellation families) and constructed defects.

Usage: python3 v3_nc.py <references_eload.json> <out v3_nc.json>
Every listed control of the non-cancellation families is rebuilt from V3's own defect model (from its
text in the package), solved exactly, and evaluated with the criterion under the published class scales.
The flag and the worst ratio are compared with the package's. Then V3's constructed defects (not
listed by the author) are evaluated in every case of their family: a defect is caught when at least
one value of some case fails the criterion.
(The cancellation-case controls are in v3_cancel.py.)
"""
import copy
import json
import math
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as Fr
import v3_core as V
from v3_cancel import seis_bin64

getcontext().prec = 80
D = Decimal
CRIT = D('1e-9')


def dq(q):
    return D(q.numerator) / D(q.denominator)


def add_nodal_vec(p, node, vec, tag, rot=False):
    for k, v in enumerate(vec):
        if v != 0:
            p.nodal.append((node, ('RX', 'RY', 'RZ')[k] if rot else ('UX', 'UY', 'UZ')[k], v, tag))


def lumped(p, which='all'):
    keep = []
    for el in p.eloads:
        partial = not (el['a'] == 0 and el['b'] == 1)
        if which == 'partial' and not partial:
            keep.append(el)
            continue
        m = p.mem[el['member']]
        W = V.vscl((el['b'] - el['a']) * m['L'], el['q'])
        if which == 'all':
            ci = cj = Fr(1, 2)
        else:
            c = (el['a'] + el['b']) / 2
            ci, cj = 1 - c, c
        add_nodal_vec(p, m['i'], V.vscl(ci, W), 'lump')
        add_nodal_vec(p, m['j'], V.vscl(cj, W), 'lump')
    p.eloads = keep


def reframe(p, how):
    for el in p.eloads:
        if el['kind'] != 'authored':
            continue
        m = p.mem[el['member']]
        e, y, z = V.local_frame(m['e'], m['yref'])
        c = el['comp']
        if how == 'global_as_local' and el['frame'] == 'global':
            el['q'] = V.vadd(V.vadd(V.vscl(c[0], e), V.vscl(c[1], y)), V.vscl(c[2], z))
        elif how == 'local_as_global' and el['frame'] == 'local':
            el['q'] = tuple(c)
        elif how == 'transposed' and el['frame'] == 'local':
            el['q'] = (V.vdot(e, c), V.vdot(y, c), V.vdot(z, c))
        elif how == 'left_handed' and el['frame'] == 'local':
            z2 = V.vcross(y, e)
            el['q'] = V.vadd(V.vadd(V.vscl(c[0], e), V.vscl(c[1], y)), V.vscl(c[2], z2))


def eps_variant(case, p, kind):
    inp = case['inputs']
    dec = Fr if p.basis == 'int' else (lambda s: Fr(float(s)))
    for m in list(p.eigen):
        if ('th.%s.T_datum' % m) not in inp:
            if kind in ('sign',):
                p.eigen[m] = -p.eigen[m]
            elif kind == 'omit':
                del p.eigen[m]
            continue
        Td, Ti, To = (dec(inp['th.%s.%s' % (m, k)]) for k in ('T_datum', 'T_install', 'T_operating'))
        ai, ao = dec(inp['th.%s.alpha_sec(T_install)' % m]), dec(inp['th.%s.alpha_sec(T_operating)' % m])
        lam = lambda a, T: 1 + a * (T - Td)
        L = p.mem[m]['L']
        fk = 'fit.%s.delta_L' % m
        dL = dec(inp[fk]) if fk in inp else Fr(0)
        eth = lam(ao, To) / lam(ai, Ti) - 1
        if kind == 'legacy_form':
            eth = ao * (To - Ti)
        elif kind == 'subtract':
            eth = lam(ao, To) - lam(ai, Ti)
        elif kind == 'datum_as_install':
            eth = lam(ao, To) - 1
        if kind == 'fit_additive':
            p.eigen[m] = dL / L + eth
        elif kind == 'fit_omit':
            p.eigen[m] = eth
        elif kind == 'fit_total_length':
            Ltot = sum(mm['L'] for mm in p.members)
            p.eigen[m] = (1 + dL / Ltot) * (1 + eth) - 1
        elif kind == 'sign':
            p.eigen[m] = -p.eigen[m]
        elif kind == 'omit':
            del p.eigen[m]
        else:
            p.eigen[m] = (1 + dL / L) * (1 + eth) - 1


def gen_variant(case, p, kind):
    g = p.gen
    for el in p.eloads:
        m = p.mem[el['member']]
        sec = m['sec']
        if el['kind'] == 'seismic':
            rho_i, rho_c, sec_m, tins = g['rho_insulation'], g['rho_contents'], sec, g['t_insulation']
            if kind == 'no_ins_mass':
                rho_i = Fr(0)
            if kind == 'no_contents':
                rho_c = Fr(0)
            if kind in ('nominal_wall_mass', 'nominal_wall'):
                sec_m = V.Section(sec.E, sec.G, sec.OD, sec.wall, Fr(0))
            mp = V.mass_per_length(sec_m, g['rho_metal'], rho_c, rho_i, tins) * V.PI
            if kind == 'contents_nominal_ID':
                secn = V.Section(sec.E, sec.G, sec.OD, sec.wall, Fr(0))
                mp = (g['rho_metal'] * (sec.OD ** 2 - sec.ID ** 2) + rho_c * secn.ID ** 2 +
                      rho_i * ((sec.OD + 2 * tins) ** 2 - sec.OD ** 2)) / 4 * V.PI
            gf = [g.get('g_factor.' + ax, Fr(0)) for ax in 'XYZ']
            if kind == 'axes_swapped':
                gf = [gf[0], gf[2], gf[1]]
            el['q'] = tuple(x * g['g'] * mp for x in gf)
            if kind == 'bin64':
                el['q'] = tuple(Fr(seis_bin64(case['inputs'], ax)) if ('gen.g_factor.' + ax) in case['inputs'] else Fr(0)
                                for ax in 'XYZ')
        elif el['kind'] == 'wind':
            Dexp = sec.OD + 2 * g['t_insulation']
            if kind == 'no_ins_diam':
                Dexp = sec.OD
            if kind == 'ins_one_side':
                Dexp = sec.OD + g['t_insulation']
            w = g['wind_pressure'] * g['shape_factor'] * Dexp
            ax = [k for k in range(3) if el['q'][k] != 0][0]
            if kind == 'bin64':
                inp = case['inputs']
                w = Fr(float(inp['gen.wind_pressure']) * float(inp['gen.shape_factor']) *
                       (float(inp['sec.G.OD']) + 2 * float(inp['gen.t_insulation'])))
            if kind == 'projected':
                d = [Fr(0)] * 3
                d[ax] = Fr(1)
                c = V.vdot(m['e'], d)
                w = w * V.fsqrt_exact(1 - c * c)
            q = [Fr(0)] * 3
            q[ax] = w
            el['q'] = tuple(q)
    if kind == 'nominal_wall':
        for m in p.members:
            s = m['sec']
            m['sec'] = V.Section(s.E, s.G, s.OD, s.wall, Fr(0))
    if kind == 'wind_unmarked':
        winds = [el for el in p.eloads if el['kind'] == 'wind']
        marked = {el['member'] for el in winds}
        for m in p.members:
            if m['name'] not in marked:
                p.eloads.append(dict(winds[0], member=m['name'], a=Fr(0), b=Fr(1)))
    if kind == 'subspan_hull':
        winds = [el for el in p.eloads if el['kind'] == 'wind']
        if len(winds) > 1:
            a, b = min(w['a'] for w in winds), max(w['b'] for w in winds)
            p.eloads = [el for el in p.eloads if el['kind'] != 'wind'] + [dict(winds[0], a=a, b=b)]


def thrust_variant(p, kind):
    for m, Fp in list(p.thrust.items()):
        sec = p.mem[m]['sec']
        pr = Fp / (V.PI * sec.ID ** 2 / 4)
        if kind == 'sign':
            p.thrust[m] = -Fp
        elif kind == 'omit':
            del p.thrust[m]
        elif kind == 'steel':
            p.thrust[m] = pr * sec.A
        elif kind == 'od':
            p.thrust[m] = pr * V.PI * sec.OD ** 2 / 4
        elif kind == 'mean_diameter':
            p.thrust[m] = pr * V.PI * (sec.OD - sec.t) ** 2 / 4
        elif kind == 'nominal_wall':
            secn = V.Section(sec.E, sec.G, sec.OD, sec.wall, Fr(0))
            p.thrust[m] = pr * V.PI * secn.ID ** 2 / 4
            p.mem[m]['sec'] = secn
            for mm in p.members:
                if mm['name'] == m:
                    mm['sec'] = secn


def no_fec_outputs(pb, sol, out):
    """Correct nodal solution; member actions from K_e u_e (no fixed-end forces), stations by statics from end i."""
    o = dict(out)
    for m in pb.members:
        rec = sol.members[m['name']]
        fs = V.FixedState(m['L'], m['e'], m['sec'].EA, m['sec'].EI)
        for el in pb.eloads:
            if el['member'] == m['name']:
                fs.add_span_load(el['q'], el['a'], el['b'])
        # only the element-load part of the fixed state is omitted (eigen and thrust are separate controls)
        Pfix = fs.end_reactions()
        P = [a - b for a, b in zip(rec['Pend'], Pfix)]
        Pi, Mi = tuple(P[0:3]), tuple(P[3:6])
        e, L = m['e'], m['L']
        for sname, frac in V.STATIONS:
            x = frac * L
            F = V.vscl(-1, Pi)
            M = V.vadd(V.vscl(-1, Mi), V.vscl(x, V.vcross(e, Pi)))
            for el in pb.eloads:
                if el['member'] != m['name']:
                    continue
                A, B = el['a'] * L, el['b'] * L
                hi = min(x, B)
                if hi > A:
                    F = V.vsub(F, V.vscl(hi - A, el['q']))
                    # int_A^hi (x - t) dt = ((x-A)^2 - (x-hi)^2)/2
                    M = V.vadd(M, V.vscl(((x - A) ** 2 - (x - hi) ** 2) / 2, V.vcross(e, el['q'])))
            if m['name'] in pb.eigen:
                F = V.vsub(F, V.vscl(m['sec'].EA * pb.eigen[m['name']], e))
            if m['name'] in pb.thrust:
                F = V.vsub(F, V.vscl(pb.thrust[m['name']], e))
            T = V.vdot(M, e)
            Mbv = V.vsub(M, V.vscl(T, e))
            if sname in ('i', 'mid', 'j'):
                o['N.%s.%s' % (m['name'], sname)] = ('v', V.vdot(F, e))
                o['T.%s.%s' % (m['name'], sname)] = ('v', T)
            o['Mb.%s.%s' % (m['name'], sname)] = ('sq', V.vdot(Mbv, Mbv), Mbv)
    return o


def lost_soft(p):
    new = []
    for n, dof, k in p.springs:
        # the member through the spring node along the spring axis: EA/L of the member ending at n
        m = [mm for mm in p.members if n in (mm['i'], mm['j'])][0]
        ka = float(m['sec'].EA / m['L'])
        keff = Fr(float(ka + float(k))) - Fr(ka)
        new.append((n, dof, keff))
    p.springs = new


def pm_variant(p, kind):
    for n, dd in p.restraints.items():
        for dof in list(dd):
            if kind == 'omit':
                dd[dof] = Fr(0)
            elif kind == 'sign':
                dd[dof] = -dd[dof]
            elif kind == 'rot_length' and dof[0] == 'R':
                dd[dof] = dd[dof] * max(m['L'] for m in p.members)
    if kind == 'first_only':
        for n, dd in p.restraints.items():
            nz = [d for d in V.DOFS if d in dd and dd[d] != 0]
            for d in nz[1:]:
                dd[d] = Fr(0)


def ce_variant(p, kind):
    if kind == 'sign':
        p.efforts = [(n, d, -v) for n, d, v in p.efforts]
    elif kind == 'omit':
        p.efforts = []
    elif kind == 'restraint':
        for n, d, v in p.efforts:
            p.restraints.setdefault(n, {})[d] = Fr(0)
        p.efforts = []
    elif kind == 'wrong_axis':
        p.efforts = [(n, 'UY', v) for n, d, v in p.efforts]
    elif kind == 'double':
        p.efforts = p.efforts + p.efforts


def extent_from_j(p):
    for el in p.eloads:
        el['a'], el['b'] = 1 - el['b'], 1 - el['a']


def fem_moment_sign(p):
    """FEM moments reversed: add twice the negative of the consistent end moments as nodal moments."""
    p._fem_moment_sign = True


def evaluate(case, out, scales):
    exp = {r[0]: (D(r[1]), r[2]) for r in case['expected']}
    w, wk, nf = D(0), None, 0
    for k, (e, c) in exp.items():
        o = V.to_dec(out[k])
        s = scales[c]
        r = abs(o - e) / (CRIT * max(abs(e), s))
        if r > 1:
            nf += 1
        if r > w:
            w, wk = r, k
    return w, wk, nf


def run_defect(case, basis, name):
    """Return the defect's published outputs, or None if the defect does not apply to the case."""
    flags = {}
    post = None
    mod = None
    nm = name
    if nm == 'NC-LUMPED-5050':
        mod = lambda p: lumped(p, 'all')
    elif nm == 'NC-LEVER-RULE':
        mod = lambda p: lumped(p, 'partial')
    elif nm == 'NC-PARTIAL-AS-FULL':
        def mod(p):
            for el in p.eloads:
                el['a'], el['b'] = Fr(0), Fr(1)
    elif nm == 'NC-NO-FEC':
        post = no_fec_outputs
    elif nm == 'NC-GLOBAL-AS-LOCAL':
        mod = lambda p: reframe(p, 'global_as_local')
    elif nm == 'NC-LOCAL-AS-GLOBAL':
        mod = lambda p: reframe(p, 'local_as_global')
    elif nm == 'NC-TRANSPOSED-FRAME':
        mod = lambda p: reframe(p, 'transposed')
    elif nm == 'NC-EIGEN-SIGN':
        mod = lambda p: eps_variant(case, p, 'sign')
    elif nm == 'NC-EIGEN-OMITTED':
        mod = lambda p: eps_variant(case, p, 'omit')
    elif nm == 'NC-EIGEN-RECOVERY-OMITTED':
        flags['no_eigen_recovery'] = True
    elif nm == 'NC-ALPHA-TIMES-INTERVAL':
        mod = lambda p: eps_variant(case, p, 'legacy_form')
    elif nm == 'NC-SUBTRACT-DILATIONS':
        mod = lambda p: eps_variant(case, p, 'subtract')
    elif nm == 'NC-FIT-ADDITIVE':
        mod = lambda p: eps_variant(case, p, 'fit_additive')
    elif nm == 'NC-FIT-OMITTED':
        mod = lambda p: eps_variant(case, p, 'fit_omit')
    elif nm == 'NC-LOST-SOFT':
        mod = lost_soft
    elif nm == 'NC-THRUST-NO-FEC':
        flags['thrust_no_fec'] = True
    elif nm == 'NC-THRUST-SIGN':
        mod = lambda p: thrust_variant(p, 'sign')
    elif nm == 'NC-THRUST-OMITTED':
        mod = lambda p: thrust_variant(p, 'omit')
    elif nm == 'NC-THRUST-STEEL-AREA':
        mod = lambda p: thrust_variant(p, 'steel')
    elif nm == 'NC-THRUST-OD-AREA':
        mod = lambda p: thrust_variant(p, 'od')
    elif nm == 'NC-NOMINAL-WALL':
        if 'generated_intensities' in case:
            mod = lambda p: gen_variant(case, p, 'nominal_wall')
        else:
            mod = lambda p: thrust_variant(p, 'nominal_wall')
    elif nm == 'NC-CE-SIGN':
        mod = lambda p: ce_variant(p, 'sign')
    elif nm == 'NC-CE-OMITTED':
        mod = lambda p: ce_variant(p, 'omit')
    elif nm == 'NC-CE-AS-RESTRAINT':
        mod = lambda p: ce_variant(p, 'restraint')
    elif nm == 'NC-BIN64-PRODUCT':
        mod = lambda p: gen_variant(case, p, 'bin64')
    elif nm == 'NC-NO-INSULATION-MASS':
        mod = lambda p: gen_variant(case, p, 'no_ins_mass')
    elif nm == 'NC-NOMINAL-WALL-MASS':
        mod = lambda p: gen_variant(case, p, 'nominal_wall_mass')
    elif nm == 'NC-NO-INSULATION-DIAMETER':
        mod = lambda p: gen_variant(case, p, 'no_ins_diam')
    elif nm == 'NC-WIND-UNMARKED':
        mod = lambda p: gen_variant(case, p, 'wind_unmarked')
    elif nm == 'NC-PROJECTED-WIND':
        mod = lambda p: gen_variant(case, p, 'projected')
    elif nm == 'NC-PM-OMITTED':
        mod = lambda p: pm_variant(p, 'omit')
    elif nm == 'NC-PM-SIGN':
        mod = lambda p: pm_variant(p, 'sign')
    # ---- constructed (V3) defects
    elif nm == 'X-EXTENT-FROM-J':
        if not any(not (el['a'] == 0 and el['b'] == 1) for el in V.build(case, basis).eloads):
            return None
        mod = extent_from_j
    elif nm == 'X-LEFT-HANDED-LOCAL-Z':
        if not any(el['frame'] == 'local' for el in case['model']['element_loads']):
            return None
        mod = lambda p: reframe(p, 'left_handed')
    elif nm == 'X-STATIONS-FROM-J':
        def post(pb, sol, out):
            o = dict(out)
            for m in pb.members:
                for a, b in (('i', 'j'), ('q1', 'q3')):
                    o['Mb.%s.%s' % (m['name'], a)], o['Mb.%s.%s' % (m['name'], b)] = out['Mb.%s.%s' % (m['name'], b)], out['Mb.%s.%s' % (m['name'], a)]
            return o
    elif nm == 'X-FEM-MOMENTS-ONLY-OMITTED':
        # consistent end forces kept, fixed-end moments omitted from the load vector (a half-consistent load)
        def mod(p):
            p._drop_fem_moments = True
    elif nm == 'X-EPS-DATUM-AS-INSTALL':
        if not any(k.startswith('th.') and k.endswith('.T_datum') for k in case['inputs']):
            return None
        mod = lambda p: eps_variant(case, p, 'datum_as_install')
    elif nm == 'X-FIT-TOTAL-LENGTH':
        if not any(k.startswith('fit.') for k in case['inputs']):
            return None
        mod = lambda p: eps_variant(case, p, 'fit_total_length')
    elif nm == 'X-EIGEN-E-OF-FIRST-MEMBER':
        secs = {m['section'] for m in case['model']['members']}
        if len(case['model']['eigen']) == 0 or len(secs) < 2:
            return None

        def mod(p):
            E0 = p.members[0]['sec'].E
            for m in list(p.eigen):
                s = p.mem[m]['sec']
                p.eigen[m] = p.eigen[m] * E0 / s.E
    elif nm == 'X-THRUST-MEAN-DIAMETER':
        if not case['model']['thrust']:
            return None
        mod = lambda p: thrust_variant(p, 'mean_diameter')
    elif nm == 'X-CAP-AT-J-ONLY':
        if not case['model']['thrust']:
            return None

        def mod(p):
            for m, Fp in list(p.thrust.items()):
                mm = p.mem[m]
                add_nodal_vec(p, mm['i'], V.vscl(Fp, mm['e']), 'cap-undo')  # removes the i-end cap
    elif nm == 'X-CE-WRONG-AXIS':
        if not case['model']['constant_efforts'] or all(c['dof'] == 'UY' for c in case['model']['constant_efforts']):
            return None
        mod = lambda p: ce_variant(p, 'wrong_axis')
    elif nm == 'X-CE-DOUBLE':
        if not case['model']['constant_efforts']:
            return None
        mod = lambda p: ce_variant(p, 'double')
    elif nm in ('X-NO-CONTENTS-MASS', 'X-G-FACTOR-Y-Z-SWAPPED', 'X-CONTENTS-NOMINAL-ID'):
        if not any(el['source'] == 'generated seismic' for el in case['model']['element_loads']):
            return None
        kind = {'X-NO-CONTENTS-MASS': 'no_contents', 'X-G-FACTOR-Y-Z-SWAPPED': 'axes_swapped',
                'X-CONTENTS-NOMINAL-ID': 'contents_nominal_ID'}[nm]
        mod = lambda p: gen_variant(case, p, kind)
    elif nm in ('X-INSULATION-ONE-SIDE', 'X-SUBSPAN-HULL'):
        if not any(el['source'] == 'generated wind' for el in case['model']['element_loads']):
            return None
        kind = {'X-INSULATION-ONE-SIDE': 'ins_one_side', 'X-SUBSPAN-HULL': 'subspan_hull'}[nm]
        if nm == 'X-SUBSPAN-HULL' and sum(1 for el in case['model']['element_loads'] if el['source'] == 'generated wind') < 2:
            return None
        mod = lambda p: gen_variant(case, p, kind)
    elif nm in ('X-PM-ROTATION-TIMES-L', 'X-PM-FIRST-COMPONENT-ONLY'):
        kind = {'X-PM-ROTATION-TIMES-L': 'rot_length', 'X-PM-FIRST-COMPONENT-ONLY': 'first_only'}[nm]
        pr = [(n, d) for n, dd in case['model']['restraints'].items() for d, v in dd.items() if Fr(v) != 0]
        if not pr or (kind == 'rot_length' and not any(d[0] == 'R' for n, d in pr)):
            return None
        mod = lambda p: pm_variant(p, kind)
    elif nm == 'X-REACTION-WITHOUT-F':
        if not case['model']['element_loads']:
            return None

        def post(pb, sol, out):
            o = dict(out)
            for (n, dof) in sol.R:
                i = sol.idx[n] * 6 + V.DOFS.index(dof)
                o['R.%s.%s' % (n, dof)] = ('v', sum(sol.K[i][j] * sol.uvec[j] for j in range(len(sol.uvec))))
            return o
    else:
        raise KeyError(nm)
    pb = V.build(case, basis, mod)
    if getattr(pb, '_drop_fem_moments', False):
        if not pb.eloads:
            return None
        sol = solve_drop_fem_moments(pb)
    else:
        sol = V.solve_problem(pb, flags)
    out = V.publish(pb, sol)
    if post:
        out = post(pb, sol, out)
    return out


def solve_drop_fem_moments(pb):
    """Load vector with the consistent end forces but without the fixed-end moments; recovery as if unloaded
    for the moment part (i.e. the moment part of the fixed state is lost)."""
    # emulate: replace each element load by nodal forces equal to the consistent end forces (no moments)
    loads = pb.eloads
    pb.eloads = []
    for el in loads:
        m = pb.mem[el['member']]
        fs = V.FixedState(m['L'], m['e'], m['sec'].EA, m['sec'].EI)
        fs.add_span_load(el['q'], el['a'], el['b'])
        P = fs.end_reactions()
        add_nodal_vec(pb, m['i'], V.vscl(-1, tuple(P[0:3])), 'fem-f')
        add_nodal_vec(pb, m['j'], V.vscl(-1, tuple(P[6:9])), 'fem-f')
    return V.solve_problem(pb)


CONSTRUCTED = {
    'UDL': ['X-EXTENT-FROM-J', 'X-LEFT-HANDED-LOCAL-Z', 'X-STATIONS-FROM-J', 'X-FEM-MOMENTS-ONLY-OMITTED', 'X-REACTION-WITHOUT-F'],
    'TH': ['X-EPS-DATUM-AS-INSTALL', 'X-FIT-TOTAL-LENGTH', 'X-EIGEN-E-OF-FIRST-MEMBER'],
    'PT': ['X-THRUST-MEAN-DIAMETER', 'X-CAP-AT-J-ONLY'],
    'CE': ['X-CE-WRONG-AXIS', 'X-CE-DOUBLE'],
    'GEN': ['X-NO-CONTENTS-MASS', 'X-G-FACTOR-Y-Z-SWAPPED', 'X-CONTENTS-NOMINAL-ID', 'X-INSULATION-ONE-SIDE', 'X-SUBSPAN-HULL',
            'X-EXTENT-FROM-J'],
    'PM': ['X-PM-ROTATION-TIMES-L', 'X-PM-FIRST-COMPONENT-ONLY', 'X-REACTION-WITHOUT-F'],
    'COMB': [],
    'CANCEL': [],
}


def comb_controls(ref):
    C = {c['id']: c for c in ref['cases']}
    A = V.run(C['RF-ELOAD-UDL-CONT2-AX'])[2]
    B = V.run(C['RF-ELOAD-COMB-B-NODAL'])[2]
    res = {}
    for cid, sgn in (('RF-ELOAD-COMB-SUM', 1), ('RF-ELOAD-COMB-DIFF', -1)):
        case = C[cid]
        scales = {k: D(v['scale']) for k, v in case['classes'].items()}
        direct = V.run(case)[2]

        def comb(sa, sb, magsum=False, magdiff=False):
            o = {}
            for k in A:
                if A[k][0] == 'v':
                    o[k] = ('v', sa * A[k][1] + sb * B[k][1])
                else:
                    vec = V.vadd(V.vscl(sa, A[k][2]), V.vscl(sb, B[k][2]))
                    o[k] = ('sq', V.vdot(vec, vec), vec)
            if magsum or magdiff:
                for k in A:
                    if A[k][0] == 'sq':
                        a = V.to_dec(A[k])
                        b = V.to_dec(B[k])
                        mval = a + b if magsum else abs(a - b)
                        o[k] = ('v', Fr(mval))
            return o
        ok = all(V.to_dec(comb(1, sgn)[k]) == V.to_dec(direct[k]) for k in direct)
        cases = {'superposition_equals_direct_solve': ok}
        for name, o in (('NC-MAG-SUM', comb(1, sgn, magsum=True)), ('NC-WRONG-DIFFERENCE', comb(1, -sgn)),
                        ('NC-B-DROPPED', comb(1, 0)), ('X-DIFFERENCE-OF-MAGNITUDES', comb(1, sgn, magdiff=True)),
                        ('X-B-MOMENT-ROWS-SIGN', None)):
            if name == 'X-B-MOMENT-ROWS-SIGN':
                o = comb(1, sgn)
                for k in B:
                    if k.startswith('R.') and k.split('.')[-1][0] == 'R':
                        o[k] = ('v', A[k][1] - sgn * B[k][1])
            if name == 'NC-MAG-SUM' and sgn == -1:
                o = comb(1, sgn)
                for k in A:
                    if A[k][0] == 'sq':
                        o[k] = ('v', Fr(V.to_dec(A[k]) + V.to_dec(B[k])))
            w, wk, nf = evaluate(case, o, scales)
            cases[name] = (format(w, '.5e'), wk, nf)
        res[cid] = cases
    return res


def main():
    ref = json.load(open(sys.argv[1]))
    listed = {}
    mism = []
    # non-float controls of the cancellation cases, under the binding recommended scale
    for case in ref['cases']:
        cid = case['id']
        if 'cancellation' not in case:
            continue
        basis = 'int' if case['basis'] == 'intended' else 'rep'
        rec_s = {r[0]: D(r[3]) for r in case['expected']}
        exp = {r[0]: D(r[1]) for r in case['expected']}
        rec = {}
        for c in case['negative_controls']:
            if c['id'] not in ('NC-CE-SIGN', 'NC-CE-OMITTED', 'NC-CE-AS-RESTRAINT', 'NC-LUMPED-5050', 'NC-NO-FEC'):
                continue
            out = run_defect(case, basis, c['id'])
            w, wk = D(0), None
            for k, e in exp.items():
                r = abs(V.to_dec(out[k]) - e) / (CRIT * max(abs(e), rec_s[k]))
                if r > w:
                    w, wk = r, k
            pr = D(c['worst_ratio_to_criterion'])
            agree = (w > 1) == c['discriminates'] and abs(w - pr) <= D('1e-4') * pr
            rec[c['id']] = {'v3': (w > 1, format(w, '.5e'), wk), 'package': (c['discriminates'], c['worst_ratio_to_criterion'], c['worst_key']),
                            'ratio_agrees_1e-4': agree}
            if not agree:
                mism.append((cid, c['id'], rec[c['id']]))
        listed[cid] = rec
    for case in ref['cases']:
        cid = case['id']
        if 'cancellation' in case or '-COMB-' in cid:
            continue
        basis = 'int' if case['basis'] == 'intended' else 'rep'
        scales = {k: D(v['scale']) for k, v in case['classes'].items()}
        rec = {}
        for c in case['negative_controls']:
            out = run_defect(case, basis, c['id'])
            w, wk, nf = evaluate(case, out, scales)
            disc = w > 1
            pr = D(c['worst_ratio_to_criterion'])
            agree = disc == c['discriminates'] and (abs(w - pr) <= D('1e-4') * pr or (w > D('1e14') and pr > D('1e14')))
            rec[c['id']] = {'v3': (disc, format(w, '.5e'), wk, nf), 'package': (c['discriminates'], c['worst_ratio_to_criterion'],
                                                                              c['worst_key'], c['values_failing']),
                            'ratio_agrees_1e-4': agree}
            if not agree:
                mism.append((cid, c['id'], rec[c['id']]))
        listed[cid] = rec
    constructed = {}
    for case in ref['cases']:
        cid = case['id']
        fam = cid.split('-')[2]
        basis = 'int' if case['basis'] == 'intended' else 'rep'
        scales = {k: D(v['scale']) for k, v in case['classes'].items()}
        for name in CONSTRUCTED.get(fam, []):
            out = run_defect(case, basis, name)
            if out is None:
                continue
            w, wk, nf = evaluate(case, out, scales)
            constructed.setdefault(fam, {}).setdefault(name, {})[cid] = (format(w, '.4e'), wk, nf)
    comb = comb_controls(ref)
    json.dump({'listed_controls': listed, 'mismatches': mism, 'constructed': constructed, 'comb': comb},
              open(sys.argv[2], 'w'), indent=1, default=str)
    n = sum(len(v) for v in listed.values())
    print('listed controls rebuilt here (non-cancellation families, plus CE/LUMPED/NO-FEC controls of the cancellation cases):', n,
          ' mismatches (flag or ratio > 1e-4 rel):', len(mism))
    for m in mism:
        print('  MISMATCH', m)
    for fam, d in constructed.items():
        for name, cs in d.items():
            caught = [c for c, v in cs.items() if D(v[0]) > 1]
            print('%-6s %-28s caught in %d/%d: %s' % (fam, name, len(caught), len(cs),
                  ', '.join('%s %s' % (c[9:], cs[c][0]) for c in cs)))
    for cid, d in comb.items():
        print(cid, d)


if __name__ == '__main__':
    main()
