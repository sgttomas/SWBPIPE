# RV40 — independent W1 limits review

**PASS as prospective decision preparation.** No blocking or SHOULD-FIX defect was found in I29's arithmetic or current-source account. Option B's **20,000,000,000 LME per case / 60,000,000,000 LME per actual invocation is ready for ROOT selection only after K6c merges**. The **4,026,531,840-byte (3.75-GiB) memory figure remains a revisable technical allowance**, with the concrete downstream disposition below; it is neither a present guard nor a deployment commitment.

Reviewed I29 `8fada07f2c975cd20e52856a3d4fb589b05767e5`, brief/NUM basis `70b1a0b150ae803445b5f0fa65c495378180629b`, and frozen K6C `9994462204231fb92073e17eb09775756bec22f3` (maintained source `81c03849033f3ce745668f581f446530789397b8`). K6c gates/merge remain ROOT's work. T3/R abbreviations follow the brief; source paths below are relative to K6C `projects/chirality-piping`.

## Independently checked warrants

The one-off standard-library [check](_run_records/arithmetic_check.py) extracts the four raw H archives and ten current VR record files: **66 ordinary H processes, 330 full repeats, 33 models, 700 H attempt-charge closures; 201 current VR cases and 390 attempt-charge closures**. VR gives **191 Selected, two Ceiling and eight geometric refusals with zero LME**. Every checked proposal process/group/option field matches; input bytes match frozen Git objects. See [arithmetic](_run_records/ARITHMETIC.json) and [origins](_run_records/ARITHMETIC_ORIGINS.json).

Read-only comparison with original VR objects at `37bff17808` confirms that A1 increases **all 191 Selected charges by 361,021–45,515,510 LME**, preserving outcome strings. Current totals include certificate work. No historical baseline was regenerated; no solver ran.

| Option | Maximum basis case, LME | Separate-call arithmetic sum, LME | Rounded case / invocation allocation |
|---|---:|---:|---:|
| A: six T3 models | 1,689,942,466 | 8,832,968,455 | 2 Bn / 10 Bn |
| B: four Selected T4 models | 16,695,322,954 | 59,054,639,393 | 20 Bn / 60 Bn |
| C: all six T4 models | 74,572,170,690 | 199,220,629,053 | 80 Bn / 200 Bn |

A contains all T1–T3 and current VR charges, but none of the six complete T4 paths. B contains all 31 Selected H model paths and current VR charges; T4 TREE costs **65,593,818,970 / 74,572,170,690 LME** and ends Ceiling. B would stop those paths on work earlier. C funds full Ceiling attempts without adding a Selected case. These are finite work-only counterfactuals; no proposed threshold was executed.

B's headroom is **3,304,677,046 LME** above the largest Selected case and **945,360,607 LME** above its four-model sum. This is allocation rounding, not uncertainty. The sum is not a measured multi-case invocation; combinations, additional cases and new builds spend additional work, while actual cache reuse can reduce shared charges. No case-count, throughput or simultaneous-storage guarantee follows.

The H table's time/work/heap/footprint/RSS ranges and VR family counts/ranges agree with raw extraction. TREE's 512/1024 own/shared work, pattern/profile counts and 8/16 limbs agree. There are **111 prefixes** (three per first-pass Selected model, nine per TREE). The cited **2,313,466,291 → 2,313,495,031 LME** prefix overshoots by **28,740 LME**. T4's largest source/solve/outer-prefix requested/moving observation is **3,030,889,709 B**. Arithmetic matching 300 raw stage fields to accepted RV34 caller expressions reproduces **469,832 B** minimum slack ([auxiliary checks](_run_records/AUXILIARY_CHECKS.json)); this does not re-prove the accepted expressions.

Sparse outcomes independently remain T2/T3 each **8 Sensitive / 4 Passed** processes and T4 **8 NumericallyUnresolved / 4 Sensitive**. Process `ok` is not a numerical pass. All eight RSS misses remain, each observed value matched to its raw process record:

| Run (RF-LARGE omitted) | Observed RSS B | Projected B |
|---|---:|---:|
| 248 CHAIN-n10000-AX sparse | 746,291,200 | 601,950,092 |
| 250 CHAIN-n10000-AX sparse | 746,373,120 | 601,950,092 |
| 255 TREE-n10000-AX w1a | 3,891,986,432 | 1,314,759,989 |
| 257 TREE-n10000-AX w1a | 3,890,855,936 | 1,314,759,771 |
| 259 TREE-n10000-ROT sparse | 700,366,848 | 649,518,420 |
| 260 TREE-n10000-ROT w1a | 3,723,083,776 | 1,317,384,262 |
| 261 TREE-n10000-ROT sparse | 700,366,848 | 649,518,420 |
| 262 TREE-n10000-ROT w1a | 3,722,526,720 | 1,317,384,044 |

The largest ratio rounds to **2.960226**; neither it nor 3 is a future upper bound. The 8-GiB RSS and 7.5-GiB requested-allocation caps remain observation controls. Precision-isolated/internal-phase heap peaks remain unavailable. T1's missed live checks, T2's correction, accepted artifact/source qualifications and historical VR launch limits remain; this review does not requalify those runs or repeat comparisons.

## Source semantics to retain

