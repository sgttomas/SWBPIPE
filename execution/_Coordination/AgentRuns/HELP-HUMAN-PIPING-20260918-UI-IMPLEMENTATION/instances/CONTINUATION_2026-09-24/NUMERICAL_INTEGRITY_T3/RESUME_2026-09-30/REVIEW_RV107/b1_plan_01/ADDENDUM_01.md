# RV107 ADDENDUM_01: confirming I84's PLAN_v2 against the review and ROOT's R1

TASK (Type 2), RV107, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. This addendum extends `REVIEW.md` (`03c3111d…a426`), which is unchanged; its SHA256SUMS still check 4/4.

**The request:** ROOT's message asking me to confirm PLAN_v2 against my review and ROOT's ruling R1, with decisions 22–26, the new stops, the estimates, and the diff from v1.

**The candidate:** NUM `06970ea95a`, folder `R/I84/b1_plan_01/`. `SHA256SUMS.v2` checks 10/10 OK.

| File | sha256 | Note |
|---|---|---|
| `PLAN_v2.md` | `c85786b704805311485b44ba27c8826ca278c1237d92010f9f997dbf3c919be0` | 61 `[r1: …]` marks |
| `R1_AMENDMENTS.md` | `5b366039cff7e32bb8e993d6529f5c488672e130065c4b479924c43692cd1f31` | |
| `PLAN.md` | `7f9699f3…4bb5c` | Unchanged |

**The ruling:** RR "R1: B1's plan ruled with seven amendments; I84 writes PLAN_v2; B6 first as its own PR" (RR:13794).

**Method.** I diffed v1 against v2 (1,201 changed lines) and read every changed passage, then read PLAN_v2 whole. I checked its new claims against the code at main `47a3bdfcf5`, which is unchanged at NUM's head outside `P/execution`, and against B6 at `a79dbd2e4a`. Documents and code reading only; no cargo, vitest, native or solver job; no installs; no Git writes; nothing in the system temp directory.

## Verdict

**Not confirmed as written: 0 BLOCKING, 2 SHOULD-FIX, 11 NOTE.**
- SF-1 to SF-4, SF-6 and SF-7 are resolved as ruled.
- SF-5 is resolved except for one new defect in how the challenge picks its bound (A1-S-1).
- The SF-2 amendment created a dependency the plan doesn't show (A1-S-2).
- Both open findings are short text amendments. ROOT can rule them into the slice briefs; no PLAN_v3 is needed.

## Open findings

| ID | Sev. | Where | Finding and evidence | Remedy |
|---|---|---|---|---|
| **A1-S-1** | SHOULD-FIX | §3.5, §3.6, decision 24 | **The challenge's per-phase bound can't be implemented as written, and it would be unsound.**<br>• **The phase can't be seen.** The challenge is an integration test, so it sees only the public surface. `RetainedPreviewOutput::retained()` and `W1Fallback` are `pub(crate)` (`lib.rs`). The N1 notice is the same plain text for Preparation, Native, Candidate, Staging, Precommit, and any Serializer failure without a C1:68 detail. "The notice's detail and count" therefore can't tell W2 from W3 or W4. Exposing the cause would be a D1 visibility change for a test, which PLAN_v2's own fence now forbids.<br>• **The mapping uses the phase reached; it needs the maximum so far.** A run that falls back at Precommit has already passed W3. Today's record has W3 above W4: 3,508,669,422 > 3,482,311,587 sparse, and 3,528,379,870 > 3,502,022,035 dense (`PINNED_RECORD`). A Precommit fallback bounded by W4 is therefore unsound.<br>• My review's SF-5(c), "choose the bound by the phase reached", invited this; I correct it here. | • In the challenge, bound every run that did W1 work (a successor, or an N1 notice) by E_mov,max. Bound runs with no permit by the W1 phase.<br>• Take each run's furthest phase from the in-crate witness-driver run of the same input, mode and build, which sees `Ran::Fallback(cause)`. The outcome is deterministic, as I81's controls show. Record it beside the challenge's peak in RSS_TIME.md.<br>• Any finer bound uses the maximum of every phase up to the furthest one reached. |
| **A1-S-2** | SHOULD-FIX | §2 table, §2.2, §4 phase 2 | **SP's multi-case tests depend on I2, and the plan doesn't say so.**<br>• PLAN_v2 has `retained_w1` refuse with `Domain` when c > `LOAD_CASES` (the SF-2 amendment).<br>• `b1` keeps `caps::LOAD_CASES = 1` until SA merges at I2, and I2 comes only after SA and RV-Q round 1.<br>• Until then, every test with c ≥ 2 on the private driver returns `Domain`: W-C2, the multi-case fault tests, and the ordinal mapping. So does the Direct coexistence pin, since D1.4 still refuses c ≠ 1.<br>• The table lists I1 as SP's only dependency, and R3′ (SP's checkpoint after T-7) is meant to show per-case custody working. | • Show "SP's c ≥ 2 tests depend on I2", and schedule I2 before R3′.<br>• Or have SA land `LOAD_CASES` = 3 and D1.4 first, as an early I2a with its own RV-Q read.<br>• Either way the critical path gains SA plus RV-Q round 1 ahead of SP's multi-case work. |

