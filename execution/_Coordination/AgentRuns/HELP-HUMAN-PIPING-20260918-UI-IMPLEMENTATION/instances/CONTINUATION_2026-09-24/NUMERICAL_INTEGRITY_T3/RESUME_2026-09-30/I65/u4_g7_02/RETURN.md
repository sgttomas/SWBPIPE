# I65 U4 G7: the Pass B repair, and Pass B on the final basis

**Basis.**
- ROOT's repair round, after RV87 (`R/REVIEW_RV87/u4_g7_01/`, PASS 0/1/4) and RV89 (`R/REVIEW_RV89/u4_g7_01/`, PASS 0/1/3).
- The final basis is **`7f07a2f7b413b37ecaa879f81b3d759a9cde7f13`** (tree `58e4b0fd…`), extracted read-only at WT/scratch/i65_u4_g7_01/final/.

**Verdict on the final basis: `DELTAS TO READ`, exit 6. The one delta, read: 6 new PP tests, all passing.**
- **The registered entry is byte-identical** to `0c7827b6ad`'s: the identity, the 14 inputs, the 4 layouts and the threshold 4,026,531,840.
- **Every other gate passes:** tree, entry, law, statics, line map, premise, delta, TEXT, FORMS, §11, controls, runner outcomes, witnesses and challenge.
- **The delta:** PP gains exactly six tests, all `ok`. They are grant 2's five `retained_facade_tests::u3g2_*` and D-U6-5's `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`, from the test file in the delta. No existing outcome changed.
- **PP:** 705 passed, 1 failed (t13), 10 ignored, as in ROOT's own run.

G7 changed no source.

## The repair, item by item (`_run_records/`)

1. **Production-delta inventory** (`delta_inventory2.py`, called by `g7_pass.sh`; RV89 S-1(a)).
   - **Classes.** Every changed file and hunk between Pass A's basis and the target gets an explicit class: `test`, `cfg-test-stmt`, `generated`, `no-code`, `unreachable` (with the reason: not on the lexical D1 graph, or reached only through `edge_zero`), `live`, `item`, `not-d1` or `data`.
   - **Stop.** A `live`, `item` or `cfg-test-stmt` hunk without a reviewed entry stops the pass (**exit 5**). Entries are keyed by fingerprint, in `delta_reviewed.json`.
   - **The final basis: 6 files, 15 classified rows:**
     - 11 `test`: 8 `lib.rs` hunks inside `#[cfg(test)] mod retained_tests_hooks`, `grant2.rs` (declared only inside it), and the two test files;
     - 1 `generated`: the T17_V4 line;
     - 3 `cfg-test-stmt`: lib.rs:2379, :3005 and retained_product.rs:3244.
   - **The run stopped (exit 5)** on the three statements (`runs/final1/`). Each now has a reviewed entry with evidence:
     - they are absent from every production build and from the challenge's binary, which links the non-test lib;
     - in the lib test binary, with nothing armed, the hooks only clone a thread-local `Option<Arc>` (a reference-count increment) and read and write a `Copy` `Cell`;
     - when armed, they make in-place assignments;
     - none allocates, adds text, or changes a production type. The law gate confirms the in-build record is unchanged.
   - **These entries are mine, made from the source,** for RV89 to confirm.
2. **One verdict, and a non-zero exit on any delta** (RV89 S-1(b), RV87 SF-1). `VERDICT PASS|STOP|DELTAS TO READ exit=<n>` comes from the gates' codes:
   - 2: tree;
   - 3: entry or law;
   - 4: line map or premise;
   - 5: delta;
   - 6: statics, TEXT (incomplete, `scc`, `self-recursion`, row or byte differences), FORMS, the §11 set ≠ the reviewed 410, controls < 12, outcomes, witnesses, or challenge.
3. **The entry and the gate** (RV89 S-1(c)).
   - **Entry:** the `REGISTERED_PROFILES` block, byte for byte against `0c7827b6ad`'s. That covers the identity, the inputs, the layouts and the threshold.
   - **Law:** the compiled identity, inputs and layouts must equal the entry; `0 failed`; and the 7 registered law tests must have run.
   - **FORMS:** the target's `retained_memory.rs` must equal its own regeneration from G7's `profile_tree.json`.
4. **D is read from the run's summary** (RV89 S-1(d)). Both the §11 dump and the controls take it (`TB_D_RUN`; final basis: 14,734).
5. **The dead-branch premise is pinned** (RV89 S-1(e), N-2). `premise_pins.json` makes retained_precision.rs:4252, :4253 and :4305 rule lines.
   - The line map stops (exit 4) if an edit touches them, and the premise gate checks their text (exit 4).
   - QUALIFICATION_G7 §1 now states the magnitude: dense W4 ≥ 3,749,527,510 B if the branch were live.
