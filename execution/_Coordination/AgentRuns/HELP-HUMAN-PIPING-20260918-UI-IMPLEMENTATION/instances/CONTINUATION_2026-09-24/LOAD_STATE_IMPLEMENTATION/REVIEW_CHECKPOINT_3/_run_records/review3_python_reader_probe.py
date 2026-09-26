"""CP3 review: Python readers on real producer outputs and the big-integer parity probe.
usage (from WORKING_ROOT): python review3_python_reader_probe.py <unavailable-sparse.raw.json> <unavailable-dense.raw.json>
The two unavailable envelopes are producer output of probe_unavailable.request.json
(examples/physics_source_connected sparse|dense)."""
import json, sys, copy
sys.path.insert(0, '.')
from core.analysis_runs.compatibility import _source_contract, numerical_use_standing
from core.analysis_runs.load_reference_evidence import validate_load_reference_evidence
F = 'fixtures/product_preview/'
cases = {'unavailable-sparse': json.load(open(sys.argv[1])), 'unavailable-dense': json.load(open(sys.argv[2]))}
for n in ['connected-sparse_interactive', 'connected-dense_scrutiny', 'pressure-sparse_interactive', 'pressure-dense_scrutiny']:
    cases['load_reference/' + n] = json.load(open(F + 'load_reference/' + n + '.raw.json'))
for n in ['eigen_motion-sparse_interactive', 'mixed-dense_scrutiny']:
    cases['load_reference_source/' + n] = json.load(open(F + 'load_reference_source/' + n + '.raw.json'))
base = json.load(open(F + 'load_reference/pressure-dense_scrutiny.raw.json'))
for label, big in [('bigint_temperature', 10 ** 20), ('bigfloat_temperature', 1e20)]:
    e = copy.deepcopy(base)
    m = e['contract_evidence']['load_reference_states'][0]['members'][0]
    m['operating_temperature_k'] = m['material_selection_temperature_k'] = m['consumed_material_points'][0]['temperature_k'] = big
    cases[label] = json.loads(json.dumps(e))
    open(label + '.raw.json', 'w').write(json.dumps(e)) if '--write' in sys.argv else None
for name, raw in cases.items():
    for label, fn in [('dispatch', lambda r: _source_contract(r)[0]), ('validator', validate_load_reference_evidence)]:
        try:
            out = fn(raw); print('PY', name, label, 'ACCEPT', out or '')
        except Exception as error:
            print('PY', name, label, 'REJECT', str(error)[:140])
