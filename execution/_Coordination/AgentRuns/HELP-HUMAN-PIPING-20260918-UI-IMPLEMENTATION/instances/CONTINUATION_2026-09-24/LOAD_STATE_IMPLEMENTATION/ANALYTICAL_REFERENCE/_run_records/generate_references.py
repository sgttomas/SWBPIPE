"""Translate reviewed analytical controls. Standard library only; no product imports/runs."""
from pathlib import Path
from fractions import Fraction as F
from decimal import Decimal as D, localcontext
import datetime, hashlib, json, math, subprocess, sys

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / 'projects/chirality-piping').is_dir())
WORK = ROOT / 'projects/chirality-piping'
OUT = WORK / 'core/product_physics/tests/fixtures/load_reference_states'
OUT.mkdir(parents=True, exist_ok=True)
REV = '9e8a55daecdeb9669131fd3e53c0e0303ee550d6'
SOURCE = 'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES'
PI_TEXT = '3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679'
PI = D(PI_TEXT)
getcontext = __import__('decimal').getcontext
getcontext().prec = 85

def decimal(q):
    return D(q.numerator) / D(q.denominator)

def quantity(q, unit='1', pi_power=0):
    q = F(q)
    value = decimal(q) * PI ** pi_power
    kind = {0: 'rational', 1: 'rational_times_pi', -1: 'rational_over_pi'}[pi_power]
    exact = {'kind': kind, 'rational': str(q)}
    return {'unit': unit, 'exact': exact, 'decimal': str(value), 'value': float(value)}

def transcendental(value, expression, unit='1'):
    return {'unit': unit, 'exact': {'kind': 'symbolic', 'expression': expression},
            'decimal': str(value), 'value': float(value), 'evaluation': 'Decimal.exp, precision 85'}

def inputs(**values):
    return values

sources = {}
startup = json.loads((HERE.parents[1] / '_run_records/STARTUP.json').read_text())
known = {x['git_blob']: x['sha256'] for x in startup['design_sources']}
for name in ['DESIGN.md', 'INTERFACE.md', 'VERIFICATION.md', 'REFERENCES.md', 'HYDROSTATIC_CONTROL.md', 'RETURN.md']:
    identity = REV + ':' + SOURCE + '/' + name
    data = subprocess.check_output(['git', 'show', identity], cwd=ROOT)
    digest = hashlib.sha256(data).hexdigest()
    assert digest == known[identity], name
    sources[name] = {'git_blob': identity, 'sha256': digest}

od, t = F(1, 5), F(1, 100)
ro, ri = od / 2, od / 2 - t
As = t * (od - t)
Ai = ri ** 2
I = (ro ** 4 - ri ** 4) / 4
J = 2 * I
assert As == ro ** 2 - ri ** 2 == F(19, 10000)
assert I == F(3439, 400000000)
assert I == As * (ro ** 2 + ri ** 2) / 4
geometry = {
    'id': 'annulus_od_0p20_wall_0p01',
    'authored': {'outside_diameter': quantity(od, 'm'), 'wall_thickness': quantity(t, 'm')},
    'derived': {'ro': quantity(ro, 'm'), 'ri': quantity(ri, 'm'),
                'As': quantity(As, 'm^2', 1), 'Ai': quantity(Ai, 'm^2', 1),
                'I': quantity(I, 'm^4', 1), 'J': quantity(J, 'm^4', 1),
                'Z': quantity(I / ro, 'm^3', 1)},
    'equations': ['As=pi*t*(OD-t)=pi*(ro^2-ri^2)', 'Ai=pi*ri^2',
                  'I=pi*(ro^4-ri^4)/4=As*(ro^2+ri^2)/4', 'J=2*I', 'Z=I/ro'],
    'purpose': 'Explicit authorable pipe companion; no inferred generic-area input and no operational material data.',
}

cases = {}
def case(name, control, equations, variants, wrong, limits=None):
    cases[name] = {'source': {'document': 'VERIFICATION.md', 'control': control},
                   'equations': equations, 'variants': variants, 'wrong_result_discriminators': wrong,
                   'limits': limits or ['Linear elastic, small displacement; synthetic inputs only.']}

