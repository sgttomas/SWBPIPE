# Discovery notes — DEL-06-02 Sandboxed unit-aware expression evaluator (PKG-06, W3)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ledger: `CLAIM_CONCORDANCE_DEL-06-02.csv` (20 claim rows, RFC-4180, CRLF).
Generator: `scratchpad/build_csv_DEL-06-02.py` (unique-named per W1 calibration item 7).

Run-level `NormativeSource` alias (addendum 12): `SOFTWARE_DECOMP §12` =
`execution/_Decomposition/SOFTWARE_DECOMP.md §12` (decision codifications, frozen tree).

## 1. Histograms (recomputed from the CSV)

**Disposition histogram (20 rows):**
- ALIGNED — 12
- PARTIALLY_IMPLEMENTED — 4
- STALE_SETUP_SPECIFICATION — 4

**ClaimType histogram (20 rows):**
- REQUIREMENT — 11
- DECLARED_STATE — 7
- EXCLUSION — 2

(ACCEPTANCE — 0, REMAINING_WORK — 0, IMPLEMENTED_UNMAPPED — 0; see §4 for why.)

## 2. Self-flagged rows

- **DEL-06-02-REQ-001** (SECURITY, sandbox/no-arbitrary-code): dispositioned
  ALIGNED (fan-in W3 harmonization). The sandbox is structural (typed
  AST, no parser, no host-access execution node) and unit-tested; sandbox
  sufficiency (validation of escape resistance) is owner-gated. Convention 6
  forbids a VERIFIED_NOT_VALIDATED downgrade on the owner-gated
  sufficiency-deferral ground, so the deferral is carried by the em-dash
  ValidationEvidence marker + AuthorityNeeded=OWNER + RemainingWork rather than
  by the disposition; the row is ALIGNED with the marker. (I originally
  dispositioned VERIFIED_NOT_VALIDATED and self-flagged the choice for fan-in
  harmonization; this is that correction, matching DEL-06-01 REQ-007 and the
  DEL-06-04 marker rows.)
- **DEL-06-02-REQ-010** (SECURITY, adapter/plugin no-bypass): dispositioned
  PARTIALLY_IMPLEMENTED (no plugin path exists in-crate; adapter/plugin
  integration and bypass tests are downstream) *and* carries the convention-6
  SECURITY marker + OWNER. Judgment call: the mechanism is genuinely partial
  (not merely unvalidated), so PARTIALLY_IMPLEMENTED over VERIFIED_NOT_VALIDATED.
- **DEL-06-02-REQ-002** (declarative-not-executable): classed SCHEMA rather than
  SECURITY, to avoid over-applying the SECURITY sufficiency marker to a
  structural declarative-grammar property that DEC-022 froze. Reviewer eyes on
  the class choice.
