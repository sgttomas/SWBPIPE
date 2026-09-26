#!/usr/bin/env python3
"""Derive execution metadata for the two reviewed original static cases.

Never reads a solver output and never recomputes a target. The original package
stays unchanged; every input, selector, value and criterion is traced by hash.
A resulting selection is not evidence that its adapter/build or run has passed.
"""
from __future__ import annotations
import argparse
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import os
import re
try:
    from .qualification_physics import SELECTED_ADMISSION_FILES
except ImportError:
    from qualification_physics import SELECTED_ADMISSION_FILES

FORMAT = 'openpipestress.qualification_selection/1'
TRANSPORT = 'ordinary_physics_1_cli_1.0_raw0.2'
PACKET_HASH = '0603a990838166c1da43b26f8b657dae3051289097700a76859d1afaa45db02b'
BINDING_HASH = 'be6e332513634f747a1f44c6f38bb94bd2ad0d7ce8e3caa75ccfcdfc9124bdf9'
ADDENDUM_HASH = '1ed8aac0910318902cd5f690aeee8cb74bfee036617e33eaa4e2c23ec98ff3c2'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(path, expected=None):
    data = path.read_bytes()
    if expected is not None and digest(data) != expected:
        raise ValueError('reviewed source hash differs: ' + str(path))
    return json.loads(data), data


def link(path, base):
    return {'path': os.path.relpath(path.resolve(), base.resolve()), 'sha256': digest(path.read_bytes())}


def write(path, data):
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False, allow_nan=False) + '\n')


