# SCA-007 Amendment Preview

**Authority:** D-47 → O-A (owner ruling 2026-07-16, packet §8), codified as `DEC-080`
(`execution/_Decomposition/SOFTWARE_DECOMP.md` §12).
**Acceptance basis:** amendment actions pre-accepted by the owner ("Pre-accept; run
end-to-end", D-47 packet §8 item 1); owner review occurs at the final PR.
**Drafted:** 2026-07-16 by a bounded Agent-2 drafter. This file previews exact
replacement texts; the parent applies them. Every "Current text" block is verbatim
from the live tree so the applying agent can string-match it. Paths are relative to
`projects/chirality-piping/`.

No stage advancement, prover activation, reproduction acceptance, lifecycle change,
release-readiness, professional-approval, certification, sealing, authentication, or
code-compliance claim is created by any action below.

---

## Action SCA-007-A001 — DEL-09-04 reproduction residual becomes actor-neutral and un-parked

**(a) Target:** `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md`

**(b) Current text:**

```
- Demonstrate external reproduction on a clean environment via the E1 runner (stage-gated: R5 exit evidence) (source: PRD plan §3 Phase E exit evidence / Receipt 1 parked items)
```

**(c) Replacement text:**

```
- Demonstrate clean-checkout reproduction of the validation examples via the documented E1 runner procedure, recording environment, tool versions, commands, exit codes, and output hashes into an immutable evidence bundle under `validation/evidence/reproduction/<run-id>/` — actor-neutral (maintainer- or agent-executable) ordinary loop work; R6-exit evidence under the amended PRD §24 (source: DEC-080 / D-47 packet §5 row 1)
```

**(d) Rationale:** DEC-080 replaces the v0.1 "external engineers" reproduction criterion with the actor-neutral clean-checkout criterion (amended PRD v0.3 §24 R6) and rules the evidence-bundle home; the stage-gate parking is removed so the item is selectable as ordinary loop work (D-47 packet §5 row 1).

---

## Action SCA-007-A002 — DEL-09-04 E2 residual: storage policy resolved by ruling

**(a) Target:** `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md`

**(b) Current text (fragment at the end of the first Remaining item):**

```
MAINTAINER_REVIEWED case-page promotion, GUI-workflow validation evidence, and evidence-bundle storage policy open (source: TP-E2-VALMANUAL-001 residuals / Receipt 10)
```

**(c) Replacement text:**

```
MAINTAINER_REVIEWED case-page promotion and GUI-workflow validation evidence open; evidence-bundle storage policy resolved by DEC-080 — ruled home `validation/evidence/reproduction/<run-id>/` (source: TP-E2-VALMANUAL-001 residuals / Receipt 10)
```

**(d) Rationale:** DEC-080 names the evidence-bundle home (D-47 packet §5 row 2), closing the storage-policy residual while keeping the other listed E2 residuals open.

---

## Action SCA-007-A003 — DEL-09-04 history line and Last Updated bump

**(a) Target:** `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md`

**(b) Current text (two independent blocks):**

Block 1:

```
**Last Updated:** 2026-07-12
```

Block 2:

```
## History
- 2026-07-12 - D-41 R5 T7 PDU-055 refreshed 3 cited declaration claims to current T1-T6 authority/evidence while preserving genuine residuals; cited-claim and Remaining backcheck closed the D-41 bootstrap.
```

**(c) Replacement text:**

Block 1:

```
**Last Updated:** 2026-07-16
```

Block 2:

```
## History
- 2026-07-16 - DEC-080/SCA-007 (D-47 O-A) propagation: the external-reproduction Remaining item re-expressed as the actor-neutral clean-checkout criterion under amended PRD §24 R6 and un-parked (stage-gate suffix removed); the E2 evidence-bundle storage-policy residual marked resolved to the ruled home `validation/evidence/reproduction/<run-id>/`. No lifecycle change.
- 2026-07-12 - D-41 R5 T7 PDU-055 refreshed 3 cited declaration claims to current T1-T6 authority/evidence while preserving genuine residuals; cited-claim and Remaining backcheck closed the D-41 bootstrap.
```

**(d) Rationale:** D-47 packet §6 requires deliverable-local edits to land with `_STATUS.md` history lines citing the DEC-080 row.

---

## Action SCA-007-A004 — DEL-09-04 ScopeOfWork.md: verify-no-change

**(a) Target:** `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/ScopeOfWork.md`

**(b)/(c) No edit.** Checked terms "22.6", "external reproduction", "external engineers", "external reviewers", and "reproduce" do not appear anywhere in the file (385 lines). The only related stems are method-level "reproducibility" mentions (lines 86, 98, 322, 337, 363) and "external engineering code" (line 175), none of which cite the amended criterion.

**(d) Rationale:** D-47 packet §5 row 3 anticipated possible criterion citations; the impact assessment finds none, so this records NO_CHANGE_VERIFIED under DEC-080.

---

## Action SCA-007-A005 — DEL-10-04 §22.6 token re-key on the D-06b signing item

**(a) Target:** `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_STATUS.md`

**(b) Current text:**

```
- Sign/notarize release artifacts or record the explicit PRD §22.6 deviation (gated: D-06b) (source: PRD plan §3 E5 row / register row D-06b / DEC-057)
```

**(c) Replacement text:**

```
- Sign/notarize release artifacts or record the explicit deviation from the historical v0.1 PRD §22.6 read through the D-21 Annex A crosswalk (forward home: the DEC-056 R6-entry release-machinery residuals) (gated: D-06b) (source: PRD plan §3 E5 row / register row D-06b / DEC-057)
```

**(d) Rationale:** DEC-080 re-keys the stale v0.1 "PRD §22.6" token to its historical reading through the D-21 Annex A crosswalk with the DEC-056 R6-entry residuals as the forward home (D-47 packet §5 row 4); the D-06b gate and everything else in the item stay intact.

---

## Action SCA-007-A006 — DEL-10-04 history line

**(a) Target:** `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_STATUS.md` (Last Updated already reads 2026-07-16 — no header change needed)

**(b) Current text:**

```
## History
- 2026-07-16 - Owner adopted CB-2026-07-15-DEL-10-04-CIBROWSER-001; `docs/BUILD_AND_RELEASE.md` §7 now records provider-neutral Playwright browser provisioning and maps both source-mode and production-dist lanes into the DEC-025-ordered phase sequence. The two documentation residuals closed; hosted/public CI, signing/notarization, publication, release authority, and lifecycle remain unchanged.
```

**(c) Replacement text:**

```
## History
- 2026-07-16 - DEC-080/SCA-007 (D-47 O-A) propagation: the D-06b signing item's "PRD §22.6" citation re-keyed to the historical v0.1 token read through the D-21 Annex A crosswalk, with forward home in the DEC-056 R6-entry release-machinery residuals; gate suffix and item scope unchanged. No lifecycle change.
- 2026-07-16 - Owner adopted CB-2026-07-15-DEL-10-04-CIBROWSER-001; `docs/BUILD_AND_RELEASE.md` §7 now records provider-neutral Playwright browser provisioning and maps both source-mode and production-dist lanes into the DEC-025-ordered phase sequence. The two documentation residuals closed; hosted/public CI, signing/notarization, publication, release authority, and lifecycle remain unchanged.
```

**(d) Rationale:** D-47 packet §6 history-line requirement for the DEC-080 re-key in A005.

---

## Action SCA-007-A007 — DEL-10-05 _STATUS.md: verify-no-change

**(a) Target:** `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_STATUS.md`

**(b)/(c) No edit.** The checked line is a method description, not a criterion citation:

```
- Bind benchmark/regression runner payloads (E2 per-case reproduction currently runs through suite tests) (source: TP-E2-VALMANUAL-001 residuals)
```

**(d) Rationale:** D-47 packet §5 row 5 anticipated verify-no-change; confirmed under DEC-080 — the item describes how reproduction currently runs, and does not encode the superseded criterion.

---

## Action SCA-007-A008 — DEL-14-05: verify-no-change

**(a) Target:** `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/ScopeOfWork.md` and `_STATUS.md`

**(b)/(c) No edit.** Neither file frames prover/CAEPIPE correlation as merely "optional"/"future" validation evidence. The closest passages are exclusions that remain true under DEC-080 (comprehensive commercial-tool ingestion stays deferred per `DEC-016`):

```
| External validation | Excluded. PKG-14 does not ingest commercial prover outputs comprehensively or determine external validation. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-14`. |
```

and

```
| Broad external comparison support versus bounded MVP | Stay within deterministic state/run comparison and comparison export semantics; comprehensive commercial prover ingestion is excluded by PKG-14. |
```

**(d) Rationale:** DEC-080 changes the PRD's validation posture, not PKG-14's ingestion exclusion; no scope text here understates prover correlation, so NO_CHANGE_VERIFIED (D-47 packet §5 row 6).

---

## Action SCA-007-A009 — DEL-15-04: verify-no-change

**(a) Target:** `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-04_External prover boundary metadata/ScopeOfWork.md` and `_STATUS.md`

**(b)/(c) No edit.** The file's framing is "diagnostic/handoff support, not proof of external verification sufficiency" (e.g. requirement DEL-15-04-R6, and:

```
| Comparison linkage | Comparison reports may support external-prover workflows but are diagnostic/handoff support only | `docs/_Registers/ScopeLedger.csv` rows SOW-073 and SOW-075; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-015 |
```

). This bounds the *metadata surface's* claim authority, which DEC-080 leaves unchanged ("These comparisons are validation evidence, not automatic professional acceptance" is retained); it does not frame prover correlation itself as optional/future validation evidence.

**(d) Rationale:** No qualifying framing found; NO_CHANGE_VERIFIED under DEC-080 (D-47 packet §5 row 6, no scope expansion).

---

## Action SCA-007-A010 — DEL-17-04: verify-no-change

**(a) Target:** `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/ScopeOfWork.md` and `_STATUS.md`

**(b)/(c) No edit.** Grep for optional/future/prover/CAEPIPE/correlation/validation-evidence framing finds only writer-contract boundaries (no compatibility/professional-acceptance claims) and "future implementation" checklist rows about the writer itself (e.g. lines 228, 234, 237); nothing frames prover correlation as merely optional/future validation evidence.

**(d) Rationale:** No qualifying framing found; NO_CHANGE_VERIFIED under DEC-080 (D-47 packet §5 row 6).

---

## Action SCA-007-A011 — DEL-17-05 ScopeOfWork.md currency note (CLM-043)

**(a) Target:** `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/ScopeOfWork.md`

Qualifying framing found (basis for the note): OUT-001 ("An optional user-owned CAEPIPE external-run harness and bounded CSV-parser contract for regression and handoff evidence"), the Datasheet harness-role row ("Optional external execution wrapper..."), and the Evidence-records row ("Future implementation shall classify evidence as regression/handoff evidence, not formal validation or professional approval"). These predate DEC-080's re-weighting of prover correlation to the principal validation posture.

**(b) Current text:**

```
> Invented public fixtures remain rights-safe test inputs, but user-provided CSV/runtime evidence must not inherit their public classification by default.

## Output and Evaluation Matrix
```

**(c) Replacement text:**

```
> Invented public fixtures remain rights-safe test inputs, but user-provided CSV/runtime evidence must not inherit their public classification by default.

### CLM-043 — DEC-080 validation-posture currency note (2026-07-16)

> ##### DEC-080 validation-posture currency note (2026-07-16)
>
> Per `DEC-080` (D-47 O-A, SCA-007, 2026-07-16), external-prover correlation (e.g., CAEPIPE) is now the amended PRD's principal validation posture (v0.3 §22.5) rather than merely future/optional validation evidence; activation remains owner-gated on lawful tool procurement, and this deliverable's bounded contract — optional user-owned harness, O10 gates, and regression/handoff classification of raw runs and parsed CSVs short of accepted correlation comparisons — is unchanged, with no scope expansion.

## Output and Evaluation Matrix
```

**(d) Rationale:** D-47 packet §5 row 6 directs a currency note wherever scope text frames prover correlation as merely optional/future; DEC-080 makes it the principal validation posture, activation owner-gated, no scope expansion.

---

## Action SCA-007-A012 — DEL-17-05 _STATUS.md history line and Last Updated bump

**(a) Target:** `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/_STATUS.md`

**(b) Current text (two independent blocks):**

Block 1:

```
**Last Updated:** 2026-07-12
```

Block 2:

```
## History
- 2026-07-12 - D-41 R5 T7 PDU-055 refreshed 6 cited declaration claims to current T1-T6 authority/evidence while preserving genuine residuals; cited-claim and Remaining backcheck closed the D-41 bootstrap.
```

**(c) Replacement text:**

Block 1:

```
**Last Updated:** 2026-07-16
```

Block 2:

```
## History
- 2026-07-16 - DEC-080/SCA-007 (D-47 O-A) propagation: ScopeOfWork currency note CLM-043 added — external-prover correlation is now the amended PRD's principal validation posture (v0.3 §22.5), activation owner-gated; the optional/user-owned harness contract, O10 gates, and PDU-050 validation hold are unchanged. No scope expansion; no lifecycle change.
- 2026-07-12 - D-41 R5 T7 PDU-055 refreshed 6 cited declaration claims to current T1-T6 authority/evidence while preserving genuine residuals; cited-claim and Remaining backcheck closed the D-41 bootstrap.
```

**(d) Rationale:** D-47 packet §6 history-line requirement for the DEC-080 currency note in A011.

---

## Action SCA-007-A013 — Loop workplan: goal-line yardstick re-point

**(a) Target:** `loop/WORKPLAN_2026-07-10_piping_loop.md` (lines 27-28)

**(b) Current text:**

```
chirality-piping toward completion per the PRD yardstick (`docs/PRD.md` §10 functional
requirements, §22 release milestones) and physical-model correctness, as far as live
```

**(c) Replacement text:**

```
chirality-piping toward completion per the PRD yardstick (`docs/PRD.md`, the amended
v0.3 after DEC-080/SCA-007: §11 functional requirements, §24 release milestones;
pre-amendment v0.1 text archived at `docs/_history/PRD_v0.1.md`) and
physical-model correctness, as far as live
```

**(d) Rationale:** DEC-080 relocates the amended PRD to `docs/PRD.md`; the yardstick tokens move to the v0.2-layout sections (§11 FRs, §24 milestones). Pointer fix only, no protocol change (D-47 packet §5 row 7).

---

## Action SCA-007-A014 — Loop workplan: pointer-index yardstick re-point

**(a) Target:** `loop/WORKPLAN_2026-07-10_piping_loop.md` (line 141)

**(b) Current text:**

```
- **What must be built and why:** `execution/_Decomposition/SOFTWARE_DECOMP.md` ·
  product yardstick: `docs/PRD.md` (§10, §22) · strategic orientation: `docs/PLAN.md`
  (non-governing).
```

**(c) Replacement text:**

```
- **What must be built and why:** `execution/_Decomposition/SOFTWARE_DECOMP.md` ·
  product yardstick: `docs/PRD.md` (v0.3 after DEC-080/SCA-007; §11 functional
  requirements, §24 release milestones; pre-amendment v0.1 archived at
  `docs/_history/PRD_v0.1.md`) · strategic orientation: `docs/PLAN.md`
  (non-governing).
```

**(d) Rationale:** Same as A013 (DEC-080; D-47 packet §5 row 7). Pointer fix only.

---

## Action SCA-007-A015 — docs/PLAN.md control-plane yardstick row

**(a) Target:** `docs/PLAN.md` (line 32)

**(b) Current text:**

```
| What must be built; the release yardstick | [docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md](_ScopeChange/OpenPipeStress_PRD_v0.2.md), adopted by `D-21` / `DEC-056` and activated by `SCA-005`; historical v0.1 references in [PRD.md](PRD.md) read forward through the D-21 Annex A crosswalk |
```

**(c) Replacement text:**

```
| What must be built; the release yardstick | [docs/PRD.md](PRD.md), adopted by `D-21` / `DEC-056` / `SCA-005` and amended/relocated to v0.3 by `DEC-080` / `SCA-007`; historical v0.1 text archived at [docs/_history/PRD_v0.1.md](_history/PRD_v0.1.md), read forward through the D-21 Annex A crosswalk |
```

**(d) Rationale:** DEC-080 relocation — the adopted PRD authority now lives at `docs/PRD.md` (D-47 packet §5 row 8).

---

## Action SCA-007-A016 — docs/PLAN.md §1 forward-authority paragraph

**(a) Target:** `docs/PLAN.md` (lines 42-47)

**(b) Current text:**

```
SCA-005 accepted the D-21 / DEC-056 PRD scope change on 2026-07-04. Forward
completion now reads against the adopted v0.2 PRD authority in
`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`, with historical v0.1
requirements and ruled records translated through the D-21 packet's Annex A
crosswalk. The v0.1 PRD text remains in `docs/PRD.md` as historical authority
for pre-SCA records, not as the forward completion yardstick.
```

**(c) Replacement text:**

```
SCA-005 accepted the D-21 / DEC-056 PRD scope change on 2026-07-04; SCA-007
(D-47 / DEC-080, 2026-07-16) amended the adopted PRD to v0.3 and relocated it
to `docs/PRD.md`. Forward completion now reads against `docs/PRD.md` (v0.3),
with historical v0.1 requirements and ruled records translated through the
D-21 packet's Annex A crosswalk. The v0.1 PRD text is archived verbatim at
`docs/_history/PRD_v0.1.md` as historical authority for pre-SCA records, not
as the forward completion yardstick; a redirect stub remains at
`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`.
```

**(d) Rationale:** DEC-080 relocation and citation-resolution arrangement (D-47 packet §5 row 8, O-A Part 2).

---

## Action SCA-007-A017 — docs/PLAN.md §1 version tokens on the two completion surfaces

**(a) Target:** `docs/PLAN.md` (lines 52 and 57)

**(b) Current text (two independent blocks):**

Block 1:

```
1. **Release milestones (v0.2 PRD §24).** R0–R7 exit criteria, taken as the
```

Block 2:

```
2. **Functional requirements (v0.2 PRD §11).** Namespaced FR families
```

**(c) Replacement text:**

Block 1:

```
1. **Release milestones (PRD v0.3 §24).** R0–R7 exit criteria, taken as the
```

Block 2:

```
2. **Functional requirements (PRD v0.3 §11).** Namespaced FR families
```

**(d) Rationale:** The adopted PRD is v0.3 after DEC-080/SCA-007; section numbering is unchanged from v0.2.

---

## Action SCA-007-A018 — docs/PLAN.md §2 R5 milestone row: amended exit-evidence tokens

**(a) Target:** `docs/PLAN.md` (line 88)

**(b) Current text:**

```
| R5 Engineering Beta (current target) | Current ordinary in-stage target under the DEC-054 stage gate, now read forward through DEC-056/SCA-005 | Phase E work now opens: validation manual/evidence-system completion, full report package/PDF, redaction workflow, signed releases, public issue templates, protected-content release scan, release-quality gates, and R5/R6-entry decision packets (`D-06`, `D-10b`, `D-20`, `D-05b`) |
```

**(c) Replacement text:**

```
| R5 Engineering Beta (current target) | Current ordinary in-stage target under the DEC-054 stage gate, read forward through DEC-056/SCA-005 and expressed in amended-PRD tokens per DEC-080/SCA-007 | Phase E work now opens: validation manual/evidence-system completion, full report package/PDF, redaction workflow, signed releases, public issue templates, protected-content release scan, release-quality gates, and R5/R6-entry decision packets (`D-06`, `D-10b`, `D-20`, `D-05b`); exit evidence reads as actor-neutral clean-checkout reproduction of the validation examples (amended PRD v0.3 §24 R6, DEC-080) plus the unchanged protected-standards criterion (v0.3 §20.1 / D-20 lineage) |
```

**(d) Rationale:** D-47 packet §5 row 8 — the Phase E row cites the amended criterion; the stage itself does not move.

---

## Action SCA-007-A019 — docs/PLAN.md §3 Intent → Requirements tokens

**(a) Target:** `docs/PLAN.md` (lines 104-105)

**(b) Current text:**

```
- **Intent → Requirements.** `INTENT.md` (why / what must remain true) and the
  `PRD.md` (what must be built, §10 FRs and §22 milestones) define the target.
```

**(c) Replacement text:**

```
- **Intent → Requirements.** `INTENT.md` (why / what must remain true) and the
  `PRD.md` (what must be built; v0.3 §11 FR families and §24 release
  milestones) define the target.
```

**(d) Rationale:** After the DEC-080 relocation, `docs/PRD.md` carries the v0.3 layout; the stale v0.1 §10/§22 tokens would misdirect readers of this forward-authority description. (Additional pointer surface beyond the packet's "at least" rows 32/42/88, same DEC-080 basis.)

---

## Action SCA-007-A020 — _COORDINATION.md Current Target Stage: ruled re-expression in amended tokens

**(a) Target:** `execution/_Coordination/_COORDINATION.md`

**(b) Current text:**

```
**Current target stage - PRD R5 exit criterion (advanced 2026-06-23 by
`DEC-054`, after the conditional R4 target-stage gate in `D-27`).** Phase D/R4
evidence was reviewed through
`plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md`
(`TP-R4-D9-EXITCHAINREFRESH-001`) and clean-head sweep
`validation/evidence/sweeps/SWEEP_20260623T051552Z_16cca07f3b64.json`.
The human project authority accepted the refreshed packet as a conditional R4
gate and authorized R4-to-R5 target-stage advancement. The ordinary in-stage
program is now Phase E: engineering beta and release machinery. R5 exit
criteria are PRD §22.6: external engineers can reproduce validation examples,
and the public repository contains no known protected standards data. The PRD
§16.2 / §16.5 complete benchmark/manual evidence system remains explicit
residual work to gather later, including once the agent harness is active and
can participate. Boundary prohibitions continue unchanged: no protected
standards content, private-data defaults, unapproved release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim.
```

**(c) Replacement text:**

```
**Current target stage - PRD R5 exit criterion (advanced 2026-06-23 by
`DEC-054`, after the conditional R4 target-stage gate in `D-27`).** Phase D/R4
evidence was reviewed through
`plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md`
(`TP-R4-D9-EXITCHAINREFRESH-001`) and clean-head sweep
`validation/evidence/sweeps/SWEEP_20260623T051552Z_16cca07f3b64.json`.
The human project authority accepted the refreshed packet as a conditional R4
gate and authorized R4-to-R5 target-stage advancement. The ordinary in-stage
program is now Phase E: engineering beta and release machinery. The exit
criteria, expressed in amended-PRD tokens per `DEC-080`, are: validation
examples reproduce from a clean checkout by following the documented
validation-manual procedure, with recorded environment, tool versions,
commands, exit codes, and output hashes, actor-neutral (maintainer- or
agent-executable) — amended PRD (v0.3) §24 R6; and the public repository
contains no known protected standards data (v0.3 §20.1 / D-20 lineage). The
historical v0.1 token "PRD §22.6: external engineers can reproduce validation
examples" resolves through the D-21 Annex A crosswalk. The complete
benchmark/manual evidence system — historically cited as PRD §16.2 / §16.5,
now the v0.3 §22.2 required solver benchmarks plus the §24 R6
validation-manual deliverable — remains explicit residual work to gather
later, including once the agent harness is active and can participate.
Boundary prohibitions continue unchanged: no protected standards content,
private-data defaults, unapproved release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim.
This record was re-expressed 2026-07-16 in amended-PRD tokens under
`DEC-080`/SCA-007; the target stage itself is unchanged.
```

**(d) Rationale:** D-47 O-A Part 4 (owner-approved within the DEC-080 ruling): re-express the ruled stage record in amended-PRD tokens with no stage advancement — DEC-054 stage authority and the Phase E posture are preserved verbatim. Token mapping basis: v0.1 §16.2 "Required Solver Benchmarks" → v0.3 §22.2 (identical heading); v0.1 §16.5 "Validation Manual" has no §22 subsection successor in the v0.2/v0.3 layout — the validation manual is carried as the §24 R6 deliverable ("validation manual", v0.2 line 1576), whose exit criterion DEC-080 amends.

---

## Action SCA-007-A021 — D-46 packet supersession note

**(a) Target:** `execution/_Coordination/_DECISIONS/D-46_r5_external_reproduction_acceptance_protocol.md`

**(b) Current text (end of file; the note is appended after this block):**

```
<!-- BEGIN OWNER RULING VERBATIM -->
My ruling is O-C on D-46 and adopting the candidate brief.
<!-- END OWNER RULING VERBATIM -->
```

**(c) Replacement text:**

```
<!-- BEGIN OWNER RULING VERBATIM -->
My ruling is O-C on D-46 and adopting the candidate brief.
<!-- END OWNER RULING VERBATIM -->

## Supersession note (2026-07-16)

The question this packet governed — what evidence satisfies the reproduction
exit criterion and how it is accepted — is superseded by D-47 Option O-A,
codified as `DEC-080` and executed as SCA-007: the criterion is now
actor-neutral (validation examples reproduce from a clean checkout per the
amended PRD v0.3 §24 R6, maintainer- or agent-executable), and independent
third-party reproduction is re-homed as a publication-era credibility
objective under the `D-05b` public-repository gate family, no longer a
milestone exit criterion. The O-C deferral ruling recorded above remains
immutable history and is not rewritten. No acceptance-protocol work proceeds
under this packet.
```

**(d) Rationale:** D-47 packet §5 row 10 — append a supersession note preserving the O-C disposition as immutable history.

---

## Action SCA-007-A022 — _LATEST.md scope-change pointer: full replacement

**(a) Target:** `execution/_ScopeChange/_LATEST.md`

**(b) Current text:** entire file (currently points at SCA-005).

**(c) Replacement text (full file content):**

```
# Latest Scope Change

|Field|Value|
|---|---|
|AmendmentID|SCA-007|
|Snapshot|`execution/_ScopeChange/SCA-007_2026-07-16_2026/`|
|Status|accepted|
|Created|2026-07-16|
|Summary|D-47 / DEC-080 ruling executed as SCA-007: adopted PRD amended v0.2 → v0.3 (actor-neutral clean-checkout §24 R6 reproduction exit criterion; §22.5 retitled — external-prover correlation is the principal validation posture, activation owner-gated on lawful tool access; §22.1 verification-philosophy paragraph; independent third-party reproduction re-homed to the D-05b publication-era gates) and relocated to `docs/PRD.md` with the v0.1 text archived at `docs/_history/PRD_v0.1.md` and a redirect stub at `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`; citation-resolution rule adopted (pre-amendment `docs/PRD.md` references resolve against the archived v0.1 plus the D-21 Annex A crosswalk); packet §5 propagation into impacted deliverables and live pointer surfaces; evidence-bundle home ruled `validation/evidence/reproduction/<run-id>/`; SOFTWARE_DECOMP advanced to revision v0.10. No scope item, dependency, requirement, package membership, lifecycle, stage, release, prover-activation, reproduction-acceptance, or professional claim is created.|

Note: SCA-006 executed under `D-43` as SOFTWARE_DECOMP revision v0.9 without a
snapshot folder; this pointer previously sat stale at SCA-005.
```

**(d) Rationale:** D-47 packet §5 row 11 — move the pointer to SCA-007 and record the SCA-006 folderless-execution note (packet §2).

---

## Action SCA-007-A023 — SOFTWARE_DECOMP.md revision v0.10 note

**(a) Target:** `execution/_Decomposition/SOFTWARE_DECOMP.md`

**(b) Current text:**

```
Revision v0.9 records `SCA-006`, a scope-neutral currency amendment under project decision `D-43`: the eight `PKG-00` four-document kits are consolidated in place into per-member `ArchitectureBasis.md` reference documents (classification `ARCHITECTURE_BASIS_REFERENCE`); SOW-059 and DEC-010 currency notes are corrected to cite their ruled decisions; no scope item, dependency, requirement, or package membership changes.
```

**(c) Replacement text:**

```
Revision v0.9 records `SCA-006`, a scope-neutral currency amendment under project decision `D-43`: the eight `PKG-00` four-document kits are consolidated in place into per-member `ArchitectureBasis.md` reference documents (classification `ARCHITECTURE_BASIS_REFERENCE`); SOW-059 and DEC-010 currency notes are corrected to cite their ruled decisions; no scope item, dependency, requirement, or package membership changes.

Revision v0.10 records `SCA-007`, propagating the accepted `D-47` / `DEC-080` amendment: the adopted PRD is amended v0.2 → v0.3 (actor-neutral clean-checkout §24 R6 reproduction exit criterion; §22.5 retitled to make external-prover correlation the principal validation posture with owner-gated activation; §22.1 verification-philosophy paragraph; independent third-party reproduction re-homed to the `D-05b` publication-era gate family) and relocated to `docs/PRD.md`, with the v0.1 text archived verbatim at `docs/_history/PRD_v0.1.md`, a redirect stub at `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`, and a citation-resolution rule (pre-amendment `docs/PRD.md` references resolve against the archived v0.1 plus the D-21 Annex A crosswalk); the D-47 packet §5 deliverable and pointer propagation executes under SCA-007; no scope item, dependency, requirement, or package membership changes.
```

**(d) Rationale:** DEC-080 disposition column advances the decomposition to revision v0.10 at SCA-007 closeout; note drafted in the style of the adjacent v0.9 paragraph.

---

## Action SCA-007-A024 — SOFTWARE_DECOMP.md §13 gate-posture currency bump

**(a) Target:** `execution/_Decomposition/SOFTWARE_DECOMP.md` §13

**(b) Current text (two independent blocks).** Block 1 (line 663):

```
This v0.9 decomposition is the accepted current decomposition basis after SCA-006.
```

Block 2 (the §13 closing paragraph; the additional currency tokens found are quoted here in place — "as amended through SCA-006", "may predate revision 0.9", and "SCA-006 does not itself approve"):

```
PREPARATION may scaffold package and deliverable folders from this decomposition and the companion registers after ORCHESTRATOR plans the accepted downstream refreshes. `PKG-00` is retained architecture-basis reference context per HUMAN-STEER-PKG00-EXCLUSION-001 and `D-43`, not a production package awaiting issuance, and its SCA-001 architecture basis as amended through SCA-006 may be injected into `PKG-01` through `PKG-17` sealed contexts and future TASK briefs. Type 2 execution still requires one sealed deliverable context, explicit write scope, applicable invariants, and acceptance criteria. Existing DEV-001 dispatch, immutable DAG snapshots, lifecycle history, implementation evidence, schemas, docs, and code may predate revision 0.9 until refreshed by their owning workflows; SCA-006 does not itself approve a DAG successor, change lifecycle, create implementation authority, or make release/professional claims.
```

**(c) Replacement text.** Block 1:

```
This v0.10 decomposition is the accepted current decomposition basis after SCA-007.
```

Block 2:

```
PREPARATION may scaffold package and deliverable folders from this decomposition and the companion registers after ORCHESTRATOR plans the accepted downstream refreshes. `PKG-00` is retained architecture-basis reference context per HUMAN-STEER-PKG00-EXCLUSION-001 and `D-43`, not a production package awaiting issuance, and its SCA-001 architecture basis as amended through SCA-007 may be injected into `PKG-01` through `PKG-17` sealed contexts and future TASK briefs. Type 2 execution still requires one sealed deliverable context, explicit write scope, applicable invariants, and acceptance criteria. Existing DEV-001 dispatch, immutable DAG snapshots, lifecycle history, implementation evidence, schemas, docs, and code may predate revision 0.10 until refreshed by their owning workflows; SCA-007 does not itself approve a DAG successor, change lifecycle, create implementation authority, or make release/professional claims.
```

**(d) Rationale:** DEC-080 — the gate posture's currency tokens follow the revision bump to v0.10/SCA-007; all other §13 semantics are unchanged.

---

## Action SCA-007-A025 — Redirect stub content for the v0.2 PRD path

**(a) Target:** `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` (parent executes the relocation; this is the full replacement content for the stub left at this path)

**(b) Current text:** entire v0.2 PRD file (1774 lines; preserved in git history by the relocation commit).

**(c) Replacement text (full stub content):**

```
# OpenPipeStress PRD v0.2 — Superseded In Place (Redirect Stub)

Superseded in place on 2026-07-16 by `D-47` / `DEC-080` / SCA-007. The adopted
PRD authority is `docs/PRD.md` (v0.3, amended and relocated under
DEC-080/SCA-007). The v0.2 text formerly at this path is preserved verbatim in
git history; references to this path resolve against `docs/PRD.md` — the v0.2
section and milestone numbering is unchanged in v0.3. The historical v0.1 text
is archived at `docs/_history/PRD_v0.1.md` and reads forward through the D-21
Annex A crosswalk.

No lifecycle advance, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim is created by
this stub.
```

**(d) Rationale:** D-47 O-A Part 2 / DEC-080 — a short redirect stub remains at the v0.2 path for the ~17 existing references.

---

## Action SCA-007-A026 — Forward Authority Note re-key for the archived v0.1 file

**(a) Target:** `docs/_history/PRD_v0.1.md` (the v0.1 text moves there verbatim from `docs/PRD.md`; only its Forward Authority Note is re-keyed — the rest of the v0.1 body is untouched)

**(b) Current text (the note as it stands at the top of the current `docs/PRD.md`):**

```
## Forward Authority Note

SCA-005 accepted the D-21 / DEC-056 ruling on 2026-07-04. For forward work,
`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` is the adopted PRD authority.
This file preserves the v0.1 PRD text for historical references and for ruled
records that were written before the v0.2 adoption.

Forward traceability from the flat v0.1 `FR-001..FR-025` identifiers to the
v0.2 namespaced FR families is governed by Annex A of
`execution/_Coordination/_DECISIONS/D-21_prd_scope_change_v0_2_milestone_set.md`.
Ruled history keeps its original v0.1 milestone and FR tokens; new planning and
execution records should cite the v0.2 PRD plus the Annex A crosswalk when they
need to relate old and new identifiers. The v0.1 R5 release-machinery items are
carried forward as explicit R6-entry residuals under DEC-056.

No lifecycle advance, release-readiness claim, professional approval,
certification, sealing, authentication, code-compliance claim, live binding, or
app-dev package consumption is created by this authority note.
```

**(c) Replacement text (the re-keyed note in the archived file):**

```
## Forward Authority Note

SCA-005 accepted the D-21 / DEC-056 ruling on 2026-07-04, and SCA-007
(D-47 / `DEC-080`, 2026-07-16) amended the adopted PRD to v0.3 and relocated
it. For forward work, `docs/PRD.md` (v0.3) is the adopted PRD authority.
This file is the v0.1 PRD text, archived verbatim, for historical references
and for ruled records that were written before the v0.2 adoption.

Forward traceability from the flat v0.1 `FR-001..FR-025` identifiers to the
namespaced FR families is governed by Annex A of
`execution/_Coordination/_DECISIONS/D-21_prd_scope_change_v0_2_milestone_set.md`.
Ruled history keeps its original v0.1 milestone and FR tokens; new planning and
execution records should cite `docs/PRD.md` (v0.3) plus the Annex A crosswalk
when they need to relate old and new identifiers. The v0.1 R5 release-machinery
items are carried forward as explicit R6-entry residuals under DEC-056.

No lifecycle advance, release-readiness claim, professional approval,
certification, sealing, authentication, code-compliance claim, live binding, or
app-dev package consumption is created by this authority note.
```

**(d) Rationale:** D-47 O-A Part 2 / DEC-080 — only the forward-authority pointer is re-keyed (a forward-pointer currency correction, not a history rewrite); the no-claims sentence is kept verbatim.

---

## Drafter observations (for the parent's fan-in)

1. The `_COORDINATION.md` §16.2/§16.5 mapping: v0.1 §16.1-16.4 map like-for-like onto v0.2/v0.3 §22.1-22.4 (identical headings). v0.1 §16.5 "Validation Manual" has **no** §22 subsection successor — in the v0.2/v0.3 layout the validation manual appears only as a §24 R6 deliverable (v0.2 line 1576) and a risk-mitigation mention (line 1639). A020 therefore maps §16.2 → §22.2 and §16.5 → the §24 R6 validation-manual deliverable.
2. `SOFTWARE_DECOMP.md` lists its revision paragraphs out of order (v0.9 at line 35 precedes v0.8 at lines 37-43). A023 inserts v0.10 directly after v0.9 per the tasking ("beside"); the pre-existing v0.9/v0.8 inversion is left as found.
3. `docs/PRD.md` (v0.1) header lines 5-7 ("Document version: 0.1 Draft, preserved as historical text", "Status: Historical v0.1 product definition; forward authority delegated by SCA-005") travel verbatim with the archive; they remain accurate (SCA-005 did delegate forward authority) and were not re-keyed, per the "leave the v0.1 body untouched" bound.
4. `docs/PLAN.md` line ~94 also carries "PRD §16.2/§16.5" inside a dated "2026-06-23 correction" block; it is a dated historical note, left unedited (resolves through the citation-resolution rule).
5. DEL-15-04's `_STATUS.md` contains no prover-framing text; DEL-14-05/DEL-17-04 `_STATUS.md` likewise. Only DEL-17-05's ScopeOfWork carries the optional/future prover-evidence framing, hence the single currency note (A011/A012).
