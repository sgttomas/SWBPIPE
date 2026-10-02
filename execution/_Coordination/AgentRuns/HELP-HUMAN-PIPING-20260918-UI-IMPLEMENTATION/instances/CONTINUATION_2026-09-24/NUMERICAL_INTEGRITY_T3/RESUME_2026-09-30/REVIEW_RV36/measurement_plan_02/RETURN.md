# RV36 — measurement plan 02 review

**Row139 no-op/counts qualification is ready for a separate bounded ROOT grant.
No numeric admission, normal solve, tier execution or measurement is approved by
this review. Later automatic tiers require complete seed correspondence; full
historical published-row comparison remains held under the unchanged criterion.**

Reviewed I26/measurement_plan_03 at
`dce527dd9f12d19410b62241a411edb6bab61d35`; its 11 payloads independently verify
against seal `7b37755ad73e92e15a59bc4ff30546e56423947de89bdbdf386eaa9c960ca701`.
Receipt was 2026-10-02 04:47:35 UTC; hard end 04:59:35 UTC.
ROOT's acceptance of ordinary-artifact correspondence is carried with its limits
and timestamp correction. This review did not reopen or expand that acceptance.

| Stage | Disposition |
|---|---|
| Same-binary no-op, then row139 exact counts-only prepass, then stop | Ready for a separately timed/guarded grant, with the complete comparison below |
| W1-T1/T2/T3 normal execution and calibration | Held for actual seed qualification, per-tier grants and fresh evidence |
| W1-T4 conditional execution | Held for actual lower-tier ascent, T3 prefixes, numeric predicates and explicit ROOT release |
| Complete KF3 published-row comparison | Not established by available fields/subsets; original evidence or an owner-approved criterion change is required for closure |

## First small step

The H binary, fixed source tar and 36 input-origin hashes match. All 13 archived
source/seed/catalog files with maintained counterparts match them as well.
The seed hash is
`29e4aba283a4edeb3287d72598d7f37d29ed65799959f89005cf58bcf1d4a9f4`,
with 33 rows. The row139 seed line and retained canonical model hash match;
canonical bytes independently have length1748 and FNV4eea722b112df206.
The future measurement root remains absent; neither the seed copy nor runtime
logs are claimed to exist.

The proposed row139 argv equals the existing binary_argv construction plus
--counts-only. Its normal retained Strings are model24 bytes, counts path131
and published path157, with the other three slots absent: total312 bytes.
argv0 and numeric tokens are recorded separately. The prospective cwd, full
tokens, five repeats, five entry repeats and prefix flag are preserved.
Qualification log paths are separate from the normal records path appearing
inside argv; counts-only does not create a published-row dump.

Source anchors: H/runner/k6_runner.py:1087–1122 and :1151–1155;
H/src/bin/k6_observe/main.rs:287–297, :625–704 and :705–726.
The no-op branch precedes model construction. Counts-only ignores reading the
counts-file while preserving its literal path, computes its own counts, emits a
summary with zero completed repeats, and returns before admission_estimate_bytes
or a timed solve. Row139 is bottom-tier CHAIN10/w1a: no never/conditional hold
and no previous-size ascent prerequisite (runner:496–516). The no-op is a
process baseline, and the counts prepass is neither a performance sample nor
a rho datum.

A grant must retain the existing guard/quiet-host checks, exact artifact/input
rehash and materialization requirements, cwd/environment and per-process
60s/600s ceilings, plus its own elapsed cutoff. Preserve raw full argv because
launch normalizes absolute tokens to basenames (runner:998–1001). Check a
successful no-op and one successful computed counts row, then stop; do not
invoke run_tier for this small step.

## Complete comparison before reliance

**P2, blocking later seed reliance but not the proposed evidence-acquisition
step:** MODEL_INPUTS.expected_structural_source_metadata is a 15-field summary.
The normal binary consumes 37 serialized identity/count fields through
parse_counts_line/parse_w1 (H/src/k6/counts.rs:396–465). Complete expected values
are available in the hash-bound seed; they are not missing evidence.

The concrete comparison map is in
_run_records/SCHEDULE_AND_COUNTS_AUDIT.json. Compare all 37 consumed fields,
plus canonical length, family, source-error state, sizeof/limb/storage metadata.
For row139 require source_ok=true and no source error. Verify every fresh
context-estimate field is present, with a positive integer complete W1 maximum.
Do not require the old seed's W1 estimate fields to equal the new launch's
estimate: their retained-path/repeat/prefix context differs. Phase time/heap
observations also are not invariants. Require the expected start context,
counts_source=computed, mode=w1a and zero completed repeats.

