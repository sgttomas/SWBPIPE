# TM-PIP-039 supersession evidence — 2026-08-09

Status: `DERIVATIVE EVIDENCE — READ-ONLY SOURCE INSPECTION`

Frozen source state:
`da40d7dc4192c9aa2f49e9438729179aae281b61`.

## Immutable source identity

Historical source:
`projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/RUN_BASIS.md`

| Check | Result |
|---|---|
| Git blob at frozen base | `f4d8a44324e8a8bdb6edb74577d05f0d32aac44a` |
| Whole-file SHA-256 | `c5655c4c43ada8edb3b3cc71a1dbf15ffded92d64d910f1c741e572632d25e61` |
| W3 pause entry | current lines 360–375, 1,102 bytes including final newline |
| W3 pause-entry SHA-256 | `b92362c99302d77eeb7bbe82c6e52a40357d40717552e92aff3cd94372fdcb3a` |
| Exact quoted substring | 188 bytes, no added terminal newline |
| Exact quote SHA-256 | `70123ce6525f7d93cb42793ce97a0b89a4076f6f4f96975b2d880c701ec56757` |
| Working-tree preservation check | no diff from `HEAD` for the historical source |

The quote hash covers the opening and closing quotation marks, the three
embedded newlines, the two leading spaces on each continuation line, the two
spaces after `W4.`, and the backticks around the prompt path.

## Byte-exact pause entry

The following fenced payload is the exact 1,102-byte source slice identified
above. The Markdown fence is not part of the payload or its hash.

```text
- **2026-07-12 — PAUSED at owner direction after W3; handoff prepared.**
  Owner direction of record (in-session, verbatim): "Pause after W3 lands
  clean and do not set off on W4.  Instead, prepare for handoff to another
  agent in a new session.  Update the `init/piping-resume-one-time.md` file
  accordingly." State at pause: W3 complete and fully accounted for (all
  five batches, fan-in, corrections, revalidation, package summaries,
  Receipt 24, wave-boundary PR #198 opened — its self-merge under the
  standing Receipt-22 grant was blocked by the session permission layer
  and not worked around, so **PR #198 is open and its merge is the owner's
  act**); no agents in flight; nothing partial. W4 (PKG-09..12, 20
  deliverables, opus pilots per the Receipt-17 steer) is NOT dispatched.
  Resume point: dispatch W4 under `R1_CONVENTIONS.md` + the W1–W3
  calibration items (PKG-00..08 package summaries) + the addendum-9
  mitigation above, on owner direction, in a new session per the updated
  `init/piping-resume-one-time.md`. Receipt 24 records the wave boundary
  and this pause.
```

`git blame` binds lines 360–365 and 370–375 to
`92baf0b7e3a0a0e09829a766b72a6b3ebae2166a`; lines 366–369 carry the
contemporaneous PR-status correction from
`a02feaae9d0640e9a7d597db090e477fb2e08767`. The exact owner quote is wholly
bound to the first commit and is unchanged in the frozen-base blob.

## Rescission and current-runtime evidence

Commit `61c0d115832db8e7619eec7fe2cd119d313ab19d` changed only live instruction
and receipt surfaces for this issue. Its message and Receipt 25 record that
Receipt-17's named-model assignments were rescinded going forward while
remaining immutable historical evidence. The frozen-base current runtime
surface is:

- `projects/chirality-piping/loop/LOOP_INIT.md`;
- Git blob `aea1bfebb8b390acf53d2fc39535aa809be207ea`;
- SHA-256
  `a1ad58ec66928331f25ab36b6829c431fefc6421aa1ec8ccf6ff1ac1ee23989e`.

Current section 7 states that historical section references do not override
the current plan or canonical runtime instructions. This is the present
operating fence; it does not rewrite the W3 record.

## Consumed-pointer and completion evidence

| Evidence | Result |
|---|---|
| Model-agnostic prompt commit | `bdcfb7dcaf8394e3aedb3c00ebc7366a7a0f0e9c`; blob `b2107ddcc5b70104dbf1caeaca28b19fb16af92e` |
| Later W5 prompt update | `4b2a9992da6177552f261326892e138e5b546727`; blob `1f8110b8658ced7bc4ad0f3d6d53669890d768cb` |
| Last pre-deletion prompt | parent of deletion commit; blob `aaf78fa9f22b8ca7e1fccd209133a7e0485f3437`, 5,534 bytes, SHA-256 `de90e65464edf0ddc028f94ebef0ba185bc2ac75bdc0acdeefc7083c7a31f5dd` |
| R6 completion | `4d4ea08b457a80a7d925f99a4dcc45c0b9f5eaf6`; run summary says the ruled D-41 discovery/reconciliation run is closed at R6 with preserved open findings |
| R7 completion | `e2ed4d3471df38bca2371ed621ac53db19cb3fe6`; current RUN_BASIS records the bounded DEC-076/PDU-077 re-extraction complete |
| Owner-directed prompt deletion | `6bb723fc373db30206d36505fa4194d97af756ff`; deletes exactly `init/piping-resume-one-time.md` |
| Prompt in frozen base tree | absent (`git cat-file -e <base>:init/piping-resume-one-time.md` fails) |

The frozen-base D-41 `RUN_SUMMARY.md` is Git blob
`f2c789f33e247acda79024b3d005732fdbc9a0ab`, SHA-256
`15831005a983d3468bc1f9788a354390718a1bd22926cc2f8238c73de7395e0c`.
It says R0/R0b, R1, all five R2 waves, R3, and R6 are complete. The later
RUN_BASIS entry supplies the R7 completion evidence. The deletion commit
message identifies the prompt as consumed and the D-41 run as complete.

The routing notice
`projects/chirality-piping/execution/_Coordination/NOTICE_2026-08-03_ROOT_PIPING_RESUME_RESIDUALS.md`
(Git blob `ac5086f3e02a72bd598fe6ba46eec03e65176f49`, SHA-256
`8fa5303c7387dc545c5360443f640ba2c7b6ae646c37d40719b140e880204125`)
preserves the stale-W3-entry residual after deletion. It is coordination, not
authority. Piping's later owner promotion and activated treatment supply the
authority for this derivative record.

## Preservation and scope result

- The historical `RUN_BASIS.md` was read only and remains byte-identical to
  the frozen base.
- No receipt, prompt history, register, decision, lifecycle, deliverable, or
  historical reconciliation file was changed.
- The only TM-PIP-039 writes are this evidence file, the sibling
  supersession record, and the structured child return.
- These facts support a supersession record only. They do not enact a Task
  Management disposition or any owner-reserved act.
