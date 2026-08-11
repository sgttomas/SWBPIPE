# STRUCTURED RETURN — A2-VERIFY

## Status

`FAIL`

The three author packages are substantially evidence-grounded, and the R14
packet passes independent review. The fan-in is not owner-ready as frozen,
however, because the DEC-046 packet does not make the controlling
convergence-versus-verification contradiction fully decision-ready, and the
MAINTAINER_REVIEW packet contains one exact evidence-identity error plus an
incomplete truthful-candidate preparation scope. These are repairable author
defects, not evidence absence and not an authorization blocker.

## Identity and parentage

| Field | Value |
|---|---|
| RunID | `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES` |
| ParentInstanceID | `WORKING_ITEMS/Agent1/working_items_del0904_owner_gates_prepare` |
| ChildInstanceID | `A2-VERIFY` |
| Agent form | fresh non-delegating ephemeral Agent 2 generalist |
| Exact branch | `codex/piping-del0904-owner-gates-20260810` |
| Exact HEAD | `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3` |

## Author-output identity verification

All six current author-file hashes equal the values frozen in the three
author `STATUS.json` files.

| Author output | Recomputed SHA-256 | Status-file match |
|---|---|---|
| `instances/A2_DEC046_CENSUS/PACKET.md` | `681fad684d1b2796ba5114bf8eec73f7dae6a8b1edf91151cb41cbdd2fce2569` | `PASS` |
| `instances/A2_DEC046_CENSUS/RETURN.md` | `c3c55ff2b166f2e4b355bdba9af4feef5b44106d6ccf9b0a5ea7f5ca984cf083` | `PASS` |
| `instances/A2_R14/PACKET.md` | `8218b4566c4e3d4476ee7f29db4b7e96ae8740c32273bc9b7c4102b86172335d` | `PASS` |
| `instances/A2_R14/RETURN.md` | `d5fb2d6c520829068425b37ace53d0f3d3f34568fa02fa3df1fb91708d2c8106` | `PASS` |
| `instances/A2_MAINTAINER_REVIEW/PACKET.md` | `baca7a85dc5e936b6f7b99c0cabcd15e327befc2259490546d48a32e4f59ae8c` | `PASS` |
| `instances/A2_MAINTAINER_REVIEW/RETURN.md` | `68c20561b368e8074560d33560319a4d2d6aa4e714a870a3e34cc9462e4fb3ea` | `PASS` |

## DEC-046 / four-suite / 13-case review

