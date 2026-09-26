# T3 stage 1 — design and reference plan

WORKING_ITEMS manager for T3, 2026-09-26. This plan follows the T0R pattern that ROOT selected in `DEFAULT_ROUTE_DESIGN/`: independent references frozen before implementation, an independent design review, an independent reference refutation, then implementation. It is an ad hoc plan for this tranche, not a reusable workflow. Read with [STAGE0_MAP.md](STAGE0_MAP.md).

## 1. Split of the design

T3's scope has two parts with different constraints:

- **Numerical core** (D1): general accuracy, general retained-source recovery, range, sparse assembly, M03 residual items and the VP-ROBUST harness. Most of it can be built in kernel crates that T1 does not touch.
- **Standing, envelopes and transport** (D2): the composite finalization fix, joined eligibility and binding, selected-UNAVAILABLE alignment, source-blocks-1 re-homing, and transport and display range. Almost all of it lands in files T1 is changing.

Two design TASKs run in parallel, each stating its assumptions about the other in an interface section. One reviewer then reviews both together (V1), so the interface is checked by someone who wrote neither.

## 2. TASKs requested

| ID | Role | Brief | Write set | Starts | Needs a build? |
|---|---|---|---|---|---|
| D1 | HELPS_HUMANS-style design TASK | [TASK_BRIEFS/D1_NUMERICS_DESIGN.md](TASK_BRIEFS/D1_NUMERICS_DESIGN.md) | `T3/DESIGN_NUMERICS/**` | Now | Reading now; an optional probe only after ROOT releases the host |
| D2 | HELPS_HUMANS-style design TASK | [TASK_BRIEFS/D2_STANDING_DESIGN.md](TASK_BRIEFS/D2_STANDING_DESIGN.md) | `T3/DESIGN_STANDING/**` | Now | As D1 |
| R1 | Reference author TASK, product-code-blind | [TASK_BRIEFS/R1_REFERENCES.md](TASK_BRIEFS/R1_REFERENCES.md) | `T3/REFERENCES/**` | Now | No; standard-library Python only |
| V2 | Independent reference refutation | written when R1 returns | `T3/REFERENCE_CHECK/**` | After R1 | No |
| V1 | Independent design review of D1 and D2 together | written when both return | `T3/REVIEW/**` | After D1 and D2 | Optional read-only probe after host release |
| P1 | Detection run on main | written after V2 | `T3/DETECTION/**` | After V2 and host release | Yes: an out-of-repository probe against the public entry, both modes |

All write sets are disjoint, and all are records folders; no TASK in stage 1 writes product source. Common terms are in [TASK_BRIEFS/_COMMON.md](TASK_BRIEFS/_COMMON.md).

**Independence.** R1's author reads no product source. V2 is not R1's author, and re-derives R1's values by a different method (for example R1's closed forms against an exact-rational stiffness assembly written from the theory, not from product code). V1 wrote neither design. Backchecks after revisions go to the same V1 or V2 reviewer, in a fresh context where possible.

## 3. Reference families

R1 writes these families (the brief has the detail): RF-CHAIN (order > 2 soft chains), RF-SKEW (skewed members with rational rotations), RF-WEAK (weak coupling), RF-LARGE (determinate and indeterminate families to 10,000 members), RF-INVARIANCE (rotation, origin offset, relabelling, units), RF-RANGE (exact power-of-two scaling to the ends of the normal range), RF-ZERO (structural zeros with derived zero scales), RF-FINITE (intended against represented-input answers per quantity) and RF-MECH (mechanisms that must be refused). Every family carries negative controls.

The existing frozen N01–N09, R01–R07 and NP-A–NP-D references remain in force unchanged, and T3 does not duplicate them.

## 4. Sequence and checkpoints

1. **Now:** D1, D2 and R1 in parallel. No builds.
2. **R1 returns → V2.** Findings go back to R1 for revision, then a V2 backcheck.
3. **D1 and D2 return → V1.** Findings go to ROOT with the manager's proposed dispositions. ROOT rules where a ruling is needed (as in `DEFAULT_ROUTE_DESIGN/ROOT_RULINGS.md`); the designers revise; V1 backchecks.
4. **Host released and references refuted → P1.** P1 records, per reference quantity, what main publishes today: expected mismatches (N05-class accuracy, order > 2, skewed members, range) and expected refusals (mechanisms). This is the pre-change baseline.
5. **Checkpoint to ROOT for selection:** the stage-0 map, both designs, the review and backchecks, the references and their refutation, the detection baseline, the file plan against T1, and any owner-level options. Nothing is implemented before ROOT selects.

## 5. Implementation shape (for planning; the designs decide)

- **Before T1 merges:** kernel-first slices in disjoint files. These are the general accuracy method and the sparse assembly path as kernel APIs with kernel-level tests, the VP-ORACLES and VP-ROBUST assets under `P/validation/`, the performance harness observations, and any display-range repair in the disjoint display and units files.
- **After T1 merges, and after main is merged into the T3 branch:** facade wiring in `PP`, `source_recovery.rs` and `source_receipt*`; the composite finalization fix; joined eligibility and binding; standing, identity and schema changes; source-blocks-1 re-homing.
- **Serialization of the shared facade.** The graph names one integration owner for `core/product_physics`. T3 writes it only after T1's merge, and ROOT serializes T3 against any other tranche writing it.
- **Merge gate** (as in the graph): complete-diff independent review; hosted CI, including the full dual-viewport dispatch for surface 4; a clean DEC-025 sweep; native witnesses on the owner's Mac, recorded as outstanding if not available.

## 6. Owner decisions

None are needed to start stage 1. `STAGE0_MAP.md` §5.3 names the one foreseeable owner-level question: whether any protected comparison predicate changes. The recommendation is to leave every protected predicate unchanged and apply the existing relative form, with stated zero scales, to new T3 cases only. If a design concludes otherwise, the manager brings ROOT concrete options with a recommendation before implementation.

## 7. Items for ROOT to confirm

1. The two T0R carries in `STAGE0_MAP.md` §2.8 (source-blocks-1 re-homing and the abs-sum summary) are T3 scope. T0R's selection routes them here, but the graph's T3 row does not list them.
2. The two-design split in §1, and the six TASKs in §2.
3. The M03 nonlinear items (mixed recovery basis, general mixed-device gap classification) stay on T5 with M06.
