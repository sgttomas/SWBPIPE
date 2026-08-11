# STRUCTURED RETURN — A2-VERIFY-V2

## Status

`PASS`

The replacement DEC-046/DEC-026 packet, replacement MAINTAINER_REVIEWED
promotion-basis packet, and unchanged R14 acceptance packet are owner-ready
for the exact preparation slate. All three original verifier findings are
cured. I found no new technical, evidence-identity, authority, inventory,
owner-interface, or containment defect.

## Identity and parentage

| Field | Value |
|---|---|
| RunID | `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES` |
| ParentInstanceID | `WORKING_ITEMS/Agent1/working_items_del0904_owner_gates_prepare` |
| ChildInstanceID | `A2-VERIFY-V2` |
| Agent form | fresh non-delegating ephemeral Agent 2 generalist |
| Exact branch | `codex/piping-del0904-owner-gates-20260810` |
| Exact HEAD | `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3` |

## Reviewed-output identities

| Output | Recomputed SHA-256 |
|---|---|
| `instances/A2_DEC046_REMEDIATION/PACKET_V2.md` | `1265e843c2c33eaa915f26cba5b75b72e811b7bf2bcc280bb90bcb02ecc5178c` |
| `instances/A2_DEC046_REMEDIATION/RETURN.md` | `2c66f1b91928937c77d22ce2f7bf85c512caf06bd1a230c50c2a0ded97ce245a` |
| `instances/A2_MAINTAINER_REVIEW_REMEDIATION/PACKET_V2.md` | `ee08a4af9cfb99ba624a2ab510f2c95d48484172a562f001bb60aa1eac9d8565` |
| `instances/A2_MAINTAINER_REVIEW_REMEDIATION/RETURN.md` | `bd3edb9a969ca06b66bbbd4e70f3a6afd7ac47f735605bcd1565333be5bb085e` |
| `instances/A2_R14/PACKET.md` | `8218b4566c4e3d4476ee7f29db4b7e96ae8740c32273bc9b7c4102b86172335d` |
| `instances/A2_R14/RETURN.md` | `d5fb2d6c520829068425b37ace53d0f3d3f34568fa02fa3df1fb91708d2c8106` |

The two R14 hashes remain exactly those independently accepted by the original
verifier; no R14 author byte changed during remediation.

## Original-finding cure table

| Original finding | Independent V2 result |
|---|---|
| `V-F01` — incomplete four-suite value/authority interface | `CURED`. The replacement makes convergence Gate C and verification Gate V separate top-level owner acts. Gate C offers C-A/C-B/C-C/C-D with exact changed-support-count semantics and caps. Gate V offers concrete V-A/V-B/V-C tables, measurement-only V-D, and defer/decline V-E. The owner can rule convergence-only, separately authorize or select comparison policy, or defer without cross-axis promotion. |
| `V-F02` — swapped `_REVIEW.md` and `Review_Findings.csv` blobs | `CURED`. `_REVIEW.md` recomputes to Git blob `fb92cc1d66426aa512253e68c2fd259552f9623c`, SHA-256 `d7fcdb41c34e3b65af17d3b542217443ed619a241bae8b51a023e51242f01236`; `Review_Findings.csv` recomputes to Git blob `c7c997d487a5222b9daae789e69ccb8e83ab2bca`, SHA-256 `08dd7619787bf7f848ef0d76b718d84eebcb478908dc0fae51c58d3b89ce6130`. |
| `V-F03` — incomplete truthful-candidate scope | `CURED`. MR-W1 Phase A covers the generator/63 page statements, the two runner-stub statements in the manual index, and both stale aggregate-authority paragraphs. It cites `_DAG/_LATEST.md` blob `5441c7127aceecdefe242bef25e5ca9cd5a330b4` (`DAG-009` active, `DAG-007` superseded) and `_Decomposition/_LATEST.md` blob `45de7c70c7c7441d1fcf73f670da8433a1c8e57b` (revision `0.11` current), while requiring a delta check or historical framing for the strategy's revision-`0.7` alignment statement. It forbids mutation or reinterpretation of authoritative decomposition/DAG truth. |

## DEC-046 / DEC-026 independent numeric recomputation

### Committed populations

- Mechanics source capture recomputes to 24 requested, 11 matched, zero
  mismatched, 13 blocked, 200 recorded values: 91 observable and 109
  unreachable. Strict equality admits 78/91 values and 7/11 complete cases;
  all 13 nonzero deltas occur in `MECH-TP-PHYS-004/005/006/007`.
