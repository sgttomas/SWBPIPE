# W2 Fan-in Verification — PKG-05 (DEL-05-01..05)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W2 fan-in (owner-ruled
adversarial pass, fable at high effort). Frozen evidence tree
`.claude-worktrees/piping-frozen-551f84ef6`, HEAD verified
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`; `git status --porcelain` **empty
before and after every verification operation** (checked at start, after each
re-execution, after each diff, and at close). All findings below are
agent-authored and non-binding; nothing here is an owner or engineering
ruling. No ledger was edited.

**Scope.** Five fable-pilot ledgers under `WAVES/W2/`:
DEL-05-01 (25 rows), DEL-05-02 (23), DEL-05-03 (19), DEL-05-04 (21),
DEL-05-05 (21) — row counts, 20-column headers, RFC-4180 parse, ClaimID
form, and notes-histogram reproduction all re-derived programmatically from
the CSVs. Verification scope per the steer: every self-flagged row, every
non-ALIGNED row, ≥2 ALIGNED spot-checks per ledger, mechanical convention
sweeps, fences, and the seven package-specific adjudication items from the
dispatch. PKG-05 strictness applied throughout: verification ≠ validation,
no unit-test promotion to engineering validation, threshold/tolerance
promotions route ENGINEERING.

**Verifier re-executions (independent, side-effect-free per addendum 9):**
- `cargo test --locked --offline` on `core/loads/stress_recovery` in place
  with external `CARGO_TARGET_DIR` in the session scratchpad: **26 passed,
  0 failed** (matches DEL-05-03's claim); porcelain empty after.
- `python3 -m pytest -p no:cacheprovider` on the four DEL-05-04 suites with
  `PYTHONDONTWRITEBYTECODE=1`: **6 passed, 0 failed**; porcelain empty after.
- Addendum-10 qualifier diffs re-run for DEL-05-01 (six path roots) and
  DEL-05-05 (`core/`, `validation/benchmarks/mechanics/`,
  `validation/hand_calcs/mechanics/`) over
  `e648462f1d05..551f84ef6`: **both empty**, confirming the
  content-identical claims as cited.
- Byte-exactness of every `DECL-005 RecordedRemaining` cell against the
  frozen `_STATUS.md ## Remaining` bytes (all five ledgers): **all equal**,
  including `§§6–8`, em-dashes, and the multi-item ` ; ` joins on
  DEL-05-01/02.

---

## 1. DEL-05-01 — Primitive load case engine

