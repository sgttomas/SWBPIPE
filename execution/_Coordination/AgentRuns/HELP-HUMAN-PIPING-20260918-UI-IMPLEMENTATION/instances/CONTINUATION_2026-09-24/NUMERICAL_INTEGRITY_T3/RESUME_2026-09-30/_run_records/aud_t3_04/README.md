# AUD-T3-04 evidence preservation return

**Support preservation complete; ROOT integration and independent review pending.**
This packet recovers the six audited final-head DEC-025 per-manifest log sets and
the two KF2 gate part-1 JSONL files. No numerical, solver, CI, or sweep rerun was
performed. Preservation does not make a new claim that the historical gates pass.

## Execution basis and boundary

The direct parent was ROOT `/root`; the executing TASK was
`/root/audit_evidence_preservation`, launched with Codex native
`collaboration.spawn_agent`. This is actual execution, not only a written brief.
The TASK loaded Root AGENTS.md, agents/AGENT_TASK.md, Piping AGENTS.md, and the
EVIDENCE/COMMON briefs from coordination HEAD
`3446bdf51d90b8f1812d161cc638d85c97bbef16`. Input origins and SHA256 values are
in INPUT_SHA256SUMS. No additional role, skill, or reusable workflow was activated.
The support work stayed within its 30-minute allowance.

Writes were restricted to this evidence directory and
`<wt>/preserved-evidence/t3-audit-20260930/`. These are prompt restrictions on a
shared host, not an OS sandbox. No Git/index mutation, upload, move, deletion,
prune, installation, tool development, build, solver run, or further delegation
was performed. Read-only Git queries confirmed the coordination basis and that
sample packet paths are not ignored.

Aliases used throughout the packet:

- `<COORD>`: supplied numerics coordination checkout.
- `T3`: this undertaking's NUMERICAL_INTEGRITY_T3 directory, relative to the repository.
- `<PACKET>`: `<COORD>/T3/RESUME_2026-09-30/evidence`.
- `<wt>`: supplied host T3 worktree/scratch root, the parent of numerics.
- `<LOCAL>`: `<wt>/preserved-evidence/t3-audit-20260930`.
- `<VENV>`: historical Piping virtual environment; `<home>`: historical user home.
- `<tmp>`: historical temporary-directory prefix.

Actual local paths remain in ROOT's supplied execution context, not committed
artifacts. The paths in INPUT_SHA256SUMS are relative to the coordination checkout
except the explicit `<wt>` entry. SHA256SUMS runs from this packet's directory.

## Recovered inventory

| Slice | Final candidate | Per-manifest logs | Source directory |
|---|---|---:|---|
| K4 | 5a46a6278af3c857a52ecb9be6880990493190f8 | 39 | <wt>/scratch/sweep_k4 |
| KF1 | 66adfede42de817efb5e0090342c02e3e4382f21 | 39 | <wt>/scratch/sweep_kf1 |
| V-K | 5f0d394262516e4053322730506cb88dfb36a2f0 | 40 | <wt>/scratch/sweep_vk |
| K6b | 597c81ba4b3dcf9a1154ef438a3c827054a57ad9 | 40 | <wt>/scratch/sweep_k6b |
| KF3 | aa83f67969c2f618034b856ba6e1fc13ae10762e | 40 | <wt>/scratch/sweep_kf3 |
| KF2 | 522167ac62f27ad999a4416d10b95f922ff8c665 | 40 | <wt>/scratch/sweep_kf2 |

There are **238 logs**, plus each sweep's manifests.txt, original SWEEP record,
meta.txt, suites.log and suites_vs_baseline.txt: **268 source files**, totaling
7,484,465 raw bytes and 7,361,427 sanitized bytes. The largest sanitized sweep
file is 970,053 bytes. The canonical sanitized copies are under `sweeps/`.
Original byte copies are retained under `<LOCAL>/raw/sweeps/`.
ORIGINALS.tsv and SANITIZED.tsv map every file to its byte size and SHA256.

