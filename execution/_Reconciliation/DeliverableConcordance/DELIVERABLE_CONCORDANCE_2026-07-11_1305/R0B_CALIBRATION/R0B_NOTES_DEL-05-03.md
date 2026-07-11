# R0b Calibration Notes — DEL-05-03 "Fundamental stress recovery module" (PKG-05)

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 (activation D-41/DEC-073)
**Pilot round:** R0b (second calibration; provisional convention set under test)
**Evidence source state:** frozen worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
(`.claude-worktrees/piping-frozen-551f84ef6/projects/chirality-piping`)
**Ledger:** `R0B_CLAIM_CONCORDANCE_DEL-05-03.csv` — 17 rows
**Disposition histogram:** ALIGNED 14 · ACCEPTED_DIVERGENCE 1 · STALE_SETUP_SPECIFICATION 1 · IMPLEMENTED_UNDOCUMENTED 1

## 1. Sources read (all from the frozen tree unless noted)

Deliverable kit: `Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_CONTEXT.md`, `_STATUS.md` (incl. `## Remaining`), `_REFERENCES.md`, `_DEPENDENCIES.md`,
`Dependencies.csv`, `MEMORY.md`, `_REVIEW.md`, `Review_Findings.csv`; run records
`TASK_RUN_2026-05-12_0956_TP-PHYS-001`, `TASK_RUN_2026-05-16_1947_TP-PHYS-003-B`,
`TASK_RUN_2026-05-17_TP-PHYS-004-D`, `TASK_RUN_2026-05-17_TP-PHYS-007-B`,
`TASK_RUN_2026-06-05_2055_REVIEW_READINESS_PREP`,
`WORKING_ITEMS_RUN_2026-06-05_2000_FORCE-PER-LENGTH-BOUNDARY_FANIN`,
`WORKING_ITEMS_RUN_2026-06-21_TP-R4-D1-BENDSTRESS-001`,
`WORKING_ITEMS_RUN_2026-06-21_TP-R4-D2-BRANCHSTRESS-001` (validation sections).

Implementation/verification/validation: `core/loads/stress_recovery/{README.md,Cargo.toml,src/lib.rs}`
(26 embedded tests), `validation/hand_calcs/stress/**` (README + 15 fixture notes + generated
witness rendering), `validation/benchmarks/stress/src/lib.rs` (22 tests),
`validation/witness/fixtures/tp_phys_015_section_property_stress_witness.json` (cited, not re-run),
`core/product_physics/src/lib.rs` (targeted greps for multiplier-review rows and the
"base frame stiffness unchanged" provenance string), git log for the crate and adjacent surfaces.

Authority/decision surfaces: `execution/_Coordination/_DECISIONS/_REGISTER.md` (D-41 row,
AWAITING_RULING at frozen SHA), `execution/_Decomposition/SOFTWARE_DECOMP.md` §12
(DEC-045, DEC-066–070), `execution/_Reconciliation/Reviews/PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md`
(existence verified), review snapshot pointer `REV_DEL-05-03_2026-06-05_2120`.

Convention-3 homing checks (`_STATUS.md ## Remaining` of candidate homes): DEL-05-02
(modulus-basis extension + D-38 gated residuals — HOMED), DEL-04-01 (mechanics-program §5
completion / DEC-066–070 residuals incl. DEC-070 exit bar — HOMED), DEL-09-02 (release
benchmark/tolerance scope — NOT homed; bootstrap-only), DEL-02-02 (conversion catalog —
NOT homed; bootstrap-only).

Method files: frozen plan §§3/6/7 (and surrounding sections for the lifecycle model);
working-tree `R0B_CONVENTIONS.md` (the one permitted working-tree read besides this run folder).

Evidence execution (convention 7): `cargo test --locked` re-executed at the frozen SHA for
`core/loads/stress_recovery` (26 passed) and `validation/benchmarks/stress` (22 passed), with
`CARGO_TARGET_DIR` pointed at the session scratchpad so no build artifact touched the frozen
tree; `git status --porcelain` verified clean (0 lines) afterward.

