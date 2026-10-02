# I26 T4 measurements — provisional return

Completed the original 24-row T4 schedule once: 12 W1 and 12 sparse runs,
120 normal repeats, 30 first-pass prefix calls, zero numeric deferrals.
All 24 process classifications are `ok`. Numerical classes remain distinct:

| Model (10,000 members) | W1, both passes | Sparse, both passes | Published first-pass rows | R1 |
|---|---|---|---:|---|
| CHAIN-AX | Selected 128; verified 256 | NumericallyUnresolved | 250013 | 103 pass |
| CHAIN-ROT | Selected 128; verified 256 | NumericallyUnresolved | 250013 | 103 pass |
| TREE-AX | Unresolved / Ceiling | NumericallyUnresolved | Unavailable | Unpublished; not evaluated |
| TREE-ROT | Unresolved / Ceiling | NumericallyUnresolved | Unavailable | Unpublished; not evaluated |
| CONT-AX | Selected 128; verified 256 | Sensitive | 265013 | 215 pass |
| CONT-ROT | Selected 128; verified 256 | Sensitive | 265013 | 215 pass |

The current raw counters are **W1: 8 Selected, 4 Unresolved/Ceiling;
sparse: 8 NumericallyUnresolved, 4 Sensitive**. Every run completed its five
requested repeats. Selected first passes have three budget prefixes apiece;
TREE first passes have nine, reaching the verify_1024 endpoint. Every prefix
retained its exact source-derived endpoint and existing parity predicate.

ROOT's explicit CONT-only continuation was used. Rows through TREE 262 had
completed validly before the original cutoff; the same wrapper then finished
only the remaining original CONT rows. This was new bounded authority, not
compliance with the old cutoff or an automatic extension. Original and revised
deadlines, receipt, launch and observations are retained separately in
`_run_records/SESSION.json`, `CONTINUATION_AUTHORITY.json` and `TIMELINE.json`.
The final normal log's last write was 15:33:08.975368 UTC; completion was first
observed at 15:33:50.997419 UTC with outer exit 0. A file write timestamp is not
claimed as an exact process-exit observation. Baseline inspection occurred
while the first normal product was already active; subsequent snapshots were
real observations, not an exhaustive pre-row validation gate.

All 37 consumed plus 20 invariant fields passed for 24 normal inputs and
12 fresh bindings (2,052 comparisons). All 264 W1 context-estimate comparisons,
caller bindings, source/work/storage/self-parity and chronological admissions
passed. Each admission used exactly eight eligible prior T2/T3 rows of the
same family/mode and the current no-op; same-size T4 rows did not calibrate.
No cap, estimate, predicate, schedule or source was changed.

All 636 matched source/solve/prefix requested/moving heap comparisons passed;
minimum slack was 469,832 bytes. Largest matched requested and moving peaks
were both 3,030,889,709 bytes. Global/pre-reset/count/parser/serialization
measurements and sparse heaps do not acquire an H bound.

TREE actually exercised 512 and 1024 bits in all four normal runs. First-pass
first-repeat work, in limb-multiply equivalents (LME), is reproduced below; the full stage records, both passes
and all repeated closure checks remain in the packet.

| TREE orientation | Precision | Own work (LME) | Shared work (LME) | Limbs per entry |
|---|---:|---:|---:|---:|
| AX | 512 | 5,102,362,329 | 13,490,278,555 | 8 |
| AX | 1024 | 14,904,124,902 | 23,532,837,836 | 16 |
| ROT | 512 | 6,709,181,547 | 15,907,310,991 | 8 |
| ROT | 1024 | 16,765,091,835 | 24,745,080,191 | 16 |

Those attempts report 1,080,036 pattern and 569,964 profile entries.
Their memory coverage includes these per-attempt storage facts and matched
whole-call heap observations; per-internal-precision heap peaks are unavailable.
See `TREE_HIGH_PRECISION.json` and `PRECISION_WORK_AND_PREFIXES.json`.

**Observed RSS exceeded its admission projection in four W1 TREE runs and
four sparse runs.** TREE first-pass AX observed 3,891,986,432 bytes versus
1,314,759,989 projected; ROT observed 3,723,083,776 versus 1,317,384,262.
These projection misses are evidence for later ROOT policy disposition.
They are not matched H-window failures; no RSS cap was reached. Per-run
RSS, footprint, wall time, host load and projection differences are retained
without recalibration in `FULL_CHECKS.json` and `ADMISSION_AND_RSS.json`.

Unchanged R1 ran exactly once over the four complete byte-verified new
Selected dumps: **636 comparisons, zero failures or missing values**, exit 0.
The four canonical dumps retain 1,030,052 published rows in total, plus exact
input-view copies. All six first-pass outcomes are accounted. TREE has no
published dump and neither TREE case receives a value-comparison pass.
R1's original exact Fraction predicate and class scales were unchanged.

Available KF3 class/reason/precision/row facts and twelve common primitive
count fields match the adopted counterparts. Historical VK SHA and H FNV/
canonical encodings retain separate provenance; unlike hashes are not equated.
Historical TREE's separate `[Restrained]` geometry suffix is unreported by H.
Full historical published-value equality remains unestablished. This does not
establish complete W1-versus-sparse value equality or an engineering guarantee.

The normal wrapper and R1 exited 0. Runtime was released before sealing.
Actual final process snapshots found none of the 37 known product PIDs,
observed groups or exact owned commands; successful default `survivors=[]`
arrays receive no group-scan credit. No kill/watchdog cleanup path was needed.

All 299 source files/modes, ordinary binary, seed, 910 prior files and
31 references remained unchanged; the previous 108-row journal is an exact
prefix of the resulting 132 rows. No maintained/Git/index/tool changes,
builds, retry, other model or automatic continuation occurred.
`RAW_INVENTORY.json` identifies only the new raw delta and growing journal;
older packets were not copied.

Finite checks pass provisionally. Independent review and ROOT acceptance,
complete K6c/E_max, W1-limit policy and F2a remain open.
