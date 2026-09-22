VERDICT: 3 LEGITIMATE, 4 CORRECT, 0 UNDECIDED

# W3 cross-package consistency verification

Run HELP-HUMAN-PIPING-20260921-RECONCILIATION. Fresh TASK (Type 2) verifier,
evidence-only, independent of every W3 manager, worker and package verifier.
Parent: HELP_HUMAN Agent 0.

- Brief: `R2-VERIFIER_brief.md`, SHA-256 `47fb3c52…5b00` (matches the launch
  message).
- Findings input: `W3_BATCH_FINDINGS.md`, SHA-256 `9c80e97f…60f7` (matches).
- Evidence: the freeze checkout of `00115c71931bcae79909602d653740d3bb72dfa1`.
  Paths below are relative to `projects/chirality-piping/` in the freeze.
- Rules applied: `CANONICAL_SITUATIONS.md` CP-11; `CONVENTIONS.md` C1
  (canonical inheritance), C3, C5, C6(a)/(c), C7, F1, F3, F8.
- Scope: the 7 flagged rows only. No ledger or resolutions file was edited.
  No git, builds, tests or network were used.

## How CP-11 constrains the fields

CP-11 fixes two things only: the row is never `ALIGNED`, and the disposition
is `PARTIALLY_IMPLEMENTED` or `DOCUMENTED_UNIMPLEMENTED` "with the gap's cause
(`DEFERRED_BY_RULING`, `NOT_STARTED` or `PARTIAL_SLICE`)"
(`CANONICAL_SITUATIONS.md:84`). It prescribes no tier, baseline class or
layer. Those follow C3, C4 and C5 for each row, and F8 says the tier follows
the row's own gap. The batch compares all five fields against a majority that
is 10 of 19 DEL-17-05 rows (`DEFERRED_BY_RULING · LOCAL_DESIGN ·
RULED_CRITERION · RECORD`, backed by DEC-080). So a row that differs from that
majority is not wrong for that reason alone. Each row is judged on its own gap
below.

**Does the PKG-17 verifier's reasoning apply to item 1?** In part. Its
reasoning ("CP-11 requires the cause of each row's own gap",
`PKG-17_VERIFICATION.md:146`) covered four rows whose only difference from the
majority was the cause. It settles the cause question here too: none of the
item-1 gaps is deferred by a ruling. It does not reach tier or layer, and all
five item-1 rows also differ on those fields. I judged those fields separately
under C3, C5 and F8. For the three DEL-06-02 rows, the tier is wrong
(see below).

---

## 1a. DEL-06-02:SOW#CLM-006.r05 — CORRECT (field; not a false alignment)