## 2. Per-convention assessment

**Convention 1 — stale-prose two-signal split (D1).** Applied. All nine requirement rows carry
substance dispositions; declaration staleness landed only on the Datasheet `DECLARED_STATE` row
(C06). It changed an encoding: without the convention I would have hesitated between
`REMAINING_STATE_MISMATCH`-adjacent handling and folding the staleness into RQ-009, and I would
not have used `STALE_SETUP_SPECIFICATION` at all — the stale prose here is not setup/future
language but an implementation-era enumeration overtaken by another deliverable's tranche
(DEC-068 modulus-basis, commit `c6430cbd5`, executed under DEL-05-02 on 2026-07-10). The split
itself was unambiguous; the reserved disposition's *name and §7 definition* ("setup/future
language") do not match the drift actually found. **Verdict: applied-with-friction** (disposition
semantics, not the split).

**Convention 2 — `DECLARED_STATE` ClaimType (D2).** Applied cleanly; three rows (C05 Specification,
C06 Datasheet, C07 Guidance). The enum extension worked. One boundary the convention does not
draw: which surfaces count as "declared-state prose surfaces." I treated the four-document kit's
current-state prose as declared state and treated `MEMORY.md` as a date-scoped historical log
(its 2026-06-21 "base pipe stress rows and frame stiffness remain unchanged" statements are
overtaken at the frozen SHA by the DEC-070 curved-bend commits, but as dated log entries they are
not stale declarations; the retirement of the matching *code provenance string* is already in
DEC-070's exit-evidence bar, homed at DEL-04-01). A different pilot could reasonably emit a
MEMORY declared-state row. **Verdict: applied-clean** (with a scoping gap noted in §4).

**Convention 3 — residual homing before mismatch (D3).** Applied four times. Two homings resolved
cleanly (modulus-basis residuals → DEL-05-02 `## Remaining`, D-38-gated; DEC-070 exit
bar/provenance-string retirement → DEL-04-01 `## Remaining` §5-completion item) → cross-references,
no findings — exactly what the convention intends. Two did not resolve (release
benchmark/tolerance scope → DEL-09-02; conversion catalog → DEL-02-02; both candidate homes carry
bootstrap-only `## Remaining`). Friction: the convention's "unresolved → `UNKNOWN` with the
smallest next check" reads as written for *omitted evidence-backed residuals*; here the open items
are **declared bounded deferrals** (spec/datasheet TBDs accepted by the 2026-06-05 blocker-closure
human ruling), so forcing `UNKNOWN` on otherwise-clean rows would manufacture doubt. I kept the
substance dispositions (`ACCEPTED_DIVERGENCE` on C03 per convention 8 precedence; `ALIGNED`
elsewhere), recorded the failed homing and the smallest next check (the candidate homes' own
concordance passes in this run) in `RemainingWork`. **Verdict: applied-with-friction** — the
convention should say whether declared-TBD deferrals that lack a `## Remaining` home anywhere are
a finding, a cross-run check, or acceptable declared state under the no-carve-out lifecycle model.

**Convention 4 — ID column controlled values (D4).** Applied cleanly. The one unmapped row uses
`UNMAPPED` in the ID cell with the ownership hypothesis (PKG-05/DEL-05-03 for the multiplier-review
rows via the 2026-06-21 run records; DEC-070/D-34 program at DEL-04-01 for the solve-path
successor) in `RemainingWork`. Without the convention I would likely have written
`PKG-05/DEL-05-03?` in the ID cell — it changed my encoding as intended. I chose bare `UNMAPPED`
over `PKG-05/UNMAPPED` because `core/product_physics` spans packages; the convention supports
either and that choice is judgment. **Verdict: applied-clean.**

**Convention 5 — column defaults and rubrics (D5, T2).** Applied throughout: `NONE_RECORDED` gate
cells on all no-residual rows (only C04 carries `(gated: D-41)`); RQ-004 classed `WORKFLOW` per the
diagnostics rubric (I might otherwise have drifted to MECHANICS because the findings guard
mechanics outputs — the rubric settled it); every `ValidationEvidence=NOT_APPLICABLE` carries an
in-cell reason; no source claimed `VETTED` (nothing in scope is a maintainer-vetted technical
source named as such — the hand-calc fixtures are project-original agent-generated artifacts that
passed human rulings, so `REVIEWED`). Friction, two small gaps: (a) `LifecycleState` has no defined
value for `IMPLEMENTED_UNMAPPED` rows — I wrote "UNMAPPED - no owning deliverable state (see
notes)", which is off-enum; (b) `SourceReliability` for `DECLARED_STATE` prose rows is ambiguous
between `REVIEWED` (they are reviewed project records) and `NOT_APPLICABLE` ("non-technical prose
rows") — I used `REVIEWED` because the prose makes technical declarations. **Verdict:
applied-with-friction** (two cell-value gaps).

**Convention 6 — mechanical selectability + SECURITY encoding (D6, T3).** Selectability derived
contract-literally: the sole residual is `(gated: D-41)` and the frozen register row is
AWAITING_RULING → `NO`; no-residual rows → `NO`; the owner suspension was never encoded per-row.
No `UNKNOWN` was needed anywhere. The SECURITY-class encoding rule had nothing to bind — DEL-05-03
has no SECURITY-class behavior claims. **Verdict: selectability applied-clean; SECURITY clause
not-exercised.**

**Convention 7 — evidence execution and basis resolvability (D7, A2–A4).** Applied. Re-executed the
two side-effect-free checks (crate tests, benchmark tests) inside the frozen tree with an external
`CARGO_TARGET_DIR`; porcelain-clean verified. All other cited passes (product_physics/Vitest/
Playwright in the 2026-06-21 run records; the witness pytest; the 2026-06-05 review validation)
carry the standardized marker `not re-executed at frozen SHA 551f84ef6` plus their run-record
binding. Both cited decision bases resolve to artifacts in the tree (blocker-closure ruling packet;
register row D-41; SOFTWARE_DECOMP §12 rows) — no `ATTESTED:` marker was needed. Friction, minor:
"side-effect-free" needed interpretation for `cargo test`, which writes build artifacts by default;
the external-target-dir technique keeps the tree byte-clean but is pilot-invented — the convention
could name it (or bless ignored-path writes) so pilots converge. **Verdict:
applied-with-friction** (minor, execution technique unspecified).

**Convention 8 — disposition precedence (D8, D9, FN1, FP2).** Applied on all four prongs.
(1) `ACCEPTED_DIVERGENCE` > `ALIGNED` taken on C03 (human-ruled bounded open-validation deferral) —
without the precedence rule I would probably have written `ALIGNED` since the declaration is
accurate. (2) The overtaken-evidence rule mattered: the recorded review evidence (24 locked tests,
2026-06-05) no longer describes the frozen crate (26 tests); RQ rows avoided
`STALE_REVIEW_OR_EVIDENCE` only because independent bind-current evidence (re-execution at the
frozen SHA) is cited in the same cells. (3) Unmapped grain: `recover_stress_range_with_modulus_basis`
was **not** ledgered as unmapped (public function inside the mapped crate, plausibly inside RQ-009's
range scope — encoded as a note on RQ-009/C06 instead), while the `core/product_physics` stress
result-row surface **was** ledgered once at material-surface grain. Friction: product_physics is a
single large crate shared by many deliverables' recorded work; "crate grain" forces either one
giant cross-package row or a named slice — I ledgered the named slice ("stress result-row surface")
and flagged the grain question here. (4) `RecordedRemaining` verbatim only on C04; `NONE_RECORDED`
elsewhere; the D-41 gate-state cell reflects the frozen register (AWAITING_RULING) without
re-deriving the time-split, per the run-level codification. **Verdict: applied-with-friction**
(shared-crate grain only; the precedence rules themselves were clean).

## 3. Substantive findings worth the reviewer's attention

1. **This deliverable is the corpus's best case, not its representative case.** The four-document
   kit was rewritten to implemented evidence on 2026-06-04, so 14/17 rows align. The calibration
   value here is the *drift-after-alignment* pattern: DEC-068 work executed under DEL-05-02 on
   2026-07-10 extended DEL-05-03's crate and silently invalidated the Datasheet's API enumeration
   within ~5 weeks of the alignment (C06). R1+ should expect aligned-then-drifted kits, not just
   setup-era kits.
2. **Cross-deliverable writes into a mapped crate.** The modulus-basis tranche (DEL-05-02) and the
   2026-06-21 multiplier-row tranches (recorded under DEL-05-03, implemented in product_physics)
   both cross deliverable/crate ownership lines. The conventions handle the ledger mechanics but
   not the ownership question itself — routed to `AuthorityNeeded=OWNER` on UNMAPPED-DEL-05-03-01.
3. **The "base frame stiffness unchanged" provenance string** is still emitted by product_physics
   at the frozen SHA while curved-bend macro-elements already enter the global stiffness
   (commits `b77e721b2`, `77d760cb2`, `50f230b09`). Its retirement is inside DEC-070's exit-evidence
   bar and homed at DEL-04-01 — recorded as a cross-reference, not adjudicated here (no mechanics
   inference made; boundary respected).

## 4. Residual ambiguities the conventions do NOT cover

1. **Declared-state surface census.** No rule says which files are "declared-state prose surfaces"
   (`MEMORY.md`? crate `README.md`? `_REVIEW.md` narrative?). I used the four-document kit only;
   pilots will diverge without a named list.
2. **`STALE_SETUP_SPECIFICATION` vs post-alignment drift.** The §7 definition is setup/future
   language; convention 1 reserves the value for declared-state rows generally. Either widen the
   §7 definition ("declaration no longer describes the frozen implemented slice") or add a
   distinct value; otherwise the disposition name misdescribes cases like C06.
3. **Declared bounded TBDs vs the no-carve-out lifecycle model.** Spec/datasheet TBDs accepted by
   a human ruling live nowhere in any `## Remaining`. For concordance encoding I used
   `ACCEPTED_DIVERGENCE`/cross-reference, but at CHECKING-entry time these same items decide
   whether `## Remaining` emptiness is warranted. The conventions need a rule for
   "ruling-accepted deferral with no recorded residual anywhere."
4. **`LifecycleState` and `SourceReliability` cell values for UNMAPPED and prose rows** (see
   convention 5 friction).
5. **Shared-crate grain for `IMPLEMENTED_UNMAPPED`** (see convention 8 friction): whether one
   cross-package crate row or per-slice rows are wanted for `core/product_physics`.
6. **Verbose `NormativeSource` paths.** The full deliverable folder path (with spaces and commas)
   makes every cell heavy; a run-level path alias convention would help ledger legibility.

## 5. Deferred claims

None deferred. No claim required `ENGINEERING_AUTHORITY_REQUIRED`, `UNKNOWN`, or
`DEFERRED_AGENT_WORKFLOW`: every mechanics formulation claim was either covered by provenance-bearing
hand-calc/benchmark evidence re-executed at the frozen SHA, or explicitly declared open and
human-accepted (C03). The DEC-066–070 overlap touches this deliverable only through (a) the
DEL-05-02-owned modulus-basis extension and (b) the product_physics multiplier-row surface — both
encoded without adjudicating any formulation, with validation questions routed to their recorded
homes (DEL-04-01 §5-completion item; DEC-070 exit bar) and the mapping question to owner authority.

## 6. Frozen-tree integrity attestation

Only side-effect-free reads and the two `cargo test --locked` re-executions (external
`CARGO_TARGET_DIR`) were performed in the frozen worktree. `git status --porcelain` returned
empty after the last evidence operation. No file in the frozen worktree was created, modified,
or deleted.
