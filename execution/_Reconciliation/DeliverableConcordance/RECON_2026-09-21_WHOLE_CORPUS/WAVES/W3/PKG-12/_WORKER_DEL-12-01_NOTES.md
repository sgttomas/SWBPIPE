# Worker G1 carry-forward notebook — W3 PKG-12 (DEL-12-01, DEL-12-02, DEL-12-03)

Recurring situations and the one treatment used for each across the three ledgers.

- **SOW surface.** CP-04 rename residue (OpenPipeStress) recorded once on `SOW`; items judged on substance. Frontmatter pin `4d153302` (rev 0.8) noted there, not a separate cause.
- **_STATUS / _CONTEXT surface rows.** Claim scoped to the file header (state, Last Updated / identity). Header accurate → ALIGNED; Last Updated older than own history → CP-05 (DEL-12-03 only).
- **PDU-055 current declarations** (12-01 STATUS block; 12-02 CLM-010/017/025; 12-03 CLM-002): CP-02 treatment STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · RECORD; CP-03 noted in Notes.
- **INIT.md pointer** (setup-era, absent in the Piping tree): STALE_SETUP_SPECIFICATION · BASIS_POINTER_STALE with CanonicalSituation CP-02 and a CANONICAL_DEPARTURE note (F3 origin class). Where the same block also pins a revision (12-03 CLM-008), the pin rule wins: STALE_REVIEW_OR_EVIDENCE.
- **Four-document residue** (CP-01): origin by `git log -S` on the PKG-12 folder: 7bee9ae41 text → STALE_SETUP_SPECIFICATION; text first present 2026-06-07 (586d3f10d) → STALE_REVIEW_OR_EVIDENCE. Cause REPRESENTATION_MIGRATED.
- **Mixed blocks without .rNN** split into .sNN (four-doc part vs the rest); mixed .rNN tables split all-or-none.
- **CP-09** on VER-001 and output-matrix OUT-001 (PASS parity, none matches frozen SOW). Purpose-section OUT-001 judged on substance (contract exists).
- **AC-001**: judged on content. It enshrines overtaken TBDs in 12-01 and 12-02 (STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE); aligned in 12-03.
- **Stale "TBD" for things a ruling settled** (container DEC-028, migration DEC-019): STALE_* · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NO.
- **OS roots / app data directory** settled in code (`lib.rs::app_store_path`) while SPEC §4.4 says TBD and no ruling found: CP-10 (IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · OWNER). Same finding in 12-01 FG-01 and 12-02 REXC-OI-010.
- **Text behind landed code** (runtime store; R15/TP-E4 redaction routes): STALE_* · DOC_BEHIND_CODE on declarations. Requirement rows judged on subject (C6(b)); stale verification-column status noted in Notes.
- **No-bypass requirements whose plugin part holds only by absence** (CP-11): PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT (IP_DATA for data boundary, SECURITY for telemetry) · OWNER.
- **RF-001/RF-002 Remaining items**: DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · LOCAL_DESIGN · LIFECYCLE · REVIEW (no governing row carries them, F2).
- **CONTEXT arch-basis**: CS-04 pin + .s01 (PKG-00 SEMANTIC_READY → STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING) + .s02 (Still TBD list overtaken → STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING).
- **Reverse**: COVERS for code that applies this deliverable's control but is owned elsewhere; adapter framework → COVERS (R0 used PARTIAL before COVERS existed).
