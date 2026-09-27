# T3 V1 — narrow check of D1's S11 containment (C3-full: S11-K and S11-F)

Type 2 TASK V1, same reviewer as `REVIEW/RETURN.md`, 2026-09-26, at the manager's request, which ROOT authorized (`ROOT_RULINGS_V1.md`, "S11 containment (ROOT, 2026-09-26, after `70f56b83e`)").

- **Scope.** Read-only. I ran standard-library Python probes only: no cargo, no Git write, no product edit. T1 was read only through `git show f3270ea79:` and `git grep f3270ea79`.
- **Inputs, on the T3 branch at `70f56b83e`, which is byte-identical for these files to the head `12f2122cd`:**
  - `T3/DESIGN_NUMERICS/S11_CONTAINMENT.md` (sha256 `616214b25c10c209…`, verified);
  - its `_run_records/probe_s11_audit.*` and `scan_load_fold.*` (SHA256SUMS verified);
  - `T3/MANAGER_NOTES/S11_MAP.md`;
  - `T3/ROOT_RULINGS_V1.md`, including ROOT's fail-closed constraint and the pre-acceptance;
  - my own finding V1-S11.
- **Line numbers** refer to `c61a540ea` for main and `f3270ea79` for T1. `PP`, `FK` and `SA` are as in the designs.

## 1. Verdict: **BLOCKING**

C3-full repairs the solve's force vector, and it adds no invocation-level `Err` if it is implemented at every site (Q3). But it leaves a live path open. The same cancelled inputs that S11 targets still produce silently wrong **published member actions**, because recovery re-folds the element loads in binary64, outside the ledger and outside the M03 audit.

- **What makes it worse under C3-full.** Once the solve is repaired, the audit passes. The recovery error is then fully silent, even though C3-detect would have flagged the same case as Sensitive.
- **The fix.** It is local, and can mostly land T1-disjoint.

## 2. ROOT's questions

### Q1. Can C3-full publish a silently wrong value, including through a producer that bypasses the ledger? Is the M03 audit a sufficient guard?

**Yes, it can (S11-V1, BLOCKING). The audit is not a sufficient guard.**

- **Recovery-side folds.** C3-full covers force-vector producers only. Published member quantities are recomputed from the element's loads, and those loads are folded in binary64:
  - element end forces are `K_e·u − equivalent`, where `equivalent = pipe.equivalent_nodal_loads_with_spans(&straight_loads, …)` (`PP:2327-2341`, published at `:2389`). That method adds every load's fixed-end terms into one `[f64; 12]` with `+=` (`P/core/solver/straight_pipe/src/lib.rs:568-585`);
  - station resultants use `straight_section_resultants` → `station_resultants_from_i_end_with_spans` (`PP:2430`, `:2496`, `:7490-7519`; `straight_pipe/src/lib.rs:934`);
  - the circular-maximum spans sum `w[axis] += load.force_per_length` (`PP:7551-7562`, via `exact_straight_summary_extrema` at `:2552`);
  - curved-bend intensities sum `… [dof] += load.magnitude.value` (`PP:8250-8252`), which feeds `curved_bend_section_resultants`.
