# K2a change record: checked formation

This is the draft PR record for slice K2a of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I6 (TASK).

- **Branch:** `codex/piping-k2a-20260927`, from main `5ae22926e` (after K-D5, PR #1017). The branch contains S11-K, K3a, S11-F, S11-G and K-D5.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.7 "Formation range" step 1, the K2a row of §6, the note "Why K2a is separate from S11-K", and §7.3 mutation 18;
  - `ROOT_RULINGS_V1.md`: the S11 decisions (D-S11-4: no in-band marker, disclosure in the change records), the formation silent zero, K2a as its own slice after K-D5 (D5_CHECK 3), and the K-D5 combined-gate ruling (the two-part gate);
  - R1's RF-RANGE references (`REFERENCES/references.json`, `c0f14201c`) and V1's FORM-C probe (`REVIEW/_run_records/r2_backcheck/probe_r2_formation.*`);
  - the I6 brief (`TASK_BRIEFS/I6_K2A_IMPLEMENTATION.md`, `40ebb49ae`) with `_COMMON.md`.

## What changes

- **`FK/lib.rs` `local_stiffness`:** every coefficient (EA/L, GJ/L, 12EI/L³, 6EI/L², 4EI/L and 2EI/L about y and z) is formed with the same binary64 operations, in the same order, as before. **Every intermediate product and quotient is checked, not only the final coefficient:** L², L³, E·A, (E·A)/L, G·J, (G·J)/L, k·E for k = 12, 6, 4, 2, (k·E)·I and (k·E·I)/Lⁿ, 26 checks in all.
  - An intermediate formed from nonzero finite operands that is zero, subnormal or non-finite is refused with the new **`FrameKernelError::NumericalRange { name }`**. `name` names the coefficient and the intermediate, for example `GJ/L: G*J` or `12EIy/L^3: (12*E)*Iy`.
  - An exactly zero operand, such as a zero modulus, is not a range error. The existing input check still refuses it first, unchanged (`NonPositiveInput`).
  - An accepted coefficient keeps exactly its previous bits. k·E is formed once and shared by the y and z coefficients; this is the same operation on the same operands, so the bits are the same.
- **The variant's Display:** `range: stiffness formation outside the binary64 normal range at <name> (zero, subnormal or non-finite from nonzero finite operands)`. The product publishes kernel errors through Display (for example `SOLVER_SYSTEM_BLOCKED`), so this is the published reason wherever a formation reaches the kernel.
- **`diagnostics/src/lib.rs` `diagnostic_from_frame_error`:** this is the one exhaustive match outside FK. It gains the mapping with **no new diagnostic code**: `InvalidNumericInput`, `Blocking`, `ModelValidation`, the same code, severity, source and class as a formation overflow before K2a (`NonFiniteInput` "computed local stiffness"). The message names the intermediate. Readers need not change.

## Which cases change standing, and why

- **At kernel level:** any formation whose intermediate leaves the binary64 normal range is now refused by name.
  - Before K2a, an underflow became an exact 0 (or an imprecise subnormal) and passed the finiteness check. That silently removed a member's torsion or bending stiffness, or made the element inconsistent.
  - Before K2a, an overflow was already blocked, as a non-finite "computed local stiffness".
  - R1's RF-RANGE LEF-small (−(200, 300, 600)) on all three bases: GJ/L, 6EI/L², 4EI/L and 2EI/L are exactly 0, and 12EI/L³ is a normal value 3.5 % wrong, from a least-subnormal intermediate (5e-324). **Now refused**, at `GJ/L: G*J`.
  - R1's LEF-large (+(200, 300, 600)) on all three bases overflows. It was blocked as non-finite and is **now refused by name**, at `GJ/L: G*J`.
- **R1's LEF cases at product level: no standing change** (see the gate below).
  - LEF-small's member lengths (about 6e-61 m) are below the kernel's 1e-12 m axis tolerance. On both entries the element is refused before formation, as a degenerate element length (`PIPE_ELEMENT_INPUT_INVALID`). This is unchanged and pinned by a test.
  - LEF-large is refused at capture on the **captured** entry (`CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`). On the **typed** entry it reaches `local_stiffness`.
    - Main blocks it with `SOLVER_SYSTEM_BLOCKED` (refused_blocked). Main's message was not recorded by P1; by the code path it is the non-finite "computed local stiffness" error.
    - K2a blocks it with `SOLVER_SYSTEM_BLOCKED` and the named reason `… at GJ/L: G*J`, on all 3 bases and in both modes.
    - The standing is unchanged. Only the blocking message changes.
  - These refusals match P1's detection run on main (`DETECTION/RETURN.md`).
  - K2a closes the silent path for any caller that does reach the kernel's formation with such inputs: the free function, `FrameElement`, assembly, straight pipes, the adapter, the performance harness and the benchmarks (`_run_records/callers.txt`).
- **Product reach through material and section values** (ROOT's product-reach rulings; the authoritative statement is `RETURN.md` §5). Capture and validation have no lower magnitude bound, so admissible but physically absurd section and material values reach the kernel's formation on both entries.
  - **Partial underflow with a nonzero subnormal-derived sibling:** main's M03 already refuses these as `NUMERICAL_INTEGRITY_UNRESOLVED`. K2a refuses them earlier, by name.
  - **An exact zero in 12EI/L³, or its least-subnormal pattern:** main accepts these, because M03's element-entry floor (about 2^-974.6) does not see them. Main then publishes a wrong value, 99.95 % (reach_zero) and 32.4 % (reach_lef), but **only as untrusted**: Sensitive on the linear route through K-D5's re-formation, and unresolved on the nonlinear route. K2a refuses them at formation, by name.
  - **No trusted route that publishes a wrong value was found on main.** Main's linear standing before K-D5 is not claimed.
  - All four cases are pinned on both entries and in both modes by `core/product_physics/tests/k2a_formation_range_runtime.rs`.
- **The benefit** (ROOT): K2a corrects no trusted published value on the product route. It is a formation-layer guarantee, independent of M03 and K-D5: a named, earlier refusal, and protection for every `local_stiffness` consumer outside M03's check (`RETURN.md` §5.7).
- **The cost:** K2a refuses 1/L-lifted zeros where main's published value was within its K-D5-limited criterion. An example is the spring-carried test case: G = 1e-300 Pa, GJ/L formed as 0, and main's θ of 1.0 rad was accurate. This is **an availability change on admissible but physically absurd inputs, where main's value was accurate**, and it is a case K2b's scaling should restore.
- **Realistic reach: nil.** A refusal needs a section product (E·I, G·J or E·A, divided by at most L³) below about 2.2e-308, or above about 1.8e308, in SI units.
- **No value changes for any normal formation.** Every coefficient that is accepted has exactly its previous bits. The tests show this against a verbatim copy of the previous formula for:
  - 130 RF-RANGE members;
  - a 1,683-case sweep;
  - K-D5's F122 and F345 models.
- **The refusal is the design's interim.** Until formation-time scaling lands (K2b, SCALE-W, the kernel half; F1 wires it in the facade), checked formation refuses; it never scales. After K2b and F1, LEF-small and LEF-large are to be solved (D1 §4.7 step 2).
- **No in-band marker** (D-S11-4). No report field, result or evidence line is added. The refusal is an ordinary kernel error, and disclosure is in these records.

## Results

- **Tests:**
  - Kernel: 10 new tests in `FK/tests/k2a_checked_formation.rs`, covering LEF-small and LEF-large on 3 bases with the silent path shown first; the other RF-RANGE vectors byte-identical; the partial-underflow and subnormal-intermediate controls; one row per checked intermediate (26); a 1,683-case bit-identity sweep; the zero-operand rule; and the K-D5 interaction.
  - 1 diagnostics test.
  - 3 product test functions for the 4 product cases, on both entries and in both modes.
- **Mutations:** 31 of 31 killed, including mutation 18 as unchecked formation and as final-coefficient-only, and 26 drop-one-check mutants.
- **Suites:** 24 of 24 crates pass (FK 159, diagnostics 25, PP 519).
- **Fixture diff:** 112 of 112 committed outputs byte-identical against main. No committed byte changes, so the stop rule did not trigger.
- **Gate:** the two-part both-entry no-Passed-breach gate against main's empty lists **passes**: 888 runs, 768 on frozen references, 0 trusted breaches, and 0 standing changes against main.
- **Values:** no value changes for any normal formation.
- **The refusal is the design's interim** until K2b (formation-time scaling, kernel half) and F1 (facade wiring).
- **No in-band marker** (D-S11-4).
- Details are in `RETURN.md`.

## Limits

- The check covers `local_stiffness` only. User-stiffness elements, curved bends, springs and loads are formed elsewhere and are unchanged. Formation-time scaling is K2b's; the facade wiring is F1's.
- A subnormal **input** (for example a modulus below 2^-1022) is the user's exact value and is not itself refused. Only the intermediates formed from it are checked.
