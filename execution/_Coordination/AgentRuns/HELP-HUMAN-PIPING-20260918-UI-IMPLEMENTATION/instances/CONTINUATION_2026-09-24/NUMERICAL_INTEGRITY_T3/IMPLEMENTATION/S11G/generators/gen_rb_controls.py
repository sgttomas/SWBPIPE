#!/usr/bin/env python3
"""T3 S11-G (I5): generate the R-b' control requests from the frozen references.

Usage (standard library only; deterministic):
    python3 gen_rb_controls.py <references.json> <out.json>

Input: T3/REFERENCES/references.json (sha256 7b176dbb..., recorded in the output), R1's frozen cases.
The product requests are authored exactly as S11-F's generator authors the RF-CANCEL requests
(IMPLEMENTATION/S11F/generators/gen_rf_cancel_cases.py `author`, imported, not copied): each intended
decimal input rounded once to binary64, in the shape P1's detection generator used.

Cases: RF-LARGE-CONT-n00100-AX and RF-LARGE-CONT-n00100-ROT, the realistic-scale continuous beams on
which R-b (without the floor clause) falsely demotes and R-b' is silent (S11G_GUARD.md section 6.4). Only
the AX case carries a behavioural pin (T12); ROT is carried for the same observation.
"""
import hashlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..', '..', 'S11F', 'generators'))
import gen_rf_cancel_cases as g  # noqa: E402

CASES = ('RF-LARGE-CONT-n00100-AX', 'RF-LARGE-CONT-n00100-ROT')


def main():
    refs_path, out_path = sys.argv[1], sys.argv[2]
    refs = json.load(open(refs_path))
    out = {
        'generator': 'T3/IMPLEMENTATION/S11G/generators/gen_rb_controls.py',
        'inputs': {'references_json_sha256': hashlib.sha256(open(refs_path, 'rb').read()).hexdigest()},
        'cases': [],
    }
    for cid in CASES:
        request, _rigid_of, captured_refused = g.author(cid, refs['cases'][cid]['model'])
        request['model']['project']['id'] = 'invented:t3-s11g:' + cid
        out['cases'].append({'id': cid, 'captured_refused_at_capture': captured_refused, 'request': request})
    with open(out_path, 'w') as f:
        json.dump(out, f, sort_keys=True, separators=(',', ':'))
        f.write('\n')


if __name__ == '__main__':
    main()
