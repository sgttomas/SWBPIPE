# I50 — RV65-I1 bitmap accounting repair

**The two-file repair is complete and ready for RV65's same-reviewer backcheck.**
All three support bitmaps now account for each initialization write before it
occurs. The first-reserve/then-exhaust discriminator stops before filling and
before a second allocation. Old and named numerical results are unchanged.

TASK `/root/i50_named_case_component` under ROOT HELP_HUMAN `/root`, native
delegated execution, no descendants. Receipt 2026-10-03 08:05:23 UTC; checkpoint5
08:10:23, first-focused10 08:15:23, cutoff15 08:20:23, seal20 08:25:23 UTC.
The first focused test finished08:08:51; checkpoint was sent08:09:50. Runtime
was released08:14:38, with no owned Cargo/rustc/test process and guard5387 active.

Frozen CODE basis: `8104a4fedd`. Repair grant: `d332699b2d`; reviewed finding:
`0869aa6265`. Full pins and instruction/review hashes are in ORIGINS.json.
Only P/core/product_physics/src/retained_product.rs and retained_product_tests.rs
changed; P=projects/chirality-piping. FINAL_SOURCE.json records their full hashes.

## Exact finite count rule

`fill_support_bitmap` first checks prior accounting status, enters one
ValidationEntry, and checks empty length plus sufficient reserved capacity.
Its owned finite loop enters one checked **MapWrite immediately before each
reserved-capacity push(false)**. It returns before an unchargeable push.
There is no resize or other bulk operation, so no fictitious resize/library
entry is charged. The reservation's existing LibraryBoundary still covers its
actual try_reserve_exact. The loop cannot reallocate because count <= capacity
was checked before it starts. Empty count enters no element writes.

The same helper fills built_used, spring_used and rigid_owned. Successful
reservation/capacity records remain in support_capacity_bytes after a later fill
failure. A prior fault stops a repeated call before further checks or writes.
No support group, spring map or PrimitiveSource is committed on the tested
first-fill failure.

Counter order below is SourceVisit, RowVisit, MapWrite, ValidationEntry,
IdentityByteRead, KeyProbe, AllocationRequest, LibraryBoundary,
RequestedCopyBytes, RustCapacityBytes. M means u64::MAX.

| Discriminator | Independently expected and observed successful prefix | Observable state |
|---|---|---|
| Named support seam; initial MapWrite=M-1 | `[0,0,M,5,0,0,1,1,0,4]` | First reserve succeeds; retained first capacity4; first fill write refuses; no second allocation and no committed maps. Four preflight validations plus one fill guard. |
| Reserve4, then allow exactly two fill writes | `[0,0,M,1,0,0,1,1,0,4]` | Exactly `[false,false]` written; third write refuses; capacity4 retained. |
| One reserve and successful fill of n | `[0,0,1+n,1,0,0,1,1,0,n]` | Checked for n=0,1,4,3,12. The one non-element MapWrite records capacity; n=0 adds no element write. |

Both stopped controls return the same typed MapWrite overflow on repetition,
with exactly unchanged counters, partial contents and retained capacity. The
after-reserve test would fail the reviewed implementation: it previously reached
the second allocation and performed uncharged fill writes. This is not another
MAX-seeded test that stops before reservation.

The rigid-boundary hits integer is replaced by a boolean match state that returns
immediately on the second match. A bounded direct-helper `[0,0]` control refuses
with rigid boundary identity. The actual production boundary was already unique
through prepare_boundary; this is malformed-helper-input hardening, **not a
demonstrated production overflow**. No huge malformed input was allocated.

## Validation and preservation

- First focused discriminator:1/1.
- Affected PP retained_product_tests: **19/19 debug and19/19 optimized**.
- In each build, **104 old semantic capture records across six original cases**
  match the frozen candidate exactly.
- Both complete named captures match in every field except adapter counts:
  request, envelope, source/native records, facts, all verdicts, observations,
  numeric/G5a/observable outcomes, certificate work and capacities are unchanged.
- Actual named accounting changes are exactly **+19 MapWrite and+3 ValidationEntry**
  (4+3+12 flags). Original base/material cases change exactly **+13 and+3**
  (1+0+12 flags). Every other counter is unchanged.
- Both named cases still have their complete finite numerical refusals; this
  repair makes no numerical or publication claim beyond preservation.

All three Cargo commands pass and are reaped. They used the sole lane, four build
jobs/two test threads, absolute locked/offline PP manifest, twenty-minute walls
and the released I47 PP cache. Command records bind before/after source digests
and deltas from one baseline. FK/S11/public formation checks were not rerun:
their source and this repair's numerical semantics are unchanged, as the brief
requires. No current attempt failed.

Only two of957 baseline core/fixture files changed;955 are unchanged. The prior
ten-file implementation packet and all73 manifested external bulk files verify
unchanged, including its seal and FIRST_RUN evidence. Prior source bytes remain
at the frozen Git revision. New bulk logs, prefix evidence and comparisons are
under WT/scratch/i50_first_publishing/repair03, with full SHA-256, bytes and
absolute locations in BULK_MANIFEST.json. The compact packet has eight files
including its single final seal, written after the final audit.

No Git/index/API write, unrelated source change, host-tool/guard change,
descendant, giant allocation, new numerical method or resource-profile claim
occurred. Host permissions remain unrestricted; the narrower brief was observed
by the agent. The known fan-in hold is for ROOT and RV65 to lift after backcheck;
passing these tests is not itself independent acceptance or public publication.
