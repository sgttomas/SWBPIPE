#!/usr/bin/env python3
"""Probe (not a reading): run the accepted Python reader's _g5_stages check list on a
receipt's product attempt, recording every failed check line instead of stopping."""
import json, sys, traceback
sys.path.insert(0, sys.argv[1])
from core.analysis_runs import retained_precision as rp
for path in sys.argv[2:]:
    d = json.load(open(path)); b = d['source']['retained_precision']['body']
    failed = []
    def fail(ok):
        if not ok: failed.append(traceback.extract_stack(limit=2)[0].lineno)
    rp._g5_stages(b['product_attempts'][0], b['cases'][0], fail)
    print(path.rsplit('/',1)[-1], 'g5_stages failed lines:', failed)
