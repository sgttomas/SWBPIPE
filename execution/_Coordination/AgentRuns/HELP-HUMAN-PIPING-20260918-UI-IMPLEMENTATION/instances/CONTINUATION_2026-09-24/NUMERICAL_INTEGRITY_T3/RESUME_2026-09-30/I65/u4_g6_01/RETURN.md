# I65 U4 G6: return

**Status.** G6 is complete. The qualification record is `QUALIFICATION.md`, the identifier audit is `ID_CLASS_AUDIT.md`, and the registration change is prepared in `registration.diff` but **not applied**.
- **Code:** WT/f2a-memory, on top of `cba3e9fda7`, uncommitted: +294 / −185 in 5 files, all inside the G6 fence (`_run_records/candidate_g6.diff`).
- **The registry is still empty** in the worktree.

## The numbers

| | Sparse | Dense |
|---|---|---|
| E_req,max (W3, Direct) | 3,318,504,979 | 3,338,215,427 |
| E_mov,max (W3) | 3,507,808,260 | 3,527,518,708 |
| E_mov,max + R | **0.8878 M** | **0.8927 M** |
| Below 0.9 M | 48,961,532 B | 29,251,084 B |
| Text-error budget (margin / TAV_W) | 3.12 % | **1.86 %** |

- **After both the 42 closures and the identifier audit, the maximum is still ≤ 0.9 M.** So no stop, and the phase-aware span stays in reserve.
- The release build's record is byte-identical to the dev/test build's.
- **Headless** is refused at D1.0, so it has no admitted maximum.

## Each item, separately

