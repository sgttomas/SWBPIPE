# N2C DEL-07-03 dependency patch — sealed launch brief

## Identity and objective

- Parent: PROJECT_SETUP R23 under HELP_HUMAN.
- Form: ephemeral Agent 2; no delegation.
- Objective: apply only the frozen DEL-07-03 dependency-currency dispositions.
- Protocol: `../../CHILD_PATCH_PROTOCOL.md` is mandatory.

## Exact targets and baseline

- CSV: `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/Dependencies.csv`
  at SHA-256 `a5ebd396227284d16e437e05149181a6f24be64c83e491487c2d38ce3a7ef0a7`.
- Index: same folder, `_DEPENDENCIES.md`, at SHA-256
  `7af5215fcd64bd25ae23e545914e0b5b20d4bff5a1c6cb375bfebfa15e7d3f82`.
- These are the only authorized write targets.

## Frozen evidence

- Consumer integration: `MEMORY.md` lines 120–152, 232–280, and 350–432 at
  SHA-256 `a830f675b05d70a01380cab3dd17c0b8e6bb9d3dd40d9a99186d5f49f579d2a6`;
  `_STATUS.md` lines 6–24 at SHA-256
  `9791393d283b1deee625754ebb4e2ef51a6d090ec6b9fcfc367d1d1982694771`.
- Target maturity PASS tuples:
  - E0489 / DEL-03-01 / SHA-256 `2aa9dce213cb3a42c8a6d2300f484e8efddd16bdc21ff9cb1c5826060b453145` / state L3 / ready L12.
  - E0490 / DEL-03-02 / SHA-256 `de08d24130fcc1a1ddb73cc49ea3af0ac735897f0cc7f4a2089426cbf1e21c3d` / state L3 / ready L12.
  - E0492 / DEL-06-01 / SHA-256 `8bfc5b3bc1d2f7395dd5e61c5c52426218399574b6ca7fc7a8d09a4943bfb507` / state L3 / ready L12.
  - E0493 / DEL-06-04 / SHA-256 `bf0b48a4e1073f1e58a52c0bd00eec0525ee61b996b887c2f797b950d1d50b52` / state L3 / ready L17.
  - E0494 / DEL-12-01 / SHA-256 `5f1dc9d0e52dc4b7b1f21cb3c482361525717796352f7810f5467964c2abf186` / state L12 / ready L28.

## Exact dispositions

- `CLOSE`: `DAG-002-E0489`, `DAG-002-E0490`, `DAG-002-E0492`,
  `DAG-002-E0493`, `DAG-002-E0494`.
- `UNCHANGED`: `DAG-002-E0491`.
- Close-row appended note, exact text:
  `; FACT: 2026-08-02 dependency-currency PASS under R23; target maturity PASS is frozen in TARGET_MATURITY_EVIDENCE.csv; consumer integration PASS is frozen in N2C_DEL0703_PATCH/LAUNCH_BRIEF.md.`
- Index satisfaction counts become: NOT_APPLICABLE=3; PENDING=1; SATISFIED=13.
- Append run history exact item:
  `- 2026-08-02: R23 dependency-currency patch; five execution rows closed by independent target-maturity plus consumer-integration evidence; DAG-002-E0491 held unchanged.`
- Append under `## Downstream Handoff Notes` exact item:
  `- R23 is a deliverable-local satisfaction refresh only; DAG-008 remains graph authority and this patch does not update execution/_DAG/_LATEST.md.`

## Return contract

Only file read and `apply_patch`; all protocol exclusions and F-PIP-1..5 apply.
Return: verdict; written paths; changed IDs; unchanged ID; post-write counts;
non-target-field and raw-row preservation confirmation; blocker. Do not write the
return file.

