"""Invented inputs only: schema completeness is separate from solve readiness."""
import copy
import json
import sys
from pathlib import Path
import pytest
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from core.library_import.provenance_checker import validate_library_import, _hanger_shape
SCHEMA = json.loads((ROOT / 'schemas/hanger.schema.yaml').read_text())
CASES = json.loads((ROOT / 'fixtures/hanger/invented_hanger_cases.json').read_text())


def test_schema_is_valid_draft_2020_12():
    Draft202012Validator.check_schema(SCHEMA)


@pytest.mark.parametrize('case', CASES, ids=lambda c: c['name'])
def test_hanger_schema_and_runtime_agree(case):
    payload = copy.deepcopy(case['payload'])
    before = copy.deepcopy(payload)
    errors = []
    _hanger_shape(payload, SCHEMA, SCHEMA, '$', errors)
    assert bool(errors) == bool(list(Draft202012Validator(SCHEMA).iter_errors(payload)))
    result = validate_library_import(payload, library_kind='hanger', intended_visibility=case['visibility'])
    assert result.outcome == case['outcome']
    if case['required_code']:
        assert case['required_code'] in {f.code for f in result.findings}
    if case.get('required_path'):
        assert (case['required_code'], case['required_path'], case['required_severity']) in {
            (f.code, f.path, f.severity) for f in result.findings
        }
    assert payload == before  # No normalization, defaults or source metadata loss.


@pytest.mark.parametrize('number', [float('nan'), float('inf'), -float('inf'), -1, False, 10**400])
def test_nonfinite_and_nonpositive_values_rejected(number):
    payload = copy.deepcopy(CASES[0]['payload'])
    payload['hanger_records'][0]['hanger']['installed_load']['magnitude'] = number
    assert not validate_library_import(payload, library_kind='hanger', intended_visibility='private').accepted


@pytest.mark.parametrize('dimension,units,field', [('force',['N','lbf'],'installed_load'), ('length',['m','mm','in','ft'],'travel_range'), ('force_per_length',['N/m','lbf/ft','lbf/in'],'stiffness')])
def test_all_supported_units(dimension, units, field):
    for unit in units:
        payload = copy.deepcopy(CASES[0]['payload'])
        q = payload['hanger_records'][0]['hanger'][field]
        if field == 'stiffness': q = q['value']
        q.update(unit=unit, dimension=dimension)
        assert validate_library_import(payload, library_kind='hanger', intended_visibility='private').accepted


def test_unknown_schema_keyword_fails_closed():
    findings = []
    _hanger_shape({}, {'type':'object','unsupported':True}, {}, '$', findings)
    assert findings[0].code == 'IMPORT_HANGER_SCHEMA_INVALID'