1. **The 42 Estimates are closed.** ESTIMATES = 0: 190 InBuild atoms, 16 SourceUpper, 6 Text. Each is bound to the actual owner its roster line denotes; the table is in QUALIFICATION.md §2. Closing them adds +9.77 MB to the maximum. The large ones are `TrackerE` (4,472 B, against G3's 64; P3's Tracker(O) law), `LazyE` (4,304), `LaneTerminal` (4,168), `RegistryE` (1,608) and `PreparedMemberEvent` (1,304). The C2 maps, conversions and lane terminals are bound conservatively, as double counts.
2. **The identifier-class audit (ROOT's addition):**
   - 747 copy entries over 599 sites, each priced by source: input 128, result id 1,024, diagnostic id 2,330, own template, composite 600, or static. 72 entries are raised and none is lowered.
   - **The run enforces it:** an identifier hit outside the table makes TEXT incomplete, and a removed-entry control proves it.
   - RV87's 18 sites are priced exactly at RV87's bounds (+32,373,120 B). Beyond them, the audit raised:
     - rows.rs:370 (a ResultItem id);
     - `projection_id`/`functional_id` (≤ 188/199);
     - lib.rs:1763's entries (≤ 8,241);
     - 3 `diagnostic_ref` copies to 2,330, plus RV87 N-1's 5;
     - the force-scaling quantity (145) and the pressure refs join (148);
     - 40 error-`Display` fields to the static class.
   - **TEXT** becomes 2,149,902,046 B (+33.3 MB: W +4.3 MB, X +33.3 MB), and is complete.
3. **The build identity:** register **one**, the dev/test build on this host:
   - aarch64-apple-darwin, rustc 1.97.1 (`8bab26f4f68e`);
   - `profile=debug`, `opt_level=0`, `debug_assertions=true`, `panic=unwind`, no RUSTFLAGS.
   
   That is the build that `cargo test` uses for U3 grant 2's milestone E2E, U7's facade publications and U9's gates on this host. **The release identity is qualified** (identical record; all nine witnesses pass) but not proposed, because nothing named is known to run a release PP build here. Hosted Linux CI is `Stale`, so it is fail-closed.
4. **The pinned record is identity-gated** (ROOT's ruling): it asserts only in `PINNED_RECORD_IDENTITY`'s build and prints a skip elsewhere, as seen in the release build.
5. **The qualification record** has every brief §4 item:
   - E_req and E_mov per caller and mode;
   - R and the S1 evidence (measured, not a proof), with the 40-frame chain (RV83 N-3), carry 8's limit and the panic-hook residual;
   - the identity and reviewed-input texts and the reader layouts;
   - the D1 predicate;
   - the D-7 non-claims (statics counted in every phase);
   - the cfg(test) argument: the production build's evaluation equals the record.
   
   **Proposed M = 4,026,531,840 B.**
6. **The registration change** (`registration.diff`, 4 files):
   - **the entry**, with `threshold_bytes` M;
   - **`admit_grants_a_permit_for_the_milestone_in_the_registered_build`**: both modes; Headless refused; a two-case variant refused;
   - **`the_registered_profile_is_the_only_permit_source`**;
   - **seven "no permit" tests restated** for a registered build. Two of them are in files outside U4's fence (`retained_facade_tests.rs`, `tests/retained_precision_admission.rs`), as proposals for ROOT to apply.
   
   **Applied in a scratch copy:**
   - **PP:** 696 passed, 1 failed (t13).
   - **Sweep:** every report goes `Missing → Registered`. The milestone publishes a successor in both modes, peaking at 3.5 / 2.3 MB. The two `rejected_stress_range` fixtures gain U3's N1 notice, because they are admitted and their W1 falls back.
7. **RV89 on part 2:**
   - **S-1:** the five result-id copies are RES by source type.
   - **S-2:** `witness_w2_deep_milestone_publishes` is committed. It publishes at 4 MiB and 1 MiB in both modes and both builds. W2 and W2b now assert `Fallback(Preparation)` and `Fallback(Candidate)`.
   - **S-3:** `maximum_takes_every_phase`; Q10 is in the mutant set and is killed.
   - The margin standard is stated above.
8. **RV83 N-3:** 40 frames, carried. N-1 and N-2 are optional and not taken.

## Controls

**Unregistered, against base `8abb5274a9`:**
- **Sweep:** byte-identical (`0690bc64…`).
- **PP:** 695 passed, 1 failed (t13), 11 ignored. Only the U4 tests differ from base.
- **runner/headless, FK and SR:** identical to base.

**Mutants:** **162 run, 158 killed by a test, 0 compile-only** (`controls/mutants_g6.out.jsonl`). The sets are part 1's 84, RV89's 24, part 2's 40 and G6's 14. G6's 14 are nine FK exports, four profile bindings and RV89's Q10, which `maximum_takes_every_phase` kills. **The 4 survivors are all recorded:** – "build_status ignores bindings" is equivalent by decision 7, since nothing is registered in the worktree; – RV89's V19 (build.rs) cannot be observed in-crate; – "Estimate count ignored" and "estimates not counted" are equivalent now that ESTIMATES = 0. `priced_maximum(1, ·)` is still tested as `Unpriced`

## Decisions for ROOT

1. **Register the dev/test identity only?** Or also the qualified release identity: one more entry, and its record and witnesses are ready.
2. **Before applying the registration, two consequences follow from D1's design** (QUALIFICATION.md §8):
   - **Admitted-but-blocked requests change bytes.** D1 admits Direct requests whose ordinary run is blocked (an invalid category or document kind; `rejected_stress_range`). Under registration, these publish U3's W1-unavailable fallback: the ordinary envelope plus one `RETAINED_PRECISION_UNAVAILABLE` info notice. They are no longer byte-identical to the value route.
   - **The permitted path's single ordinary dispatch (U3's B-1) is uncounted.** No test hook counts it; that needs a lib.rs hook. **Proposed for U3 grant 2.**
3. **Two hunks of `registration.diff` touch tests outside U4's fence:** `retained_facade_tests.rs` and `tests/retained_precision_admission.rs`. ROOT applies them with the entry, or routes them to their owner.

## Execution record

- **Who and when:** I65, TASK (Type 2) under ROOT, no descendants; 2026-10-04.
- **Memory guard:** PID 5387 was running throughout, and every cargo job checked it.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses and the challenge), one cargo job at a time.
- **Writes:**
  - the five code files in WT/f2a-memory;
  - this folder;
  - WT/scratch/i65_u4_g6_01/ (the TEXT runs, the candidate, registration and mutant copies, logs);
  - WT/targets/i65-g5/ and WT/targets/i65-g6/.
- **Not run:** no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs. The Python is stdlib only.
- **Records:** placeholder paths only; a check for machine paths finds none. `SHA256SUMS` covers this folder.

## Addendum: ROOT's rulings on G6 (RR, after G6's return)

**Ruling 1: register the dev/test identity only.** Done.
- `registration.diff` registers that one identity.
- QUALIFICATION.md §5 records the release identity as qualified, ready and unregistered. It becomes a one-entry change when a release run is named.

