#!/usr/bin/env python3
"""Build a bounded R6 derivative; independently check the changed-key manifest.

Manifest selection uses old source spans intersecting line edits. Verification
ignores those spans and re-extracts complete old/new unit dictionaries, comparing
their contents. Parent envelopes explicitly bind old worker-minted .sNN keys.
"""
import argparse
import collections
import csv
import difflib
import hashlib
import importlib.util
import json
import re
import subprocess
import sys
from pathlib import Path

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[8]
PROJECT = ROOT / 'projects/chirality-piping'
RUN = PROJECT / 'execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS'
R5 = RUN / 'R5'
OUT = RUN / 'BACKCHECK/R6_2026-09-22'
BASE = '379df923927d157be3ebb51d8a1dcf783d970112'
FREEZE = '00115c71931bcae79909602d653740d3bb72dfa1'
SURFACES = {'ScopeOfWork.md': 'SOW', 'ArchitectureBasis.md': 'AB', '_CONTEXT.md': 'CONTEXT', '_STATUS.md': 'STATUS', 'MEMORY.md': 'MEMORY'}
BESPOKE = {'Capability_Comparison.csv', 'Vocabulary_Coverage.csv', 'Palette_Operation_Routing.md', 'Palette_Organization_Contract.md'}
spec = importlib.util.spec_from_file_location('piping_extractor_v2', RUN / 'tools/extract_claims_v2.py')
extractor = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = extractor
spec.loader.exec_module(extractor)

def sha(value):
    return hashlib.sha256(value.encode() if isinstance(value, str) else value).hexdigest()

def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT), *args])

def read_csv(path):
    return [r for r in csv.DictReader(path.open()) if list(r.values())[0] != '#END']

