# Repaired units witness return

PASS: exact three original frozen m/mm/in fixtures produce exactly equal complete mechanics envelopes, all MECHANICS_SOLVED, maximum displacement 0.406023 mm. Before repair the same fixtures returned 0.406023,406023368.175308,24777.066116 mm respectively. No fixture or driver changes were made to obtain this comparison.

Source binding: W2-frozen core/product_physics/src/lib.rs SHA256887fb99ce98cc6e08f65fcbb87c970ce71a9df5e3bbe282406a738a3e67752ce verified immediately before and after execution, copied as product_physics.rs. Driver bytes match baseline snapshots/UNITS_V1/native_witness.rs. Baseline A2 51manifest files plus FINAL_MANIFEST.json hash are unchanged. Exact original fixture hashes are in COMPARISON.json. No W2-added regression fixture was substituted.

Commands executed from /Users/ryan/.codex/worktrees/8728/chirality:

`CARGO_TARGET_DIR=/tmp/piping-a2-repaired-units-target cargo build --offline --manifest-path /tmp/piping-a2-witness-20260905/Cargo.toml`

Offline isolated build exit0 in4.82seconds (session5172), original lock retained and copied. Then three executions:

`/tmp/piping-a2-repaired-units-target/debug/piping-a2-witness projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/A2/snapshots/UNITS_V1/fixtures/invented_linear_m.json`

Same exact command with mm and in filenames. All exit0, raw stdout native_m.json/native_mm.json/native_in.json. REPLAY.py repeats the comparison without altering any packet output. Source toolchain unchanged from UNITS_V1 environment (isolated devbuild). Repaired binary SHA is bound in MANIFEST.json.

Calibration: actual unchanged scratch driver invokes repaired native product function; this is not a packaged-app/transport walkthrough or independent engineering suitability validation. No other families tested, no lifecycle/hold closure, no product code changed by A2. Root combines this evidence with W2 fresh review; all original findings and hold records remain immutable.