Sealed: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · NONE ·
SECURITY`.

Right values: **Disposition** `PARTIALLY_IMPLEMENTED` · **CauseTag**
`PARTIAL_SLICE` · **AuthorityTier** `INVARIANT` · **BaselineClass** `NONE` ·
**DivergenceLayers** `SECURITY`. Not a firm false alignment; the disposition
stands. This is a field error: the tier changes.

Evidence:
- The claim: "Adapters and plugins must not bypass sandboxing, unit
  validation, provenance checks, diagnostics, or public/private data
  controls" (`execution/PKG-06_…/DEL-06-02_…/ScopeOfWork.md:88`).
- The same rule is stated as a specification invariant at
  `docs/SPEC.md:37` and `docs/SPEC.md:353`, and in `docs/TYPES.md:216`. It is
  also accepted architecture basis AB-00-07
  (`execution/_Decomposition/SOFTWARE_DECOMP.md:444`).
- `core/adapters/framework/adapter_framework.py:505-534`: the
  `gate_adapter_runtime_dispatch` gate is deny-only and returns
  `BLOCKED_RUNTIME_NOT_SELECTED` with `runtime_dispatched=False`. Its
  docstring says the adapter execution model and the plugin runtime "remain
  owner-held TBDs".
- `DEL-06-02/_STATUS.md:9`: "Bind any future adapter/plugin invocation to a
  governed runtime dispatch path; the DEC-074 O7/E5 declaration gate is
  deny-only". The same point is at `DEL-10-02/_STATUS.md:8` and
  `DEL-10-02/ScopeOfWork.md:352-356`.

Reason:
- **CP-11 applies.** The no-bypass property holds only because nothing
  dispatches to the evaluator.
- **Cause.** `PARTIAL_SLICE` is right: the declaration-admission slice
  landed under DEC-074 O7, and the rest has not.
  - DEC-074 selected the narrow seam. It does not defer the runtime; the
    ruling text is at
    `execution/_Coordination/_DECISIONS/D-41_R4_RULING_2026-07-12.md:45-78`.
  - The runtime is a specification TBD (`docs/TYPES.md:216`), not a ruled
    deferral.
  - So the majority's `DEFERRED_BY_RULING` does not fit this row.
- **Layer.** `SECURITY` is right. The unmet element is the no-bypass control
  at the evaluator seam, which is a security subject (C5).
- **Tier.** `PROJECT_BASELINE` is wrong.
  - C3 puts claims that restate a specification or boundary invariant on
    security and privacy at `INVARIANT`, with `DivergenceLayers` carrying the
    subject.
  - F8 keeps `INVARIANT` when "the gap touches the boundary's subject". Here
    the remaining gap (a governed dispatch path with no-bypass tests) is the
    no-bypass boundary itself. It is not a stale record clause.
  - The sealed pair `PROJECT_BASELINE · SECURITY` is internally
    inconsistent. The same ledger records its security gap on OUT-001 as
    `INVARIANT · SECURITY` (`DEL-06-02_forward.csv` line 6), and the PKG-06
    verifier confirmed that treatment (`PKG-06_VERIFICATION.md:201`).
- **Contrast with item 1d.** In DEL-14-02 r09, F8 rightly moves the tier off
  the IP boundary, because that gap is a reference-binding scope item and not
  the IP boundary.
- **Disclosure.** The PKG-06 verifier's non-aligned sample included this row
  (`PKG-06_VERIFICATION.md:139`), and it did not contest it.
- **Recording.** After correction, the row still differs from the CP-11
  majority on cause, tier and layer. That difference is legitimate. Record the
  pair as resolved at the corrected values.

## 1b. DEL-06-02:SOW#CLM-013/REQ-06-02-010 — CORRECT (field; not a false alignment)

Right values: `PARTIALLY_IMPLEMENTED` · `PARTIAL_SLICE` · `INVARIANT` ·
`NONE` · `SECURITY`.

Evidence:
- REQ-06-02-010, "Adapters and plugins shall not bypass sandboxing, unit
  checks, provenance checks, diagnostics, or public/private data boundaries"
  (`DEL-06-02/ScopeOfWork.md:176`, basis AB-00-07).
- Otherwise the same evidence as 1a: `adapter_framework.py:505-534`,
  `DEL-06-02/_STATUS.md:9`, `docs/SPEC.md:37` and `:353`.

Reason: the same claim substance as 1a (Notes: "CP-11, as CLM-006.r05"). It
takes the same correction. This is a duplicate within the deliverable, which C1
requires to take the same disposition.

## 1c. DEL-06-02:SOW#CLM-016/REQ-06-02-010 — CORRECT (field; not a false alignment)

Right values: `PARTIALLY_IMPLEMENTED` · `PARTIAL_SLICE` · `INVARIANT` ·
`NONE` · `SECURITY`.

Evidence:
- The acceptance criterion: "Adapter/plugin boundary tests proving no bypass
  path reaches the evaluator without validation"
  (`DEL-06-02/ScopeOfWork.md:215`).
- The declaration-admission checks exist at `adapter_framework.py:897` and
  `:1125-1145`. There is no evaluator dispatch path that a bypass test could
  exercise (`adapter_framework.py:505-534`).
- `DEL-06-02/_STATUS.md:7` keeps the "plugin/adapter bypass-attempt" test
  family open.

Reason:
- The missing element is the security boundary test itself, so F8 puts the
  tier at `INVARIANT`.
- The cause is the same as 1a: a slice landed, and no ruling defers the rest.

## 1d. DEL-14-02:SOW#CLM-011.r09 — LEGITIMATE

Sealed values stand: `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE
· NONE · BASELINE`.

Evidence:
- R009: rule-pack and library references "shall expose
  identity/provenance/checksum-style metadata where source-supported, without
  embedding protected … content", with basis SOW-072 and IP_AND_DATA_BOUNDARY
  (`execution/PKG-14_…/DEL-14-02_…/ScopeOfWork.md:177`).
- SOW-072 is in accepted scope: analysis runs bound to "rule-pack references,
  library references", under DEC-014 (`docs/_Registers/ScopeLedger.csv:73`).
- Both record builders hard-code the lists as empty:
  - `core/analysis_runs/records.py:132-133` (`"rule_pack_refs": []`,
    `"library_refs": []`);
  - `apps/desktop/src/services/analysisRunCompatibility.ts:67`
    (`rule_pack_refs: [], library_refs: []`).
- The schema models these fields (`schemas/analysis_run.v0.2.schema.json:311-312`,
  `:357`, `:363`).
- `DEL-14-02/_STATUS.md:6-8` records no ruling that defers them.

Reason:
- **CP-11 applies.** The no-embedding half holds only because no reference is
  ever recorded.
- **Cause.** No ruling defers the population, and the schema and record slice
  landed. So the cause is `PARTIAL_SLICE`, not the majority's
  `DEFERRED_BY_RULING`.
- **Tier and layer.** F8 is correctly applied: the gap is the unbound
  SOW-072 accepted-scope category, not the IP boundary. That gives
  `PROJECT_BASELINE` (C3: the claim conflicts with accepted scope) and
  `BASELINE` (C5: an accepted baseline). Neither `LOCAL_DESIGN · RECORD` nor
  `INVARIANT · IP_DATA` fits.
- **Consistency.** This matches the sibling PROJECT_BASELINE rows in the same
  finding group, which the PKG-14 verifier confirmed
  (`PKG-14_VERIFICATION.md:209-216`).
- **Recording.** Record the pair as resolved at the sealed values.

## 1e. DEL-15-04:SOW#CLM-012.r03 — LEGITIMATE

Sealed values stand: `PARTIALLY_IMPLEMENTED · DEFERRED_BY_RULING · INVARIANT ·
NONE · CLAIMS`.

Evidence:
- The acceptance criterion: "software metadata cannot create professional
  acceptance and cannot preserve acceptance across bound-hash changes"
  (`execution/PKG-15_…/DEL-15-04_…/ScopeOfWork.md:165`).
- **Half 1 holds.** `core/handoff/external_prover/metadata.py:14`, `:59` and
  `:274-278` reject proposed authority claims as `rejected_boundary_claim`.
  The tests are `tests/test_external_prover_boundary_metadata.py:200`, `:234`
  and `:245`.
- **Half 2 has no subject.** No acceptance-reference field exists in
  `metadata.py` or in `schemas/external_prover_metadata.schema.json`; a search
  for `acceptance_ref` or `human_acceptance` finds nothing.
- DEC-081 reserves `ENGINEER_ACCEPTED` and states "no acceptance workflow
  exists in MVP" (`docs/claims_registry.md:157-158`;
  `execution/_Decomposition/SOFTWARE_DECOMP.md:672`).

Reason:
- **Cause.** Here the gap really is deferred by a governing ruling (DEC-081),
  so `DEFERRED_BY_RULING` matches the row's own cause. On cause, the row
  agrees with the majority.
- **Tier and layer.** They differ from the majority because the subject
  differs.
  - The gap is the acceptance/authority boundary: `docs/TYPES.md` §4 (line 52)
    and BS-ACCEPT in the DEC-081 claims taxonomy.
  - That is a professional-boundary invariant, so the tier is `INVARIANT`
    (C3), and the gap touches it (F8).
  - The layer is `CLAIMS`, the claims-language boundary (C5).
- The DEL-17-05 majority's gap is a ruled validation criterion recorded as
  local record drift. It is not the same subject.
- **Disposition.** `PARTIALLY_IMPLEMENTED` fits: one half is met and tested.
- **Recording.** Record the pair as resolved at the sealed values.

## 2. DEL-15-04:SOW#CLM-010.r07 — LEGITIMATE

Sealed values stand: `DOCUMENTED_UNIMPLEMENTED · DEFERRED_BY_RULING ·
INVARIANT · NONE · CLAIMS`.

Evidence:
- R7: "Any external human acceptance reference, if later represented, shall
  be external, human-actor-owned, and bound to reviewed payload hashes";
  verification "Tests require external/hash-bound representation and prevent
  content-change survival without re-review" (`DEL-15-04/ScopeOfWork.md:138`).
- There is no acceptance-reference representation or test
  (`metadata.py`; `schemas/external_prover_metadata.schema.json`;
  `tests/test_external_prover_boundary_metadata.py` tests only rejection, at
  `:200`, `:234` and `:245`).
- DEC-081 reserves `ENGINEER_ACCEPTED` with no workflow
  (`docs/claims_registry.md:157-158`; `SOFTWARE_DECOMP.md:672`).

Reason:
- **Majority.** The majority for this pattern (`NOT_STARTED · LOCAL_DESIGN ·
  RECORD`) comes from DEL-11-03, DEL-17-04 and DEL-09-01 rows, where nothing
  implements the claim and no ruling defers it.
- **Cause.** For R7, a governing ruling (DEC-081) withholds the acceptance
  workflow. C7 therefore requires `DEFERRED_BY_RULING`; `NOT_STARTED` is
  defined as "no ruling defers it".
- **Tier and layer.** They follow the acceptance-boundary subject, as in 1e.
- **Disposition.** `DOCUMENTED_UNIMPLEMENTED` is right: no element of R7's
  governed behaviour exists. 1e differs because half of its claim is met.
- **Recording.** Record the pair as resolved at the sealed values.

## 3. DEL-11-01:SOW#CLM-028 vs DEL-09-04:SOW#CLM-028 — CORRECT (flagged row wrong; majority partner right)

Flagged row, sealed: `STALE_SETUP_SPECIFICATION · RECORD_DRIFT · LOCAL_DESIGN
· NONE · RECORD`.

Right values for **DEL-11-01:SOW#CLM-028**: **Disposition** `ALIGNED` ·
**CauseTag** (empty) · **AuthorityTier** (empty) · **BaselineClass** (empty) ·
**DivergenceLayers** `NONE`.
- `RemainingWork` should be `NONE`.
- This is not a false alignment. It is the reverse: a firm false
  non-alignment that changes the disposition.
- **DEL-09-04:SOW#CLM-028** (`ALIGNED · NONE`) is right and stands.

Evidence:
- The two bodies are byte-identical. Both read "No source conflict identified
  in setup pass", with a Human ruling cell of TBD
  (`execution/PKG-11_…/DEL-11-01_…/ScopeOfWork.md:379-385`;
  `execution/PKG-09_…/DEL-09-04_…/ScopeOfWork.md:373-379`).
- The DEL-11-01 setup pass was 2026-04-30 (`DEL-11-01/_STATUS.md:13-15`). The
  guide work began 2026-05-09 (`:16-17`).
- The later conflict is between the SOW's "read-only for this deliverable"
  treatment (`DEL-11-01/ScopeOfWork.md:209`) and the guide edits. The owner
  has already ruled on it: "apply a guide-only exception to the SOW's
  otherwise read-only treatment"
  (`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18/instances/WI-PKG11-DEL1101-EXECUTION/OWNER_ADOPTION.md:20`,
  `:48`).
- The DEL-11-01 ledger itself records that divergence as
  `ACCEPTED_DIVERGENCE · SCOPE_GREW_BY_DIRECTION` on CLM-010.s02, CLM-014.s02,
  CLM-017 and CLM-018.r05 (FG-DEL-11-01-01).

Reason:
- **The claim is true as written.** The text is explicitly scoped to the
  setup pass, and the row's own Notes concede "Accurate for the setup pass".
- **No pending conflict is omitted.** The only later conflict was settled by
  a hash-bound owner exception, and it is carried as an accepted divergence
  on the governing rows. The table therefore records no unmet element of its
  own claim (F1 is not engaged). The C6(c) stale class needs text that has
  been overtaken, and this text has not.
- **Corpus precedent.** Across W1–W3, identical setup-pass "no conflict"
  tables are `ALIGNED` even where a later conflict arose and was ruled. For
  example, DEL-10-04:SOW#CLM-032 ("A later attribution conflict … was ruled
  by DEC-076; it does not falsify this setup-pass statement"), DEL-01-04
  CLM-028, DEL-04-04 CLM-026 and DEL-07-03 CLM-039.
- **Distinguishable cases.** The stale treatments elsewhere apply to
  different bodies. DEL-11-02 CLM-034 commits to recording conflicts found
  during drafting. DEL-03-08 CLM-015 and CLM-030 list conflicts that are now
  settled. DEL-16-03 CLM-028 contains text that was overtaken.
- **Consistency.** The same body in two deliverables must take one reading
  (C1). The partner's reading is the correct one.

---

## Summary table

| # | Key | Decision | Right values (Disp · Cause · Tier · Baseline · Layers) | False alignment |
|---|---|---|---|---|
| 1a | DEL-06-02:SOW#CLM-006.r05 | CORRECT (field: tier) | PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE · SECURITY | no |
| 1b | DEL-06-02:SOW#CLM-013/REQ-06-02-010 | CORRECT (field: tier) | PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE · SECURITY | no |
| 1c | DEL-06-02:SOW#CLM-016/REQ-06-02-010 | CORRECT (field: tier) | PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE · SECURITY | no |
| 1d | DEL-14-02:SOW#CLM-011.r09 | LEGITIMATE | sealed values | no |
| 1e | DEL-15-04:SOW#CLM-012.r03 | LEGITIMATE | sealed values | no |
| 2 | DEL-15-04:SOW#CLM-010.r07 | LEGITIMATE | sealed values | no |
| 3 | DEL-11-01:SOW#CLM-028 | CORRECT (firm, false non-alignment) | ALIGNED · (empty) · (empty) · (empty) · NONE; partner DEL-09-04 correct | no |

The owner should know:
- The three DEL-06-02 corrections move a security no-bypass claim from
  `PROJECT_BASELINE` to `INVARIANT`. That changes the tier the row routes
  under, although the disposition stays.
- The PKG-06 verifier sampled CLM-006.r05 and did not contest it. This
  verifier disagrees, for the C3 and F8 reasons given in 1a.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
