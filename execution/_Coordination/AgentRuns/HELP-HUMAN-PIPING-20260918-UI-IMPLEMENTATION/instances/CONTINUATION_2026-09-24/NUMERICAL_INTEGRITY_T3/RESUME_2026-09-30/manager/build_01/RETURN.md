# Manager G0 build return

**PASS for G0's one build. No solver case ran.** Existing I22 completed its
follow-up and is idle; I21 remained idle throughout. No owned build is running.
ROOT's separate B grant and independent comparator are still required.

Grant: <COORD>/<R>/BRIEFS/I22_G0_BUILD.md at
90b6bcbbf64b13975211038bf3f33bb87273e646, SHA256
2c1c6e0afdd5bd1a0a96623ca75e2c7084dc8503ab0f5318e23a244e9cc8706b.
DISPATCH.json and FOLLOWUP.txt preserve the actual native follow-up identity,
start acknowledgment, normalized complete prompt, raw transport prompt hash,
scope and returned identities. No fresh child was spawned.

## Observed build and verified result

I22 archived the pinned unchanged FK source, copied the original response
probe and two-package lock, and changed only the scratch Cargo.toml dependency
path. It ran exactly one offline/locked debug build with rustc 1.97.1,
-j4, incremental disabled and its private scratch target. The truthful
SOURCE_COMMIT field remains 3bddc2b05f6106e969c7cf43373b230845c7cc66;
FK source equality to product d01ad98a754698631f927709d08284c272de85e8 is
preserved. No product code or maintained harness source changed.

The build launched at 2026-10-01T04:58:43.989Z under tool PTY 24126.
The supported poll returned exit 0. /usr/bin/time -l reported 3.80 seconds
real, 3.10 user, 0.42 system, maximum RSS 430,161,920 bytes and reported
peak footprint 7,357,040 bytes. Cargo reported 3.77 seconds. These are
observed command fields, not a process-group bound, per-process hard cap or
performance guarantee.

Both resolved feature sets and rustflags are empty. mutation-controls is
declared by FK but was not enabled. Compiler commands show no cfg(test).
The full portable stderr and unchanged empty stdout are in I22's build packet;
original raw logs and their hashes remain in its owned scratch.
Relevant Cargo config inventory found no config files; inherited configuration
roots were preserved and explicit job-only injection-variable removal matched G0.

The manager read the full child return, portable build log, fingerprints,
rebound Cargo.toml and preflight/PTY/postflight evidence. Independently checked:
- Nine child build payload entries: all hashes match.
- 115 archived source entries: all match; full check output SOURCE_VERIFY.txt.
- 141 scratch inventory entries: all match; full output SCRATCH_VERIFY.txt.
- Binary rehash: bcbe897204ec702b99529d25e6d0213d0132af5e6086e0397fae3a8f8ef8a08f.
- Original six-file I22 checkpoint and eight-file manager checkpoint seals:
  unchanged and passing.
- Product tracked and staged diffs: empty.

Child RETURN.md SHA256:
68e536ee6b3b68fc0b2876f274944043a99ed038145c6bd175fe182c3b0c87f2.
Child build SHA256SUMS SHA256:
0a32a4194a0b231c60bf2904bab36d36dab84efd19fd10141f9f77057aa8170e.

## Supervision, writes and limits

I22's before/after process evidence identifies existing guard PID 5387 and
unchanged memguard hash ee11ea1f43f0892dd0396bfe5c68e133084c90c87abbf757ce0be723e0ca7b79.
This is before/after liveness, not a continuous guard audit. The host-wide floor
was the only adopted memory enforcement. The 300-second operator stop threshold
was not reached; no automatic deadline or job RSS cap is claimed. No custom
wrapper or guard was created. Normal PTY completion and the scoped post-build
process query support the no-running-owned-build statement.

I22 wrote only <A1_WT>/<R>/I22/build_01/** and <wt>/scratch/i22/**.
The manager wrote only the additive manager/build_01/** packet. Prior seals
were not rewritten. Source, probe, lock, raw-log and compiled-artifact identity
is in the child's canonical packet and inventory. No Git/index mutation,
new host tooling, repair, solver case, unit variant or oracle comparison occurred.
All G0 Git reads explicitly used GIT_OPTIONAL_LOCKS=0.

Two manager evidence reads initially used wrong filenames/directories and
returned missing-file errors. They were corrected using the known child
paths. They were not failed build/hash checks: the preceding child manifest
check had already passed. A broad command-history projection was tool-truncated;
the complete build log and decisive supervision sections were separately read.
Native transcript preserves those diagnostic reads and errors.

G0 establishes compile readiness only. Numerical admission, all-row truth,
false-publication investigation and the universal publication design remain
open under checkpoint 0. Return to ROOT to decide the B grant; no case may
start on the strength of this build return.

