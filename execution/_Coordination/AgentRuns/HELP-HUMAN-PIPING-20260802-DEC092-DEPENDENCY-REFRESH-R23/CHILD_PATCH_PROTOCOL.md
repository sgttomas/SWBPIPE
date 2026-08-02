# R23 child patch protocol

This protocol is frozen for N2A–N2E.

- Each child is an ephemeral Agent 2 with the exact parentage and scope in its
  `LAUNCH_BRIEF.md`. Agent 2 does not delegate.
- Allowed tools: file read and `apply_patch` only. Bash, shell, Python, Git,
  network, broad search, and any write outside the two exact consumer targets
  are prohibited.
- A row may close only when both its target-maturity record and the named
  consumer-integration evidence pass. Do not infer closure from target maturity
  alone. Do not close a row whose disposition is `UNCHANGED` or `HOLD`.
- For a `CLOSE` row, change only `SatisfactionStatus` to `SATISFIED`, `LastSeen`
  to `2026-08-02`, and append the exact launch-brief note to `Notes`. Preserve
  every other CSV field, row, ordering, quoting convention, and newline.
- For the one `NORMALIZE_ANCHOR` row, change only `SatisfactionStatus` to
  `NOT_APPLICABLE`, `LastSeen` to `2026-08-02`, and append its exact note.
- Every `UNCHANGED` or `HOLD` CSV row must remain raw-row byte-identical.
- In `_DEPENDENCIES.md`, change only the lifecycle satisfaction counts named in
  the launch brief, append the exact run-history item, and append the exact
  downstream-handoff item. Do not rewrite any other index content.
- Do not write the managed run triplet. PROJECT_SETUP owns `LAUNCH_BRIEF.md`,
  `STATUS.md`, and `RETURN.md` and will record the child's terminal return.
- Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
- F-PIP-1 through F-PIP-5 remain in force. No lifecycle transition, DAG write,
  `_LATEST` update, decision/register write, status/memory/product write,
  evaluation, receipt, commit, push, PR, or merge is authorized.

