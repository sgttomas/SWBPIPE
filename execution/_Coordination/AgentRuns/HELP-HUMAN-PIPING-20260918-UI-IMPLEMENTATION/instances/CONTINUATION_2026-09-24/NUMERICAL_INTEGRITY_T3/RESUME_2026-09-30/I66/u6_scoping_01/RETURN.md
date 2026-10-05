# I66 return: U6 carriers and standing, scoping (records only)

I66 is a TASK (Type 2) dispatched directly by ROOT under `BRIEFS/I66_U6_CARRIERS_SCOPING.md`. It did not delegate.

**The plan is `PLAN.md`,** sha256 `8742d10521cf5587748357076ab404d6d659f7e18aadffccd3fcfdcee82ed542`. `SHA256SUMS` covers every file in this folder. Paths are placeholders.

## Headline

**U6 is bounded but about 2.5 times I61's figure:**

| | Hours |
|---|---|
| Carrier authoring, U6a–U6d | 19–26 |
| Reader round (U6e) | 6–9 |
| Reviews | 12.5–16.5 |
| Elapsed, run in parallel | about 15–20 |

- **What drives the size:**
  - the TypeScript registration and standing work, which keeps D2 §4.7's standing parity;
  - the derivative's class disclosures. In the milestone, 69 of the 98 or 99 rows are `absolute_verified`.
- **The critical path.** U6 should stay off it if dispatched now. The rest of the chain to U7 runs U4 G4–G6, then U3 grant 2, then the live milestone. TypeScript is the unit to watch.

## Units and costs (PLAN §6)

| Unit | Content | Author | Review |
|---|---|---|---|
| U6.0 | ROOT rulings D-U6-1 to D-U6-9, the reservation and the T6 notice | ROOT, 0.5–1 h | — |
| **U6a (first: end-to-end)** | **Checkpoint A:** the two pinned milestone successor files, byte-identical to PP's pins, plus the shared standing-parity cases. **Then the Rust carriers:** dispatch, downgrade guard, fresh set, standing, binding refusal, summary, and `derive_document`/`validate_document` receipt copy and class disclosures. **The end-to-end check:** source → derivative → receipt back out byte-equal and revalidated; standing `needs_recompute`. | 5–7 h | 2–3 h |
| U6b | Python: dispatch, standing, AnalysisRun copy and validate, binding codes, the records.py guard, and a pin on the stress-neutral refusal | 4–5 h | 1.5–2 h |
| U6c | AnalysisRun and stress-neutral successor schema branches, RowDisclosure codes, and RV78-N2's probes as tests | 2–3 h | 1–1.5 h |
| U6d | TypeScript: route, async registration with a byte-bound cache, sync standing, AnalysisRun, reopen, rule-check gate, notices and the shared T6 output refusal | 8–11 h | 3 h |
| U6e | Reader round: D-U6-1 (Python entry), F5 (A2's exact list, which amends checkpoint A's D6a), RV79-N1 and RV80-N2 | 6–9 h | 2–3 h |
| U6f | Complete-diff review, three-language parity, ROOT suite runs, and the R-1 interface items C-1 to C-3 | ROOT 1 h | 3–4 h |

**The order:**
1. U6.0.
2. U6a checkpoint A, together with D-U6-1.
3. In parallel: U6a, U6b, U6c, U6d and F5.
4. U6f.
5. U7, which also waits on U3 grant 2, the U4 G6 permit and the live milestone.

## Decisions needed

All are for ROOT; none is owner-reserved (PLAN §7).

| # | Decision | Proposed answer |
|---|---|---|
| D-U6-1 | The Python reader's public entry, which refuses everything while incomplete | Align with Rust and TS: run every gate; the flag gates eligibility only. A scoped reader change, reviewed by RV79. |
| D-U6-2 | Derivative class disclosure | (A): `absolute_verified` and `not_covered` rows become `row_disclosures` with two new reason codes. This preserves 1:1 accounting. |
| D-U6-3 | TS scope while native W1 is held | The full TS carrier set now, with mocked IPC; the native witness is deferred. |
| D-U6-4 | Reservation | Reserve the 25 new names and 9 paths. All were absent at NUM, at main and at the facade head. |
| D-U6-5 | Fixture provenance | Byte-identical copies of PP's pinned successor files, plus a one-assertion PP test routed to U3 grant 2. |
| D-U6-6 | Current-admission sets | The successor joins them in U6; standing, not freshness, gates every reliance. |
| D-U6-7 | Reader-round bundling | F5, RV79-N1, RV80-N2 and D-U6-1 now; RV78-N2 in U6c. Defer RV78-N1, because of its re-pin cascade. F5 amends checkpoint A's D6a. |
| D-U6-8 | T6 coordination | Reserve the stress-neutral schema and `loadReferenceOutputAvailability.ts`, and post a notice on T6's row. No T6 panel is edited. |
| D-U6-9 | The legacy 0.1.0 AnalysisRun wrapper | Refuse sources carrying `retained_precision`. |

## Findings worth ROOT's attention

- **F-1. No product caller can deliver a successor in the milestone domain.** The Direct entry has no product caller, Tauri calls only the ordinary wrapper, and Headless is refused under D-2.
  - In the first package, the successor reaches only library (Rust and Python) consumers.
  - A native desktop witness of a successor is impossible until native activation.
- **F-3. The Python reader's public entry refuses all input while incomplete** (PY:1582–1585). Rust and TS hold only eligibility.
- **F-5. D2 §4.9.3's G4 premise does not hold for the readers.** The premise is that base readers refuse a `retained_precision` member. That is true of the results schema, but not of the base preview-physics-1 readers in any of the three languages. U6 adds a dispatch-level downgrade guard instead of editing the base readers.
- **F-6. The typed `MechanicsEnvelope` cannot carry a successor.** Headless carriage, which uses `into_parts()`, is a wider-F2a precondition of admitting Headless.
- **F-7. Reader eligibility omits D2 §4.9.4's `not_required` ordinary-eligibility conjunct.** This is unreachable in D1. U6's carriers apply the conjunct; the reader-side fix is wider F2a.

## Run facts

- **Run:** 2026-10-04, about 06:35Z to 07:15Z, with the memory guard (PID 5387) running.
- **Basis:**
  - NUM at `4c0b9c735a`, then `fc5d92c56c`, then `7e4f5a51dd`. ROOT's merge and records commits landed during the run; no reader or carrier file changed (`_run_records/BASIS.txt`).
  - U3 grant 1b was read as committed `4b31bbf23a` via `git show`.
  - I61's working tree was not read.
- **Method:**
  - Git reads only, with `GIT_OPTIONAL_LOCKS=0`;
  - stdlib Python only, for `collision_check.py` (output `COLLISIONS.json`) and `read_origins.py` (output `READ_ORIGINS.json`, 73 files hashed).
  - Nothing else was run: no Cargo, npm, pytest, schema validator, solver, native or DEC-025 job.
- **Writes:** only this folder and `WT/scratch/i66_u6_scoping_01/`, which holds two `git show` extracts of PP/lib.rs.
- **One slip, disclosed:**
  - the first extract was written one level up, as `WT/scratch/i66_u6_scoping_01_lib_bee3.rs`;
  - it was moved into the scratch folder within a minute;
  - no other path outside the fence was written.
- **No machine paths** appear in any file in this folder.
