# SCA-007 Impact Assessment

Compiled by the parent from node A's read-only sweep (2026-07-16), reconciled
at fan-in against node C's `Amendment_Actions.csv`. Search families run by
node A: criterion text variants; v0.1 §22.6/§16.2/§16.5 tokens as current
policy; `docs/PRD.md`-as-yardstick pointers; v0.2-path pointers on live
surfaces; optional/future prover framings; verification of the D-47 packet §5
inventory. Full grep list in the node-A return (AgentRuns record).

## Live surfaces requiring edits (all covered by A001–A026)

- DEL-09-04 `_STATUS.md` (criterion item + storage-policy fragment) — A001–A003
- DEL-10-04 `_STATUS.md` ("PRD §22.6 deviation" token) — A005/A006
- DEL-17-05 `ScopeOfWork.md`/`_STATUS.md` (only deliverable with
  optional/future prover-posture framing) — A011/A012
- `loop/WORKPLAN_2026-07-10_piping_loop.md` lines 27–28, 141 — A013/A014
- `docs/PLAN.md` lines 32, 42–47, 52/57, 88, 104–105 (node A found 44/46 and
  105 beyond the packet's approximate rows; node C drafted them) — A015–A019
- `execution/_Coordination/_COORDINATION.md` stage record (v0.1 §22.6 +
  §16.2/§16.5 tokens) — A020
- D-46 packet (supersession note) — A021
- `execution/_ScopeChange/_LATEST.md` (stale at SCA-005) — A022
- `SOFTWARE_DECOMP.md` revision/currency tokens — A023/A024
- Relocation set (`docs/PRD.md`, `docs/_history/PRD_v0.1.md`, v0.2-path
  stub) — A025/A026 + the amendment locus itself (v0.3 criterion/§22.5/§22.1)

## Verified no-change (recorded)

- DEL-09-04 `ScopeOfWork.md` — no criterion citation exists (packet §5 row 3
  overstated) — A004
- DEL-10-05 `_STATUS.md` — method description, not a criterion citation — A007
- DEL-14-05, DEL-15-04, DEL-17-04 — only DEC-016 ingestion-exclusion and
  DEC-015 claim-authority boundaries, which DEC-080 leaves intact — A008–A010
- `docs/README.md` / root `README.md` — point at `docs/PRD.md` with no version
  tokens; become correct automatically after relocation
- `loop/LOOP_INIT.md`, `AGENTS.md`, `CONTRIBUTING.md`,
  `docs/BUILD_AND_RELEASE.md` — no criterion/yardstick hits
- `docs/user_guide/index.md` "optional user-owned CAEPIPE run evidence" —
  activation-optionality (license/executable ownership), which DEC-080
  preserves; not a posture demotion

## Fan-in dispositions of node-A flags

1. `docs/PLAN.md` line 20 ("before 2026-06-13 ... `PRD.md` §22") — dated
   historical statement, not current policy; LEAVE (citation-resolution note
   governs). Node A's LIVE-EDIT flag rejected at fan-in.
2. `docs/PLAN.md` lines 87/94/222 (§16.2/§16.5 in residual-work narrative and
   a dated correction block) — semantically current or dated history; LEAVE;
   the stage record (A020) now carries the authoritative v0.3 mapping.
3. **Named residual (out of SCA-007 scope, surfaced for future loop work):**
   `docs/validation_manual/index.md:62`, 
   `docs/validation_manual/cases/generate_validation_case_pages.py:4,219`,
   and one generated case page cite v0.1 "PRD section 16.2/16.5" numbering.
   Re-keying touches an executable generator (DEC-025 sweep territory) and
   regenerated pages; deferred as a candidate DEL-09-04 hardening item rather
   than silently expanded into this pre-accepted action set. Recorded in
   `Handoff_State.md`.
4. Superseded `loop/WORKPLAN_2026-07-04_piping_loop.md` yardstick lines —
   archived plan (current loop plan is 2026-07-10); LEAVE.
5. Ruled history (receipts, run records, ruled packets, §12 rows, `plans/**`,
   prior SCA snapshots, reconciliation snapshots) — LEAVE by design; governed
   by the v0.3 citation-resolution note and the D-21 Annex A crosswalk.

## Token mapping adopted (A020)

v0.1 §16.2 "Required Solver Benchmarks" → v0.3 §22.2 (identical heading;
§16.1–16.4 map 1:1 onto §22.1–22.4). v0.1 §16.5 "Validation Manual" has no
§22 successor — it maps to the §24 R6 validation-manual deliverable, whose
exit criterion DEC-080 amends.