## Notes

| ID | Where | Note |
|---|---|---|
| A1-N-1 | §2.2, N-1 | `retained_tests_hooks::before_precommit` doesn't hand the successor to the test today. It only applies the `corrupt` fault (`lib.rs`). SP needs a small `cfg(test)` capture of the successor in the hooks module. That module is in `lib.rs`, inside SP's fence, and the capture should be carried across the reserved-stack hop like the other hooks. The rest of N-1 is placed as ruled. |
| A1-N-2 | §2.1, decision 22 | **The seam must record no adapter event.** `product_attempts[].adapter.counts` is serialized into the receipt (`retained_wire.rs`), and the capture's other state writes are each preceded by `capture_entry(AdapterEvent::MapWrite)`. A MapWrite for `late_loads_total` would change every c = 1 successor's bytes. The precedent for skipping it is gate state such as `late_refusal`, which is written with no event. Say so in §2.1, and say that SP's c = 1 adapter counts stay identical as part of byte identity. |
| A1-N-3 | §3.3, N-8 | **The list of tests SQ re-pins is incomplete.**<br>• `profile_laws_hold_in_this_build` holds a third 0.9 M margin pin (`3_623_878_656`), `caps.complete[3] == 9_361` (D_env), `text_atoms::ROW == 11_474`, and the atom-binding counts.<br>• `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` holds literals for `P_FINAL` 2,115, `TEXT_TEXT_DIAG_ENV`, PushCap(D_env), L_PUB, L_DIAGID and (3m + 1)·Text(err).<br>• `structural_budgets_are_u3s` asserts B-6 ≤ 2,048. Today's `NOTICE_RESERVE_BYTES` is about 886 B (Diagnostic 144 + 340 + 250 + 24 + 128), so × 3 is about 2,658 B, and the assertion would fail.<br>Either name these tests, or state the rule: every law-test literal derived from M, the caps or the profile is re-pinned, as an expression in SA or as a value in SQ. |
| A1-N-4 | §4 | **Merging into a worktree someone is working in.**<br>• `b1` is SP's live worktree, and I2 and I3 merge into it. ROOT should merge only at a commit point of I-P's (I-P commits and pauses), never over uncommitted work.<br>• In phase 4, run G5-early on an archive of a named `b1` commit, as SG and SB do. Then it doesn't share `WT/b1` with I-RS's harness-pin edits, which build RE.<br>• SC commits on `b1` directly, so "SC is merged" at I5 should read "SC's commits are on `b1`".<br>• §8 risk 14's integration-conflict stop could also be listed under R8. |
| A1-N-5 | §2.0 item 2, decision 25 | **The support cap limits construction (c).** Each milestone copy uses 2 nodes, 1 member and 4 supports (1 rigid and 3 springs at N0; checked in the request). At most 7 copies leave at least one support for the filler bodies. With 7, the filler has 18 nodes, 25 members and 4 supports, so it needs a few anchored, connected bodies with chords. Reaching 128 loads stacks about 18 moments per copy onto its three rotational DOFs at N1.<br>Record which typed caps the publishing input actually reaches. SW runs main's one-case producer, so item 2 is c = 1 only; the three-case W3 measurement depends on item 3's cases selecting on item 2's model once SP exists. |
| A1-N-6 | §2.2, decision 23 | Define the hook's `index` as the **request** index (in W-C2, 2 is case C). Extend `Armed` (in `lib.rs`), `grant2::merge` and `grant2::names` too, so that `armed_names()` and the "every armed fault fired" assertions see the new fault. Both files are in SP's fence. |
| A1-N-7 | §2.2, decision 26 | n05 with a second case may end at T-3 (b), `SOURCE_BLOCKS_FINALIZATION_FAILED`, rather than at Coexistence. s11g's `t20_characterization_rb_prime_residual_c1` shows a two-case captured invocation refused fail-closed on the legacy route. The stated fallback covers this; record which outcome occurred. The pin runs through Direct at c = 2, so it also needs I2 (A1-S-2). |
| A1-N-8 | §4 phases 1–2 | SW is a phase-1 implementer. If it runs into phase 2, beside SP, SA and SR-RS, it is a fourth. SR-RS should then wait. |
| A1-N-9 | §3.5, §3.6 | **Measurement details.**<br>• For RSS, the challenge needs one test entry point for each input and mode, not one per mode.<br>• Its counting allocator does SeqCst atomics on every allocation, which slows the dev/test timings. RSS_TIME.md should say so. Release times come from the witness tests, without that allocator.<br>• The host has no swap (`memguard.sh` header). PLAN_v2's host statement already covers this. |
| A1-N-10 | §6, N-14 | **I84's B6 fixture remark.** Of the "two fixtures", the corpus (`retained_precision_cases.json`, 07m) is B6's mandated output. Only `retained_precision_carrier_cases.json` departs from B6's brief.<br>• That file is read only by tests: RE's `retained_precision_carriers.rs`, pytest's carriers test, and six desktop tests, including T6S's export, output-policy and stress-neutral tests.<br>• It is not one of the 13 D-6 reviewed statics.<br>**It does not affect PLAN_v2:** B1 does not touch it, and SR starts from I1′ after B6 merges. B6's own full suite, with the T6S suites unchanged, is the check, and B6's reviewer judges the change itself. |
| A1-N-11 | §1 "The registration" | PLAN_v2 says the registration changes `LOAD_CASES` and `TOTAL_LOADS`, but SA sets them in `b1-a`, which reaches `b1` at I2. The registration diff is `threshold_bytes`, the generated block, and §3.3's re-pins. The same wording was in v1, and I missed it then. |

