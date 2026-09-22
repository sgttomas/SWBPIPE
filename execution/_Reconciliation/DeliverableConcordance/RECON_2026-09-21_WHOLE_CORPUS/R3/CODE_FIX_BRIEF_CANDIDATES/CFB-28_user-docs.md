# CFB-28 — User guide, theory note and examples: missing slots and non-claim notices

**Candidate brief (H2). Not executed.** Area: User documentation (DEL-11-01, DEL-11-03, DEL-11-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add an equipment-load slot to user-guide section 9 (or drop it from the outline) and accessibility intent to the guide contract; add the verification-versus-validation distinction and the bar on presenting unverified extracted equations to the theory-note contract; select public/permissive sources for the three deferred theory scopes or record the owner discharge; author a tutorial over the invented examples or record it out of scope; add an explicit certification/approval/sealing/code-compliance non-claim to both example fixture notices and the test's required notice text.

## Affected claims

8 claim rows on 3 deliverable(s): DEL-11-01, DEL-11-03, DEL-11-04.

Classes (portion in this brief / class total): T6-C01 7/180 (Authority NONE); T6-C03 1/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-28"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-11-01:SOW#CLM-006.r07` | T6-C01 | NONE | — | — | Add an equipment-load slot to guide section 9 or drop it from the outline row. |
| `DEL-11-01:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C01 | NONE | — | — | Add the accessibility intent to the contract (and guide), and refresh the PDU-055 current declarations. |
| `DEL-11-01:SOW#production-and-verification-method-praxeology/VER-001.s02` | T6-C01 | NONE | — | — | Add accessibility content to the contract, then review it. |
| `DEL-11-03:SOW#CLM-004.r01` | T6-C01 | NONE | — | — | (none recorded; see Notes in ledger) |
| `DEL-11-03:SOW#CLM-010.s02` | T6-C01 | NONE | — | — | Select public/permissive sources for the three deferred scopes (history, formula-level frame, local-FEA practice) or have the owner discharge them. |
| `DEL-11-03:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C01 | NONE | — | — | Add to the contract the verification-versus-validation distinction and an explicit bar on presenting unverified extracted equations or narrative as a design basis. |
| `DEL-11-04:CONTEXT#anticipated-artifacts` | T6-C01 | NONE | — | — | Author a tutorial that walks through the invented examples, or record a decision that DEL-11-04 tutorials are out of scope. |
| `DEL-11-04:SOW#CLM-011/R-DEL-11-04-002` | T6-C03 | REVIEW | H3[T6-C03];C4 | — | Add an explicit certification, approval, sealing and code-compliance non-claim to both fixture notices (and to the test's required notice text), or narrow the requiremen… |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-11-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-11/DEL-11-01/DEL-11-01_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/docs/user_guide/index.md#L210`, `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/ScopeOfWork.md`, `projects/chirality-piping/docs/user_guide/index.md`.
- DEL-11-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-11/DEL-11-03/DEL-11-03_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/docs/theory/centerline_analysis.md#L17`, `projects/chirality-piping/docs/theory/centerline_analysis.md#L40`, `projects/chirality-piping/docs/theory/centerline_analysis.md#L73`, `projects/chirality-piping/docs/theory/centerline_analysis.md#L49`, `projects/chirality-piping/docs/theory/centerline_analysis.md#L252` ….
- DEL-11-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-11/DEL-11-04/DEL-11-04_forward.csv`; ImplementationEvidence cited: `examples/models/invented/mechanics_only_toy_span.json`, `examples/models/invented/fake_rule_pack_toy_model.json`, `projects/chirality-piping/docs/user_guide/index.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Guide and contract text contain the named slots or the outline is amended (record repair where narrowed).
- The example notice test requires the non-claim wording.
- No protected standards text, tables or equations are introduced (DEC-043); sources are public or permissive.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 1 of 8 rows are at INVARIANT tier or carry protected layers (CLAIMS). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

1 of 8 claim rows carry a block: C4 (1); H3[T6-C03] (1). `H3[<class>]` names the H3 register item for that class (review before repair). Unblocked rows may proceed separately once selected.

## Notes and open views

- DEL-11-04 R-DEL-11-04-002 is CONTESTED;FIELD: the candidate reading makes it a stale requirement overtaken by the DEC-081/DEC-107 directive, which C4 decides; it also needs the T6-C03 review.
- The DEL-11-01 "refresh the PDU-055 current declarations" part is a D-41 declaration question (A9 context).
- DEL-11-03 CLM-010.s02 source selection may need the owner to discharge the deferral (RF-11-03-C-003 human DEFER); no packet names it.

## Dependencies

C4, H3[T6-C03], A9 (context).

