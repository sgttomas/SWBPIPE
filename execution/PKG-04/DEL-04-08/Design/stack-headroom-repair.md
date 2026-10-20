# Stack-headroom repair (b7-plan item 5a)

Item 5a governs this work: "Stack-headroom repair (owner, 2026-10-20: "Proceed with option 1"): restore the R/64 = 1 MiB witness margin on the retained W1 path (first failing at #1398's 60441e1159; O4 added 89,440 B), re-register under D3, and run the stack witnesses in a maintained check." Some things do not change: R = 64 MiB, the witness divisor k = 16, the R/64 headroom leg, every assertion, and the memory policy (M = 12 GiB, ⌊0.9M⌋ = 11,596,411,699 B, the 5 % TEXT reserve).

## Measured failure

The witnesses run the permitted W1 work on one thread of the requested stack. In the registered build identity (debug, opt-level 0, aarch64-apple-darwin), all three headroom witnesses abort at 1 MiB in both modes:

- `witness_headroom_w1_at_one_mebibyte`
- `witness_w2_deep_milestone_publishes`
- `witness_w_c2_publishes`

Their R/16 = 4 MiB legs pass.

- **Painted peaks.** These come from a painted-stack probe appended to the witness file for measurement only. They are the same for all six witness/mode pairs:

  | Build | Peak (B) |
  |---|---:|
  | f63a4877e1, last pass | 1,032,720 |
  | 60441e1159 | 1,135,232 |
  | main before O4 | 1,149,520 |
  | fa019d9cef, main with O4 | 1,238,960 |

- **Smallest passing stack.** On fa019d9cef, a bisection over the requested stack size gives 1,228,800 B. A requested R yields R + 12,288 B of thread stack.
- **Peak chain on fa019d9cef.** Innermost frames first. Bytes are whole frames, from lldb:

  | Frame | Bytes |
  |---|---:|
  | `product_certificate::material` | ≈21,400 |
  | `build_member` | 50,544 |
  | `source_residual::coefficients` | 35,120 |
  | `source_residual::member` | 86,848 |
  | `source_residual::recover` | 203,632 |
  | `source_residual::run` | 53,632 |
  | `begin_prepared_product_reduced::{closure#0}` | 53,824 |
  | `retained_product.rs freeze_case::{closure#0}` | 502,784 |
  | `freeze_case` | 59,296 |
  | `freeze` | 36,832 |
  | `lib.rs w1_transaction` | 58,256 |

- **Why the frames are large.**
  - At opt-level 0 a frame is the sum of every stack slot in the function, so slots are not reused across scopes.
  - LLVM-IR slot attribution, matched to the measured frames within padding, puts the closure's 499,328 B mostly in:
    - 61 slots of `PreparedCandidateError` (4,616 B each), from its `?` and error returns;
    - 18 slots of `ProductProofDraft` (4,600 B);
    - 14 slots of `ProductProofFailure` (4,400 B);
    - 8 slots of `CertifiedProductProof` (4,256 B).
  - Every fallible phase of the closure, lines 6502–6629, coexists in that one frame.

## Repair: split the closure into sequential phases (as built)

The body of `freeze_case`'s closure is now a short parent that calls `#[inline(never)]` phase functions in the original order (`27e4dfa972`). During the deep call, the frame live above the source residual holds only the parent's and the draft phase's slots. Each phase's body is the original lines, moved unchanged.

| Phase | Original lines | Receiver | Returns |
|---|---|---|---|
| `freeze_case_inputs` | 6502–6515: attempt guard, native owner, `base_rows`, `specs`, MapWrite, `reduced` | `&self` (`capture_entry` takes `&self`) | the inputs |
| `freeze_case_draft` | 6516–6518: `begin_prepared_product_with` or `_reduced`, then `into_ready` | free function | the ready draft |
| `freeze_case_values` | 6519–6557: projection, maxima, sections, not-covered, values, aliases | `&self` | the payload, `projected` and `value_work` |
| `freeze_case_certify` | 6558–6564: distinct copy, rows, `certify_final` | `&self` | `distinct`, `rows` and the certification `Result` |
| `freeze_case_settle` | 6565–6610, except `pass`: failure bookkeeping, matches-values, verdict copy, observables, G5a | `&mut self` | `SettledCase::Certified` or `Refused(certificate, error)`, where no error means the owner mismatch |
| `freeze_case_commit` | 6611–6629: move counts, adapter charge | `&self` | `Result<(), …>` |

- **Ownership in the parent.** The parent keeps ownership of `base_rows`, `specs`, `reduced`, the maxima, `payload`, `distinct` and `rows`, declared in the original order. Each therefore drops at the same point and in the same order as before, including `drop(rows); drop(base_rows); drop(specs);` before the pass test.
- **`pass`.** `full_case_passed()` stays in the parent, on its original line.
- **Refusals.** On every settlement refusal the parent sets `certificate`, drops `rows`, then moves `payload.values` into `saved_values`, in the original order. The owner-mismatch text is still formed after `rows` drops.
- **Explicit matches.** The parent propagates each phase result with an explicit `match`, not `?`. That gives the same values and saves 46,640 B of parent frame.
- **Invariants.** The same calls run in the same order, with the same arguments, error values and mapping. The `trace` stages and the `#[cfg(test)]` output are unchanged. No allocation is added or removed. There is no unsafe code and no raw pointer.
- **Independent review.** It found the split EQUIVALENT on every success and refusal path, with no behavioural finding.

