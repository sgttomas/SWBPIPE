#!/usr/bin/env python3
"""Offline draft-shape checks. No producer execution or semantic conformance claim."""
import copy
import json
from pathlib import Path
from jsonschema import Draft202012Validator

HERE = Path(__file__).resolve().parent

def reject(validator, value, description):
    if validator.is_valid(value):
        raise AssertionError('Negative witness unexpectedly accepted: ' + description)

for name in ('ValidationResult', 'TransformResult', 'ComparisonResult'):
    schema = json.loads((HERE / (name + '.v0.1.draft.schema.json')).read_text())
    fixture = json.loads((HERE / 'fixtures' / (name + '.shape-only.json')).read_text())
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)
    validator.validate(fixture)
    negative = copy.deepcopy(fixture)
    negative['schema_version'] = '99.0.0'
    reject(validator, negative, name + ' unsupported version')
    negative = copy.deepcopy(fixture)
    negative['professional_boundary']['software_makes_approval_claim'] = True
    reject(validator, negative, name + ' automatic approval')
    negative = copy.deepcopy(fixture)
    negative['source_refs'] = []
    reject(validator, negative, name + ' absent source binding')
    negative = copy.deepcopy(fixture)
    if name == 'ValidationResult':
        del negative['payload']['diagnostics']
    elif name == 'TransformResult':
        negative['payload']['analytical_model']['model_role'] = 'physical_source_of_truth'
    else:
        negative['payload']['comparison_review']['participants'] = []
    reject(validator, negative, name + ' missing required domain structure')
    print(name + ': draft metaschema and positive shape PASS; 4 negative witnesses rejected')
print('Only structural draft preparation checked; IF-001..007 semantic/producer witnesses remain outstanding.')
