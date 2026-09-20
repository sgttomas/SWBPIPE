# B3A independent-review repair 1 — same implementation TASK

ROOT confirmed P2 on frozen80b9efb48: A clean→editB→Save pending→still-enabled
native Open dispatch→Open fails/finds nothing (generation unchanged, request increments)
→SaveB lands. ownsPersistence rejects validB solely for request mismatch. B remains
Edited and UndoA falsely clears despite persistedB. This is B3A marker correctness,
not permission to implement deferred B3B busy/integrity changes.

Continue same Astra/low implementation assignment, same original exact source scope,
no delegation/Git/native/browser action until manager reservation. Native worker is
finishing80b9 happy path; leave its built/running bundle untouched. Source-only repair
is permitted, but no uncoordinated rebuild and tests bind their actual source snapshot.
Retain original worker return/source snapshots/logs unchanged. New evidence filenames.

Before repair add meaningful failing App regression: SaveB pending, later missing or
failed Open, verified SaveB completion, B clean; UndoA dirty; RedoB clean. Preserve
model/history/content and existing integrity-response behavior in all paths. Distinguish
later request initiation from a later successful project adoption or verified write.
A later successful same-project save must remain the comparison basis when earlier
asynchronous verification completes; same-ID Open replacement is a new generation.
Canonical returned model/project/hash/envelope and supported-normalization checks remain.

Separate last-landed same-session saved-content observation from latest-request UI
adoption. Do not gate marker-only observation on request number when generation and
project remain current. Do not overwrite newer verified persisted baseline due to
out-of-order hash verification. Explain chosen response-observation/order evidence
and residual protocol limits rather than assuming request initiation proves a write.
Keep stricter current request/UIepoch guards for model adoption, metadata, history,
result and integrity behavior. No stale model adoption or busy menu repair.

Run focused fail-before/pass-after race tests and relevant old failure/supersession/
normalization tests, typecheck. Report precise code/hash/test changes and any required
extra contract scope. Manager owns exact-source connected browser/native delta after
freeze, ROOT independent backcheck and final sweep. No full suite repetition here.
