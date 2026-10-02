# I22 checkpoint 0 — source-only diagnosis and finite run proposal

Status: checkpoint complete; numerical investigation remains open. No new solver
witness, false publication, proof of exclusion, accepted oracle, design selection,
repair or product qualification is claimed. No experiment is running from I22.
All Rust/build/solver commands below are proposals requiring ROOT's separate grant.

## Basis, execution and custody

TASK `/root/t3_recovery_manager/i22_a1` reports to WORKING_ITEMS
`/root/t3_recovery_manager`; native mechanism is `collaboration.spawn_agent`.
No delegation occurred. The role and fence are prompt restrictions on the shared
host, not separate OS enforcement; no model identity/diversity claim is made.
The selected software-defect-diagnosis skill's actual available origin is
`<APP_WORKTREE>/.agents/skills/software-defect-diagnosis/SKILL.md`, SHA256
`7e423dfd24132c33d3aa8fe6994bf17a1def966c396723f6bd72ac8d2442112b`.

Aliases: P = projects/chirality-piping; FK = P/core/solver/frame_kernel;
T3 = this recovery root's parent; R = T3/RESUME_2026-09-30;
I22 = R/I22; Response = P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260930-T3-AUDIT-RESPONSE;
A1 = Response/instances/A1-DIAGNOSIS. Host bindings are supplied by ROOT, never
inferred from the calling cwd. This packet writes only A1_WT/I22. Owned scratch
<wt>/scratch/i22 was not written.

Product A1_WT HEAD is `d01ad98a754698631f927709d08284c272de85e8`, initially
clean. COORD is `3446bdf51d90b8f1812d161cc638d85c97bbef16`. COMMON and I22
brief hashes match the supplied pins. Root, TASK, Piping, COMMON, I22 and PLAN
were read in that order. The immutable response scope handoff was read before
selected response material at `520d7dfb790bcedabc03e92b9692884ce295be54`.
Only the three assigned audit records and needed response/probe/matrix records
were read, not all response files. BASIS.json gives complete relevant byte hashes.
Read-only `git diff --exit-code` confirms the complete FK subtree unchanged
between audit source `3bddc2b05f6106e969c7cf43373b230845c7cc66` and A1_WT HEAD.

COMMANDS.json retains source-read commands, exits and tool-returned output.
Machine roots in initial identity/hash outputs are replaced by explicit aliases;
source excerpts are otherwise unchanged. Failed discovery queries are retained:
one nonexistent analysis_runner directory, one unmatched shell glob, and bounded
searches with no match. One broad filename listing was tool-truncated; no
conclusion relies on its completeness. All actual Git commands explicitly used
GIT_OPTIONAL_LOCKS=0. No Git/index write, Python process, network call, Rust
invocation, host-tool operation, signal, guard start/change or maintained edit ran.

## Observed symptom and causal boundary

The inspected source establishes the disputed junction:

1. FK/src/structural/retained/adaptive.rs:1904 builds maxima from verification
   values at 2p, excluding input-derived rows and rows the candidate cannot
   publish. It then multiplies/divides by the binary64 body extent in retained
   arithmetic. The stop rule at :2013 onward uses this scale in the complete
   disagreement/verification-error test, then estimate, bound/theta/g and charge.
2. FK/src/structural/retained/wide/multi.rs:714 correctly rounds each candidate
   row once to binary64. A nonzero value rounding to zero has Underflow and no
   value; it is not a published zero. Subnormal rounding takes place here,
   before coupling.
3. adaptive.rs:2603 forms maxima from those rounded values. :296 couples them
   as max(tr,L*ro), max(ro,tr/L), max(fo,mo/L), max(mo,L*fo), in binary64.
   A raw subnormal rounding error can therefore be amplified by L or 1/L.
4. adaptive.rs:379 uses the extra own-row rounding allowance only for
   0 < S_pub < 2^-988; S_pub = 0 gives b = 0 and S_pub >= 2^-988 uses the
   original bound. :412 classifies at the published scale and value.
   The floor at p512 applies only to force/moment after coupling.

