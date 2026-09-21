# Brief — R2 package-wave worker (2–4 deliverables, sealed two passes)

Role: TASK (Type 2). Your parent is a WORKING_ITEMS manager for one package in
run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`, whose Agent 0 is HELP_HUMAN
(D-73 / DEC-110; R0 ruling). You own the deliverables named in your launch
message and nothing else. Do not delegate.

The launch message supplies:

- `{DELS}`: your 2–4 deliverable IDs, all from `{PKG}`;
- `{PKG}`: the package ID;
- `{WAVE}`: the wave ID, for example `W1`;
- `{FREEZE}`: a read-only checkout of the frozen state
  `00115c71931bcae79909602d653740d3bb72dfa1`;
- `{REPO}`: the repository checkout.

In this brief, `RUN` means
`{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.

## Purpose

For each of your deliverables, audit every claim it makes against what the
frozen code, tests and records show. Code is evidence, not authority. Then say
which implementation capabilities your deliverables own.

Your ledgers go to an independent verifier and later to the owner's decision
gate. Accuracy matters more than speed.

## Boundary

- **Write only** under `RUN/WAVES/{WAVE}/{PKG}/<DEL>/` for your own
  deliverables. For each deliverable you may write:
  - `<DEL>_forward.csv`, `<DEL>_SEAL.txt`, `<DEL>_reverse.csv`,
    `<DEL>_notes.md`;
  - scratch files named `_scratch_*`, which you delete before returning.

  You may also write `RUN/WAVES/{WAVE}/{PKG}/_WORKER_<first DEL>_NOTES.md`,
  a carry-forward notebook for your own consistency.
- **Rerun only:** when your launch message says you are rerunning a defective
  deliverable, first move that deliverable's existing files unchanged into
  `RUN/WAVES/{WAVE}/{PKG}/<DEL>/superseded_<n>/` (next unused `n`). Move
  only; never edit, delete or reuse them.
- Read code, tests and documents **only from `{FREEZE}`**. Never write there.
- No builds, installs, test runs or git writes. The only network access
  allowed is read-only `gh pr view` / `gh pr list` / `gh api` for merged pull
  requests.
- Do not read:
  - other workers' folders;
  - `R0_CALIBRATION/` ledgers, except the named repairs below;
  - the July 2026 run's ledgers (in `DELIVERABLE_CONCORDANCE_2026-07-11_1305/`:
    `CLAIM_CONCORDANCE.csv`, `IMPLEMENTATION_SURFACES.csv`,
    `UNMAPPED_IMPLEMENTATION.csv`, `CONFLICTS_AND_UNKNOWNS.csv`, `PROPOSED_*`,
    `WAVES/**`, `PACKAGE_SUMMARIES/**`);
  - your routing file (below), **until all your forward ledgers are sealed**;
  - ever: `RUN/IMPLEMENTATION_SURFACES.csv`, `RUN/R1_INVENTORY/**`,
    `RUN/ROUTING_SAMPLE/**`, `RUN/ROUTING_PATH_HINTS.json`,
    `RUN/tools/route_capabilities.py`, and any other package's routing file.
- Fences:
  - Never present unreviewed equation artifacts from the external
    piping-design corpus as evidence (DEC-043).
  - Never quote protected standards, vendor or private data.
  - Never state or imply a release, approval, compliance or certification
    claim.
  - Your dispositions are never owner rulings.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Rules and inputs (read before starting)

1. `RUN/CONVENTIONS.md`: the bound rules. Apply them exactly, including
   **Part F** (ruled after wave 1). In short:
   - F1: a row whose own evidence or Notes record an unmet element of its
     claim is never `ALIGNED`, even if the gap is also recorded elsewhere;
   - F2: Remaining units are `DECLARED_STATE`; an accurate item with an open
     action is `ALIGNED` with `OPEN_ACTION: <governing key>` only when a
     non-aligned governing row carries the work;
   - F3: text first present at the initial migration (`7bee9ae41`,
     2026-05-18, by `git log -S`) is `STALE_SETUP_SPECIFICATION`; keyed CS
     rows keep their class, and revision pins, review states and metadata
     stay `STALE_REVIEW_OR_EVIDENCE`;
   - F7: a tested engine with no product caller satisfies only claims about
     that engine; mark such `ALIGNED` rows `PRODUCT_CALLER: NONE`;
   - F8: the tier follows the remaining gap.
2. `RUN/CANONICAL_SITUATIONS.md`: canonical and pattern situations.
3. `RUN/AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`: what is governing, context
   and evidence.
