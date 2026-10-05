# I40 — recorded kernel origins, checkpoint B return

Bounded native candidate for ROOT freeze and fresh independent implementation
review. Focused checks pass. No receipt, caller, memory profile, engineering-policy
acceptance, public W1 activation or release follows from B.

TASK `/root/i40_kernel_origins`, parent `/root` HELP_HUMAN, used delegated-harness-
native execution. No delegation or Git/index/API mutation occurred. HEAD remains
`887b790c2a646a67f3ac28e63c3eb57275c1868b`; the reviewed maintained prerequisite is
`fdae294643b798c1849da8b2e643085562593686`. The first UTC witness was 2026-10-02
23:54:14 (the assignment exposed no machine receipt timestamp). Checkpoint was
returned at 2026-10-03 00:08:07. All test commands ended by 00:06:38.794; the lane
was explicitly released, and no compiler was restarted. This is an early return.

## Implementation boundary

- `RecordedInvocation` owns the sole private InvocationMeter, six inventories and
  selected-owner registry. It requires an explicit work limit and finite opaque
  OriginCapacity. ROOT acknowledged this concrete adjustment from C2's illustrative
  separately borrowed-meter signature. There is no default allowance, mutable
  meter escape, reset/refund or caller-supplied BuildRef registration.
- Context case batches return `RecordedCase { outcome, run }`; combinations return
  `RecordedKernelCombination`: typed pre-source refusal, typed missing-origin
  refusal, or WithRun. Existing legacy and I37 RecordedCombination signatures stay
  compatible. Metadata is descriptive, not a self-authenticating receipt.
- The common case loop preserves status/exhaustion, full-stiffness-byte grouping,
  group preparation and case preparation order. The one numerical core receives
  optional trace hooks at actual obtain/obtain_verify returns and finish. Legacy
  mode projects each result before appending and allocates no origin inventory.
  No arithmetic, price, schedule, stop rule or numerical helper changed.
- Actual Calls/Runs/groups/builds have consecutive native identities. Each physical
  record has two separate requested-build links; builds retain actual WorkTotal,
  stage bundle and success/nonbudget/budget-failure state. Budget failures remain
  uncached; hits preserve the prior Build/status and are not charged again.
- Source/group/schedule/entry phases follow the actual branch. An exhausted case
  has Source/Run but no group/build and zero actual increment. A pre-source refused
  combination has Call but no Source/Run/group/import/build; its native reason order
  and actual unchanged meter survive. Empty attempts do not determine phase.
- Sources retain full K4SRC/K4CMB and stiffness bytes. Successful combined prep
  additionally retains its actual K4LED before moving into the core, including an
  unavailable result with no surviving RetainedSolve. Case entries have no invented
  combination-ledger value. Native owner ordinals bind call position and source;
  they are not claimed as PP authored-request indices.
- Selection freezes seven slots. Imports choose the first occupied selected
  snapshot per slot in authored order, preserving original build/group/run. New
  combination slots cannot backfill operands or later combinations.

## Selected-owner and reservation proof

Only actual Selected outcomes enter the private registry. Every offered context
entry creates its own fresh Arc<CasePrep> once; no offered API accepts or reruns a
prep, imports an arbitrary cache, or registers an external origin. The registry's
strong Arc prevents address reuse after the result is dropped. RetainedSolve's
public interface cannot replace prep/cache/evidence; Clone preserves that same
immutable selected owner and finish snapshot. Arc identity therefore identifies
the actual selected run in this API. Full source bytes and slot occupancy are also
checked. Equal-value foreign and legacy solves refuse before combined preparation
or cache import. Public copied metadata cannot grant provenance.

For declared batch lengths n_i and combination lengths h_j, construction checks
C=sum(n_i), Z=count(h_j), H=sum(h_j), A=count(n_i)+Z, R=C+Z, 4R and 7R. Record count
× Rust element size must fit isize::MAX; reservations are fallible and exact.
Each call checks consumed A/C/Z/H against that finite declaration and reserves its
nested owner/source/run/operand vectors and output/index scratch from actual input
lengths before work. There is no automatic quota growth.

Each case creates one Source/Run, at most one group/selected entry; each valid
combination creates one of each. Pre-source failure consumes its call/term quota
but creates none of those records. Thus their global vectors fit R. The fixed
schedule has at most four physical records and seven build opportunities per Run;
hits create no Build, while budget retries consume the next actual Run's allowance.
Builds therefore fit 7R. Per-call source/run references fit its actual case count
or one combination result. Snapshot/import/link arrays are fixed. Base+position
and base+group-index ordinals are dominated by these checked aggregate bounds.
Internal assertions protect private schedule invariants, not a fallback admission.