- **DEL-06-02-REQ-006** (grammar/parser/library "remain TBD, require future human
  decision"): dispositioned ALIGNED. The literal requirement wording says TBD,
  but the *required* future human architecture decision was made (DEC-022 froze
  the typed-AST grammar; DEC-037 decision-permits deferring a writable text
  parser). Under addendum 11 a recorded adoption yields ALIGNED, not
  ACCEPTED_DIVERGENCE. The residual kit "TBD" prose is ledgered as staleness on
  the Specification/Guidance surface rows, not here (requirement rows never take
  STALE_SETUP_SPECIFICATION, convention 1).
- **DEL-06-02-REQ-005 / REQ-009 / REQ-011**: PARTIALLY_IMPLEMENTED grain calls —
  the crate-level behavior exists and is verified, but the sealed binding
  contract (005), result-envelope integration + final diagnostic taxonomy (009),
  and the plugin/adapter + public-example test families (011) are recorded
  downstream deferrals. Reviewer confirmation of the "partial vs aligned" line.
- **DEL-06-02-DECL-005** (MEMORY surface): ALIGNED-with-note under W2 calibration
  item 9. The undated Open Items block still reads "Final expression
  grammar/library selection remains TBD," which is drift, but it is corrected in
  the same file by the dated 2026-06-11 "TP-C1-GRAMMAR-001: DEC-022 grammar
  freeze implemented" entry (named in-row). Judgment call on whether the in-file
  correction lifts the undated block to ALIGNED-with-note vs STALE.
- **DEL-06-02-DECL-007** (crate README as a DECLARED_STATE surface): judgment
  call under W2 calibration item 10. The crate README
  (`core/rules/expression_evaluator/README.md`) self-identifies as "the bounded
  implementation slice for DEL-06-02"; I treated it as a deliverable-owned
  in-tree README and censused it (ALIGNED — current, accurate, omits no
  implemented family). The crate is shared across DEL-06-01/06-02/07-03/11-01
  (SURF-117) but the README describes the DEL-06-02 evaluator specifically.
- **ACCEPTANCE = 0 decision**: the Specification "Verification" table is a
  per-requirement mirror (one approach per REQ-06-02-NNN), and the Procedure
  "Verification" checks are setup-gate process checks (four-doc kit exists,
  semantic PASS, dependency schema valid, status value valid) — not
  product-acceptance criteria at addendum-12 grain. Per the brief's rule
  ("verification tables that merely restate requirements do NOT get mirrored
  ACCEPTANCE rows") I produced no ACCEPTANCE rows. Reviewer confirmation wanted.

## 3. Evidence-execution log

**Re-executed (side-effect-free, addendum 9):**
- `cargo test --locked --offline` in `core/rules/expression_evaluator/`, run
  in-place at the frozen SHA with `CARGO_TARGET_DIR` and `CARGO_HOME` redirected
  to the external scratch dir (so no build/bytecode artifact lands in the frozen
  tree, not even the git-ignored `/target/`). Result: **PASS** — 31 in-crate
  unit tests + `tests/conformance_corpus.rs` (`conformance_corpus_pins_the_
  frozen_grammar`, 69 invented-value cases) + 0 doc-tests.
- `git -C FROZEN status --porcelain` was **empty before and after** the run, and
  again after writing both output files. HEAD unchanged at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
- Note on calibration item 12: the out-of-tree byte-identical-copy pattern was
  **not needed** — the addendum-9 external-`CARGO_TARGET_DIR` in-place run kept
  the frozen tree clean (verified), so the copy pattern's four conditions did
  not have to be invoked.

**Cited as recorded (not re-executed at frozen SHA 551f84ef6):**
- MEMORY 2026-06-11 (TP-C1-GRAMMAR-001): evaluator 31 unit tests + 69-case
  corpus, lifecycle 12, `python3 -m pytest` 353, `cargo fmt --check` clean.
- MEMORY 2026-06-05 (PKG06-02-PKG02-001 verification): `cargo test` 17 unit
  tests, `cargo fmt --check` pass.
- R1 `VERIFICATION_INDEX.csv` RUST-18: `cargo_crate_sweep=pass` at ancestor
  commit `e648462f1` (sweep `SWEEP_20260711T040758Z_e648462f1d05.json`). I did
  **not** re-run the sweep's own `diff` myself, so I did **not** transcribe the
  addendum-10 `content-identical … (diff empty over <paths>)` qualifier onto any
  row (W1 calibration item 4); my direct frozen-SHA re-execution is the primary
  verification leg instead.

**Absence checks (calibration item 15 discipline):** full-tree `find` at the
frozen SHA confirmed `tools/validation/check_four_documents.sh` and
`tools/validation/validate_enum.py` (both named in Procedure.md) **do not exist**
anywhere in the tree; only `tools/validation/validate_dependencies_schema.py`
exists. Recorded in-row on DEL-06-02-DECL-004 (no convention-7 ATTESTED marker
used: the tools are named as procedure commands, not as DecisionBasis for a
claim).

## 4. Row census rationale

- **11 REQUIREMENT rows** — one per current requirement ID REQ-06-02-001..011
  (re-verified against Specification.md at the frozen SHA and against
  DELIVERABLE_INVENTORY.csv row DEL-06-02). ClaimIDs use the addendum-12 fixed
  form `DEL-06-02-REQ-NNN`; the native IDs are recorded in NormativeSource.
- **0 ACCEPTANCE rows** — see §2 (per-requirement mirror + setup-gate checks, no
  addendum-12-grain acceptance surface).
- **2 EXCLUSION rows** — EXC-001 durable IP/protected-content/no-compliance
  product boundary; EXC-002 sibling-scope non-absorption (schema/completeness/
  lifecycle/GUI/reports/unit-conversion belong to sibling DELs). The Scope
  sentence's setup-era portion ("does not implement an evaluator module / create
  tests / choose grammar as final") is overtaken and ledgered as staleness on
  the Specification surface row (DECL-001), not as an exclusion.
- **7 DECLARED_STATE rows** — Specification, Datasheet, Guidance, Procedure,
  MEMORY, _STATUS (addendum 1 census), plus the crate README (W2 calibration
  item 10; see §2). No separate DECL rows for `_CONTEXT.md` / `_REFERENCES.md` /
  `_REVIEW.md` (not in the addendum-1 census; the exemplar treats them the same
  way). The rev-0.7→0.8 authority-pointer drift carried by `_CONTEXT.md` /
  `_REFERENCES.md` is recorded in-row on the Datasheet surface (its Identification
  sources from `_CONTEXT.md`); the owner-calibration caveat is stated once in §6.
- **0 REMAINING_WORK rows** — the only `## Remaining` item is the seeded
  `(gated: D-41)` bootstrap, which goes verbatim into the `_STATUS.md` surface
  row's `RecordedRemaining` and is excluded from all residual/gate/selectability
  analysis (addendum 2 / calibration item 11). The MEMORY "Open Items" TBDs
  (tolerances, diagnostic taxonomy, result-envelope integration, threat-model
  depth, binding contract) are declared downstream/future-implementation
  deferrals recorded on the relevant requirement rows' RemainingWork — none is an
  omitted `## Remaining` residual needing its own row or an UNKNOWN home.
- **0 IMPLEMENTED_UNMAPPED rows** — the primary material surface
  `core/rules/expression_evaluator` (SURF-117) IS mapped to DEL-06-02; sibling
  surfaces (rule_check_runner, rule_pack_lifecycle, rule_pack_document,
  completeness_checker) map to sibling DELs and are excluded scope, not unmapped.

## 5. Bootstrap `_STATUS` surface handling (calibration items 5 & 11)

The `(gated: D-41)` item is transcribed byte-exact into DEL-06-02-DECL-006
`RecordedRemaining` (en-dash `§§6–8` preserved; no transliteration). Per
calibration item 11 the exclusion variant is used: `RemainingSource` and
`GateOrStageConstraint` stay `NONE_RECORDED` scoped to non-bootstrap content
(not annotated with the bootstrap item). `SelectableUnderCurrentLoop=NO` on all
rows (no non-bootstrap residual; the bootstrap is excluded from selectability).

## 6. Owner-calibration caveat (W1 calibration item 1 — stated once)

`_CONTEXT.md` (Decomposition Reference / Architecture Basis Injection) and
`_REFERENCES.md` cite `execution/_Decomposition/SOFTWARE_DECOMP.md`
**revision 0.7**, `status: current_basis`; the frozen decomposition header is
**revision 0.8**, `status: current_basis`. This is ruling-after-freeze /
refresh-lag pointer drift, not a live authority conflict. It is recorded as a
drift fact in-row on DEL-06-02-DECL-002 (the Datasheet surface, whose
Identification sources from `_CONTEXT.md`). That surface is STALE_SETUP_
SPECIFICATION and OWNER-routed on independent grounds (evaluator implemented
while the Datasheet declares it "not produced"), consistent with calibration
item 1's "OWNER where the kit also carries overtaken TBD registers." The MEMORY
2026-06-04 dated entry also cites revision 0.7 + DAG-006; that is a dated
historical log entry (addendum 1) and is noted here rather than dispositioned.

## 7. Convention-friction notes

- **SECURITY disposition vs the marker.** Convention-6 / W1 calibration item 2
  fixes the ValidationEvidence marker and OWNER routing for owner-gated SECURITY
  sufficiency. Fan-in resolved the disposition question: convention 6 forbids a
  VERIFIED_NOT_VALIDATED downgrade on the owner-gated sufficiency-deferral
  ground, so a mechanism-complete row whose only open item is that deferral is
  ALIGNED with the marker (REQ-001, corrected in W3). PARTIALLY_IMPLEMENTED still
  applies where the mechanism itself is downstream (REQ-010). Harmonized with
  DEL-06-01 REQ-007 and the DEL-06-04 marker rows.
- **Requirement-that-declares-a-TBD resolved by a later ruling (REQ-006).** The
  binding set (addenda 5/11) covers "TBD accepted by a ruling that *permits* the
  deferral" (→ ACCEPTED_DIVERGENCE) and "recorded adoption" (→ ALIGNED), but the
  wording is thin on a requirement whose literal text *asserts* a TBD that a
  later ruling then *resolves*. I read it as ALIGNED (the required decision
  occurred). Flagged.
- **DECLARED_STATE SourceReliability.** Per addendum 6 all seven DECL prose rows
  use `SourceReliability=NOT_APPLICABLE` (the exemplar, which predates the
  addenda, is inconsistent on this — I followed the adopted addendum, not the
  exemplar).
- **SourceReliability = UNVERIFIED on all substantive rows** (calibration item
  13, weakest load-bearing leg): the implementation + verification legs are
  project-original agent-authored code and tests (deliverable IN_PROGRESS); the
  single human disposition on record (PKG06-02-PKG02-001 ACCEPT_AS_IS) covers one
  unit-metadata finding, not the whole surface, so no row reaches REVIEWED.
- **AuthorityNeeded routing (calibration item 14).** OWNER only on the two
  SECURITY sufficiency rows (001, 010) and the four STALE setup-surface rows
  (001–004 DECL); all recorded downstream TBDs route NO (recorded deferral with
  no numeric/authority claim being promoted). No gate-named token
  (e.g. `D-38`-style) governs any DEL-06-02 claim.

## 8. Boundary-compliance statement

All fences held. Discovery was read-only outside the two W3 output files
(`CLAIM_CONCORDANCE_DEL-06-02.csv`, `NOTES_DEL-06-02.md`). No lifecycle
transition, DAG mutation, cross-project edit, or edit to any `_STATUS.md`,
register, or product file was made (`LIFECYCLE_REASSESSMENT_REQUIRED` was not
used; STALE surfaces route OWNER as R5 repair candidates, never applied). No
release-readiness, issuance, certification, sealing, professional-approval, or
code-compliance claim appears anywhere in the outputs (F-PIP-1..5); the
requirement rows record that the *software* avoids such claims, which is not
itself such a claim. No sandbox-sufficiency assertion is made by the pilot. All
dispositions are agent judgments routed via `AuthorityNeeded`, never phrased as
owner or engineering rulings. The frozen evidence worktree
(`551f84ef6be656f1603ce0acfa5e3935aa9683c7`) porcelain was empty before and
after every read, the test re-execution, and both file writes; no writes (not
even git-ignored build artifacts) landed under the frozen tree.