- **Charging:** `frame_kernel/src/structural/retained/adaptive.rs:3647–3743,3836–3892` charges each case full own/shared/verification-shared work, including reused shared work and spent failures. The invocation pays each incurred new build once plus own work. Non-budget failed builds retain their spent work in cache; budget failures do not (`:3571–3638,3780–3833`).
- **Actual cache scope:** `solve_cases` groups by full stiffness encoding with a local cache (`:4342–4398`). `solve_case` starts a group of one; retaining a meter does not retain factors. `combine.rs:79–120` checks stiffness/layout/stations/supports, constructs its exact ledger, merges available operand caches and runs its own schedule. “Shared once” means once per incurred build, not a global identity cache across all calls/combinations. F2a must use one meter for the actual request including combinations.
- **Thresholds:** `adaptive.rs:93–112,224–280` saturates charges; checkpoints test `used > room`, Case wins when both fail, and invocation exhaustion is `charged >= limit`. Later `solve_cases` cases are withheld once exhausted. Preserve actual spent work rather than clipping it. No maximum overshoot, time or RSS bound is established.
- **Outside LME:** source construction/validation, geometry, graph/order/free blocks, encodings and case/combination ledger/layout preparation precede guarded work (`adaptive.rs:894–950,4310–4395`, `combine.rs:89–118`). Ordinary binary64/caller work is additional. Zero LME is not zero CPU or allocation.
- **A1:** `adaptive.rs:2766–2804,3304–3375,4069–4108` meters new Wide/Sum certificate work, including failures, into candidate `stop_rule`, case and invocation totals. Legacy exact prescriptions remain unmetered; finalization, serialization and future facade unit/derived certification are not thereby covered.

## RV40-N1 — concrete memory adoption boundary

**Required downstream design/ruling detail; not a defect in this explicitly conditional proposal.** I29 RETURN:9 and EVIDENCE_OPTIONS:17 require “explicit resource refusal.” An enacted rule must name its scope and product outcome.

**3.75 GiB is supportable only as a provisional technical target:** exactly half H's 7.5-GiB cap. Current full conditional H estimates peak at **348,410,932 B** for T3, **3,266,423,037 B** for Selected T4 and **3,276,270,506 B** for all T4. Thus 512 MiB is the next power of two above T3. B leaves **760,108,803 B** above the largest Selected H estimate, but that margin proves no F2a fit.

Before enforcing a memory rule, the owning F2a design/ruling must:

1. **Specify W1 admission/refusal semantics.** Accepted `T3/DESIGN_NUMERICS/DESIGN.md:511–522,573` preserves ordinary outcome and standing when W1 cannot select. Name case/invocation handling, combinations, partial results, reason/diagnostic and release/recovery behavior. Blocking an otherwise publishable ordinary result would need its own product-semantics disposition; Lc/Li selection does not authorize it.
2. **Establish the complete composition before affected allocations.** Include source capture/construction and count/graph preparation, resident ordinary inputs/results, all groups/caches and shared ownership, failed/active attempts, retained states and operands, certificate/publication and caller/receipt/output lifetimes. Cover requested and moving models, invalid/missing proof and overflow, including the allocations needed to acquire admission counts. A post-allocation check is insufficient.
3. **Qualify actual source/build/caller premises.** The H/VR descriptor covers single-case adapters, no aggregate support groups and zero nonzero-prescription terms (`performance_harness/src/k6/w1/envelope.rs:1–11,70–86`). F2a needs its own populations, capacities, compiler/target facts, input identities and state lifecycles. A sum or maximum of H cases is not a complete multi-case proof.
4. **Supply a production mechanism.** The kernel accepts only CaseLimit/InvocationMeter. H's allocator is binary-specific, caps requested CURRENT and aborts on refusal; moving peak is observed. Its half-cap preflight and per-call meter are likewise harness mechanisms (`alloc.rs:1–18`, `main.rs:709–725`, `staged.rs:69–88`). None is inherited product enforcement.

ROOT may carry these as proposed F2a requirements. Successful V-P measurements must not be made a prerequisite to selecting the work limits or to F2a existing: amended Q5 orders **K6/V-K → ROOT limits → F2a → V-P confirmation/revision**. F2a still needs its actual producer/reader/publication and source/caller qualification. If this memory requirement is adopted for F2a, that slice must also qualify its guard before relying on it; a V-P policy revision requires its own ruling.

**Exactly what remains conditional:** adopting 4,026,531,840 B as the final qualified F2a request-window allowance, its supported-machine/platform/concurrency interpretation and refusal semantics, or revising it for the actual caller. ROOT's work allocation does not decide supported-machine policy or the owner's existing **6-GiB dense/observation ceilings**. Those separate guards (`product_physics/src/lib.rs:2879–2948,3001–3044`) are not W1 memory guards. PHYS-R4/availability, observation framing, KF3-B1's λ/design split and KF2 dense-screen choices remain separate. No adopted KF3 comparison or F17 decision is reopened.

## Return boundary

Direct native TASK under ROOT HELP_HUMAN Agent 0; no delegation. Receipt **2026-10-02 17:10:18 UTC**, new-check cutoff **17:45:18**, deadline **17:50:18**. Host permissions are unrestricted; the write fence is instructional. Only this review folder was written. No Git/index/API write, build/solver/model/comparator experiment, source edit, policy enactment or implementation occurred.

[Origins](_run_records/READ_ORIGINS.json), [session](_run_records/SESSION.json), [commands](_run_records/COMMANDS.md) and `SHA256SUMS` preserve the finite review. Completion and seal verification are returned to ROOT. Stop after this packet.
