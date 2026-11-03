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


def test_range_rules_run_only_on_a_well_formed_range():
    # A range that fails the shape check gets its shape findings and no range-order
    # finding (DEL-04-11 Design §8.2; persistence review of #79).
    shape_failing = [c for c in CASES if c['name'] in (
        '1.1.0 working load range negative ends fail shape only',
        '1.1.0 working load range not an object',
        '1.1.0 working load range end missing unit',
        '1.1.0 working load range missing maximum',
        '1.1.0 working load range extra member',
    )]
    assert len(shape_failing) == 5
    for case in shape_failing:
        result = validate_library_import(case['payload'], library_kind='hanger', intended_visibility=case['visibility'])
        assert 'IMPORT_HANGER_LOAD_RANGE_INVALID' not in {f.code for f in result.findings}, case['name']


def test_range_magnitude_beyond_binary64_is_refused_by_shape():
    # The bounded interpreters admit only finite binary64 magnitudes. A full JSON Schema
    # validator admits 10**400 as a positive integer; the boundary refuses it, as for
    # every force quantity before format 1.1.0 (Design §8.2, numeric domain).
    case = next(c for c in CASES if c['name'] == '1.1.0 working load range valid')
    payload = copy.deepcopy(case['payload'])
    payload['hanger_records'][0]['hanger']['working_load_range']['maximum']['magnitude'] = 10**400
    result = validate_library_import(payload, library_kind='hanger', intended_visibility='private')
    assert not result.accepted
    assert 'IMPORT_HANGER_LOAD_RANGE_INVALID' not in {f.code for f in result.findings}