def build(package: Path, output: Path, candidate_commit: str, executable_sha256: str,
          physics_binding: Path, mode: str) -> Path:
    if mode not in ('sparse_interactive', 'dense_scrutiny'):
        raise ValueError('unsupported closed mode')
    if not re.fullmatch('[0-9a-f]{40}', candidate_commit) or not re.fullmatch('[0-9a-f]{64}', executable_sha256):
        raise ValueError('exact candidate and executable hashes required')
    packet, _ = read(package/'PACKET_MANIFEST.json', PACKET_HASH)
    for entry in packet['files']:
        data = (package/entry['path']).read_bytes()
        if digest(data) != entry['sha256']:
            raise ValueError('original reviewed packet file changed: ' + entry['path'])
    basis, _ = read(package/'INPUT_REFERENCE_BINDINGS.json', BINDING_HASH)
    # Explicit technical selection and its independent backcheck are evidence,
    # not reconstructed or inferred from a candidate's review-pending labels.
    for name, expected in SELECTED_ADMISSION_FILES.items():
        if digest((package/name).read_bytes()) != expected:
            raise ValueError('selected review/admission bytes differ: ' + name)
    read(package/'STRUCTURAL_NUMERIC_ADDENDUM/STRUCTURAL_NUMERIC_BINDING.json', ADDENDUM_HASH)
    binding, _ = read(physics_binding)
    if binding.get('status') != 'reviewed_candidate' or not binding.get('review_basis'):
        raise ValueError('physical reader binding not reviewed/admitted')
    output.mkdir(parents=True, exist_ok=False)
    cases, derivations = [], []
    for source in basis['cases']:
        case_id = source['case_id']
        input_path = package/source['input']['path']
        read(input_path, source['input']['sha256'])
        reference, reference_bytes = read(package/source['reference']['path'], source['reference']['sha256'])
        criterion, criterion_bytes = read(package/source['criterion']['path'], source['criterion']['sha256'])
        selectors, selector_bytes = read(package/source['selectors']['path'], source['selectors']['sha256'])
        admitted_reference = deepcopy(reference)
        admitted_reference['readiness'] = 'ready'
        admitted_reference['independent_review_ref'] = {
            'reference_review': link(package/'INDEPENDENT_REVIEW/RETURN.md', output),
            'technical_selection': link(package/'ROOT_SELECTION.md', output),
            'scope': 'agent technical review/selection under owner delegation; no personal human equation review or release acceptance'}
        admitted_criterion = deepcopy(criterion)
        admitted_criterion['tolerance_profile']['profile_status'] = 'reviewed'
        for rule in admitted_criterion['tolerance_profile']['rules']:
            rule['review'] = {'source_review': link(package/'INDEPENDENT_REVIEW/RETURN.md', output),
                              'technical_selection': link(package/'ROOT_SELECTION.md', output),
                              'human_equation_review_claimed': False}
        # Only explicitly selected metadata is changed. All numeric values,
        # rules/units/predicates, zeros and selectors remain identical objects.
        if admitted_reference['values'] != reference['values']:
            raise ValueError('target substitution')
        restored = deepcopy(admitted_criterion)
        restored['tolerance_profile']['profile_status'] = criterion['tolerance_profile']['profile_status']
        for new, original in zip(restored['tolerance_profile']['rules'], criterion['tolerance_profile']['rules']):
            new['review'] = original['review']
        if restored != criterion:
            raise ValueError('criterion substitution')
        stem = 'axial' if case_id == 'original_static_axial_v1' else 'bending_torsion'
        ref_path, criterion_path = output/(stem+'.reference.json'), output/(stem+'.criteria.json')
        write(ref_path, admitted_reference)
        write(criterion_path, admitted_criterion)
        assertions = [{key: deepcopy(row[key]) for key in ('id', 'selector', 'criterion_rule_id')} for row in selectors['assertions']]
        if len(assertions) != 73 or len({x['id'] for x in assertions}) != 73:
            raise ValueError('reviewed scalar denominator differs')
        cases.append({'id': case_id, 'input': link(input_path, output), 'reference': link(ref_path, output),
                      'criterion': link(criterion_path, output), 'assertions': assertions,
                      'structural': {'obligations': link(package/'STRUCTURAL_EXPECTATIONS.json', output),
                                     'section_reference': link(package/'section.reference.json', output),
                                     'section_criteria': link(package/'STRUCTURAL_NUMERIC_ADDENDUM/STRUCTURAL_NUMERIC_BINDING.json', output)}})
        derivations.append({'case_id': case_id, 'input_unchanged': source['input'],
                            'original_reference_sha256': digest(reference_bytes), 'original_criterion_sha256': digest(criterion_bytes),
                            'original_selectors_sha256': digest(selector_bytes),
                            'reference_numeric_values_unchanged': True, 'criterion_nonreview_fields_unchanged': True,
                            'assertion_id_selector_rule_fields_unchanged': True,
                            'selector_origin_metadata_retained_in_original': source['selectors']})
    selection = {'format': FORMAT, 'profile_id': 'first_static_ordinary_physics_1/'+mode,
                 'purpose': 'development_comparison', 'transport': TRANSPORT,
                 'runner': {'candidate_commit': candidate_commit, 'executable_sha256': executable_sha256,
                            'solver_mode': mode, 'explicit_local_private_intent': True},
                 'physics_binding': link(physics_binding, output),
                 'reference_basis': link(package/'INPUT_REFERENCE_BINDINGS.json', output), 'cases': cases}
    path = output/'selection.json'
    write(path, selection)
    write(output/'DERIVATION.json', {'source_package_binding_sha256': BINDING_HASH, 'structural_addendum_sha256': ADDENDUM_HASH,
                                    'selection_sha256': digest(path.read_bytes()), 'derivations': derivations,
                                    'scalar_obligations': 146, 'structural_obligations': 20, 'section_subchecks': 18,
                                    'actual_runs_performed': 0, 'claim': 'execution metadata derivation only; no observed target values or profile qualification'})
    return path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--package', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--candidate-commit', required=True)
    parser.add_argument('--executable-sha256', required=True)
    parser.add_argument('--physics-binding', type=Path, required=True)
    parser.add_argument('--mode', choices=('sparse_interactive', 'dense_scrutiny'), required=True)
    args = parser.parse_args()
    print(build(args.package.resolve(), args.output.resolve(), args.candidate_commit,
                args.executable_sha256, args.physics_binding.resolve(), args.mode))


if __name__ == '__main__':
    main()