- Mechanics maximum absolute delta is
  `3.552713678800501e-15`. Per-kind recomputation matches the replacement:
  displacement `1.734723475976807e-18 m` absolute and
  `1.2390881971262908e-16` relative; force
  `1.7763568394002505e-15 N` absolute and
  `4.440892098500626e-16` relative; moment
  `3.552713678800501e-15 N-m` absolute,
  `1.7763568394002505e-15 N-m` at a zero reference, and
  `1.7763568394002505e-15` relative; rotation
  `8.673617379884035e-19 rad` absolute and
  `2.0016040107424698e-16` relative. Count, ratio, length, and linear
  stiffness observable members are exact. Stress has 11/11 exact values in
  3/3 cases.
- Nonlinear regression has 5/5 exact numeric and categorical projections,
  with two recorded converged and three recorded nonconverged. The accepted
  assembled observations support 9 seed plus 13 multi-support cases; the
  cascade reaches iteration 3 and the other accepted multi-support cases
  reach at most iteration 2. The live consumer uses
  `max(residual_tolerance, absolute_residual_floor)` and inclusive iteration
  and changed-support-count comparisons.
- Sparse DEC-053 recomputes to 9 observations with maxima: relative parity
  `7.060341894958857e-11`; absolute sparse/dense solution delta
  `5.342535303043405e-10`; absolute residual
  `1.0058283805847168e-7`; repeat delta `0`; nonpositive pivots `0`.

### Option tables and admissions

- C-A (`0/0` count, caps `2/2/2/2/3`), C-B (`0/0`, cap `4`), and C-C
  (`0/0`, cap `6`) each admit all 22 accepted assembled observations. C-A
  is the observed envelope, C-B adds direct bounded headroom and preserves
  accepted validation values, and C-C adds unevidenced cap headroom. None
  affects mechanics, stress, exact regression, or sparse comparison results.
- V-A values equal the measured relative maxima and zero-reference absolute
  floors. V-B is exactly 10 times every nonzero V-A member with zeros
  preserved. V-C is the disclosed `1.0e-9` relative/internal-absolute
  precedent projection with exact discrete semantics. Re-evaluation of every
  observable mechanics value under the stated inclusive formula admits
  91/91 and 11/11 cases for V-A, V-B, and V-C; stress admits 11/11 and 3/3;
  nonlinear numeric/categorical regression admits 5/5; sparse admits 9/9 on
  each option's declared subcriteria.
- The options are materially different future envelopes even though the
  current observed pass counts do not distinguish them. The packet correctly
  exposes V-A inferred zero floors, V-B's owner-selected (not measured)
  headroom factor, V-C's governance inference, V-A/V-B `TBD` values for
  blocked-only force-per-length and mass-per-length, and V-C's absent
  absolute sparse/dense parity member.
- The formula, finite/fail-closed behavior, equality boundary, relative and
  unit-bearing absolute members, zero-reference behavior, and exact
  categorical/count behavior are explicit. `INTERNAL_ASSERTION_EPSILON =
  1.0e-9` remains identified as mechanics/stress crate-internal absolute
  comparison precedent, not existing public policy.

### Exact blocked-case census and currency

The packet's 13 IDs have an empty symmetric difference from the committed
historic capture. Their value counts and quantity-kind summaries recompute
exactly. `benchmark_binding.rs` has 11 mechanics mappings and a fail-closed
`CaseEvaluation::NotReusable` catch-all; each blocked row therefore has null
observed/delta/predicate fields before any tolerance is evaluated. Primary
classification is exactly implementation=`13`, data=`0`, tolerance=`0`,
fixture=`0`; every C/V option compares 0/109 blocked values and unblocks 0/13.
The packet correctly limits that exact census to the historic 24-fixture
capture and labels a likely fourteenth current DEC-092 block as inference:
the current source has 25 fixtures, the binding has no DEC-092 arm, and no
current whole-suite capture proves the inferred count.

## MAINTAINER_REVIEWED independent inventory and authority review

- Generator registrations, generated files, and unique index case links each
  recompute to exactly 64 with identical membership: mechanics 21, stress 15,
  nonlinear 28. Missing, extra, duplicate, unindexed, or unregistered counts
  are all zero. Non-writing generator `--check` reports 64 checked pages.
- All 64 pages have frontmatter `draft_evidence` and visible
  `DRAFT_EVIDENCE`. The replacement's 64-row inventory has identical case-ID
  membership; all 128 page/note `HEAD:path` Git-blob identities recompute
  exactly. Evidence/missing-basis counts are 63 `E63` + `M1+M2+M3` and one
  DEC-092 `E92` + `M1+M2`; review=`NONE` and eligibility=`NO` for all 64.
