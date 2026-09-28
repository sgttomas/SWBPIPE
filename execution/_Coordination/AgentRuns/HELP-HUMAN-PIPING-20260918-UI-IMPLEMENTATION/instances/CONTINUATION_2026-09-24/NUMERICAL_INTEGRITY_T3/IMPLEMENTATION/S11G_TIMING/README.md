# S11-G timing: interleaved comparison of pre-S11-G main against S11-G

This T3-close item was assigned by ROOT and run by I3R, the TASK under the T3 manager, on 2026-09-27. It checks the S11-G performance finding recorded in the K-D5 RETURN (combined-tree addendum, C-8). That finding put S11-G's cost at about +15–20% on dense 1000-member solves, but the estimate was not interleaved.

## Result

**S11-G adds no measurable cost on dense 1000-member solves.** Run interleaved on the same host and occasion, the differences are −0.4% and +0.9%. That is within the run-to-run spread of the same binary, up to 1.8%.

**Per run**, in run order:

| # | Case | Rep | Binary | Wall | Peak RSS (KiB) | Timed out | Load (1-min), start → end | Published |
|---|---|---|---|---|---|---|---|---|
| 1 | RF-LARGE-CHAIN-n01000-AX | 1 | 72d5ff864 | 496.2 s | 3693024 | no | 2.75 → 2.47 | sensitive / needs_recompute |
| 2 | RF-LARGE-CHAIN-n01000-AX | 1 | b24b3d536 | 494.3 s | 3692736 | no | 2.47 → 1.32 | sensitive / needs_recompute |
| 3 | RF-LARGE-CHAIN-n01000-AX | 2 | 72d5ff864 | 496.9 s | 3692960 | no | 1.32 → 1.04 | sensitive / needs_recompute |
| 4 | RF-LARGE-CHAIN-n01000-AX | 2 | b24b3d536 | 494.6 s | 3692976 | no | 1.04 → 2.22 | sensitive / needs_recompute |
| 5 | RF-LARGE-TREE-n01000-ROT | 1 | 72d5ff864 | 479.9 s | 3705488 | no | 2.22 → 1.00 | sensitive / needs_recompute |
| 6 | RF-LARGE-TREE-n01000-ROT | 1 | b24b3d536 | 483.3 s | 3705356 | no | 1.00 → 1.54 | sensitive / needs_recompute |
| 7 | RF-LARGE-TREE-n01000-ROT | 2 | 72d5ff864 | 486.2 s | 3705204 | no | 1.54 → 1.05 | sensitive / needs_recompute |
| 8 | RF-LARGE-TREE-n01000-ROT | 2 | b24b3d536 | 492.0 s | 3705424 | no | 1.05 → 1.15 | sensitive / needs_recompute |

**Per case:**

| Case (dense, captured) | 72d5ff864, pre-S11-G: rep 1 / rep 2 / mean | b24b3d536, S11-G: rep 1 / rep 2 / mean | Difference of means | Per interleaved pair |
|---|---|---|---|---|
| RF-LARGE-CHAIN-n01000-AX | 496.2 / 496.9 / **496.5 s** | 494.3 / 494.6 / **494.4 s** | **−0.4%** | −0.4%, −0.5% |
| RF-LARGE-TREE-n01000-ROT | 479.9 / 486.2 / **483.1 s** | 483.3 / 492.0 / **487.6 s** | **+0.9%** | +0.7%, +1.2% |

- **Spread:** across the four pairs the difference is −0.5% to +1.2%, mean +0.3%. The spread between the two reps of the same binary is 0.1% for CHAIN-AX and 1.3–1.8% for TREE-ROT. So neither difference exceeds noise, and their signs disagree.
- **Peak RSS** is the same for both binaries: 3.69 GB for CHAIN-AX and 3.71 GB for TREE-ROT.
- **No run timed out.** Every run published `sensitive` / `needs_recompute` on both binaries. So on these two cases neither K-D5's check nor a Passed-only path runs.
- **Load.** The 1-minute load average was between 1.00 and 2.75 at the run starts and ends. Other agents were active on the host. Interleaving puts that load on both binaries alike: the first pair started at 2.75 and 2.47, and the rest between 1.00 and 2.22.

