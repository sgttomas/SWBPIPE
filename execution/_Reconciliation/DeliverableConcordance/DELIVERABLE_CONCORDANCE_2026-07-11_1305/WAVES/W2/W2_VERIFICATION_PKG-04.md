# W2 Fan-in Verification — PKG-04 (Solver Core and Numerical Methods)

Verifier: fable at high effort (owner-ruled wave-boundary verification per the
Receipt-17 steer). Scope: the six W2 discovery ledgers
`CLAIM_CONCORDANCE_DEL-04-01..06.csv` + `NOTES_DEL-04-01..06.md` under
`WAVES/W2/`, verified against the FROZEN evidence worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` (HEAD re-verified; porcelain empty
before and after all verification reads — verification used read-only git/grep
operations only). Method authority: `R1_CONVENTIONS.md` (conventions 1–8,
addenda 1–13, Part C), pinned plan §§6–7, `RUN_BASIS.md`, plus the W1
calibration items in `PACKAGE_SUMMARIES/PKG-0{0..3}.md`. All findings below
are agent-authored and non-binding; nothing here is an owner or engineering
ruling. No ledger was edited.

**Verdicts: DEL-04-01 SOUND · DEL-04-02 SOUND · DEL-04-03 SOUND ·
DEL-04-04 SOUND · DEL-04-05 SOUND · DEL-04-06 SOUND.**
Spot-check totals: **55 PASS / 13 QUALIFIED / 0 FAIL.** Two string
corrections recommended (non-blocking, see §3.1). Frozen tree untouched.

## 1. Mechanical conformance (all six ledgers)

- **Structure:** every CSV parses RFC-4180 clean at exactly 20 columns; no
  duplicate ClaimIDs; ClaimID form conforms to addendum 12
  (`DEL-XX-XX-<TYPE>-NNN`; kit-native RQ/2-digit/REQ-04-06-* forms recorded
  in-row in `NormativeSource` as the notes declare).
- **Histograms:** independently recounted from each CSV — all Disposition and
  ClaimType histograms in all six notes files **reproduce exactly**
  (28/16/23/25/18/23 rows).
- **Addendum-1 census:** 7/6/7/7/7/7 DECL rows. Four-document kit + `_STATUS`
  + `MEMORY` present on all six; crate README censused on five (see §3.2 for
  the DEL-04-02 exclusion). Dated-MEMORY-entry protection applied everywhere;
  no staleness disposition rests on drift inside a dated entry.
- **Addendum-2 bootstrap:** the seeded `(gated: D-41)` item appears verbatim
  only in each `_STATUS` surface row's `RecordedRemaining`, never as its own
  row, and is excluded from residual/gate/selectability analysis in all six.
  All `RecordedRemaining` cells verified **byte-exact** against the six frozen
  `_STATUS.md ## Remaining` sections (13 non-bootstrap + 6 bootstrap items).
- **Convention 3 / addendum 3 selectability:** re-derived mechanically —
  DEL-04-01 2 YES (DECL-005, REM-001 ungated), DEL-04-03 2 YES (DECL-005,
  REM-001), DEL-04-04 3 YES (DECL-005, REM-002, REM-003), others 0 YES; all
  match the ledgers and the R1 inventory grain. Owner suspension kept
  run-level everywhere.
- **Addendum-6 ladder:** all DECL rows `NOT_APPLICABLE`; no `VETTED` claimed
  anywhere; `REVIEWED` variance noted in §3.4.
- **Addendum-10 qualifiers:** every content-identical qualifier was backed by
  a diff the pilot actually ran, and this verifier **independently re-ran all
  of them** at the frozen worktree: ancestry of `e648462f1d05` confirmed
  (`git merge-base --is-ancestor`); diffs empty over every claimed path set
  (DEL-04-01's four roots; DEL-04-02's `core/` + validation + release script;
  DEL-04-03's two crate paths; DEL-04-04/05/06's crate/validation/tests
  paths). The only differing path under `validation/` is the sweep-record
  JSON itself — exactly as DEL-04-04/05 disclose in their exclusion clauses;
  DEL-04-06 correctly scoped its qualifier to `core/` only and did not claim
  undiffed paths. W1 calibration item "qualifier only with actually-run
  diffs": **satisfied by all six.**
