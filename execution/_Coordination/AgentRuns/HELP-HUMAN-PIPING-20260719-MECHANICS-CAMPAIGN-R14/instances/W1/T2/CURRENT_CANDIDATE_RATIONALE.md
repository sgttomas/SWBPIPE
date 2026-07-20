# T2 Rationale — DEL-04-03 Constant-Effort Spring-Hanger Solve Behavior

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T2
**Author:** WORKING_ITEMS (Agent 1, PKG-04 package manager)
**Candidate brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T2_DEL-04-03_CONSTANT_EFFORT_SOLVE.md`
**Lane:** D-54/`DEC-087` reasoned selection on the D-52/`DEC-085` overlay
**Date:** 2026-07-19

**Amendment record (v2):** the v1 fresh-context verifier
(`instances/W1/T2/VERIFY_BRIEF.md`, preserved) returned `BLOCK`, refuting
C6's consequence clause, C8, and C11: the pinned fixture's constant-effort
support declares `"restraints": []`, so the v1 fail-closed rule would have
blocked the documented reproduction case; the v1 fence made the default
benchmark path self-defeating (README mirrors the inventory and was
non-writable); and the DEC-049 "D6 remains the owner" annotation was
unsurfaced. The brief was amended (v2): data-driven opt-in consumption
with a non-blocking non-consumption warning (accepted inputs stay valid;
pinned case keeps exit 0); one additive README inventory line permitted;
the reproduction-manual case-1 staleness recorded as a HELP_HUMAN
follow-on per the R12→R13 precedent; the DEC-049 annotation surfaced with
its rehomed-row reading. D-54 basis for the amended selection: data-driven
opt-in is the one project-grounded outcome that adds the directed solve
behavior without retroactively invalidating any accepted input shape;
the material alternatives (blocking on incomplete data; consuming with an
inferred direction) are rejected in §3 items 8–9 below. Claims C6, C8,
C11 are restated in v2 form and C16–C18 added.

**Amendment record (v3):** the v2 verifier
(`instances/W1/T2/VERIFY_BRIEF_V2.md`, preserved) returned `BLOCK` on one
residual fact — `validation/hand_calcs/mechanics/README.md` is a second
fixture-inventory mirror the v2 fence excluded — plus stale v1 wording in
§10 (`SelectedOutcome` "fail-closed"; verifier pointer). The brief was
amended (v3): one additive per-fixture inventory line permitted in each
of the two mirror READMEs (truthful listings only), §6 verifies both
mirrors with exactly-one-additive-line diffs, and the §10 wording now
states the opt-in rule. C8 is restated in v3 form below (its v2 text is
preserved history in the verifier returns).

## 1. Ten-Class Fast-Reject Screen (D-52 §4.1, item by item)

1. **Irreducible owner preference / choice with two defensible outcomes as
   a stop condition:** not hit. The direction-convention and evidence-shape
   choices are design multiplicity handled under D-54 reasoned selection
   (§3 below); the work item itself is unambiguous recorded deliverable
   scope with a governing ruling (DEC-049 Option B) that expressly
   anticipates a later assembled-solve tranche.
2. **Professional/safety/legal/fiduciary/hazard accountability:** not hit.
   Ideal-element preview mechanics on invented fixtures; the review rows,
   professional-boundary flags, and human-review posture are preserved; no
   reliance or compliance outcome.
3. **Conflict ruling not determined by the authority chain:** not hit.
   DEC-049 and the Remaining row agree; no contrary record exists. The
   review-row disclosure text update is a truth-maintenance edit following
   the landed behavior, not a conflict resolution.
4. **Scope/boundary change, new normative content, new acceptance
   criteria:** not hit. The Remaining row IS the scope; the DEC-026
   analytic comparison tier is reused as already recorded; the user-limit
   warnings compare against user-entered values only — the software
   introduces no constant, threshold, or criterion.
5. **Lifecycle/stage/issuance/release/acceptance/evidence-posture act:**
   not hit. Lifecycle stays `IN_PROGRESS`; suite claim posture unchanged;
   no promotion of any kind.
6. **Third-party/procurement/spending/publication/external action:** not
   hit. Local, offline; no push/PR/merge in the tranche.
7. **Merge/integration authority over accepted baseline, destructive
   action:** not hit. Ordinary commits on the wave branch; frozen witness
   files untouched (the pinned fixture's OUTPUT change is a lawful
   physics-behavior consequence, recorded before/after; the committed
   historical witnesses are not edited).
8. **Protected/private data exposure:** not hit. DEC-049's
   no-catalog/no-protected-values exclusions are acceptance predicates and
   fence exclusions; all fixtures invented.
9. **Evidence unavailable / stale basis / claim beyond warrant:** not hit.
   Solve-gap, normalization, fixture-exposure, and suite-exposure facts
   were verified live at preparation (including that no benchmark or
   del1005 surface contains a constant-effort support); symbol-level
   references immunize the brief against T1's serialized additive edits;
   the executor freeze-check re-verifies.
10. **Protected domain-engine paths, prover activation, higher-order
    boundaries:** not hit.

**Screen result: PASS.** DEL-04-03's three `TBD` prerequisite rows were
examined under class 9: the targets (DEL-04-01, DEL-02-01, DEL-02-02) hold
committed implementation evidence consumed read-only, the DEC-049 slice
landed under the identical posture, and R14 selection authority is HUMAN;
the brief records the posture truthfully and resolves no row.

## 2. Four-Lens Analysis

- **Ontology.** The constant-effort support already exists as a governed
  model entity with user-entered data and review evidence; the tranche adds
  its missing solve consumption as the ideal constant-force element — the
  standard analysis idealization — without creating any new entity class,
  input schema, or authority. Review evidence, solve consumption, and
  acceptance remain distinct things.
- **Epistemology.** The gap (filtered out of `BuiltModel.supports`; review
  rows expressly disclaim solve consumption) is verified in source. The
  physics is elementary statics (superposition of a constant nodal force),
  witnessed by a closed-form hand calc and unit tests; recorded benchmark
  values stay regression evidence under the suite's existing claim posture.
  No claim exceeds that warrant.
- **Praxeology.** Entering the force at per-load-case assembly gives one
  seam consumed identically by dense, sparse, and nonlinear paths —
  bounded, deterministic, fail-closed on ambiguous input, with explicit
  disclosure of the direction convention and truthful before/after capture
  of the one pinned fixture whose output lawfully changes.
- **Axiology.** Advances the adopted mechanics program (physical-model
  correctness) and PRD D5 scope while preserving DEC-049's protective
  exclusions, the claim fence, human review, and reversibility. No value
  hierarchy is created; user data remains the only source of magnitudes.

All four lenses support the same bounded outcome: author and advance this
tranche brief through the governed verify→execute→verify chain.

## 3. Materially Important Rejected Alternatives

1. **Gravity-opposing direction inference** (direction = −ĝ from the
   model's declared gravity). Rejected: couples support behavior to a
   separate input surface, creates a hidden inference the user never
   entered, and fails ambiguously for load cases without gravity data; the
   declared-DOF positive-axis convention is explicit, disclosed, and
   fail-closed.
2. **Signed-value reinterpretation of `constant_load`.** Rejected: the
   landed slice validates/reviews positive force magnitudes; silently
   accepting signed values would change the meaning of already-recorded
   user data.
3. **Stiffness-element emulation** (very stiff spring plus imposed load).
   Rejected: numerically polluting and physically false for an ideal
   constant-effort device; the constant-force treatment is exact.
4. **Nonlinear-path implementation** (constant-effort as a nonlinear
   support class). Rejected: constant force with zero stiffness is
   load-case-static; the assembled force vector already reaches the
   nonlinear loop; a nonlinear class would expand DEL-04-04's owner-gated
   surface for no modeling gain.
5. **Omit the benchmark-suite fixture** (hand-calc + unit tests only).
   Rejected as the default (kept only as the recorded §3.7 fallback): the
   mechanics program's evidence convention pairs hand-calc witnesses with
   suite fixtures, and the additive fixture is lawful under the existing
   claim posture.
6. **Defer despite no defect.** Rejected: owner-directed queue, satisfied
   evidence basis, no supporting risk record.
7. **Park because the pinned fixture output changes.** Rejected: the
   committed witnesses are already historical for pinned pre-#287 commits;
   refusing every solver-behavior improvement that changes solve output
   would freeze the mechanics program; the change is captured before/after
   and surfaced to HELP_HUMAN.
8. **(v2) Blocking diagnostics for incomplete constant-effort data.**
   Rejected: the accepted pinned fixture legitimately carries a
   constant-effort support with empty restraints under the landed DEC-049
   data slice; converting that accepted shape into a blocking failure
   would retroactively invalidate governed inputs and falsify the
   documented reproduction case — the non-consumption warning preserves
   both truthfulness and the accepted input contract.
9. **(v2) Consume with an inferred direction when restraints are empty.**
   Rejected: any inferred direction (gravity-opposing or axis default) is
   a hidden default DEC-049 forbids; opt-in consumption keeps the user
   the sole source of direction.

## 4. Enumerated Refutable Claims (for the fresh-context verifier)

- C1. DEL-04-03 `_STATUS.md ## Remaining` contains exactly one item whose
  text matches the brief's selected item, and lifecycle is `IN_PROGRESS`.