| Claim class | Independent result | Direct basis |
|---|---|---|
| D-19 axis law | `PASS` | D-19 blob `9275cd9c44f1af84b443910e08c8ba73aa0e8791` expressly makes DEC-046 solver convergence separate from DEC-024/026 result-versus-reference verification. |
| Current mechanics/stress internal comparison | `PASS` | Mechanics blob `eb65e53075110995a4ddcd93b4181b15392f91d5` and stress blob `201f5d84d1a666975000d07fb8e21900b88f9807` expose finite inclusive absolute `abs(observed-recorded) <= 1.0e-9` and disclaim release-policy status. |
| Mechanics evidence population | `PASS` | `SUITE_RUN_MECHANICS.json` blob `c0fee1b6581169e4427686e5c1932ec6176f5ceb`: 24 requested, 11 matched, 0 mismatched, 13 blocked, 91 compared values. Recomputed maximum absolute delta is `3.552713678800501e-15`; 13 nonzero deltas occur in exactly PHYS-004/005/006/007. |
| Mechanics per-dimension maxima | `PASS` | Independent `jq` grouping reproduced all seven packet rows: dimensionless `9/0/0`; displacement `6/1.734723475976807e-18/1.2390881971262908e-16`; force `42/1.7763568394002505e-15/4.440892098500626e-16`; length `7/0/0`; linear stiffness `2/0/0`; moment `20/3.552713678800501e-15/1.7763568394002505e-15`; rotation `5/8.673617379884035e-19/2.0016040107424698e-16`. |
| Stress comparison capture | `PASS` | R14 stress output blob `2feb1b9cf0abc0f978457a6e1564db4596239a22`: 3 requested/matched, 0 blocked, 11 exact value comparisons. |
| Nonlinear runner comparison capture | `PASS` | R14 nonlinear output blob `ce8dc556b66dc4384b87acc73829129f1b70b719`: 5/5 exact categorical/result projections, 2 recorded converged and 3 recorded nonconverged. |
| Sparse values | `PASS` | Observation blob `ad57a55ca33e9b3f7f1a57eb196db52adc49484c`: 9 rows; recomputed maxima `7.060341894958857e-11`, `5.342535303043405e-10`, and `1.0058283805847168e-7`; repeat deltas and nonpositive-pivot counts are all zero. DEC-053 policy limits are `1e-9`, `1e-6`, `0`, and `0`, with explicit non-release fences. |
| Effective convergence semantics | `PASS` | Nonlinear-integration blob `36062a62a95da7efe2bf801a7dad8f3c398f6c9d`: `effective_tolerance=max(residual_tolerance, absolute_residual_floor)`; iteration loop is `1..=max_iterations`; active-set count is compared inclusively. A count threshold in `[0,1)` is behaviorally zero; `>=1` can accept a changing support. |
| Current validation-scope values | `PASS` | DEC-046 records blobs `d3850f5becda48c523dfb15cd5d87aabe54220c0` and `a6f43a50aa7682d85409f1d25b8982aeff7bd6a9` carry `0 count / 0 count / cap 4` for four seed classes and multi-support. Both expressly exclude release scope. |
| 22-fixture cap derivation | `PASS` | Nonlinear source blob `37d19abf27a17c1d1333a81732d0b413e4ca2880` records 9 accepted assembled-seed observations (all at most 2 iterations) and 13 accepted multi-support observations (12 at 2; cascade at 3). Thus O-A caps `2/2/2/2/3`, O-B cap `4`, and O-C cap `6` admit/exclude iterations as stated. |
| Exact 13-case set and rows | `PASS` | Recomputed source/packet ID symmetric difference is empty. Every source row has recorded expected values and all `observed`, `delta`, and `within_recorded_basis` slots null. Packet value counts and quantity-kind summaries match all 13 source rows. |
| 13-case cause | `PASS` | Runner blob `75fa69df616dd803ebd8409683d2468536c4b6ac` maps 11 mechanics IDs and routes other IDs through `CaseEvaluation::NotReusable`; no numeric predicate is reached for the 13. Primary classification `implementation=13`, `data=0`, `tolerance=0`, `fixture=0` is supported; secondary fixture/API descriptions are consistent with live suite functions. Every convergence option unblocks `0/13`. |
| 24-versus-25 currency caveat | `PASS` | Current mechanics source asserts 25 and includes DEC-092 added by `c394365ca72b8383c7d7203ce5be2cb9ea67d508`; runner binding is unchanged and has no DEC-092 arm. The historic 13 is exact, a current 14th block is a labeled inference, and no current whole-suite capture proves it. |
| Four-suite release-value decision interface | `FAIL` | See finding `V-F01`. |

## R14 P1-P16 and currency review

### Exact bundle inventory and integrity

- Bundle tree: `3d847390dfa74f8dced090164fb95f31eade83c7` both at introduction commit
  `4ff617ae123131a1c0152ad8fa42a46fbe1b305d` and at exact HEAD.
- Source commit/tree: `a5235340aae3c41cf227f5617e593b268936f6b3` /
  `071da0894065e465f43ea2204e09dd728e413ae2`; source is an ancestor of HEAD.
- Inventory: 75 regular files, 1,761,024 bytes; 74 entries in
  `SHA256SUMS.txt`; all 74 independently returned `OK`.
- Shape: 10 root records, 6 check records, 8 output captures, and 17 each in
  `stdout/`, `stderr/`, and `exit_codes/`.
- Exact 17 exits: `0,0,0,1,1,0,0,0,0,0,1,1,0,0,0,0,0`.
- Packet hashes for the checksum index, manifest, README, procedure, source
  workflow profile, runner manifest, and source contract test all recompute as
  stated.

