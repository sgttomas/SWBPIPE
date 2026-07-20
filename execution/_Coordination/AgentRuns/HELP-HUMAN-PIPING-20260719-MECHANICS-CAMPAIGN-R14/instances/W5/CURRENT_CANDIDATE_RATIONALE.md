# W5 D-54 Reasoned-Selection Rationale — DEL-09-04 Clean-Checkout Reproduction at the R14 Head

**Run:** HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W5
**Author:** WORKING_ITEMS (Agent 1, W5 reproduction-wave manager)
**Brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_DEL-09-04_CLEAN_REPRO_R14.md` (`CB-2026-07-20-DEL-09-04-CLEAN-REPRO-R14-001`)
**Date:** 2026-07-20

## 1. Sources Opened (live tree at head `a5235340a`, branch `claude/piping-r14-w5-clean-repro`)

- The current procedure `docs/validation_manual/headless_runner_reproduction.md`
  read in full at head (Part 1 fixture table with the dated 2026-07-19
  case-3 and 2026-07-20 case-1 notes; Part 2 bound fixture set, expected
  exits 0/0/0/1/1; Review Checks; Rerun Consequence), SHA-256 pinned in
  the brief.
- The R11 precedent brief
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`
  (execution mechanics §3, bundle layout §3.5, profile checks §4.2,
  failure disposition §7, rerun clause §8) and the completed R11 bundle
  `validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`
  (manifest schema, checks inventory, README claim posture).
