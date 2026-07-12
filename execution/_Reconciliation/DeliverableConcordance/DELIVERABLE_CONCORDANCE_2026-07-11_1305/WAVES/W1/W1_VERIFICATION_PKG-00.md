# W1 Fan-in Verification — PKG-00 (Software Architecture Runway)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305 · R2 wave W1 fan-in checkpoint.
Verifier: fable at high effort (owner-ruled scope: all self-flagged rows, all
non-ALIGNED rows, ≥2 ALIGNED rows per ledger, convention sweeps, fences).
Frozen evidence tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
(HEAD re-verified; `git status --porcelain` empty before and after all
verification reads — no writes anywhere under the frozen worktree).
Method authority: `R1_CONVENTIONS.md` (conventions 1–8, addenda 1–13, Part C),
pinned plan §§6–7, `RUN_BASIS.md`.

**All findings below are agent-authored and non-binding. Nothing here is an
owner or engineering ruling; defect adjudications route to the orchestrator/
owner per the wave protocol.**

## Scope

Ledgers reviewed: DEL-00-01 … DEL-00-08 (`CLAIM_CONCORDANCE_*.csv` +
`NOTES_*.md`, 137 claim rows total). Package-specific dispatch items:
(a) the rev-0.7-vs-0.8 `STALE_SETUP_SPECIFICATION` encoding split across
pilots; (b) DEL-00-02 EXC-001 (run-records-hosting judgment); (c) DEL-00-05
REQ-05-04 `PARTIALLY_IMPLEMENTED` vs delegation to DEL-07-02. No
SECURITY-class rows appear in PKG-00, so the Part C convention-6 SECURITY
spot-check does not attach to this package.

---

## Package finding 1 (principal): the rev-0.7 staleness split is a REAL
## encoding divergence, and it corrupts the package staleness histogram

**Fact base (verified against the frozen tree).** The drift is textually
identical across all eight deliverables — the same three generated kit lines
in every deliverable folder:

- `Specification.md:32` — "Upstream authority is …SOFTWARE_DECOMP.md
  revision 0.7, the SCA-001/SCA-003/SCA-004 architecture-basis records, and
  approved …DAG-007…"
- `Datasheet.md:32–34` — "…SOFTWARE_DECOMP.md revision 0.7 for package and
  deliverable authority."
- `Procedure.md:8` — "…SOFTWARE_DECOMP.md revision 0.7 is the current basis."

The frozen `execution/_Decomposition/SOFTWARE_DECOMP.md` header is
`revision: 0.8`, `status: current_basis` (SCA-005/D-21 propagated; RUN_BASIS
confirms). Line ~655 of the decomposition states downstream surfaces "may be
stale relative to revision 0.8 until refreshed by their owning workflows."

**How the eight ledgers encoded the identical drift:**

| Ledger | Specification | Datasheet | Procedure | Net |
|---|---|---|---|---|
| DEL-00-01 | STALE | STALE | STALE | 3 STALE |
| DEL-00-02 | ALIGNED, **no note** | ALIGNED, **no note** | ALIGNED, **no note** | 0 STALE, drift invisible |
| DEL-00-03 | STALE | STALE | STALE | 3 STALE |
| DEL-00-04 | ALIGNED + note | STALE (other grounds) + note | ALIGNED + note | 1 STALE (other grounds) |
| DEL-00-05 | ALIGNED, **no note** | STALE (other grounds), **no rev note** | ALIGNED, **no note** | 1 STALE (other grounds), rev drift invisible |
| DEL-00-06 | ALIGNED + note (self-flagged) | ALIGNED + note (self-flagged) | ALIGNED, **no note** | 0 STALE |
| DEL-00-07 | ALIGNED + note | ALIGNED + note | ALIGNED + note | 0 STALE |
| DEL-00-08 | STALE | STALE | STALE | 3 STALE |

Nine of the package's eleven `STALE_SETUP_SPECIFICATION` rows are this one
pattern, present in three ledgers and absent from five ledgers whose kits
carry the identical text. The split is pilot-identity-driven, not
evidence-driven; a per-surface-difference defense fails because the surfaces
are the same lines. The PKG-00 staleness histogram is therefore not currently
aggregable.