## 1. SF-1 to SF-7, as ruled

| Finding | Status | Evidence |
|---|---|---|
| **SF-1** | **Resolved** | **Implementers per phase:** phase 1 has two (ST and SW); phase 2 has three (SP, SA, SR-RS); phases 3 and 4 never exceed three, with SR-PY and SR-TS taking slots as they free, and a repair round takes its lane's slot back. A1-N-8 covers SW overrunning.<br>**Worktrees:** `b1` for ST, SP, SC and SQ, used one after another; `b1-a` for SA from I1; `b1-r`, `b1-p` and `b1-t` from I1′; archives for SW, SG and SB.<br>**Integration:** I1, I1′ and I2 to I5, with `--no-ff` merges and no rebase. ST comes before SA, and SA is split from SP by file. The two lanes share no file except the seam's two fields, which ST adds.<br>See A1-S-2 and A1-N-4 for the mechanics |
| **SF-2** | **Resolved** | **Seven oracles plus the D1.4 line:** four in the facade tests at `retained_memory::caps::LOAD_CASES + 1`. That constant is visible inside the crate, since `caps` is `pub(super)` in a private module of the crate root. The other three are in the law tests, `retained_memory.rs` and the runner.<br>**The runner** uses a literal 4, with a PP-side test tying it to `admit`'s D1.4 refusal. No visibility change is made; this is on the never-touched list and is an R8 stop.<br>**`retained_w1`** refuses c = 0, c > C and any combination, with a mutant. That refusal causes A1-S-2 |
| **SF-3** | **Resolved** | **The fence** gains `s11f_site_test.rs`, with rule 8's `TABLE` rows and their dispositions reviewed by RV-P. It also gains PP-tests' `retained_precision_admission.rs` and `grant2.rs`.<br>**The guard list** is complete, inside and outside `src`. New accumulation sites go into s11f's table, or use shapes outside rule 8 |
| **SF-4** | **Resolved** | **The seam** is `ProductCapture.late_loads_total` and `CompleteFacts.requested_cases`, added in ST.<br>• The law tests build `CompleteFacts` literally at exactly seven sites, as the plan says.<br>• `s(ProductCapture)` is not a profile atom (the generated block names `s(ThreadPacketOutput)` and component types, not the capture), so the new field moves no atom.<br>• The facade's source pins (`u3_capture_permit_is_linear`'s substrings) survive the seam.<br>• `LATE_FACTS` goes from 9 to 10; `ordinary_solve_attempted(capture, requested)` takes the count, and its callers are all in SA's files.<br>• The form-valued tests assert expressions, and SQ re-pins their values.<br>See A1-N-2 (adapter event) and A1-N-3 (the re-pin list) |
| **SF-5** | **Resolved except A1-S-1** | **In place:**<br>• each run records its furthest phase;<br>• SW item 2 looks for a publishing input, with its stop and a plain statement that W3–W5 are unmeasured if none is found;<br>• the counting-allocator peak is recorded beside RSS;<br>• `CAP_BYTES` goes to 16 GiB;<br>• one mode per process;<br>• the host, build and RSS statements;<br>• R9, after RV-Q passes SQ and before PR-B1 merges.<br>**Defective:** how the bound is chosen, and how the phase is observed from the challenge (A1-S-1) |
| **SF-6** | **Resolved** | The src-tauri suite is in §3.9, in gate items 3 and 6, and in the package, using U9 G7's method. B6 is rightly exempt, because it touches no PP `src` |
| **SF-7** | **Resolved** | RV-X is a fresh reviewer of the whole PR diff against main, in addition to the slice reviews, with the same reviewer confirming each repair, and RV-P's ledger as its input. Decision 16 is recorded as overruled |

