import copy, json, math, sys
src, out = sys.argv[1:]
model = json.load(open(src, encoding='utf-8'))

def wrap(m):
    return {'model': m, 'materials': copy.deepcopy(m['materials'])}

def set_mu(m, coefficient):
    for support in m['supports']:
        if support['id'] == 'support:NL-130-FRIC':
            support['nonlinear']['friction_coefficient']['value'] = coefficient
            return
    raise RuntimeError('friction support missing')

def add_probe(m, case_index, force):
    load = {
        'id': f'load:F4-AFFINE-PROBE-L{case_index + 1}',
        'category': 'occasional',
        'target': {'type': 'node', 'node': 'node:N-130'},
        'direction': 'global_z',
        'magnitude': {'value': force, 'unit': 'N'},
        'dimension': 'force',
        'provenance': 'invented_disposable_read_only_public_api_affine_probe'
    }
    m['load_cases'][case_index]['primitive_loads'].append(load)

controls = {'retained_spring': copy.deepcopy(model)}
old = copy.deepcopy(model)
old['supports'] = [
    s for s in old['supports']
    if s.get('stiffness') is None and s.get('family') != 'variable_spring_hanger'
]
nodes = {n['id']: n['position'] for n in old['nodes']}
pipes = {p['id']: p for p in old['pipe_segments']}
for case in old['load_cases']:
    converted = []
    for load in case.get('primitive_loads', []):
        if load.get('dimension') != 'force_per_length':
            converted.append(copy.deepcopy(load))
            continue
        pipe_id = load['target']['pipe']
        pipe = pipes[pipe_id]
        i = nodes[pipe['from']]
        j = nodes[pipe['to']]
        length = math.sqrt(sum((j[k] - i[k]) ** 2 for k in ('x', 'y', 'z')))
        for end, node_id in (('i', pipe['from']), ('j', pipe['to'])):
            nodal = copy.deepcopy(load)
            nodal['id'] = f"{load['id']}:historical-nodal-{end}"
            nodal['target'] = {'type': 'node', 'node': node_id}
            nodal['dimension'] = 'force'
            nodal['category'] = 'occasional'
            nodal['magnitude'] = {
                'value': load['magnitude']['value'] * length / 2.0,
                'unit': 'N',
            }
            converted.append(nodal)
    case['primitive_loads'] = converted
controls['historical_no_spring'] = old

variants = [('base_mu0', 0.0, None, None)]
for case_index, case_label in ((0, 'l100'), (1, 'l200')):
    for magnitude in (1, -1, 2, -2, 3, -3, 5, -5, 7, -7, 10, -10, 17, -17, 23, -23, 37, -37, 50, -50):
        sign = 'plus' if magnitude > 0 else 'minus'
        variants.append((f'probe_{case_label}_{sign}{abs(magnitude)}_mu0', 0.0, case_index, float(magnitude)))
for index, magnitude in enumerate((0.123456, -0.123456, 0.314159, -0.314159, 0.707107, -0.707107, 1.414213, -1.414213, 2.718282, -2.718282, 3.141593, -3.141593, 6.283185, -6.283185, 11.111111, -11.111111, 19.876543, -19.876543, 31.415927, -31.415927, 47.123456, -47.123456, 49.885, -49.89, 49.8703, -49.8805)):
    variants.append((f'probe_l200_fractional_{index:02d}_mu0', 0.0, 1, magnitude))
variants.extend([
    ('target_mu001', 0.01, None, None),
    ('cross_mu0005', 0.005, None, None),
    ('cross_mu002', 0.02, None, None),
])

for label, control in controls.items():
    for variant, mu, case_index, probe in variants:
        item = copy.deepcopy(control)
        set_mu(item, mu)
        if probe is not None:
            add_probe(item, case_index, probe)
        with open(f'{out}/{label}_{variant}.json', 'w', encoding='utf-8', newline='\n') as f:
            json.dump(wrap(item), f, sort_keys=True, separators=(',', ':'))
            f.write('\n')