- C2. `DEC-049` (SOFTWARE_DECOMP §12) adopts D-15 Option B with the quoted
  exclusions and the "unless a later D6/D9 tranche proves deeper
  assembled-solve consumption" clause; the deliverable Remaining row
  directs that deeper consumption.
- C3. In live `core/product_physics/src/lib.rs`, `build_model` filters
  constant-effort supports out of the solve (after the `nonlinear`
  branch), and no force/stiffness/reaction consumption of
  `hanger.constant_load` exists on any solve path.
- C4. `hanger.constant_load` is unit-normalized to `Dimension::Force` by
  the existing input normalization, and the review rows
  (`constant_effort_user_input_review`) currently carry a disclosure that
  no global constant-effort load is claimed.
- C5. Per-load-case force assembly (global load vector + element/pressure/
  thermal additions) precedes `reduce_system`, and the nonlinear
  active-set loop consumes the same assembled force vector — so predicate
  §3.1's single-seam design is implementable as described.
- C6 (v2). The pinned `tp_runner_015_final_cli_solve_input.json` contains
  a `constant_effort_support` whose `restraints` list is empty; under the
  v2 opt-in rule that support stays review-only, the case keeps exit
  0/COMPLETED, and the output changes only by the new non-consumption
  warning; no `validation/benchmarks/**` fixture and no
  `del1005_payload_binding_*` input/witness contains a constant-effort
  support, so the del1005 byte-identity predicate is satisfiable.