areas = [('generic_reviewed', F(1, 1000), 0), ('annular_companion', As, 1)]
E1, E2, L, g = F(200000000000), F(100000000000), F(1), F(1, 10000)
translation, fixed, shared, serial = {}, {}, {}, {}
for name, area, power in areas:
    a_input = {'area': quantity(area, 'm^2', power)} if power == 0 else {'geometry_ref': geometry['id']}
    k1, k2 = E1 * area / L, E2 * area / L
    middle = k1 * g / (k1 + k2)
    n = k1 * (middle - g)
    assert middle == F(1, 15000)
    assert n == k2 * (0 - middle)
    translation[name] = {'inputs': {**a_input, 'L1': quantity(L, 'm'), 'L2': quantity(L, 'm'),
        'E1': quantity(E1, 'Pa'), 'E2': quantity(E2, 'Pa'), 'root_UX': quantity(g, 'm'), 'far_UX': quantity(0, 'm')},
        'expected': {'middle_UX': quantity(middle, 'm'), 'member1_N': quantity(n, 'N', power),
                     'member2_N': quantity(n, 'N', power), 'root_Fx': quantity(-n, 'N', power), 'far_Fx': quantity(n, 'N', power),
                     'free_rhs_after_prescribed_coupling': quantity(k1 * g, 'N', power)}}
    fixed[name] = {'inputs': {**a_input, 'L': quantity(L, 'm'), 'E': quantity(E1, 'Pa'),
        'root_UX': quantity(g, 'm'), 'far_UX': quantity(0, 'm'), 'free_dofs': []},
        'expected': {'N': quantity(-k1 * g, 'N', power), 'root_Fx': quantity(k1 * g, 'N', power), 'far_Fx': quantity(-k1 * g, 'N', power)}}
    eps = F(1, 1000)
    u = k2 * eps * L / (k1 + k2)
    cold_n, hot_n = k1 * u, k2 * (u - eps * L)
    assert u == F(1, 3000) and cold_n + hot_n == 0
    shared[name] = {'inputs': {**a_input, 'material_id': 'material:user-shared', 'member_length': quantity(L, 'm'),
        'cold': {'E': quantity(E1, 'Pa'), 'thermal_strain': quantity(0)},
        'hot': {'E': quantity(E2, 'Pa'), 'thermal_strain': quantity(eps)},
        'topology': 'Two coincident parallel axial bars, root fixed, common unloaded free tip; transverse modes suppressed.'},
        'expected': {'tip_UX': quantity(u, 'm'), 'cold_N': quantity(cold_n, 'N', power), 'hot_N': quantity(hot_n, 'N', power),
                     'total_root_Fx': quantity(0, 'N')}}
    n_serial = -(eps * L) / (L / (E1 * area) + L / (E2 * area))
    umid = n_serial * L / (E1 * area)
    assert umid == -F(1, 3000) and n_serial == hot_n
    serial[name] = {'inputs': {**a_input, 'material_id': 'material:user-shared', 'L1': quantity(L, 'm'), 'L2': quantity(L, 'm'),
        'E1': quantity(E1, 'Pa'), 'E2': quantity(E2, 'Pa'), 'epsilon1': quantity(0), 'epsilon2': quantity(eps),
        'root_UX': quantity(0, 'm'), 'far_UX': quantity(0, 'm'),
        'topology': 'Serial nodes at x=0,1,2 m; axial middle DOF free; all transverse modes suppressed.'},
        'expected': {'middle_UX': quantity(umid, 'm'), 'member1_N': quantity(n_serial, 'N', power),
                     'member2_N': quantity(n_serial, 'N', power), 'root_Fx': quantity(-n_serial, 'N', power), 'far_Fx': quantity(n_serial, 'N', power)}}

