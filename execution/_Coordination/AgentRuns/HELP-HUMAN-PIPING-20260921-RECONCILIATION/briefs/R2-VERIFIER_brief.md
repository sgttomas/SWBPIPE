# Brief — R2 package verifier (fresh, evidence-only)

Role: TASK (Type 2), fresh context, **evidence-only**. Your parent is
HELP_HUMAN Agent 0 of run `HELP-HUMAN-PIPING-20260921-RECONCILIATION` (D-73 /
DEC-110; R0 ruling). You never edit a ledger, never repair anything, and never
delegate. The manager and its workers do not direct you.

The launch message supplies these values:

- `{PKG}`: the package to verify.
- `{WAVE}`: the wave.
- `{SAMPLING}`: `DOUBLE` for the first wave, otherwise `STANDARD`.
- `{FREEZE}`: the read-only evidence checkout.
- `{REPO}`: the repository checkout.

In this brief, `RUN` means
`{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.

## Boundary

- Write only `RUN/WAVES/{WAVE}/{PKG}/{PKG}_VERIFICATION.md`. Scratch files go
  in `RUN/WAVES/{WAVE}/{PKG}/_verify_scratch_*`; delete them before you return.
- Read code and documents from `{FREEZE}`. Read the ledgers under
  `RUN/WAVES/{WAVE}/{PKG}/`.
- No builds, test runs or git writes. The only network access allowed is
  read-only `gh`.
- Do not read the July 2026 run's ledgers.
- The fences are the same as the workers'.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Rules

Judge every row against `RUN/CONVENTIONS.md`, `RUN/CANONICAL_SITUATIONS.md`
and `RUN/AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`. Judge its evidence against
`{FREEZE}`.

## Sampling

Sampling is set by the R0 ruling (`R0_REVIEW.md` §7 as adopted). `DOUBLE`
doubles the rates marked *.

| Rows | Rate |
|---|---|
| `INVARIANT`, `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`, `UNKNOWN`, `LIFECYCLE_REASSESSMENT_REQUIRED`; every row of an ISSUED deliverable; every row carrying `PROTECTED_CHECK` or `FROZEN_CONTRACT` | 100% |
| Every row whose unit has `SharedTextCount > 1` in `CLAIM_KEYS_V2.csv` (owner-confirmed consistency enforcement) | 100% |
| Other non-aligned rows | 25%* |
| `ALIGNED` normative rows (`REQUIREMENT`, `ACCEPTANCE`, `EXCLUSION`), weighted toward `LOW`/`MEDIUM` confidence, `NONE_FOUND` verification and `NONE` verification class | 20%* |
| Structural and inherited canonical rows | 10%* |

- Conformance to `CanonicalSituation` and same-body rows is checked
  mechanically on 100%, using `validate_ledger_v2.py --batch`.
- The reverse answers are checked on 100% of `CLAIMED_BY`, `PARTIAL`,
  `UNKEYED` and `CONSTRAINS` rows, plus 10% of `NOT_MINE`. Worker routing
  files use routing-local IDs (`RC-<nn>-<NNNN>`); resolve them to inventory
  IDs, areas and routing (`AREA` or `SAMPLE`) through
  `RUN/ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`, which workers do not see. Check every sampled row
  whose evidence names a path the package's deliverables declare, and report
  the sampled rows' answer distribution against the area rows'. About half
  the sampled rows are recognisable from their `EntryPoints` paths (inherent:
  workers need the paths); weigh the comparison with that in mind.
- Select samples deterministically: sort candidate keys by SHA-256 of the key
  and take the lowest fraction. Report the selection.

## What to judge for each sampled row

- Does the cited implementation do what the row says?
- Do the cited tests exercise it?
- Are the disposition, cause, tier, baseline class and layers right under the
  conventions?
- Is `ALIGNED` real? Rule C6(a) forbids aligned-by-construction.
- Is any context standing in as authority?

Classify each disagreement as:

- **firm**: you would re-dispose the row;
- **weak**: defensible only under a reading the conventions should settle;
- **field**: a correct disposition with a wrong field.

## Output — `{PKG}_VERIFICATION.md`

1. First line: `VERDICT: ACCEPT`, `ACCEPT WITH CONTESTED ROWS` or
   `RERUN <DEL list>`.

   A deliverable needs a rerun when its firm error rate on sampled rows
   exceeds 10%, or when it has a firm error on a 100%-sampled class that
   changes the tier or the owner routing.
2. A table per deliverable, with these columns:
   - rows;
   - sampled per class;
   - firm, weak and field counts;
   - **firm false-alignment rate** among sampled `ALIGNED` normative rows.
3. The package-level firm false-alignment rate. This is the scale-out gate
   input; the gate is 5% or less.
4. Every disagreement, giving: key, what the row says, what you found (with
   paths), and the right values.
5. The batch-consistency result, and any shared-situation conflict.
6. Reverse-pass findings. Report:
   - capabilities claimed by more than one of the package's deliverables;
   - suspected missed claims;
   - anchored answers.
7. Anything the owner must see, stated plainly.

## Return

```
VERIFIED {PKG} verdict=<…> firm_false_alignment=<pct> sha256=<sha256>
```

Then at most eight lines of summary.
