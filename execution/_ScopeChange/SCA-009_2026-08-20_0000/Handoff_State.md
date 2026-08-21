# Piping SCA-009 Handoff State

**Current state:** `GATE 4 APPLIED — GATE 5 VALIDATION AND CLOSURE PENDING`

**Current basis:** `main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb` (merge of
PR #591), with the SCA-009 gate commits accumulated on
`claude/piping-sca-009-gate2-20260820` (single landing PR #593 per the
owner's process direction; ruled-upon commits `f5112824f` and `d50e72c4b`
intact in history).

**Original durable basis (Gate-1 package):**
`7584de0a8d53d69a135c22fe39a78cb4a30b6cb2`; Gate-1 package merged as PR #592
(merge `01a8dd4c0aabd4fe1f71bba7201a4345f9e6cfdc`)

**Gate 1:** `CONFIRMED` per `ACCEPTANCE_RECORD.md` — D1 confirmed; D2
Option A (ADD `DEL-07-09`); D3 two-class vocabulary ratified; D4 single
palette owner; D5 fold R-005/R-006, DEL-16-04 generator out; D6 `SOW-077`.

**Gate 2:** `APPROVED` — on `Impact_Assessment.md`
`bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` at
commit `f5112824f`, with envelope-L override and the implementation-landing
column.

**Gate 3:** `APPROVED` — verbatim ruling on `Amendment_Preview.md`
`802c2ce92c5a48651f4d06312d4ba26593f0134e3b7c443988e74b79c0e170d4` at
commit `d50e72c4b`: apply exactly the six preimage→postimage pairs,
`DEC-094`, and no other surface. (Post-ruling whitespace-only preview
cleanup recorded in `ACCEPTANCE_RECORD.md`; post-cleanup preview SHA-256
`44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab`; the
six pair hashes and `postimages/` bytes unchanged.)

**Gate 4:** `APPLIED` — the six approved postimages were copied
byte-for-byte onto their live paths and every live-file SHA-256 was proven
equal to its approved postimage hash (proof table in `RUN_SUMMARY.md`). No
other surface was touched.

**Decomposition state:** the live
`execution/_Decomposition/SOFTWARE_DECOMP.md` on this branch is now
**revision 0.12** (with `SOW-077`, `DEL-07-09` at envelope L, the
OBJ-006/OBJ-015 and PKG-07 mappings, telemetry 77/102 and
`S=9, M=69, L=24, XL=0`, and `DEC-094`), while
`_Decomposition/_LATEST.md` still cites revision 0.11 and
`_ScopeChange/_LATEST.md` still points to SCA-008. **This divergence is
the expected mid-amendment state under the pointer-last rule**: pointers
advance only at Gate 5 after snapshot completion and validation. Until
then, consumers resolving through `_LATEST.md` continue to see the
accepted 0.11/SCA-008 state.

**DecompositionTruthState:** `AMENDED_PENDING_VALIDATION` (live working
surfaces carry the approved postimage bytes; acceptance as validated truth
and pointer advance are Gate 5)

**DerivativePackageState:** `STALE_PENDING_GATE5_RERUNS` (per
`Impact_Assessment.md` and `Amendment_Actions.csv`: AUDIT_DECOMP pre/post
recompute, DAG-008 rebuild, targeted RECONCILIATION refresh, DEL-07-09
dependency extraction, PREPARATION scaffold — none run yet)

**ContentRemediationState:** `NOT_APPLICABLE`

**DownstreamRerunState:** `NONE_RUN — OBLIGATIONS RECORDED`

**MetadataAlignmentState:** `NOT_APPLICABLE`

**AuditState:** `NOT_RUN` (pre-change baseline capture and post-change
comparison are Gate-5 entry work)

**ReadyForNextPhase:** `NO — GATE 5 REQUIRED` (snapshot completion:
Pre/Post_Change_Coverage, supersession artifacts, Propagation_Plan
consolidation; pre/post AUDIT_DECOMP; pointer advances pointer-last;
Piping dev-loop coordination notice; owner closure confirmation)

**ClosureVerdict:** `NOT_CLOSED — GATE 4 APPLIED ONLY`

**NextOwner:** `Ryan Tufts` (Gate-5 closure confirmation after validation
evidence is assembled)

**NextAction:** run Gate 5 — capture the pre-change audit baseline
recorded against the pre-application commit, complete the immutable
SCA-009 snapshot artifact set, run post-change AUDIT_DECOMP against the
applied state, advance `_Decomposition/_LATEST.md` and then
`_ScopeChange/_LATEST.md` pointer-last, route the Piping dev-loop
coordination notice, and present the closure state for the owner's
confirmation.

Applied-effect inventory (exactly six live surfaces, per the Gate-3
ruling): `SOFTWARE_DECOMP.md`, `ScopeLedger.csv`, `Deliverables.csv`,
`ContextBudgetQA.csv`, `DEL-07-03/_STATUS.md`, `DEL-07-03/_CONTEXT.md`.
Nothing else changed: no pointer, no other deliverable folder (all
DEL-16-04 surfaces untouched), no implementation tree, no PRD, no
historical snapshot, no dependency edge of any existing deliverable, no
lifecycle value (DEL-07-03 remains `IN_PROGRESS`). No estimate, schedule,
release, or professional-reliance effect. No professional-reliance,
certification, sealing, authentication, or code-compliance claim is
created. SCA-008's snapshot and all prior snapshots remain exactly as they
were.
