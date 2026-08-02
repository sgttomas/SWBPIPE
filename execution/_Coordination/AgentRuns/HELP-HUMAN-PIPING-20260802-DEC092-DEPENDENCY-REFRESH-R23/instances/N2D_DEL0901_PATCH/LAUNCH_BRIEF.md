# N2D DEL-09-01 dependency patch — sealed launch brief

## Identity and objective

- Parent: PROJECT_SETUP R23 under HELP_HUMAN.
- Form: ephemeral Agent 2; no delegation.
- Objective: apply only the frozen DEL-09-01 dependency-currency dispositions.
- Protocol: `../../CHILD_PATCH_PROTOCOL.md` is mandatory.

## Exact targets and baseline

- CSV: `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark suite/Dependencies.csv`
  at SHA-256 `48a1bf3717b0675d82fc047003ef02b99f9dc3fdf8beed6f0242d4e5b5ee5a97`.
- Index: same folder, `_DEPENDENCIES.md`, at SHA-256
  `38992b8d46d9a22a5dc0247b72770858d773ff9c7ada8d37844280619b2448db`.
- These are the only authorized write targets.

## Frozen evidence

- Consumer integration: `_STATUS.md` lines 6–29 at SHA-256
  `7f32084cd63efd5829e40b6aa4dec5c84b00f53dc777b28dc695779be1f97941`;
  `MEMORY.md` lines 431–463, 670–703, and 740–755 at SHA-256
  `d556cf5d1951598821a299b0220ade0750cb162734cac29301bab5920395ca31`.
- Target maturity PASS tuples:
  - E0534 / DEL-04-03 / SHA-256 `008d635e8825468d2c2092dfe45f7433044c8d71059359b47572c2219b0d40ee` / state L3 / ready L11.
  - E0535 / DEL-05-01 / SHA-256 `f5d705f7fcd07e8d61af60492f84431af6ae94ce495966c47a9abb5f87f07de7` / state L3 / ready L10.
  - E0536 / DEL-01-02 / SHA-256 `f74b633c9835fabf5333d27c9c69222592f2a296bf376385661e244d92891875` / state L3 / ready L12.
  - E002 / DEL-04-06 / SHA-256 `e0fd91bb6f62b5b97e8ea1d3c3dee84bcebbc495c1d9e5f3024ba4c4a1ffd520` / state L3 / ready L11.

## Exact dispositions

- `CLOSE`: `DAG-002-E0534`, `DAG-002-E0535`, `DAG-002-E0536`,
  `TP-DAG-004-DEL-09-01-E002`.
- `UNCHANGED`: `DAG-002-E0532`, `DAG-002-E0533`.
- `HOLD`: `TP-DAG-004-DEL-09-01-E001`; its raw row must remain byte-identical.
- Close-row appended note, exact text:
  `; FACT: 2026-08-02 dependency-currency PASS under R23; target maturity PASS is frozen in TARGET_MATURITY_EVIDENCE.csv; consumer integration PASS is frozen in N2D_DEL0901_PATCH/LAUNCH_BRIEF.md.`
- Index closure counts become: NOT_APPLICABLE=3; SATISFIED=8; TBD=3.
- Append run history exact item:
  `- 2026-08-02: R23 dependency-currency patch; four execution rows closed by independent target-maturity plus consumer-integration evidence; E0532, E0533, and TP-DAG-004-DEL-09-01-E001 held unchanged.`
- Append under `## Downstream Handoff Notes` exact item:
  `- R23 is a deliverable-local satisfaction refresh only; DAG-008 remains graph authority and this patch does not update execution/_DAG/_LATEST.md.`

## Return contract

Only file read and `apply_patch`; all protocol exclusions and F-PIP-1..5 apply.
Return: verdict; written paths; changed, unchanged, and hold IDs; post-write
counts; non-target-field and raw-row preservation confirmation; blocker. Do not
write the return file.