| Predicate | Result | Independent witness check |
|---|---|---|
| P1 | `PASS` | cmd01/cmd02 exits `0/0`; scoped diff capture empty. |
| P2 | `PASS` | cmd03 exit 0; job `COMPLETED`; request/result validation diagnostics empty; 830 refs; exactly one constant-effort warning at mechanics diagnostic index 4 with the stated affected refs. |
| P3 | `PASS` | Regenerated/witness SHA-256 pair `b3cd85af…` / `c406d9c2…`, unequal as explicitly required. |
| P4 | `PASS` | cmd04 exit 1; `HEADLESS_RUNNER_LOAD_BASIS_MISSING`; null runner result. |
| P5 | `PASS` | cmd05 exit 1; `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`; historical witness mismatch preserved. |
| P6 | `PASS` | cmd06/cmd07 exits `0/0`; scoped diff capture empty. |
| P7 | `PASS` | Mechanics 1/1 named case matched, zero diagnostics; required witness hash matches. |
| P8 | `PASS` | Stress 3/3 named cases matched, zero diagnostics; required witness hash matches. |
| P9 | `PASS` | Nonlinear whole-suite default true, 5/5 matched, zero diagnostics; required witness hash matches. |
| P10 | `PASS` | cmd11 exit 1; benchmark-payload-missing diagnostic and required witness hash match. |
| P11 | `PASS` | cmd12 exit 1; regression-payload-missing diagnostic and required witness hash match. |
| P12 | `PASS` | cmd13/cmd14 both exit 0; checksum-bound logs present. |
| P13 | `PASS` | Before/after records show detached source and empty porcelain/diffs; cmd15-cmd17 output files are zero bytes. |
| P14 | `PASS` | Complete checksum inventory; manifest marks derivative/non-authoritative and `PASS`; README records `INTERNALLY_VERIFIED` and no owner acceptance. |
| P15 | `PASS` | Introduction commit changes the brief, bundle, sweep, W5 records, and three DEL-09-04 state surfaces; no receipt path; lifecycle unchanged; source-to-commit Remaining section byte-equal. |
| P16 | `PASS` | Evidence sweep and change-scope records pass; one sweep delta, 80 checked paths, zero scope violations; W5 return corroborates 507 piping tests, preserved first harness failure/cure, 311-pass rerun, and self-check PASS. |

PRD §24 R6 at blob `d07cc61c6e2889b059983fa74225757db8794e1d`
is actor-neutral exactly as the packet states; PRD §22.1 separately places
third-party reproduction in the publication-era lane. Acceptance of the exact
bundle is therefore not defective merely because an agent executed it, and it
does not create professional, external-prover, release, case-tier, tolerance,
GUI, lifecycle, or reliance effects.

The packet's currency analysis also passes. Direct source/HEAD comparisons
reproduce the stated changes in runner tree (`d5b2ae2c…` to `c349ee45…`),
product physics (`dcef46cd…` to `f609ba62…`), mechanics suite (`b93b556f…`
to `bfed462c…`), contract-test blob (`ba3864e8…` to `a84d089b…`), workflow
profile (`ab5768a7…` to `71feb5d6…`), and DAG pointer (`fdc6d39d…` to
`5441c712…`). The procedure, PRD, stress/nonlinear suites, and low-level
solver tree remain identical. Current local Git/Python/Rust/Cargo/macOS values
also reproduce the packet's environment-drift table. Qualified historic
acceptance O-B is consequently defensible; current-head reproduction is not
demonstrated.

## MAINTAINER_REVIEWED corpus review

