# Prior and Corrected Final-Byte Verifier Returns — WI-PKG08-DEL0801

## Prior accepted BLOCK

**Prior candidate SHA-256:**
`22aba867501c5dd61a865c3764ac7042a414c00c46ef6b799b6c63210bc144ca`
**Prior candidate bytes:** `15,516`
**Write authority:** none
**Verifier identity:**
`/root/working_items_pkg08_candidate_design/del0801_final_byte_verifier`
**Verifier status:** `COMPLETED`
**Fan-in verdict:** `BLOCK — ACCEPTED`

## Terminal verifier return

The exact authorized child was resumed once and returned terminally from
already gathered read-only evidence.

Observed:

- candidate SHA-256
  `22aba867501c5dd61a865c3764ac7042a414c00c46ef6b799b6c63210bc144ca`;
- size `15,516` bytes;
- frozen checkout
  `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`;
- current seam/gap, W1-W3, exclusions, registered checks, toolchain hold,
  adoption gate, and proposal-only/no-lifecycle language otherwise accurate
  and narrowly fenced.

Blocking finding:

- AC-01 requires W1 to equal raw TypeScript builder output.
- That builder deliberately emits `private_project_data`, `private_only`, and
  `pending` provenance at
  `renderableReportInput.ts:73-83,135-145,163-167,378-380,449-450`.
- The TypeScript tests preserve those classifications at
  `renderedReport.test.tsx:338-392` and prove restored user-local report data
  is blocked before native IPC at lines 424-449.
- The Rust renderer maps those private classifications into the linter and
  uses a `PublicReportExample` lint target at
  `report_renderer/src/lib.rs:239-305,854-887`.
- Existing unblocked Rust fixture coverage instead uses
  `InventedPublicExample`, `InventedNonEngineeringExample`, and `Accepted` at
  `report_renderer/tests/render.rs:13-40`.
- AC-04's `export_blocked=false` and AC-05's invented-public classification
  are therefore unsupported for the exact raw W1 payload required by AC-01.

Required correction:

Revise the candidate to distinguish raw builder-output/private-boundary
evidence from a separately authorized export-eligible sanitized payload, or
remove the unblocked/public requirement and assert the correct gated outcome.
Any revision changes candidate bytes and requires a new hash plus fresh
final-byte verification.

## WORKING_ITEMS validation and disposition

WORKING_ITEMS independently opened every cited range and confirmed the
classification mismatch, redaction-blocking test, renderer provenance mapping,
public-surface lint target, and existing Rust fixture classifications. The
child stayed read-only and returned the required verdict, evidence, and
correction.

Fan-in disposition: `ACCEPT_BLOCK`. Candidate bytes remain unchanged and
proposal-only. Adoption and implementation release are held pending a
separately authorized candidate correction and fresh refutation.

## Corrected-candidate verification

**Corrected candidate SHA-256:**
`030c5f7821ac93ce71f64f8b48fbfcf454d858dbd27faf49c22dd1c641c12229`
**Corrected candidate bytes:** `19,951`
**Write authority:** none
**Fresh verifier identity:**
`/root/working_items_pkg08_candidate_design/del0801_s1_corrected_final_byte_verifier`
**Verifier status:** `COMPLETED`
**Fan-in verdict:** `PASS — ACCEPTED`

## Terminal corrected-candidate return

The fresh verifier independently confirmed:

- exact SHA-256
  `030c5f7821ac93ce71f64f8b48fbfcf454d858dbd27faf49c22dd1c641c12229`;
- exact size `19,951` bytes;
- frozen/live HEAD
  `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`;
- TypeScript emits private/private-only/pending component provenance;
- the lossless public-route control yields blocked/null/withheld and the panel
  stops before IPC;
- Tauri only deserializes and renders;
- generator boundary gaps and pending-review lint are warnings, while the
  renderer blocks only blocking diagnostics/findings;
- the independent Rust fixture uses public/invented/accepted
  classifications; and
- no accepted lossless export-eligible TypeScript transformation exists.

The verifier found the candidate source-faithful and internally coherent. It
preserves the TypeScript-to-Rust gap, prohibits invented sanitation and UI
bypass, requires distinct future raw/private and eligible lanes, withholds all
writes, keeps checks and toolchain prerequisites conditional, requires human
adoption plus rerun, and has no implementation, lifecycle, or Git effect.

No correction is required.

## WORKING_ITEMS final disposition

WORKING_ITEMS accepted the terminal `PASS`.
Fan-in disposition:
`ACCEPT_PASS / HOLD_FOR_ACCEPTED_TRANSFORMATION`. Candidate status remains
`CANDIDATE / NOT ADOPTED / PROPOSAL ONLY / STOPPED`.