- **Addendum-11:** both `ACCEPTED_DIVERGENCE` rows rest on named permitting
  records that resolve in-tree (§2 row checks).
- **Addendum-12 grain:** ACCEPTANCE minted only where verification content
  exceeds requirement restatement (DEL-04-04 ACC-001, DEC-067 transition
  witnesses — verified real); zero-ACC censuses on the other five are
  disclosed and consistent with the W1 encoding.
- **PKG-04 strictness (verification ≠ validation):** checked on every
  MECHANICS/VALIDATION row — no unit test is promoted to engineering
  validation anywhere; witness-backed benchmark executions are cited as
  validation and unit suites as verification; threshold/tolerance promotion
  residuals route ENGINEERING (DEL-04-04 REM-002..004, DEL-04-05 REM-001);
  agent-generated witnesses are consistently labeled agent-audited with human
  disposition pending.
- **SECURITY (Part C / W1 item):** zero SECURITY-class rows exist in PKG-04;
  the em-dash marker + OWNER routing harmonization did not arise (N/A,
  confirmed by class census of all 133 rows).

## 2. Per-ledger verification tables

### 2.1 DEL-04-01 (28 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (23 ALIGNED / 4 STALE / 1 ACC_DIV; types 12/7/7/2) | PASS |
| 2 | `RecordedRemaining` byte-exact vs frozen `_STATUS.md` (DECL-005, REM-001, REM-002; bootstrap excluded from gate/source) | PASS |
| 3 | Addendum-10 diff re-run over `core/solver/ core/product_physics/ validation/benchmarks/ validation/hand_calcs/` → empty; ancestry OK | PASS |
| 4 | ALIGNED spot 1 — REQ-002: `FrameDof`, `NODE_DOF_ORDER`, `node_dof_index`, `element_dof_map` present in frozen `frame_kernel/src/lib.rs`; 36 `#[test]` counted | PASS |
| 5 | ALIGNED spot 2 — REQ-005: `reduce_system_with_prescribed_displacements` present; restraint-rejection paths as claimed | PASS |
| 6 | REQ-006 ACCEPTED_DIVERGENCE: D-17 ruling record resolves; RULED Option B (dense default + non-blocking sparse observer; promotion follow-on) — names and permits exactly the encoded deferred state (addendum-11 bar met) | PASS |
| 7 | REQ-007 DecisionBasis: `Review_Findings.csv` PKG04-DEL0401-PKG02-001 = ACCEPT_AS_IS / RESOLVED; blocker-closure packet present | PASS |
| 8 | DECL-001..004 STALE facts: Specification setup future-tense confirmed; Guidance TBD trade-off rows confirmed; Datasheet cites rev 0.7 vs frozen decomp header rev 0.8 `current_basis`; owner-calibration caveat stated once in notes | PASS |
| 9 | DECL-006 MEMORY ALIGNED-with-note (package item 2 — see §3.3): undated `## Open TBDs` header still says sparse library TBD; dated 2026-06-11 entry records DEC-023 resolution — facts verified | QUALIFIED |
| 10 | DECL-007 README: self-identifies "conservative first slice for DEL-04-01"; content matches frozen crate | PASS |
| 11 | EXC-001 ATTESTED marker: `DEV-001_DISPATCH_DEL-04-01.md` confirmed absent from frozen tree — convention-7 marker correctly applied (and the pilot correctly declined to cite the likewise-absent DEV001 finding-resolution directory) | PASS |
| 12 | REM-001/REM-002: D-34 record + HC-MECH-020 witness + mechanics plan present; gating and OWNER routing consistent | PASS |
| 13 | SourceReliability REVIEWED on REQ rows (see §3.4 ladder variance) | QUALIFIED |

Tally: 11 PASS / 2 QUALIFIED / 0 FAIL. Byte-identical copy re-execution
(frame_kernel, `diff -r` verified, external `CARGO_TARGET_DIR`, porcelain
clean): sound — §3.5.

