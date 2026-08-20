# TASK-PKG02-DEL0204-REVIEW-V28 Return

Verdict: `FAIL` — three blocking findings; not valid for fan-in.

1. Raw malformed-input marker fallback calls `.get` on exact caller dictionaries
   outside containment. A non-string key with colliding hash and raising
   equality can escape before provenance/metadata marker recovery. Evidence:
   `plugin_verification.py:269-312,418-431` and
   `adapter_framework.py:243-254`.
2. Unit quarantine fallback scans `range(len(raw unit_evidence))` without a
   deterministic width/node guard after preflight node-limit failure. Evidence:
   `plugin_verification.py:409-451,530-535`.
3. Malformed-manifest fallback calls `_safe_provenance_snapshot`, applying
   looser adapter bounds beyond the manifest's 10,000-node/1 MiB limits;
   over-limit raw `plugin_id` can also enter diagnostics. Evidence:
   `plugin_verification.py:269-277` and
   `adapter_framework.py:120-122,247-259`.

All 17 V28 hashes/line counts, the full 6,964-line frozen surface, amended diff
from the original basis, explicit scope, and diff integrity passed. Supplied
node evidence was `253 passed`. Fresh bounded remediation/review is required.
