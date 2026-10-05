# RV89: independent review of U4 G5 (the admission law in code), part 1 now and part 2 later

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You did not write this code. Don't rely on the author's tests as your oracles.** You review part 1 now and part 2 when it lands, keeping your context between them.

## The candidate (part 1)

- **The commit:** on branch `codex/piping-f2a-memory-20261004`, against base `8abb5274a9` (U1, U2, U3 grants 1–1d). Your dispatch prompt gives the head.
- **The change:**
  - `PP/src/retained_memory.rs`: `admit`, the census extensions, D1.0–D1.11, the D-6 build status, the bound check (`Unpriced` until part 2), `PhaseFact` and `PhaseRefusal`, the gate checks, R and the budgets, and an empty `REGISTERED_PROFILES`;
  - the new `PP/build.rs` and `PP/src/build_identity.rs`, for D-6, including a self-contained SHA-256;
  - `PP/Cargo.toml` (`build`);
  - the two authorized struct-construction edits, `LateFacts{…,capture}` in `retained_product.rs` and `CompleteFacts{…,capture}` in `lib.rs`;
  - the new `retained_memory_law_tests.rs`.
- **The author's account:** `R/I65/u4_g5_01/RETURN.md`, IMPLEMENTATION.md (code → derivation) and R4_CALLGRAPH.md.
- **The basis:**
  - the brief `BRIEFS/I65_U4_G5_IMPLEMENTATION.md`;
  - the derivations `R/I65/u4_g2_01/`, `u4_g3_01/` and `u4_g4_01/` (with the addendum), including DOMAIN.md, BUILD.md with G2_AMENDMENTS §1, and API_G4.md;
  - every U4 ruling in `T3/ROOT_RULINGS_V1.md` from "U4 plan: decisions…" through "U4 G5 part 1 verified…".

## Review, in priority order

1. **No permit can exist, and nothing published changes.**
   - Confirm by reading that `REGISTERED_PROFILES` is empty, and that `admission()` is the only constructor, conditioned on a registered index and no refusal.
   - Confirm there is no test permit and no `unsafe`.
   - Run your own sweep: at least 30 request fixtures × 5 routes × 2 modes, byte-identical to base, including every public admission-report field. Check that the new private fields are not serialized.
   - Run PP `--lib` and the PP, runner/headless and result_export suites against base.
2. **D1.0–D1.11 against DOMAIN.md** (with `l ≤ 128`, no sections, D1.10 and D1.11).
   - Is each clause implemented exactly, in order, with the right refusal kind?
   - Headless is refused at D1.0.
   - Write your own boundary inputs (cap and cap+1) for at least ten facts, plus D1.10 and D1.11 edge cases (whitespace before `{`, a control byte in a key, 0x7F).
3. **The census extensions.** They must be allocation-free: confirm with the existing counting-allocator pattern. Check the nested typed walk against T03's roster, and that the typed capacity caps read actual capacities.
4. **D-6:**
   - the build script never fails the build, an empty variable is a value, and a read failure is `v1;unavailable`;
   - the escaping round-trips for every byte, so a newline or `;` cannot forge a key;
   - `option_env!` absence means `Stale`;
   - the reviewed-input hashes of the PP lock and the reader's 13 statics match G4's record;
   - **the self-contained SHA-256:** check it against `sha2` yourself, on the NIST vectors, the empty input, the 55/56/63/64/65-byte padding edges, and large random inputs;
   - the layout witnesses never become a compile error.
5. **The gates.**
   - `check_late` and `check_complete` read their facts without allocating.
   - `PhaseRefusal` carries the gate, fact, observed value and cap.
   - G-C bounds the longest string (2,599,962 B) and the longest diagnostic id (2,330 B).
   - The two authorized struct edits are the only changes to `lib.rs` and `retained_product.rs`.
6. **The bound arithmetic.** `E_mov + R ≤ M` uses checked arithmetic, with M−1, M and M+1. `Unpriced` fails closed.
7. **Mutants.** Re-run I65's 84, and add at least eight of your own: a D1 clause dropped or weakened, a cap off by one, the identity escaping, the lock-hash comparison, a gate fact ignored, the bound comparison inverted. Report the survivors.
8. **R4_CALLGRAPH.md (a records item):** does the stated remaining limit (name rebinding within a function body) hold, and does the fix leave TEXT and the recursion inventory unchanged as claimed? Spot-check only. RV83 confirms R-4 in detail.

## Host and method

- **Your copy:** from `git archive` of the candidate, in `WT/rv89/`, with targets `WT/targets/rv89/` and scratch `WT/scratch/rv89_u4_g5/`. Delete the copies afterwards. Never write to the system temp directory.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time. Other TASKs also run cargo in their own targets.
- **The memory guard** must be running.
- **Never:** Git writes, installs, new tooling, or native, solver-at-scale or DEC-025 jobs.

## Output

- **The report:** `NUM/R/REVIEW_RV89/u4_g5_01/REVIEW.md`, containing a verdict, counts, findings (path:line, evidence, remedy) and SHA256SUMS.
- **Time box:** 3.5 h.
- **End your turn** with the verdict, the counts, one line per finding, the sha256, and anything ROOT must rule on. **Part 2 comes by message.**
