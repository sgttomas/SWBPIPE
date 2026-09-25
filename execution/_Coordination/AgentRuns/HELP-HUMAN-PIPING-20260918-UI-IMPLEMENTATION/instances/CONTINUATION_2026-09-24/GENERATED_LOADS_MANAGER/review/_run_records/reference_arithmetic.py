"""Independent rational/Decimal reference; imports no product code."""
from decimal import Decimal, localcontext
from fractions import Fraction as F
from pathlib import Path
import json, math, hashlib

CASES = [
    ('ordinary', .1, .01, 1000., 0., 0., 0.),
    ('thin_1e9', 1., 1e-9, 1., 0., 0., 0.),
    ('thin_1e12', 1., 1e-12, 1., 0., 0., 0.),
    ('sub_ulp_wall', 1., 2.**-55, 1., 0., 0., 0.),
    ('thin_insulation', 1., .01, 1e-30, 0., 1e-12, 1e12),
    ('sub_ulp_insulation', 1., .01, 1e-30, 0., 2.**-55, 1e15),
    ('small_bore_contents', 1., .5-2.**-30, 1e-30, 1e12, 0., 0.),
]

def atan(x):
    total = term = x
    x2 = x*x
    k = 1
    while True:
        term *= -x2
        update = term / (2*k+1)
        newer = total+update
        if newer == total:
            return total
        total = newer
        k += 1

def decimal(q):
    return Decimal(q.numerator)/Decimal(q.denominator)

def ratio(q):
    return {'numerator': str(q.numerator), 'denominator': str(q.denominator)}

with localcontext() as ctx:
    ctx.prec = 100
    pi = 16*atan(Decimal(1)/5)-4*atan(Decimal(1)/239)
    rows = []
    for name, od, wall, rho, contents, ins, insrho in CASES:
        D,t,r,c,d,s = map(F, (od,wall,rho,contents,ins,insrho))
        # Source radii and exact differences of disks establish the independent
        # identities before evaluating the proposed factored expressions.
        R = D/2
        ri = R-t
        ro_ins = R+d
        aw = R*R-ri*ri
        ab = ri*ri
        ai = ro_ins*ro_ins-R*R
        identities = [aw == t*(D-t), ab == (D/2-t)**2, ai == d*(D+d)]
        components = {'wall':aw*r, 'contents':ab*c, 'insulation':ai*s}
        coeff = sum(components.values(),F(0))
        expected = decimal(coeff)*pi
        bore_f = od-2*wall
        metal_f = (math.pi/4)*(od*od-bore_f*bore_f)*rho
        contents_f = (math.pi/4)*(bore_f*bore_f)*contents
        insod_f = od+2*ins
        ins_f = (math.pi/4)*(insod_f*insod_f-od*od)*insrho
        legacy = metal_f+contents_f+ins_f
        rows.append({
            'name':name,
            'input_f64':dict(zip(('outside_diameter_m','effective_wall_m','material_density_kg_m3','contents_density_kg_m3','insulation_thickness_m','insulation_density_kg_m3'),(od,wall,rho,contents,ins,insrho))),
            'exact_represented_inputs':[ratio(q) for q in (D,t,r,c,d,s)],
            'exact_identity_equalities':identities,
            'areas_over_pi_m2':{'wall':ratio(aw),'bore':ratio(ab),'insulation':ratio(ai)},
            'component_mass_over_pi_kg_m':{k:ratio(v) for k,v in components.items()},
            'total_mass_over_pi_kg_m':ratio(coeff),
            'expected_mass_kg_m_decimal':str(expected),
            'expected_q_N_m_at_minus_one_m_s2_decimal':str(-expected),
            'legacy_python_same_expression_mass_kg_m':legacy,
            'legacy_python_relative_error':str(abs(Decimal(legacy)-expected)/expected),
            'legacy_python_component_mass_kg_m':{'wall':metal_f,'contents':contents_f,'insulation':ins_f},
        })
    out = {'claim':'Independent exact rational algebra on represented input floats; 100-digit Decimal pi computed by Machin identity. Python legacy expression is illustrative arithmetic, not Rust production execution. No tolerance or acceptance criterion is established here.',
           'pi_decimal':str(pi),'cases':rows}
    dest = Path(__file__).with_name('SEVEN_MASS_REFERENCES.json')
    dest.write_text(json.dumps(out,indent=2)+'\n')
    for row in rows:
        print(row['name'], 'expected_mass='+row['expected_mass_kg_m_decimal'][:30], 'old_relative_error='+row['legacy_python_relative_error'][:24])
    print('output_sha256='+hashlib.sha256(dest.read_bytes()).hexdigest())
