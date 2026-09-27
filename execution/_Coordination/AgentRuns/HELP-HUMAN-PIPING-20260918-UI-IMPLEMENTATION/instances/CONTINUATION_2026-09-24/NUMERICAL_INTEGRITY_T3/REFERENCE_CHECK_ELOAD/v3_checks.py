"""V3 step 6: formulation checks.

Usage: python3 v3_checks.py <references_eload.json> <out v3_checks.json>
* Q3 and Q9 reconstructed from the case data; orthogonality and det +1 checked exactly; rotational
  invariance checked by solving V3's own unrotated models and comparing invariants.
* Hand-calc reproductions with V3's solver: tp_phys_006 (partial span), tp_phys_008 (thermal + pressure,
  fixed-fixed), constant_effort_support_applied_load, fixed_fixed_thermal_axial.
* NC-LOST-SOFT bound at r = 1e-6.
* Section constants.
"""
import copy
import json
import math
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as Fr
import v3_core as V

getcontext().prec = 60
D = Decimal


def det3(c1, c2, c3):
    return V.vdot(c1, V.vcross(c2, c3))


class Sec:
    def __init__(self, E, G, A, I, J):
        self.E, self.G, self.A, self.I, self.J = E, G, A, I, J
        self.EA, self.EI, self.GJ = E * A, E * I, G * J


def custom(nodes, members, restraints, eloads=(), nodal=(), efforts=(), eigen=None, thrust=None):
    pb = V.Problem()
    pb.case_id = 'custom'
    pb.basis = 'int'
    pb.nodes = nodes
    pb.members = []
    for name, i, j, sec in members:
        d = V.vsub(nodes[j], nodes[i])
        L = V.fsqrt_exact(V.vdot(d, d))
        pb.members.append(dict(name=name, i=i, j=j, L=L, e=V.vscl(1 / L, d), sec=sec, yref=(Fr(0), Fr(0), Fr(1))))
    pb.mem = {m['name']: m for m in pb.members}
    pb.restraints = restraints
    pb.springs = []
    pb.nodal = list(nodal)
    pb.efforts = list(efforts)
    pb.eloads = [dict(member=m, q=q, a=a, b=b, kind='authored', key='x', frame='global', comp=q) for m, q, a, b in eloads]
    pb.eigen = eigen or {}
    pb.thrust = thrust or {}
    pb.gen = {}
    return pb


FIX = {d: Fr(0) for d in V.DOFS}