The audit's exact example refutes a general proof premise. It is not evidence
that the complete solver accepts the abstract candidate. The old B01 observation
is a source selection with honest rows; both old B02 observations are numerically
honest forensic outputs from guard-failed runs. All remain provisional evidence
inputs. B03–B16 and all C cases remain unrun. No old B02 is promoted to a clean pass.

Causal confidence is high for the source junction and missing raw-operand error
term; realized false publication remains unestablished. Competing outcomes remain
source/geometry refusal, ledger/precision/exponent limits, condition or pivot
escalation, different refinement, positive W-plus, complete-layout rejection,
force/moment estimate or charge, O9 range exclusion and receipt encoding.
verify.rs:1113–1179 includes positive t1 even if residuals/delta are zero; exact
verification truth cannot be used to set W-plus to zero.

## Source admission and product reachability

The 24 fixed inputs have two finite nodes, one nonzero axis-aligned member,
orthogonal y-reference, positive normal E/G/A/Iy/Iz/J, valid distinct zero
constraints, finite separate nonempty-ID load terms and positive normal springs
where present. The optional collections are empty. STATIC_INPUT_CHECK.json
confirms probe per-case bits, load IDs/terms, free-index complement and springs
match the raw matrix. Shared probe constants were inspected separately.
These facts meet the inspected constructor checks, but the constructor has not
been executed in this run.

source.rs:288 refuses nonpositive/nonfinite properties and subnormal derived
A/I/J. :356 onward requires finite coordinates, exact nonzero chord and
nonparallel reference; it places no general 2^53 coordinate restriction.
Finite subnormal loads are allowed; spring stiffness must be finite and positive.
All matrix A/I/J are normal, including the short-body cases. Positive scalar
stiffness and the C axial spring construction support nonsingularity as source
algebra, but do not certify any solver gate.

Product entry is a separate boundary:
- product_physics/src/lib.rs:2151 invokes CapturedInvocation::parse before the
  captured solve. source_receipt.rs:109 calls checked(raw). canonical_json/src/lib.rs:94
  rejects integers/integral floats outside ±(2^53−1).
- Literal SI transport of B01/B05/B06/B10/B11/B13/B15 is excluded by their large
  coordinates or properties. All C17–C24 have large section-area numbers, including
  short C22; their literal primitive numeric encoding also fails that checked
  profile. This is a sufficient literal-transport obstruction, not a proof that
  every physically equivalent product request or unit representation is excluded.
- Remaining B inputs avoid this particular magnitude obstruction. They still
  lack a demonstrated product model mapping: the public primitive permits
  independent A/I/J while product sections are derived, and typed/captured
  routing, normality and model/domain checks remain relevant.
- A bounded maintained-core search finds actual retained_api solve consumers in
  the kernel/harness, not the ordinary product_physics route. The historical typed
  product entry bypasses original request custody but does not thereby call the
  new retained solver. No product witness or typed/captured equivalence follows.

No arbitrary cutoff is proposed. Failure to find a witness would not prove
exclusion, and constructor admission would not prove complete selected publication.

## Finite matrix, controls and interpretation

Let h = 2^-1074. The table is diagnosis from the provisional existing inputs,
not oracle truth accepted by ROOT. T is a tip-torsion source, A is tip-axial.
Each B source has a scalar free stiffness 4. Full publication inventory is
required, not just its one free displacement.

