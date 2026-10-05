# RV68 custody and conversion backcheck

**RV68-2 CLOSED. RV68-3 CLOSED. RV68-1 remains CLOSED.** The verdict-buffer
overlap is removed and retained successful-view mask capacities are recorded.
Final-head coverage, the completed local account and remaining failure/provenance
notes still await the sealed handoff. No numerical blocker was found in this delta.

Subject: CUSTODY_C4_BOUNDARY.json SHA-256
`e5feb41a4ced28bf593a5c67b68ec3b850484e93ee7d93b8ddec42e5c6585223`,
patch SHA-256
`b4eb24e89b65ca10e7d586be953d605ec70353bed8e6cd3e9e247cb8fb9b99eb`.
All five files were reconstructed from `430bc4f798` and matched by bytes/hash.
The complete delta from FIRST_AMENDED FK plus REENTRY PP was inspected.

## Closed ownership path

Production `PreparedCase::prepare_observed` now runs the actual ordinary observer
and immediately passes its returned envelope to private `prepare_owned_case`.
That state owns the envelope, and `project_candidate` takes no envelope argument.
Only immutable ordinary/capture views escape. Success/refusal wrappers keep the
prepared state private and preserve the sticky proof-attempt guard. The previous
arbitrary-envelope constructor survives only under cfg(test), for focused tests.
This closed transition supplies the same-owner guarantee without a redundant Arc
for the envelope. It is materially different from merely adding header checks.

The substitution control modifies every challenged header on a detached test
copy, then checks the committed candidate retains the actual owned original
headers and has no source-block selection. Production has no argument/mutator
that could pass that detached copy into projection. Ordinary headers are also
positively revalidated before source preparation.

## Conversion and local work

`project_hull` reserves a per-row outcome vector, enters a separate conversion
count, checks joined accounting, performs the conversion and immediately stores
its exact Binary64Outcome. No fallible action separates the actual conversion
from its retained result. Normal, Subnormal, Underflow and Overflow are preserved;
only the candidate value canonicalizes zero. Final error predicates are unchanged.
Both success/failure work moves preserve the records, count and capacity.
Tests directly exercise all four outcomes and an accounting failure after entered
fixed-precision arithmetic. Maximum/support/ancillary formations remain separate.

`row_scales` now reuses its existing verdict capacity on the normal begin/final
path. A growth path drops the earlier vector before allocating its replacement;
the pointer/capacity control confirms repeated same-shape reuse. Source residual
records `view.data_capacity()` before seed validation or lane execution, retaining
the actual capacity through later failure and the consuming readout move.

Accessor/bijection work is entered before searches/copies. Builder access work
moves into completion, or `abandon_values(builder.abandon())` if PP maxima fail.
Closed row matching counts identity-byte checks. Maximum row/evidence lookups and
Number reads gain entered adapter work. Fixed support arrays and G5a iterators
from the prior repair remain. These changes preserve existing numerical order.

## Evidence and limits

fk_custody_conversion_05 and pp_custody_debug_04 have the same producing source
digest `0638c04880366fb9777f224d2c1a07225361c20138b9bf340e677d573df5b39e`.
All applicable frozen source hashes match their command records. Stable-source,
exit0/reaped records and raw logs show FK 24/24 and PP 28/28. Tests include seed
foreign/stale/missing/inverted/wrong-law refusal, exact prescription preservation,
support identity/order/zero/subnormal/overflow and failed-prefix cases, post-freeze
row mutation, numerical refusal, re-entry and precommit accounting fallback.

These are inspected author runs, not an RV68 Cargo execution. Concrete source,
command and log hashes are in scratch/rv68_prepared_producer/CUSTODY_AUDIT.json.
All six successful frozen-row events in the PP log (five sparse, one dense) also
exactly match the independently checked FIRST_AMENDED rows, raw/normalized bits,
scales, classes, predicates, observables and G5a. Their raw-log line provenance is
in CUSTODY_NUMERIC_PRESERVATION.json; diagnostic work-counter changes are excluded
from that numeric comparison.
No final component acceptance, public route/resource qualification or native
Current claim follows. Prior failures and C0/source-recovery qualifications remain.
