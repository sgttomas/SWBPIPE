# D-73 — ruling addendum: gate evidence cited, not re-run

Status: owner direction of 2026-09-21, recorded by HELP_HUMAN Agent 0 (Claude
Code, Claude Opus 5). It amends one execution parameter of
[the D-73 ruling](D-73_RULING_2026-09-21.md), Item 6. The ruling record is
unchanged; this addendum governs where the two differ.

## Owner acts

Stored-transcript custody for this session
(`efe0b4ff-c1b6-4a97-af70-b04220464651.jsonl`), UTF-8, no trailing newline;
not original transport bytes.

2026-09-21T17:24:42.046Z (entry 904), SHA-256 `1baa99b73b9c267c9ff176607b387c2ca7fc9fd967d38b0bedc1645c5b88e245` (88 bytes),
asked while Agent 0 had started the R0 gate run:

~~~~
why are you doing a full 60 minutes CI run for what was essentially a plan being merged?
~~~~

Agent 0 explained that the run was the R0 gate transcript, not CI for the
merge, that its script duplicated the sweep's surfaces, and offered a trimmed
run or citing existing records. 2026-09-21T17:25:45.749Z (entry 925), SHA-256
`41221e64eb4870ff9e3b47a6fcefc426daf471745b9dcf69fad03dee8233d8ab` (140 bytes):

~~~~
I think you can just site the existing sweep and CI records you've just obtained.  What value is there in running even a trimmed test suite?
~~~~

## Effect

D-73 Item 6's "one fresh gate transcript at R0" is replaced: **the run runs no
test suite.** Its verification evidence is the existing records bound to the
frozen Piping code, copied with their hashes into the run's evidence folder
under `GATE_EVIDENCE/`:

| Record | Binding to the frozen Piping tree `38cbfc64be7e55126c90b82a266846815aa52efb` |
|---|---|
| Hosted CI on PR #834 head `339be6c9ee6616dfd60952957ee193f897bd05aa` (all 8 checks passed before merge; runs `35614512667` governance-harness and `35614512716` Piping Desktop E2E succeeded) | The head's Piping tree is the same tree object |
| Clean local DEC-025 sweep on candidate `9d55bce37a3e683d6a1e026920855270b8929113` (five surfaces PASS) | Differs from the frozen tree only under `execution/`; every product path is byte-identical |

The partial gate run Agent 0 had started (npm install, harness self-check,
part of the harness tests) was stopped and discarded; it is not evidence.
Workers cite suites and tests from these records and from reading the test
files; per-test pass status at the frozen commit is not asserted beyond what
the records show. Native desktop behaviour remains cited from existing native
witnesses. The evidence checkout of the frozen state is unchanged.

Nothing else in D-73 changes. Standard claim fence applies (F-PIP-2; claims
taxonomy per DEC-081).
