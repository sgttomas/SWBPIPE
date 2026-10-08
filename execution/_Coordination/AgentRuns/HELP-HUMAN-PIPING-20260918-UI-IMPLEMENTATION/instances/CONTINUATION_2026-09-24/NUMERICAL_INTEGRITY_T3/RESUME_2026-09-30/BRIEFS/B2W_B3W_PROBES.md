# B2-W and B3-W: phase-0 probes for B2 and B3 (records only)

Two TASKs share this brief. Each holds one probe:
- **B2-W:** the combination witnesses' one-case proxies;
- **B3-W:** the exact route's verdicts.

You are a fresh TASK (Type 2), an implementer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. Cite the records, and assume nothing beyond them.

## Why

B2's and B3's witnesses must be chosen before their producers are written. Each probe runs the ordinary route, and the private W1 driver where the method needs it, on a disposable archive of main. It records what each candidate input actually does in both modes: the verdict, the seed, the furthest W1 stage reached, and the native terminal. **No maintained file changes.**

## The method and its basis

- **The method:** I81's probe (`R/I81/b1_probe_01/PROBE.md`, brief `R/BRIEFS/B1_0_PROBE.md`) and I86's (`R/I86/b1_w_probe_01/PROBE.md`, brief `R/BRIEFS/B1_SW.md`). Use their harnesses as references. Probe-only code goes under `cfg(test)` in your archive only.
- **The plan:** I93's PLAN.md §1.2.6 (the witness table), §6 (stop rules) and REVISION_01 §5 (the B2-W and B3-W outlines) with RV114's N-7. Also RR "B2/B3 R1: …" and "I93's REVISION_01 accepted; …".
- **The designs, where they bear on a witness:**
  - B2-C (`R/I97/b2_c_01/CONTRACT.md`, under review): C-1 says repeated-case mechanics stay `ordinary`, and C-9 makes ids disjoint;
  - B3-D (`R/I96/b3_d_01/DESIGN.md` with REVISION_01, final for J1): §5, P-13, and the open question in §13 ("The exact-route verdicts are B3-W's").

## B2-W

1. **W-CB1's proxy.** SW's cap-maximal cases A and B (`R/I86/b1_w_probe_01/`, accepted) combined as one case carrying `A + B`'s loads: 2l = 256 nodal loads, **unnetted** (RV114 N-7).
   - Bypass G-A and G-B with probe-only code in the archive, and record the over-cap fact.
   - A netted proxy is rejected, because it changes the cancellation the combined ledger preserves.
   - Record the verdict and native terminal in both modes, and whether the proxy would select.
   - If `A + B` does not select at the caps, its asserted outcome stands, and RSS_TIME later states the furthest phase (PLAN §6, item 7). Record the furthest phase reached.
2. **W-CB3's candidates.** A `not_required` case on a body that does not change A's numerics, beside the milestone case. One example: a force on the L = 0 base's fully restrained isolated node.
   - Try at most 6 variants.
   - **The stop rule** (PLAN §6, item 7): if none gives a `not_required` operand beside a selecting combination, return. The mixed-selected shape then rests on the kernel test (C01) plus a labelled synthetic base, as ROOT rules.
3. **W-CB2's prediction.** Check that W-C2's case C (`Unresolved(Ceiling)`, RR "I81's B1-0 probe verified…") still predicts W-CB2's `retained_unavailable` and `combination_unresolved`. Re-run case C's proxy at main and compare it with I81's lines.
4. **Controls:**
   - each run twice, byte-identical;
   - a reproduction of I81's and I86's recorded lines for at least one input each.

## B3-W

1. **The milestone authored as 0.3.0 exact.** Use E and ν, and explicitly empty pressure regions (`[]`). Also n05 and n06.
   - Run each in both modes, through the ordinary route on main.
   - Record the verdict, physics-source-1's selection (`source_block_recovery`), and the rows the exact route publishes with empty regions.
   - **Say which input selects** (DESIGN §13's open question). That gives the exact successor's witness.
2. **A mixed exact base:** whether any exact-route input gives a case that selects beside one that does not. Try at most 4 variants, with each recorded. This settles P-13's and 07o's "if B3-W finds one".
3. **B3a's check.** The milestone authored as 0.3.0 `legacy_pressure_v1` with zero pressure. Its ordinary bytes must equal the 0.1.0 milestone's, except the model echo. Show the difference.
4. **P-2's budget parity, as a note:** whether physics-source-1 selects differently under the Direct entry's 8M exact-block budget than under `permitted_run`'s default 4M. Run it only if your harness reaches it cheaply.
5. **The same controls** as B2-W.

## Host (strict)

- **Your archive:** a `git archive` of main `0b6c5d7362` in `WT/scratch/<id>_<b2w|b3w>/`, with targets under `WT/targets/<id>-<b2w|b3w>*`.
- **Every cargo** goes through `WT/tools/t3_cargo.sh` (`--locked --offline`). Any direct test binary, native run or heavy pytest runs under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.
- **B1's work goes first on the lock.** Before each heavy job, wait while more than one `/usr/bin/lockf` process is running (`pgrep -f '^/usr/bin/lockf' | wc -l` above 1), so that a job already queued goes before yours. Never kill another job.
- **Waits:** one wait per job, and every wait loop ends when the job's process has gone. Stop your own waits before you return.
- **Not allowed:** DEC-025, evidence sweeps, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Paths:** absolute paths for every write. Scratch, and TMPDIR, go in your scratch folder. Nothing goes to the system temp directory.
- **Records:**
  - placeholder paths only;
  - no symlink, and no folder named `build`;
  - remove the `hostname` attribute from any pytest junit output;
  - before returning, screen with the strict pattern `~/|/Users/|/private/|\.claude/worktrees|swbpipe-control-layer` and the machine's host name, decompressing any `.gz` file, and run `git status --ignored` on your folder.

## Output

- **The record:** `R/<id>/<b2_w|b3_w>_probe_01/PROBE.md`, with `_run_records/` (inputs with sha256s, the harness, logs) and SHA256SUMS.
- Delete your archive and targets when done.
- **Budget:** 3–5 h each.
- **End your turn with:**
  - PROBE.md's sha256;
  - per item, the variants and their outcomes in both modes;
  - the recommended witnesses, with their inputs' sha256s;
  - whether a stop rule fired;
  - anything for ROOT.
