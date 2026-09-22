# R6 backcheck — bounded Piping record repair

Deterministic verdict: **PASS** on the file-hash-bound current derivative.
Integration still requires root Agent 0's independent source/delta/record
reviews and actual-candidate registered/CI checks. The root review records are
the source for that gate, not this deterministic verdict.

| Population | Result |
|---|---:|
| Changed deliverable source files | 287 |
| Re-extracted changed references, including collateral units | 1,721 |
| Legacy minted subclaims explicitly bound to parent envelopes | 252 |
| Original discovery claims individually accounted | 9,889 |
| Original divergent claims, unchanged historical denominator | 3,299 |
| Direct record-defect repairs in the discovery claim population | 475 |
| Remaining-record premises corrected with substantive residual retained | 29 |
| Partially repaired discovery claims | 410 |
| Complete H4 row accounting | 2,234 |
| Capability accounting, with no scope/implementation mutation | 598 |
| Remaining census deliverables | 102 |
| Remaining census rows | 188 |

The census contains 151 recorded items across 65 deliverables and 37 explicit
NONE records. NONE is not proof of completion. `ACCOUNTING_SUMMARY.json` keeps
the other outcomes: unchanged source, original no-repair rows, preserved
aligned claims with new bindings, and unresolved findings affected only by a
containing edit. These outcomes must not be summed into a project-completion
claim.

## Reproduction and independent bases

The repair manifest selects changed units using old source ranges intersecting
actual line changes; the checker independently re-extracts complete old/new
unit dictionaries and compares exact key multisets and hashes. Bespoke paired
CSV ROWS are selected by changed row identity/content and checked against the
stage4 physical inverse/replay. A copied manifest is not the backcheck output.

`CHANGED_CLAIM_REEXTRACTION.csv` is fresh extraction of the final source,
including aligned/enclosing units affected by physical edits. Its exact
multiset equals `R5/REPAIR_MANIFEST.csv`. Direct patches separately retain
before/after text and body hashes, source references and per-operation (a)/(b)
reasons in the task manifests. The 252 parent-envelope bindings explicitly
preserve the older minted-subclaim limitation instead of inventing original
sub-body evidence.

The checker exactly compares the saved full Remaining census with a fresh
current extraction and rejects duplicate deliverable/item identities. Negative
controls remove and duplicate one changed-reference row and one census row;
all four fail as required. An independent integration review caught the first
version's missing persisted-census comparison; the corrected verifier and all
four negative results are recorded in `CHECK_RESULT.json`.

`SOURCE_BINDINGS.csv` binds every source file to baseline
`379df923927d157be3ebb51d8a1dcf783d970112` and the actual repaired bytes.
`FROZEN_DISCOVERY_AUDIT.json` verifies 749 accepted upstream run artifacts:
historical bytes remain unchanged and RUN_STATE remains append-only. Original
discovery source is `00115c71931bcae79909602d653740d3bb72dfa1`.

## Tranche checks and preserved limits

- Initial claim/prose/metadata tranche: 202 files and 829 exact patches;
  three-stage inverse/replay and all 90 changed SoW structure checks pass.
- Supplement: 95 files and 165 exact patches, including 91 required pins and
  15 T5B-C03 source keys. All 92 non-ISSUED SoWs now use one accepted revision
  pin. The 91 changed SoWs validate; paired CSV changes affect only five
  current event/evidence fields, preserving historical/deferred/residual cells.
- STATUS tranche: 64 files, 158 exact operations; 29 inaccurate Remaining
  premises, 57 stale dates and one declaration-history annotation are repaired.
  Strict replay verifies unchanged lifecycle and protected history.
- Dependency mirrors: 30 accepted retired EdgeIDs in 15 files; only Status
  changes, with exact identity and other-cell equality checked against DAG-010.

The initial independent source review and the independent STATUS review have
passed on their frozen scopes; the supplemental delta and final record review
are separately bound in root Agent 0's review folder. No earlier review is
represented as covering later source bytes.

No product source, schema, test, fixture, immutable DAG, protected criterion,
instruction, lifecycle or ISSUED DEL-01-01 file changed. Product suites were
not rerun under the preserved D-73 direction. Prior product gate results retain
their actual revision and scope. Root owns final registered checks and Git
integration; evidence coherence here does not mean engineering approval,
release readiness or accepted deliverable issuance.

See `HANDOFF.md`, `HELD_AND_DEFERRED_AUDIT.md`,
`RIDER_AND_ASSESSMENT_AUDIT.md` and `LESSONS.md` for exact continuation,
derivative limits, prior-decision application and calibration lessons.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