6. **Self-recursion** (RV87 N-1). The chain's `text_budget.py` fails the run (`self-recursion`) on a self-recursive text ancestor outside `loop_bounds.recursion_reviewed`. That list holds the 11 that G6 and G7 share, each with the reason its text is priced.
7. **Controls** (`pass_b_controls.sh` → `runs/pass_b_controls.out.txt`): **33 of 33 pass.** Each new stop fires on a mutated copy through the same gate:

   | Gate | Controls |
   |---|---|
   | tree | mutated and added files |
   | entry | threshold doubled; identity edited; a layout edited |
   | law | a failed test; a Stale identity; a layout; a missing registered test; an empty log |
   | premise | the projection keeping the successor id; `validate` passing the unprojected source |
   | line map | rule lines in edited hunks |
   | delta | U6's F5 live hunk with no entry → 5 (the tool classes the whole U6 delta as Pass A did) |
   | statics | one added |
   | FORMS | Pass A's block without the line |
   | TEXT | one W row raised; an incomplete `scc` run |
   | §11 | one new non-candidate |
   | controls | one failing |
   | outcomes | one test flipped |

   The pass-side controls are now 12: Pass A's 11, plus c11, self-recursion unreviewed. With them, each new gate also has its passing case.
   - **Self-test on Pass A's basis:**
     - with the adopted T17_V4 line as an overlay: **PASS, exit 0**, every gate 0 (`runs/selftest_ov/`);
     - without it, exactly as `ba1faa1c`: exit 6, with only `forms` (`runs/selftest_noov/`), the documented pre-proposal state.
8. **QUALIFICATION_G7.md** (u4_g7_01; its SHA256SUMS line is updated) gains these sentences:
   - RV87 N-2's in §4.1 (the check is only as complete as the call graph), with the self-recursion stop;
   - RV89 N-1's in §3 (the producer, retained_wire.rs:1426–1429, is the refs bound's warrant; uniqueness alone allows D_env + 1, at most 8 B);
   - RV89 N-2's magnitude in §1;
   - RV87 SF-1's correction in §4.2;
   - §6 now points here.
   
   RV89 N-3 (the generated header) is left as reviewed.

## Pass B on the final basis (`runs/final/`)

| Gate | Result |
|---|---|
| tree | 2,950 of 2,950 blobs equal `7f07a2f7b4` |
| entry | **equal to `0c7827b6ad`'s block**, threshold 4_026_531_840 |
| law | identity, inputs and layouts equal; 42 passed, 0 failed; the registered tests ran |
| statics | none added or removed |
| line map, premise | 0 rules moved (PP's hunks are line-for-line); 3 pins as reviewed |
| delta | 15 rows classified; the 3 `cfg-test-stmt` reviewed |
| TEXT | complete; D 14,734; every row and output **identical to Pass A** |
| FORMS | equal to the regeneration from G7's tree (the adopted T17_V4 line) |
| §11 | the 410, no new non-candidate |
| controls | 12 of 12 |
| witnesses / challenge | 9 of 9 / pass (peaks 3,541,898 / 2,252,863 B) |
| runner/headless | 85 passed, 2 failed, **identical to Pass A** |
| **PP outcomes** | **6 tests added, all `ok` (the one delta)** |

**The in-build maxima are unchanged:** 0.8881 M sparse and 0.8929 M dense; 48,100,370 / 28,389,922 B under 0.9 M (`runs/final/law_record.txt`).
- `price_delta.out.json` adds F5 on top of a tree that already carries it, so its V4 figure counts F5 twice. That is conservative: V4 stays 1,131,620,001 B under V2_hash.

## Execution record

- **Who and when:** I65, TASK (Type 2) under ROOT, no descendants; 2026-10-04.
- **Memory guard:** PID 5387 was running, and every job checked it.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses and the challenge), one cargo job at a time; targets in WT/targets/i65_g7/work*-pass_<tag>.
- **Writes:**
  - this folder;
  - u4_g7_01's QUALIFICATION_G7.md and its SHA256SUMS line;
  - WT/scratch/i65_u4_g7_01/: `s2/`, `pass_*`, `ctlB/` and logs.
  
  The extracts and WT/f2a-memory were only read.
- **Not run:** no Git writes or index operations (`GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs.
- **Records:** placeholder paths only (WT). `SHA256SUMS` covers this folder.