### 2.2 DEL-04-02 (16 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (11 ALIGNED / 4 STALE / 1 PARTIAL; types 7/3/6) | PASS |
| 2 | Bootstrap-only `RecordedRemaining` byte-exact; gate/source at column defaults (W1 calibration item 5) | PASS |
| 3 | Addendum-10 diff re-run over `core/` + validation + `tools/release/check_release_readiness.py` → empty; ancestry OK | PASS |
| 4 | ALIGNED spot 1 — REQ-001: frozen `straight_pipe/Cargo.toml` sole path dependency `open_pipe_stress_frame_kernel`; boundary test `straight_pipe_stiffness_matches_frame_kernel_boundary` at cited L1517; 33 `#[test]` counted | PASS |
| 5 | ALIGNED spot 2 — REQ-005 + validation posture: findings PKG04-DEL0402-PKG02-001/002 = ACCEPT_AS_IS / RESOLVED; HC-MECH-004 witness present; benchmark suite (RUST-34 = 33 tests) matches R1 index; validation cited only via witness-executing benchmarks (strictness holds) | PASS |
| 6 | REQ-007 PARTIALLY_IMPLEMENTED: MEMORY Open-TBDs result-envelope residual and `REV_DEL-04-02_2026-06-05_2120` snapshot confirmed; grain disclosed in-row | PASS |
| 7 | DECL-001..004 STALE facts: setup-era kit prose confirmed ("this setup pass does not implement solver code"); rev-0.7 cite on Datasheet; caveat once in notes | PASS |
| 8 | DECL-005: both LifecycleCorrection Decision_Logs + blocker packet present; K-CONFLICT-1 basis resolves | PASS |
| 9 | DECL census = 6 (crate README excluded) — package intra-wave inconsistency; disclosed with W1 precedent (§3.2) | QUALIFIED |
| 10 | Addendum-13-by-analogy MEDIUM cap + OWNER routing on REQ-002/004/005 (witness state "ruling TBD", not literal TECHNICALLY_ADDRESSED_PENDING_HUMAN) — disclosed, conservative, aggregation-safe | QUALIFIED |

Tally: 8 PASS / 2 QUALIFIED / 0 FAIL. In-tree `--locked` benchmark re-run +
byte-identical copy re-run (both `diff -r` verified) sound — §3.5.

