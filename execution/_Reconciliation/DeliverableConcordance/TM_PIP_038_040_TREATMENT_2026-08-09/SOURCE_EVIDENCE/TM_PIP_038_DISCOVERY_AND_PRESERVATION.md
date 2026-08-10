# TM-PIP-038 discovery and preservation evidence

## Status and bounds

- Result: `PASS — DEFECT SUBSTANTIATED; HISTORICAL EDIT PROHIBITED`.
- Item: `TM-PIP-038` only.
- Parent: managed `RECONCILIATION` Agent 1
  `/root/reconciliation_tm038_040_treatment`.
- Executor: one non-delegating ephemeral Agent 2 specialist
  `/root/reconciliation_tm038_040_treatment/tm038_discovery`.
- Frozen source state: `da40d7dc4192c9aa2f49e9438729179aae281b61`.
- Method: committed-file and Git-object inspection only. No meaning was
  inferred from a model name; every attribution below is an explicit
  assignment or execution statement in committed evidence.
- Mutation boundary: no historical package summary, register, evidence
  outcome, or Git state was changed. This file is a new derivative evidence
  record, not a rewrite of the accepted discovery snapshot.

## Input bindings

| Input | Git blob | SHA-256 | Bytes | Relevant evidence |
|---|---|---|---:|---|
| `DELIVERABLE_CONCORDANCE_2026-07-11_1305/PACKAGE_SUMMARIES/PKG-06.md` | `1740d083616599990c7412e675deb1c3704a3d7f` | `8bb6e1b5366447d3fdadfcd83ff63818b60493401a5ffc8c1064640b0130193a` | 7,101 | lines 17–18 |
| `DELIVERABLE_CONCORDANCE_2026-07-11_1305/PACKAGE_SUMMARIES/PKG-07.md` | `78607755f645967d4fcebb5d5bd484d65cffabc8` | `b3783ffee3068fa9a802b18763b7092ae01cd6c9d6f2f8ef75ca4d3f29b225b4` | 7,730 | lines 18–19 |
| `DELIVERABLE_CONCORDANCE_2026-07-11_1305/PACKAGE_SUMMARIES/PKG-08.md` | `9d88043e410df474813e3f1e8648f6ccbabc9faf` | `fc31720f56ef2d1c1b21652d18332903d0b383f3a47ef680999e5f470a864863` | 7,341 | lines 18–19 |
| Historical `DELIVERABLE_CONCORDANCE_2026-07-11_1305/RUN_BASIS.md` | `f4d8a44324e8a8bdb6edb74577d05f0d32aac44a` | `c5655c4c43ada8edb3b3cc71a1dbf15ffded92d64d910f1c741e572632d25e61` | 56,582 | lines 179–190 and 303–310 |
| `loop/LOOP_RECEIPTS.md` | `838e623b1f81769b3c0ee711928536bbc6345868` | `1371b8b644d7acfa34f2dd247ef7169d7ad8d82d9f9c2ea408a016533de8ee3f` | 169,362 | Receipt 17 lines 482–490; Receipt 24 lines 614–620; Receipt 25 lines 713–743 |
| `execution/_Coordination/NOTICE_2026-08-03_ROOT_PIPING_RESUME_RESIDUALS.md` | `ac5086f3e02a72bd598fe6ba46eec03e65176f49` | `8fa5303c7387dc545c5360443f640ba2c7b6ae646c37d40719b140e880204125` | 2,818 | item 1 |
| Routed handoff | `5e05568fe2276f753858bffb993d98109a12d9a4` | `7bca6073f2ba9aa1c4350ee694e979fb1b04fee561ab09329ba09a1ae3ebdd30` | 4,807 | TM-PIP-038 section |
| Activation record | `e8ee259b46f0ca4fa5a235c9f5ea9a5991c279e8` | `e8ef649f54145e8c82b1d45bcce31bea2ec9f15d30f45bda7a464cd752f1309e` | 4,997 | activated-scope and write-fence sections |

The three package-summary blobs match the exact targets pinned in the routed
handoff. The worktree versions and `HEAD` versions are byte-identical at the
frozen source state.

## Factual finding

Each target carries the same false logical sentence:

Escaped byte representation (`\n` denotes the one embedded LF; there is no
terminal LF):

```text
All pilots fable per the\nReceipt-17 steer.
```

The committed record distinguishes the roles instead:

1. Historical `RUN_BASIS.md` lines 183–189 records the Receipt-17 steer as
   `opus discovery pilots for W1/W3/W5` and `fable-at-high-effort per-wave
   fan-in verification`.
2. The same file lines 303–310 records W3 as 19 ledgers across PKG-06,
   PKG-07, and PKG-08, explicitly says `opus pilots per the Receipt-17 steer`,
   and then separately identifies `Fan-in verification (fable, high effort,
   one verifier per package)`.
3. Receipt 17 lines 482–490 preserves the owner steer verbatim, including
   `opus for R2 discovery pilots in waves W1-W5` subject to exceptions that do
   not include PKG-06, PKG-07, or PKG-08.
4. Receipt 24 lines 614–620 records the completed W3 execution as 19
   `opus` pilots and its separate `fable` fan-in.
5. Receipt 25 lines 713–719 expressly preserves the earlier steer as immutable
   history of how W1–W3 were actually executed, while rescinding it only
   going forward. Lines 739–743 independently identify these three sentences
   as copied-W2 factual defects and preserve the fan-in statements as correct.