The distinction matters: bind_w1_launch_counts checks classification, model-row
uniqueness and a positive integer estimate; it does **not** compare fresh counts
to the seed. run_tier immediately uses that fresh row for admission while the
normal binary independently rereads the seed and composes H from it
(runner:1103–1122, :1231–1246; main.rs:625–656).
A row139 success qualifies only that row/model context. Before granting later
automatic tiers, establish the complete seed correspondence for every model
they will rely on, and retain normal/prepass context correspondence. Do not
treat runner shape validation or the 15-field summary as this gate. This is
a finite execution-evidence prerequisite, not a commission to change the runner.

## Later schedule and calibration

All132 launch identities/order/repeats match the original K6b packet. Independent
comparison with all132 original b3 .record.json files found equal caps/timeouts
and argv differences confined to the explicitly rebound binary/count/output
paths. W1-T4 remains orders247–270, 12 W1 plus12 sparse, six first-pass W1
prefix/row-dump launches, with no dense/n² command. Every planned argv and
retained String length was recomputed from the read source without importing
the runner.

Fresh W1-T1, T2 and T3 chronology and each tier's no-op remain required.
Eligibility requires a recorded previous-size attempt in the same orientation
and mode; ratio calibration separately requires successful, strictly smaller,
same-family/mode rows. Orientation is pooled for ratios; targets>=1000 exclude
10-member calibration. Sparse cannot calibrate W1 and same-size pass1 cannot
calibrate pass2. These distinctions match runner:453–516 and :555–594.

The unchanged b3 wrapper clears the sole CONDITIONAL_TIERS entry W1-T4.
That can lift only the named hold under ROOT's later explicit grant. The
eligibility/prepass/admission sequence, 8GiB C, 7.5GiB heap cap, rho2 fallback,
RSS/footprint1.45 fallback, binary half-heap-cap backstop and projected RSS<=0.8C
remain active. Historical replay/approval is not fresh calibration.
Prefix limits come from the actual last full call's segment ends excluding its
last segment (staged.rs:346–353; main.rs:1008–1038); requested repeats and the
prefix upper bound do not prove completed repeats/prefixes.

## KF3 comparison boundary

**P2, blocking complete historical comparison closure, non-blocking row139:**
the original requirement is model-by-model KF3 B outcome comparison
(T3/TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md:48), with a stop for any changed W1
outcome or published row (:91). KF3_COMPARATORS.json:109 must be interpreted
as permission to perform bounded partial comparison while retaining this open
obligation. ROOT's agent-level scope disposition cannot silently replace the
protected criterion; changing that criterion requires its owner's decision.

I independently verified the six stdout hashes, original metadata source
e114b23c1/tree53f78023…, source and canonical model hashes, outcome/precision
fields and optional publication fields. CHAIN/CONT are Selected128 with
verification256 and 250013/265013 published rows respectively. Both TREE cases
are Unresolved Ceiling [Restrained], with publication count unavailable, not zero.
No .rows dump exists in the inspected KF3 subtree, and the retained case records
contain aggregate counts/attempts rather than the full published values or a
complete publication digest.

The existing R1 references contain103/194/215 expected values per CHAIN/TREE/CONT
orientation. The retained comparator applies the unchanged exact-Fraction
predicate |obs-exp|<=1e-9*max(|exp|,class scale). It is an analytic subset check,
not complete historical equality; an unresolved run does not gain a publication
or a passing row comparison. Current H repeat/publication digests and sparse
self-parity likewise do not supply absent KF3 publication values. Sparse/dense
cross_mode checks do not compare W1 with sparse, and 10000 sparse argv lacks
a solution dump.

Available-field outcome comparison must normalize the schemas: H class plus
reason/precision, rows and rows_unpublishable correspond to KF3's outcome,
published_rows and unpublishable_rows (H/src/bin/k6_observe/w1.rs:74–114;
H/src/k6/w1/staged.rs:91–96).
Do not compare H's FNV source digest directly to KF3's SHA256 as if they were
the same field. Preserve the respective source/model bindings.

Finite disposition: acquire original full-row evidence under a separate
authorized scope, or obtain an owner-approved amended criterion; otherwise
keep complete historical published-row equality unclaimed and closure held.
No recovery, old baseline run or new comparator is authorized here. H windows
retain their H expressions; KF3 VR global peaks retain VR terms. No forced VR
10000 run or measurement waiver follows.

## Evidence and stop

Only VENV JSON/hash/String metadata and source reads were used. No runner import,
model/count construction, executable, reporter, compiler, solver, measurement,
host probe, Git/index write or delegation occurred. One metadata check initially
expected absent historical pass fields, one encountered the explicitly absent
large-model path, and one over-broad counterpart check included a NUM-only
brief. Corrected bounded reads completed. One orchestration syntax error ran no
nested command; no runtime retry occurred. Raw commands/results and source
origins are under _run_records. The new brief matches its dispatched NUM commit.
The seal records directly sampled completion after the work; no inferred
last-check timestamp is supplied. No follow-on is active.
