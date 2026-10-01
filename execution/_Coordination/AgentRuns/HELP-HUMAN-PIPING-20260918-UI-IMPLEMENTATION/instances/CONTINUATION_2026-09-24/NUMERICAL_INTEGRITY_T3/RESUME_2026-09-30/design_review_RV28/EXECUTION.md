# RV28 execution receipt

Canonical review packet: <A1_WT>/<R>/design_review_RV28.
Role TASK; parent /root; native child /root/rv28_a1_design. Prompt-enforced
shared-host write boundary, not an OS sandbox. No delegation or model-diversity
claim. Parent's final clarification confirms preserving SI normalization and
requests an additive author correction plus this same reviewer's backcheck;
no new contract/domain decision was accepted.

The initial instruction/source reads used exec_command with cat, rg, nl/sed
and read-only git show/rev-parse. Applicable instruction origins/hashes and
every source relied on are in BASIS.json. REVIEW.md records the precise source
locations supporting the findings and derivation. Relevant bounded excerpts
were read after broad discovery output truncation. No broad governance body,
unrelated role, workflow or old conditional design was activated.

The recorded exact run was:

    <VENV>/bin/python -B <A1_WT>/<R>/design_review_RV28/review_checks.py \
      <APP_WORKTREE> <COORD> <A1_WT> <A1_WT>/<R>/design_review_RV28 \
      > <A1_WT>/<R>/design_review_RV28/CHECK.stdout.json \
      2> <A1_WT>/<R>/design_review_RV28/CHECK.stderr.txt

Exit 0. Raw stdout is CHECK.stdout.json; stderr is empty.
The script's actual Git argv, exit statuses, raw-output hashes and stderr are
in COMMANDS.json. It supplies GIT_OPTIONAL_LOCKS=0 for every Git invocation.
Git show outputs are recoverable verbatim from the recorded object/path;
they are not copied as a duplicate source library. The exact-check details are
EXACT_CHECKS.json. Python platform/version/executable hash are in BASIS.json.
The first successful scratch run had 21 basis records and the same 46 checks;
the canonical run added the D1 base-design file as the 22nd record. No failed
execution or numerical repair was needed.

Writes: this packet only, plus scratch/rv28-a1-design/review_checks.py, which
is an earlier working copy of the canonical review script. The canonical copy
is the packet's review_checks.py. No source, seal, index, branch, build target,
guard or host configuration was changed. Directory creation and file writing
stayed inside the granted review and scratch subtrees.

SHA256SUMS inventories every packet file except itself. Its digest is returned
to ROOT. The packet is sealed after the recorded verification completes; later
author fixes require an additive backcheck and must preserve these bytes.
