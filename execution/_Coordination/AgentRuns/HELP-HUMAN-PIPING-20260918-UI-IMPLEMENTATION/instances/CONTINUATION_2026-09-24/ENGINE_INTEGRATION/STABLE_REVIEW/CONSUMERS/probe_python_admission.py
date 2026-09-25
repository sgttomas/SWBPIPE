"""Independent corruption probes; no producer values used as physical oracles.

Run with python3 -B from this directory. Writes only the explicitly supplied
output path, or stdout. The checkout must remain at the intended candidate.
"""
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import sys

PROJECT = next(p for p in Path(__file__).resolve().parents if (p / 'core/analysis_runs/compatibility.py').is_file())
sys.path.insert(0, str(PROJECT))
from core.analysis_runs.compatibility import (
    _source_contract, build_analysis_run, numerical_use_standing,
    validate_analysis_run_v0_3, verify_analysis_run_record,
)

fixture = PROJECT / 'fixtures/results/physics_connected_mechanics_sparse.json'
source = json.loads(fixture.read_text())

def extra_maximum(s):
    row = deepcopy(next(r for r in s['results'] if r['kind'] == 'pipe_elastic_normal_stress_maximum_v2'
                        and r['basis_ref']['ref_id'] == s['contract_evidence']['exact_cases'][1]['load_case_id']))
    row.update(id='unbound:maximum', value=100000000.25)
    s['results'].append(row)
    s['summary']['max_open_formula_stress'] = dict(value=row['value'], unit=row['unit'], location_ref=row['entity_ref'], result_ref=row['id'])

def cyclic_non_v2(s):
    row = next(r for r in s['results'] if not r['kind'].endswith('_v2'))
    row['source_result_refs'] = [{'ref_type':'result_value', 'ref_id':row['id']}]

def duplicate_group(s):
    groups = s['contract_evidence']['exact_cases'][0]['pressure_rhs_assembly']['groups']
    groups.append(deepcopy(groups[0]))

mutations = {
    'unmodified_control': lambda s: None,
    'unbound_extremum_headline_100000000_25': extra_maximum,
    'gap_zero_despite_nonzero_global_minus_lower': lambda s: s['contract_evidence']['exact_cases'][1]['pipe_stress_extrema'][0].update(certified_gap_pa=0),
    'unknown_extrema_coefficient_basis': lambda s: s['contract_evidence']['exact_cases'][0]['pipe_stress_extrema'][0].update(coefficient_basis='source_blocks_recovery_v1'),
    'closure_transfer_disagreement': lambda s: s['contract_evidence']['pressure'][0]['terminals'][0].update(pipe_cap_transfer_global_n=[999, 0, 0]),
    'duplicate_terminal': lambda s: s['contract_evidence']['pressure'][0]['terminals'].__setitem__(1, deepcopy(s['contract_evidence']['pressure'][0]['terminals'][0])),
    'rhs_term_coefficient_999': lambda s: s['contract_evidence']['exact_cases'][0]['pressure_rhs_assembly']['groups'][0]['terms'][0].update(coefficient=999),
    'rhs_pressure_moment_999': lambda s: s['contract_evidence']['exact_cases'][0]['pressure_rhs_assembly']['assembled_pressure_rhs_global'].__setitem__(3, 999),
    'duplicate_rhs_group': duplicate_group,
    'cyclic_non_v2_operand': cyclic_non_v2,
}
results = []
for name, mutate in mutations.items():
    candidate = deepcopy(source)
    mutate(candidate)
    result = {'name':name}
    try:
        result['source_contract'] = _source_contract(candidate)[0]
    except Exception as error:
        result['source_contract_error'] = f'{type(error).__name__}: {error}'
    result['numerical_use_standing'] = numerical_use_standing(candidate, [q['basis_ref'] for q in candidate['numerical_quality']['cases']])
    try:
        record = build_analysis_run(candidate, input_manifest_ref={'object_type':'InputManifest','ref':'manifest:independent-admission-probe'}, input_manifest_hash='1' * 64)
        validate_analysis_run_v0_3(record, candidate)
        result['public_analysis_builder'] = 'accepted'
        result['record_checksum_verification'] = verify_analysis_run_record(record)
    except Exception as error:
        result['public_analysis_builder_error'] = f'{type(error).__name__}: {error}'
    results.append(result)
output = {'scope':'Reader admission and immutable analysis construction only; no authentic producer/current/native qualification is claimed.',
          'source_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),
          'reader_sha256':hashlib.sha256((PROJECT/'core/analysis_runs/physics_evidence.py').read_bytes()).hexdigest(),
          'results':results}
text = json.dumps(output, indent=2) + '\n'
if len(sys.argv) > 1:
    Path(sys.argv[1]).write_text(text)
print(text, end='')