| # | Row / check | What was checked against the frozen tree | Result |
|---|---|---|---|
| 1 | Histograms | Recounted from CSV: ALIGNED 19 / STALE 6; REQ 12, EXC 5, DECL 7, REM 1 — matches notes exactly | PASS |
| 2 | REQ-008 (self-flag) | Spec line 20 "shall accept only explicit equivalent mechanics loads … unless a later sealed scope authorizes dynamic procedure generation"; frozen `lib.rs` carries `generate_seismic/wind_equivalent_static_loads` (fn defs lines 1751/1859) under DEC-068 (D-36 ruling resolves in tree); `prepare_equivalent_static_loads` boundary unchanged; dynamics absent. ALIGNED @ MEDIUM with in-row note, prose gap ledgered on DECL-001/002 | PASS (QUALIFIED: disclosed judgment; IMPLEMENTED_DIFFERENTLY would overstate — the required constraint holds) |
| 3 | REQ-009 (self-flag) | 45 `#[test]` counted in frozen `lib.rs`; Spec verification row still says "focused 40-test" (line 42) — substance ALIGNED, staleness correctly homed on DECL rows per convention 1 | PASS |
| 4 | DECL-001 STALE, OWNER | Spec 40-test claim + generator absence + "Remaining TBDs" register row partially overtaken by DEC-068 (static-equivalent generation landed; dynamics still open) — all verified in Spec | PASS (OWNER routing on the overtaken-TBD-register reading is the same judgment the DEL-04-01 ledger flagged; disclosed) |
| 5 | DECL-002 STALE | Datasheet line 91 cites decomp "revision 0.7" (frozen header: revision 0.8 `current_basis`); lines 72–73 claim 40 tests; Dynamic-scope attribute overtaken — all verified. W1 calibration item 1 form followed, caveat stated once in notes | PASS |
| 6 | DECL-003 / DECL-004 STALE (self-flag) | Guidance lines 25/36 and Procedure lines 21/38 describe explicit-input-only equivalent-static handling; landed generation surface absent from both — omission verified | PASS (QUALIFIED: omission-vs-ALIGNED judgment disclosed; MEDIUM confidence appropriate) |
| 7 | **DECL-006 STALE — dispatch item 1 (MEMORY variance)** | Frozen MEMORY: undated "Boundaries Preserved" header block states "Wind and seismic are represented only as user-supplied equivalent mechanics loads. Dynamic or code-procedure generation remains `TBD`" (lines 16–24); dated log ends 2026-06-22; `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P3-OCCLOADGEN-001.md` exists with **no** corresponding MEMORY entry; Procedure step 10 expects a closeout entry | **PASS — adjudicated correct on its own facts.** Addendum 1 bars staleness dispositions only for drift *inside dated log entries*; this staleness sits on an undated current-state header block with nothing in-file correcting it. The DEL-04-01 ALIGNED precedent rested on a later dated entry correcting the block in-file — a distinguishing fact, not a contradiction. Both ledgers apply the same operative rule ("header declaration current unless corrected in-file"); aggregation-safe. One-line codification recommended for W3+ (see §7) |
| 8 | DECL-007 STALE | Frozen crate README: scope list has no generator, generation-finding-code, or generation-input mention (grep confirms); `MissingGenerationInput`/`InvalidGenerationInput` live in the same crate | PASS |
| 9 | ALIGNED sample: DECL-005 | `_STATUS.md ## Remaining` transcribed byte-exact (2 items incl. bootstrap); K-CONFLICT-1 Decision_Log resolves; UNGATED/YES matches R1 inventory (DEL-05-01 = YES, 2 non-bootstrap-eligible rows) and addendum 2/3 handling | PASS |
| 10 | ALIGNED sample: REM-001 | Whole-span residual: `schemas/model.schema.yaml` `exposed_element_refs` (lines 621/633), `core/product_physics` `exposed_pipe_refs` blocking (lines 448/4854); run-record Boundaries language; SR=UNVERIFIED correct (validation leg agent-generated, no human disposition) | PASS |
| 11 | ALIGNED sample: REQ-004 | `lib.rs` accepts `force_per_length` (line 174), rejects retired aliases `temperature_difference`/`area_moment` (lines 195/199) | PASS |
| 12 | Evidence chain | DEC-025 sweep JSON exists: `decision_basis=DEC-025`, commit `e648462f…`, `working_tree_dirty=false`, `overall_status=pass`, `cargo_crate_sweep` surface present; addendum-10 diff re-run empty over the six cited roots; HC-MECH-006/021 witness files and benchmark fixture IDs present | PASS |
| 13 | DecisionBasis resolvability | `D-36_RULING_2026-07-09.md`, `LIFECYCLE_CORRECTION_2026-07-02_2050/Decision_Log.md`, `REV_DEL-05-01_2026-06-05_2021/` all present; `Review_Findings.csv` header-only as claimed | PASS |
| 14 | SourceReliability sweep | DECL rows all NOT_APPLICABLE (addendum 6). REVIEWED on REQ/EXC rows is grounded: `_STATUS.md` History records "2026-06-05 — Human Gate 5 approval accepted REVIEW recommendation … from REV_DEL-05-01_2026-06-05_2021" — a recorded human disposition covering the crate evidence. However REVIEWED here keys to the verification leg while the mechanics validation legs are agent-generated pending disposition | QUALIFIED (cross-ledger keying variance — see §7 risk 1; not a per-ledger defect: addendum 6 is satisfied on the leg the cell keys to, and the pending-validation status is stated verbatim in every ValidationEvidence cell) |

**Tally: 12 PASS / 3 QUALIFIED / 0 FAIL. Verdict: SOUND.** No defects.

---

## 2. DEL-05-02 — Load-case algebra engine

