#!/usr/bin/env python3
"""R5/C6-U2: exact accepted DAG-010 Status repair, not a graph rebuild."""
import argparse
import csv
import hashlib
import io
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[8]
PROJECT = ROOT / 'projects/chirality-piping'
RUN = PROJECT / 'execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS'
OUT = RUN / 'R5/DEPENDENCY_MIRROR_REPAIRS.json'

def digest(data):
    return hashlib.sha256(data).hexdigest()

def main():
    args = argparse.ArgumentParser()
    args.add_argument('--check', action='store_true')
    check = args.parse_args().check
    dag = PROJECT / 'execution/_DAG/DAG-010'
    authority = {r['DependencyID']: r for r in csv.DictReader((dag / 'DependencyEdges.csv').open())}
    retired = {r['RetiredDependencyID'] for r in csv.DictReader((dag / 'DAG-010_DuplicateEdgeWorklist.csv').open())}
    assert len(retired) == 30
    assert all(authority[k]['Status'] == 'RETIRED' for k in retired)
    found, records = set(), []
    for path in sorted(PROJECT.glob('execution/PKG-*/1_Working/DEL-*/Dependencies.csv')):
        before = path.read_bytes()
        reader = csv.DictReader(io.StringIO(before.decode()))
        rows, fields = list(reader), reader.fieldnames
        hits = []
        for row in rows:
            key = row['DependencyID']
            if key not in retired:
                continue
            assert key not in found, key
            found.add(key)
            ref = authority[key]
            for field in ('FromDeliverableID', 'TargetDeliverableID', 'Direction', 'DependencyType'):
                assert row[field] == ref[field], (key, field)
            if check:
                assert row['Status'] == 'RETIRED', key
            else:
                assert row['Status'] == 'ACTIVE', key
                row['Status'] = 'RETIRED'
            hits.append(key)
        if hits and not check:
            # Preserve original line formatting, quoting and newlines. CSV parsing
            # proves cell equivalence; each replacement changes one Status field.
            lines = before.decode().splitlines(keepends=True)
            status_index = fields.index('Status')
            for i, line in enumerate(lines[1:], 1):
                row = next(csv.reader([line]))
                if row[fields.index('DependencyID')] not in hits:
                    continue
                original = list(row)
                assert row[status_index] == 'ACTIVE'
                # Status is immediately before Notes in this schema. Locate the
                # one matching comma-bounded ACTIVE token outside quoted fields.
                in_quotes, starts = False, [0]
                j = 0
                while j < len(line):
                    if line[j] == '"':
                        if in_quotes and j + 1 < len(line) and line[j + 1] == '"':
                            j += 2
                            continue
                        in_quotes = not in_quotes
                    elif line[j] == ',' and not in_quotes:
                        starts.append(j + 1)
                    j += 1
                a, b = starts[status_index], starts[status_index + 1] - 1
                assert line[a:b] == 'ACTIVE'
                lines[i] = line[:a] + 'RETIRED' + line[b:]
                changed = next(csv.reader([lines[i]]))
                original[status_index] = 'RETIRED'
                assert changed == original
            after = ''.join(lines).encode()
            path.write_bytes(after)
            records.append({'path': str(path.relative_to(ROOT)), 'dependency_ids': hits,
                            'before_sha256': digest(before), 'after_sha256': digest(after),
                            'changed_field': 'Status', 'before': 'ACTIVE', 'after': 'RETIRED',
                            'authority': 'execution/_DAG/DAG-010/APPROVAL_RECORD.md; DependencyEdges.csv',
                            'judgment': 'Agent0 C6/U2; accepted graph and all other fields unchanged'})
    assert found == retired, sorted(retired - found)
    if not check:
        OUT.write_text(json.dumps(records, indent=2) + '\n')
    else:
        for record in json.loads(OUT.read_text()):
            assert digest((ROOT / record['path']).read_bytes()) == record['after_sha256']
    print(f'PASS: {len(found)} retired statuses in 15 local mirrors; graph unchanged')

if __name__ == '__main__':
    main()
