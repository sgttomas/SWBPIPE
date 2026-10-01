# RV27 — frozen resumption records review

**Verdict: NOT YET SUITABLE FOR MERGE — one SHOULD-FIX records finding.**
No source defect or new numerical finding is asserted. The preservation and
attribution work is suitable for fan-in after the status correction below and
its same-reviewer backcheck. All other required exact-final-head gates remain.

Reviewed the complete 322-path diff from
`d01ad98a754698631f927709d08284c272de85e8` to
`90b6bcbbf64b13975211038bf3f33bb87273e646` on
`codex/piping-t3-resume-records-20260930`, PR
https://github.com/sgttomas/chirality/pull/1068. The candidate checkout was clean
before and after checks. Carried `cd7e5f629f8e3f61aaea79b0216a451888e6bc9e`
PR1063 records are included in this review, not treated as already covered.

Aliases: P = projects/chirality-piping; T3 =
P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3;
R = T3/RESUME_2026-09-30; packet = R/_run_records/aud_t3_04.

## Actionable finding

**RV27-S1 — SHOULD-FIX (P2), merge-blocking under this run's review rule:**
Reconcile the latest current dispatch status with the included G0 grant.

- Location: `P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md:35`, the newly added “Current” status; related initial-dispatch account at `T3/ROOT_RULINGS_V1.md:3182-3187` and `R/DISPATCH_UPDATES.jsonl`.
- Evidence: the current graph says “Source-only I22/I21 ... delegation is running,” while `R/BRIEFS/I22_G0_BUILD.md:8-9` already authorizes scratch preparation and exactly one offline locked Rust build. The ruling describes the initial source-only wave but supplies no later G0 status pointer.
- Trigger and impact: a continuation reader relying on the graph/current dispatch record receives an out-of-date authority boundary and cannot tell that the included G0 grant superseded I22's initial source-only restriction. This is a records consistency defect, not evidence of an unauthorized solver run or numerical closure.
- Remedy: preserve the historical dispatch/ruling bytes, append the later G0 authorization and its build-only limit, and update the current graph wording accordingly. Distinguish the grant from actual execution and from solver-case authorization. Do not import later G1/B activity from outside this frozen slice. Return the repaired candidate for RV27's bounded delta check.

## Independent checks and conclusions

- Scope checker PASS: all 322 paths are the declared records/evidence surfaces;
  no solver, harness, validation, instruction, workflow or host-tool change.
  Rulings and RV26's confirmation are append-only. The entire prior audit is
  unchanged. Numerical source matches the #1064 merge.
- Preservation seal `729d342640301294dfd31e02659a0a443231f627858aca85c57ee5b7b8192b97`
  verifies all 293 entries, with no missing or unlisted payload. Every relocated
  byte matches the retained original return packet, including its seal.
  EVIDENCE_RELOCATION.json correctly supplies the old/new prefix map.
- All 268 original sweep files and their local raw copies match the original
  hashes and sizes. All 268 committed sanitized copies independently reproduce
  the declared path/ANSI transformation. All 238 per-manifest logs reproduce
  their summary counts. Six clean final-head sweep identities match their merge
  records and original SWEEP hashes; all three historical Mac failed-test names
  remain visible in every slice. These are historical records, not fresh passes.
- Both committed gzip files stream-restore to 884 JSON objects and their original
  byte counts/hashes, matching scratch originals and the pre-existing KF2 hash
  record: base `c42981e541650578a60efe48cbf346360fedbb4507b3d02365b5f5abb78e9f68`;
  candidate `11dfe8296681280e0fd4163ee3195cc16ff0f78f3ce81dff82516a520bf2ef3f`.
  The two files are below 100 MB; the size warnings/retention limits are disclosed.
- All 90 historical manifests / 9,088 entries and the audit's 14 entries verify.
  The five exceptional parent-directory checksum bases work as documented.
  All 61 principal inputs match the claimed `d01ad98a...` replay basis; the current
  candidate differs only in its intentionally updated ruling/graph. The script
  matches its `7fd632f...` source/hash. Seven verification outputs exactly match
  their sealed historical counterparts. Current candidate state is not silently
  relabeled as the old basis.
- The exception ledger matches the named S11K, K2B, K6 and K6B return passages
  and the S11K fix brief: three index pairs, the fetch, and an expressly permitted
  fast-forward remain separate. Nil net effect is not promoted to no-write history.
- Live GitHub/remote reads confirm PR1066 CLOSED/unmerged at the retained
  `520d7df...` branch head, and PR1068 at the reviewed full candidate/base.
  PR1063's head/merge/time and 7 successful / 6 skipped checks match the carried
  record. Neither this review nor those records imply personal owner review.
- A1 remains open SHOULD-FIX with escalation on an unmutated false publication;
  F2a reliance remains held. Independent exact-fraction arithmetic reproduces
  the abstract 20% scale loss / 9/8 interval defect and zero-scale boundary,
  without asserting source reachability. A2 stays conditional. K6c's unfinished
  source packet is input, not accepted E_max. No new bound, physical cutoff,
  D2 contract, availability change, design acceptance, release, or guard-tool
  development is selected here.
- The owner receives explicit credit for catching/stopping tool drift. Native
  parentage, prompt-only write fences, source-only initial launches, the exposed
  oracle's provenance-only return and its fresh replacement are disclosed.
  The old B02 outputs remain honest forensic results with failed guard monitoring;
  no numerical conclusion is promoted from them or from G0 diagnostics.
- Fresh GEN-8 on the clean frozen candidate: **1 passed, 10 deselected, 32.30 s**.
  Bytecode and pytest cache were disabled. The relocated evidence avoids the
  active-surface classification without changing a checker or sealed bytes.

## Execution limits and return

RV27 is a fresh TASK reviewer reporting directly to ROOT and authored none of
this slice. Used the software-code-review skill; actual origins/hashes are in
CONTEXT.json. Only TASK's full role body was activated. No delegation, Rust,
solver, DEC-025, Git/index mutation, source repair or host-tool work was performed.
All Git reads set GIT_OPTIONAL_LOCKS=0. Writes stayed within the assigned review
subtree and owned scratch. Shared-host prompt scopes are not OS isolation.

No fresh DEC-025, T9, both-entry gate, or final hosted-CI verdict is supplied by
this records review. ROOT reported CI and practitioner checks in flight and
DEC-025 not yet begun. Their final-head requirements remain outstanding at this
return. Later candidate changes require review coverage. Historical passes,
archive integrity and initial diagnostics do not close numerical work.

Original scratch remains required for byte-exact unsanitized suite custody and
uncopied full/envelope directories. The dated-generator/sparse-checkout issue
remains open. No pruning is authorized. See COMMANDS.md and evidence/ for the
executed checks and outputs. Review seal covers this completed review return;
a later correction check must be additive.
