"""Original static-annulus reference arithmetic; no production imports.

Run from repository root:
python3 projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/SHEAR_REFERENCE/_run_records/shear_refutation/reference_arithmetic.py
"""
from fractions import Fraction as F
from math import pi
import platform
import sys


def show(label, value):
    print(f"{label}: exact={value}; decimal={float(value):.16g}")


def mm(a, b):
    return [[sum(x*y for x, y in zip(row, col)) for col in zip(*b)] for row in a]


def mv(a, b):
    return [sum(x*y for x, y in zip(row, b)) for row in a]


def solve2(a, b):
    det = a[0][0]*a[1][1]-a[0][1]*a[1][0]
    return [(b[0]*a[1][1]-b[1]*a[0][1])/det,
            (a[0][0]*b[1]-a[1][0]*b[0])/det]


def stiffness(ei, ks, length):
    phi = 12*ei/(ks*length**2)
    l = length
    return [[ei/(l**3*(1+phi))*v for v in row] for row in
            [[12, 6*l, -12, 6*l],
             [6*l, (4+phi)*l*l, -6*l, (2-phi)*l*l],
             [-12, -6*l, 12, -6*l],
             [6*l, (2-phi)*l*l, -6*l, (4+phi)*l*l]]]


def distal_half(q, l, phi):
    return [q*l*(3+4*phi)/(32*(1+phi)),
            q*l*l*(5+8*phi)/(192*(1+phi)),
            q*l*(13+12*phi)/(32*(1+phi)),
            -q*l*l*(11+8*phi)/(192*(1+phi))]


def shear_factors(a, b, nu):
    m = b/a
    s, p = a*a+b*b, a*a*b*b
    kc = 6*(1+nu)*(1+m*m)**2 / ((7+6*nu)*(1+m*m)**2+(20+12*nu)*m*m)
    ke = 6*(1+nu)**2*(1+m*m)**2 / ((7+14*nu+8*nu*nu)*(1+m*m)**2+4*m*m*(5+10*nu+4*nu*nu))
    # Analytic radial integral of angularly averaged SV shear stress squared.
    # Area and second moment below omit their common factor pi.
    area, inertia = a*a-b*b, (a**4-b**4)/4
    c, e = (3+2*nu)/4, (1+2*nu)/4
    radial = c*c*s*s*(a*a-b*b)/2-c*s*(a**4-b**4)/4
    radial += (F(1,4)+e*e)*(a**6-b**6)/6
    radial += c*c*p*(a*a-b*b)/2+c*p*e*(a*a-b*b)
    # int tau^2/V^2 = integral_without_pi / pi.
    integral_without_pi = radial/(2*(1+nu)**2*inertia**2)
    assert ke == 1/(area*integral_without_pi)
    # Section mean-displacement integral J also omits pi.
    j = -c*(s*inertia+p*area/2)+(a**6-b**6)/24
    assert kc == -2*(1+nu)*inertia**2/(area*j)
    return kc, ke, integral_without_pi


print('Environment:', sys.version.replace('\n', ' '), platform.platform())
print('Arithmetic: Python standard-library Fraction; pi only for decimal SI conversion.')
for nu in [F(-1,2), F(0), F(3,10), F(49,100)]:
    for a, b in [(F(1), F(0)), (F(1), F(1,4)), (F(1), F(9,10)), (F(3,50), F(1,20))]:
        shear_factors(a, b, nu)
print('Exact SV energy and mean-displacement formula equalities: 16 rational cases pass.')

a, b, E, nu = F(1,10), F(9,100), F(200_000_000_000), F(3,10)
G = E/(2*(1+nu))
A_pi, I_pi = a*a-b*b, (a**4-b**4)/4
kc, ke, integral = shear_factors(a, b, nu)
EI_pi, Ks_pi = E*I_pi, ke*G*A_pi
print('\nPhysical annulus: ro=0.1 m, ri=0.09 m, E=200e9 Pa, nu=0.3.')
for name, val in [('A/pi (m2)', A_pi), ('I/pi (m4)', I_pi), ('G (Pa)', G), ('kappa_C', kc), ('kappa_E', ke), ('EI/pi (N m2)', EI_pi), ('Ks/pi (N)', Ks_pi), ('kappa_E/kappa_C', ke/kc), ('shear compliance C relative to E', ke/kc-1)]:
    show(name, val)
