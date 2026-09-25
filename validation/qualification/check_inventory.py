#!/usr/bin/env python3
"""Read-only consistency check for this bounded planning inventory.

This is not a benchmark runner or a qualification gate. It checks identities,
links, denominator declarations and pinned Git source bytes without running code
from those sources, building, downloading, or changing reference data.
"""
from __future__ import annotations
import argparse
from copy import deepcopy
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess

DEFAULT = Path(__file__).with_name('capability_inventory.json')
VP_IDS = set('VP-REF VP-GO VP-SCOPE VP-HARNESS VP-ORACLES VP-SOURCES VP-STATIC VP-RESTRAINT VP-PUBLISHED VP-INTERCHANGE VP-LICENSED VP-WORKFLOW VP-ROBUST VP-DYNAMICS VP-PHYSICAL VP-SEARCH VP-ASSESS'.split())


def check(data: dict, root: Path, verify_git: bool = True) -> list[str]:
    errors: list[str] = []
    def require(condition: bool, message: str) -> None:
        if not condition:
            errors.append(message)
    tables = {}
    for table in ['sources', 'capabilities', 'output_sets', 'profiles', 'vp_mapping', 'findings', 'source_catalogue', 'deliverables', 'first_profile_cuts', 'work_packets']:
        rows = data.get(table, [])
        ids = [row.get('id') for row in rows]
        require(bool(ids) and all(type(x) is str and x for x in ids), f'{table}: empty/missing identity')
        require(len(ids) == len(set(ids)), f'{table}: duplicate identity')
        tables[table] = set(ids)
    def links(values: list, table: str, owner: str) -> None:
        require(len(values) == len(set(values)), f'{owner}: duplicate {table} references')
        for value in values:
            require(value in tables[table], f'{owner}: unknown {table} reference {value}')
    require(data.get('format') == 'piping-validation-planning-inventory-v1', 'unexpected inventory format')
    require(data.get('status') == 'bounded_vp_scope_and_harness_preparation', 'inventory must not claim programme qualification')
    require(tables['findings'] == {f'M{i:02}' for i in range(1, 39)}, 'original38 finding denominator changed')
    require(tables['vp_mapping'] == VP_IDS, 'VP coverage changed from pinned17-node route')
    closed = {row['id'] for row in data['findings'] if row['integration_state'] == 'closed_merged'}
    require(closed == {'M04', 'M09', 'M24', 'M35'}, 'closure differs from pinned graph; make a reviewed source/status update')
    for row in data['capabilities']:
        require(bool(row['source_refs']) and bool(row['limits']), f"{row['id']}: source/limits missing")
        links(row['source_refs'], 'sources', row['id'])
        links(row['finding_refs'], 'findings', row['id'])
        links(row['q_refs'], 'profiles', row['id'])
        links(row['required_output_sets'], 'output_sets', row['id'])
        links(row['deliverables'], 'deliverables', row['id'])
    for row in data['findings']:
        require(bool(row['capability_refs']), f"{row['id']}: missing capability obligation")
        links(row['capability_refs'], 'capabilities', row['id'])
        links([row['status_source']], 'sources', row['id'])
    for row in data['profiles']:
        require(bool(row['capabilities']) and bool(row['required_output_sets']), f"{row['id']}: empty planned denominator")
        require(row['qualification'] == 'not_run_under_programme', f"{row['id']}: unsupported programme outcome")
        require(row['required_case_inventory'] == 'not_frozen', f"{row['id']}: this inventory has no executed/frozen case list")
        links(row['capabilities'], 'capabilities', row['id'])
        links(row['vp_refs'], 'vp_mapping', row['id'])
        links(row['required_output_sets'], 'output_sets', row['id'])
    caps = {row['id']: row for row in data['capabilities']}
    profiles = {row['id']: row for row in data['profiles']}
    vps = {row['id']: row for row in data['vp_mapping']}
    for row in data['profiles']:
        expected_caps = {c['id'] for c in data['capabilities'] if row['id'] in c['q_refs']}
        require(set(row['capabilities']) == expected_caps, f"{row['id']}: capability reverse mapping differs")
        expected_outputs = set().union(*(set(caps[c]['required_output_sets']) for c in row['capabilities'] if c in caps))
        require(expected_outputs <= set(row['required_output_sets']), f"{row['id']}: capability-required outputs omitted")
    for row in data['vp_mapping']:
        links(row['q_refs'], 'profiles', row['id'])
        expected_q = {q['id'] for q in data['profiles'] if row['id'] in q['vp_refs']}
        require(set(row['q_refs']) == expected_q, f"{row['id']}: profile reverse mapping differs")
    for row in data['output_sets']:
        require(bool(row['required']) and len(row['required']) == len(set(row['required'])), f"{row['id']}: empty/duplicate outputs")
    for row in data['first_profile_cuts']:
        require(bool(row['cases']) and bool(row['held_requirements']), f"{row['id']}: case/exclusion obligations absent")
        links(row['q_refs'], 'profiles', row['id'])
        links(row['source_refs'], 'sources', row['id'])
        links(row['required_output_sets'], 'output_sets', row['id'])
    for row in data['source_catalogue']:
        require(row['programme_outcome'] == 'not_run' and bool(row['missing']), f"{row['id']}: source readiness is not a passing programme")
        links(row['source_refs'], 'sources', row['id'])
        if 'work_packet_ref' in row:
            links([row['work_packet_ref']], 'work_packets', row['id'])
    for table in ['contracts', 'harness_gaps']:
        ids = [row['id'] for row in data[table]]
        require(bool(ids) and len(ids) == len(set(ids)), f'{table}: empty/duplicate entries')
        for row in data[table]:
            require(bool(row['source_refs']), f"{row['id']}: empty contract/gap source basis")
            links(row['source_refs'], 'sources', row['id'])
    for row in data['deliverables']:
        links([row['scope_source']], 'sources', row['id'])
    for packet in data['work_packets']:
        require(packet['ready_for_campaign_scoring'] is False, f"{packet['id']}: unreviewed reference promoted")
        require(bool(packet['files']), f"{packet['id']}: empty packet binding")
        for bound in packet['files']:
            path = PurePosixPath(bound['path'])
            safe = not path.is_absolute() and '..' not in path.parts and str(path).startswith('projects/chirality-piping/')
            require(safe, f"{packet['id']}: path outside project")
            if verify_git and safe:
                local = root / str(path)
                require(local.is_file(), f"{packet['id']}: missing packet file")
                if local.is_file():
                    require(hashlib.sha256(local.read_bytes()).hexdigest() == bound['sha256'], f"{packet['id']}: packet hash mismatch")
    for source in data['sources']:
        path = PurePosixPath(source['path'])
        safe = not path.is_absolute() and '..' not in path.parts and str(path).startswith('projects/chirality-piping/')
        require(safe, f"{source['id']}: path outside project")
        revision_ok = bool(re.fullmatch('[0-9a-f]{40}', source['revision']))
        require(revision_ok, f"{source['id']}: source revision not exact")
        require(bool(re.fullmatch('[0-9a-f]{64}', source['sha256'])), f"{source['id']}: invalid hash")
        if verify_git and safe and revision_ok:
            result = subprocess.run(['git', 'show', source['revision'] + ':' + str(path)], cwd=root, capture_output=True)
            require(result.returncode == 0, f"{source['id']}: missing pinned Git object/path")
            if result.returncode == 0:
                require(hashlib.sha256(result.stdout).hexdigest() == source['sha256'], f"{source['id']}: source hash mismatch")
    return errors