Every SWEEP record names its audited final candidate and a clean working tree.
Its original hash matches the original-hash record already in the corresponding
T3/IMPLEMENTATION/*_MERGE/dec025 folder. All 238 logs reproduce their committed
suite-summary test totals. Their failed-test lines recover the same three named
Mac failures in each slice; checks/FAILURE_IDENTITIES.tsv records them without
changing their historical dispositions. Earlier K4 and KF1 attempts, including
K4's dirty first attempt, are separately identified in checks/EARLIER_ATTEMPTS.tsv;
their suite logs were not selected or duplicated, and their original paths remain.

| KF2 part-1 side | Restored bytes | Records | Gzip bytes |
|---|---:|---:|---:|
| base | 605,711,611 | 884 | 26,557,740 |
| candidate | 605,711,700 | 884 | 26,553,914 |

The canonical compressed copies proposed for Git are
`gate/part1_base.runs.jsonl.gz` and `gate/part1_cand.runs.jsonl.gz`.
GATE_ARCHIVES.tsv contains raw, sanitized and compressed SHA256 values and byte
sizes. Both restored hashes match KF2's pre-existing
`_run_records/b/gate/uncommitted_sha256.txt`:

- base: `c42981e541650578a60efe48cbf346360fedbb4507b3d02365b5f5abb78e9f68`
- candidate: `11dfe8296681280e0fd4163ee3195cc16ff0f78f3ce81dff82516a520bf2ef3f`

Raw and sanitized JSONL content is byte-identical: no matching host path or ANSI
sequence was present. Each is therefore represented once in the Git packet.
Verified raw and sanitized gzip copies also remain under `<LOCAL>/raw/gate/`
and `<LOCAL>/sanitized/gate/`; their duplication is local only.

## Provenance and transformation

The sweep's historical working tree was `<wt>/sweep-skewpin`; its Piping root was
the run-from directory for dec025_mac.sh and the manifest-relative cargo commands.
The common target was `<wt>/sweep-skewpin-target`. The original driver is already
preserved at T3/IMPLEMENTATION/M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt.
The previously external run_suites_nff.sh is copied, unchanged, under
`provenance/` as text, with its raw local copy and source hash retained.
These scripts are historical evidence only and were not executed here.

KF2 part-1 is the historical B gate at base `78f55f927` and implementation
candidate `1b10121fa`; it is not a newly executed gate at final merge candidate
`522167ac6`. Its retained SUMMARY, scripts, indexes and merge record explain
the original run and the historical argument carrying B forward to the final head.
The source JSONL paths are `<wt>/scratch/i20/b/gate/part1_{base,cand}/runs.jsonl`.

Original bytes are never rewritten. A line-preserving Perl filter replaces the
historical VENV, T3 worktree root and home prefixes, in that order; maps Darwin
temporary prefixes to <tmp>; strips terminal OSC and CSI sequences. The exact
filter and commands are in COMMANDS.md. No numerical text, line order, test result,
or JSON value is otherwise transformed. Gzip uses `-9 -n -c`: no original filename
or timestamp is embedded. All sanitized suite copies were reproduced from the
unchanged originals and byte-compared. Gzip integrity, decompression to original
bytes, original recorded hashes, JSON syntax/object type and 884 line counts were
checked. checks/ carries the completed outputs.

## Size disposition and custody

The existing D-GOV-45 checker warns above 5,000,000 bytes; it does not prohibit
such files. ROOT explicitly accepted these two existing-policy size warnings in
its follow-up and directed one sanitized gzip per side into this packet, provided
each was below 100 MB. Both are below 100,000,000 bytes. No LFS or alternate
storage system was introduced. This is an execution disposition, not an amendment
to the repository's size policy. checks/SIZE_DISPOSITION.tsv records the two warnings.

At TASK return, all copies are **verified local custody only**. Nothing in this
TASK was committed, pushed, uploaded or remotely verified. Git recovery of these
logs and gate bytes is pending ROOT's review, commit and push of this complete
packet. The locally retained raw suite copies still contain historical host paths;
sanitized suite content and exact original hashes are proposed for Git. Restoring
the byte-exact unsanitized suite logs still requires that local custody copy.

Keep all original scratch paths and the current sweep_kf2 baseline intact.
The part-1 full/ and envelopes/ directories are not copied by this bounded task;
their original paths remain, and no claim is made that these directories were
independently restored. No pruning authorization is created.

AUD-T3-04 should not be marked fully closed solely by this local preservation.
ROOT must complete integration/durability and retain the separate maintained
generator-input/sparse-checkout reproducibility issue. Earlier attempt logs and
byte-exact unsanitized suite custody remain local. No historical evidence,
acceptance decision, source code or prior sealed record was amended.

## Restore and seal

From the packet directory run `shasum -a 256 -c SHA256SUMS`.
To restore either gate file, create a destination directory under an authorized
location, then run (replace SIDE with base or cand):

```sh
gzip -t gate/part1_SIDE.runs.jsonl.gz
gzip -dc gate/part1_SIDE.runs.jsonl.gz > <RESTORE>/part1_SIDE.runs.jsonl
shasum -a 256 <RESTORE>/part1_SIDE.runs.jsonl
wc -c -l <RESTORE>/part1_SIDE.runs.jsonl
```

Compare the result with GATE_ARCHIVES.tsv. Sanitized suite logs are ordinary files
and require no extraction. Byte-exact raw suite restoration is a copy from
`<LOCAL>/raw/sweeps/`, checked against ORIGINALS.tsv. The local custody directory
also carries a complete packet copy and its own SHA256SUMS.

The packet seal covers completed preservation checks and the return, not future
ROOT integration, independent review, remote retention or numerical acceptance.
