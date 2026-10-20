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

## Repair: split the closure into sequential phases

The body of `freeze_case`'s closure becomes a short parent that calls `#[inline(never)]` phase functions in the present order. During the deep call, the frame live above the source residual then holds only the parent's and the draft phase's slots. Each phase returns the values the next phase uses.

| Phase | Lines | Receiver | Returns |
|---|---|---|---|
| Inputs | 6502–6515: attempt guard, native owner, `base_rows`, `specs`, MapWrite, `reduced` | `&self` (`&mut` for `capture_entry`) | inputs |
| Draft | 6516–6518: `begin_prepared_product_with` or `_reduced`, then `into_ready` | free function | the ready draft |
| Projection, maxima, sections, not-covered, values, aliases | 6519–6557 | `&self` | the payload, `projected` and `value_work` |
| Distinct copy, rows, `certify_final` | 6558–6564 | `&self` | `distinct`, `rows` and the certification `Result` |
| Settlement | 6565–6610: failure bookkeeping, matches-values, verdict copy, observables, G5a, pass | `&mut self` | — |
| Commit plan | 6611–6629: move counts, adapter charge | `&mut self` | `Ok(payload)` |

The parent keeps ownership of `reduced`, `maxima`, `distinct`, `rows`, `base_rows` and `specs`. Each therefore drops at the same point and in the same order as today, including `drop(rows); drop(base_rows); drop(specs);` before the pass test.

Every settlement error path that today sets `saved_values=Some(payload.values)` returns to the parent with a marker, and the parent performs that move after `rows` is dropped, exactly as now. `certificate` is set by the parent from the settlement's result, at the points it is set today.

**Invariants.** The same calls run in the same order, with the same arguments, error values and error mapping. No allocation is added or removed and none moves to the heap. Only where values live on the stack changes. Because the draft and projection borrow `self.facts`, these phases take `&self` and only the settlement and commit phases take `&mut self`. The measurement prototypes used a raw-pointer alias instead; production must not.

**Expected result.**
- A measurement prototype of this split, with inputs and draft separated as above, gave:
  - a painted peak of 801,488 B;
  - a smallest passing stack of 802,816 B, against the target of ≤ 983,040 B (64 KiB below 1 MiB);
  - a closure frame of 51,312 B and a draft frame of 14,000 B.
- The production shape has a few more call slots in the parent, so its estimated peak is 800–835 KB.

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

## Verification

- **Witnesses**, in the registered build, one process each:
  - the six headroom witnesses;
  - `witness_w7_fault_fallbacks`, `witness_w_c2_ac_publishes_sparse` and the `control_*` entries;
  - the painted peak and the bisected smallest passing stack for all six.
- **Full product_physics suite**, in the registered build and in an emulated unregistered build ([CI strategy](../../../../docs/CI_STRATEGY.md)).
- **No numerical change:**
  - the committed J documents, preview records and `generate --check` are byte-identical;
  - the O4 M3 judging fast set and the declared-problem oracle pass unchanged.
- **Judging warrant.** The S6 judging warrant (65fd80b176) stands, because no numerical path changes. That rests on the byte-identity above and on the review of the split's equivalence.
- **Independent scrutiny by a separate agent** covers:
  - the split's equivalence: evaluation and drop order, error mapping, and the timing of the `saved_values` and `certificate` moves;
  - the D3 re-pricing and registration.

## Maintained check

The witnesses abort the process on overflow, so each runs in its own process: `cargo test --offline --locked --manifest-path core/product_physics/Cargo.toml --lib -- retained_memory::witness_tests::<witness>::<mode> --exact --ignored --test-threads=1`.

- **Registered evidence (VER-004).** `tools/release/check_release_readiness.py` gains a `stack-witnesses` profile, also included in `all`, that runs the six commands. On the qualification host this is the registered-profile record. The `cargo` profile stays one step per manifest, which `tools/ci/numerical_ci.py` relies on.
- **Early warning on every selected PR.** The hosted numerical plan runs the same six commands in the default debug profile whenever product_physics is selected.
  - Hosted builds are unregistered, so this lane is a regression signal, not registered evidence.
  - Its frames are those of an unoptimized x86_64 Linux build.
  - It needs the six witnesses not to depend on registration. W3's G0 assertion is not among them; this is checked in the emulated unregistered build before the lane is added.
  - It follows the optimized-profile change to that plan (#1469).

## Lesson

At opt-level 0, each `?` or error return on a large error type costs the caller 2–4 stack slots the size of the error (`PreparedCandidateError` is 4,616 B). A long fallible body that stays live during a deep call should be split into `#[inline(never)]` phases, so the frame on the deep path holds only that phase's slots. After each split, measure the next sibling branch.
