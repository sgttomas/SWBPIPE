# Run basis — owner treatment rulings and TM-PIP-040 LOST outcome

## Status and identity

- Status: `ACTIVE — OWNER RULING RECORDING AND OUTCOME IMPLEMENTATION`.
- Run ID: `OWNER_RULING_2026-08-09_TM_PIP_038_040`.
- Parent: `/root`, acting as `HELP_HUMAN` Agent 0.
- Manager: `/root/reconciliation_tm040_lost_ruling`, acting as managed
  `RECONCILIATION` Agent 1.
- Selection authority: direct owner acts supplied by Ryan C Tufts on
  2026-08-09 in the supervising session.
- Posture: `MIXED`; this run records accepted treatment rulings, one owner
  evidence-outcome act, and one future Task Management closure proposal. It
  does not reopen discovery or authorize register disposition.

## Frozen source state and ancestry

- Repository root resolved from this checkout with
  `git rev-parse --show-toplevel`.
- Working root: `projects/chirality-piping` under the resolved repository.
- Clean launch `HEAD` and fetched `origin/main`:
  `38801d299b19b36f40009714d2c7015db0bd6484`.
- The treatment commit
  `7c8cac7ae93204f5a5903f732755d60e65ab1a50` and activation commit
  `3f00a351695ec3943be6d60a89643795a28f9220` are ancestors of the frozen
  source state.
- The activation record is present at the frozen source state with Git blob
  `e8ee259b46f0ca4fa5a235c9f5ea9a5991c279e8` and SHA-256
  `e8ef649f54145e8c82b1d45bcce31bea2ec9f15d30f45bda7a464cd752f1309e`.
- Receipt 96 is present in `projects/chirality-piping/loop/LOOP_RECEIPTS.md`;
  the full receipt file is Git blob
  `1759f5a8e0f57d23d9f899f532d514d50b0f95a1`, SHA-256
  `d7ddc93a731038dabbd718625b87f1119f9a9baa691a6d3df88b8ad9905a962e`.

## Frozen authorities

| Surface | Git blob | SHA-256 |
|---|---|---|
| `AGENTS.md` | `826bccb9cf33a2079921fc9ef36fc7d63bd72e85` | `268becd0bac9da8421b30089e4e4167a5e5f79bf3892d0f72ad41a63180a3aeb` |
| `projects/chirality-piping/AGENTS.md` | `79a70b4ac8e4a53ba2842290fda62aab982887bb` | `9e5dec7df1a56f05ff947ad8fea982613e7350e5b4833b4be4f69875a922fb2b` |
| `agents/AGENT_RECONCILIATION.md` | `a358ab08c5fb22e77c9c9a8353d95a32505e6eb7` | `46bca06f907c4da765b1b1177ecd51c6858fdf45bf7620341175c3b847a3e4f7` |
| `docs/DELIVERABLE_CONCORDANCE_METHOD.md` | `137209cb37e8d8204a7f2bd78114b4b5753c6c2e` | `abf3e78fce606c4557d61cdbfbdb7292a3d858838f6526da6b433d1bcd0ef627` |

## Accepted treatment inputs

The accepted upstream treatment package is the 18-file derivative merged by
PR #533. Its terminal bindings at this basis are:

| Surface | SHA-256 |
|---|---|
| `RUN_BASIS.md` | `d7d2ad304c7b9f605f7549156327638edaf8a2dcad2d0e656138daba8902525e` |
| `RUN_RECORD.md` | `9ec5525b0960ec437012eae152b45fee05c2ce7088d6189ad7fac37e09257394` |
| `TREATMENT_VERDICTS.md` | `d945d99f7f337cb8c6b4584a9a7439cc7cad035e5031d7dc72645d3ac6d96064` |
| `VALIDATION_BACKCHECK.md` | `3ee34b465655f7a00c92451aac595760110ad9f11f08651cdf6abca5b980ac41` |
| `HANDOFF_STATE.md` | `519e93487bc136359818e5a1394eee93cce09af129ede220dc8db6ddf07b1259` |
| `DECISION_PACKETS/TM_PIP_040_OWNER_DECISION_PACKET.md` | `3ab98c5127bd31af4ea9a2f5646d784582f4ca4d177572ce247279e0ac7c467b` |
| `SOURCE_EVIDENCE/TM_PIP_040_PROVENANCE_INVESTIGATION.md` | `fc247435dd8734dfb94aabce4e268e00cb554ace314154ce89453cf31b52ac36` |

All 19 pre-existing tracked files under the parent treatment root, including
the activation record and the accepted 18-file derivative, are immutable
inputs. The intentionally blank ruling form in the accepted TM-PIP-040 packet
remains blank and is not recoded. This new subpackage is a later immutable
derivative implementing the packet's `LOST` on-ruling mechanism.

## Owner acts and bounded implementation

1. `TM-PIP-038`: owner accepts the amended preservation treatment. The
   correction stands as the derivative record; the accepted snapshot remains
   immutable; no direct edit is authorized.
2. `TM-PIP-039`: owner accepts the supersession record as executed.
3. `TM-PIP-040`: owner accepts the investigation and packet and selects
   `LOST` on a supplied personal-act basis.
4. Further recovery is declined. Historical test results and ledger encodings
   remain the evidence of record and are not invalidated.
5. `TASK_MANAGEMENT` is directed only to propose closure of `TM-PIP-040` as
   `RESOLVED_BY_DECISION` at a later closure session. No register disposition
   is applied by this run.

The exact owner words are preserved in `OWNER_TREATMENT_RULINGS.md`.

## Write and execution fences

- Sole persistent write root:
  `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/OWNER_RULING_2026-08-09_TM_PIP_038_040/`.
- Every previously merged treatment file, historical D-41 file, package
  summary, Task Management register, receipt, decision, lifecycle surface,
  evidence object, and every other repository path is read-only.
- No historical object is restored, deleted, copied, regenerated, or treated
  as reconstructable from Git.
- No Git stage, commit, push, PR, merge, fetch, reset, rebase, or clean is
  authorized for this manager.
- No closure direction is invented for `TM-PIP-038` or `TM-PIP-039`.

## Rerun triggers

Rerun before acceptance if the frozen source state moves, any accepted input
hash changes, the owner's quoted ruling fails byte-fidelity validation, any
write escapes the declared root, or any protected source/register/receipt
surface changes. A later Task Management closure session consumes this
subpackage as evidence; it does not rewrite it.