case('prescribed_translation_two_bar', 1,
     ['k_i=E_i*As/L_i', '(k1+k2)*u_middle=k1*g_root+k2*g_far', 'N1=k1*(u_middle-g_root)', 'N2=k2*(g_far-u_middle)', 'R_root=-N1; R_far=N2'],
     translation, {'omitted_Kfc_gc': {'middle_UX': quantity(0, 'm'), 'reason': 'Fails the original free equation by -k1*g_root.'}})
case('prescribed_translation_all_fixed', 1, ['N=E*As*(g_far-g_root)/L', 'R_root=-N; R_far=N'], fixed,
     {'zero_size_free_problem_discarded': 'All prescribed DOFs still have nonzero actions; zero free DOFs do not imply a zero physical result.'})

beam_fixed, beam_free = {}, {}
theta, beam_l = F(1, 1000), F(2)
for name, ei, power in [('generic_reviewed', F(2000), 0), ('annular_companion', E1 * I, 1)]:
    params = {'L': quantity(beam_l, 'm'), 'root_RZ': quantity(theta, 'rad'), 'root_UY': quantity(0, 'm')}
    if power:
        params.update({'geometry_ref': geometry['id'], 'E': quantity(E1, 'Pa'), 'EI': quantity(ei, 'N*m^2', power)})
    else:
        params['EI'] = quantity(ei, 'N*m^2')
    fi, mi, fj, mj = 6*ei*theta/beam_l**2, 4*ei*theta/beam_l, -6*ei*theta/beam_l**2, 2*ei*theta/beam_l
    assert fi + fj == 0 and mi + mj + beam_l * fj == 0
    beam_fixed[name] = {'inputs': {**params, 'far_UY': quantity(0, 'm'), 'far_RZ': quantity(0, 'rad'), 'all_other_DOFs': 'prescribed zero'},
        'expected': {'root_Fy': quantity(fi, 'N', power), 'root_Mz': quantity(mi, 'N*m', power),
                     'far_Fy': quantity(fj, 'N', power), 'far_Mz': quantity(mj, 'N*m', power)}}
    beam_free[name] = {'inputs': {**params, 'far_end': 'free, no applied force/moment'},
        'expected': {'tip_UY': quantity(beam_l*theta, 'm'), 'tip_RZ': quantity(theta, 'rad'),
                     'root_Fy': quantity(0, 'N'), 'root_Mz': quantity(0, 'N*m'), 'wall_N': quantity(0, 'N')}}
case('prescribed_rotation_all_fixed', 2,
     ['v(x)=L*theta_i*(s-2*s^2+s^3), s=x/L', 'support-on-member=[6*EI*theta/L^2,4*EI*theta/L,-6*EI*theta/L^2,2*EI*theta/L]'],
     beam_fixed, {'translation_substitution_or_length_normalized_angle': 'Must reproduce all four signed reactions and force/moment balance.'},
     ['Euler-Bernoulli bending; root/end RZ values are radians, not length-normalized rotations.', 'Small rotations; no finite rigid-body rotation claim.'])
case('prescribed_rotation_free_tip', 2, ['v(x)=theta*x', 'v_prime(x)=theta', 'v_second(x)=0'], beam_free,
     {'missing_rotation_coupling': {'wrong_tip_UY': quantity(0, 'm')}, 'length_normalized_angle': {'wrong_tip_UY': quantity(theta, 'm'), 'wrong_tip_RZ': quantity(theta/beam_l, 'rad')}},
     ['Stress-free rigid rotation only to first order; no finite-rotation mechanics qualification.'])
case('shared_material_parallel', 3,
     ['N_cold=E_cold*As*(u/L-eps_cold)', 'N_hot=E_hot*As*(u/L-eps_hot)', 'N_cold+N_hot=0'], shared,
     {'material_ID_only_E_override': {'wrong_tip_UX': quantity(F(1,2000), 'm')},
      'mixed_basis_result_sum': 'A sum of separately solved states is not a new resolved physical equilibrium.'},
     ['Coincident parallel members are an analytical topology only; public/native admissibility must be established separately.',
      'Original reference specifies E and strain, not temperatures/nu. Do not infer unentered material or temperature data.'])