**Ruling 2(a): G-C declines W1 when the ordinary route did not attempt the case's solve.** Done, inside U4's fence (QUALIFICATION.md §8a).
- **The fact** is `PhaseFact::OrdinarySolveNotAttempted`, the last of G-C's 19 facts, with a bound of 0. A refusal there is `W1Fallback::CompleteGate`: the ordinary bytes are exact, and no notice is rendered, because the notice exists only inside `retained_w1`.
- **The predicate** is `ordinary_solve_attempted(capture)`: the capture has at least one seed, and every seed has `initial.is_some()`. It reads the observer's own G-b record, which is set exactly at the attempt (lib.rs:4125–4148) or at an `Ok` attempt's report (:4543).
- **It cannot read `blocked_envelope`.** That function is returned both before and after an attempted solve (`solver_blocked`; post-solve `has_blocking`), so the envelope's status is ambiguous.
- **The predicate is unambiguous for every ordinary outcome inside D1, so there is no stop.** The exits before and after the attempt are cited in §8a and in the function's doc comment.
- **The examples, checked in both modes:**
  - **Declined** (exact bytes, no notice): invalid `document_kind`, invalid load category, no supports, and a lone spring (`SOLVER_SYSTEM_BLOCKED` before the attempt).
  - **Proceeds:**
    - the milestone (Sensitive), which publishes;
    - a 1e-300 spring (the attempt fails `NumericallyUnresolved`), which reaches W1;
    - W6 (the attempt fails, then W2 publishes);
    - **`rejected_stress_range`.** Its solve ran Sensitive, and only the legacy source-block finalization then blocked the envelope. By your rule it proceeds and keeps U3's N1 notice on fallback. **Note this one:** of the examples I named, it is the one whose bytes still change under registration.
