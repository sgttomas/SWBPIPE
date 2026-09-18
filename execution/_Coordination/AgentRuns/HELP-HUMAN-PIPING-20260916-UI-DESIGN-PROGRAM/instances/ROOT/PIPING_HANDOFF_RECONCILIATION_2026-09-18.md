# Piping handoff reconciled — the baseline report, the picking repair and the fresh demonstration as accepted context

Status: ROOT record, 2026-09-18, by the successor ROOT ([activation](SUCCESSOR_ACTIVATION_2026-09-18.md)). It reconciles the piping session's completed handoff into this program's working state and dependency graph. It is a reading of the piping session's published records, not an edit of them, and it accepts nothing on the piping session's behalf. Form: the fact, its owning record with hash, and the consequence for this program.

## 1. What landed, and where it is owned

All three are ancestors of `origin/main` at `451c5f595e0488a0d6064d9b0f972c5e8fc1d09d`, verified at activation.

| PR | Merge | What it published | Owning record (relative to `../../../`, the AgentRuns directory) | SHA-256 at `451c5f595` |
|---|---|---|---|---|
| #793 | `1f977a9352ddd370ed54b8cb363b2a2777f56213` | The original D-70 baseline characterization and its limitations | `HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md` | `56aa2f08de5ed0b5935d507c22d34d6aa22cda22d24e6b7227cadb4f64caffb7` |
| | | Its technical acceptance and redesign handoff | `…/D70_BASELINE/FINAL_ACCEPTANCE_AND_HANDOFF.md` | `aa68529b9141bb771fc413671d603ddb20ae31cc8bccba633e45f0ab73f596f9` |
| #794 | `362dcffc0f66d52c58689a58f268891461db0346` | The bounded shared-endpoint picking repair (stable shared sphere and capsule closest-point residuals; grouping, priority and tolerances preserved) | `HELP-HUMAN-PIPING-20260918-PICKING-STABILITY/HANDOFF_STATE.md` | — |
| #795 | `ea1de1a45f25e208f877813c5a88222cea59fedd` | The reviewed evidence of one fresh 10,000-pipe demonstration on the repaired product | `HELP-HUMAN-PIPING-20260918-PICKING-STABILITY/RUNTIME_REPORT.md` | `e8acb49bce189f15ddf6958e16ffdc34f8ddc502de51df2065f0502c18dc4a40` |
| | | Evidence custody and bindings | `…/_run_records/RUNTIME_EVIDENCE.json` | `ac2892d5f3b9f06a4f47db5a39390cf1fe6a7bd3f9ebc2440a338a5ab43e9ba7` |
| | | Verification and closeout | `…/_run_records/RUNTIME_CLOSEOUT.json` | `0226b6209c145f93078a85762e0b54a1bed65fd66609fdd34d62a18274f869fa` |

The raw evidence stays in its one canonical copy under the piping session's custody; this program refers to it by these hashes and copies none of it.

## 2. The two results, kept apart

**The original D-70 cohort (historical verdict, unchanged).** Product PR #789 at `8468a33c86adb622b25e98f98b0eaf28c7e9fa0e`. One 1,000-pipe run completed validly and met every original timing target, on the external LG display in its reported 60 Hz mode with an older method. None of the five allotted 10,000-pipe attempts completed validly: four stopped at point-selection 58, one was affected by owner-reported manual camera interaction; the attempt budget is exhausted. The cause of point 58 was reproduced as a numerical instability in product picking. The report closes as an incomplete attempt, not as performance acceptance.

**The successor demonstration (separate attribution).** Product at the PR #794 merge. One owner-requested 10,000-pipe run completed all 243 measurement segments (one assignment, 200 point selections, 20 box selections, 20 filters, two orbit modes), passed the formerly failing point 58 and the complete frozen point-selection population, and met every unchanged original timing target; zero owned pending viewport animation frames at the settled check; cleanup and source, build and profile binding checks passed.

| Metric | Recorded (ms) | Unchanged target (ms) |
|---|---:|---:|
| Assignment | 640.786 | 2000 |
| Point-selection p95 | 42.036 | 100 |
| Box-selection p95 | 44.296 | 200 |
| Filter p95 | 43.092 | 200 |
| Centreline orbit reported-presentation interval p95 | 8.335 | 16.7 |
| Actual OD orbit reported-presentation interval p95 | 8.335 | 33.3 |

Reference conditions: Apple M5 Max, 128 GiB; owner-approved internal 120 Hz display; pinned Chromium 153.0.8010.36; browser viewport 1440 × 920; canvas 794 × 557 CSS pixels, drawing buffer 1588 × 1114, device pixel ratio 2; Light appearance; Comfortable density; 280 px tree and 340 px inspector rails. Label populations at the recorded boundaries: assignment, labels on, one rendered; points, boxes and filters, labels off, none rendered; both orbits, labels on, two rendered at ready and three at stopped, under the unchanged budget of 80. Those are boundary observations, not continuous monitoring.

Limits this program carries with the numbers: it is one demonstration, not five-run qualification and not acceptance of the redesigned product. The 1,000-pipe and 10,000-pipe records differ in display, product and method, so no same-profile scaling comparison or speedup is stated. Main-frame trace spans are not complete frame cost and not hardware GPU execution time; Chromium presentation feedback is not physical scanout; no observer overhead was subtracted. Native startup, JavaScript heap and process RSS were not measured. Neither result rewrites the other's verdict.

## 3. Consequences for this program

1. **D-70's performance handoff condition is satisfied.** D-70 effect 6 made the baseline report, not a passing score, the condition for starting redesign implementation. The piping session's acceptance record states that the report supplies it, and its later runtime report states that redesign implementation need not wait on that session. Every line in this run that says the baseline tranche is still to come, or that implementation waits for it, is historical from here on; the working state strikes those lines and keeps them.
2. **That does not authorize implementation.** This program's assignment is design only. Implementation of the redesign needs its own readiness decision, the outstanding rulings and an owner authorization of the implementation tranche.
3. **Still open, carried forward:** the D-68 performance obligation, now against the redesigned product under criteria that the owner rules before use (proposal at [`ACCEPTANCE_CRITERIA_PROPOSAL_2026-09-18.md`](ACCEPTANCE_CRITERIA_PROPOSAL_2026-09-18.md)); the settled-frame and owned-resource obligations; the independent-usability holds PDU-045 and PDU-046.
4. **The demonstration is a comparison basis, not advance acceptance.** The redesigned canvas, geometry, labels and overlays change the workload.
5. **The picking repair and its maintained regression tests are a preserved foundation** for any implementation brief: pick tolerances and oracle expectations are not altered to obtain a benchmark pass.
6. **Observation work is separate and blocks nothing.** The piping session has read the rendering brief and its notice; no overlay or deformation observation run has been performed. The requested workloads are classified by RESEARCH-G, and a bounded observation brief for supported behaviour follows that return. The acceptance-criteria proposal no longer waits for an observation report; it identifies which of its portions need later observation evidence.
7. **The physics backlog** (pressure runtime, connector mechanics, sparse execution, export implementation) stays separately scoped and is not a prerequisite for interface design.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
