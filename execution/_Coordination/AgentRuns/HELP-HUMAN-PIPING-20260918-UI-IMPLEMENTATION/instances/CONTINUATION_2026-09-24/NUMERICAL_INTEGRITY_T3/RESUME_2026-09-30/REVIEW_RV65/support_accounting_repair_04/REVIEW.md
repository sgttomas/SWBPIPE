# RV65 — support accounting repair backcheck

**RV65-I1 CLOSED. Bounded CLEAR for the complete same component at
`c79a1c293dbf5581468e839c8545b5a815a32df7`.** The two-file repair resolves the
known local accounting defect. Prior RV65-1/2 and dense custody conclusions remain
applicable. No actionable finding remains at this private component scope; ROOT
may proceed with local fan-in under its authority. This is not main/publication,
availability or complete resource-profile acceptance.

Repair base `8104a4fedd0fd575f20b3d723cc0cdd722d73ba1`; records/brief pin
`4f9799a9631895e1bf1bbfe9bb9755728b8ffe2c`. P=projects/chirality-piping;
PP=P/core/product_physics. The complete diff, repair RETURN/count table, source
hashes, author comparator and command custodian were read. Origins and decisive
checks are bound by CHECKS.json. The prior sealed review was not changed.

## Why the defect is closed

PP/src/retained_product.rs:783–801 defines an owned finite initialization loop.
It checks the prior sticky status **before** any new entry, enters one
ValidationEntry, verifies empty length and reserved capacity >= count, then
enters one checked MapWrite immediately before each `push(false)`. At iteration
j, length=j<count<=capacity; no push can reallocate. On accounting loss it returns
before that push. Empty count performs one shape validation and no element write.

All three consumers at850,852,854 now use that helper: built_used, spring_used
and rigid_owned. None of their former resize calls remains. This implements the
expressly selected per-element count rule: the allocation helper's existing
LibraryBoundary covers its actual `try_reserve_exact`; there is no new bulk fill
or resize call, so no fictitious library event is added. This is the selected
finite alternative contemplated by I1, not an accounting waiver.

I independently derived the following counters from the entered source paths,
then checked the fresh log. Order: SourceVisit, RowVisit, MapWrite,
ValidationEntry, IdentityByteRead, KeyProbe, AllocationRequest, LibraryBoundary,
RequestedCopyBytes, RustCapacityBytes. M=u64::MAX; C is the observed Rust vector
capacity in bytes, not allocator/RSS storage.

| Path | Expected successful prefix | Checked state |
|---|---|---|
| Reserve then fill n | `[0,0,1+n,1,0,0,1,1,0,C]` | n=0,1,4,3,12; observed C=n; exact false contents and capacity |
| Named seam with MapWrite seeded M-1 | `[0,0,M,5,0,0,1,1,0,4]` | Four preflight validations plus fill guard; reserve succeeds, first element cannot enter; no second allocation; no committed support/source maps |
| Reserve4, allow two writes | `[0,0,M,1,0,0,1,1,0,4]` | Exactly two false elements; third write refuses; capacity4 retained |

The failure tests retain the same typed MapWrite overflow on repetition and
assert unchanged counters, partial contents/capacity and observer/source state.
Because the helper checks sticky status before shape, repeating a failed partial
fill does not replace the original fault with a nonempty-bitmap shape error.
The after-reserve test reaches the previously omitted writes: its expectation of
exactly one allocation request would fail the former implementation, which
entered a second reservation. This is correction-sensitive evidence rather than
another MAX seed that stops before any reservation. Source inspection establishes
the ordering; test success alone is not the closure warrant.

## Duplicate-hit hardening and consumers

The old inferred integer is replaced at921–938 by a boolean seen state. A second
matching DOF immediately returns `rigid boundary identity`; no count can overflow.
The bounded `[0,0]` direct-helper test reaches this branch, returns the precise
association error and commits no support group or source. No giant malformed
input was allocated. The prior conclusion stands: actual production restraints
were already unique through prepare_boundary; this hardens malformed direct
helper input and is not reclassified as a demonstrated production overflow.

Valid producer scans still visit the same restraints. No other consumer, source
law, row mapping, metadata, numerical predicate, public route, observer capture,
native certificate work or capacity layout changed. Earlier RV65-1/2, empty-row
G5a, non-aliasing coverage and immutable dense custody remain confirmed by the
prior full review and the unchanged affected semantic captures.

## Fresh checks and exact preservation

Fresh PP `retained_product_tests`: **19/19**, exit0, on the exact candidate,
08:33:19–08:33:33UTC. It used the absolute locked/offline PP manifest, released
RV65 PP target, four build jobs/two test threads and a1200-second wall. Source
hashes were stable; process21988 was reaped. No unaffected FK/S11/formation check
was rerun.

The author's final debug and optimized PP commands also pass19/19. Their full
before/after inventory digest is independently reconstructed as
`fd6ceddfbbe9e25fd46d511504e326521484aec56bf9041b6660f50fc4d9ba52`, with the
exact two final source hashes. **Optimized execution is verified author evidence,
not a fresh reviewer run.** No new concern required another optimized run.
The earlier one-test command is preserved as historical: its test-file hash
precedes the final test bytes, so this backcheck grants it no final-candidate
execution credit.

The independent comparator imports no author comparator. Against my own prior
sealed full-review debug capture, it checks fresh debug plus the author's final
debug and optimized captures:

- All104 old semantic records over the two base/four material cases are exactly
  equal, including source identities, rows, values, classes, scales, predicates,
  G5a/observable outcomes, capacities and native certificate records.
- Both named I50_RECORD objects retain their exact key sets and **every field
  except adapter counts**, including complete envelopes, source/native records,
  all verdicts, observations, numerical/full-case outcomes, certificate work and
  capacities. The same immutable input/row meaning therefore underlies the prior
  independent194-row/135-PASS proof. That reference transfers by equality; it
  was not relabelled as a new numerical execution.
- Named adapter deltas are exactly `[0,0,19,3,0,0,0,0,0,0]` because the maps have
  4+3+12 entries. Each old base/material case changes exactly
  `[0,0,13,3,0,0,0,0,0,0]` for1+0+12 entries. All other counters are unchanged.

Both named cases still give complete numerical refusals:60 sparse/61 dense
actual truth-miss rows and five conservative rows per mode, with no false PASS
in the prior complete check. Repairing accounting does not create publication.
The prior fixed-scale torsion scope, public integration and resource limits stand.

Independently verified the exact two-path repair, clean candidate, eight repair
packet files, all15 new author bulk files,955 unchanged baseline files, original
ten-file packet/all73 original bulk files, and my prior review seal/bulk. No old
source/evidence was rewritten. New fresh logs remain in the authorized external
repair04 directory; BULK_MANIFEST.json identifies them by location/size/hash.

The runtime lane was released after the reaped result; a subsequent process
inspection showed guard5387/sleep only among the watched runtime processes.
The guard remains intact. No source/Git/index/API write, descendant, installation,
host-tool change, broad model/sweep or unrelated runtime occurred. ROOT owns any
integration and subsequent wider checks; this sealed backcheck closes the local
RV65 hold only.