- W4 chain evidence for the current case-1 head behavior:
  `instances/W4/T7/EXECUTE_RETURN.md` (live offline head run — exit 0,
  `COMPLETED`, 830 `result_refs`, exactly one
  `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning at
  `mechanics_envelope.diagnostics`, regenerated digest `b3cd85af…8613` vs
  committed witness `c406d9c2…5188`) and `instances/W4/RETURN.md`
  (W4-C7 frozen-surface byte-identity; sweep-deviation record).
- The eight committed witnesses and two generators, hashed directly at
  head (digests pinned in brief §3); the frozen solve input's
  `supports[6]` (`support:CE-120`, `restraints: []`).
- DEL-09-04 `_STATUS.md` (IN_PROGRESS; two owner-gated Remaining bullets;
  History showing R11/R13/W4-T7 lineage), `MEMORY.md`, and the R11
  run-record lineage.
- D-52/`DEC-085` overlay, D-54/`DEC-087` packet (ten fast-reject classes,
  §3.2 eligibility, §3.3 attribution), DEC-080 evidence home, DEC-081
  claims taxonomy, DEC-065 exit policy.
- The R12 environment-repair disposition
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/ENVIRONMENT_REPAIR_DISPOSITION.md`
  (R12-ENVREPAIR-01 precedent, bounded §4.4 contingency only).
- `software-workflow.json` (path rules selecting piping-pytest,
  evidence-sweep, harness-pytest, harness-self-check for
  `validation/**` + `execution/**` writes), `loop/LOOP_RECEIPTS.md`
  cursor `Receipt-63`, and the R14 `ORCHESTRATION_PLAN.md`.

## 2. Limits Screen (D-54 §3.1 ten classes, run first)

1. **Irreducible personal preference/identity** — not touched; the shape
   is fully project-derived (manual + R11 precedent + DEC-080).
2. **Material purpose/scope boundary change** — none; the reproduction
   re-executes the already-documented procedure; no scope inclusion or
   exclusion moves.
3. **New normative content / new-changed acceptance criteria** — none
   created; every predicate restates the live manual's own documented
   expectations and committed witness bytes; the bundle is labeled
   evidence, never a criterion.
4. **Professional/safety/legal/fiduciary accountability** — not
   exercised; the bundle README restates that acceptance and professional
   judgment remain with the responsible engineer.
5. **Spending/procurement/external commitment** — none; offline,
   local-only, no network.
6. **Publication/release/issuance/reliance acceptance** — none; no push,
   PR, merge, receipt, or release act by any W5 node.
7. **Protected/private-data exposure** — none; invented public-metadata
   fixtures only.
8. **Destructive/irreversible/history-rewriting action** — none; new
   immutable bundle only; prior bundles/witnesses byte-preserved; cleanup
   limited to the validated task-created temp root.
9. **Lifecycle/stage advancement, reproduction acceptance,
   evidence-posture promotion, other D-52/D-49 limits** — all preserved:
   `INTERNALLY_VERIFIED` label only; no Remaining row struck; lifecycle
   `IN_PROGRESS` unchanged; DEC-046/D-45/D-06b untouched.
10. **Unavailable/stale evidence, authority conflict, claim beyond
    warrant** — the trigger and every predicate are anchored to the live
    manual, committed witnesses hashed at head, and preserved W4 chain
    evidence; the one head-behavior expectation (case-1 warning) was
    live-verified by the W4 T7 executor and independently re-run by its
    verifier at the same solver surface this head carries.

No fast-reject hit. Ambiguity check: the W5 dispatch names exactly this
rerun (R11 §8 clause) and directs a NEW brief because the R11 predicates
are stale; the NO-OP alternative is refuted with evidence (§4 item 5).

## 3. Four-Lens Analysis

- **Ontology.** The object is a derivative evidence package — a record of
  what one clean checkout of one pinned commit does under the documented
  procedure. It is not decomposition truth, not a witness change, not an
  acceptance. The prior bundle lineage (one `PASS` plus preserved
  `FAIL`/`BLOCKED` records) establishes the kind; this run adds a sibling
  pinned to the R14 head, keeping the historical bundles distinct and
  truthful for their own commits. The stale-vs-current
  predicate split (R11's pins vs the manual's dated notes) is carried by
  a NEW brief so no historical document is reinterpreted.
- **Epistemology.** Every expected exit and diagnostic has a named
  warrant: the manual's current tables and dated notes; the five del1005
  witness byte-pins (hashed directly at the source commit); the W4 T7
  live head run for the non-diagnostic-clean case-1 shape. Where warrant
  is thinner (case-2 byte relationship; the exact regenerated case-1
  digest) the brief records observations instead of predicates — the
  claim never exceeds its warrant. Byte-comparison for the five bound
  cases is warranted by the R12 chain having reproduced those witnesses
  and by the W5 dispatch requiring it; a mismatch is a truthful FAIL,
  not a softened note.
- **Praxeology.** Reusing the R11 execution mechanics verbatim (clone,
  cleanliness proofs, offline posture, capture discipline, bundle layout,
  checksum-last) gives operators one stable reproduction pattern across
  runs; re-anchoring only the predicates minimizes drift surface. The
  fail-closed ordering (predicates before state updates before checks
  before commit) and the strike-no-rows rule make the bundle unable to
  outrun its evidence or its authority. The §4.4 contingency is bounded
  to the one failure class with recorded precedent, keeping the executor
  from improvising environment repair.
- **Axiology.** Truthfulness of committed history (prior bundles,
  witnesses, R11 brief) preserved untouched; evidence over narrative
  (separate argv/stdout/stderr/exit captures, hashes, predicate
  verdicts); human responsibility conserved (INTERNALLY_VERIFIED only,
  adoption basis and no-owner-acceptance stated in the README, owner
  gates enumerated and untouched); claim posture within DEC-081.

## 4. Materially Important Alternatives (rejected, with reasons)

1. **Rerun under the R11 brief unchanged.** Rejected: its pinned case-3
   diagnostic (`HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`)
   and diagnostic-clean case-1 predicate are contradicted by the current
   manual and head solver; executing stale predicates would manufacture a
   false FAIL or force in-flight reinterpretation. The R11 brief §8
   itself demands a fresh basis.
2. **E1-only reproduction (three cases).** Rejected: the current manual
   documents Part 2 as part of the reproduction surface; the R6-criterion
   value of the rerun is coverage of the CURRENT procedure, and the W5
   dispatch names all documented cases.
3. **Defer to a later, quieter head.** Rejected: the R14 head is merged
   main, the natural SOURCE_COMMIT named by the campaign; deferral leaves
   the R6 criterion without current-head evidence for no gain.
4. **Regenerate/refresh the committed solve witness while at it.**
   Forbidden: frozen surface, DEL-10-05 lane, and it would convert an
   evidence tranche into a witness change.
5. **Truthful NO-OP.** Refuted: the R11 §8 rerun clause fired on three
   independent rows (procedure, runner, solver) and no current-head
   bundle exists; the newest completed bundle is pinned to pre-#287
   commit `23eeaabc9`.
6. **Predicate-only comparison without witness byte-checks for Part 2.**
   Rejected as weaker: the committed witnesses are byte-identical at head
   and were reproduced in the R12 chain, so byte-equality is the
   strongest available oracle; dropping it would discard warranted
   evidence. (Conversely, byte-equality is NOT imposed where the manual
   itself declares divergence — cases 1 and 3 — keeping claims inside
   warrant.)

## 5. Selected Outcome

One new brief (`CB-2026-07-20-DEL-09-04-CLEAN-REPRO-R14-001`); one clean
local-only clone at `a5235340aae3c41cf227f5617e593b268936f6b3`; all eight
documented cases with current dated expectations (P1–P13); one immutable
`INTERNALLY_VERIFIED`-only bundle under
`validation/evidence/reproduction/<RUN_ID>/`; single evidence-sweep with
one-file delta; strike-no-rows state handling (History/MEMORY/run-record
only); W5 chain artifacts; manager-committed, no push/PR/receipt.

## 6. Attempted Failure Mode (adversarial self-check)

Attempted: read the P7–P11 witness byte-equality predicates as promoting
the witnesses into acceptance criteria or release thresholds. Fails: the
witnesses are already the manual's documented comparison basis ("outputs
byte-compared to committed witnesses" is regression evidence under
DEC-081); no tolerance, threshold, or release judgment is created and
DEC-046 stays untouched. Attempted: classify the §4.4 lockfile
contingency as provisioning/installation. Fails: it copies only
already-present gitignored local state from the same machine's primary
tree, offline, with recorded evidence — the exact class R12-ENVREPAIR-01
dispositioned as no-network and reversible; anything outside that class
remains a truthful BLOCKED. Attempted: treat the reproduction PASS as
discharging a Remaining bullet (as R11 did). Fails: the R11-era bullet no
longer exists; both current bullets are owner-gated and the brief
hard-requires their byte-identity — the run produces evidence, not
closure.

## 7. Classification

`CLASSIFY_ELIGIBLE`; `ACTIVATE_OWNER_STANDING_APPROVAL`;
`AgentJudgment: SELECT_AND_ADVANCE`; `OwnerCaseSelection: NONE`;
`EffectStatus: HELD` pending fresh-context independent refutation.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