**Result.**

| Measure (registered build, all six witness/mode pairs) | Before (`fa019d9cef`) | After | Target |
|---|---:|---:|---:|
| Painted peak | 1,238,960 B | 893,168 B | — |
| Smallest passing stack | 1,228,800 B | 884,736 B | ≤ 983,040 B |
| Largest failing stack | — | 868,352 B | — |

- The parent's frame is 142,992 B. A measurement prototype reached 802,816 B by moving drops, which the as-built shape does not do.
- If more margin is wanted, the next step is to group `freeze_case_values`, `freeze_case_certify` and `freeze_case_settle` under one `#[inline(never)]` call, an estimated saving of about 50 KB.

**Margin left in reserve.** Other splits are measured or estimated:

| Split | Est. saving |
|---|---:|
| `freeze_case`'s match | ~37 KB |
| `freeze` lines 5264–5267 | ~18 KB |
| `w1_transaction`'s tail | ~40 KB |
| `begin_prepared_product_reduced`'s validation | ~25 KB |
| `run`'s pre-pass | ~19 KB |
| `recover`'s loop and tail | ≤ ~124 KB, capped by the `residual_recorded` sibling branch |

They are not taken: the closure split alone restores the margin and keeps the priced-source change to one function. Per-leaf splits inside the kernel gain little, because the branches under `member` are within about 2 KB of each other.

**For other designs that read this code.**
- `source_residual::recover`, `source_residual::member`, the kernel and every function the closure calls are unchanged.
- Only the closure's body is reorganized into phase functions in `retained_product.rs`, with the same calls, order and outputs.
- A design that cites `freeze_case`'s closure by line should cite the phase functions instead.

## Re-pricing and re-registration (D3)

D3 re-priced the repaired source at `6b90748dec`, after the full-file and function-body pins were reviewed and the two shifted caller keys carried through the line map. All four points complete with no unmapped header or unclassified argument and both o10 checks at zero. No positive owner or pricing formula changed: all point trees and the merged tree are coefficient-identical to O4's registration.

The generated profile and registration now name that commit (retained-pricing-rules §8.10). All 70 evaluations fit at the unchanged policy; the tightest remains merged dense W3, with 2,795,665,581 B headroom, a difference of 0 B from §8.9. The build identity, reader layouts, twelve reviewed-input digests and every numerical law-test pin are unchanged. `generate --check`, the law tests, both full suites and all six registered stack witnesses pass; the phase table is in §8.10.

## Verification (as built)

- **Witnesses**, in the registered build, one process each:
  - the six headroom witnesses pass;
  - `witness_w7_fault_fallbacks`, `witness_w_c2_ac_publishes_sparse` and all 17 `control_*` entries pass.
- **Full product_physics suite:** 1,119 passed, 0 failed and 131 ignored, both in the registered build and in an emulated unregistered build (`RUSTFLAGS=--cap-lints=warn`, [CI strategy](../../../../docs/CI_STRATEGY.md)).
- **Other crates:**
  - result_export, at the hosted flags: 338 passed;
  - headless: 114 passed.
- **The six headroom witnesses also pass unregistered.**
- **No numerical change:**
  - the pytest suite (7,284 passed, 50 skipped, 0 failed) leaves every committed J document and preview record byte-identical;
  - `generate --check` is equal at the re-registration;
  - the O4 M3 judging fast set passes;
  - the declared-problem oracle gives 578 PASS and 0 FAIL.
- **Judging warrant.** The S6 judging warrant (65fd80b176) stands, because no numerical path changed.
- **Independent scrutiny by a separate agent:**
  - the split's equivalence: EQUIVALENT;
  - the D3 re-pricing and registration: SOUND (no findings).

## Maintained check (as built)

The witnesses abort the process on overflow, so each runs in its own process: `cargo test --offline --manifest-path core/product_physics/Cargo.toml --lib -- retained_memory::witness_tests::<witness>::<mode> --exact --ignored --test-threads=1`. `check_release_readiness.stack_witness_commands` defines this list once.

- **Registered evidence (VER-004).**
  - `tools/release/check_release_readiness.py --profile stack-witnesses` runs the six commands. The profile is also included in `all`.
  - The macOS release sweep (`run_evidence_sweep.py`, surface `cargo_crate_sweep`) runs it after the crate sweep. On the qualification host this is the registered-profile record.
  - The `cargo` profile stays one step per manifest, which `tools/ci/numerical_ci.py` relies on.
- **Early warning on every selected PR.**
  - Whenever product_physics is selected, the hosted numerical plan runs the same six commands, with `--locked`, at the default debug profile, after the optimized suite.
  - Hosted builds are unregistered, so this lane is a regression signal, not registered evidence. Its frames are those of an unoptimized x86_64 Linux build.
  - `build_commands` compiles the harness-filtered commands once.

## Lesson

At opt-level 0, each `?` or error return on a large error type costs the caller 2–4 stack slots the size of the error (`PreparedCandidateError` is 4,616 B). A long fallible body that stays live during a deep call should be split into `#[inline(never)]` phases, so the frame on the deep path holds only that phase's slots. After each split, measure the next sibling branch.
