# Validation and backcheck — TM-PIP-038 through TM-PIP-040

Status: `PASS`
Frozen source state:
`da40d7dc4192c9aa2f49e9438729179aae281b61`

## Independent fan-in

Independent non-delegating Agent 2 verifier:
`/root/reconciliation_tm038_040_treatment/fan_in_verifier`.

- Brief SHA-256:
  `1faf45ab9327161fb85b05bc3901d80d79bb76765e198765cbd184c8dc5b6acb`.
- Return:
  `CHILD_RETURNS/FAN_IN_VERIFIER_RETURN.md`.
- Return SHA-256:
  `0e2e6b36db14b3e40816a600bdee298fd9de65592ce9377d8d0cad6792aa34a7`.
- Verdict: `PASS`; no verifier repair, delegation, or Git mutation.

## Source and activation checks

- Clean launch `HEAD` and `origin/main` matched the frozen source state.
- Activation commit
  `3f00a351695ec3943be6d60a89643795a28f9220` is in its ancestry.
- Activation record SHA-256 reproduced as
  `e8ef649f54145e8c82b1d45bcce31bea2ec9f15d30f45bda7a464cd752f1309e`.
- Routed handoff and Task Management register hashes reproduced; register rows
  `TM-PIP-038` through `TM-PIP-040` remain `OPEN` and unchanged.
- All frozen authority, method, project-rider, decomposition revision `0.11`,
  DAG-009, decision, lifecycle, and adoption-plan bindings in `RUN_BASIS.md`
  reproduced.

## TM-PIP-038 backcheck

- Protected PKG-06/07/08 summary blobs and SHA-256 values reproduce exactly
  and have no worktree diff.
- Each contains one false 42-byte segment with one embedded LF and no terminal
  LF; the exact 51-byte corrected projection reproduces.
- Fresh in-memory replacement reproduces every projected whole-file blob,
  SHA-256, and byte count in the evidence record.
- Committed W3 run basis and Receipts 17, 24, and 25 reproduce the correct
  `opus` discovery / `fable` fan-in distinction.
- Preservation analysis reproduces: the accepted historical discovery
  snapshot is immutable; no direct summary edit was lawful or executed.
- Fan-in initially rejected an intermediate 43/52-byte, two-LF count. The
  owning TM-PIP-038 child corrected only its two authorized outputs; the
  manager corrected `TREATMENT_VERDICTS.md`; independent terminal
  reproduction passed. The corrected evidence SHA-256 is
  `8621cfcb1c599c4e935d3950675486ff47d5c791ea8d1eee2dae985ca9e11809`.

## TM-PIP-039 backcheck

- Historical D-41 `RUN_BASIS.md` remains blob
  `f4d8a44324e8a8bdb6edb74577d05f0d32aac44a` with no worktree diff.
- Exact W3 pause slice reproduces as 1,102 bytes, SHA-256
  `b92362c99302d77eeb7bbe82c6e52a40357d40717552e92aff3cd94372fdcb3a`.
- Exact owner quote reproduces as 188 bytes, SHA-256
  `70123ce6525f7d93cb42793ce97a0b89a4076f6f4f96975b2d880c701ec56757`.
- The eight-commit supersession chain is in the frozen ancestry; prompt blobs,
  R6/R7 completion, prompt consumption, deletion, and final absence reproduce.
- The new supersession record is derivative only and changes no historical act.

## TM-PIP-040 backcheck

- The six-set population reproduces from committed D-41 records, ignore rules,
  and the frozen Git tree.
- The last positive committed observation, later absence carrier, absent Git
  worktree registration/path, lack of original hashes/custody evidence, and
  later non-identical lookalikes reproduce with the recorded reliability
  limits.
- The owner packet's selection field remains blank. `RESTORED`, `LOST`, and
  `UNDETERMINED` appear only as conditional options/on-ruling mechanisms; no
  disposal outcome is selected or performed.

## Containment and deterministic checks

- Tracked diff: empty.
- All non-ignored untracked paths are inside the canonical current treatment
  run root; no untracked output exists elsewhere.
- Ignored-state delta: none.
- No historical summary, historical run basis, routed handoff, register,
  receipt, decision, decomposition, DAG, lifecycle, deliverable, source, test,
  or other repository path changed.
- Child manifests reconcile to two TM038 paths, three TM039 paths, three TM040
  paths, and one verifier-return path. Manager paths are separately enumerated
  in `HANDOFF_STATE.md`.
- Candidate whitespace: pass, zero binary/symlink skips.
- `git diff --check`: pass.
- Claims-language validator: pass across 269 scanned surfaces.
- Path-anchor validator: pass across 1,221 live surfaces.

This is deterministic concordance evidence only. It makes no lifecycle,
release, professional-reliance, register, or evidence-outcome claim.
