# Maintained fixture promotion — narrow backcheck request

Root identified a pre-merge maintainability issue: new permanent tests depended
on the dated FIRST_STATIC_BINDINGS AgentRuns directory. This delta removes all
three such executable dependencies (structure, derivation and integration).

Exactly twelve consumed JSON files are promoted byte-for-byte into maintained
validation/qualification/fixtures/first_static, with per-file original origin and
hash in PROVENANCE.json. All 25 historical originals and the independently
selected section addendum remain unchanged. No numerical value, selector, input,
criterion or runtime admission is altered; no full run/review archive is copied.

The structural tests read these maintained fixtures directly. A small test-only
helper materializes a minimal temporary package for derivation/integration tests,
with explicit synthetic package/review metadata and patched metadata hashes. It
retains all original numeric/input/binding bytes. It does not claim actual review
or production admission and does not replace any runtime implementation.

Only affected checks were rerun: 37 PASS, with an audit guard rejecting access to
historical AgentRuns files. The guard's negative control worked; actual test
access attempts were zero. Source remained unchanged during checks. Complete
adapter review/76-method evidence and the four genuine comparison runs remain
valid for their recorded source basis; all runtime files and actual result bytes
are unchanged, so no solver rerun was performed or needed for this test-only move.

SOURCE_FREEZE_03 has 26 total maintained source/test/doc/fixture paths; 18 differ
from freeze02 (three test edits, one new helper, twelve copied JSON files and two
fixture documentation/provenance files). SOURCE_REPAIR_02_TO_03.patch is the exact
narrow delta. A full source snapshot and base patch are retained for recovery.

Please independently check minimality, exact source bytes, historical preservation,
absence of executable dated-file dependence, test-only metadata patches and the
unchanged runtime/result boundary. Write only FIXTURE_PROMOTION/INDEPENDENT_BACKCHECK,
return to Root and this manager. No source edits, Git, builds, native or real solves;
reuse recorded affected checks, with a bounded probe only for a concrete concern.
The previously supplied TASK role and software-code-review instructions apply.
