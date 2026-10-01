#!/usr/bin/env python3
"""Narrow TSV prevalidation around the unchanged frozen exact comparator.

python3 validate_compare.py PACKET_DIR RELEASED_TSV REPORT_JSON
Exit 0: syntax and compared predicates pass; 1: numerical failure;
2: invalid input, missing frozen evidence, or comparator exception.
No files are written on validation failure.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

sys.dontwrite_bytecode = True

FROZEN_ORACLE = 'fa8ea6f303148d9babb5d9fe6c53f64377b13cb130d03d076d4cec7d3f4c15e0'
FROZEN_TRUTH = '1772d703e032a71587b922a2f6d825718f777c9dae78d1041fbd874210ee87ea'
KINDS = {'Translation', 'Rotation', 'Force', 'Moment'}
STATUSES = {'Selected', 'Refused', 'Unresolved', 'SourceRefused'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def bits(text, nonnegative=False):
    require(bool(re.fullmatch(r'[0-9a-f]{16}', text)), 'bits must be 16 lowercase hexadecimal digits')
    b = int(text, 16)
    require((b >> 52) & 2047 != 2047, 'bits must encode finite binary64')
    require(b != 0x8000000000000000, 'zero must use canonical positive-zero bits')
    require(not nonnegative or b < 1 << 63, 'scale, floor, or bound must be nonnegative')
    return b


def validate(text):
    records = [line.split('\t') for line in text.splitlines()]
    def tagged(tag):
        return [r for r in records if r[0] == tag]
    def singleton(tag, fields):
        rows = tagged(tag)
        require(len(rows) == 1 and len(rows[0]) == fields, f'{tag}: exactly one well-formed record required')
        return rows[0]
    require(singleton('FORMAT', 2)[1] == 'a1-public-tsv-v1', 'unsupported FORMAT')
    case = singleton('CASE', 2)[1]
    require(bool(re.fullmatch(r'(B(0[1-9]|1[0-6])|C(1[7-9]|2[0-4]))', case)), 'unknown CASE')
    status = singleton('STATUS', 2)[1]
    require(status in STATUSES, 'unknown STATUS')
    if status != 'Selected':
        require(not any(tagged(t) for t in ('SELECTED','ROW','SCALE','FLOOR')),
                'unselected status must not carry selection or publication records')
        return {'case': case, 'status': status, 'row_count': 0}

    selected = singleton('SELECTED', 3)
    require(selected[1:] in [['128','256'],['256','512'],['512','1024']],
            'SELECTED must contain a supported candidate and exactly doubled verification precision')
    p = int(selected[1])
    # Check duplicate keys before building any map: dict would hide duplicates.
    floors = tagged('FLOOR')
    require(all(len(r) == 3 for r in floors), 'malformed FLOOR')
    require(len({r[1] for r in floors}) == len(floors), 'duplicate FLOOR kind')
    if p == 512:
        require(len(floors) == 2 and {r[1] for r in floors} == {'Force','Moment'},
                'precision 512 requires exactly Force and Moment FLOOR entries')
    else:
        require(not floors, 'FLOOR is only allowed at precision 512')
    for r in floors:
        bits(r[2], nonnegative=True)

    scales = tagged('SCALE')
    require(len(scales) == 4 and all(len(r) == 3 for r in scales), 'exactly four well-formed SCALE records required')
    require({r[1] for r in scales} == KINDS, 'SCALE requires each kind exactly once')
    for r in scales:
        bits(r[2], nonnegative=True)

    rows = tagged('ROW')
    require(bool(rows) and all(len(r) == 7 for r in rows), 'malformed or missing ROW')
    require(len({r[1] for r in rows}) == len(rows), 'duplicate ROW key')
    for _, key, kind, outcome, raw_bits, cls, bound in rows:
        require(kind in KINDS, 'unknown ROW kind')
        require(outcome in {'Value','Underflow','Overflow'}, 'unknown ROW outcome')
        if outcome == 'Value':
            bits(raw_bits)
            require(cls in {'InputDerived','RelativeVerified','AbsoluteVerified'}, 'value row has invalid class')
            if cls == 'AbsoluteVerified':
                bits(bound, nonnegative=True)
            else:
                require(bound == 'none', 'non-absolute row must have no bound')
        else:
            require(raw_bits == 'none' and cls == 'Unpublishable' and bound == 'none',
                    'underflow/overflow row must have no bits, Unpublishable class, and no bound')
    return {'case':case, 'status':status, 'precision':p, 'row_count':len(rows)}


def run(packet, output, report):
    validation = validate(output.read_text())
    for name, expected in [('exact_oracle.py', FROZEN_ORACLE), ('TRUTH.json', FROZEN_TRUTH)]:
        require(hashlib.sha256((packet/name).read_bytes()).hexdigest() == expected,
                f'frozen {name} hash mismatch')
    spec = importlib.util.spec_from_file_location('frozen_a1_oracle',packet/'exact_oracle.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.self_check()
    module.compare(packet,output,report)
    result = json.loads(report.read_text())
    result['prevalidation'] = validation
    result['prevalidator_sha256'] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    report.write_text(json.dumps(result,indent=2)+'\n')
    return 1 if result['failures'] else 0


if __name__ == '__main__':
    try:
        require(len(sys.argv) == 4, 'usage: validate_compare.py PACKET_DIR RELEASED_TSV REPORT_JSON')
        status = run(*(Path(p) for p in sys.argv[1:]))
    except (ValueError, AssertionError, KeyError, StopIteration, OSError) as exc:
        print(json.dumps({'status':'invalid_input', 'error':str(exc)}), file=sys.stderr)
        status = 2
    raise SystemExit(status)
