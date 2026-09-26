# Workflow-library naming and wording fixes (wave 3)

Owner-authorized Root tranche `ROOT-WORKFLOW-WAVE3-NITS-20260926` applies deferred naming and wording fixes from the earlier workflow-library waves:
- `scope-change-packet`: the `Proposed_SCA_Actions.csv` column `SCOPE_CHANGE_Gate` is renamed `SCOPE_CHANGE_Checkpoint` (values are checkpoint groups). `tools/validation/validate_scope_change_packet.py` requires the new name for new packets and still accepts `SCOPE_CHANGE_Gate` on read, so existing packets stay valid and are not rewritten.
- `tools/scaffolding/create_snapshot_folder.sh` never reuses an existing folder. When `{PREFIX}_{LABEL}_{YYYY-MM-DD}_{HHMM}` already exists (for example, a second run in the same minute), it creates `..._{HHMM}_{NN}` (02 to 99) and prints that path; it exits 1 rather than reuse a folder. Callers must use the printed path. `review` already does.
- `tools/scaffolding/write_status.sh` resolves a `--ruling` path physically (`pwd -P`), so the recorded `Authorization Basis` is repo-relative under a symlinked working directory. Guard behavior is otherwise unchanged.
- `project-setup` names coordination representations as `docs/TYPES.md` §6 does (`SCHEDULE_FIRST | DEPENDENCY_TRACKED | HYBRID`), separate from the dependency tracking mode. Records carrying the former labels remain readable as written.
- `reconciliation` wording: an owner-delegated application stays within its words "and this run" in the contract (matching the method); some long lines are rewrapped; the two representation-migration profiles now cross-reference each other. `bounded-reconciliation` acknowledges `review`'s read-only candidacy-comparison use.
- `project-decomp` and `domain-decomp` gain the Precedence section `software-decomp` has; `software-decomp` notes that `validate_id_format.sh` also accepts 3-digit widths; several workflows name the `preparation` skill instead of a `preparation` workflow.

Runs pinned to earlier `reconciliation` bytes (D-73) keep that basis until this loop adopts the revision. Historical run records are not rewritten. This loop decides its own adoption; this source tranche grants no release.
