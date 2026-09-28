# K4 change record: the W1a kernel method (retained precision)

> **DRAFT (checkpoint D not reached).** Written while D1 revision 5a.3 (the stop rule's resolution floor) is drafted. Every place that waits on it is marked **[pending D1 5a.3]**. Checkpoints B (suites, T9) and C (mutations) have not run; their rows are placeholders.

This is the draft PR record for slice K4 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. The implementation is I12 (TASK, the owner's Mac); the detail is in `RETURN.md`.

- **Branch:** `codex/piping-k4-20260928`, from main `e7d930d49` (K2b merged, PR #1040).
- **Checked revisions:**
  - checkpoint A1, `cef218a10` (tests B–F);
  - checkpoint A2, `3ed6c0e26` (F-1's combination, tests G–L, the R1 lane, O8);
  - the S\* addendum's implementation **[pending D1 5a.3]**;
  - checkpoint B (suites, T9), checkpoint C (mutations) and these records: to come. ROOT records the final candidate, PR and merge revisions.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.1 (§4.1.1–§4.1.8, §4.1.6.1), §4.3, §4.10, §5 items 1 and 7, the K4 row of §6, §7.3; revision 5a.3 **[pending D1 5a.3]**;
  - `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md` with its ROOT rulings Q1–Q12;
  - `ROOT_RULINGS_V1.md`: "K4: spawn and rulings", "K4: Q5 amended", "K4: rulings on I12's checkpoint-0 plan" (`bacf939a8`), "K4: A1 findings F-1 to F-3, the stop rule's blind spot" (`2f48511b7`), and ROOT's relayed A2 rulings (RETURN §1).

## What changes

K4 adds W1a's kernel method under `FK/src/structural/retained/`: a per-case primitive source, the exact load ledger at p, formation and assembly at p, the p-factor with its screens, solve with the p + 64 residual, recovery before rounding, combinations, and the adaptive schedule with its stop rule, classification and evidence. **K4 has no product caller (Q1); F2a wires W1.** Nothing K4 adds is reachable from outside FK (Q9).

- **New files in `FK/src/structural/retained/`:**
  - `wide_sum.rs`: the correctly rounded exact multi-term sum (Q2(a)): stack magnitudes, an 8,128-bit span limit that refuses and never truncates, one rounding through K3's `from_integer`;
  - `source.rs`: `PrimitiveSource` and its parts, validation, bodies, the canonical encodings (Q10);
  - `ledger.rs`: the exact per-DOF load ledger and its projection to p;
  - `assemble.rs`: formation (frame, B, D, K_e) and exact assembly at p on K1's `SparsePattern`; the exact reduced right-hand side;
  - `factor.rs`: geometry first (Q6 as amended by O1), the RCM port (Q7), radix equilibration, the profile LDLᵀ at p with its exact pivot screen, negative energy, Hager–Higham;
  - `recover.rs`: the published layout, recovery at p, the +0.0 publication rule (Q11), the retained-state encoding;
  - `combine.rs`: combinations as their own solve (ROOT's F-1 ruling);
  - `adaptive.rs`: the schedule, the stop rule, S\* and the classification, budgets and work, factor reuse, the evidence and the outcome types.
- **`FK/src/exact_sum.rs` (+19):** the approved read accessor `net_parts` (Q3); additive, no behaviour change.
- **`FK/src/structural/retained/mod.rs` (+15):** the `pub(crate) mod` lines and documentation.
- **`FK/tests/s11_site_table.rs` (+61):** K4's eight files join SOURCES (Q8), with the disposition "p-bit exact expansion, rounded once" on the exact sites and integer counters listed.
- **`FK/tests/retained_k4/` (new):** ten `#[path]` test modules of K4's files (87 tests), `support.rs`, `models.rs`, the standard-library generator `gen_k4_vectors.py` with `--check`, the vectors (about 4.8 MB) and `SHA256SUMS`.

## Standing, values and bytes

- **No published byte changes are expected** (Q1): K4 adds no caller outside `retained`. The evidence is T9 **[checkpoint B]**.
- **Unchanged:** K3a's and K3's code and test directories (both generators' `--check` OK), K-D5's tests (NI kd5 16 passed; FK `formation_check`), `exact_sum`'s tests (8 passed), `wide.rs`, `multi.rs`, `structural.rs`, `lib.rs`, `sparse.rs`, `load_ledger.rs`, `rigid_body.rs`, `exact_boundary.rs`, `formation_check.rs`, SA, PP, NI, `sparse_direct`, the fixtures and R1's references. No dependency or lockfile change.
- **The both-entry gate is not run** (Q1).

## Files

| File | +/− | Lines |
|---|---|---|
| `P/core/solver/frame_kernel/src/structural/retained/wide_sum.rs` (new) | +503 | 503 |
| `P/core/solver/frame_kernel/src/structural/retained/source.rs` (new) | +743 | 743 |
| `P/core/solver/frame_kernel/src/structural/retained/ledger.rs` (new) | +202 | 202 |
| `P/core/solver/frame_kernel/src/structural/retained/assemble.rs` (new) | +559 | 559 |
| `P/core/solver/frame_kernel/src/structural/retained/factor.rs` (new) | +763 | 763 |
| `P/core/solver/frame_kernel/src/structural/retained/recover.rs` (new) | +504 | 504 |
| `P/core/solver/frame_kernel/src/structural/retained/adaptive.rs` (new) | +2099 | 2099 |
| `P/core/solver/frame_kernel/src/structural/retained/combine.rs` (new) | +133 | 133 |
| `P/core/solver/frame_kernel/src/structural/retained/mod.rs` | +15 | 39 |
| `P/core/solver/frame_kernel/src/exact_sum.rs` | +19 | — |
| `P/core/solver/frame_kernel/tests/s11_site_table.rs` | +61 | — |
| `P/core/solver/frame_kernel/tests/retained_k4/*_tests.rs` (new, 10 modules) | +4488 | 4488 |
| `P/core/solver/frame_kernel/tests/retained_k4/{support,models}.rs` (new) | +880 | 880 |
| `P/core/solver/frame_kernel/tests/retained_k4/gen_k4_vectors.py` (new) | +1923 | 1923 |
| `P/core/solver/frame_kernel/tests/retained_k4/*.txt` and `SHA256SUMS` (new; generated) | +44185 | 44185 |

Line counts are at `3ed6c0e26` plus the Q10 pins; recounted at the candidate **[checkpoint D]**. The addendum's implementation changes `adaptive.rs` and its tests **[pending D1 5a.3]**.

## Checks

All run on `aarch64-apple-darwin` with rustc 1.97.1, `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 8`, `RUST_TEST_THREADS=4`, under the Mac host rules.

- **K4's tests (A1 + A2, and the Q10 pins):** 87 of 87, plus `exact_sum`'s 8; the whole suite took 205.6 s of debug wall time at `3ed6c0e26`. RETURN §11 lists them by the brief's letters.
- **The references lane (K):** R1's 128 K4 cases through the adapter: 6,475 comparisons pass, 0 fail, 51 not covered (exactly §4.10's list), 8 refusals; the routed cases and the exact-block oracle pass (RETURN §12). The floor check is S\*-dependent **[pending D1 5a.3]**.
- **O8:** the generator's bit-for-bit emulation of the method reproduces five retained-state digests.
- **Generators:** `gen_k4_vectors.py --check` OK for 13 of 13; K3a's and K3's `--check` OK.
- **Hygiene:** the non-test and test builds of FK have no warnings; rustfmt applied to K4's files; the S11 site table passes with K4's rows; no `__pycache__` is written.
- **Suites (39 manifests, `--no-fail-fast`) against the Mac baseline of main:** **[checkpoint B]**.
- **T9 (Mac-only):** **[checkpoint B]**.
- **Mutations:** **[checkpoint C]**.

## Remaining

- **For I12:** implement D1 revision 5a.3 **[pending D1 5a.3]** and update the S\*-dependent assertions (RETURN §11.2); B, C and D.
- **For ROOT:** the review, hosted CI (record the numerical job's time, Q12), the merge record.
- **Deferred by ruling:** the constructed-ceiling combination and K4-M24's kill (to the addendum; RETURN §8.4); F2a's limits (Q5 amended); the public export (Q9, first consumer); unifying `Binary64Outcome` with K2b's `Representability` (Q11, F2a); a full geometric treatment of partial directional grounds (O1, W4/K5).