| Cases | Existing inputs / role | Required observations |
|---|---|---|
| B01, B02 | T L=2^100; A L=2^-100; load 5h | Raw driver 5h/4 rounding before L and 1/L amplification; selected p/P and all rows. B01 prior honest; B02 prior guard-failed. |
| B03, B04 | T/A L=1, load 5h | No-amplification controls; direct row rounding and small-scale allowance. |
| B05, B06 | T L=2^85 / 2^86, load 5h | Coupled translation scale below / at 2^-988 from a rounded h driver; branch and bound bits. |
| B07, B08 | A L=2^-85 / 2^-86, load 5h | Inverse-coupled rotation below / at 2^-988; same boundary in the other direction. |
| B09, B10 | T L=2^-100; A L=2^100; load 5h | Rounded coupling becomes S_pub=0 while a retained coupled quantity may be positive; distinguish zero scale from a nonzero false row. |
| B11, B12 | B01/B02 geometry, load 4h | Exact-subnormal driver h; remove raw rounding loss. |
| B13, B14 | B01/B02 geometry, load 2^-1020 | Normal-boundary driver 2^-1022; no raw subnormal loss. |
| B15, B16 | B01/B02 geometry, load 2h | Driver h/2 tie rounds to zero: O9 Underflow/Unpublishable control, never fabricated zero publication. |
| C17, C18, C19 | L=2^100, axial a=2^76, ground spring k=2^-33, opposed F=2^-900, separate t=9h/8h/10h; torsional driver | Finite tail/candidate-disagreement hypothesis at below/equal/full verification allowance; actual source/ordering, refinement, W-plus and full-layout gate evidence control the result. |
| C20 | C17 with t=0 | Tail-off control; does removing the discrepancy change the actual path? |
| C21 | C17 with ground spring/load at other node | Ordering/ground-placement control; no assumption of same candidate. |
| C22 | L=2^-100, a=2^276, k=2^167, F=2^-900, t=9h | Zero-scale/sub-binary64 truth hypothesis; independently retain any nonzero truth exactly, distinguish O9 refusal. |
| C23 | L=2^100, a=2^56, k=2^-33, F=2^-920, t=9h, separate kq with q=2^-1008 | Relative-floor hypothesis; report both relative denominators separately. |
| C24 | C23 with t=0 | Relative-tail-off control. |

Proposal: exactly one fresh M5 invocation for B01–B16 in listed order, each
reviewed before the next. This deliberately produces fresh basis evidence for
B01/B02 without reclassifying old runs. Return B checkpoint before a separate
C17–C24 grant. One invocation per case; no retry, repeated run, cap increase,
new exponent, variant or bulk search is implicit. Stop the affected path on an
unexpected standing/outcome change or tooling failure. A false unmutated
publication is an immediate BLOCKING return.

This matrix directly varies raw translation/rotation subnormal drivers in both
coupling directions and observes every force/moment row. It does not by itself
supply isolated nonexact raw force/moment driver witnesses for both cross-kind
directions. Record that coverage gap rather than claim the common function's
four expressions were all realized by an independent witness.

### Finite unit controls and limits

Propose these output-only comparison bundles after source truth is frozen:
U1 identity units on B03/B04; U2 B04/B08/B10 translations and scales m→mm and
m→ft; U3 B03/B06/B09 rotations and scales rad→deg; U4 B01/B02 force and moment
rows/scales N→lbf and N·m→lbf·ft. Include zeros, bounds and class thresholds where
present; record exact definitional conversion and rounded catalog arithmetic
separately. Actual catalog constants/order must be pinned by the oracle before use.
Units::convert_for_dimension :917 preserves identity; different units route
through canonical scaling. display_units.rs:27 accepts any finite converted
result, so a rounded-to-zero display outcome is not a retained solver zero claim.

These use the same solver source/output bytes. They do not establish source-model
unit-variant reachability, re-run solving under different primitive bits, native
UI behavior, or repaired display/receipt semantics. The old Rust probe has no
display conversion entrypoint; a direct Rust facade/unit witness needs its own
explicit small grant or remains an unrun gap. No new source variants are currently
prepared or authorized. Independent source-unit variants, if required by design,
must be finitely specified and re-oracled through ROOT, not silently appended.

## Independent oracle interface

ROOT approved only the input data shape, not any truth/denominator convention.
ORACLE_INPUTS.json SHA256
`fdc86cca269c660e88a9a977990bf52bf3beadd08b67dc8fe8ab30358fdcd091`
contains case IDs, raw primitive bits, separate load terms and IDs, source pin
and provisional input hashes. It omits old expected rows, parameters, hypotheses,
free_dofs, statuses and outcomes. Parent verified the shape and ROOT released it
to a fresh oracle. I22 has seen old expectations and is not that oracle.
No old oracle.py or comparator was executed or supplied to the fresh oracle.