serial['annular_companion']['inputs'].update({'nu1': quantity(F(3,10)), 'nu2': quantity(F(1,4)),
    'material_point_cold': 'point:user-cold', 'material_point_hot': 'point:user-hot'})
serial['annular_companion']['expected'].update({'G1': quantity(E1/(2*(1+F(3,10))), 'Pa'), 'G2': quantity(E2/(2*(1+F(1,4))), 'Pa')})
case('shared_material_serial_companion', 3,
     ['N=-(eps1*L1+eps2*L2)/(L1/(E1*As)+L2/(E2*As))', 'u_middle=N*L1/(E1*As)+eps1*L1', 'G_i=E_i/(2*(1+nu_i))'], serial,
     {'material_ID_only_E_override': {'wrong_middle_UX': quantity(-F(1,2000), 'm')}},
     ['Derived companion to reviewed control 3, not the original parallel topology.',
      'Added annular nu values and point IDs are explicit invented inputs; direct strain is not a temperature inferred from a property point.'])

install_dilation, hot_dilation = F(12,1000000)*30, F(16,1000000)*130
datum_eps = (1+hot_dilation)/(1+install_dilation)-1
assert datum_eps == F(43,25009)
thermal = {'inputs': {'coefficient_definition': 'engineering_secant', 'datum_temperature': quantity(20, 'degC'),
    'installation_temperature': quantity(50, 'degC'), 'operating_temperature': quantity(150, 'degC'),
    'alpha_install': quantity(F(12,1000000),'1/K'), 'alpha_operating': quantity(F(16,1000000),'1/K'),
    'interpolation': 'linear_coefficient',
    'table_points': [{'temperature': quantity(20,'degC'), 'alpha': quantity(F(11,1000000),'1/K')},
                     {'temperature': quantity(50,'degC'), 'alpha': quantity(F(12,1000000),'1/K')},
                     {'temperature': quantity(150,'degC'), 'alpha': quantity(F(16,1000000),'1/K')}]},
    'expected': {'dilation_install': quantity(install_dilation), 'dilation_operating': quantity(hot_dilation),
                 'thermal_stretch': quantity(1+datum_eps), 'thermal_strain': quantity(datum_eps),
                 'datum_temperature_K': quantity(F(5863,20),'K'), 'installation_temperature_K': quantity(F(6463,20),'K'),
                 'operating_temperature_K': quantity(F(8463,20),'K')}}
thermal_comp = {'inputs': {**thermal['inputs'], 'geometry_ref': geometry['id'], 'L': quantity(1,'m'), 'E': quantity(E1,'Pa')},
    'expected': {**thermal['expected'], 'free_tip_UX': quantity(datum_eps,'m'), 'free_wall_N': quantity(0,'N'),
                 'fixed_wall_N': quantity(-E1*As*datum_eps,'N',1), 'fixed_root_Fx': quantity(E1*As*datum_eps,'N',1),
                 'fixed_far_Fx': quantity(-E1*As*datum_eps,'N',1)}}
case('thermal_datum_ratio', 4, ['lambda(T)=1+alpha_sec(T)*(T-T_m)', 'eps_th=lambda(T)/lambda(T_install)-1'],
     {'generic_reviewed': thermal, 'annular_companion': thermal_comp},
     {'alpha_hot_times_operating_minus_install': quantity(F(16,1000000)*100), 'subtract_datum_dilations': quantity(hot_dilation-install_dilation)},
     ['Added 20degC coefficient point is explicit invented coverage data; reviewed 50/150degC values and 43/25009 result are unchanged.',
      'Absolute temperature conversion includes +273.15; inverse-temperature coefficient conversion does not.'])

integral = F(3,2000)
first_half, second_half = F(1,1600), F(7,8000)
assert first_half + second_half == integral
with localcontext() as ctx:
    ctx.prec = 85
    exp_forward = decimal(integral).exp()
    exp_reverse = (-decimal(integral)).exp()
    exp_first, exp_second = decimal(first_half).exp(), decimal(second_half).exp()
    assert abs(exp_first * exp_second - exp_forward) < D('1e-83')