| Claim class | Independent result | Direct basis |
|---|---|---|
| Tier law and owner boundary | `PASS` | Manual index blob `71984004dc6976d9d4f28c647b5b988ca21e210b` defines the tier and §9 checklist; strategy blob `56e4edc1132c854ebd514313f4bc8c4f969cd886` supplies witness/provenance requirements; DEC-027 identifies the sole human authority. |
| Complete registration/file/index inventory | `PASS` | Independent generator-module enumeration, filesystem enumeration, and index-link parsing each yield 64 with identical membership: mechanics 21, stress 15, nonlinear 28; no duplicates, missing targets, missing registrations, or extras. Generator `--check` returned `checked 64 case page(s)`. |
| Exhaustive 64 packet rows | `PASS` | Parsed all 64 table rows. Every case ID, page path/blob, note path/blob, suite, tier, evidence code, missing-basis code, and eligibility result matches the live generator registration and file bytes. |
| Current page tiers | `PASS` | All 64 have frontmatter `status: draft_evidence` and visible `DRAFT_EVIDENCE`. Generator hard-codes both and has no review-record field or promotion map. |
| Qualifying page review evidence | `PASS` | Repository searches and direct review of TP-E2, R13/R14, DEC-092, deliverable review, and findings surfaces found no owner/page/hash-bound checklist disposition. Existing evidence is useful reviewer input but not the missing act. |
| Machine-readable witness-chain claim | `PASS` | Strategy §§2/5 and manual §§7/9 require an authoritative machine-readable witness and deterministic human rendering. The current pages bind Markdown notes and fixture constructors/expected slots, but no review record establishes the required witness mapping. The packet correctly says the chain is not demonstrated rather than asserting unknown provenance as fact. |
| Page stale-runner census | `PASS` | Exact phrase `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` occurs in 63 pages; only DEC-092 omits it and records locked/offline crate-test-only reproduction. Current runner evidence proves the blanket future-stub wording is superseded. |
| Index stale-runner census | `PASS` | Manual index §§5 and 10 contain the two stated stale benchmark/regression-stub declarations. |
| Deliverable review evidence identities | `FAIL` | See finding `V-F02`; the packet swaps the two Git blobs. |
| Truthful-candidate preparation completeness | `FAIL` | See finding `V-F03`; MR-W1 omits a live aggregate authority contradiction while claiming to make the review candidate truthful. |
| Promotion gate | `PASS` | MR-A/B/C are non-binding and perform no promotion. Phase E requires exact later owner dispositions and a separate application gate. |

## Findings

### V-F01 — HIGH — DEC packet narrows the four-suite value request without a complete authority-blocker decision interface

The sealed author brief requires proposed final public-benchmark release
tolerance **values**, relative-plus-absolute comparison semantics and units,
and per-option admits/excludes across mechanics, stress, nonlinear, and sparse.
The packet correctly discovers that D-19/DEC-046 owns solver convergence while
DEC-024/026 owns result-versus-reference verification. It then makes O-A/O-B/
O-C exclusively active-set-count iteration-cap choices. Its own matrix records
`no effect` for mechanics, stress, legacy nonlinear comparison, and sparse;
§3 supplies only a proposed comparison algebra with no numeric per-kind value
table. Thus those options do not answer the four-suite comparison-value branch
of the request.

Escalating the contradiction is lawful; silently inventing cross-axis values
would not be. The defect is that §8 offers only a convergence ruling and a
`DEC-026 ... DEFER` statement. It does not present a precise owner choice with
consequences among:

1. rule only the strictly fenced DEC-046 convergence values now;
2. authorize a separate bounded DEC-026-derived public-comparison-policy
   preparation (current 25-fixture observable capture, quantity-kind units,
   relative values, near-zero absolute floors, and four-suite admits/excludes),
   without treating that preparation as a value ruling; or
3. defer both value lanes.

As written, an owner who intended the cross-suite comparison branch cannot
authorize its bounded preparation or rule values from this interface. The
packet therefore does not satisfy the user/brief merely by selecting the
narrower axis. Rerun or amend `A2-DEC046-CENSUS`; retain the verified numerical
tables and 13-case census.

### V-F02 — MEDIUM — MAINTAINER packet reverses two exact Git blob identities

`A2_MAINTAINER_REVIEW/PACKET.md` §3 states:

- `_REVIEW.md` blob `c7c997d487a5222b9daae789e69ccb8e83ab2bca`; and
- `Review_Findings.csv` blob `fb92cc1d66426aa512253e68c2fd259552f9623c`.

The live committed identities are the reverse:

- `_REVIEW.md` = `fb92cc1d66426aa512253e68c2fd259552f9623c`;
- `Review_Findings.csv` = `c7c997d487a5222b9daae789e69ccb8e83ab2bca`.

Their substantive characterization is correct (`_REVIEW.md` is the dated
compatibility/checking review; the CSV is header-only), but an owner packet
that promises exact evidence identities cannot ship the swapped pointers.

### V-F03 — MEDIUM — MR-W1's truthful-candidate scope omits active-DAG staleness