**Adjudication under addendum 4** ("the surface's declaration no longer
describes the frozen implemented slice … covers … post-alignment drift; one
controlled value"): **the STALE side is convention-correct.**

1. "Revision 0.7 is the current basis" is a current-state declaration that is
   false at the frozen SHA (frozen basis is 0.8, `current_basis`). This is
   post-alignment drift in the plain sense of addendum 4 — the kits were
   aligned when refreshed to 0.7 (2026-06-04) and the world moved to 0.8.
2. The corpus's own accepted language classifies these surfaces as *stale*
   ("may be stale relative to revision 0.8 until refreshed"); the sanction
   makes the state permitted-pending-refresh, which properly informs
   `AuthorityNeeded=NO` / immateriality notes / RemainingWork, not the
   controlled disposition. (No pilot invoked `ACCEPTED_DIVERGENCE`; addendum
   11 aside, addendum 4 fixed "one controlled value" for this pattern.)
3. Package-internal consistency: DEL-00-04 and DEL-00-05 dispositioned
   declared-state prose overtaken by later accepted records (SCA-003
   resolutions; DEC-009 resolutions) as STALE. An authority pointer overtaken
   by the accepted SCA-005/0.8 amendment is the same species; encoding one
   STALE and the other ALIGNED — inside the very same ledger for DEL-00-04 —
   is not a coherent per-surface distinction.
4. DEL-00-06's counter-reading ("the declaration still describes the
   docs-only implemented slice") does not survive the Procedure line, which
   declares a fact about the current authority basis, not about the docs.

**Defect assignment (per the dispatch: defect on the wrong side).**
Wrong-side rows: DEL-00-04 DECL-001/DECL-004; DEL-00-06 DECL-001/DECL-002;
DEL-00-07 DECL-001/DECL-002/DECL-004. Additionally-defective discovery
misses (the drift is not even noted, so it is invisible to any later repair
queue **under either encoding ruling**): DEL-00-02 DECL-001/DECL-002/DECL-004;
DEL-00-05 DECL-001/DECL-004 (+ no rev note on DECL-002); DEL-00-06 DECL-004.

**Flip caveat for the orchestrator:** if the owner instead calibrates this
pattern to ALIGNED-with-note, the wrong side becomes DEL-00-01/03/08 (9 rows
to downgrade) and DEL-00-02/05 (and DEL-00-06 DECL-004) remain defective for
the missing notes. Either way, one re-encode pass over the named rows is
required before the PKG-00 package summary is computed; the fix is mechanical
in every case (all facts are already in the ledgers or in this report).

## Package finding 2: SourceReliability ladder inconsistency on ACC/EXC rows
## (QUALIFIED, not a per-ledger defect)

Addendum 6 pins DECL rows (`NOT_APPLICABLE` — all 48 DECL rows in the package
comply; verified mechanically). For ACCEPTANCE/EXCLUSION rows on these
documentation-only deliverables the eight ledgers used three encodings:
`NOT_APPLICABLE` (DEL-00-04), `UNVERIFIED` (DEL-00-01/03/06/07/08 mostly),
`REVIEWED` (DEL-00-02 EXC-001/EXC-002; DEL-00-05 ACC/EXC rows). Two cells are
at the edge of the addendum-6 bar: DEL-00-02 EXC-001 (`REVIEWED` with
`DecisionBasis=NONE_FOUND` and agent-inspection evidence only) and DEL-00-05's
`REVIEWED` cells resting on the PKG00 lock review that D-40/DEC-072
administratively reset — DEL-00-07 hit the same posture and chose `UNVERIFIED`
with an explicit self-flag (the better reading, in this verifier's judgment).
No disposition depends on these cells; recorded as a calibration item for the
package summary, not a defect.

## Package finding 3: ACCEPTANCE/EXCLUSION census grain varies across
## near-identical kits (QUALIFIED)

For eight kits with essentially the same six acceptance bullets, pilots
produced ACC/EXC splits of 4/2, 2/3, 4/4, 3/3, 3/2, 3/3, 3/2, 4/4. Every
ledger discloses its folding rationale (avoid double-counting), and addendum
12's grain rule genuinely under-specifies bulleted acceptance criteria
(DEL-00-07 friction note 1 says so explicitly). Not defects; but ClaimType
histogram comparability across PKG-00 deliverables is weaker than the
identical-kit structure would suggest. Candidate convention addendum for
later waves.

---

## Per-ledger check tables and verdicts

Legend: PASS = citation/claim verified against the frozen tree or run
indexes; QUALIFIED = verified with a disclosed, aggregation-safe caveat;
FAIL = material defect. "Self-flag" rows cover every item in each notes
file's self-flagged section.

### DEL-00-01 — Architecture decision record baseline — **SOUND**

| Row / item | Check | Result |
|---|---|---|
| REQ-001 (ALIGNED sample) | `docs/architecture/adr/index.md` status vocabulary lists proposed/accepted/superseded/rejected (lines 17–20) | PASS |
| REQ-002 (ALIGNED sample) | `template.md` carries Status/Human authority/Context/Decision/Alternatives/Consequences/Affected Packages/Reconsideration Triggers/Source References/Boundary Note | PASS |
| REQ-003 / self-flag | ADR-0001 records the one accepted choice citing D-13 (register, RULED) + DEC-020 (decomp §12) | PASS |
| DECL-001/002/004 (non-ALIGNED) | Rev-0.7 citations at Specification:32, Datasheet:32, Procedure:8; frozen decomp header `revision: 0.8` | PASS — STALE is the adjudicated correct side |
| DECL-006 self-flag (MEMORY census call) | MEMORY.md is a single dated 2026-06-04 entry; ALIGNED-historical treatment matches addendum 1 | PASS (judgment endorsed) |
| Self-flag REQ rows (REVIEW routing, UNVERIFIED) | No current human acceptance at frozen SHA (D-40 reset); addendum 6 honored | PASS |
| Histograms; addendum 1/2/6/12 sweeps; folder md-only (0 non-.md files); run record TASK_RUN_2026-06-11_1408 exists | Recount + mechanical sweep | PASS |

Tally: 9 PASS / 0 QUALIFIED / 0 FAIL. Fences held. **Verdict: SOUND.**
Note: DEL-00-01 routes `AuthorityNeeded=REVIEW` on all five REQ rows where
DEL-00-03/08 route `NO` for the same pending-review posture — dedupe at
aggregation (disclosed in its friction note 4).

### DEL-00-02 — Repository and module boundary architecture — **DEFECTIVE**

| Row / item | Check | Result |
|---|---|---|
| REQ-001 (ALIGNED sample) | AB-00-02 row at SOFTWARE_DECOMP.md:427 verbatim covers layer responsibilities / inward dependencies / adapter no-bypass; DEC-008 human approval | PASS |
| EXC-001 (dispatch item, self-flag) | Folder is 0 non-.md files; `WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_*.md` present under DEL-00-02 `_run_records/`; the wasm code landed at `apps/desktop/src/services/wasmEngine/*`, `core/model_operations/operation_applier` (outside the folder, separate Type 1 tranche) | PASS — ALIGNED-for-the-exclusion with REVIEW routing is the right encoding; hosting-location question properly surfaced, not dispositioned |
| REQ-005 (non-ALIGNED, self-flag) | REQ-02-05 ("review checks that detect package-local architecture drift", Specification:13); kit checks are kit-hygiene grained | PASS — PARTIALLY_IMPLEMENTED defensible, disclosed |
| REQ-001/002/003 by-reference ALIGNED (self-flag) | AB-00-02 keyed to DEL-00-02 and human-accepted | PASS (judgment endorsed) |
| `docs/architecture/module_boundaries.md` absent; SOW-057/DEC-012 deferral | Verified absent; permitted-deferral reading consistent | PASS |
| DECL-001/002/004 | Kit carries the identical rev-0.7 lines (Spec:32, DS:32, Proc:8); ledger neither dispositions **nor notes** the drift on any of the three rows | **FAIL** — discovery miss; wrong disposition under the adjudicated encoding (3 rows) |
| EXC-001 SourceReliability=REVIEWED | `DecisionBasis=NONE_FOUND`, agent-inspection evidence; addendum-6 REVIEWED bar not met | QUALIFIED (no disposition impact) |
| Histograms; sweeps | Recount + mechanical | PASS |

Tally: 7 PASS / 1 QUALIFIED / 1 FAIL (3 rows). **Verdict: DEFECTIVE.**
Defects: DECL-001, DECL-002, DECL-004 — rev-0.7 authority-pointer drift
uncaptured (should be `STALE_SETUP_SPECIFICATION` per the addendum-4
adjudication; at minimum a note under any ruling). Repair is mechanical.

### DEL-00-03 — Application service command-query-job model — **SOUND**

| Row / item | Check | Result |
|---|---|---|
| DECL-001/002/004 (non-ALIGNED) | Rev-0.7 cites verified at the exact cited lines (Spec:32, DS:33, Proc:8); decomp 0.8; sanction language at decomp ~655 quoted accurately | PASS — correct side |
| ACC-001 (ALIGNED sample) | Identity grep counts reproduce exactly (Spec 2, DS 3, Guidance 1, Procedure 1) | PASS |
| EXC-001 (ALIGNED sample) | 0 non-.md files in folder; DEL-00-03 absent from IMPLEMENTATION_SURFACES / VERIFICATION_INDEX / VALIDATION index (grep = 0) | PASS |
| Self-flags (REQ-003/004 ALIGNED-at-runway-grain; EXC-003 non-exhaustive scan) | Disclosed, convention-open judgments; TBD slots verified present | PASS (judgments endorsed) |
| DECL-005 RecordedRemaining | `_STATUS.md` bootstrap item reads "§§6–8"; CSV cell transliterates to "sections 6-8" — not strictly verbatim (addendum 2) | QUALIFIED (excluded item; zero analysis impact) |
| Histograms; sweeps | Recount + mechanical | PASS |

Tally: 6 PASS / 1 QUALIFIED / 0 FAIL. **Verdict: SOUND.**

### DEL-00-04 — Persistence and schema versioning architecture — **DEFECTIVE (narrow)**

| Row / item | Check | Result |
|---|---|---|
| REQ-001/002 (ALIGNED samples) | AB-00-04 at SOFTWARE_DECOMP.md:429 verbatim ("deterministic, versioned, unit-aware … JCS-compatible canonicalization … SQLite store"); `docs/architecture/persistence_contract.md` declares `schema_version`/`migration` (lines 21, 39, 42) | PASS |
| DECL-002/003 (non-ALIGNED) | Datasheet TBD slots (lines 39–43) still list physical format / hash algorithm / storage backend; Guidance queue (29–31) still lists format + canonicalization; both resolved by SCA-003/DEC-017/AB-00-04; `persistence_versioning.md` absent tree-wide | PASS — STALE calls verified, OWNER routing sound |
| Self-flag REQ REVIEWED provenance | SCA/AB rows live in human-accepted `current_basis` decomposition | PASS (judgment endorsed; addendum 6 satisfied via named accepted basis) |
| Self-flag REQ-004 diagnostics-enumeration gap → ALIGNED MEDIUM | AB-00-04/SOW-050 enumeration verified | PASS (disclosed) |
| DECL-001/DECL-004 | Same rev-0.7 drift, fully noted in-row, but dispositioned ALIGNED — wrong side under the addendum-4 adjudication; internally inconsistent with this ledger's own overtaken-TBD STALE calls | **FAIL** (2 rows) |
| Histograms; sweeps; addendum-6 DECL NOT_APPLICABLE incl. `_STATUS` (overrides pre-addenda exemplar — correctly) | Recount + mechanical | PASS |

Tally: 6 PASS / 0 QUALIFIED / 1 FAIL (2 rows). **Verdict: DEFECTIVE** —
DECL-001 and DECL-004 should carry `STALE_SETUP_SPECIFICATION` under the
adjudicated encoding. All facts already recorded in-row; flip is mechanical.
(If the owner calibrates ALIGNED-with-note instead, this ledger becomes the
package exemplar and is SOUND.)

### DEL-00-05 — GUI state and interaction architecture — **DEFECTIVE**

| Row / item | Check | Result |
|---|---|---|
| REQ-004 (dispatch item; non-ALIGNED, self-flag) | REQ-05-04 = "Define selection and property-inspector behavior without committing to a framework implementation" (Spec:12). Kit has a selection-model role; zero property-inspector architecture content; DEL-07-02 "Model tree and property inspector" exists in PKG-07; **no record delegates inspector architecture out of DEL-00-05** | PASS — PARTIALLY_IMPLEMENTED at MEDIUM with `AuthorityNeeded=OWNER` is the convention-correct call: without a named delegation record, ALIGNED-by-delegation would fail addendum 11's bar, and UNKNOWN is not warranted (the evidence is resolved — the content is absent; only the scope boundary needs the owner). Routing endorsed |
| DECL-002/003 (non-ALIGNED) | DEC-009 (decomp:587) adopts React/Three.js while "component/state libraries remain TBD"; `apps/desktop/package.json` has `react ^19.0.0`, `three ^0.181.0`; Datasheet TBD slots (40–44) and Guidance queue (29–30) still list framework/viewport open; `gui_state_model.md` absent | PASS — STALE calls verified; partial-staleness named per-item correctly |
| ACC-003 self-flag (no double-count) | Acceptance-at-production-grain reading; staleness dispositioned on surface rows | PASS (judgment endorsed) |
| REQ-003/REQ-005 self-flags (thin-but-present → ALIGNED MEDIUM) | Lock-review row RECONCILIATION-PKG00-05 verified (LOCK_AS_BASIS; "exact component/state-management library remains intentionally unresolved") | PASS (disclosed) |
| DECL-001/DECL-004 | Kit carries the identical rev-0.7 lines (Spec:32, DS:34, Proc:8); ledger neither dispositions **nor notes** the drift (DECL-001 says NONE_OBSERVED; DECL-004 notes only SEMANTIC_READY; DECL-002's STALE row omits the rev pointer from its repair list) | **FAIL** — discovery miss (2 rows wrong under the adjudicated encoding; note missing under any ruling) |
| SourceReliability=REVIEWED on ACC/EXC rows citing the D-40-reset lock review | DEL-00-07 encoded the same posture UNVERIFIED with a self-flag | QUALIFIED (ladder inconsistency; no disposition impact) |
| Histograms; sweeps | Recount + mechanical | PASS |

Tally: 6 PASS / 1 QUALIFIED / 1 FAIL (2 rows). **Verdict: DEFECTIVE.**
Defects: DECL-001, DECL-004 rev-0.7 drift uncaptured. REQ-05-04 is NOT a
defect — the encoding is endorsed as-is.

### DEL-00-06 — Diagnostics, warning, and result-envelope contract — **DEFECTIVE**

| Row / item | Check | Result |
|---|---|---|
| REQ-001 (ALIGNED sample) | AB-00-06 at SOFTWARE_DECOMP.md:431 verbatim (eight fields; six warning classes) | PASS |
| REQ-002 (ALIGNED sample, self-flag on REVIEWED) | `TASK_RUN_2026-05-17_TP-DIAG-019.md` exists; class-mapping rule ("map only the diagnostic class when the receiving schema has local class…") and No-Claim Closeout present | PASS — REVIEWED resting primarily on AB-00-06/SCA-001 endorsed; TP-DIAG-019 correctly weighted as corroborating |
| C18 residual homing (convention 3) | DEL-00-06 `## Remaining` carries only the bootstrap; envelope surfaces (SURF-101/114/121/187/202/226/228) all multi-attributed to downstream implementers | PASS — cross-reference resolution, no REMAINING_STATE_MISMATCH; well documented |
| EXC-001 (folder boundary) | 0 non-.md files; TP-DIAG-019 recorded no code added | PASS |
| DECL-002 self-flag (anticipated artifacts absent → ALIGNED MEDIUM) | `diagnostics_contract.md` absent; runway framing | PASS (disclosed judgment; consistent with DEL-00-07's identical call) |
| DECL-001/DECL-002 | Rev-0.7 drift noted + self-flagged but kept ALIGNED — wrong side under the addendum-4 adjudication; the ledger's own friction note concedes the pattern "has no clean home" | **FAIL** (2 rows) |
| DECL-004 | Procedure.md:8 carries the same "revision 0.7 is the current basis" line; row notes only the SEMANTIC_READY wording — rev drift uncaptured on this surface | **FAIL** (1 row) |
| Histograms (all-ALIGNED 17); sweeps | Recount + mechanical | PASS |

Tally: 6 PASS / 0 QUALIFIED / 2 FAIL (3 rows). **Verdict: DEFECTIVE.**
The "all-ALIGNED result" the notes defend is an artifact of the wrong-side
encoding: under the adjudicated rule this ledger has 3 STALE rows like its
DEL-00-01/03/08 siblings.

### DEL-00-07 — API boundary and adapter contract map — **DEFECTIVE**

| Row / item | Check | Result |
|---|---|---|
| REQ-005 (non-ALIGNED, self-flag) | REQ-07-05 (Spec:13) enumerates storage/reports/private-libraries/local-FEA/external-automation handoffs; kit covers private-library role + OPS-K-MECH-1 local-FEA path (Spec:27); AB-00-07 (decomp:432) does not restate handoff obligations | PASS — PARTIALLY_IMPLEMENTED verified and endorsed |
| REQ-001 (ALIGNED sample, self-flag on "define" grain) | AB-00-07 row verbatim; `api_boundary_map.md` absent tree-wide (find = 0); TBD posture accepted in AB-00-07 | PASS (disclosed judgment endorsed) |
| DECL-002 self-flag (Outputs Expected absent → ALIGNED) | Verified absent; Procedure completion condition does not require them | PASS (disclosed; consistent with DEL-00-06) |
| UNVERIFIED-after-reset SourceReliability (self-flag) | D-40/DEC-072 reset verified; encoding matches addendum 6's current-posture reading | PASS — this is the better side of the package's ladder split |
| DECL-001/DECL-002/DECL-004 | Rev-0.7 drift noted on all three rows but kept ALIGNED — wrong side under the addendum-4 adjudication | **FAIL** (3 rows) |
| Histograms; sweeps; DecisionBasis resolvability (SCA-001/AB-00-07/D-40/DEC-072 all resolve in-tree) | Recount + mechanical + spot resolution | PASS |

Tally: 5 PASS / 0 QUALIFIED / 1 FAIL (3 rows). **Verdict: DEFECTIVE** —
solely the rev-0.7 encoding side; everything else in this ledger checked
clean. Mechanical flip (facts already in the RemainingWork cells).

### DEL-00-08 — Layered software test and acceptance strategy — **SOUND**

| Row / item | Check | Result |
|---|---|---|
| DECL-001/002/004 (non-ALIGNED) | Rev-0.7 cites verified at the exact cited lines (Spec:32, DS:34, Proc:8) | PASS — correct side |
| ACC-001 (ALIGNED sample) | Identity grep counts reproduce exactly (2/3/1/1) | PASS |
| EXC-001 + material-surface analysis (self-flag) | All DEL-00-08-attributed surfaces in IMPLEMENTATION_SURFACES.csv are multi-attributed to owning deliverables (verified row-by-row; none NONE_FOUND); folder is 0 non-.md files; Datasheet Type `TEST_SUITE` (DS:9) with explicit no-implementation scope | PASS on the judgment (not-unmapped, not-owned; endorsed) |
| Surface count "14" | Actual attribution-column count is **17** (SURF-003/004/005/006/027/056/058/062/065/066/099/102/140/155/160/215/222 — the notes' own list has 17 IDs) | QUALIFIED — count error in EXC-001 evidence cell and notes prose; no disposition or histogram impact |
| REQ-001/002/004 self-flags (ALIGNED at runway grain; unbuilt `test_strategy.md`/gate matrix as forward artifacts) | Absence verified; DEL-00-03 peer precedent applied consistently | PASS (disclosed) |
| DECL-006 (MEMORY multi-entry) | Dated entries treated per addendum 1; test-harness work correctly attributed to owning deliverables | PASS |
| Histograms; sweeps; RemainingSource "sections 6-8" transliteration | Recount + mechanical | PASS / QUALIFIED (verbatim-copy nit, excluded item) |

Tally: 7 PASS / 2 QUALIFIED / 0 FAIL. **Verdict: SOUND.** Re-encode of the
"14" → "17" count is recommended whenever the ledger is next touched, but it
is not defect-grade.

---

## Convention-conformance sweeps (mechanical, all eight ledgers)

- **Addendum 1 census:** 6 DECLARED_STATE rows per ledger (4 kit surfaces +
  `_STATUS.md` + `MEMORY.md`); no deliverable-owned README exists in any of
  the eight folders (verified); MEMORY dated-entry drift handled as notes,
  never staleness dispositions — CONFORMS ×8.
- **Addendum 2 bootstrap:** the seeded `(gated: D-41)` item appears only in
  each `_STATUS.md` surface row's RecordedRemaining; zero REMAINING_WORK
  rows; excluded from residual/gate/selectability analysis — CONFORMS ×8.
  Verbatim nit: DEL-00-03/05/08 transliterate "§§6–8"→"sections 6-8" in one
  or both of RecordedRemaining/RemainingSource (QUALIFIED).
- **Addendum 6 ladder:** all 48 DECL rows `NOT_APPLICABLE` — CONFORMS ×8.
  ACC/EXC-row ladder inconsistent across ledgers (package finding 2).
- **Addendum 11:** no ACCEPTED_DIVERGENCE used anywhere in PKG-00 — no
  threshold question arises.
- **Addendum 12:** ClaimID form `DEL-00-0X-<TYPE>-NNN` conforms on all 137
  rows (regex-checked); `SelectableUnderCurrentLoop=NO` on all rows (no
  non-bootstrap residuals; matches R1 inventory); path aliases declared once
  per ledger where used.
- **Convention 8 / addendum 10 overtaken-evidence bar:** no ALIGNED row in
  the package rests on class-required validation/verification evidence (all
  documentation-class; ValidationEvidence NOT_APPLICABLE with in-cell
  reasons) — bar not engaged; recorded-pass citations carry the
  `not re-executed at frozen SHA 551f84ef6` marker where cited (DEL-00-06/07).
- **Histograms:** recounted from all eight CSVs — every disposition and
  ClaimType histogram in the eight notes files reproduces exactly.

## Fence compliance

All eight ledgers and notes: no lifecycle/DAG/scope mutation proposed as
operative (`LIFECYCLE_REASSESSMENT_REQUIRED` unused; nothing applied); no
F-PIP-1..5 claim language outside descriptive recording of the deliverables'
own exclusions (checked: EXC rows assert absence of certification/compliance
claims and make none); agent dispositions routed via `AuthorityNeeded`, never
phrased as rulings; DEL-01-01 is not in this package (no ISSUED handling
required). Each pilot's boundary statement claims read-only discovery with
writes confined to its two W1 output files; consistent with the artifacts
observed. Frozen tree porcelain: empty before and after this verification
pass; HEAD re-verified as `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## Package summary line

**PKG-00: 3 SOUND (DEL-00-01, DEL-00-03, DEL-00-08), 5 DEFECTIVE
(DEL-00-02, DEL-00-04, DEL-00-05, DEL-00-06, DEL-00-07) — every defect is
the single rev-0.7-vs-0.8 declared-state encoding divergence (wrong-side or
uncaptured), adjudicated to the STALE side under addendum 4; all repairs are
mechanical row-level re-encodes (13 rows total across the five ledgers), and
one owner/orchestrator calibration note (this pattern recurs corpus-wide on
every kit refreshed to 0.7) should be issued before W2 so later waves encode
it uniformly.**