def self_check(data: dict, root: Path) -> dict:
    faults = {}
    def rejects(name, mutate):
        changed = deepcopy(data)
        mutate(changed)
        failures = check(changed, root, verify_git=False)
        faults[name] = bool(failures)
        if not failures:
            raise AssertionError(f'undetected inventory fault: {name}')
    rejects('missing_original_finding', lambda x: x['findings'].pop())
    rejects('duplicate_source', lambda x: x['sources'].append(deepcopy(x['sources'][0])))
    rejects('unknown_output_set', lambda x: x['profiles'][0]['required_output_sets'].append('absent'))
    rejects('zero_profile_denominator', lambda x: x['profiles'][0].update(capabilities=[]))
    rejects('invented_profile_pass', lambda x: x['profiles'][0].update(qualification='passed'))
    rejects('path_escape', lambda x: x['sources'][0].update(path='../elsewhere'))
    rejects('silent_finding_closure', lambda x: x['findings'][0].update(integration_state='closed_merged'))
    rejects('unknown_reverse_q_link', lambda x: x['vp_mapping'][0]['q_refs'].append('absent'))
    rejects('missing_unreferenced_vp', lambda x: x.update(vp_mapping=[r for r in x['vp_mapping'] if r['id'] != 'VP-SOURCES']))
    rejects('duplicate_first_cut', lambda x: x['first_profile_cuts'].append(deepcopy(x['first_profile_cuts'][0])))
    rejects('empty_contract_sources', lambda x: x['contracts'][0].update(source_refs=[]))
    rejects('dropped_capability_output', lambda x: x['profiles'][1].update(required_output_sets=['run_identity']))
    rejects('source_ready_without_admission', lambda x: x['work_packets'][0].update(ready_for_campaign_scoring=True))
    return faults


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--inventory', type=Path, default=DEFAULT)
    parser.add_argument('--self-check', action='store_true')
    args = parser.parse_args()
    root = Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip())
    data = json.loads(args.inventory.read_text())
    errors = check(data, root)
    faults = self_check(data, root) if args.self_check and not errors else {}
    print(json.dumps({'status': 'PASS' if not errors else 'FAIL', 'meaning': 'inventory consistency only; no solver/profile qualification',
                      'source_bindings': len(data['sources']), 'capabilities': len(data['capabilities']),
                      'original_findings': len(data['findings']), 'profiles': len(data['profiles']),
                      'fault_detection': faults, 'errors': errors}, indent=2))
    raise SystemExit(1 if errors else 0)


if __name__ == '__main__':
    main()
