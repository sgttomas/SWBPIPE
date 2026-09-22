#!/usr/bin/env python3
"""Exact population accounting for a bounded record-repair tranche."""
import argparse
import collections
import csv
import json
from pathlib import Path

RUN = Path(__file__).resolve().parents[2]
R5 = RUN / 'R5'
OUT = RUN / 'BACKCHECK/R6_2026-09-22'

def read(path):
    return [x for x in csv.DictReader(path.open()) if list(x.values())[0] != '#END']

def write(path, rows):
    with path.open('w', newline='') as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]), lineterminator='\n')
        w.writeheader()
        w.writerows(rows)

def build():
    corpus = read(RUN / 'R3/CORPUS_CLAIMS.csv')
    classes = {x['ClaimKey']: x for x in read(RUN / 'R3/CLASS_ASSIGNMENTS.csv')}
    packets = {x['ClaimKey']: x['Packet'] for x in read(RUN / 'R4/PACKET_CLAIMS.csv')}
    changed = {x['ClaimKey']: x for x in read(R5 / 'REPAIR_MANIFEST.csv')}
    direct = {x['ClaimKey']: x for x in read(R5 / 'TASKS/REPAIR_DESIGN/SOURCE_KEY_OUTCOMES.csv')}
    supplement = R5 / 'TASKS/REPAIR_DESIGN/STAGE4/SOURCE_KEY_OUTCOMES.csv'
    if supplement.exists() and (supplement.parent / 'APPLIED.json').exists():
        direct.update({x['ClaimKey']: x for x in read(supplement)})
    status = {x['source_key']: x for x in read(R5 / 'TASKS/STATUS_REPAIR/REMAINING_RESULTS.csv')}
    status_ops = read(R5 / 'TASKS/STATUS_REPAIR/operations.csv')
    sealed = {}
    for path in sorted((RUN / 'WAVES').glob('**/*_forward.csv')):
        if 'superseded' in str(path):
            continue
        for row in read(path):
            sealed[row['ClaimKey']] = row
    barriers = {
        'T4A-C01': 'Mixed injection scope-item/applicability text requires subject-specific reach check; edited metadata alone does not settle it.',
        'T4A-C03': 'Required schema pins handled in separate exact-field tranche; other mixed revision/amendment narratives retain decision and scope context.',
        'T4A-C04': 'Review-state assertion is separate from stale pin repair; no review acceptance manufactured.',
        'T4A-C05': 'Successor path/anchor must be verified for this exact claim; historical source identity is not blindly rewritten.',
        'T4B-C05': 'Original migration provenance, old checker commands and current operative references differ; preserved historical evidence is not rewritten as a new PASS.',
        'T4B-C06': 'ISSUED DEL-01-01 remains protected; no ordinary correction to its current/history surface.',
        'T4B-C08': 'Readiness/governed-artifact declaration needs exact owning record and current evidence; no broad completion claim.',
        'T5A-C01': 'Setup phrasing can also carry current scope/exclusions; subject review must preserve stable obligations before removal.',
        'T5A-C02': 'Implementation-to-declaration mismatch requires its exact authority and verification reach; code presence alone is insufficient.',
        'T5A-C03': 'Partial implementation remains explicit; current code does not authorize shrinking intended scope.',
        'T5A-C04': 'Runtime-applier ownership and schema-conformance subject remains governed; no new scope allocation or runtime exception.',
        'T5A-C08': 'Formal review/lifecycle premise retains exact human/review residual after collateral wording repair.',
        'T5B-C03': 'Historical count/result retained as dated evidence; any true review duty remains separate.',
        'T5B-C05': 'Still-TBD list mixes ruled and unruled subjects; exact governing decisions must cover each item before removal.',
        'T5B-C06': 'TBD text includes public interfaces, engineering, legal/reviewer authority and incomplete CP10 choices; no collective discharge.',
        'T5B-C08': 'Owning write-scope statement must match accepted deliverable responsibility; no ownership expansion by text.',
        'T5B-C10': 'Frozen contract/hash label and current semantic version require exact compatibility/authority treatment.',
        'T7-C11': 'Schema construction note requires per-field semantic agreement; no schema/code change inferred from wording.'}
    for row in status_ops:
        if row['operation_kind'] in ('date', 'declaration'):
            direct[row['source_key']] = {'Outcome': 'DIRECT_REPAIRED', 'Posture': 'a', 'Reason': row['rationale']}
    results = []
    for row in corpus:
        key = row['ClaimKey']
        original = classes.get(key, {})
        if key in status:
            outcome, reason = 'REPAIRED_RECORD_RESIDUAL_PRESERVED', 'False Remaining premise corrected; substantive residual/hold retained in current STATUS and complete census.'
        elif key in direct:
            outcome, reason = direct[key]['Outcome'], direct[key]['Reason']
            # A repaired parent/date/prose does not settle a different class defect.
            cls = original.get('ClassID', '')
            if key.endswith(':STATUS') and cls not in ('T4B-C06', ''):
                outcome = 'PARTIAL_REPAIR'
            if cls == 'T4B-C01' and ':SOW' in key:
                outcome = 'PARTIAL_REPAIR'
                reason += ' Exact legacy identifiers/compatibility remain held; prose rename alone does not settle whole-surface identity.'
        elif key in changed:
            outcome = 'CHANGED_BINDING_RESIDUAL_RETAINED' if row['Divergent'] == 'YES' else 'PRESERVED_CLAIM_REBOUND'
            reason = 'Collateral unit/parent binding changed; no semantic closure inferred from an enclosing edit.'
        elif row['Divergent'] == 'YES':
            outcome = 'NO_REPAIR_REQUIRED' if original.get('Route') == 'NO_ACTION' else 'HELD_UNCHANGED'
            reason = 'Original disposition preserved; packet application refines authority/route but does not assert unexecuted repair.'
        else:
            outcome, reason = 'UNCHANGED_ORIGINAL_DISPOSITION', 'Unchanged source; discovery remains the evidence basis and is not rerun or recoded.'
        if outcome in ('HELD_UNCHANGED', 'PARTIAL_REPAIR', 'CHANGED_BINDING_RESIDUAL_RETAINED'):
            reason += ' ' + barriers.get(original.get('ClassID', ''), 'Exact packet/handoff authority or production obligation retained; current application does not activate unexecuted work.')
        source = sealed.get(key, {})
        results.append({'ClaimKey': key, 'DeliverableID': row['DeliverableID'], 'PackageID': row['PackageID'],
                        'OriginalDisposition': row['Disposition'], 'OriginalDivergent': row['Divergent'],
                        'ClassID': original.get('ClassID', ''), 'OriginalRoute': original.get('Route', ''),
                        'Packet': packets.get(key, ''), 'R5Outcome': outcome,
                        'ChangedBinding': 'YES' if key in changed else 'NO',
                        'ClaimSubject': source.get('ClaimSummary', ''),
                        'OriginalRepairProposal': source.get('RemainingWork', ''),
                        'OriginalNormativeBasis': source.get('NormativeSource', ''),
                        'Reason': reason, 'NextHolder': 'Existing production/review/authority owner per R4 handoff' if outcome in ('HELD_UNCHANGED', 'PARTIAL_REPAIR', 'CHANGED_BINDING_RESIDUAL_RETAINED') else 'R6 record verification; existing substantive holds unaffected'})
    assert len(results) == 9889 and len({x['ClaimKey'] for x in results}) == 9889
    lookup = {x['ClaimKey']: x for x in results}
    h4 = []
    mirror_dels = {Path(x['path']).parent.name[:9] for x in json.loads((R5 / 'DEPENDENCY_MIRROR_REPAIRS.json').read_text())}
    for row in read(RUN / 'R4/R5_TRANCHE_PROPOSAL/R5_REPAIR_ROWS.csv'):
        key = row['Key']
        if row['KeyKind'] == 'CLAIM':
            outcome, reason = lookup[key]['R5Outcome'], lookup[key]['Reason']
        elif row['ClassID'] == 'T11 D-02' or ('D-02' in row['ClassID'] and row['DeliverableID'] in mirror_dels):
            outcome, reason = 'REPAIRED_STATUS_MIRROR', 'Exact DAG-010 dependency Status cells repaired; all other fields and graph unchanged.'
        elif key == 'REMAINING_CENSUS':
            outcome, reason = 'BACKCHECKED_CENSUS', 'Current census completed; individual residuals remain individually accountable.'
        else:
            outcome, reason = 'HELD_OR_PARTIAL_VIEW', 'View is checked against actual changed-key set; no full completion claimed for unexecuted members.'
        h4.append({**row, 'R5Outcome': outcome, 'Reason': reason})
    assert len(h4) == 2234 and len({(x['Key'], x['DeliverableID']) for x in h4}) == 2234
    capabilities = []
    disposition = {x['CapabilityID']: x for x in read(RUN / 'R3/CAPABILITY_DISPOSITIONS.csv')}
    for row in read(RUN / 'IMPLEMENTATION_SURFACES.csv'):
        key = row['CapabilityID']
        d = disposition.get(key, {})
        capabilities.append({'CapabilityID': key, 'OriginalClassification': d.get('Classification', 'Original coverage retained'),
                             'ProposedOwner': d.get('ProposedOwner', ''), 'R5Disposition': 'NO_SCOPE_OR_IMPLEMENTATION_MUTATION',
                             'NextStep': 'Existing keyed ownership retained; H1/B packet follow-through for unresolved ownership; proposed owners must independently assess routing gaps before assignment.'})
    assert len(capabilities) == 598 and len({x['CapabilityID'] for x in capabilities}) == 598
    summary = {'corpus_claims': 9889, 'original_divergent': sum(x['OriginalDivergent'] == 'YES' for x in results),
               'corpus_outcomes': dict(sorted(collections.Counter(x['R5Outcome'] for x in results).items())),
               'h4_rows': 2234, 'h4_outcomes': dict(sorted(collections.Counter(x['R5Outcome'] for x in h4).items())),
               'capabilities': 598, 'closure': 'BOUNDED_RECORD_REPAIR; substantive authority/production residuals preserved'}
    return results, h4, capabilities, summary

def main():
    p = argparse.ArgumentParser(); p.add_argument('--check', action='store_true'); args = p.parse_args()
    results, h4, capabilities, summary = build()
    products = [(OUT / 'CLAIM_DISPOSITIONS.csv', results), (OUT / 'H4_ACCOUNTING.csv', h4), (OUT / 'CAPABILITY_ACCOUNTING.csv', capabilities)]
    if args.check:
        for path, expected in products:
            assert read(path) == expected, str(path)
        assert json.loads((OUT / 'ACCOUNTING_SUMMARY.json').read_text()) == summary
    else:
        for path, rows in products:
            write(path, rows)
        (OUT / 'ACCOUNTING_SUMMARY.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary, sort_keys=True))

if __name__ == '__main__':
    main()