Verdict: all three sentences falsely attribute the W3 discovery pilots to
`fable`. The correct discovery-pilot attribution is `opus`. The later fan-in
statements in the three summaries are correct and require no correction.

## Exact before bytes and corrected projection

The defective logical segment is identical in all three files. It is 42 bytes
including one embedded LF byte and no terminal LF; the next source byte is a
space before `Verification`.

```text
All pilots fable per the\nReceipt-17 steer.
```

In this escaped representation, `\n` denotes the LF byte and is not two
literal payload bytes.

UTF-8 hexadecimal:

```text
41 6c 6c 20 70 69 6c 6f 74 73 20 66 61 62 6c 65 20 70 65 72 20 74 68 65 0a 52 65 63 65 69 70 74 2d 31 37 20 73 74 65 65 72 2e
```

The bounded corrected projection is 51 bytes including one embedded LF byte
and no terminal LF:

```text
All discovery pilots opus per the\nReceipt-17 steer.
```

Again, `\n` denotes the LF byte and is not two literal payload bytes.

UTF-8 hexadecimal:

```text
41 6c 6c 20 64 69 73 63 6f 76 65 72 79 20 70 69 6c 6f 74 73 20 6f 70 75 73 20 70 65 72 20 74 68 65 0a 52 65 63 65 69 70 74 2d 31 37 20 73 74 65 65 72 2e
```

For reproducibility, the following whole-file after-identities were computed
in memory by replacing exactly that one 42-byte segment with the 51-byte
segment and changing no other byte. `git hash-object --stdin` was used without
`-w`; these are projected object identifiers, not written Git objects or file
edits.

| Target | Before blob | Projected after blob | Before SHA-256 | Projected after SHA-256 | Before/after bytes |
|---|---|---|---|---|---:|
| `PKG-06.md` | `1740d083616599990c7412e675deb1c3704a3d7f` | `2c088be65e46ed2afd1c5a97a948b8d82de24047` | `8bb6e1b5366447d3fdadfcd83ff63818b60493401a5ffc8c1064640b0130193a` | `8913e07c8c36b2b1257a02e03a46f57eb7111f0c9660e773ed0fd24205022788` | 7,101 / 7,110 |
| `PKG-07.md` | `78607755f645967d4fcebb5d5bd484d65cffabc8` | `cc507cf7aeaf8bd0d4d7740e7daae87efdb4289c` | `b3783ffee3068fa9a802b18763b7092ae01cd6c9d6f2f8ef75ca4d3f29b225b4` | `2ddffe69aafe11ed53bd4473c3983cfcaa3d1d112955ac1f4ce55f3f00673972` | 7,730 / 7,739 |
| `PKG-08.md` | `9d88043e410df474813e3f1e8648f6ccbabc9faf` | `3ab38dcf647b7d1f5d64dd412b4107d89caec88b` | `fc31720f56ef2d1c1b21652d18332903d0b383f3a47ef680999e5f470a864863` | `56de2551dde231640c8a28e72066a16c0447b478643e85714496410ece00ebc7` | 7,341 / 7,350 |

This projection is exact before/after evidence only. It does not authorize or
represent an edit to any historical summary.

## Preservation analysis

The conditional historical-summary repair prerequisite fails:

- `AGENT_RECONCILIATION.md` SPEC 13 says the accepted discovery snapshot
  remains immutable and R6 is a new source-state-bound derivative snapshot.
- Its R6 protocol says to create a new immutable backcheck derivative and not
  rewrite the accepted discovery snapshot.
- The ratified shared method treats one run's phase state as an immutable,
  append-only run folder.
- Receipt 25 classifies `PACKAGE_SUMMARIES` as historical/evidence that stays
  frozen and explicitly records this defect without repairing it silently.
- The frozen treatment `RUN_BASIS.md` therefore makes the three historical
  summaries read-only unless an authority affirmatively proves otherwise. No
  such authority was found at the frozen source state.

Consequently, direct replacement of the false bytes in the three accepted
summary files is not permitted in this treatment. Editing them would satisfy
the factual wording request while violating the stronger preservation rule.
All historical bytes and substantive findings therefore remain unchanged.

## Lawful treatment options

1. **Recommended — adopt with preservation:** retain the three historical
   blobs unchanged and publish a new source-state-bound derivative correction
   record that cites this evidence, states the corrected discovery-pilot
   attribution, preserves the projected before/after identities, and warns
   consumers not to rely on the three historical sentences. This records the
   bounded factual correction without recoding accepted history.
2. **Defer:** retain this evidence and leave the correction unissued pending a
   later owner-selected derivative instrument. The historical defect remains
   visible and must continue to be carried as a blocker/caveat.
3. **Historical rewrite:** not lawful on the frozen authority. It would need a
   new explicit governing change path that overrides the accepted-snapshot
   preservation rule; the present activation does not supply one.

Treatment recommendation to the parent manager:
`ADOPT_WITH_PRESERVATION — CORRECTION BY NEW DERIVATIVE RECORD; NO HISTORICAL EDIT`.
This is an agent treatment verdict for owner consideration, not a register
disposition or owner ruling.

## Changed-path manifest

This Agent 2 created only:

1. `SOURCE_EVIDENCE/TM_PIP_038_DISCOVERY_AND_PRESERVATION.md`
2. `CHILD_RETURNS/TM_PIP_038_RETURN.md`

All paths are relative to
`execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/`.
The three historical summaries and every other repository path remained
read-only.
