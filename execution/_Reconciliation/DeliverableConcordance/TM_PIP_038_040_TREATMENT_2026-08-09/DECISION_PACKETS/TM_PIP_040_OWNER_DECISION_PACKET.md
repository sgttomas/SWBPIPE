# Owner decision packet — TM-PIP-040

## Question presented

What disposition, if any, does the owner assign to the six Addendum-9 ignored
artifact sets formerly present in the D-41 frozen evidence worktree?

This packet is decision support only. It does not make, pre-fill, or perform a
`RESTORED`, `LOST`, or `UNDETERMINED` act. Register disposition remains with
`TASK_MANAGEMENT` under a separate owner closure ruling.

## Bound evidence

- Frozen treatment source state:
  `da40d7dc4192c9aa2f49e9438729179aae281b61`.
- Investigation:
  `SOURCE_EVIDENCE/TM_PIP_040_PROVENANCE_INVESTIGATION.md`.
- Historical run basis: Git blob
  `f4d8a44324e8a8bdb6edb74577d05f0d32aac44a`.
- Final historical run summary: Git blob
  `f2c789f33e247acda79024b3d005732fdbc9a0ab`.
- Last positive frozen-set closeout commit:
  `9b8b6c91966fdd4aab1988ef422176239cc11651`.
- Subsequent absence carrier: Git blob
  `ac5086f3e02a72bd598fe6ba46eec03e65176f49`.

## Facts established

1. The six exact ignored sets were positively observed in the frozen worktree
   through the 2026-07-12 closeout.
2. The orchestrator's contemporaneous deletion attempt was blocked and not
   worked around; restoration remained described as an owner act.
3. Later committed records say the frozen worktree no longer exists and no
   outcome record was found.
4. The worktree path and current Git worktree registration are absent now.
5. The sets were ignored and untracked. No original content hashes, sizes,
   custody-linked archive, clean/recreation output, or removal record was
   located.
6. Current lookalikes in the primary checkout are later filesystem objects
   born 2026-07-31; the two corresponding lockfiles are absent. They do not
   establish identity or custody.
7. The pinned tracked tree remains reproducible from Git, but recreating that
   clean tree would not determine what happened to the former ignored objects.
8. The historical incident record expressly says the test results and ledger
   encodings were not invalidated.

## Evidence assessment

The present evidence does not factually establish a particular disposal
outcome. The missing worktree narrows current recoverability but does not
identify the intervening act. If the owner chooses to rule from this record
without supplying new evidence, the uncertainty-preserving option is the one
that does not assert an unproved physical history. That assessment is advice,
not a selected outcome or owner act.

## Owner options and consequences

| Owner option | Minimum evidentiary basis | Consequence and risk |
|---|---|---|
| `RESTORED` | A verbatim owner statement or contemporaneous record that the frozen tree was scoped-cleaned or recreated from `551f84ef6...`, with date/mechanism and any available command output. | Closes the physical-contamination question if accepted. Without that added basis, it would overstate what the repository and present filesystem prove. It does not alter the historical incident record. |
| `LOST` | A verbatim owner acceptance that the former ignored filesystem objects were discarded or are no longer recoverable, with any known removal mechanism. | Acknowledges loss of those generated objects. No test/ledger invalidation follows automatically because the historical run expressly preserved their results; no original byte identity can be reconstructed from Git. Without an owner statement or removal evidence, it would infer mechanism from absence. |
| `UNDETERMINED` | The current bounded investigation is sufficient to establish that no outcome is proven; owner may also identify specific future evidence triggers. | Preserves epistemic accuracy and permits later reconsideration. The evidence-outcome question remains unresolved unless the owner separately directs closure on an accepted uncertainty basis. |
| `DEFER — request evidence` | Owner names an expected recoverable source, custodian, backup/snapshot, or personal recollection to be captured. | Avoids a premature outcome but leaves the decision open. Investigation must remain bounded and source-state-bound. |

## Exact on-ruling mechanisms

No mechanism below is activated by this packet.

### If the owner selects `RESTORED`

1. Supply a verbatim ruling that names the six-set population, the restoration
   mechanism (`scoped clean` or `recreated frozen tree`), and when/by whom it
   occurred.
2. RECONCILIATION records that ruling and its supplied evidence in a new
   immutable outcome record under this treatment run; historical D-41 files
   remain unchanged.
3. `TASK_MANAGEMENT` separately proposes and, only on owner closure ruling,
   applies the register disposition.

### If the owner selects `LOST`

1. Supply a verbatim ruling accepting that the original ignored filesystem
   objects are unavailable, including any known removal mechanism and whether
   further recovery is declined.
2. RECONCILIATION records the owner act without rewriting the incident or
   claiming test/ledger invalidation.
3. `TASK_MANAGEMENT` separately handles register closure under the owner's
   closure ruling.

### If the owner selects `UNDETERMINED`

1. Supply a verbatim ruling that the available evidence is insufficient and
   state whether the matter remains open, is deferred to named evidence, or is
   accepted closed on uncertainty grounds.
2. RECONCILIATION records the owner act and exact rerun trigger without
   inventing physical history.
3. `TASK_MANAGEMENT` separately applies only the register disposition the
   owner expressly rules.

### If the owner defers for more evidence

Name the exact source or custodian and an expiry/review trigger. A future
investigation may inspect only the supplied source and must preserve hashes,
origin, and custody information before returning a refreshed packet.

## Owner ruling form — intentionally blank

```text
TM-PIP-040 outcome selection: ______________________________

Evidence or personal-act basis: ____________________________
____________________________________________________________

Known mechanism and date (if any): _________________________
____________________________________________________________

Recovery/reinvestigation direction (if any): ______________
____________________________________________________________

Separate TASK_MANAGEMENT register-closure direction:
____________________________________________________________
```

## Reserved effects

The owner ruling does not by itself edit the Action Item register, rewrite
historical evidence, restore/delete/copy files, validate product behavior,
advance lifecycle, authorize release or reliance, or merge this treatment PR.
Those effects remain with their owning instruments and gates.
