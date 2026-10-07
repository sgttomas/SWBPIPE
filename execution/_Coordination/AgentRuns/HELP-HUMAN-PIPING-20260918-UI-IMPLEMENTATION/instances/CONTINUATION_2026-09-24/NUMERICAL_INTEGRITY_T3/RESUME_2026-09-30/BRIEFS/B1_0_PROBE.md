# B1-0: the probe before B1's widening (records only; nothing maintained)

TASK (Type 2), dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Your ID names a records folder and a role, with no memory of earlier sessions. Earlier holders of related roles (I61, I68, I78, RV105) left their work in records. Cite them, and assume nothing beyond them.

## Why

B0's contract is selected (RR "B0 selected on DESIGN_v2; C3a's names reserved; B1 opens with a probe and the cap/M study"). B1 widens the F2a D1 milestone to n load cases. Before any maintained edit, three facts must be established on the real producer, because B1's witnesses and re-basings depend on them (DESIGN_v2 §1.3, §1.4 and §9).

**The basis:**
- `R/I78/b0_contract_01/DESIGN_v2.md`: §1.2's T-3 and T-4, §1.3, §1.4, decision 21, and §10's list of code;
- `R/REVIEW_RV105/b0_01/`: REVIEW §1–§2, ADDENDUM_01, and `evidence/probe_quality.log`;
- U8's probe, `R/I68/u8_probe_01/PROBE.md`, with its `I68_ORDINARY` log lines and method. Follow that method;
- the brief U8's probe answered: `R/BRIEFS/I68_U8_PROBE_AND_WITNESSES.md`, Part 1;
- QUAL §4 (`T/IMPLEMENTATION/F2A_D1/copies/QUALIFICATION.md`).

## Where and how

- **Your copy:** a disposable `git archive` of main `d8c88774d0` into `WT/scratch/<id>_b1_probe/`, with target `WT/targets/<id>-b1-probe/`. Main carries the D1 milestone and U8. Probe-only test code lives in the archive copy only.
- **The build and entry:** the registered dev/test build (a default `cargo test` on this host), in both modes (SparseInteractive and DenseScrutiny), through the real Direct entry `run_linear_static_preview_value_with_retained_direct` unless noted.
- **Every cargo goes through `WT/tools/t3_cargo.sh`** (`--locked --offline`). Other T3 jobs share the lock, including ROOT's DEC-025; wait for it, and never kill another job.
- **Limits:**
  - no DEC-025 and no evidence sweep;
  - no runs beyond the committed witness inputs and the probe inputs named here;
  - no installs and no Git writes (read Git with `GIT_OPTIONAL_LOCKS=0`);
  - nothing in the system temp directory.

## What to establish

1. **The verdict audit (S-1).** Record these for every QUAL §4 witness input (W1 to W7, W2-deep, headroom, and W2 and W2b in particular) and every `attempted_examples` input in `retained_memory_law_tests.rs` (the milestone, the 1e-300 spring, K2a's deferred formation, and the `rejected_stress_range` pair):
   - the published `solve_quality`;
   - the seed's `initial` (kind, outcome and tag);
   - the seed's `w2`;
   - whether W1 runs today;
   - the W1 outcome today.

   Then classify each under the selected T-4 and decision 21: in A, `not_required`, or excluded. Say which witness or test changes.
2. **Decision 21's tags.** Record the seed's structural error tag for every attempted failure among those inputs, and for `tiny_spring`.
3. **W-C2's case C** (DESIGN_v2 §1.4). Use U8's two-body model (`u8_two_body_case_a` and `u8_two_body_case_b` in `PP/src/retained_facade_tests.rs`). Case C is A's loads plus B's loads, run as a one-case request in both modes. Record:
   - the verdict and seed;
   - whether W1 runs;
   - the native terminal, with its `UnresolvedReason` (via a probe-only print);
   - the fallback and the notice.

   **The stop rule:** if C's verdict is `checks_passed`, or its native run does not end at Ceiling, record the actual outcome and try fallbacks (i) and (ii) from §1.4 the same way. Stop after them, whatever they show.
4. **Re-basing:** does case C alone (or the fallback that works) give the outcome W-C1 and the W6 stack witness need? That is a Native fallback at Ceiling, and for W6 the force-scaled path. Name the input B1 should use for each changed witness from item 1.
5. **Not this probe's work:** no multi-case invocation. The producer admits one case today (D1.4). State that it is untested.

## Output

- **The record:** `R/<id>/b1_probe_01/PROBE.md` plus SHA256SUMS, with placeholder paths only. Put logs, with probe-only prints, in `_run_records/`.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/<id>_b1_probe/records/` and say so.
- **Delete** your archive copy and target afterwards.
- **Budget:** 3–5 h.
- **End your turn with:**
  - PROBE.md's sha256;
  - one line per audited input (its verdict and its T-4 class);
  - case C's outcome in both modes, and the fallbacks if tried;
  - the re-basing proposal;
  - anything ROOT must rule on.
