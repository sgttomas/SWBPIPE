# Interim portability correction check

ROOT ran the unchanged GEN-8 selection on clean detached head
9ef9508dea8205297631299b3c384a3cd5185bef in the existing sweep checkout.
It passed:1 passed,10 deselected; the checkout remained clean. Exact argv,
UTC times and stream hashes are in _run_records/RESULT.json.

The earlier hosted failure remains at _run_records/ci_interim_4972 in the
run root. REVIEW_EVIDENCE_RELOCATION_2026-10-01.json maps the thirteen whole
sealed review packets to their structural evidence directory; all payloads
and manifests retain their original bytes. No source, oracle, harness or
policy change was made. This verifies the placement correction at this head;
it is not the later exact-final-head merge gate.