- The superseded runner diagnostic phrase occurs exactly once in each of 63
  pages; DEC-092 is the sole omission. The manual index has the two stated
  stale runner-stub declarations. Exactly the two controlling aggregate
  surfaces (`docs/validation_manual/index.md` and
  `docs/VALIDATION_STRATEGY.md`) name stale `DAG-007`; the authoritative
  pointer names `DAG-009` current and `DAG-007` superseded.
- The replacement contains 148 unique 40-hex identities; all resolve (146
  blobs, two commits). Direct source inspection supports zero qualifying
  page/hash-bound owner-maintainer review records. Passing tests, deterministic
  generation, delivery review, R13/R14, and DEC-092 evidence are correctly
  treated as reviewer inputs rather than fabricated review acts.
- MR-W1 Phase A is bounded and complete: it prepares truthful documentation,
  witness-chain mapping, and review inputs, but neither performs the owner
  checklist/dispositions nor promotes any page. Phases C/D preserve the
  page-specific owner act, and Phase E remains separately authorization-gated.
  MR-A/MR-B/MR-C are therefore lawful preparation choices with zero immediate
  promotions.

## R14 continued-validity spot-check

- The R14 packet/return hashes are unchanged. The bundle has 75 regular files
  and 1,761,024 bytes; all 74 `SHA256SUMS.txt` entries independently verify.
  Its Git tree is `3d847390dfa74f8dced090164fb95f31eade83c7` both at introduction
  commit `4ff617ae123131a1c0152ad8fa42a46fbe1b305d` and at the sealed base.
  Source `a5235340aae3c41cf227f5617e593b268936f6b3` remains an ancestor.
- `validation_summary.json` and the direct committed witnesses still record
  P1-P16 `PASS`. Independent spot-checks reproduced the exact 17 exits
  `0,0,0,1,1,0,0,0,0,0,1,1,0,0,0,0,0`, zero-byte scoped-diff and final
  cleanliness outputs, P2's sole constant-effort warning and 830 refs,
  P4/P5 fail-closed diagnostics, P7 mechanics 1/1, P8 stress 3/3 and 11 exact
  values, P9 nonlinear 5/5, and byte-identity for P7-P11 required witnesses.
- PRD §24 R6 remains actor-neutral. O-B correctly accepts only the immutable
  source-pinned historic result while retaining `INTERNALLY_VERIFIED`. The
  runner/dependencies, product-physics, mechanics inventory, contract test,
  workflow profile, DAG pointer, and environment/toolchain changes remain
  explicit currency triggers. Acceptance does not assert current-head
  reproduction or create tolerance, page-tier, GUI, professional, reliance,
  lifecycle, external-prover, release, or TM-PIP-037-closure effects.

## Owner interfaces, future application, and exclusions

Each packet stops before its owner gate and gives copyable bounded options and
a non-binding recommendation. No packet fabricates an owner act. A later
TM-PIP-037 disposition remains conjunctive on the exact DEC value ruling and
R14 ruling; TASK_MANAGEMENT and a separate application authorization remain
required. DEL-09-04 Remaining edits are limited to what exact owner rulings
authorize, and no packet proposes changing unrelated text. GUI-workflow
evidence, export/CAEPIPE, D-61 reliance, lifecycle, publication/release,
professional acceptance, case repairs, and merge remain fenced.

## Transient wrong-checkout write audit

The remediation return transparently records that it briefly created an
untracked `PACKET_V2.md` at the same run-relative path in the primary checkout
and immediately removed it with `apply_patch`. I independently inspected the
exact primary path:

`/Users/ryan/.codex/worktrees/619f/chirality/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/instances/A2_MAINTAINER_REVIEW_REMEDIATION/PACKET_V2.md`

Terminal result: the path is absent, is not a symlink, is not tracked, and
produces no path-scoped Git status entry. Because the path did not exist in
the committed base and terminal inspection finds no filesystem or index
object there, there is no residue and no evidence of modification,
replacement, deletion, staging, or other impact on pre-existing state. The
incident remains a process nonconformance preserved in the author return; it
does not create a terminal containment defect or block PASS.

## Containment and attestation

- Exact branch/HEAD pass. Receipt ledger is unchanged at SHA-256
  `73e71b3751fc8db16b947cd7199e0c807f59694c327796d2df901420e588e0f9`
  and Git blob `8371276b81d3eeb7b78181a6279d80d43115e10a`.
- `git diff --check` is clean; the index has no staged paths; ignored drift is
  zero. Every untracked path in the isolated worktree is within the declared
  run root.
- I wrote only this `instances/A2_VERIFY_V2/RETURN.md` via `apply_patch`. I
  performed no repair, project-truth edit, ruling, promotion, register or
  Remaining mutation, receipt append, stage, commit, fetch, push, merge,
  rebase, reset, clean, deletion, network action, or external action.
- I did not delegate or spawn another agent.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
