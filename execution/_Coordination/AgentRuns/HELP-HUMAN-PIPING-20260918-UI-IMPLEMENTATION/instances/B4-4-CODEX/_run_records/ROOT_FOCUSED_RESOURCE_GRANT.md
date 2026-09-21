# Sections focused resource grant

ROOT grants the retained WORKING_ITEMS manager and its existing Sections TASK
the four requested operation-applier tests, sequentially, from wt3. No new
worker, production scope or protected criterion is introduced.

The peer dev - app acknowledged this reservation on 2026-09-21:

> No conflicting active or queued builds/tests, browser/native/CUA activity or oMLX work in this session. Reservation acknowledged for the bounded single-worker operation-applier Rust tests, followed by focused browser journeys on 5174 under swbpipe-e2e.lock. I will defer competing workloads until your release notice. This acknowledgment covers those requested tests only; native/CUA/full-sweep use remains separately coordinated. Existing write ownership, sole shared-status editing and pending CLI activation are unchanged.

From wt3 REPO_ROOT, execute each command with this form, one at a time:

```sh
cargo test -j 1 --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml FILTER -- --test-threads=1
```

The four exact FILTER values are:

- section_geometry_batch_validates_each_intermediate_state_and_never_publishes_failure
- section_grid_edits_validate_and_apply_same_geometry
- shared_section_propagates_atomic_geometry_and_full_diffs_then_detaches
- shared_wall_update_fails_atomically_on_one_local_mill_tolerance

Retain command, source/test identities, environment, raw output and actual
collected/pass/filtered counts, including failures. These are bounded checks,
not a complete cargo suite or the final registered sweep. Do not edit source
during a running check. Current focus remains the authorized test-code-only
coupled-order regression; Rust implementation/schema stays unchanged.

Browser resources are reserved next, but execution awaits the manager's exact
source checkpoint, collection and focused plan, followed by ROOT's execution
message. Use one worker, the pinned Chromium and shared lock on source port
5174. No native/CUA, production app build or full-sweep grant is included.
Release processes and report the handoff when the bounded checks finish.

This is an implementation/resource decision under the approved strategy.
Owner-held C4 and live-control choices, DAG/deliverable reconciliation,
qualification and release remain outside scope.