coefficient = {'inputs': {'datum_temperature': quantity(20,'degC'), 'installation_temperature': quantity(20,'degC'),
    'operating_temperature': quantity(120,'degC'), 'linear_coefficient_points': [
        {'temperature':quantity(20,'degC'),'coefficient':quantity(F(1,100000),'1/K')},
        {'temperature':quantity(120,'degC'),'coefficient':quantity(F(1,50000),'1/K')}]},
    'expected': {'integral': quantity(integral), 'first_half_integral': quantity(first_half), 'second_half_integral': quantity(second_half),
        'datum_length_strain_forward': quantity(integral), 'datum_length_strain_reverse': quantity(1/(1+integral)-1),
        'datum_length_strain_first_half': quantity(first_half),
        'datum_length_strain_second_half': quantity((1+integral)/(1+first_half)-1),
        'current_length_strain_forward': transcendental(exp_forward-1,'exp(3/2000)-1'),
        'current_length_strain_reverse': transcendental(exp_reverse-1,'exp(-3/2000)-1'),
        'current_length_strain_first_half': transcendental(exp_first-1,'exp(1/1600)-1'),
        'current_length_strain_second_half': transcendental(exp_second-1,'exp(7/8000)-1')}}
case('coefficient_definition', 5,
     ['integral=(alpha_start+alpha_end)*deltaT/2', 'lambda_datum=1+integral_from_datum',
      'lambda_current=exp(integral_from_datum)', 'eps(a->b)=lambda(b)/lambda(a)-1', '(1+eps_01)*(1+eps_12)=1+eps_02'],
     {'generic_reviewed': coefficient}, {'endpoint_alpha_times_interval': quantity(F(1,50000)*100),
     'reverse_by_negation': {'wrong_datum_reverse':quantity(-integral),'wrong_log_reverse':transcendental(1-exp_forward,'1-exp(3/2000)')}},
     ['Exact endpoint/bracket inclusion required; no extrapolation or fabricated table tails.', 'Same numeric alpha table denotes distinct data under the two definitions.'])
case('constant_alpha_interval', 4, ['eps_th=alpha*deltaT'],
     {'generic_reviewed': {'inputs': {'alpha':quantity(F(1,100000),'1/K'),'deltaT':quantity(80,'K'),
        'meaning':'engineering_interval','material_selection':'explicit applicable fixed user data; no inferred absolute temperature'},
      'expected': {'thermal_strain':quantity(F(1,1250)), 'thermal_stretch':quantity(F(1251,1250))}}},
     {'invented_ambient_temperature': 'No absolute temperature is required or derivable from this interval-only case.'})

fit = {}
fit_l, cut = F(10), -F(1,500)
fit_eps, hot_eps = cut/fit_l, F(1,1250)
total_hot = (1+fit_eps)*(1+hot_eps)-1
assert total_hot == F(3749,6250000)
for name, area, power in areas:
    a_input = {'area': quantity(area,'m^2',power)} if power == 0 else {'geometry_ref':geometry['id']}
    states = {}
    for state, e, eth in [('cold',E1,F(0)),('hot',F(150000000000),hot_eps),('return',E1,F(0))]:
        total = (1+fit_eps)*(1+eth)-1
        n = -e*area*total
        states[state] = {'E':quantity(e,'Pa'), 'thermal_strain':quantity(eth), 'fit_strain':quantity(fit_eps),
            'total_eigenstrain':quantity(total), 'fixed_wall_N':quantity(n,'N',power),
            'fixed_root_Fx':quantity(-n,'N',power), 'fixed_far_Fx':quantity(n,'N',power),
            'released_tip_UX':quantity(fit_l*total,'m'), 'released_wall_N':quantity(0,'N')}
    fit[name] = {'inputs':{**a_input,'L':quantity(fit_l,'m'),'signed_fit_length_change':quantity(cut,'m'),
        'installation_temperature':quantity(20,'degC'),'hot_temperature':quantity(100,'degC'),
        'coefficient_datum_temperature':quantity(20,'degC'),'alpha':quantity(F(1,100000),'1/K'),
        'coefficient_definition':'constant engineering_secant', 'cold_E':quantity(E1,'Pa'),'hot_E':quantity(150000000000,'Pa')},
        'expected':states, 'baselines':{'no_fit_cold_N':quantity(0,'N'),
        'no_fit_hot_N':quantity(-F(150000000000)*area*hot_eps,'N',power), 'cut_long_cold_N':quantity(-E1*area*(-fit_eps),'N',power)}}
