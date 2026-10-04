**piping(T3): F2a D1 milestone: retained-precision successor through the Direct entry, with reader eligibility**

This PR brings T3's first retained-precision (F2a) code to main.

The synthetic skew cantilever RF-SKEW-T-CANT-OFF-122-r1e-04 now publishes an M03-INTEGRITY-MP-v2 successor:
- through the product-physics Direct entry, in both solver modes;
- in the registered dev/test build only;
- in agreement with its independent reference.

The Python, Rust and TypeScript readers now treat such a statement as numerically eligible when it is supplied with its actual invocation. In TypeScript, the live native capture must also hold (a declared difference).

**This is an intermediate F2a PR. It does not complete F2a,** and it adds no product caller.

## What is public

- **The Direct retained entry** publishes a successor only for in-domain (D1) requests: one load case, no combinations, the preview family, no pressure, and counts within D1's caps.
  - It does so only in the registered dev/test build: aarch64-apple-darwin, rustc 1.97.1, debug, opt-level 0, debug assertions, panic=unwind, no RUSTFLAGS.
  - M = 4,026,531,840 B, selected under D-7.
  - Every other request keeps the ordinary route. A request whose retained work ran and fell back carries one unavailable notice.
- **The Headless entry** is refused.
- **Reader eligibility** in all three languages. **The carriers** transport the successor, and the desktop export panels refuse it explicitly.

## What stays closed

- **Public activation.** No desktop, native or CLI caller publishes successors. The desktop calls only the ordinary wrapper. Activation has its own checklist and a fresh review (CHANGE_RECORD §4).
- **Every other build is Stale and keeps the ordinary route,** including release builds and hosted Linux CI. **Hosted CI therefore exercises only the ordinary route.** The registered path is evidenced on the owner's Mac (DEC-025, Pass B, the controls).
- **No supported-machine statement of M** is made; that stays with the owner. Registering any further identity must re-establish the milestone's bytes and verdicts on it. Retained values use the platform `hypot`, and dense ordinary bytes are target-dependent, so the U1 ordinary-bytes pin is asserted on the registered target only.
- **Still open:** U8's deferred witnesses, the rest of F2a, S-I, F2b per domain and F3, and the owner-held items (CHANGE_RECORD §5).

## Source and packaging

- **Source.** 139 maintained files under `projects/chirality-piping` (59 added, 80 modified). They are byte-identical to the integration head NUM `bb3d766379`, except two files main also changed, each equal to its recorded three-way merge:
  - `core/analysis_runs/compatibility.py` keeps a single `_same_canonical` helper (byte-identical to main's #1078) and takes main's two call-site changes;
  - `core/reporting/result_export/src/source_blocks.rs` merges cleanly: main's #1080 plus this PR's 32-bit bound.
  - `source_equality.py` proves this: 5/5 PASS.
- **After the cut,** four commits carried five source changes found by hosted CI and RV95: the CI policy, the 32-bit bound, the U1 pin scoped to the registered target, RV95 N-3 and RV95 S-1 (CHANGE_RECORD §3).
- **This branch is cut from main.** It carries none of the integration branch's history, which stays on `codex/piping-numerical-integrity-20260926`. Its evidence package was regenerated at the freeze.
- **Records committed:** 10 files, 192,326 B including `SHA256SUMS`, all in `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/F2A_D1/`.
  - **Why:** the change record, the two verification scripts, the citation index, and four small records that maintained comments depend on. One of them is the generator that rebuilds the memory profile block.
  - The integration branch's 5,677 record files (191.4 MB) are not brought. Maintained-source citations of them resolve through `citations.json` to commit-pinned URLs.

## Reviews

Every unit had a fresh independent review, and each repair was confirmed by the same reviewer:
- U1–U2: RV82;
- U3: RV85 and RV93;
- U4: RV83, RV84, RV87 and RV89;
- U5: RV86;
- U6: RV88, RV90, RV91 and RV92;
- U7: RV94;
- the Pass B qualification: RV89;
- earlier components: RV68, RV72, RV74, RV77 and RV78–RV81.

**RV95 reviewed the complete PR diff: PASS** (0 blocking, 2 should-fix, 7 notes).
- Its S-1 is repaired.
- Its S-2 is this regenerated package.
- Its confirmation runs on the frozen head.

These are agent reviews, not personal review by the owner. Details: [CHANGE_RECORD.md](`…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md`).

## Gates

| Gate | Result |
|---|---|
| T9 | 112/112 identical, plus the 2 milestone outputs, which are the ordinary bytes; extra corpus 16/16 |
| Both-entry gate, part 1 | 884/884 identical; gate_check PASS, with 0 trusted breaches |
| Both-entry gate, part 2 | 4 dense runs in 45–53 s (limit 1,800 s), with identical envelopes |
| src-tauri suite | 116 = 116 |
| Pressure and coexistence controls; the 324-output sweep | PASS; the sweep is byte-identical, registered and Stale |
| Full Pass B (RV89 confirms) | Only the six added tests differ; the entry and maxima are unchanged |
| `source_equality.py` and `check_citations.py` | 5/5 and 368/0/0 PASS |
| GEN-8 at the cut | 1 passed |

**On the frozen head:** hosted CI with the full-SHA dispatch, GEN-8, the Mac baseline with DEC-025, the native witness, the frozen-head Pass B and RV95's confirmation are recorded in the post-merge record (`…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/F2A_D1_MERGE/`, on the integration branch).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
