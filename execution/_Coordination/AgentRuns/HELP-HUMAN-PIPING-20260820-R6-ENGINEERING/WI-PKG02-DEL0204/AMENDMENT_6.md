# N1 integrated-review Amendment 6

Trigger: integrated review v6 after Amendment 5 / V31 fan-in.

Blocking finding: post-snapshot plugin-manifest schema mismatch construction
interpolates finite caller-controlled property names and raw instance text into
finding paths/messages without canonicalization or byte limits. Finite
adversarial or near-1-MiB keys can therefore enter result-envelope diagnostic
references despite successful bounded snapshotting.

Bounded remediation: canonicalize and byte-bound every schema-generated path
segment and sanitize/bound all mismatch instance text before findings are
composed. Add direct/composed finite adversarial-key and huge-key regressions,
including protected-marker precedence, canonical schema-valid references and
messages, and `runtime_dispatched=false`.

No schema, runtime, N2/N3, lifecycle, Git, receipt, or PR change. Existing
closeout remains untouched until complete proof and fresh full-diff review pass.

Implementation complete: schema mismatch path segments are canonical and byte
bounded; caller-derived mismatch instance text is sanitized and bounded. Direct
and composed finite adversarial/huge-key regressions preserve protected marker
precedence, canonical envelopes, and runtime non-dispatch. Complete N1 suite:
`314 passed in 0.85s`; containment and diff checks PASS. Fresh review required.

V32 verified Amendment 6 path/message bounds but found normalized schema-valid
plugin IDs could still bypass the 256-byte diagnostic reference fence. Exact
helper-reuse and regression remediation is frozen in
`TASK-AMENDMENT-6/REMEDIATION_V32.md`; fan-in remains deferred.

V32 remediation routes normalized plugin IDs through the existing canonical
256-byte safe-reference helper. Four direct/composed over-limit and near-1-MiB
ID regressions pass. Complete suite: `318 passed in 0.87s`; containment/diff
PASS; fresh V33 review required.

V33 matched all 22 hashes/line counts, reviewed the complete 8,470-line frozen
set and original-basis amended N1 diff, independently reran `318 passed`, and
returned `PASS` with zero findings. Amendment 6 is complete and valid for
fan-in.