assert fit['generic_reviewed']['expected']['cold']['fixed_wall_N']['value'] == 40000
assert fit['generic_reviewed']['expected']['hot']['fixed_wall_N']['value'] == -89976
case('signed_fit_states', 6,
     ['lambda_fit=1+delta_L_fit/L', 'eps_star=lambda_fit*(1+eps_th)-1', 'N=E*As*((u_j-u_i)/L-eps_star)',
      'fixed: N=-E*As*eps_star', 'released: u_tip=L*eps_star, N=0'], fit,
     {'hot_additive_strains':{'wrong_generic_N':quantity(-F(150000000000)*F(1,1000)*(fit_eps+hot_eps),'N')},
      'hot_uses_cold_E':{'wrong_generic_N':quantity(-E1*F(1,1000)*total_hot,'N')},
      'fit_applied_twice':{'wrong_cold_generic_N':quantity(-E1*F(1,1000)*(2*fit_eps),'N')},
      'sign_flipped':{'wrong_cold_generic_N':quantity(-40000,'N')},
      'misfit_plus_anchor_motion': 'Applying an equivalent fit eigenstrain and an additional equivalent support motion double-counts one physical mismatch.'},
     ['Cold/hot/return are independent linear equilibria, not a qualified contact/friction history.',
      'Direct fit strain delta_L_fit/L is an alternative input, not a second source.',
      'delta_L_fit<=-L is invalid; unsupported state must not publish successful zero.',
      'Natural-length composition does not make the small-strain constitutive law an exact finite-strain rod.'])

source_variants = {}
for name, k, power in [('generic_reviewed',F(1000),0), ('annular_companion',E1*As,1)]:
    common = {'preload':quantity(100,'N'),'weight':quantity(-1000,'N'),'independent_action':quantity(200,'N')}
    if power: common.update({'geometry_ref':geometry['id'],'E':quantity(E1,'Pa'),'L':quantity(1,'m'),
        'topology':'Axial cantilever; +100N is an explicitly equivalent affine tip action for this reference, not a qualified device law.'})
    common['structure_stiffness'] = quantity(k,'N/m',power)
    source_variants[name] = {'inputs':common,
        'expected':{'combined_rhs':quantity(-700,'N'),'combined_displacement':quantity(-F(700)/k,'m',-power),
                    'root_Fx':quantity(700,'N'), 'weight_total_rhs':quantity(-900,'N'),
                    'additional_total_rhs':quantity(300,'N'),'naive_total_sum_rhs':quantity(-600,'N'),
                    'naive_total_sum_displacement':quantity(-F(600)/k,'m',-power),
                    'two_distinct_equal_actions_rhs':quantity(400,'N'),'excluded_unreferenced_action_rhs':quantity(-700,'N')}}
case('persistent_source_once', 9, ['F_combined=F_preload+F_weight+F_independent', 'u=F_combined/k',
     '(F_preload+F_weight)+(F_preload+F_independent) duplicates F_preload'], source_variants,
     {'duplicate_source_id':'Reject duplicate inclusion of the same source ID, even if factors differ.',
      'numeric_deduplication':'source:action-a=+200N and source:action-b=+200N are two real sources and sum to +400N.',
      'implicit_case_array':'An unreferenced stored +999N action remains excluded; do not add it implicitly.'},
     ['Scalable ordinary applied-source factors do not multiply thermal/fit/reference/device states.',
      'Annular equivalent tip action checks affine counting; it does not qualify physical hanger preload/lock behavior.',
      'Source inclusion is explicit; stable identity, not labels or numeric equality, decides duplication.'])