Proposed return contract, to be frozen by that TASK through ROOT:
- Before receiving solver outputs, return input-file hash; independently derived
  source equations, sign convention, constrained/free layout, complete stable
  row-key inventory and exact rational truth for every row; representation as
  numerator/denominator or n*2^e, preserving values below binary64. Independently
  define nearest-ties-even and overflow/underflow classification.
- State adopted document/revision for each publication guarantee. Return both
  |published−truth|/|published| and /|truth| with zero denominators explicit.
  Do not choose whichever ratio passes. Any conflict in accepted text returns
  to ROOT; neither old comparator nor this packet accepts a denominator.
- After the derivation/hash freeze, accept one complete probe stdout plus stderr,
  process exit, guard outcome, candidate/source/probe/binary/input hashes and
  environment. Validate all stable row keys exactly once, source encoding,
  status, selected p/P, full rows and ranges, four body scales, class lists,
  absolute-bound bits and p512 force/moment floors.
- Report raw absolute error, claimed interval/qualified interval predicates
  separately, both relative ratios, scale/class/encoding mismatches, range
  discrepancies, missing/duplicate rows and any underflow-to-zero substitution.
  Output observed/refused/unresolved/guard-failed separately; refusal is no pass.
  Old R7 absolute factors 1+2^-22 at p128/256 and 1+2^-21 at p512 are disputed
  guarantee context to inspect, not permission to relax a protected criterion.
- Independently derive finite unit conversions when those controls are granted.
  Public summaries cannot verify private W-plus/t1/t3 or full retained states;
  they can still refute a published claim against independent truth.

Coordination and results return through recovery manager to ROOT. The fresh
oracle must not receive this diagnosis, matrix hypotheses or old solver outputs
until its derivation is frozen.

## Source-pinned build/run proposal and resource request

ROOT must grant preparation/build and bind the existing M5 guard liveness check
and invocation supervision, exact cargo executable/configuration and <VENV>.
No obsolete M3 wrapper, qualification, latch or limit is inherited.
Existing <wt>/guard/memguard.sh remains unchanged. If missing/not running or if
the normal route cannot supervise these jobs, stop and return the exact blocker;
do not build a wrapper or repair guard tooling.

Use scratch layout:
- <wt>/scratch/i22/source: immutable FK subtree archived from 3bddc2b..., whose
  equality with product d01ad98... is verified in this packet.
- <wt>/scratch/i22/probe: pinned A1/src/main.rs, src/cases.rs and continuation_01/Cargo.lock.
- <wt>/scratch/i22/target: private target, wholly inside the existing write fence.
- <wt>/scratch/i22/runs/<case>: complete stdout/stderr, exits and guard evidence;
  portable copies/hash inventory later in I22/runs/<case>.

After a grant, the intended source/copy argv are:
```sh
GIT_OPTIONAL_LOCKS=0 git archive 3bddc2b05f6106e969c7cf43373b230845c7cc66 projects/chirality-piping/core/solver/frame_kernel
GIT_OPTIONAL_LOCKS=0 git show 520d7dfb790bcedabc03e92b9692884ce295be54:<A1>/src/main.rs
GIT_OPTIONAL_LOCKS=0 git show 520d7dfb790bcedabc03e92b9692884ce295be54:<A1>/src/cases.rs
GIT_OPTIONAL_LOCKS=0 git show 520d7dfb790bcedabc03e92b9692884ce295be54:<A1>/continuation_01/Cargo.lock
```
ROOT-approved preparation directs the archive into the named source folder and
show stdout to the named probe files, hashes copies, and authors only probe
Cargo.toml with the same original package/profile and dependency path
`../source/projects/chirality-piping/core/solver/frame_kernel`.
No solver/harness source edit is needed. Retain the probe's truthful
SOURCE_COMMIT=3bddc2b... field and separately record d01ad98... parent basis;
do not relabel the binary as a repaired source. The two-package historical lock
must be validated by locked offline build; no generate-lockfile or install.