def write_csv(path, rows, fields):
    with path.open('w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=fields, lineterminator='\n')
        writer.writeheader()
        writer.writerows(rows)

def units(path, text):
    did = re.search(r'/(DEL-\d\d-\d\d)[^/]*/', path).group(1)
    surface = SURFACES[Path(path).name]
    if surface == 'MEMORY':
        values = [extractor.Unit(did + ':MEMORY', 'SURFACE', '', 1, text.count('\n') + 1, text, 'MEMORY')]
    elif surface == 'STATUS':
        values = extractor.status_units(did, text)
    else:
        values = extractor.markdown_units(did, surface, text, surface == 'SOW')
    return {u.key: u for u in values}

def source_files():
    paths = git('diff', '--name-only', BASE, '--', 'projects/chirality-piping/execution/PKG-*').decode().splitlines()
    return [p for p in paths if Path(p).name in SURFACES or Path(p).name in BESPOKE or Path(p).name == 'Dependencies.csv']

def source_state():
    files = []
    old_units, new_units, key_paths = {}, {}, {}
    spans = set()
    bespoke_dirs = set()
    for path in source_files():
        assert '/DEL-01-01_' not in path, 'ISSUED baseline changed'
        before_bytes = git('show', f'{BASE}:{path}')
        after_bytes = (ROOT / path).read_bytes()
        files.append({'Path': path, 'BeforeSHA256': sha(before_bytes), 'AfterSHA256': sha(after_bytes), 'BaseCommit': BASE})
        if Path(path).name in BESPOKE:
            bespoke_dirs.add(str(Path(path).parent))
        if Path(path).name not in SURFACES:
            continue
        before, after = before_bytes.decode(), after_bytes.decode()
        old, new = units(path, before), units(path, after)
        old_units.update(old)
        new_units.update(new)
        key_paths.update({k: path for k in old.keys() | new.keys()})
        edits = [x for x in difflib.SequenceMatcher(None, before.split('\n'), after.split('\n'), autojunk=False).get_opcodes() if x[0] != 'equal']
        for key, unit in old.items():
            start, end = unit.start - 1, unit.end
            hit = any((a < end and b > start) if a != b else (start <= a < end) for _, a, b, _, _ in edits)
            if hit and (key not in new or unit.text != new[key].text):
                spans.add(key)
        spans.update(new.keys() - old.keys())
    for directory in sorted(bespoke_dirs):
        did = Path(directory).name[:9]
        names = {p.name for p in (ROOT / directory).iterdir() if p.is_file() and p.suffix in ('.md', '.csv') and p.name not in extractor.STANDARD}
        before = {name: git('show', f'{BASE}:{directory}/{name}').decode() for name in sorted(names)}
        after = {name: (ROOT / directory / name).read_text() for name in sorted(names)}
        old = {u.key: u for u in extractor.bespoke_surfaces(did, before)}
        new = {u.key: u for u in extractor.bespoke_surfaces(did, after)}
        old_units.update(old); new_units.update(new)
        for key in old.keys() | new.keys():
            unit = new.get(key) or old[key]
            key_paths[key] = ';'.join(directory + '/' + n for n in unit.source.split(';'))
        # CSV ROWS units combine the two keyed tables. Selection is by changed
        # row identity/content, not shifted line numbers; exact physical edits
        # and their inverse are separately checked by the stage4 manifest.
        spans.update(k for k in old.keys() | new.keys() if k not in old or k not in new or old[k].text != new[k].text)
    return files, old_units, new_units, key_paths, spans

def add_envelopes(keys, old, new, paths, corpus):
    for row in corpus:
        key = row['ClaimKey']
        if key in old or key in new or not re.search(r'\.s\d\d$', key):
            continue
        parent = re.sub(r'\.s\d\d$', '', key)
        if parent in keys:
            keys.add(key)
            paths[key] = paths[parent]
    return keys

def row_for(key, old, new, paths):
    envelope = key not in old and key not in new
    ref = re.sub(r'\.s\d\d$', '', key) if envelope else key
    before, after = old.get(ref), new.get(ref)
    ops = operations()
    related = lambda a, b: a == b or any(a.startswith(b + d) or b.startswith(a + d) for d in ('#', '/', '.s', '.r'))
    matching = [op for op in ops if op['path'] in paths[key].split(';') and (any(related(key, k) for k in op['keys']) or (before and op.get('base_coordinates') and op['start'] <= before.end and op['end'] >= before.start))]
    postures = sorted({op['posture'] for op in matching})
    return {'ClaimKey': key, 'SourcePath': paths[key], 'Binding': 'PARENT_ENVELOPE' if envelope else 'EXACT_UNIT',
            'BoundReference': ref, 'BeforeSHA256': sha(before.text) if before else '', 'AfterSHA256': sha(after.text) if after else '',
            'CurrentPresence': 'PRESENT' if after else 'REMOVED', 'BeforeLine': before.start if before else '', 'AfterLine': after.start if after else '',
            'RepairPosture': '+'.join(postures) or 'a',
            'OperationRefs': ';'.join(op['id'] for op in matching),
            'PostureReason': ' | '.join(sorted({op['reason'] for op in matching})) or 'New current record/evidence unit; no new product requirement',
            'Authority': 'D-73_EXECUTION_ADDENDUM_2026-09-22; R5/RUN_RIDER; root AGENT0_DISPOSITIONS',
            'ClosureLimit': 'Source binding only; closure separately accounted in CLAIM_DISPOSITIONS.csv'}

_ops = None
def operations():
    global _ops
    if _ops is not None:
        return _ops
    result = []
    for name in ('PHYSICAL_EDITS.json', 'PHYSICAL_EDITS_STAGE2.json', 'PHYSICAL_EDITS_STAGE3.json', 'STAGE4/PROPOSED_PHYSICAL_EDITS_STAGE4_V2.json'):
        path = R5 / 'TASKS/REPAIR_DESIGN' / name
        if not path.exists():
            continue
        data = json.loads(path.read_text())
        if name.startswith('STAGE4/') and not (R5 / 'TASKS/REPAIR_DESIGN/STAGE4/APPLIED.json').exists():
            continue
        for file in data['files']:
            for i, edit in enumerate(file['edits'], 1):
                result.append({'id': name + ':' + file['path'] + '#REPAIR-' + str(i), 'path': file['path'],
                               'keys': edit['keys'], 'start': edit['start'], 'end': edit['end'],
                               'base_coordinates': name == 'PHYSICAL_EDITS.json',
                               'posture': edit['posture'], 'reason': edit['reason']})
    path = R5 / 'TASKS/STATUS_REPAIR/operations.csv'
    if path.exists():
        for edit in read_csv(path):
            result.append({'id': edit['operation_id'], 'path': edit['path'], 'keys': [edit['source_key']],
                           'start': int(edit['before_start_line']), 'end': int(edit['before_end_line']),
                           'base_coordinates': True,
                           'posture': 'b' if edit['operation_kind'] == 'remaining' else 'a',
                           'reason': edit['rationale'] + '; current declared state/history is verification-bound (a); stale mechanism choice lifted (b)'})
    _ops = result
    return result

def expected_from_full_extraction(old, new, paths, corpus):
    changed = {k for k in old.keys() | new.keys() if k not in old or k not in new or old[k].text != new[k].text}
    return add_envelopes(changed, old, new, paths, corpus)

def verify(manifest, old, new, paths, corpus):
    expected = expected_from_full_extraction(old, new, paths, corpus)
    actual = collections.Counter(r['ClaimKey'] for r in manifest)
    assert actual == collections.Counter(expected), {'missing': sorted(expected - actual.keys())[:20], 'extra_or_duplicate': [k for k, n in actual.items() if n != 1 or k not in expected][:20]}
    for row in manifest:
        fresh = row_for(row['ClaimKey'], old, new, paths)
        for field in ('SourcePath', 'Binding', 'BoundReference', 'BeforeSHA256', 'AfterSHA256', 'CurrentPresence'):
            assert row[field] == fresh[field], (row['ClaimKey'], field)
    return len(expected)

def census():
    rows = []
    for item in read_csv(RUN / 'DELIVERABLE_INVENTORY.csv'):
        p = ROOT / item['Folder'] / '_STATUS.md'
        text = p.read_text()
        match = re.search(r'^\*\*Status:\*\*\s*(\S+)|^Status:\s*(\S+)', text, re.M)
        lifecycle = next((x for x in match.groups() if x), item['Lifecycle']) if match else item['Lifecycle']
        # Lifecycle must retain its original record; no promotion occurs here.
        before = git('show', f'{BASE}:{p.relative_to(ROOT)}').decode()
        old_state = re.search(r'^.*(?:\*\*Status:?\*\*|Current State|Lifecycle State).*$|^Status:.*$', before, re.M)
        new_state = re.search(r'^.*(?:\*\*Status:?\*\*|Current State|Lifecycle State).*$|^Status:.*$', text, re.M)
        assert (old_state.group(0) if old_state else '') == (new_state.group(0) if new_state else '')
        section = re.search(r'^## Remaining\s*\n(.*?)(?=^## |\Z)', text, re.M | re.S)
        bullets = re.findall(r'^- .*(?:\n(?!- |## ).+)*', section.group(1), re.M) if section else []
        if not bullets:
            bullets = ['NONE — no item recorded; not evidence of completion']
        for i, body in enumerate(bullets, 1):
            none = body.startswith('NONE') or bool(re.fullmatch(r'-\s+None\.?\s*', body, re.I))
            rows.append({'DeliverableID': item['DeliverableID'], 'PackageID': item['PackageID'], 'Lifecycle': item['Lifecycle'], 'Item': 'NONE' if none else str(i), 'RecordedText': body, 'EvidenceMeaning': 'Residual record only; C9 work graph governs selection; no closure inferred', 'SourcePath': str(p.relative_to(ROOT)), 'SourceSHA256': sha(p.read_bytes())})
    assert {r['DeliverableID'] for r in rows} == {x['DeliverableID'] for x in read_csv(RUN / 'DELIVERABLE_INVENTORY.csv')}
    return rows

def verify_census(saved, current):
    identities = [(row['DeliverableID'], row['Item']) for row in saved]
    assert len(identities) == len(set(identities)), 'Duplicate Remaining census identity'
    assert saved == current, 'Saved Remaining census differs from current source extraction'

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--build', action='store_true')
    args = parser.parse_args()
    corpus = read_csv(RUN / 'R3/CORPUS_CLAIMS.csv')
    files, old, new, paths, spans = source_state()
    assert len(corpus) == 9889
    if args.build:
        OUT.mkdir(parents=True, exist_ok=True)
        keys = add_envelopes(spans, old, new, paths, corpus)
        manifest = [row_for(k, old, new, paths) for k in sorted(keys)]
        verify(manifest, old, new, paths, corpus)
        write_csv(R5 / 'REPAIR_MANIFEST.csv', manifest, list(manifest[0]))
        write_csv(OUT / 'SOURCE_BINDINGS.csv', files, list(files[0]))
        remaining = census()
        write_csv(OUT / 'REMAINING_WORK_CENSUS.csv', remaining, list(remaining[0]))
    manifest = read_csv(R5 / 'REPAIR_MANIFEST.csv')
    count = verify(manifest, old, new, paths, corpus)
    bound = read_csv(OUT / 'SOURCE_BINDINGS.csv')
    assert bound == files, 'File bindings differ from current candidate'
    negatives = {}
    for name, bad in [('missing-key', manifest[1:]), ('duplicate-key', manifest + manifest[:1])]:
        try:
            verify(bad, old, new, paths, corpus)
        except AssertionError:
            negatives[name] = 'REJECTED'
        else:
            raise AssertionError(f'Negative control {name} failed to fail')
    current_census = census()
    saved_census = read_csv(OUT / 'REMAINING_WORK_CENSUS.csv')
    verify_census(saved_census, current_census)
    for name, bad in [('missing-census-row', saved_census[1:]), ('duplicate-census-row', saved_census + saved_census[:1])]:
        try:
            verify_census(bad, current_census)
        except AssertionError:
            negatives[name] = 'REJECTED'
        else:
            raise AssertionError(f'Negative control {name} failed to fail')
    # Backcheck rows are freshly extracted; they are not copies of the manifest.
    fresh = [row_for(k, old, new, paths) for k in sorted(expected_from_full_extraction(old, new, paths, corpus))]
    if args.build:
        write_csv(OUT / 'CHANGED_CLAIM_REEXTRACTION.csv', fresh, list(fresh[0]))
    else:
        assert read_csv(OUT / 'CHANGED_CLAIM_REEXTRACTION.csv') == [{k: str(v) for k, v in r.items()} for r in fresh]
    result = {'verdict': 'PASS', 'base_commit': BASE, 'discovery_commit': FREEZE, 'changed_references': count,
              'changed_files': len(files), 'minted_subclaim_parent_envelopes': sum(r['Binding'] == 'PARENT_ENVELOPE' for r in manifest),
              'remaining_deliverables': len({r['DeliverableID'] for r in current_census}), 'remaining_census_rows': len(current_census), 'negative_controls': negatives,
              'scope': 'exact changed-reference and file binding; no semantic acceptance inferred'}
    if args.build:
        (OUT / 'CHECK_RESULT.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, sort_keys=True))

if __name__ == '__main__':
    main()