print(f'EI={float(EI_pi)*pi:.16g} N m2; Ks={float(Ks_pi)*pi:.16g} N')
print('Exact SI fixture: tip load F=100*pi N; all response fractions below are exact.')
for l in [F(1), F(2), F(10)]:
    f_pi = F(100)
    vb, vs = f_pi*l**3/(3*EI_pi), f_pi*l/Ks_pi
    theta, gamma = f_pi*l*l/(2*EI_pi), f_pi/Ks_pi
    phi = 12*EI_pi/(Ks_pi*l*l)
    print(f'\nLength L={l} m')
    for name, val in [('phi',phi), ('v_b (m)',vb), ('v_s (m)',vs), ('v_tip (m)',vb+vs), ('theta_tip (rad)',theta), ('slope_tip (rad)',theta+gamma), ('v_s/v_b',vs/vb)]:
        show(name,val)
    print(f'Realistic F=100 N: v_b={float(vb)/pi:.16g} m; v_s={float(vs)/pi:.16g} m; v_tip={float(vb+vs)/pi:.16g} m; theta={float(theta)/pi:.16g} rad; slope={float(theta+gamma)/pi:.16g} rad')
    ksolid = 6*(1+nu)/(7+6*nu)
    wrong_v = vb+f_pi*l/(ksolid*G*A_pi)
    show('Wrong solid-Cowper deflection/exact energy-annulus deflection',wrong_v/(vb+vs))
    show('Full-integration linear-element deflection/exact Timo deflection',phi/(1+phi))

print('\nAbstract exact model fixture: EI=1 N m2, Ks=12 N, L=1 m, phi=1.')
ei, ks, l, q = F(1), F(12), F(1), F(1)
k = stiffness(ei, ks, l)
print('K rows:', k)
assert mv(k,[1,0,1,0]) == [0]*4
assert mv(k,[0,1,l,1]) == [0]*4
assert k == list(map(list,zip(*k)))
print('Symmetry and both rigid modes: exact pass.')
kff = [row[2:] for row in k[2:]]
fhalf = distal_half(q,l,F(1))
feb = distal_half(q,l,F(0))
for name, vec in [('distal-half exact feq',fhalf), ('distal-half stale EB feq',feb)]:
    print(name, vec)
    assert vec[0]+vec[2] == q*l/2
    assert vec[1]+l*vec[2]+vec[3] == 3*q*l*l/8
    print(name+' -> cantilever [v,theta]',solve2(kff,vec[2:]))
assert solve2(kff,fhalf[2:]) == [F(53,384),F(7,48)]
assert solve2(kff,[1,0]) == [F(5,12),F(1,2)]
uniform = [q*l/2,q*l*l/12,q*l/2,-q*l*l/12]
assert solve2(kff,uniform[2:]) == [F(1,6),F(1,6)]
print('Point-load, full uniform-load and partial-load exact cantilever references: pass.')
print('Tip F=1: theta=1/2, slope=7/12; tip C=1: v=1/2,theta=1; fixed-guided v=1: force=6.')

# Circular transverse-coordinate covariance, using beta=(theta_z,-theta_y)
# so v and beta transform alike in the transverse plane.
k8 = [[k[i//2][j//2] if i%2 == j%2 else F(0) for j in range(8)] for i in range(8)]
r = [[F(3,5),F(-4,5)],[F(4,5),F(3,5)]]
t = [[r[i%2][j%2] if i//2 == j//2 else F(0) for j in range(8)] for i in range(8)]
assert mm(list(map(list,zip(*t))),mm(k8,t)) == k8
print('Circular transverse basis rotation 3-4-5: exact covariance pass.')
print('No-locking normalized point-load compliance = 1+phi/4 (relative to EB).')
for phi in [F(1),F(1,100),F(1,10**6),F(1,10**12)]:
    show('phi',phi)
    show('Exact Timo/EB',1+phi/4)
    show('Locked linear full-integration/Exact Timo',phi/(1+phi))
print('\nInterior spring independently checks indeterminate assembly redistribution.')
def spring_case(ei, inverse_ks, length, station, tip_force, spring_k):
    caL = station**2*(3*length-station)/(6*ei)+station*inverse_ks
    caa = station**3/(3*ei)+station*inverse_ks
    cLL = length**3/(3*ei)+length*inverse_ks
    ua = tip_force*caL/(1+spring_k*caa)
    reaction = -spring_k*ua
    ul = tip_force*cLL+reaction*caL
    theta = (tip_force*length**2+reaction*station**2)/(2*ei)
    return [reaction,ua,ul,theta,-(tip_force+reaction),-(tip_force*length+reaction*station)]

spring_timo = spring_case(F(1),F(1,12),F(1),F(1,2),F(1),F(12))
spring_eb = spring_case(F(1),F(0),F(1),F(1,2),F(1),F(12))
assert spring_timo == [F(-7,8),F(7,96),F(37,128),F(25,64),F(-1,8),F(-9,16)]
assert spring_eb == [F(-5,6),F(5,72),F(71,288),F(19,48),F(-1,6),F(-7,12)]
print('Timoshenko [R,u(a),u(L),theta(L),root F,root M]:',spring_timo)
print('EB [R,u(a),u(L),theta(L),root F,root M]:',spring_eb)
physical = spring_case(float(EI_pi)*pi,1/(float(Ks_pi)*pi),2.,1.,100.,1e6)
print('Physical companion L=2,a=1,F=100,k=1e6 in SI, same field order:',physical)
print('\nAll assertions completed. This is reference arithmetic, not product verification.')