def main():
    ref = json.load(open(sys.argv[1]))
    C = {c['id']: c for c in ref['cases']}
    out = {}
    # ---- Q9 from CONT2-AX -> CONT2-Q9 (node N1 (9,0,0) -> (1,8,-4); load (0,-2700,0) -> (-2400,-300,-1200))
    q1 = (Fr(1, 9), Fr(8, 9), Fr(-4, 9))
    q2 = tuple(Fr(x) / 2700 for x in ('2400', '300', '1200'))
    q3 = V.vcross(q1, q2)
    Q9 = (q1, q2, q3)   # columns
    orth = all(V.vdot(Q9[a], Q9[b]) == (1 if a == b else 0) for a in range(3) for b in range(3))
    rot = lambda Q, v: V.vadd(V.vadd(V.vscl(v[0], Q[0]), V.vscl(v[1], Q[1])), V.vscl(v[2], Q[2]))
    m2 = rot(Q9, (Fr(0), Fr(-1800), Fr(900)))
    out['Q9'] = {'columns': [[str(x) for x in c] for c in Q9], 'orthonormal': orth, 'det': str(det3(*Q9)),
                 'maps_M2_load': [str(x) for x in m2]}
    # invariance: every published scalar invariant and every nodal vector norm equal between AX and Q9 cases
    oa = V.run(C['RF-ELOAD-UDL-CONT2-AX'])
    oq = V.run(C['RF-ELOAD-UDL-CONT2-Q9'])
    inv_ok = all(oa[2][k][1] == oq[2][k][1] for k in oa[2] if k.split('.')[0] in ('N', 'T', 'Mb', 'tw', 'ext'))
    vec_ok = all(rot(Q9, oa[1].u[n][:3]) == oq[1].u[n][:3] and rot(Q9, oa[1].u[n][3:]) == oq[1].u[n][3:] for n in oa[0].nodes)
    out['Q9']['invariants_equal'] = inv_ok
    out['Q9']['nodal_vectors_rotate_exactly'] = vec_ok
    # ---- Q3 (LOC case): e = (1,2,-2)/3, y = (2,1,2)/3, z = e x y
    e = (Fr(1, 3), Fr(2, 3), Fr(-2, 3))
    y = (Fr(2, 3), Fr(1, 3), Fr(2, 3))
    Q3 = (e, y, V.vcross(e, y))
    out['Q3'] = {'columns': [[str(x) for x in c] for c in Q3], 'orthonormal':
                 all(V.vdot(Q3[a], Q3[b]) == (1 if a == b else 0) for a in range(3) for b in range(3)), 'det': str(det3(*Q3))}
    # V3's own X-axis model of the Q3 case: L = 3, global load (300,-2000,1200)
    cq = C['RF-ELOAD-UDL-CANT-FULL-LOC-Q3']
    sec = V.Section(Fr('200e9'), Fr('80e9'), Fr('0.2'), Fr('0.01'), Fr(0))
    pbx = custom({'N0': (Fr(0),) * 3, 'N1': (Fr(3), Fr(0), Fr(0))}, [('M1', 'N0', 'N1', sec)], {'N0': dict(FIX)},
                 eloads=[('M1', (Fr(300), Fr(-2000), Fr(1200)), Fr(0), Fr(1))])
    solx = V.solve_problem(pbx)
    ox = V.publish(pbx, solx)
    pq, sq, oq3 = V.run(cq)
    out['Q3']['invariants_equal'] = all(ox[k][1] == oq3[k][1] for k in ox if k.split('.')[0] in ('N', 'T', 'Mb', 'tw', 'ext'))
    out['Q3']['nodal_vectors_rotate_exactly'] = all(rot(Q3, solx.u[n][:3]) == sq.u[n][:3] and rot(Q3, solx.u[n][3:]) == sq.u[n][3:]
                                                  for n in pbx.nodes)
    # ---- hand calcs
    hc = {}
    s6 = Sec(Fr(1000), Fr(400), Fr(3), Fr(2), Fr(1))   # Iz = 2 governs the Y load
    p6 = custom({'0': (Fr(0),) * 3, '1': (Fr(4), Fr(0), Fr(0))}, [('M', '0', '1', s6)], {'0': dict(FIX)},
                eloads=[('M', (Fr(0), Fr(-2), Fr(0)), Fr(1, 4), Fr(3, 4))])
    so6 = V.solve_problem(p6)
    Pi = so6.members['M']['Pend']
    hc['tp_phys_006'] = {'u_y': str(so6.u['1'][1]), 'theta_z': str(so6.u['1'][5]), 'expected': ['-7/500', '-13/3000'],
                         'ok': so6.u['1'][1] == Fr(-7, 500) and so6.u['1'][5] == Fr(-13, 3000),
                         'Vy_i (support on member)': str(Pi[1]), 'Mz_i': str(Pi[5])}
    s8 = Sec(Fr(1000), Fr(400), Fr(4), Fr(2), Fr(1))
    p8 = custom({'0': (Fr(0),) * 3, '1': (Fr(6), Fr(0), Fr(0))}, [('M', '0', '1', s8)], {'0': dict(FIX), '1': dict(FIX)},
                eigen={'M': Fr('0.00001') * 75}, thrust={'M': Fr(90) * Fr('0.1')})
    so8 = V.solve_problem(p8)
    o8 = V.publish(p8, so8)
    hc['tp_phys_008'] = {'N (tension +)': str(o8['N.M.mid'][1]), 'expected': '-12 (compression 12 N; local end forces +12 at i, -12 at j)',
                         'ok': o8['N.M.mid'][1] == -12, 'local end force i': str(so8.members['M']['Pend'][0]),
                         'local end force j': str(so8.members['M']['Pend'][6])}
    sc = Sec(Fr(1200), Fr(400), Fr(1), Fr(4), Fr(8))
    pc = custom({'0': (Fr(0),) * 3, '1': (Fr(10), Fr(0), Fr(0))}, [('M', '0', '1', sc)], {'0': dict(FIX)},
                nodal=[('1', 'UY', Fr(-6), 'W')], efforts=[('1', 'UY', Fr(9))])
    soc = V.solve_problem(pc)
    hc['constant_effort'] = {'tip_uy': str(soc.u['1'][1]), 'R_y': str(soc.R[('0', 'UY')]), 'M_z': str(soc.R[('0', 'RZ')]),
                             'ok': soc.u['1'][1] == Fr(3) * 1000 / (3 * 1200 * 4) and soc.R[('0', 'UY')] == -3 and soc.R[('0', 'RZ')] == -30}
    sf = Sec(Fr(2000), Fr(800), Fr(3), Fr(1), Fr(2))
    pf = custom({'0': (Fr(0),) * 3, '1': (Fr(5), Fr(0), Fr(0))}, [('M', '0', '1', sf)], {'0': dict(FIX), '1': dict(FIX)},
                eigen={'M': Fr('0.000012') * 75})
    of = V.publish(pf, V.solve_problem(pf))
    hc['fixed_fixed_thermal_axial'] = {'N': str(of['N.M.i'][1]), 'ok': of['N.M.i'][1] == Fr('-5.4')}
    # tp_pmm_p3_occloadgen: m' and intensities (hand-calc inputs), V3's mass formula
    so = V.Section(Fr(1), Fr(1), Fr('0.2'), Fr('0.01'), Fr('0.00125'))
    mp = V.mass_per_length(so, Fr(7850), Fr(800), Fr(120), Fr('0.025')) * V.PI
    mpd = float(mp)
    hc['tp_pmm_p3_occloadgen'] = {'m_prime': repr(mpd), 'hand_calc': '64.31699191139529',
                                  'w_X': repr(float(mp * Fr('0.3') * Fr('9.80665'))), 'hand_calc_w_X': '189.22026861836537',
                                  'w_wind': str(Fr(480) * Fr('0.7') * (Fr('0.2') + 2 * Fr('0.025'))), 'hand_calc_w_wind': '84',
                                  'rel_diff_m_prime': format(abs(D(mpd) - D('64.31699191139529')) / D('64.31699191139529'), '.2e')}
    out['hand_calcs'] = hc
    # ---- NC-LOST-SOFT bound at r = 1e-6
    c = C['RF-ELOAD-TH-SPRING-LEG-r1e-06']
    pb = V.build(c)
    m = pb.members[0]
    ka = float(m['sec'].EA / m['L'])
    k = pb.springs[0][2]
    out['lost_soft_r1e-6'] = {'fl(EA/L)': repr(ka), 'ulp': repr(math.ulp(ka)), 'k': str(k),
                              'addition_exact': Fr(ka + float(k)) - Fr(ka) == k,
                              'worst_case_relative_error_of_k': format(D(math.ulp(ka)) / 2 / D(k.numerator) * D(k.denominator), '.3e')}
    # ---- section constants
    secs = {}
    for s in (('N', '0.2', '0.01', '0'), ('G', '0.2', '0.01', '0.00125'), ('B', '0.25', '0.012', '0')):
        S = V.Section(Fr('200e9'), Fr('80e9'), Fr(s[1]), Fr(s[2]), Fr(s[3]))
        secs[s[0]] = {'t_eff': str(S.t), 'ID': str(S.ID), 'EA/pi': str(S.EA / V.PI), 'EI/pi': str(S.EI / V.PI), 'GJ/pi': str(S.GJ / V.PI),
                      'J=2I': S.J == 2 * S.I}
    out['sections'] = secs
    json.dump(out, open(sys.argv[2], 'w'), indent=1)
    print(json.dumps(out, indent=1))


if __name__ == '__main__':
    main()