## 2. N-1 to N-16

Each note is placed as ruled:
- N-1 to N-3 and N-5 to N-9: in their slices and §4;
- N-10: in §10;
- N-11: in §0, decision 21 and §11;
- N-12: in §9 (STUDY §3.2's `t_c3_m12` and `t_c3_k20_l128` exist);
- N-13: in §2.3;
- N-14: in §6, and Basis;
- N-15: R9;
- N-16: in §2.2, §3.1, §5, R8 and risk 13.

Two are placed but inaccurate or incomplete:
- **N-1:** the hook it relies on does not expose the successor today (A1-N-1).
- **N-8:** the re-pin list misses three law tests (A1-N-3).

## 3. Decisions 22–26

| # | Verdict | Reason |
|---|---|---|
| 22: the seam lands in ST before SA forks | **AGREE**, with A1-N-2 | It is the smallest edit that lets SA fork from I1 with both fields present. It is behaviour-neutral at c = 1, provided the field records no adapter event: neither field is an atom, and the seven literals and the source pins check out |
| 23: `fail_preparation_of_case(index)` in `grant2.rs` | **AGREE**, with A1-N-6 | It is test-only (`retained_tests_hooks` is `#[cfg(test)]`). Today's `fail_next_preparation` would fail W-C2's case A. The index should be the request index, and `Armed`, `merge` and `names` must learn the new fault |
| 24: `CAP_BYTES` at 16 GiB; the bound chosen by the furthest phase | **DISAGREE as written** (A1-S-1) | **The cap is right:** 16 GiB is above any admissible priced need (0.9 × 12 GiB ≈ 10.8 GiB), within T3's 64 GiB host allowance, and far from `memguard.sh`'s floor (it kills T3 jobs only when available memory drops below 35 % of 128 GB). **The bound selection is not right:** the phase can't be seen from the integration test, and the bound must be the maximum of the phases up to the furthest one, not that phase alone |
| 25: SW's publishing input, built by replicating the milestone body | **AGREE**, with A1-N-5 | The milestone publishes, and unloaded restrained bodies give exact zero rows (L = 0, W-C2's case A). It is a sound probe direction with a stated stop. The support cap allows at most 7 copies, and the record must state which caps the input reaches |
| 26: the coexistence pin on n05 with a second case | **AGREE**, with A1-N-7 | It is the natural committed pin for CR §5.2's no-attempt rule, with a fallback. It may end at T-3 (b) instead, and through Direct it needs I2 |