**The earlier non-interleaved +15–20% estimate is superseded; the S11-G performance finding is refuted.** It came from runs made with different builds and on different host occasions.
- It compared the combined-tree gate (about 475–490 s on these cases) with the pre-S11-G K-D5 gate (407–425 s), which ran hours apart.
- On this occasion pre-S11-G main itself takes 480–497 s.
- It also matches the interleaved K-D5 comparison earlier the same day: main `b24b3d536` 489–498 s, and the K-D5 candidate 469–485 s.
- So the faster pre-S11-G figures reflect a faster host occasion, not S11-G.

**Scope.** These are two cases, dense mode, captured entry, two reps each. That is enough to rule out a 15–20% effect on these solves, but not to measure an effect below about 2%.

## Method

**Probes.** Both were built fresh from a `git archive` of each commit into scratch, never from a worktree.
- The archive covers `projects/chirality-piping/{core,fixtures,schemas,validation}`.
- The source is P1's probe (`probe_main.rs.txt`, sha256 `8dc727f476ed4ade…`) with the same lockfile (`probe_Cargo.lock.txt`), built `cargo build --release --offline --locked` with toolchain 1.97.1 and `CARGO_INCREMENTAL=0`.
- Each probe's `Cargo.toml` differs only in the archive path (`probe_72d_Cargo.toml.txt`, `probe_b24_Cargo.toml.txt`). The build targets were pruned after building.

| Label | Commit | Commit tree | `projects/chirality-piping` tree | Probe binary sha256 |
|---|---|---|---|---|
| 72d5ff864 (pre-S11-G main, PR1002) | `72d5ff86443345b6594942bb08139c5cec999015` | `2c56e7e718fae6ae6128333704ae80749702bf01` | `8f1a2e77e6ff7ce6d379647e2f11e06ab4692417` | `e0bceb61e7c5cf5b0aed2fd727d8deab77bee2d9df1f688e2322ecc1adfcb52e` |
| b24b3d536 (S11-G merged, PR1003; no K-D5) | `b24b3d5360a4809d7c584c1780a39955fa810dcd` | `44e0500ccb1f280838e3a41371faba5c12310b79` | `ecc499c508f327a4ba74bbfcb35d92ae2b385ee6` | `0fc8e322d2de1b41891990df2299f170cc334e27f21b8a79b9ba874518dd1d7f` |

The earlier b24b3d536 probe (`12811c32…`, built by I3) was not reused. Its source tree is byte-identical to this archive, but which binary was built from it could not be verified. The rebuilt binary's hash differs because each binary embeds its build path.

**Runs.**
- `s11g_timing.py.txt` is I3's K-D5 `timing_compare.py`, with labels as arguments and one more wait pattern; the diff is only those lines.
- Order: 72d, b24, 72d, b24 on RF-LARGE-CHAIN-n01000-AX, then the same on RF-LARGE-TREE-n01000-ROT. That is 8 runs, dense mode, captured entry.
- Requests: P1's generated R1 requests, the same as the K-D5 gate.
- Each run is a fresh process through P1's `run.py` `run_one` (RLIMIT_AS 6 GiB, 1800 s timeout for 1000 members). `run.py` and `compare.py` are unchanged from the K-D5 gate (same sha256).
- Each run records wall time, peak RSS, the load average at start and end, and the published standing.
- Before every run the script waited while any cargo process, DEC-025 sweep or other timing comparison was running. No cargo ran during the 8 runs.
- The runs went from 21:09 to 22:14 UTC.

## Records (`_run_records/`)

- `runs.jsonl`: the 8 runs.
- `timing.log`: the driver's stdout.
- `summary.txt`: the table and means, from `summarize.py.txt`.
- `s11g_timing.py.txt`, `summarize.py.txt`, `run.py.txt`, `compare.py.txt`: the scripts.
- `probe_main.rs.txt`, `probe_72d_Cargo.toml.txt`, `probe_b24_Cargo.toml.txt`, `probe_Cargo.lock.txt`: the probe sources.
- `build_72d.log`, `build_b24.log`: the probe builds.
- `environment.txt`.

Hashes are in `SHA256SUMS`. Paths are placeholders (`<scratch>`, `<wt>`).

## Not done

- No other cases, modes or entries, and no sparse runs.
- No quiet-host guarantee beyond waiting for idle cargo; the load averages are recorded instead.
- No product or test change, and no Git write. The manager commits.
