# Executed command/evidence catalog

All commands ran with cwd `/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c`. `T3` and `R` in shell commands were the repository-relative roots given by the parent. `REC=R/I25/historical_replay_01/_run_records`. `WT=/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3`. Python below is the parent's existing interpreter `/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/projects/chirality-piping/.venv/bin/python`, always `-B`.

## Read-only identity and frozen sources

Actual shell queries began with `pwd`, `cat AGENTS.md`, `cat agents/AGENT_TASK.md`, `cat projects/chirality-piping/AGENTS.md`, `cat "$R/BRIEFS/COMMON.md"`, then:

```sh
GIT_OPTIONAL_LOCKS=0 git rev-parse --show-toplevel
GIT_OPTIONAL_LOCKS=0 git rev-parse HEAD
```

They returned the cwd above and9086964a1fb656a76cda6d1002d8594efa636fdc. Source snapshot operations were:

```sh
GIT_OPTIONAL_LOCKS=0 git show 9086964a1fb656a76cda6d1002d8594efa636fdc:projects/chirality-piping/core/solver/performance_harness/runner/k6_runner.py > "$REC/k6_runner_908.py"
GIT_OPTIONAL_LOCKS=0 git show 9086964a1fb656a76cda6d1002d8594efa636fdc:projects/chirality-piping/validation/benchmarks/numerical_robustness/runner/vk_scale_runner.py > "$REC/vk_scale_runner_908.py"
GIT_OPTIONAL_LOCKS=0 git show 9086964a1fb656a76cda6d1002d8594efa636fdc:projects/chirality-piping/core/solver/performance_harness/src/k6/w1/h_envelope.rs > "$REC/h_envelope_908.rs"
GIT_OPTIONAL_LOCKS=0 git show 9086964a1fb656a76cda6d1002d8594efa636fdc:projects/chirality-piping/validation/benchmarks/numerical_robustness/src/envelope.rs > "$REC/vr_envelope_908.rs"
GIT_OPTIONAL_LOCKS=0 git show 9086964a1fb656a76cda6d1002d8594efa636fdc:projects/chirality-piping/core/solver/performance_harness/src/bin/k6_observe/main.rs > "$REC/h_main_908.rs"
GIT_OPTIONAL_LOCKS=0 git show 4eeb206c09b67b9c5f83ca89f5e66d188ad5a02b:projects/chirality-piping/core/solver/performance_harness/runner/k6_runner.py > "$REC/k6_runner_historical_4eeb.py.txt"
GIT_OPTIONAL_LOCKS=0 git show 4eeb206c09b67b9c5f83ca89f5e66d188ad5a02b:projects/chirality-piping/core/solver/performance_harness/src/bin/k6_observe/main.rs > "$REC/h_main_historical_4eeb.rs.txt"
```

`final_checks.py` repeated precisely those seven read-only Git shows through `subprocess.run`, with environmentGIT_OPTIONAL_LOCKS=0, comparing bytes. Exact argv/exit/stderr/SHA are in SOURCE_ORIGINS.json. No status/index/fetch or other Git operation occurred.

## Exploration and selected reads

Used `rg --files` on T3 and the named H/VR runner/source directories, filtered to the selected packet/function/file names, followed by bounded `cat`, `head`, `sed -n` and `rg -n` views. Several initial tool-display outputs were truncated by the display budget; they were navigation, not numeric inputs. Structured input reads, exact hashes and script sources are retained, and all numerical inputs were subsequently consumed from full bytes.

Selected content reads included: original I21 brief; K0 DESIGN_AND_PLAN/HISTORICAL_COMPARISON; H19 RETURN/FORMULAS/LAUNCH_BINDING/PHASES/HISTORICAL_COMPARISON; kernel22 RETURN; caller14 RETURN/METHOD/CLI24_CALLERS/CLI24_LAUNCHES/CLI_EXTERNAL_INPUTS; join15 RETURN/METHOD/RESULT5_CLI24; four independent reviews; H25 and VR01 author returns; original K6B/VK/KF3 records/metadata/stdout; original VK/KF3 RETURN and setup/binary.txt; K6B b3 run_slot/build_release/run_w1t4 records. READS.jsonl records actual hash-bound paths. No unselected broad governance or extra role body was loaded.

The bounded raw-source commands were:

```sh
cat "$WT/scratch/i16/b3/run_slot.sh" "$WT/scratch/i16/b3/build_release.sh"
ls "$WT/scratch/i17/b_records"
cat "$WT/scratch/i17/b_build_archive.log" "$WT/scratch/i17/b_build_release.log" | head -100
head -4 "$WT/scratch/i17/b_records/07_RF-LARGE-CHAIN-n01000-AX_w1a.time.txt"
head -30 "$WT/scratch/i17/b_records/07_RF-LARGE-CHAIN-n01000-AX_w1a.record.json"
head -30 "$WT/scratch/i16/b3/records/139_RF-LARGE-CHAIN-n00010-AX_w1a.record.json"
```

Relevant raw H scripts and VK archive build log are copied under REC; originals were unchanged. No broad recovery scan occurred. The parent, not I25, reported absence of possible KF3 raw launch roots; GRANT.md preserves that distinction.

Two incidental read-only exploration errors are disclosed: `ls -d .venv */.venv 2>/dev/null` hit zsh's no-match behavior (after `command -v python3` returned /usr/bin/python3); a bounded `rg` query named a nonexistent KF3 `_run_records/a/build_release.log`. Neither executed Python, ran a build, changed a source nor supplied an inferred result. There were no failed numerical checks.

## Bounded Python execution

The exact one-off script bodies are preserved here. They read existing evidence, calculate integers/ratios, extract only pure admission functions by AST and write this packet. Neither complete runner is imported or launched. The frozen `members_of` helper's model-generation fallback is never reached; no model implementation is loaded into its namespace.

Actual execution order:

```sh
<VENV>/bin/python -B "$REC/inspect.py"
<VENV>/bin/python -B "$REC/inspect2.py"
<VENV>/bin/python -B "$REC/inspect3.py"
<VENV>/bin/python -B "$REC/inspect4.py"
<VENV>/bin/python -B "$REC/replay.py" > "$REC/replay.stdout.json" 2> "$REC/replay.stderr.txt"
<VENV>/bin/python -B "$REC/h_launch_binding.py" > "$REC/h_launch_binding.stdout.json" 2> "$REC/h_launch_binding.stderr.txt"
<VENV>/bin/python -B "$REC/apply_h_actual.py"
<VENV>/bin/python -B "$REC/replay.py" --h-actual > "$REC/actual_h_replay.stdout.json" 2> "$REC/actual_h_replay.stderr.txt"
<VENV>/bin/python -B "$REC/verify_basis.py" > "$REC/verify_basis.stdout.json" 2> "$REC/verify_basis.stderr.txt"
<VENV>/bin/python -B "$REC/metric_disposition.py" > "$REC/metric_disposition.stdout.txt" 2> "$REC/metric_disposition.stderr.txt"
<VENV>/bin/python -B "$REC/enrich.py" > "$REC/enrich.stdout.txt" 2> "$REC/enrich.stderr.txt"
<VENV>/bin/python -B "$REC/final_checks.py" > "$REC/final_checks.stdout.json" 2> "$REC/final_checks.stderr.txt"
```

One additional read-only inline Python command printed the first two summary objects from HISTORICAL_RAW_HEAP_FIELDS.json for schema inspection. Numerical scripts were created through shell here-documents. `apply_h_actual.py` was a one-time edit to this packet's own replay script, adding the actual-H option; its original behavior is still the default. **Do not rerun apply_h_actual.py** against the already edited source. It is retained as edit provenance, not a maintained tool.

Reproduction from the final scripts: run replay.py; h_launch_binding.py; replay.py --h-actual; verify_basis.py; metric_disposition.py; enrich.py; final_checks.py, in a fresh owned evidence directory using equivalent path constants. This is a finite evidence replay, not a framework or instruction change. Initial exploration scripts and original raw stdout remain for provenance. All substantive computation stdout/stderr are unabridged files. No evidence was silently taken from a later measurement.

## Own writes and sealing

Shell `mkdir -p` created only OUT/_run_records. Shell here-documents and the named one-off Python scripts wrote only OUT. RETURN.md and this command/grant record were created after checks. Final `seal.py` verifies external input hashes still match, creates an input inventory and timing, and emits SHA256SUMS plus WRITE_INVENTORY.json. Their mutual sealing exclusions are explicit to avoid self-referential hashes. No previous sealed packet was edited or sealed before return.
