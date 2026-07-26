# Agent 2 return — R20 terminal-label correction

`BLOCK / NOT COMMIT-SAFE`

## Blocking finding

`WORK_GRAPH.json` remains pre-terminal at SHA-256
`49b7aebd060f51cc52f05e989dcc369e4163cbc96c9cc6c7d02351858db2ea2d`:

- P1/P2 remain `COMPLETED_AWAITING_VERIFICATION`;
- V1/I1 remain `PENDING`.

That state contradicts the accepted Agent 2 `PASS / COMMIT-SAFE`, terminal
manager status and handoff, Receipt-74, and the corrected terminal labels.

## Provenance limitation

Reversing only the corrected orchestration status produces
`43587ba42940a72a2e513fc3493be7dca4805a1d2650c3e262ab652ceadf463e`,
not the frozen pre-V1 plan hash
`689d239744da9e57dccaf8e4633abd0c517f843a5979d29f50545dc00925f9bf`.
The immutable artifacts therefore do not independently prove that this status
was the only post-manifest plan change. The current frozen manifest SHA-256 is
`9c013e3f381963ddefc4c14d2783b71f8c30477028aa1c87899c1a3f6334781e`;
the manager reports exact patch containment and no manifest edit, but no
separate prior digest exists.

## Checks that pass

- Receipt-74 occurs once, follows Receipt-73, names Receipt-73 as parent,
  records the accepted verifier pass, and the piping receipt validator returns
  `VALID`.
- Reversing only the `DEC-089` label reproduces frozen SHA-256
  `45a940bd1cde5ca2cd13bf45d926146c97f03cec4866422a67a228f0b1e1e06d`.
- Current corrected hashes:
  - plan:
    `41f2d6930005a67dc367b45cf27b51dd8f09e30a0d51a8a483faea54d49c5c91`;
  - decomposition:
    `7c447acceb5fb57aeae370cfebd3cfd7186dde278ae15417c42e14d70fcc31ce`;
  - receipts:
    `e48f71edb46fa3403d513d9db27df3d43a193d4f1bc3c8fda944f92d546cdb73`;
  - correction record:
    `c82cc9f242baf0da1c06c1c038a87330df43d603014dff5d300c009fac0f5265`.
- R19 aggregate remains
  `1802ace0cc5fa898935c2069cfbf0a9d4ebe5c77b25b76c89cded9e748e868f4`.
- All R20 JSON parses; `git diff --check` passes.
- Protected D-06, D-21, PRDs, DEL-10-04 status, build/release guide, Tauri
  config, DAG pointer, and DAG approval hashes match.
- HEAD remains `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`; staged path count is zero.
- No verifier write, Git mutation, network call, or external effect occurred.

## Recommended bounded recovery

Correct only the four stale `WORK_GRAPH.json` node statuses, durably bind the
before/after state, and dispatch a fresh read-only verifier.