4. `RUN/RUN_BASIS.md`: frozen state and gate evidence.
5. For each deliverable:
   - its rows in `RUN/CLAIM_KEYS_V2.csv`. Required keys are your coverage
     denominator. `.rNN` keys are optional, all-or-none per block (C1);
   - its rows in `RUN/CANONICAL_ASSIGNMENTS.csv`, which you inherit (C1);
   - its rows in `RUN/EVIDENCE_MAP.csv` (A3, CP-09);
   - its folder in `{FREEZE}`.
6. `RUN/DELIVERABLE_INVENTORY.csv`, `RUN/VERIFICATION_INDEX.csv` and
   `RUN/VALIDATION_AND_PROVENANCE_INDEX.csv`, as finding aids.

**Named calibration repairs.** Where one of your deliverables was an R0
pilot, read under `RUN/R0_CALIBRATION/<DEL>/` its forward ledger and notes,
and the forward-ledger repairs named for it in
`RUN/R0_CALIBRATION/R0_REVIEW.md` §4. Encode afresh under the bound
conventions, carrying those repairs. Do not copy rows. Read its R0 reverse
file and any reverse repairs **only after all your forward ledgers are
sealed** (B1).

## Method

Work **one deliverable at a time**. Keep `_WORKER_<first DEL>_NOTES.md` as a
short running record of how you judged situations that recur across your
deliverables, so the same situation gets the same treatment.

**Pass 1: forward ledger, per deliverable.**

1. Write `<DEL>_forward.csv` to the Part D schema. It needs exactly one row
   per required key, any `.rNN` rows for blocks you split, and any `.sNN`
   sub-claims. It ends with an `#END` sentinel that carries the row count in
   `Notes`.
2. Run the validator with the Part F checks until it passes, from `{REPO}`:
   `PYTHONDONTWRITEBYTECODE=1 python3 RUN/tools/validate_ledger_v2.py --run-dir RUN --repo-root . --deliverable <DEL> --forward <path> --notes-gap`
   For each F4 finding, either re-dispose the row (F1) or, only when the
   wording truly is not about an unmet element of this claim, end its Notes
   with `GAP_WORDING_CHECKED: <why>` (the last clause, at least 25
   characters). The verifier checks every such row.
3. **Seal.** Compute the ledger's SHA-256 and write `<DEL>_SEAL.txt` with one
   line: `SEALED <DEL> <sha256> <UTC timestamp>`. Never edit the forward file
   after sealing.

**Consistency check.** When all your forward ledgers are sealed, run:
`python3 RUN/tools/validate_ledger_v2.py --run-dir RUN --repo-root . --batch <your forward files>`

Do not edit sealed files. Record any flagged pair in the notes. Say either
that the difference is justified, and why, or that it is an error you would
correct. The verifier and a fresh worker handle corrections.

**Pass 2: reverse answers**, after all seals exist.

1. Your parent's launch message names your package routing file:
   `RUN/ROUTING/{PKG}_capabilities.csv`.
2. For **each** of your deliverables, write `<DEL>_reverse.csv`. It holds one
   row per capability in the routing file, using the B2 answers, and ends
   with the `#END` sentinel. Use the routing file's `CapabilityID` values
   as given. Judge every row on its evidence alone. Where a capability's
   `EntryPoints` hit a path your own forward ledger cites, a `NOT_MINE`
   reason must address that capability specifically (F5).
3. Validate each with `--forward <file> --reverse <file> --inventory RUN/ROUTING/{PKG}_capabilities.csv --notes-gap`.

**Notes, per deliverable.** Write `<DEL>_notes.md`. It covers:

- path aliases;
- judgment calls;
- canonical departures;
- convention friction;
- the smallest check for every `UNKNOWN` row;
- whether the reverse pass changed your view of anything sealed (say it here;
  do not edit);
- the batch-consistency outcome;
- the statement that selectability is `NOT_APPLICABLE` (C9);
- the claim-fence line.

## Return

Delete your `_scratch_*` files. Return one line per deliverable:

```
DONE <DEL> forward=<sha256> reverse=<sha256> notes=<sha256> validator=PASS
```

Then add one line: `BATCH <PASS|FAIL> <n findings>`.

After that, at most eight lines covering:

- disposition counts across your deliverables;
- the top cause tags;
- anything the verifier or owner must see: protected-check, invariant,
  ISSUED or authority-conflict rows, and possible defects.
