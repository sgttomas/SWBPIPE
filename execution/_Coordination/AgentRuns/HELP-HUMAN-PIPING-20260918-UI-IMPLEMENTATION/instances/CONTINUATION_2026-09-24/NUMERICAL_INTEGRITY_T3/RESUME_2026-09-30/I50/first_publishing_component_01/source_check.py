"""Checkpoint 0: source/fixture checks only. No model, Rust, or solver execution."""
from pathlib import Path
from fractions import Fraction as F
import hashlib, json, re, struct

CODE = Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a')
NUM = CODE.parent / 'numerics'
P = Path('projects/chirality-piping')
OUT = Path(__file__).resolve().parent
test_path = P / 'core/product_physics/tests/formation_check_runtime.rs'
test = (CODE / test_path).read_text()
raw = re.search(r'const RF_SKEW_T_CANT_OFF_122_R1E_04: &str = r#"(.*?)"#;', test).group(1).encode()
request = json.loads(raw)
m = request['model']
assert m['project']['id'] == 'invented:t3-p1:RF-SKEW-T-CANT-OFF-122-r1e-04'
assert request['materials'] == [] and m['combinations'] == []
assert len(m['load_cases']) == 1 and len(m['pipe_segments']) == 1
assert len(m['nodes']) == 2 and len(m['materials']) == 1
assert m['pipe_segments'][0]['y_reference'] == {'x': 1, 'y': 0, 'z': 0}
assert [m['nodes'][1]['position'][v] for v in 'xyz'] == [1, 2, 2]
assert m['supports'][0]['id'] == 'rigid:N0'
assert m['supports'][0]['restraints'] == ['UX', 'UY', 'UZ']
assert 'family' not in m['supports'][0]
springs = m['supports'][1:]
assert [(s['id'], s['node'], s['restraints'], s['stiffness']['dof'], s['stiffness']['value']['value'], s['stiffness']['value']['unit']) for s in springs] == [
    ('spring:N0:0', 'N0', ['RX'], 'RX', 144, 'N*m/rad'),
    ('spring:N0:1', 'N0', ['RY'], 'RY', 1000000, 'N*m/rad'),
    ('spring:N0:2', 'N0', ['RZ'], 'RZ', 1000000, 'N*m/rad')]
loads = m['load_cases'][0]['primitive_loads']
assert [(x['direction'], x['magnitude']['value']) for x in loads] == [('RX', .0048), ('RY', .0096), ('RZ', .0096)]
assert all(x['category'] == 'concentrated_moment' and x['target'] == {'type': 'node', 'node': 'N1'} and x['magnitude']['unit'] == 'N*m' for x in loads)
fraction = lambda x: F.from_float(float(x))
bits = lambda x: struct.pack('>d', float(x)).hex()
alpha = fraction(loads[0]['magnitude']['value'])
moments = [fraction(x['magnitude']['value']) for x in loads]
assert moments == [alpha, 2 * alpha, 2 * alpha]
theta0 = [v / fraction(s['stiffness']['value']['value']) for v, s in zip(moments, springs)]
# Exact identity checks, not an executed mechanics solution.
rigid_tip = [2 * theta0[1] - 2 * theta0[2], theta0[2] - 2 * theta0[0], 2 * theta0[0] - theta0[1]]
assert rigid_tip[0] == 0 and rigid_tip[1] == -rigid_tip[2]
frozen = ['core/product_physics/src/retained_product.rs', 'core/product_physics/src/retained_product_tests.rs',
          'core/product_physics/src/lib.rs', 'core/product_physics/src/preview_physics.rs',
          'core/solver/linear_supports/src/lib.rs', 'core/solver/frame_kernel/src/structural/retained/source.rs',
          'core/solver/frame_kernel/src/structural/retained/recover.rs',
          'core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs',
          'core/solver/frame_kernel/src/structural/retained/product_certificate/source_residual.rs']
hashes = []
for relative in [str(test_path)] + [str(P / p) for p in frozen]:
    a, b = (CODE / relative).read_bytes(), (NUM / relative).read_bytes()
    assert a == b, relative
    hashes.append({'relative': relative, 'bytes': len(a), 'sha256': hashlib.sha256(a).hexdigest(), 'code_num_equal': True})
adapter = (CODE / P / frozen[0]).read_text()
assert 'if !springs.is_empty()' in adapter
assert 'springs: vec![]' in adapter
assert 'k::QuantityId::Reaction(k::Dof {\n                        node: self.supports[s].1' in adapter
assert 'fn kd5_required_true_positive_122_demotes_in_both_modes_on_both_entries()' in test
assert 'for (entry, envelope) in both_entries(&value, mode)' in test
(OUT / 'NAMED_REQUEST.json').write_bytes(raw)
result = {'kind': 'static-source-and-exact-input-check-only', 'fresh_runtime': False,
          'request_bytes': len(raw), 'request_sha256': hashlib.sha256(raw).hexdigest(),
          'source_derived_expected_census': {'nodes': 2, 'members': 1, 'materials': 1, 'cases': 1, 'stations': 3,
              'global_springs': 3, 'constraints': 3, 'individual_load_terms': 3, 'supports': 4,
              'native_Q': 7*2+12+6*3+3+3+2*4, 'support_component_rows': 24,
              'support_component_laws': {'reaction': 3, 'spring_action': 3, 'empty': 18},
              'support_magnitude_rows': 8, 'final_rows': 7*2+12+6*3+8*4+21+1, 'mechanical_rows': 97},
          'bits': {'outside_diameter': bits(.2), 'wall': bits(.01), 'E': bits(200e9), 'G': bits(80e9),
                   'moments': [bits(x['magnitude']['value']) for x in loads],
                   'springs': [bits(s['stiffness']['value']['value']) for s in springs]},
          'exact_identity_checks': {'moment_parallel_to_chord': True, 'alpha': str(alpha), 'length_squared': 9,
              'root_rotation_law_M_over_k': [str(x) for x in theta0], 'rigid_tip_cross_product': [str(x) for x in rigid_tip]},
          'code_num_comparison': hashes}
(OUT / 'SOURCE_CHECK.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'passed': True, 'request_sha256': result['request_sha256'], 'expected_census': result['source_derived_expected_census'], 'runtime_executed': False}))