## 4. The new stops, and R2's retirement

**R8's additions are sound:**
- a D1 visibility change (the SF-2 rule);
- a batch outcome that differs from PROBE's (N-16).

R8 now holds an FK, schema, base-reader, reviewed-input or visibility change; a c = 1 byte change; an unexplained sweep row; a batch outcome differing from PROBE's; and §9's contingency.

**The other ruling points:**
- R3 is ST's checkpoint, before I1.
- R3′ is SP's checkpoint, after T-7. It is well placed, once A1-S-2 is settled.
- R9 matches R1's wording.
- **R2's retirement is sound.** The cap ruling it held was made by RR "I82's addendum…" (option S3), so nothing is pending there.
- §8 risk 14's integration-conflict stop sits outside R8's list; A1-N-4 suggests adding it there.

## 5. The revised estimates

**The arithmetic holds:**
- Slices: 77–116 h. Repair rounds: 11–21 h agent. Total: 88–137 h.
- Review: RV-P 12–19 h, RV-R 8–12 h, RV-Q 12–20 h, RV-X 6–10 h, total 38–61 h. The repair-round review hours sit inside these totals, as the plan says.
- The critical path: 44–66 h of agent work and 25–41 h of review.

**The calibration claims are true in RR:**
- RV87 was NOT CONFIRMED twice: on S-2 in G5 part 2, and on the identifier audit in G6.
- RV89 routed items forward three times: at G5 part 1, at G5 part 2, and at G6 with the registration diff.

**Honest, as reading estimates.** The elapsed figure is probably conservative. Every RR heading from U4 G2 to G7, including both NOT CONFIRMED rounds, is dated 2026-10-04. U8 and T6S also each went from dispatch to merge in about a day. A1-S-2 may add SP's wait for I2 (SA plus RV-Q round 1, about 7–11 h), which fits within the stated range.

## 6. What else changed between v1 and v2

Every changed passage is consistent with R1, except:
- A1-S-1, introduced by SF-5's bound change;
- A1-S-2, introduced by SF-2's domain re-check;
- the notes above.

**I found nothing else broken.** In particular:
- §0's findings restate the confirmed facts correctly.
- §6's B6 file list matches `git diff --stat bfb26596bf a79dbd2e4a`: 11 files, none in D1 `src`.
- B6's gates are the product set without item 7.
- The owner-held list is unchanged and untouched.
- Every decider is ROOT, except M above 12 GiB, which is the owner's.

## Records

- `ADDENDUM_01.md` (this file)
- `SHA256SUMS.addendum_01`

Placeholder paths only. `REVIEW.md`, `evidence/` and `SHA256SUMS` are unchanged.
