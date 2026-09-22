# DEL-06-04 notes — Private rule-pack lifecycle and checksum handling

Wave W3, PKG-06, worker G2. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger sealed first (116 rows: 80 required keys, 34 optional `.rNN`
rows for 5 split blocks, 2 `.sNN` sub-claims); reverse file written after both
of this worker's seals.

## Path aliases

- `LC` = `core/rules/rule_pack_lifecycle/src/lib.rs` (this deliverable's crate,
  package name `open_pipe_stress_rule_pack_lifecycle`).
- `DOC` = `core/rules/rule_pack_document/src/lib.rs` (C2 seam; the lifecycle
  crate's first production consumer; computes the JCS checksum).
- Desktop product callers: `apps/desktop/src-tauri/src/lib.rs`
  (`validate_rule_pack`, `compute_rule_pack_document_checksum`, local store
  `local_rule_packs` v10) and `apps/desktop/src/services/rulePackService.ts`.
- Parity: root-level `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-06-04/`.
- Gate evidence: `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/` (the sweep log lists
  `cargo test` for rule_pack_lifecycle, rule_pack_document, rule_check_runner).
  No suite was rerun.

## Judgment calls

- **Report references (R-06-04-008, CLM-003.r07, CLM-025.r05, FG-01).** The
  lifecycle crate exposes `audit_manifest_entry` with private-payload
  redaction, and the audit-manifest and report-package crates accept
  rule-pack refs, but every product report request sends `rule_pack_refs: []`
  (`reportPackageRequest.ts`, `renderableReportInput.ts`). Under F7 the claim
  about reports is `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE`, even though the
  "may" wording is permissive; OUT-001 lists report/audit references as
  contract coverage.
- **Non-JSON manifest hashes (R-06-04-005, CLM-003.r03, FG-02).** Only the
  schema enum value `non_json_asset_manifest` and crate `PayloadScope`
  variants exist; nothing produces or checks one. The claim's own "TBD"
  covers only the partition detail, so the row is not "met by construction".
- **Schema-first envelope (R-06-04-012, CLM-004.r05, FG-04).** Rule-pack
  commands return `Result<Value, String>`. The W2 resolution on DEL-00-06
  REQ-06-03 was followed (`PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  PROJECT_BASELINE · RECORD`). The no-bypass half holds only because no
  rule-pack plugin runtime exists (CP-11 portion).
- **Data-gap diagnostics (R-06-04-007).** Every lifecycle diagnostic exists,
  but rule-check-required data gaps come from the DEL-06-03 completeness
  checker in `run_rule_checks`, not from the lifecycle, and not through a
  schema-first envelope. Result: `PARTIALLY_IMPLEMENTED · OWNERSHIP_ELSEWHERE`,
  MEDIUM. Remaining R01 is then `ALIGNED` with `OPEN_ACTION` on this row (F2).
- **Unit metadata (R-06-04-010, CLM-004.r04, CLM-025.r06).** Judged `ALIGNED`:
  the behaviour exists in the product rule-check path through the owning
  DEL-06-02/DEL-06-03 crates, as PDU-044 records. Remaining R03 is an accurate
  standing hold (`NO_OPEN_ACTION`).
- **Storage deferrals (CLM-004.r06, R-06-04-011, CLM-019.r04, C-06-04-001,
  FG-05).** DEC-017 settled the MVP storage profile and rule packs use the
  local SQLite store; DEC-028 settled the container. The deferral text is
  therefore `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING`.
  Encryption, access control, permission persistence and secret handling stay
  deferred to PKG-12, so those parts remain accurate. AC-001 and CLM-025.r07
  word the deferral as "to their governed owners" or "persistence decisions"
  and stay `ALIGNED`.
- **C-06-04-002 (CP-10).** The redistribution enum was settled in the DEL-06-01
  schema without a recorded ruling (the row's human ruling is still TBD),
  following the row's own proposal: `IMPLEMENTED_DIFFERENTLY ·
  AUTHORITY_UNCLEAR · PROJECT_BASELINE · OWNER`, MEDIUM. **Owner-visible.**
- **D-41 declarations (CLM-007/014/021, CP-03).** Accurate existence
  statements with no revision pin, so `ALIGNED`. The residual delegation is not
  relied on (A4).
- **Identification r08, scope, construction, procedure purpose (FG-03).**
  "No implementation files changed" and "setup does not implement" text is
  overtaken by commits `ad270f6` and `e200e47a3`.
- **MEMORY.** Dated entries are accurate history. The undated "Remaining TBDs"
  block says the production JSON canonicalization library is TBD, but
  `rule_pack_document` uses `core/serialization/canonical_json` (RFC 8785).
  It was first present at `7bee9ae41`, so the row is `STALE_SETUP_SPECIFICATION ·
  DOC_BEHIND_CODE`.
- **INIT.md pointers (CLM-006, CLM-017, FG-06).** INIT.md was removed by
  `9c4caf8fd`. Several R-rows also cite INIT.md in their source-basis column;
  that is recorded once on the References and Prerequisites rows and does not
  change those requirement rows, whose substance holds.

## Canonical departures

- CLM-006 and CLM-017 use CP-02 with `STALE_SETUP_SPECIFICATION` instead of
  `STALE_REVIEW_OR_EVIDENCE`, because F3 (ruled after the table) classes text
  first present at `7bee9ae41` as setup-era, and a file pointer is not a
  revision pin. Each row says `CANONICAL_DEPARTURE:`.
- All keyed CS rows (CS-01, 02, 04, 06, 07) keep their assigned fields.
  `architecture-basis-injection.s01` (the PKG-00 SEMANTIC_READY statement)
  follows the W1/W2 AGENT reading: `STALE_REVIEW_OR_EVIDENCE ·
  SCOPE_REDIRECTED_BY_RULING`, the W2 majority cause. `.s02` (Still TBD: the
  grammar was ruled by DEC-022, and the container by DEC-017/DEC-028) is
  `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING`, the verifier
  reading in W2 RESOLUTIONS.

## Convention friction

- The F4 scan matches finding-code names such as `MISSING_REDISTRIBUTION_STATUS`.
  Three rows carry `GAP_WORDING_CHECKED` for that reason only.
- CP-09 was applied to VER-001 and the output-matrix OUT-001. W2 left
  VER-001's reading contested (W2 RESOLUTIONS DEL-04-06). The purpose-section
  OUT-001 was judged on its substance.
- CP-04 does not apply to the keyed surfaces. They name no former-name
  identifier; OPS-K identifiers are current contract IDs. The crate package name
  `open_pipe_stress_rule_pack_lifecycle` carries the former name, but no
  DEL-06-04 keyed surface names it (the `_REFERENCES.md` mention is unkeyed).
  R3 may cluster it with other active-identifier residue.

## UNKNOWN rows

None.

## Reverse pass

- CLAIMED_BY: RC-06-0207 (lifecycle crate) and RC-06-0065 (JCS checksum
  computation). PARTIAL: RC-06-0182 (document validation) and RC-06-0041
  (checksum-binding fixtures). COVERS and CONSTRAINS answers give specific
  reasons. All 28 path-overlap rows carry capability-specific reasons (F5).
- The reverse pass did not change my view of anything sealed. Two items
  reinforce or add nuance:
  - RC-06-0018's routing note (report packages refuse rule-check binding)
    reinforces FG-01.
  - RC-06-0122's note says the canonical_json form keeps integers beyond 2^53
    exact, a documented divergence from strict JCS. R-06-04-004 says
    "JCS-compatible". I left it `ALIGNED`, because rule-pack payloads are not
    known to carry such integers. The verifier may weigh this.

## Batch consistency

`--batch` over the DEL-06-04 and DEL-06-05 forward ledgers: **PASS, 0
findings**. DEL-06-05's CP-03 rows record a `CANONICAL_DEPARTURE:` explaining
why they differ from this ledger's CP-03 rows (pins versus no pins).

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): Piping
selects work through owner-steered work graphs, not `## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