| # | Row / check | What was checked | Result |
|---|---|---|---|
| 1 | Histograms | ALIGNED 21 / STALE 2; REQ 9, EXC 5, DECL 7, REM 2 — matches notes | PASS |
| 2 | DECL-002 STALE | Datasheet line 47 cites revision 0.7; frozen decomp header 0.8 `current_basis`; W1 item-1 form, AuthorityNeeded=NO (no overtaken TBD register on the surface) | PASS |
| 3 | DECL-003 STALE, REVIEW (self-flag) | Guidance line 20 verbatim: "DEL-05-01 and DEL-05-04 remain pending dependency maturity items in the local register"; frozen `Dependencies.csv` E0451/E0453 `SATISFIED` via the 2026-06-05 blocker-closure ruling (ruling packet resolves in tree and names both closures) | PASS (QUALIFIED: item-3 analog extended from review prose to dependency-maturity prose — STALE-side, ruling as DecisionBasis, aggregation-safe; harmonization line recommended, see §7 risk 5) |
| 4 | REM-001 ALIGNED, D-38 | `_STATUS` item 1 byte-exact; D-38 register row `AWAITING_RULING` at frozen SHA with packet `D-38_temperature_interpolation_policy.md` present; run record MODULUSBASIS Boundaries language; `schemas/material.schema.yaml` exact-match/no-interpolation posture; Selectable=NO correct (gated) | PASS |
| 5 | REM-002 ALIGNED, D-38 (self-flag) | `_STATUS` item 2 byte-exact; D-38 register scope is "Temperature interpolation policy for material property bases" — the shear-modulus residual's recorded gate is transcribed mechanically with the possible over-breadth disclosed in `RemainingWork` | PASS (QUALIFIED: REMAINING_STATE_MISMATCH would have overstated an owner-consolidated register row; the conservative transcription is the aggregation-safe call) |
| 6 | REQ-005 SECURITY (self-flag) | Structural absence verified: no parser/eval surface in frozen `lib.rs` public API; E0616 evaluator interface `RETIRED`, human-ruled non-gating (`NOT_APPLICABLE` for the review cycle, verbatim in Dependencies.csv and the ruling packet). No owner-gated sufficiency deferral exists → the convention-6 deferral marker + W1 OWNER harmonization do not attach (they bind *owner-gated deferrals*, PKG-02 risks 1–2) | PASS (reading conforms; EXC-004 same) |
| 7 | REQ-003/REM-001 SR=UNVERIFIED (self-flag) | 2026-07-10 TP-PMM-P3 witness/benchmark legs agent-generated, no human disposition — addendum 6 gives UNVERIFIED; other REQ rows key to the 2026-06-05 human-ruled crate evidence | PASS (deliberate split disclosed; feeds §7 risk 1) |
| 8 | DECL-001 ALIGNED at contract grain (self-flag) | Additive RangeMode helpers (2026-06-12; 17→18 tests) and modulus-basis records contradict no Specification sentence; 18 `#[test]` counted in frozen lib.rs | PASS (W1 calibration item "grain splits histogram as judgment splits" applies) |
| 9 | ALIGNED samples: DECL-005, DECL-007 | `_STATUS` 3 items byte-exact incl. two `gated: D-38` suffixes → GateOrStageConstraint `gated: D-38; gated: D-38`, Selectable NO (matches R1 inventory NO); README self-names "the bounded implementation slice for DEL-05-02" (line 3) | PASS |
| 10 | Evidence chain | 18-test re-execution claim corroborated by frozen `#[test]` count + sweep pass + addendum-10 window claim (docs-exclusion clause carried per W1 item); ruling packet and `Review_Findings.csv` ACCEPT_AS_IS/RESOLVED verified | PASS |
| 11 | AuthorityNeeded=D-38 token | Gate-named routing token has W1 precedent (`D-07b` in DEL-01-02/DEL-01-03, verified in W1 as SOUND) | QUALIFIED (R3 must fold gate-named tokens into the routing families; see §7 risk 2) |

**Tally: 8 PASS / 3 QUALIFIED / 0 FAIL. Verdict: SOUND.** No defects.

---

## 3. DEL-05-03 — Fundamental stress recovery module