Proposed child argv after ROOT's existing-guard precondition:
```sh
cargo build --offline --locked --manifest-path <wt>/scratch/i22/probe/Cargo.toml --bin a1_public_probe --no-default-features -j 4
<wt>/scratch/i22/target/debug/a1_public_probe B01 100000000 100000000 B
```
Environment: RUSTUP_TOOLCHAIN=1.97.1, RUSTUP_AUTO_INSTALL=0,
CARGO_INCREMENTAL=0, RUST_TEST_THREADS=2, CARGO_NET_OFFLINE=true,
CARGO_TARGET_DIR=<wt>/scratch/i22/target. One cargo process for this slice.
Preserve owner's HOME/CODEX_HOME and configured identity. Reject/unset injected
FK_SEEDED_FAULT, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, RUSTC_WRAPPER and
RUSTC_WORKSPACE_WRAPPER as ROOT permits, and record effective cargo config.
No mutation-controls, cfg(test), source patch, alternate toolchain or network.

Substitute only the finite case ID and matching B/C argument after the prior
result's review and current ROOT grant. C's first argv is C17 100000000 100000000 C.
Comparator argv depends on the fresh oracle artifact/API; request an explicit
interface such as <VENV>/bin/python <ORACLE>/compare.py --inputs <ORACLE_INPUTS>
--frozen-truth <TRUTH> --stdout <CASE_STDOUT>, but do not assume those files
or an accepted CLI exist. No Python command has been executed here.

Resource proposal, not admission or proven bounds: one build slot, 300 s wall
limit and 2 GiB process memory; one case at a time, 60 s and 512 MiB, 100,000,000
LME case/invocation each. ROOT must select/enforce or reject caps using existing
M5 facilities; the old M3 numeric proposals do not prove M5 resource adequacy.
Maximum numerical jobs: 16 B then 8 separately granted C. No dense large model,
timed measurement, DEC-025 sweep or product test is part of this checkpoint.
The model has one member. No builds overlap ROOT's exclusive timed slot.

Before build: confirm guard, source/probe/lock hashes, toolchain/config, slot and
target. After build: retain full logs, feature fingerprint and binary hash.
Before/after each case: source/input/binary hashes, ROOT grant identity, raw
guard and process outcomes, full output, meter, attempts/reasons and comparison.
Any failed/unknown guard status remains failed/unknown regardless of numerics.
An old result cannot satisfy a newly changed source or oracle obligation.

## Downstream consequences and exact outstanding work

The immediate dependent proof surfaces are stop (a), force/moment charge (d),
p512 floors, absolute/relative classes and zero-bound behavior. R7 §6.3
(:839) also assumes converted S*_tr is relatively within 2^-50 of the retained
scale for G5a's lower bound. D2 §4.9.3 G5b/G5c re-derives published scale/class,
and §4.11 binds intervals, including exact-point treatment at zero bound.
The source evidence identifies these dependencies; it does not establish a G5a
runtime failure, amend D2 or authorize F2a reliance. Historical D2 base wording
must be reconciled with later A1/R7 amendments by the design owner.

Still required:
1. ROOT's source-pinned preparation/build/run grant, guard/environment bindings,
   finite resource disposition and fresh oracle derivation/interface acceptance.
2. Fresh B runs and complete independent comparisons; then separately granted
   C runs with actual gate/refinement evidence, or precise refusal/unresolved gaps.
3. Explicit disposition of raw force/moment-driver coverage and source-unit
   variants. Output conversion controls do not close either gap.
4. Independent design re-derivation of the full publication guarantee, including
   threshold crossings, zero S_pub and G5a/unit consequences. ROOT selects within
   its authority; product-domain/contract changes return to the owner.
5. Any separately selected repair, fresh independent complete-diff review and
   all actual-candidate protected checks, CI, DEC-025, GEN-8, T9/both-entry and
   applicable practitioner/receipt obligations. None is waived or reported run.
   Required mutants, if subsequently granted, must die; a surviving required
   mutant stops that acceptance path. Mutated sources are labelled and cannot
   establish unmutated false publication.

Numerical work remains at source/checkpoint stage. Supporting work produced
this finite packet and input-only transfer, not host-tool development.
No source-model solve, build or experimental process was launched; therefore
none is left running by I22. SHA256SUMS seals this checkpoint only. Later work
must use an additive continuation, preserving these checkpoint bytes.