### 2.3 DEL-04-03 (23 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (19 ALIGNED / 4 STALE; types 12/3/7/1) | PASS |
| 2 | `RecordedRemaining` byte-exact (DEC-049 residual + bootstrap, `;`-joined in `## Remaining` order) | PASS |
| 3 | Addendum-10 diff re-run over `linear_supports/` + `frame_kernel/` → empty; ancestry OK | PASS |
| 4 | ALIGNED spot 1 — REQ-001: `SupportFamily` + all six constructors (`anchor/guide/line_stop/vertical_support/spring/imposed_displacement`) present; 14 `#[test]` counted (= RUST-27) | PASS |
| 5 | ALIGNED spot 2 — REQ-012: `prepare_boundary` / `apply_linear_supports` present; HC-MECH-007 witness present; validation breadth limitation carried, not suppressed | PASS |
| 6 | DECL-001..004 STALE facts (hanger post-alignment drift): D-15 ruling RULED Option B (DEC-049 constant-effort user-data slice); MEMORY 2026-06-21 TP-R4-D5-HANGERDATA-001 entry present; `constant_effort` surfaces in frozen `core/product_physics` (11 hits); kit-wide crate-only declarations confirmed; encoding matches the W1 fan-in's under-staling adjudication | PASS |
| 7 | DECL-007 README: self-identifies "bounded implementation slice for DEL-04-03" | PASS |
| 8 | REM-001: D-15/DEC-049 + PRD plan §3 D5 row resolve; UNGATED, Selectable YES; ENGINEERING deferred-to-future correctly reasoned in `RemainingWork` | PASS |
| 9 | DECL-005 bootstrap gate/source-cell scoping (package item 3 — §3.3): aggregation-safe | QUALIFIED |
| 10 | REQ-004 `ValidationEvidence` cites `DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv` — **absent from the frozen tree** (transcribed from the kit's own MEMORY.md pointer; the substantive record, the `Review_Findings.csv` TECHNICALLY_ADDRESSED_PENDING_HUMAN row, resolves and matches; disposition rests on independent grounds) — string correction recommended (§3.1) | QUALIFIED |
| 11 | Copy re-execution via `cp -R` without an independent byte-identity diff recorded (§3.5) — side-effect-free and disclosed; evidentially weaker than the sibling `diff -r` pattern | QUALIFIED |

Tally: 8 PASS / 3 QUALIFIED / 0 FAIL.

### 2.4 DEL-04-04 (25 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (21 ALIGNED / 2 PARTIAL / 2 STALE; types 9/1/4/7/4) | PASS |
| 2 | `RecordedRemaining` byte-exact (4 residuals + bootstrap); gate cell lists 4 non-bootstrap values in order | PASS |
| 3 | Addendum-10 diff re-run: empty over both nonlinear crates + `validation/benchmarks/nonlinear/` + `validation/hand_calcs/nonlinear/`; the only `validation/` diff is the sweep JSON — exclusion clause verified verbatim-true | PASS |
| 4 | ALIGNED spot 1 — REQ-002: `classify_support_state` present; DEC-067 (D-35 ruling) resolves; all three transition witnesses (`assembled_one_way_reengagement/gap_lift_off/friction_bounded_sliding.md`) present; 19 + 16 `#[test]` counted (= RUST-29 family / sweep claim) | PASS |
| 5 | ALIGNED spot 2 — ACC-001: witness/fixture inventory and DEC-046 policy-record framing verified; acceptance grain justified by verification-section substance beyond the requirement table | PASS |
| 6 | REQ-006 PARTIALLY_IMPLEMENTED: finding PKG04-DEL0404-PKG02-001 WARNING / TECHNICALLY_ADDRESSED_PENDING_HUMAN / TBD confirmed; addendum-13 MEDIUM cap applied; grain disclosed | PASS |
| 7 | DECL-002 STALE: rev-0.7 cites confirmed on frozen Datasheet vs decomp 0.8 header; single overtaken friction-TBD cell → AuthorityNeeded=NO reasoning disclosed (not an overtaken register) | PASS |
| 8 | DECL-003 STALE: Guidance "prepares a future … before implementation work begins" confirmed verbatim in frozen file | PASS |
| 9 | DECL-004 ALIGNED (package item 1 — §3.3): frozen Procedure verified — it is a *documentation setup* procedure whose step 3 was refreshed with current DEC-067 implemented-state language ("the implemented classifier decision boundary is state-switched…"); its declarations do not misdescribe the frozen slice. Encoding convention-correct | PASS |
| 10 | REM-001..004: D-16/D-19/D-25/D-27 records + mechanics plan + PRD plan resolve; Receipt 6 correctly ATTESTED (not in tree); gate suffixes byte-exact; ENGINEERING routing on threshold-promotion residuals matches PKG-04 strictness | PASS |
| 11 | Re-executions (benchmark `--locked` in-tree + pytest with no-cacheprovider) as recorded; `--locked` crate refusal correctly handled by recorded-pass + actually-run diff | PASS |
| 12 | `AuthorityNeeded=ENGINEERING` on ALIGNED REM rows — reading of the column disclosed in notes §4.4; consistent across 04-04/04-05; R3 should confirm the intended semantics | QUALIFIED |

Tally: 11 PASS / 1 QUALIFIED / 0 FAIL.

### 2.5 DEL-04-05 (18 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (12 ALIGNED / 1 PARTIAL / 1 ACC_DIV / 4 STALE; types 7/2/7/2) | PASS |
| 2 | `RecordedRemaining` byte-exact (2 residuals + bootstrap); both gated → Selectable NO derivation confirmed | PASS |
| 3 | Addendum-10 diff re-run incl. the two `tests/*.py` files → empty; sweep-JSON-only exception verified | PASS |
| 4 | ALIGNED spot 1 — REQ-004: `FixtureProvenance`, `HarnessRunRecord`, `run_invented_fixture_suite`, `solver_version`, `provenance_notes` all present; 20 `#[test]` counted | PASS |
| 5 | ALIGNED spot 2 — REQ-002/REQ-005: all five DEC-050/DEC-053 governed JSON records present under `validation/benchmarks/` | PASS |
| 6 | EXC-001 ACCEPTED_DIVERGENCE: MEMORY 2026-05-01 "Human project authority authorized proceeding" entry verified in frozen file; DEC-023 (D-03 RULED) resolves; addendum-11 named-permitting-record bar met. Novel species (overtaken setup-pass exclusion prose) — disclosed; convention gap noted for R3 (§3.6) | QUALIFIED |
| 7 | REQ-006 PARTIALLY + finding PKG04-DEL0405-PKG02-001 confirmed; but `VerificationEvidence` cites the absent `RESOLUTION_MATRIX.csv` as "recorded technical resolution" without an absence marker (same species as DEL-04-03 REQ-004; §3.1) | QUALIFIED |
| 8 | DECL-001..004 STALE facts: Specification "setup evidence for a future test-suite harness" / "does not implement a benchmark runner…" verified verbatim; Datasheet rev 0.7; Procedure declares "the current setup-only boundary" as present-tense current state with an overtaken TBD prerequisite and no post-implementation refresh — **STALE is convention-correct here** (package item 1, §3.3); OWNER-vs-NO routing split reasoned per the W1 item-1 trigger | PASS |
| 9 | DECL-007 README: self-identifies DEL-04-05; DEC-023/050/053 content matches | PASS |
| 10 | REM-001/REM-002: DEC-053/DEC-054/DEC-059 records resolve; REM-002 `AuthorityNeeded=NO` on an already-ruled conditional gate — disclosed alternative (OWNER) noted | QUALIFIED |

Tally: 7 PASS / 3 QUALIFIED / 0 FAIL.

### 2.6 DEL-04-06 (23 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (18 ALIGNED / 5 STALE; types 11/5/7) | PASS |
| 2 | Bootstrap-only `RecordedRemaining` byte-exact; gate/source at defaults | PASS |
| 3 | Addendum-10 diff re-run over the five named `core/` paths (and all of `core/`) → empty; the ledger correctly does NOT claim `validation/`/`docs/` under the qualifier | PASS |
| 4 | ALIGNED spot 1 — REQ-002: all ten `SolverDiagnostic` fields (`code/class/severity/source/message/affected_ref/canonical_ref/remediation/provenance/quantity_units`) present in frozen `src/lib.rs` | PASS |
| 5 | ALIGNED spot 2 — REQ-004: `analysis_boundary_mapping` present; mapping semantics as claimed | PASS |
| 6 | REQ-008/009/010: `classify_condition_ratio`, `convergence_diagnostic`, `sparse_solver_tbd_diagnostic`, `tolerance_policy_tbd_diagnostic`, `diagnostic_from_sparse_error`, `NonPositivePivot` all present; 24 `#[test]` counted (= RUST-25, matching the pilot's independent static count) | PASS |
| 7 | Findings PKG04-DEL0406-PKG02-001/002: WARNING / TECHNICALLY_ADDRESSED_PENDING_HUMAN / TBD confirmed; REVIEW routing with HIGH-not-capped rests on the addendum-13 validation-scope reading — disclosed; variance with the DEL-04-02 by-analogy cap named for R3 (§3.4) | QUALIFIED |
| 8 | DECL-001/002/004 STALE facts: Datasheet rev 0.7 + 19-test snapshot verified in frozen file vs 24 at frozen SHA; 2026-06-05 run record (19 tests) present | PASS |
| 9 | DECL-006 STALE on undated MEMORY head: head block verified ("10 tests"; "Accepted sparse numerical library remains `TBD`") vs frozen 24-test / DEC-023 state — facts right; encoding split vs DEL-04-01/02 ALIGNED-with-note named as package risk (§3.3) | QUALIFIED |
| 10 | DECL-007 README ALIGNED: DEC-023/050/053-aware content matches frozen crate | PASS |
| 11 | `--locked` refusal handling + sweep-granularity disclosure (sweep does not name crates; pilot paired it with R1 index + own static count — exemplary) | PASS |

Tally: 9 PASS / 2 QUALIFIED / 0 FAIL.

## 3. Adjudications and cross-ledger findings

### 3.1 Absent `RESOLUTION_MATRIX.csv` citations (DEL-04-03 REQ-004, DEL-04-05 REQ-006) — QUALIFIED, string corrections recommended

`execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/`
does not exist anywhere in the frozen tree (verified by full-tree find). Both
rows transcribe the path from the kits' own MEMORY.md entries ("Resolution
evidence is indexed in …"), i.e. transcribed-not-fabricated, and in both rows
the substantive record — the kit `Review_Findings.csv`
TECHNICALLY_ADDRESSED_PENDING_HUMAN row — resolves in-tree and says exactly
what the rows claim; neither disposition rests on the matrix. This matches the
R0 calibration precedent (non-resolvable transcribed citations = QUALIFIED)
and the W1 precedent of string corrections on SOUND ledgers. **Recommended
owning-pilot string correction:** append a convention-7-style marker
(`record not present in tree at frozen SHA 551f84ef6`) to both cells. Note
the DEL-04-01 pilot independently detected this absence and declined to cite
the directory — the correction brings 04-03/04-05 to that standard.

### 3.2 DEL-04-02 crate-README census exclusion — QUALIFIED

`core/solver/straight_pipe/README.md` self-identifies as "the bounded
implementation slice for `DEL-04-02`" — the same species the other five
ledgers censused as DECL-007. The pilot excluded it citing the W1
DEL-03-07/08 precedent (README-in-deliverable-folder reading); W1 DEL-02-02
censused its crate README under the opposite reading, and both W1 ledgers
were verified SOUND. The addendum-1 phrase "deliverable-owned in-tree README"
has verified precedent on both sides, and the exclusion is disclosed with the
README carried as implementation evidence, so this is a QUALIFIED judgment,
not a defect. **Harmonization recommendation for W3–W5 and R3:** census any
product-tree README that textually self-identifies as the deliverable's
implementation slice; DECL-row counts are otherwise not comparable across
ledgers (6 vs 7 inside this one package).

### 3.3 Package-specific items from the dispatch (adjudicated against the frozen tree)

1. **Procedure-surface variance (DEL-04-04 ALIGNED vs DEL-04-05 STALE): both
   convention-correct on their facts.** The two Procedures are materially
   different documents. DEL-04-04's defines a documentation-setup procedure,
   accurately describes its own (completed) scope, and — decisively — was
   refreshed post-implementation: step 3 states the DEC-067 state-switched
   decision boundary as *implemented* current mechanics. Nothing in it
   misdescribes the frozen slice → ALIGNED holds. DEL-04-05's declares "the
   current setup-only boundary" as present-tense current state (false since
   the recorded 2026-05-01 implementation authorization), carries an
   overtaken "current value is `TBD`" threshold prerequisite, and received no
   refresh → widened addendum-4 STALE holds. Neither ledger is defective on
   this row; the variance is fact-driven, not convention drift.
2. **DEL-04-01 DECL-006 (MEMORY ALIGNED on corrected-in-file basis):
   convention-permitted — QUALIFIED.** Facts verified: the undated
   `## Open TBDs` header block still lists the sparse library as TBD while
   the dated 2026-06-11 entry records the DEC-023 resolution. Addendum 1's
   protection covers *dated* entries; the pilot did not claim that protection
   for the header — it disclosed the undated-block judgment in-row and in a
   friction note, and the remaining header TBDs (tolerance policy, unit
   basis) are genuinely open. ALIGNED-with-note is defensible where the drift
   is a single line corrected in-file. **However** the package now carries a
   three-way treatment of undated MEMORY head blocks (04-01 and 04-02
   ALIGNED-with-note on light drift; 04-06 STALE on heavy drift — 10-vs-24
   tests, 5-vs-10 envelope fields, overtaken sparse TBD). The degree-based
   split is reasonable and all three are self-flagged, but a run-level rule
   for undated MEMORY head sections is needed before W3 (cross-ledger risk
   below).
3. **DEL-04-03 DECL-005 bootstrap gate/source scoping: aggregation-safe —
   QUALIFIED.** `RecordedRemaining` is byte-exact including the bootstrap;
   gate/source cells list values for the non-bootstrap residual only. W1
   itself split on this cell shape (DEL-02-05 annotated the bootstrap gate
   in-cell; DEL-01-03 excluded it), and addendum 2 requires excluding the
   bootstrap from gate analysis, which the exclusion variant satisfies
   directly (1:1 value-to-residual correspondence; no annotated token for
   sweeps to strip). W2 uses the exclusion variant uniformly (04-03/04/05).
   Not a defect; recommend R3 standardize on the W2 exclusion variant.
4. **Byte-identical out-of-tree copy re-execution (convention ruling
   requested by DEL-04-03): all three uses sound — QUALIFIED convention note
   for R3.** Uses: DEL-04-01 (frame_kernel), DEL-04-02 (straight_pipe +
   frame_kernel), DEL-04-03 (linear_supports + frame_kernel). All three
   preserved addendum-9 side-effect freedom: no write into the frozen tree
   (the `--locked` refusals wrote nothing), external `CARGO_TARGET_DIR`,
   porcelain verified empty before/after (and the tree is porcelain-clean
   now). Evidence value: source identity is what carries the result to the
   frozen SHA — DEL-04-01/02 proved it independently (`diff -r` clean);
   DEL-04-03 relied on `cp -R` copy semantics without recording an
   independent diff (acceptable, weaker). All three disclose the copy-run
   in-row and never represent it as an in-tree run. The complementary path
   (04-04/05/06: recorded sweep pass + actually-run addendum-10 diff, no
   copy-run) is equally sound. **Proposed R3 convention line:** a copy-run
   counts as re-execution at frozen content iff (a) the copy's byte identity
   to the frozen paths is independently verified and recorded (`diff -r` or
   hash), (b) build artifacts are external, (c) frozen-tree porcelain is
   verified empty before/after, and (d) the row discloses the copy basis.
   None of the W2 uses was unsound; only (a) needs tightening for the
   DEL-04-03 pattern.
5. **W1 calibration items:** rev-0.7→0.8 pointer drift — encoded STALE-side
   on the Datasheet DECL row of all six ledgers with the owner-calibration
   caveat stated exactly once per notes (rev-0.7 cites verified present in
   all six frozen Datasheets; frozen decomp header rev 0.8
   `current_basis`). Overtaken Gate-C review prose — no instance arises in
   PKG-04 (recorded 2026-06-05 reviews are cited as records, not as
   bind-current evidence). Addendum-10 qualifiers — all backed by actually-
   run diffs, independently re-verified (§1). SECURITY marker/routing — N/A
   (zero SECURITY rows).

### 3.4 SourceReliability ladder variance (cross-ledger risk, not a defect)

DEL-04-01 marks its REQ rows (and REM-002) `REVIEWED`; DEL-04-03/04/05/06
mark equivalent REQ rows `UNVERIFIED`; DEL-04-02 marks `REVIEWED` only where
a named ACCEPT_AS_IS disposition covers the record. All are individually
defensible under addendum 6 (each kit passed the human-ruled 2026-06-05
blocker-closure/lifecycle-readiness review, so "named human ruling covering
the cited record" is arguable either way; R0b DEL-05-03 used the REVIEWED
reading and was verified SCALE-READY, W1 PKG-02/03 used the UNVERIFIED
reading). No single ledger is wrong, but package-level SourceReliability
histograms are not comparable. Same species as the W1 PKG-00 risk-2 note —
carry to R3 dedupe; candidate one-line rule: REVIEWED on a REQ row requires a
human ruling/disposition naming that requirement's record specifically, not
kit-level review coverage.

### 3.5 Evidence-execution audit summary

Re-executions claimed: DEL-04-01 copy-run 36/36; DEL-04-02 in-tree `--locked`
benchmark 33/33 + copy-run 33/33 + fmt; DEL-04-03 copy-run 14/14; DEL-04-04
in-tree `--locked` nonlinear benchmark 19/19 + pytest 8/8; DEL-04-05 pytest
2/2; DEL-04-06 none (refusal only). Static-test counts independently
recounted from frozen sources by this verifier: frame_kernel 36,
straight_pipe 33, linear_supports 14, nonlinear_supports 19,
nonlinear_integration 16, performance_harness 20, diagnostics 24 — all match
the ledgers and `VERIFICATION_INDEX.csv` (RUST-24..34 rows cross-checked).
The recorded DEC-025 sweep JSON, all cited DEC/D-XX ruling records
(D-03/05/05b/15/16/17/19/25/26/27/34/35), both LifecycleCorrection
Decision_Logs, the blocker-closure packet, review snapshots, PRD/mechanics
plans, and all cited hand-calc witnesses resolve in the frozen tree. The two
ATTESTED markers (DEV-001 dispatch; Receipt 6) correspond to genuinely absent
records — correctly marked. The only non-resolving unmarked citations are the
two RESOLUTION_MATRIX.csv cells (§3.1).

### 3.6 Additional convention gaps surfaced by this package (for R3)

- ACCEPTED_DIVERGENCE as the substance disposition for *overtaken setup-pass
  exclusion prose* (DEL-04-05 EXC-001): convention 1 bars staleness tokens on
  exclusion rows and addendum 11's bar is met, but the divergence species
  ("exclusion text overtaken by permitted scope evolution") has no dedicated
  encoding; DEL-04-01 EXC-001 encoded the temporally-scoped variant ALIGNED.
  Both disclosed; R3 should aggregate these two shapes knowingly.
- Addendum-13 scope: three pilots handled pending-human review findings three
  ways (04-02 MEDIUM+OWNER by analogy; 04-04/05 MEDIUM+REVIEW; 04-06
  HIGH+REVIEW on independent-grounds rows). W1 already queued the
  OWNER/REVIEW variance for R3; add the cap-scope question (validation
  evidence only vs any pending-human record).
- `AuthorityNeeded` semantics on ALIGNED REMAINING_WORK rows (authority to
  perform the recorded work vs to resolve the row) — 04-04/05 used the
  perform-the-work reading (ENGINEERING); consistent intra-wave, needs a
  definition line.

## 4. Fence compliance

All six ledgers and notes: no lifecycle/DAG/scope mutation proposed as
operative (repair candidates are routed via `AuthorityNeeded`/`RemainingWork`
only); no F-PIP-1..5 claim language outside clearly attributed quotes (rows
quoting "release use"/"R5 release evidence"/compliance-exclusion text say so
in-row); agent dispositions nowhere phrased as rulings; `SelectableUnderCurrentLoop`
kept mechanical with the owner suspension run-level; gate-state cells reflect
the frozen register (D-41 AWAITING_RULING) per the RUN_BASIS codification.
DEL-01-01 is not in this package. Each pilot's writes were confined to its
two output files; this verifier wrote exactly this one file.

## 5. Frozen-tree status

`git -C FROZEN status --porcelain` empty and HEAD
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` re-verified at verification start
and end. All verification operations were read-only (git diff/merge-base/
status, grep, file reads); no build or test execution was performed by this
verifier.

## 6. Package summary line

**PKG-04: 6/6 ledgers SOUND (133 rows; spot-checks 55 PASS / 13 QUALIFIED /
0 FAIL); two recommended string corrections (absent RESOLUTION_MATRIX.csv
pointer on DEL-04-03 REQ-004 and DEL-04-05 REQ-006 — add the not-present-in-
tree marker); cross-ledger risks for R3: crate-README census grain (6-vs-7
DECL counts), undated-MEMORY-head treatment, SourceReliability ladder on
reviewed kits, ACCEPTED_DIVERGENCE species for overtaken exclusions,
addendum-13 scope/routing, AuthorityNeeded semantics on REM rows, and the
byte-identical copy-run convention (proposed rule in §3.3 item 4).**