cases['persistent_source_once']['source_ledger'] = [
    {'source_id':'device:preload','owner_kind':'selected_device_reference','classification':'affine_reference','force_N':100,'consumed_once':True},
    {'source_id':'source:weight','owner_kind':'stored_primitive','classification':'ordinary_applied','factor':1,'force_N':-1000},
    {'source_id':'source:independent','owner_kind':'stored_primitive','classification':'ordinary_applied','factor':1,'force_N':200},
    {'source_id':'source:stored-unused','owner_kind':'stored_primitive','classification':'excluded','force_N':999}]

negative = [
    {'id':'duplicate_temperature','expected':'diagnostic/no qualified state'},
    {'id':'missing_thermal_interval_bracket','expected':'diagnostic/no extrapolation'},
    {'id':'datum_outside_thermal_coverage','expected':'diagnostic/no fabricated reference'},
    {'id':'nonpositive_free_stretch','expected':'diagnostic/no qualified state'},
    {'id':'rotation_with_length_units','expected':'dimension diagnostic'},
    {'id':'translation_with_angle_units','expected':'dimension diagnostic'},
    {'id':'unknown_coefficient_definition','expected':'typed-boundary rejection'},
    {'id':'same_member_fit_length_and_fit_strain','expected':'mutually exclusive representations rejected'},
    {'id':'same_member_legacy_thermal_plus_resolved_thermal','expected':'double-consumption rejection unless explicitly distinct authorized sources'},
]
record = {'schema_version':'independent.load_reference_state_examples/1.0.0',
    'purpose':'Maintained independent analytical expectations; not authored product request DTOs or observed solver outputs.',
    'source_revision':REV,'sources':sources,
    'numbers':'Invented design/test quantities; decimal input rationals are exact mathematical targets. No operational material/code data.',
    'numeric_representation':{'exact_is_authoritative':True,'pi_decimal':PI_TEXT,'decimal_precision':85,
        'value':'Binary64 convenience projection of the independent expectation, never a product observation.',
        'tolerance':'Preserve existing protected relative 1e-9 criteria; use existing dimension-aware zero-reference handling. No new or relaxed threshold is allocated here.',
        'input_rounding':'Authored decimal rationals are generally not binary64-exact; this reference does not claim exact represented-operand equality.'},
    'sign_conventions':{'coordinates':'Straight reference axis +X from i to j; +Y and right-hand +RZ.',
        'axial':'N>0 tension; support-on-member root=-N, far=+N for a force-free span.',
        'beam':'Positive right-hand moments and global forces; all-fixed values are support-on-member actions.',
        'fit':'delta_L_fit<0 is cut short; displayed reference coordinates stay unchanged.'},
    'geometry':geometry,'cases':cases,'negative_contract_controls':negative,
    'open_capabilities':['M21 full hydrostatic head/mass-state and support lock/device closure remain unfinished.',
        'Nonlinear predecessor/internal-history qualification is not supplied by independent cold/hot/return states.',
        'Actual public/native/source-recovery/persistence checks remain implementation obligations.']}
(OUT/'reference_cases.json').write_text(json.dumps(record,indent=2,allow_nan=False)+'\n')

report = {'actor':'/root/physics_resume/membrane_backcheck','parent':'/root/physics_resume',
    'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checkout':str(ROOT),'python':sys.version,
    'method':'Independent Fraction closed forms and Decimal exp; existing reviewed design translated; no product imports or observed result oracle.',
    'read_only_Git_authorization':'Parent clarified read-only show/rev-parse/hash/diff authorized; no Git mutation.',
    'source_hashes_verified':sources,'case_count':len(cases),'checks':'Annulus identity, serial/parallel force balance, beam moment balance, thermal43/25009, split log composition, cold40000/hot-89976 assert passed.',
    'output':str(OUT/'reference_cases.json'),'output_sha256':hashlib.sha256((OUT/'reference_cases.json').read_bytes()).hexdigest(),
    'limitations':record['open_capabilities']}
(HERE/'generation.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'cases':len(cases),'source_documents_verified':len(sources),'output_sha256':report['output_sha256']},indent=2))