- **Tests:**
  - `g_c_declines_w1_when_the_ordinary_solve_was_not_attempted`, in the worktree (both modes, both groups, and the predicate's empty and unset-seed edges);
  - `registered_g_c_declines_only_unattempted_solves`, in `registration.diff`. In the registered build, for each blocked example, Direct's bytes equal the value route's exactly, with `CompleteGate(OrdinarySolveNotAttempted)`. The milestone publishes, and the failed attempt and `rejected_stress_range` reach W1.
  - The admission integration test's invalid-document case now expects exact bytes in every build.
- **Mutants:** 5 new ones for the fact (§9). The bound raised to 1, the fact zeroed, empty seeds counted as attempted, a seed alone counted as attempted, and only a reported solve counted as attempted are all killed.

**Ruling 2(b):** B-1's single-dispatch count goes to I61's U3 grant 2, as recorded.

**Ruling 3:** the two out-of-fence test hunks stay in `registration.diff`, to be applied with the entry.

**Re-run after 2(a):**
- **Unregistered candidate:**
  - the sweep is byte-identical to base (`0690bc64…`);
  - PP: 696 passed, 1 failed (t13), 11 ignored;
  - runner/headless: identical to base;
  - law suite: 39 passed in both builds;
  - nine witnesses pass in both builds;
  - the profile and record are unchanged, at 0.8878 / 0.8927 M.
- **Registered copy:**
  - PP: 698 passed, 1 failed (t13);
  - the milestone publishes in both modes (challenge peaks 3.5 / 2.3 MB);
  - every retained report is `Registered` (140 of 140 parsable);
  - the sweep sha256 is unchanged, `3b22de97…`, since no sweep fixture is an unattempted solve;
  - the blocked examples are byte-identical (the test above).
- **`registration.diff`** is regenerated: 4 files, 287 lines.
- **Mutants:** 167 run, 163 killed by a test, 0 compile-only. The 5 new G-C mutants are all killed, and the 4 survivors are unchanged and recorded.

## Addendum 2: the pre-registration repair (RV87 SF-1 to SF-3, N-1, N-2; RV89 S-1 to S-3)

**Basis.**
- RR "U4 G6 committed unregistered as `2bb81ec1ea`; RV87 did not confirm the identifier audit; repair before registration" (`R/REVIEW_RV87/u4_g6_01/REVIEW.md`).
- RR "RV89 on U4 G6 with registration.diff: PASS; three items join the pre-registration delta" (`R/REVIEW_RV89/u4_g6_01/REVIEW.md`).

**Status.** The repair is done (at return, uncommitted in WT/f2a-memory on top of `2bb81ec1ea`; ROOT has since committed it as `b43378d90a`, which equals `candidate_g6r.diff`, RV89 G6r N-3): 3 files inside the fence, +142 / −14 (`_run_records_g6r/candidate_g6r.diff`). `REGISTERED_PROFILES` is still `&[]`. `registration.diff` is updated, and the reviewed G6 version is kept as `registration.g6.diff`.

**The numbers after the repair** (the in-build record, `_run_records_g6r/per_identity/`):

| | Sparse | Dense |
|---|---|---|
| E_mov,max (W3) | 3,508,669,422 | 3,528,379,870 |
| E_mov,max + R | **0.8881 M** | **0.8929 M** |
| Below 0.9 M | 48,100,370 B | 28,389,922 B |
| Text-error budget (margin / TAV_W) | 3.06 % | **1.81 %** |
| Change from G6 (every W phase / X phase) | +861,162 / +845,828 | the same |

- **The maximum is ≤ 0.9 M,** and it matches RV87's estimate (0.8929 M dense). No stop.
- The change is TAV_W +861,146 and TAV_X +845,812, plus 16 B in every phase. The 16 B is `s(ThreadPacketOutput)` (1,688 → 1,704): the admission report inside `RetainedPreviewOutput` gains S-3's `required: Option<u64>`. No other atom or form moved.
- The release record equals the dev/test record apart from the identity and the identity-gated skips.

### Each item, separately

**SF-1: the node-DOF label is priced at its source bound.**
- The five format sites (`lib.rs:1632`, `:1644`, `:1708`, `:1721`, `:1739`) price `integrity_dof_label(..)` at 131 B: an input node id (≤ 128), `:` and a 2-B DOF name.
- An `integrity_dof_label\(` argument rule now precedes the integer rule (`_dof`), and each site is in the audit table (class TPLLABEL).
- The by-type enforcement found a sixth label site, `lib.rs:1149` (`formation_check`'s `global_dof` label, multiplicity 2), now priced the same way.
- **The `:1763` template bound** (TPL_ENTRY) is recomputed from the raised `:1699–1753` rows: 8,241 → **8,352** (the site 8,243 → 8,354).
- **Δ:** +620,934 B on RV87's rows, exactly RV87's figures, plus +444 B at `:1149`.

**SF-2: the adapter's `copy(s)` is priced by source.**
- G4's `lex_site_size` entry (class result_id, 1,024 B) is re-keyed from the stale `:2963` to its actual line, `retained_product.rs:3125`, and the copy is audited as RES. That is G4's intended class for all 199 copies, which ROOT accepted.
- **Δ:** 327 → 1,024 B, **+277,406 B** (RV87's own figure for that class).
- **The three `site_zero` keys that matched no row are removed** (`retained_product.rs:529`, `structural/formation_check.rs:524`, `structural/retained/assemble.rs:15`), and recorded under `site_zero_removed_g6` in `text_args.g4.json`. None changed any row.

**SF-3: the enforcement is by type, and it fails on stale keys** (`_run_records_g6r/text_budget.py`).
- **Every identifier-bearing candidate** at a positive-multiplicity site must be in the audit table, **whichever rule would price it**, or TEXT is incomplete (`id-unaudited`). A candidate is an expression that:
  - names an id, ids, ref, refs, name, key, label, suffix or identity, in any receiver or call; or
  - is a bare text parameter (`&str`, `String`, `impl Into<String>`, …) of its function, so `copy(s)` qualifies; or
  - is matched first by an identifier-class rule.
  
  This covers sites priced by `site_size`/`site_aggregate` (their audited evaluation then applies when larger) and by `lex_site_size` (the entry applies when larger).
- **A site-keyed rule matching no row fails.** That is any key of `site_size`, `lex_site_size`, `site_total`, `site_zero`, `site_from`, `site_aggregate` or `id_audit` (`stale-key`), or an audit entry naming no expression at its site (`stale-audit-entry`).
- **The enforcement found 48 candidates outside G6's table.** Each was read at its site and classified (ID_CLASS_AUDIT.md §2a):
  - 6 TPLLABEL (SF-1);
  - 1 RES (SF-2);
  - 10 IN128;
  - 24 STATIC (`&'static str` parameters and literal tables);
  - 7 NOTID (finding and error message text, and a JSON pointer).
  
  Only the SF-1 and SF-2 rows change; every other entry keeps its row's bytes.
- **Site overrides keep their floor.** Where a `site_size` override priced the site, the entry is the source bound and the override stays the floor. That applies to `source_receipt/source.rs:27` (188), `:51` (129) and `lib.rs:13834` (150).
- **The table converged in three runs** (`iter_g6r.sh`): the functional-id template bound read 2,233 B while `case` was unaudited, then 188; the third table equals the second.
- **Controls** (`_run_records_g6r/controls/audit_controls_g6r.out.json`). The unmodified copy is complete. Each of the following makes TEXT incomplete with exactly its own finding:

| Control | Finding |
|---|---|
| **c1: RV87's `primitive_loads/src/lib.rs:299` removal** (G6: complete, −370 B) | `id-unaudited` |
| **c2: a stale `site_zero` key** | `stale-key` |
| c3: `lib.rs:5592` removed (G6's control) | `id-unaudited` |
| c4: `lib.rs:1708`'s label removed (SF-1) | `id-unaudited` |
| c5: `retained_product.rs:3125` removed (SF-2) | `id-unaudited` |
| c6: the `copy()` key moved back to stale `:2963` (SF-2's original state) | `stale-key` |
| c7: an entry naming no expression | `stale-audit-entry` |
| c8: a STATIC entry removed (`lib.rs:8795` `label`) | `id-unaudited` |

- **Residual** (stated in the audit): the predicate is syntactic. A local alias of an identifier under a name with no token above, and not a text parameter, is not a candidate. A key that drifted onto another row on the same line is not detected.

**The TEXT delta** (`_run_records_g6r/text_g6r/`, against `text_g6/`; inputs otherwise byte-identical: part 2's edges and lexicon, G6's inventory and loop bounds):

| Run | G6 | Repaired | Δ |
|---|---|---|---|
| TAV (whole) | 2,149,902,046 | 2,150,800,830 | +898,784 |
| TAV_W | 1,569,180,716 | 1,570,041,862 | +861,146 |
| TAV_X | 1,439,555,190 | 1,440,401,002 | +845,812 |

- 8 rows change, all upward. No row is lowered, D and D_env are unchanged, and the run is complete.
- The whole-run Δ is SF-1 +620,934, plus `:1149` +444, plus SF-2 +277,406.

**N-1: the audit's scope names the copies priced elsewhere.** One paragraph in ID_CLASS_AUDIT.md's scope names the `TEXT_NAMES`-filtered data clones and non-literal `.into()` (RV87's eight sites) and the families that price them: row text in O (Text(row), 4 × 1,024-B refs) or T25, and diagnostic refs in Text(diag) at 152 B per ref.

**N-2: the `suffix` rows are labelled as input ids.** A provenance rule ahead of the STATIC literal rule classes `suffix = stable_suffix(<id>)` as IN128. That relabels 25 rows (RV87's rows 44–54 and the other `let suffix = stable_suffix(..)` rows). The bound is the same 128 B, and no row's bytes change.

**RV89 S-1: the runner's Headless profile flip is in `registration.diff`.**
- `runner/headless/tests/retained_precision_admission.rs`, `explicit_headless_refusal_…`: the expected profile is this PP build's own status, the one the Direct entry reports for the same input. That is `Registered` in the qualified build and `Stale` otherwise, never `Missing`. A runner test cannot read PP's build-script identity, so the Direct entry in the same build is the oracle.
- Headless itself stays refused at D1.0, and the output still equals the ordinary run's.
- The flip is applied with the entry under ruling 3.
- **Re-run registered:** runner/headless is now **identical to base**: 85 passed, and 2 failed, base's own two `load_reference` failures. `explicit_headless_refusal_…` passes with `Registered`. Unregistered it is identical to base too.

**RV89 S-2: the deferred-formation arm is pinned.**
- `attempted_examples()` gains K2a's `product-reach-partial-underflow` shape, built in the law tests as K2a's `PARTIAL_UNDERFLOW` request.
- `g_c_declines_…` asserts it is inside D1 and attempted, and that its one seed is `InitialSeed::FormationFailure`. `registered_g_c_…` (in `registration.diff`) iterates the same examples, so the arm is pinned in both builds.
- **RV89's R8 is now killed** (below).

**RV89 S-3: admission's bound adds R, and that is tested.**
- `admit` prices its bound through a named pure function, `admission_bound(maximum, threshold)`. It is `bound_admits(maximum?, RESERVED_STACK_BYTES, threshold)`: the constant R, never the `cfg(test)` stack override.
- The private law record keeps the bound's `required` bytes (`bound_required`: admitted or exceeded; `None` when unpriced, overflowed, or refused earlier).
- `admission_bound_adds_r_before_comparing_with_m` tests it in both builds:
  - at M − R − 1, M − R and M − R + 1, at M itself, Unpriced and Overflow;
  - on this build's own maxima, with the override set;
  - in `admit`'s source: one `admission_bound(..)` call, `required` recorded, and no direct `bound_admits`.
- In the registered build, `admit_grants_…` (in `registration.diff`) asserts `law.required == Some(cap_priced_maximum(mode) + R)`. In another build it asserts `None`.
- **RV89's R6, at its new home, is killed** (below).

### The re-run (`_run_records_g6r/controls/`, `registration/`, `per_identity/`)

| Check | Result |
|---|---|
| Unregistered sweep | byte-identical to base, `0690bc64…41e1` |
| PP (unregistered) | 697 passed, 1 failed (the Mac t13), 11 ignored. The only change from G6 is the new `admission_bound_adds_r_…` |
| runner/headless (unregistered) | identical to base |
| Law suite | 40 passed in both builds |
| Witnesses | all nine pass in both builds, one process each, with the same outcomes as G6 |
| `challenge_bounds_are_the_profile` / `profile_in_build_record` | pass in the pinned dev/test build; skip in release (`I65_G6_RECORD_SKIP`) |
| Registered copy (`registration.diff` applied with `patch -p1`) | PP: 699 passed, 1 failed (t13), 11 ignored. Challenge: the milestone runs the permitted path (peaks 3,541,898 / 2,252,863 B against 3,508,669,422 / 3,528,379,870). Sweep sha256 `3b22de97…0f60`, identical to G6's registered sweep. runner/headless: identical to base, with S-1's test passing |
| Mutants (`controls/mutants_g6r.out.jsonl`) | **89 run, 86 killed by a test, 0 compile-only, 0 not applied.** The sets are RV89's 24, part 2's 40, G6's 19 and the repair's 6 (`G6R`). **All 6 G6R mutants are killed:**<br>– RV89's R6 at its new home (`admission_bound` passes 0 for R);<br>– `admit` bypassing `admission_bound` without R;<br>– the bound reading the test stack override;<br>– `required` not recorded;<br>– an exceeded bound recording nothing (all five by `admission_bound_adds_r_…`);<br>– RV89's R8, a `FormationFailure` seed not counted (by `g_c_declines_…`).<br>The 3 survivors are G6's recorded ones: V19, and the two Estimate-count mutants, equivalent at ESTIMATES = 0.<br>Part 1's 84 were not re-run in full: their anchors are untouched by the repair (checked), and the first 12, run before the full run was stopped for time, are all killed (`mutants_g6r_full_partial.out.jsonl`) |

### Record changes

- **Updated in place:**
  - `QUALIFICATION.md`: a revision note at the head; §1, §3, §7 and §8; §8a's example; §9's pointer;
  - `ID_CLASS_AUDIT.md`: scope (N-1), result, method and enforcement (SF-3), the class table, the new §2a, §3, and §4 regenerated (795 rows, N-2);
  - `registration.diff`: 5 files, 323 lines, sha256 `976b722d…bcbf`;
  - this file.
  
  G6's sealed versions are at NUM `c4a1bcefa2`. `SHA256SUMS`' lines for these four files are replaced, and the new files are appended.
- **New:**
  - `registration.g6.diff` (the reviewed G6 package, sha256 `35c72703…4fd2a`, unchanged);
  - `_run_records_g6r/`: the repaired `text_budget.py`, `text_args.g4.json` and `mutants_g6.py` (`G6R` set); `audit_controls_g6r.py`; `iter_g6r.sh`; `run_g6r.sh`; `candidate_g6r.diff`; and `text_g6r/`, `id_audit/`, `controls/`, `per_identity/` and `registration/`.
- **Unchanged:** `_run_records/` keeps G6's sealed inputs and outputs. To reproduce the repaired run, overlay `_run_records_g6r/{text_budget.py,text_args.g4.json}` on a copy of `_run_records/`.

### Execution record (repair)

- **Who and when:** I65, TASK (Type 2) under ROOT, no descendants; 2026-10-04.
- **Memory guard:** PID 5387 was running throughout, and every cargo job and mutant checked it.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses), one cargo job at a time; `TMPDIR` in scratch.
- **Writes:**
  - three code files in WT/f2a-memory;
  - this folder;
  - WT/scratch/i65_u4_g6_01/;
  - WT/targets/i65-g5/ and i65-g6/.
- **Not run:** no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`; the sealed G6 inputs were restored from `git show HEAD:` into the working files), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs.
- **Records:** placeholder paths only.

## Addendum 3: the runner-test change before registration (RV89 G6r N-1), and records-only items

**Basis.** RR "RV87 confirms the identifier class closed…" and RR "RV89 passes the pre-registration delta; one runner-test change before registration". The code basis is `b43378d90a` (ROOT's commit of the repair, equal to `_run_records_g6r/candidate_g6r.diff`). WT/f2a-memory is unchanged, and `REGISTERED_PROFILES` is still `&[]`.

**RV89 G6r N-1 (gating): no runner test reaches W1.**
- **The change.** In `registration.diff`, runner/headless `explicit_headless_refusal_…` now reads the profile from the Direct entry on the **two-case variant** of `rf_skew_t_cant_off_122` (a second load case). D1.4 refuses it after the build clause, so the report still carries `Registered` or `Stale`, but no permit is granted, and the runner workspace (serde_json 1.0.151, not PP's reviewed 1.0.149) never runs W1. The test also asserts that the Direct bytes equal the value route's, that there is no successor, and that the census sees two load cases.
- **`registration.diff`:** 5 files, 337 lines, sha256 **`9ae2c889daeb8ec59c159c49b2f5eddf99472eaff7cd8d63bbd8263f909ce40b`**. Only the runner hunk changed. The previous version is kept as `registration.g6r.diff` (`976b722d…bcbf`), and G6's as `registration.g6.diff`. It applies cleanly (`patch -p1 --dry-run`) to WT/f2a-memory at `b43378d90a`.
- **The W1 check: a scratch-only permit probe** (`_run_records_g6r/registration/permit_probe/permit_probe.txt`). In the registered copy only, PP's `admit → permitted_dispatch` branch appended one line per permit granted to a file named by `I65_PERMIT_PROBE`. The whole runner/headless suite ran twice with it:
  - **with the previous S-1 test** (positive control): **2 permits**, sparse and dense, both for the one-case milestone. So the probe fires, and the build is `Registered`;
  - **with the new test: 0 permits** across the whole suite. No runner test is granted a permit, so none reaches W1.
  
  The probe was then removed (the copy's `lib.rs` equals the worktree's), and both suites were re-run clean.
- **Registered copy, clean** (`_run_records_g6r/registration/r2_reg_*.outcomes`):
  - **runner/headless: 85 passed, 2 failed, identical to base** (base's own two `load_reference` failures); `explicit_headless_refusal_…` passes;
  - **PP: 699 passed, 1 failed (t13), 11 ignored**, outcome-identical to the previous registered run.

**Records-only items** (optional, non-gating; no byte of TEXT or the profile moved):
- **RV87 G6r N-2:** `retained_product.rs:2332` and `:2351`'s `suffix` rows (`stable_suffix(&row.entity_ref)`, `:2223`) are relabelled IN128, and `:2351`'s `station_id_location(loc)` COMP. The bounds and bytes are unchanged.
- **RV87 G6r N-3:** `audit_key` and `id_table.py` strip `//` line comments from a placeholder's text, so editing a comment no longer fails the run. Only `lib.rs:1149`'s key changed.
- **RV87 G6r N-1:** the syntactic-predicate residual is recorded as a re-qualification obligation (QUALIFICATION.md §11).
- **RV89 G6r N-3:** Addendum 2's status line now notes the commit `b43378d90a`.
- **Re-run after these:** the whole TEXT chain (`run_text_part2.sh`) gives outputs **byte-identical** to `_run_records_g6r/text_g6r/` (all 13 files), so the profile and the pinned record are unchanged. All 8 enforcement controls still pass (`controls/audit_controls_g6r.out.json`, re-run). `_run_records_g6r/{text_budget.py,text_args.g4.json}` and `id_audit/` are the final versions.

**Execution.** I65, TASK (Type 2) under ROOT, no descendants; 2026-10-04. Memguard PID 5387 was running for every job, with one cargo job at a time, `--locked --offline`, `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`. No Git writes. The probe existed only in WT/scratch/i65_u4_g6_01/reg/.