- **Demonstration.** Probe A (`_run_records/s11/probe_s11_check.*`, using D1's binary64 element unchanged):
  - One 2 m cantilever element carries three uniform local-y loads (G, 0.3, −G) N/m. The force is C3-full's correctly rounded net, so the displacement is exact.
  - The M03 audit with the exact ledger gives ratio 0 and does not flag.
  - The published root end shear is still wrong by 1.24e-9 at G = 1e7 and 4.97e-9 at G = 1e8. The root moment is wrong by 2.48e-9 at 1e8.
  - These errors are relative to the member's own scale, which is the body scale here, because the net load is the only load.
  - On main the same case is already wrong at the solve as well. C3-full fixes the displacement but not the published actions.
- **Is the audit a sufficient guard?**
  - **For a producer left entirely outside the ledger:** yes, grossly. The ledger then lacks that producer's whole contribution, so the residual flags it.
  - **For published quantities computed after the solve:** no. The audit sees the solve only.
  - **For producers that pre-sum internally before pushing one value:** no. An example is the curved thermal equivalent `value += K_macro·u_free` (`PP:8086-8090`), a 12-term binary64 dot product. The design does not define the ledger's contribution granularity (S11-V3).
  - **On the guard path:** only up to the row-scale floor (Q2).
- **T1 disjointness.** `straight_pipe` and `primitive_loads` are not changed by T1 (`git diff c61a540ea f3270ea79 -- core/solver core/loads` is empty).

### Q2. Does the row-scale screen catch every absorption that breaches 1e-9 on published quantities?

**No.** I constructed misses (probe B). They matter only on the guard path, because under C3-full the ledger repairs every ledger-routed load.

| Case (guard path: the load is still folded) | Screen | Worst ratio / target | Published error, relative to its own magnitude |
|---|---|---|---|
| Tip UY (1e7, 0.3, −1e7) N + tip RZ 1e3 N·m | flags | 2.9 | root reaction Fy 2.5e-9 |
| Tip UY (1e7, 0.3, −1e7) N + tip RZ **1e6** N·m | **passes** | 0.0029 | root reaction Fy **2.5e-9**; member shear the same |
| Tip UY (1e8, 0.3, −1e8) N + tip RZ **1e6** N·m | **passes** | 0.012 | root reaction Fy **9.9e-9** |
| **Restrained** root UY (1e7 / 1e8, 0.3, −G) + tip RZ 1e6 | **passes** | 0.0017 / 0.0070 | root reaction Fy **2.5e-9 / 9.9e-9** |

- **Why it misses.** A large moment at the same node makes `Σ|K_ij||u_j|` on the UY row about 12M/L = 6e6. The fold error of about 3e-9 N is then roughly 1e-16 of the row scale. The published quantities it corrupts (the root shear reaction of 0.3 N, and the member shear) carry only the small net.
- **Tip displacement stays accurate:** its relative error is 1e-15 or less. The screen protects displacements, and quantities whose own scale is comparable to the row scale. It does not protect reactions or member invariants whose own magnitude is far below the row scale.
- **Against D1's body-coupled scale** (force S\* ≈ S(moment)/L = 5e5 N) the same errors are about 6e-15, so whether this counts as a miss depends on the scale convention. This is the same floor question as V1-S8, which ROOT ruled must be proved or stated and enforced. S11-V2 asks for it to be stated here too, and for bypass to be made impossible by construction, so that the guard never has to carry 1e-9.

### Q3. Do the source_recovery fold check, exact_boundary coverage and receipt replay stay bit-consistent, on main and on T1? Can a case be knocked out or reach the composite `Err`?

**Consistent by construction, and no new `Err`, provided every site moves together and all sites use one correctly rounded function. The design's write set names main's lines only, and misses T1's extra sites (S11-V4).**

- **Exact sums are order-independent, and correct rounding is unique.** The facade ledger and source_recovery's term sum therefore agree bit for bit when (a) both contain the same term set and (b) both round the same way.
  - **(a) holds within retained-source scope.** On main, only nodal terms are admitted (`source_recovery.rs:554-589`), and other producers are refused by scope. On T1, nodal terms plus the eigen axial pairs are admitted (`source_recovery.rs:1270-1287` at T1). The ledger for an in-scope case therefore holds exactly those terms.
  - **(b) needs care.** The existing `Expansion::rounded()` is a naive ascending sum (`FK/structural.rs:366-368`) and is **not** correctly rounded. In probe D, the expansion {2^-110, 2^-53, 1.0} gives 1.0 by the naive sum, 1.0 by the binary64 fold, and 1.0000000000000002 correctly rounded. D1's new `exact_rounded_sum` must be the only function used at every site, including the exact context's new acceptance clause (`exact_boundary.rs:387-392`), which today compares only against the naive projection or the ordered fold.
- **Sites that must all change together** (T1 numbering; D1 cites main's):
  1. The live fold check: `source_recovery.rs:609-667`, which includes `close_load_state`'s eigen fold at `:1270-1274` ("Mirror `add_thermal_equivalent_loads` exactly, after the nodal fold"). D1 names only main's `:580-595`.
  2. The 0.4.0 receipt replay: `source_receipt.rs:218-219` (`global_load_vector` + `add_thermal_equivalent_loads`). **Not named by D1.**
  3. The pre-0.4 receipt replay: `source_receipt.rs:320` (main `:154`). Named.
  4. The exact context: `exact_boundary.rs:387-392`. Named.
- **What a missed site does:**
  - If (1) keeps the eigen fold, 0.4.0 cases with eigen and nodal terms on a shared DOF lose selection per case. That is declared (`SOURCE_BLOCK_RECOVERY_UNAVAILABLE`), but it is a regression.
  - If (2) is missed, joined finalization fails and T1's SF-1 republishes ordinarily. That is declared and not an `Err`, but it knocks out a correct result.
  - If (3) is missed, pre-0.4 finalization fails and reaches `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (`PP:1249-1253`). **That is the forbidden outcome.** D1 does name this site.
- **Signed zero.** A net of exactly zero must be +0.0 at every site. `Expansion::add(-0.0)` stores no term (`FK/structural.rs:334-336`), and the fold gives +0.0. The new function must match, because the checks compare `to_bits`.
- **Tests.** D1's test F4 (a pre-0.4 retained case with (1e8, 0.3, −1e8)) exercises sites 1, 3 and 4. It needs a 0.4.0 twin: eigen pair plus nodal load at a shared node, a selected join, and successful finalization.

### Q4. Are reactions on restrained DOFs and the nonlinear loop covered?

- **Restrained reactions.** They are covered for ledger-routed loads: the force at `PP:2135-2139` becomes the correctly rounded net. The guard on restrained rows has the floor of Q2: the restrained-row misses in probe B pass the screen.
- **The nonlinear loop.**
  - Its base force is repaired, because it consumes the same vector.
  - Applied sliding-friction forces are then added in binary64 (`nonlinear_integration/src/lib.rs:1642-1646`) and added to reported reactions (`:1650-1659`). That is one extra term per DOF, a bounded double rounding, not an absorption.
  - There is no load guard inside the loop until T5 (D1 §3.3). That is acceptable if it is recorded as an open item with T5 (S11-V5).

### Q5. Is "three or more contributions" a real narrowing? Does the no-interim-measure ruling still hold?

- **The narrowing is arithmetically correct, but it is not a narrowing for realistic models.** Three or more contributions on one DOF is normal wherever a node joins two loaded spans and carries a third term. Examples:
  - weight equivalents from two adjacent spans plus a nodal load, a constant-effort hanger force (`PP:9808`) or a thermal axial pair;
  - three uniform loads (pipe, contents, insulation) on one element, which put three terms on each end node from that element alone, and six at an interior node;
  - interior nodes of any hot straight run, which carry two eigen or thermal pair terms per axis (`PP:8000-8001`; T1's eigen equivalents), before any nodal load.
- **The committed JSON is not representative.** My broader structural scan counts nodal, element, thermal and constant-effort producers (`_run_records/s11/scan_shared_node_contributions.*`). It finds, in 156 models (89 unique), 173 cases and 1,371 loaded DOFs, at most **two** contributions per DOF. So the scan's claim "nothing committed has that" holds for all producers, but the committed models are small invented fixtures.
- **The real limiter is the gross-to-net ratio, and also the order.** Probe C:
  - The error only arises when the small term is added **before** the cancelling pair and has bits below ulp(gross).
  - Integer-valued small terms under 2^53, and orders where the cancellation happens first (Sterbenz), are exact. So weight-then-hanger patterns are exact even at a ratio of 2e8.
  - A realistic breach does exist. A large-bore hot line has E·A·α·ΔT = 2e11 · 0.057 · 1.2e-5 · 300 = 4.1e7 N. Its thermal pair at an interior node of a colinear run, after a 1.3 N co-axial nodal load, gives a net error of 2.3e-9 (8.1e-10 for 3.7 N). The absolute error is about 3e-9 N.
- **Does the ruling still hold?**
  - It holds in substance. Every realistic error is at most about ulp(gross)/2 per absorbed term, which is negligible in engineering terms. A 1e-9 breach needs a physically negligible net that dominates some published quantity, for example the member forces of a run free to expand thermally. Losing the whole response needs a ratio of 2^53 or more.
  - Its stated rationale should be corrected (S11-V6). "Three or more contributions" is common, not a filter. The operative conditions are the ratio (about 1e7 or more), the production order, and the scale convention. Such ratios are reachable with realistic large-bore thermal loads.
  - The reopen condition (T1's merge slipping) remains appropriate.

## 3. Findings

| ID | Severity | Where | Evidence | Consequence | Required change |
|---|---|---|---|---|---|
| **S11-V1** | **BLOCKING** | S11_CONTAINMENT §1 item 3, §3.3 ("every force contribution goes into one per-case exact ledger"; the audit "is the guard") | Probe A: under C3-full the audit ratio is 0 and does not flag, yet published root shear is off by 1.24e-9 / 4.97e-9 and root moment by 2.48e-9 at G = 1e7 / 1e8, on the member's own (body) scale. Recovery folds: `PP:2327-2341` → `straight_pipe/src/lib.rs:568-585`; stations `PP:2430`, `:2496`, `:7490-7519` → `straight_pipe:934`; extrema `PP:7551-7562`; curved `PP:8250-8252` | Silently wrong member actions, station resultants and stress maxima are published as Passed and Current. Under C3-full the solve is repaired and the audit passes, so this is fully silent (C3-detect would at least demote the case) | Accumulate every recovery-side load sum exactly and round once, with the same `exact_rounded_sum` discipline: `equivalent_nodal_loads_with_spans`, `station_resultants_from_i_end_with_spans`, the extrema `w`, and the curved intensity. `straight_pipe` is T1-disjoint, so its part can join S11-K, with byte changes listed only for multi-load elements. Add probe A's case, and a station and extrema variant, to S11-K and S11-F as positive tests. Add a mutation that restores the binary64 fold in recovery, which must fail |
| S11-V2 | SHOULD-FIX | §5 (scale), §3.3 (guard) | Probe B: with a large moment at the same node, the screen passes (0.0029 to 0.012 of target) while the root reaction and member shear carry 2.5e-9 / 9.9e-9 relative to their own magnitude. The restrained-row variant is the same | On the guard path, 1e-9 is not guaranteed for published quantities whose magnitude is far below their row's scale | State the guard's floor explicitly, as ROOT ruled for V1-S8. Make ledger bypass impossible by construction: the case force vector is constructible only from the ledger, plus a test or CI check enumerating every `force[…] +=` site in `PP`. Then the guard never has to carry 1e-9 |
| S11-V3 | SHOULD-FIX | §3.3 | Pre-summing producers: the curved thermal equivalent `value += K_macro·u_free` (`PP:8086-8090`) pushes one pre-summed value. Element equivalents are per load (`:7789-7798`) | The contribution granularity is undefined, so internal cancellation inside a producer is invisible to both ledger and audit | Define the granularity per producer (individual terms or a declared formation step), and justify each formation step |
| S11-V4 | SHOULD-FIX | §3.5 write set (S11-F), §7 | T1 sites: `source_recovery.rs:609-667` including `:1270-1274`; `source_receipt.rs:218-219` (0.4.0 replay, not named) and `:320`. `Expansion::rounded()` is not correctly rounded (probe D) | A missed site knocks out correct retained recovery, or for pre-0.4 replay reaches the forbidden composite `Err` | List every T1 site in S11-F. Use one correctly rounded function everywhere, with a stated +0.0 zero convention. Add a 0.4.0 test (eigen pair plus nodal load at a shared node, selected join, successful finalization) beside F4 |
| S11-V5 | NOTE | §3.3 (nonlinear loop) | Friction forces added after the ledger (`nonlinear_integration/src/lib.rs:1642-1659`); no load guard in the loop | A bounded double rounding; unguarded against bypass in nonlinear models | Record it as an open item with T5 |
| S11-V6 | SHOULD-FIX (records) | `ROOT_RULINGS_V1.md` S11 pre-acceptance ("exposure needs three or more contributions …; nothing committed has that") | Realistic three-plus patterns are common. Committed models reach only two contributions per DOF (scan). Probe C gives a realistic breach: a 4.1e7 N thermal pair plus a 1.3 N nodal load gives 2.3e-9 | The ruling's conclusion stands, but its stated basis is the wrong filter | Restate the rationale: the ratio (about 1e7 or more), the production order and the scale convention, with errors of at most about ulp(gross)/2 per absorbed term. Keep the reopen condition |
| S11-V7 | NOTE | §6 | Three or more contributions are typical in user models | S11-F changes result bits widely in real user models, though not in committed fixtures. D1's differential covers the committed fixtures only | Disclose this in S11-F's record, and in release notes if relevant |

## 4. Confirmed

- **The map and D1's reach table R1–R9 agree with the source.** Every force-accumulation site D1 lists exists at `c61a540ea`, with the same set at T1's shifted lines (`PP_t1:8371-10420`) plus T1's global-load start at `:2248`.
- **A fold of one or two terms is correctly rounded (R9).** This is confirmed by construction and by probe C.
- **D1's scan** reproduces the nodal-only result. My broader scan agrees: at most two contributions per DOF in committed models.
- **C3-full adds no path to the composite `Err`** when every site in Q3 moves together. The exact context already solves from exact force expansions (`exact_boundary.rs:373-392`).
- **C1 and C2 are rightly rejected** against ROOT's constraint (`PP:1249-1253`; `PP:1754-1778` with `:1430-1432`).
- **The M03 audit starts from the summed force** (`FK/structural.rs:513-538`), as D1 states.

## 5. Not checked

- No product build or run. Every product-behaviour statement comes from reading the source.
- I did not read the `straight_pipe` formula order in detail. Probe A uses the textbook fixed-end terms folded per load, which is the structure at `straight_pipe:576-578`.
- R1's RF-CANCEL references are uncommitted, and I did not read them.
- I did not audit T4's or T5's future producers.

## 6. Run records (`T3/REVIEW/_run_records/s11/`)

- `probe_s11_check.py.txt` → `probe_s11_check.stdout.json`: checks A to D. It imports D1's `probe_skew_precision.py` unchanged. Run it from that folder.
- `scan_shared_node_contributions.py.txt` → `scan_shared_node_contributions.stdout.json`. Run it from `P/` with argument `.`.
- Standard-library Python 3.11.15, `nice 19`, each under 1 s. Hashes are in `T3/REVIEW/_run_records/SHA256SUMS`.