- C7. The three committed tp_runner_015 witnesses are historical records
  (pinned pre-#287 commits) and are not edited by this brief; the
  reproduction manual records that posture.
- C8 (v3). The §5 fence covers every §4 task and nothing materially more,
  including the one additive inventory line in EACH of the two fixture
  inventory mirrors (`validation/benchmarks/mechanics/README.md` and
  `validation/hand_calcs/mechanics/README.md`) that the default benchmark
  path requires; no third inventory mirror exists in the live tree; the
  excluded surfaces are genuinely not required.
- C9. DEL-04-03's deliverable-local execution-upstream rows are five
  `SATISFIED` constraints plus three `TBD` prerequisites
  (DEL-04-01/DEL-02-01/DEL-02-02), each target holding committed
  implementation evidence in the live tree; the brief records this posture
  truthfully and resolves no row.
- C10. The user-limit warning (§3.5) compares only user-entered values
  against computed displacements and introduces no software-defined
  constant, threshold, or acceptance criterion.
- C11 (v2). The §1 screen above, read with the v2 amendment, is complete
  against D-52 §4.1 / D-54 §3.1 with no class hit: the class-9 basis
  defect the v1 verifier found (mischaracterized pinned-fixture
  consequence) is cured by the corrected §1/§2/§3 text, and the class-4
  ambiguity (retroactive invalidation of an accepted input shape) is
  removed by the opt-in consumption rule.
- C12. The §6 plan is executable offline in this worktree.
- C13. §10 keeps owner standing approval and agent classification
  distinct, `OwnerCaseSelection: NONE`, `EffectStatus: HELD`.
- C14. The R14 campaign plan authorizes W1 to run this tranche chain with
  per-tranche commits and no push/PR/merge/receipt at manager level; the
  DEC-025 single wave-level sweep refinement is recorded in the T1 brief
  §6 and applies wave-wide.
- C15. The DEC-026 analytic-class relative tier is already recorded as the
  mechanics suite's comparison basis (suite README/source), so §3.7 is
  reuse, not tolerance creation.
- C16 (v2). The pinned fixture's constant-effort support declares
  `"restraints": []` in the live tree, and the v2 §3.1–§3.2 predicates
  keep that case exit 0/COMPLETED while every consumption path requires
  explicit user data (no inference, no default).
- C17 (v2). DEC-049's "D6 remains the owner for any future nonlinear or
  deeper assembled-solve consumption" annotation is surfaced in brief §1
  with a truthful reading: the rehomed ungated DEL-04-03 Remaining row is
  the successor selection surface (F-PIP-5 rehoming, Receipt 12), the
  tranche implements only linear constant-force consumption, and queue
  selection is HUMAN campaign authority — no owner gate is crossed.
- C18 (v2). The reproduction-manual case-1 staleness consequence is
  recorded (brief §3.9/§7) as a HELP_HUMAN docs-lane follow-on per the
  R12→R13 case-3 precedent, with no docs write inside this tranche's
  fence.

## 5. Attempted Failure Mode

Attempted refutation before dispatch: classify the direction convention
(§3.3) as new normative content (class 4) because the software chooses the
positive-axis meaning of the user's restraint declaration. The attempt
fails: a deterministic, disclosed sign convention for consuming an existing
user input is interface semantics, not a normative engineering criterion;
it constrains no acceptance, imposes no value, and every ambiguous case
fails closed back to the user. A second attempt — treating the pinned
fixture's output change as destroying evidence (class 7) — fails because
the committed witnesses are preserved untouched as historical records,
the change is captured before/after, and no completed evidence bundle is
edited or reinterpreted.

## 6. Classification, Effect, and Preserved Gates

- **Classification:** `STANDING_APPROVAL_ELIGIBLE`; **Agent
  classification:** `CLASSIFY_ELIGIBLE`.
- **Rule activation:** `ACTIVATE_OWNER_STANDING_APPROVAL`; adoption is the
  owner's conditional act under DEC-085/D-52 §2 as refined by DEC-087/D-54
  §1; `OwnerCaseSelection: NONE`.
- **Effect:** `HELD` pending the fresh-context verifier
  (`instances/W1/T2/VERIFY_BRIEF.md`).
- **Preserved gates:** as enumerated in brief §9/§10.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
