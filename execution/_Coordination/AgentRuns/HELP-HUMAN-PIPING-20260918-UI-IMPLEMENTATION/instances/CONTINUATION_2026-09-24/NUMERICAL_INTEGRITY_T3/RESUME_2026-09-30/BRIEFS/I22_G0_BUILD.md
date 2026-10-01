# I22 G0 — source-pinned preparation and one build, ROOT grant

ROOT read I22/CHECKPOINT_0.md at SHA256
eb4da1fc3b075a65bc63edc36581d6e6f263e80f46f277aa15cef3c174f46002
and verified its seal. The finite source/run plan is suitable to proceed to build.
No solver case, unit variant, repair or design change is granted by G0.

Authorize the checkpoint's scratch-only archive/copy preparation and exactly one
offline locked build of the unchanged probe, with only its dependency path rebound
to the archived FK source. Source is 3bddc2b05f6106e969c7cf43373b230845c7cc66;
ROOT/I22 verified it is identical to d01ad98a754698631f927709d08284c272de85e8.
Use original probe source and two-package lock from response head 520d7dfb...
Do not change the truthful SOURCE_COMMIT field. Record hashes and resolved features.

Use the existing M5 host-wide memguard, confirmed by ROOT at PID 5387 before this
grant, and recheck before launch. Installed rustc 1.97.1 was verified by ROOT.
Cargo executable is the configured <home>/.cargo/bin/cargo. Preserve existing
CARGO_HOME/RUSTUP_HOME; explicitly set toolchain and the COMMON settings.
Unset FK_SEEDED_FAULT, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, RUSTC_WRAPPER and
RUSTC_WORKSPACE_WRAPPER for this job. Inspect/hash relevant config without copying
credentials. No install, network, lock generation or tool/config change.

ROOT does not adopt the proposed per-job hard RSS limits: the existing guard
enforces the host-wide memory floor. Do not claim a per-process cap it lacks.
The one-member probe has a fixed small model. Use the tool-managed PTY and
/usr/bin/time -l to supervise and record this one build; no custom wrapper.
The proposed 300 seconds is an operator stop/report threshold, not an automatic
enforced deadline. If reached, interrupt this job's own PTY with the supported
tool, verify its owned build processes have stopped and return the blocker.
If ordinary tool supervision is unavailable, stop and tell ROOT; do not engineer
a replacement. No build overlap with a ROOT exclusive measurement slot.
ROOT has reserved this small build slot before the records DEC-025 sweep.

Write additive I22/build_01/** plus existing owned scratch only; do not amend the
sealed checkpoint. Return actual argv/env/config, source/probe/lock/binary hashes,
full stdout/stderr, exit, duration, resource observations and any supervision gap.
After successful build return and wait for the separate B grant and frozen
independent comparator. Manager verifies returned evidence; ROOT commits.