These representation checks and successful allocations are not a qualified memory
permit. Future P1/M1 admission must bind actual source/encoding, numerical/cache,
error-payload populations and lifetimes before a qualified product route uses
this API. No historical K6c/P3 profile is rebound.

## Ownership and lifetime delta

Recorded mode adds capacities for Calls/Sources/Groups/Builds/Runs/selected entries;
nested call reference arrays; requested factor/source refs; selected-index/output
scratch; full source/stiffness byte copies; and actual combined-ledger bytes.
Builds own typed stop/work/stages; Groups own seven imports/live slots; Runs own
before/after slots, four dual links, native references, phase and RunWork. Context
inventories remain until context drop; returned outcomes keep native ownership.

Each selected registry entry adds one strong Arc<CasePrep> until context drop,
extending its existing PrimitiveSource, ledger, prescribed pairs, identity,
layout, extents and factor-list lifetime even when a caller drops RetainedSolve.
The registry adds no factor/state/cache payload clone. Source-registry byte copies
precede the kernel exhaustion check: they are unpriced setup, not evidence of zero
CPU/allocation cost. No GroupPrep, CasePrep or build passes the exhausted case's
kernel branch.

Legacy mode adds no origin allocation, but optional trace/projection stack layouts
may change. Extracted validation/preparation helpers also drop the stiffness
comparison byte vector and temporary prep-reference vector before the schedule;
old lexical scopes kept them live through it. This reduction and all recorded
owners require profile rebinding. No identical heap-window/stack-size claim is
made. Numerical/Debug bytes and prices are separate, tested claims.

## Checks and exact write set

Eight maintained paths and SHA-256 values are in
`_run_records/FINAL_MAINTAINED_HASHES.json` (hash of that file:
`affddc13b0cca8d4c3ac56e1978d62367269285144e8996792729048eb88db2c`). Source remains
byte-identical to tested `candidate02_hashes.json`, including the fingerprinted
unchanged fixtures. `WRITE_INVENTORY.json` lists paths and informational hashes.
No fixture/oracle/tolerance, instruction, dependency/Cargo/lockfile, H/VR, PP,
reader, schema or host tool changed. Root/TASK/Piping and accepted C2/brief hashes
are retained, with exact accepted-commit byte checks.

| Command record | Result |
|---|---|
| test02_origins | 9 origin controls pass, debug |
| test03_origins_release | Same 9 pass, optimized |
| test04_checked | 17 existing checked-work controls pass |
| test05_legacy_prices | Existing golden-work-count control passes |
| test06_site_inventory | 3 scanner/inventory controls pass |

`COMMAND_INDEX.json` retains exact argv, source hashes, four jobs/two threads,
per-manifest target, 1,200-second walls, PIDs, times, exits and raw logs.
Cargo-generated metadata identifies rustc 1.97.1 / aarch64-apple-darwin / LLVM
22.1.6; reading it after lane release started no compiler. Diff check passes.

Origin controls cover a fixed ordered build-event oracle; u128 charge conservation;
shared-stage reconciliation; real cached nonbudget failure versus budget retry;
separate calls; exhaustion/group failure; full legacy Debug/work parity; frozen
snapshots and authored imports; equal-value foreign/legacy denial; Clone identity;
pre-source numbering/order; and exact K4CMB/K4LED custody. The no-backfill control
uses the existing thread-local final-state seed to cause genuine combination-local
builds and unavailability; it injects no origin, cache, work or BuildRef.

The first run is preserved: 7/8 passed; the new mechanism fixture cleared
constraints while retaining incompatible supports, causing truthful constructor
SupportMismatch. Clearing that fixture's support declarations corrected setup.
No product or expected numerical criterion changed to obtain a pass.

No new natural post-work Refused or every-preparation-error witness is claimed;
the common typed preparation and terminal-custody paths remain inherited from the
reviewed I37 basis. No heavy/native/timing/model-scale/DEC-025/full historical
observation programme ran. Prepared ordinary operands, PP source/material/id maps
and owner binding, closed serialized CountRange/WorkAccounting/OriginError maps,
safe integer projection, producer/three readers, admission and profiles remain
named next interfaces. WorkAccounting and numeric prior stay typed, without Debug
decoding. No unresolved native design decision was found inside B. ROOT owns
integration, independent review and later gates. All I40 processes exited; existing
memguard PID 5387 remained running. See `_run_records/FINAL_STATE.json`.