The packet identifies 63 stale page statements and two stale runner statements
in the manual index, then Phase A says it will correct **only** those statements
to make the review candidate truthful. But the same controlling aggregate
surfaces carry a second direct currentness contradiction:

- `docs/validation_manual/index.md` lines 29–31, blob
  `71984004dc6976d9d4f28c647b5b988ca21e210b`, says DAG-007 is the current
  approved graph authority;
- `docs/VALIDATION_STRATEGY.md` lines 22–23, blob
  `56e4edc1132c854ebd514313f4bc8c4f969cd886`, likewise names DAG-007; while
- `execution/_DAG/_LATEST.md`, blob
  `5441c7127aceecdefe242bef25e5ca9cd5a330b4`, names DAG-009 approved and
  DAG-007 superseded.

MR-W1 should either include the smallest lawful currentness correction/routing
needed for the review basis or explicitly exclude and disposition this stale
aggregate authority text in every per-page review input. Adopting a plan that
claims truthful final candidates while leaving an unmentioned live authority
contradiction is not owner-ready.

## Future-integration and owner-gate review

- No packet applies a decision, threshold, acceptance, review act, promotion,
  register disposition, Remaining edit, receipt, lifecycle transition,
  reliance change, or release act.
- R14 correctly routes its half only to later fan-in with the separate value
  disposition before TASK_MANAGEMENT may act on `TM-PIP-037`.
- The current first DEL-09-04 Remaining bullet is the only case-page/GUI/
  storage residual; the second is the tolerance gate. No packet proposes
  changing another Remaining surface.
- GUI evidence, export/CAEPIPE, D-61 reliance, lifecycle, publication/release,
  and professional acceptance remain fenced.
- After `V-F01` is repaired, the manager must ensure the exact owner interface
  states which axis a ruling governs and which of the first two Remaining
  bullets, if either, becomes eligible for a bounded later edit. Preparation,
  a ruling, and application must remain separate acts.

## Containment and repository-state checks

| Check | Result |
|---|---|
| Exact branch/HEAD | `PASS` — branch and commit match the sealed brief. |
| Tracked working/index delta | `PASS` — both empty; `git diff --check` and cached equivalent pass. |
| Receipt identity | `PASS` — `loop/LOOP_RECEIPTS.md` SHA-256 `73e71b3751fc8db16b947cd7199e0c807f59694c327796d2df901420e588e0f9`, Git blob `8371276b81d3eeb7b78181a6279d80d43115e10a`, unchanged from base. |
| Untracked containment | `PASS` — every untracked path is under the declared run root. |
| Ignored drift | `PASS` — no ignored path is reported. |
| Whitespace/diff cleanliness | `PASS` before this allowed return write. |
| Unsupported claim scan | `PASS` — no fabricated owner/maintainer act, release/reliance/professional/code-compliance claim, TBD laundering, or hidden promotion found. |
| Verifier write containment | `PASS` — only this `instances/A2_VERIFY/RETURN.md` was created. |

## Residual caveats

1. The 13 blocked mechanics rows are an exact July 20 record, not a proved
   present-day whole-suite total; current source strongly implies a fourteenth
   DEC-092 block but no current committed run establishes it.
2. R14 qualified acceptance can close only the historic/source-pinned
   reproduction disposition. It cannot assert current-head reproduction; the
   explicit rerun triggers have fired.
3. No case page is presently promotable. Substantial suite and derivation
   evidence does not substitute for the missing owner/hash-bound review act or
   the unestablished machine-readable witness chain.
4. Fixing `V-F02` and `V-F03` does not itself create review evidence or promote
   a page. Fixing `V-F01` does not rule a value; it makes the owner gate
   decision-ready.

## Attestation

- I did not delegate, spawn, or use another agent.
- I independently read the three packets and returns and checked their claims
  against live committed evidence at the exact sealed base.
- I wrote only this verifier `RETURN.md`.
- I did not repair an author file or modify a brief, status, evidence, code,
  fixture, case page, manual, register, receipt, lifecycle, decision, or Git
  state.
- I performed no stage, commit, push, fetch, merge, branch, reset, rebase,
  clean, deletion, network, external, or destructive action.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