| # | Row / check | What was checked | Result |
|---|---|---|---|
| 1 | Histograms | ALIGNED 16 / ACCEPTED_DIVERGENCE 1 / STALE 2; REQ 9, ACC 1, EXC 2, DECL 7 — matches notes; R0b re-encode deltas (bootstrap row eliminated, census 3→7, UNMAPPED row not re-minted) all conform to addenda 1/2/8 | PASS |
| 2 | **ACC-001 — dispatch item 4 (ACCEPTED_DIVERGENCE + ENGINEERING)** | Spec `## Verification` "Hand calculations" row verbatim: "Current tests use synthetic/cleared values. Exact release hand-calc, benchmark publication, and tolerance acceptance scope remain `TBD`" (line 37) — a distinct acceptance-scope claim, so the single ACC row at addendum-12 grain is justified. Named record: `PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md` (human ruling, "Implement the proposed plan") + `REV_DEL-05-03_2026-06-05_2120/` whose Gate-4 rationale accepts that "residual TBDs remain explicit" and whose RUN_SUMMARY names the exact deferrals ("conversion catalog, production tolerance policy, release benchmark scope … remain explicit TBDs") | **PASS (QUALIFIED).** Addendum-11 adjudication: the human-approved packet acted on DEL-05-03 (finding dispositions, dependency closures, review recommendation) *with these TBDs named as accepted residual boundaries* — that is a named human record permitting the deferred acceptance-scope state, satisfying addenda 5/11, though it is the package's thinnest permission record (acceptance-by-rationale rather than an explicit deferral grant). The pilot disclosed all three stacked judgments. Routing: **ENGINEERING is correct, not addendum-13 OWNER** — addendum 13 routes OWNER when the disposition *rests on* pending validation evidence; here the disposition rests on the ruling (independent grounds, exactly as addendum 13 demands), Confidence is MEDIUM, and defining numeric tolerance/threshold scope is an engineering adjudication under the W2 solver-mechanics strictness. Not addendum-13 OWNER territory. |
| 3 | **REQ-003 — dispatch item 2a (ENGINEERING on ALIGNED)** | Spec Unit-safety verification row verbatim: "conversion constants remain `TBD`" (line 32); DEL-02-02 candidate home checked in frozen tree — its `## Remaining` carries only the D-41 bootstrap item (verified), so the deferral is declared-prose on both sides; substance ALIGNED with the W001 human disposition as DecisionBasis (packet lists `DEL-05-03-PKG02-W001` ACCEPT_AS_IS/RESOLVED) | PASS (QUALIFIED: routing the *resolving* authority on an ALIGNED row is a disclosed semantic choice — defensible, but inconsistent with DEL-05-05's reading; see item 2 adjudication in §7 risk 2) |
| 4 | REQ-005 OWNER on ALIGNED | Same resolve-authority reading (persistence-ownership TBD, no candidate home names it); W002 disposition verified in packet | PASS (same qualification) |
| 5 | **DECL-007 — dispatch item 3a (README STALE)** | Frozen README: 60 lines, **zero** occurrences of "station"/"sweep"/"modulus" vs three public station functions (`recover_station_stresses` line 542, `recover_station_stress_sweep` line 556, `from_station_resultants`) and `recover_stress_range_with_modulus_basis` (line 784) / `StressRangeModulusBasisRecord` (line 384) in the same crate | PASS — enumerated Scope/Verification lists missing landed public surface; widened addendum-4 STALE is convention-correct; consistent with DEL-05-05-DECL-007 and DEL-05-01-DECL-007 (see §7 risk 4) |
| 6 | DECL-002 STALE | Datasheet line 22 enumerates exactly seven API functions (modulus-basis pair absent); line 64 cites revision 0.7 — both drift facts verified | PASS |
| 7 | REQ-001/008/009 SR=UNVERIFIED (self-flag) | Load-bearing hand-calc witness/benchmark legs agent-generated (R1 index UNVERIFIED); dispositions stay ALIGNED because the Specification itself declares validation acceptance open — no unit-test promotion | PASS (the R0b→W2 REVIEWED→UNVERIFIED flip follows addendum 6; correct under the W2 strictness) |
| 8 | DECL-001 ALIGNED at contract grain (self-flag) | 2026-07-10 additive extension contradicts no Spec sentence (behavior-class prose, no exhaustive enumeration — unlike the Datasheet/README) | PASS |
| 9 | ALIGNED samples: REQ-004, DECL-005 | Finding/blocking surface in frozen lib.rs; `_STATUS` Remaining = bootstrap only, byte-exact; K-CONFLICT-1 log resolves; Selectable NO matches R1 inventory | PASS |
| 10 | Verifier re-execution | `cargo test --locked --offline` on the frozen crate, external target dir: **26 passed, 0 failed**; porcelain empty after — independently confirms the pilot's in-place addendum-9 run | PASS |
| 11 | Evidence chain | 26 `#[test]` counted; benchmarks/stress 22 `#[test]` counted; witness fixture paths and `tp_phys_015` witness JSON present; sweep record verified; product_physics line 6462 provenance string verbatim (DECL-006 dated-drift note) | PASS |

**Tally: 9 PASS / 2 QUALIFIED / 0 FAIL. Verdict: SOUND.** No defects.

---

## 4. DEL-05-04 — Analysis status semantics

| # | Row / check | What was checked | Result |
|---|---|---|---|
| 1 | Histograms | ALIGNED 18 / PARTIALLY_IMPLEMENTED 1 / STALE 2; REQ 14, EXC 1, DECL 6 — matches notes; 6-row DECL census correct (no deliverable-owned README exists; the architecture note is deliverable-authored product content, not a census surface — reasonable under addendum 1) | PASS |
| 2 | **REQ-014 — dispatch item 5 (PARTIALLY_IMPLEMENTED)** | Spec line 24 (requirement) and line 47 (verification approach: "Negative tests for automatic approval/compliance plus stale-hash acceptance reuse") verified verbatim; Procedure fixture-selection bullet verified. Negative automatic-approval/compliance tests exist (FORBIDDEN_AUTOMATIC sweep at `tests/test_analysis_status_schema.py:189`); `bound_hashes` required-member assert at line 138. Verifier swept the **whole** frozen `tests/` dir: no test exercises acceptance reuse/invalidation after a bound-content hash change | **PASS — disposition and MEDIUM confidence verified.** One evidence nuance, disposition-neutral: the row's "the frozen suite asserts only that bound_hashes is a required member" under-reports two adjacent *declaration-level* asserts the verifier found — `tests/test_persistence_schema.py` (~line 236–239) asserts `invalidates_on_hash_change` const `true` and `tests/test_model_state_schema.py` (line 164) asserts `hash_invalidates_external_acceptance` const `true`. Both verify that schemas *declare* the invalidation semantic; neither exercises stale-hash acceptance reuse, so the kit-named negative test is still missing and PARTIALLY_IMPLEMENTED stands. Worth carrying into any future repair note. |
| 3 | REQ-008 ALIGNED at record-contract grain (self-flag) | `HumanAcceptanceRecord` contract fields verified in frozen schema; the single runtime-invalidation gap homed once on REQ-014 (no double-counting) — clean altitude split | PASS (QUALIFIED: disclosed grain call; defensible either way per W1 calibration item on grain splits) |
| 4 | REQ-010 ALIGNED at present-test grain (self-flag) | Anti-collapse tests exist and pass; release gates are future-by-design and release readiness is an explicit kit exclusion | PASS (QUALIFIED: disclosed) |
| 5 | EXC-001 merged row (self-flag) | One Scope sentence, one posture, one evidence base; addendum-12 acceptance-grain rule not violated (no ACC rows merged) | PASS |
| 6 | DECL-002 / DECL-004 STALE | Datasheet line 60 and Procedure line 12 cite revision 0.7 (frozen header 0.8) — verified; AuthorityNeeded=NO correct (pointer-only; the kit's TBD registers all remain open, checked) | PASS |
| 7 | DECL-006 MEMORY (self-flag) | MEMORY is dated-log-only; 2026-06-17 "Lifecycle Housekeeping" heading verified vs the ruling's recorded 2026-06-16 header reversal — in-row historical note per addendum 1, no staleness disposition | PASS |
| 8 | Verifier re-execution | The four cited suites re-run read-only at the frozen SHA (`PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider`): **6 passed, 0 failed**; porcelain empty | PASS |
| 9 | ALIGNED samples: REQ-007, DECL-005 | `test_downstream_automatic_status_surfaces_exclude_human_approval_and_passed_claims` exists at line 189; `_STATUS` Remaining = bootstrap only, byte-exact; DEL-05-04-PKG02-I001 ACCEPT_AS_IS verified in packet + Review_Findings | PASS |
| 10 | SR sweep | REVIEWED grounded on the 2026-06-05 human blocker-closure ruling, which names DEL-05-04 directly (lifecycle ruling `IN_PROGRESS -> CHECKING` + I001 disposition) — the strongest human-coverage basis in the package; no addendum-13 pending-disposition cap applies (I001 is RESOLVED) | PASS |

**Tally: 8 PASS / 2 QUALIFIED / 0 FAIL. Verdict: SOUND.** No defects.

---

## 5. DEL-05-05 — Concentrated and distributed user load application

| # | Row / check | What was checked | Result |
|---|---|---|---|
| 1 | Histograms | ALIGNED 17 / STALE 4; REQ 10, EXC 4, DECL 7 — matches notes | PASS |
| 2 | **DECL-007 — dispatch item 3b (README STALE)** | Frozen README: **zero** occurrences of "straight", "axial", "station", or "oriented" — the entire R7–R10 recovery family (both `apply_straight_pipe_equivalent_user_loads*` public APIs, 13 of 28 tests) absent from Scope/Verification; boundary/unit paragraphs current | PASS — same enumerated-declaration rule as DEL-05-03-DECL-007; convention-correct and mutually consistent (see §7 risk 4; the HIGH-vs-MEDIUM confidence spread between the two rows is immaterial to aggregation) |
| 3 | **REQ-005 — dispatch item 2b (threshold TBDs routed NO)** | Spec Documentation (line 80) and Datasheet Remaining TBDs (line 27) verified verbatim: "production tolerance policy, release thresholds … remain TBD"; no numeric threshold value exists anywhere in the frozen records to promote | PASS (QUALIFIED — adjudication in §7 risk 2: defensible on its own facts. The row records a declared bounded TBD with no numeric claim to adjudicate, so NO does not corrupt substance dispositions; it diverges from DEL-05-03's resolve-authority reading of the *same corpus-level tolerance-policy TBD*, which DEL-05-03-ACC-001 already routes ENGINEERING. R3 must not read PKG-05's ENGINEERING count as a census of open engineering adjudications) |
| 4 | **REQ-007..010 SR=UNVERIFIED — dispatch item 6** | Validation legs verified in the frozen tree: `validation/benchmarks/mechanics/Cargo.toml` line 19 depends on `open_pipe_stress_user_loads`; `src/lib.rs` imports and calls `apply_straight_pipe_equivalent_user_loads` (lines 43–44, call sites 3033/3152 et al.); witnesses agent-generated, no recorded human disposition on the witness set (R1 index) | **PASS — addendum-6-conformant.** For mechanics-class rows the validation leg is load-bearing under the W2 strictness; agent-generated + agent-audited + pending human disposition = UNVERIFIED is the ladder's literal middle rung. Crate verification being human-ruled (2026-06-05) does not lift the *validation* legs to REVIEWED. Confidence HIGH on independent grounds is coherent (confidence grades the alignment finding, SR grades the evidence source). Divergence from DEL-05-01's keying is a cross-ledger risk (§7 risk 1), not a defect here — this ledger is the stricter, brief-consistent encoding. |
| 5 | REQ-001 grain (self-flag) | Distributed/station categories benchmark-validated; generic nodal DOF-mapping path unit-test-verified only, with "no separate numeric validation basis applies at input-vocabulary grain" stated in-row — no promotion of unit tests to validation occurs | PASS (QUALIFIED: disclosed; VERIFIED_NOT_VALIDATED would overstate for an input-vocabulary claim) |
| 6 | EXC-004 ALIGNED (self-flag) | Exclusion sentence's "unless separately dispatched" branch verified against MEMORY 2026-06-11/12 GUI tranches (separately dispatched WORKING_ITEMS; no `user_loads` source change); no permitting-record question arises because nothing is divergent — ACCEPTED_DIVERGENCE would require a deferral, and addendum 11's threshold cuts the other way here | PASS |
| 7 | DECL-001/002/004 STALE | Spec line 49, Datasheet line 72, Procedure line 11 each cite revision 0.7 (frozen 0.8 `current_basis`) — three pointer-drift rows, W1 item-1 form, caveat once, AuthorityNeeded=NO | PASS |
| 8 | DECL-006 MEMORY ALIGNED | Undated header sections verified present and current; 2026-05-16 "HumanDisposition stays TBD" superseded in-file by the dated 2026-06-05 entry; dated-entry filing disorder verified (05-17 slices at lines 89/118 precede 05-16 slices at 134/159) — note-only per addendum 1, consistent with the package MEMORY rule | PASS |
| 9 | ALIGNED samples: REQ-010, DECL-005 | `apply_straight_pipe_equivalent_user_loads_with_axial_effects` in frozen lib.rs reusing `open_pipe_stress_straight_pipe` types (no duplicated axial mechanics); `_STATUS` Remaining = bootstrap only, byte-exact; Selectable NO matches R1 inventory | PASS |
| 10 | Evidence chain | 28 `#[test]` counted in frozen lib.rs (matches the 2026-06-05 ruling-recorded count and RUST-06); addendum-10 diff re-run by the verifier over the three cited roots: **empty**; exclusion clause carried per W1 item; sweep record verified; W001/W002 ACCEPT_AS_IS verified in packet | PASS |

**Tally: 8 PASS / 2 QUALIFIED / 0 FAIL. Verdict: SOUND.** No defects.

---

## 6. Convention-conformance sweeps (mechanical, all five ledgers)

- **Addendum 1 census:** 7/7/7/6/7 DECL rows; each census enumerated and
  plausible (DEL-05-04's 6 is correct — no deliverable-owned README exists;
  the other four crates' READMEs self-name their deliverable, verified).
- **Addendum 2 bootstrap:** no bootstrap claim rows anywhere; the seeded
  item lives verbatim only in `DECL-005 RecordedRemaining` (byte-exact in
  all five, verified programmatically) and is excluded from residual/gate/
  selectability analysis.
- **Addendum 3 / convention 5 gate columns:** multi-residual joins on
  DEL-05-01 (UNGATED → YES) and DEL-05-02 (`gated: D-38; gated: D-38` → NO)
  derive mechanically and reproduce the R1 inventory grain (YES only for
  DEL-05-01).
- **Addendum 6 ladder:** all 34 DECL rows NOT_APPLICABLE; every REVIEWED
  cell traced to a named human ruling/disposition (2026-06-05 blocker-closure
  packet for 05-02/03/04/05; the recorded 2026-06-05 human Gate-5 approval in
  `_STATUS.md` History for 05-01); every UNVERIFIED cell traced to
  agent-generated pending-disposition evidence. Leg-keying variance recorded
  as §7 risk 1.
- **Addendum 11:** exactly one ACCEPTED_DIVERGENCE in the package
  (DEL-05-03-ACC-001), adjudicated above; no other row needed the
  precedence check (no residual+ruling coincidences found).
- **Addendum 12:** ClaimID forms all conform (REM token has W1 precedent);
  acceptance-row grain honored (mirrored verification tables not minted;
  the one distinct acceptance-scope claim got exactly one ACC row); path
  alias declared once per notes; defaults correct on no-item rows.
- **Addendum 13:** no row rests its disposition on
  TECHNICALLY_ADDRESSED_PENDING_HUMAN evidence (the only candidate,
  ACC-001, rests on the ruling; pending witnesses are cited as
  corroboration and the row is MEDIUM anyway). No cap violations.
- **Convention 8 / addendum 10 overtaken-evidence bar:** every ALIGNED row
  citing the DEC-025 sweep carries the content-identical qualifier with an
  actually-run diff (two re-run independently by the verifier, both empty)
  and the W1 docs-exclusion clause where the window differs. DEL-05-04
  needed no qualifier (direct re-execution at the frozen SHA). W1
  calibration items 2 (addendum-10 truncation) and 5 (byte-exact bootstrap)
  fully honored.
- **Addendum 9 soundness (dispatch item 7):** two patterns in use — byte-
  identical scratch-copy (05-01/02/05, forced by missing `Cargo.lock`) and
  in-place with external `CARGO_TARGET_DIR` (05-03, `--locked` accepted the
  committed lockfile; 05-04 read-only Python). Both leave the frozen tree
  byte-untouched (porcelain empty at every check, including after the
  verifier's own runs); the scratch-copy pattern's `diff -r` identity
  premise makes its results equivalent to in-tree execution. **Both
  conforming; recommend naming both as blessed W3+ patterns.**
- **Histogram reproduction:** all five notes histograms recomputed from the
  CSVs — exact matches.

## 7. Cross-ledger consistency risks (for the PKG-05 summary / R3)

1. **SourceReliability leg-keying on mechanics rows.** DEL-05-01 keys
   REVIEWED to its human-Gate-5-covered crate evidence while its mechanics
   validation legs are agent-generated pending disposition; DEL-05-02/03/05
   key UNVERIFIED to the load-bearing validation leg on equivalent rows.
   Both are addendum-6-literal; they are mutually inconsistent for R3 SR
   histograms. R3 must not read `SourceReliability=REVIEWED` on
   MECHANICS-class rows as "human-validated mechanics" anywhere in this
   package (no PKG-05 mechanics witness set has a human disposition).
   Recommend the DEL-05-05 friction-note rule for W3+: SourceReliability
   follows the weakest load-bearing evidence leg.
2. **AuthorityNeeded semantics on declared-TBD ALIGNED rows** (dispatch
   item 2 adjudication). DEL-05-03 routes the authority that would
   *resolve* a declared TBD (REQ-003 ENGINEERING, REQ-005 OWNER, ACC-001
   ENGINEERING); DEL-05-05 routes NO because no adjudicable claim exists
   (REQ-005/006/010); DEL-05-02 uses gate-named tokens (D-38, with W1
   D-07b precedent). **Both readings are defensible on their own facts and
   neither corrupts substance dispositions — but the package's
   ENGINEERING/OWNER counts are not a census of open adjudications.** The
   corpus-level production-tolerance/release-threshold TBD is routed exactly
   once (DEL-05-03-ACC-001, ENGINEERING) — R3 should dedupe on that row and
   treat DEL-05-05-REQ-005's NO as non-counting rather than missing.
   Recommend a precedence line before W3.
3. **MEMORY-surface rule** (dispatch item 1 outcome): the package applies
   one coherent rule — undated header declarations judged current-state
   (STALE when overtaken with no in-file correction: 05-01; ALIGNED when
   corrected in-file or purely historical: 05-02/03/04/05) — and it is
   consistent with the DEL-04-01 precedent. Recommend codifying the
   "corrected-in-file" distinguisher as a one-liner so W3–W5 don't re-litigate.
4. **README enumerated-declaration grain** (dispatch item 3 outcome): all
   three PKG-05 crate READMEs with material public-API omissions encoded
   STALE (05-01/03/05 DECL-007) under the same rule — enumerated scope/API
   lists missing a landed public surface are stale; narrative prose judged
   at contract grain (05-02 DECL-007 ALIGNED, verified current). Internally
   consistent; recommend adopting the DEL-05-03 friction-note wording as the
   W3+ rule.
5. **Overtaken-prose analogs:** DEL-05-02-DECL-003 extends the W1 item-3
   encoding from review prose to human-ruled dependency-maturity prose
   (STALE + ruling DecisionBasis + REVIEW). Aggregation-safe; generalize the
   rule ("kit prose contradicted by a human-ruled register state → STALE
   with the ruling as DecisionBasis") for W3–W5.
6. **DEL-05-04 stale-hash evidence pointer:** any future repair of the
   REQ-014 gap should also reference the declaration-level invalidation
   const-flags in `tests/test_persistence_schema.py` and
   `tests/test_model_state_schema.py` (found by this verification; absent
   from the row's evidence inventory; disposition unaffected).

## 8. Fence compliance

Discovery and verification both read-only: frozen-tree porcelain empty
before and after every operation in this pass (including the verifier's
cargo/pytest re-executions, which used an external scratchpad target dir /
`PYTHONDONTWRITEBYTECODE=1`); no ledger, register, `_STATUS.md`, or
product file was modified; this report is the only file written. No
lifecycle transition, DAG mutation, or scope change is proposed as
operative anywhere in the five ledgers (document-repair `RemainingWork`
text is uniformly marked "not applied by this discovery"). No F-PIP-1..5
claim language appears outside clearly attributed quotes of exclusion/
disclaimer prose (checked on every EXC/GOVERNANCE row). All dispositions
are phrased as agent judgments routed through `AuthorityNeeded`; none is
represented as an owner or engineering ruling. No ISSUED deliverable is in
this package (DEL-01-01 check not applicable).

## 9. Verdicts

| Ledger | Verdict | Defects | Spot-check tally (PASS/QUALIFIED/FAIL) |
|---|---|---|---|
| DEL-05-01 | **SOUND** | none | 12 / 3 / 0 |
| DEL-05-02 | **SOUND** | none | 8 / 3 / 0 |
| DEL-05-03 | **SOUND** | none | 9 / 2 / 0 |
| DEL-05-04 | **SOUND** | none | 8 / 2 / 0 |
| DEL-05-05 | **SOUND** | none | 8 / 2 / 0 |

**Package summary line:** PKG-05 fan-in — 5/5 ledgers SOUND, 0 DEFECTIVE;
109 rows verified against the frozen tree at `551f84ef6` with 45 recorded
checks (45 PASS-or-QUALIFIED, 0 FAIL, 12 QUALIFIED judgment notes, all
disclosed by the pilots); one ACCEPTED_DIVERGENCE (DEL-05-03-ACC-001,
addendum-11-adjudicated with ENGINEERING routing upheld); frozen tree
porcelain empty throughout; six cross-ledger harmonization items carried to
the package summary and R3 (SR leg-keying, AuthorityNeeded TBD-routing
semantics, MEMORY header rule, README enumeration grain, overtaken-prose
analog, REQ-014 evidence pointer).
