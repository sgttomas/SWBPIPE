# T3 current state — handoff, 2026-10-05

**Handed off at the owner's request. Nothing is running. The memory guard stays active.**

[Handoff](../HANDOFF_2026-10-05_TO_NEXT_ROOT.md) · [Terms](GLOSSARY.md) · [Decision history](../ROOT_RULINGS_V1.md) · [Previous handoff](../HANDOFF_2026-10-03_TO_NEXT_ROOT.md)

## Closed or accepted

- **A1:** PR1070. **K6c:** PR1071, with its recorded limits.
- **The F2a D1 milestone: PR #1082,** merge `0b00b8e8b6`, at F′ `5488136a19`.
  - RF-SKEW-T-CANT-OFF-122-r1e-04 publishes its M03-INTEGRITY-MP-v2 successor through the Direct entry in both modes, in the registered dev/test build only. It matches its independent reference, and the refusal and coexistence controls are preserved.
  - The Python, Rust and TS readers treat it as eligible with its invocation, and the carriers transport it.
  - No product caller exists.
  - The records are `../IMPLEMENTATION/F2A_D1/` and `../IMPLEMENTATION/F2A_D1_MERGE/` (with ERRATA E-1).
- **U8,** the F2a-breadth roadmap and S-I1's readiness are planned (`I61/u8_plan_01/PLAN.md`) and ruled. The owner pulled T6's successor-output slice forward.

## Open

- **G10:** the native witness, on the owner's Mac.
- **U8,** whose briefs are ready.
- **The rest of F2a:**
  - **B0–B6 (numerical breadth):**
    - B0, contract and identities;
    - B1, multi-case, with W-C2 and D38;
    - B2, combinations, preparation-only and mixed invocations;
    - B3, the exact routes;
    - B4, cap growth, if ruled;
    - B6, the reader items;
  - **B7,** the release identity;
  - **B8,** activation with native Current.
- **S-I1,** in parallel.
- **The T6 successor-output slice,** in parallel.
- **Then** S-I2, F2b per family, and F3.

## Next

Follow the handoff's "The next work, in order": verify the state, absorb main if needed, then dispatch U8 (I68 Part 1), with S-I1 (I73) and the T6 slice plan (I74) alongside.

## Branches

- **NUM** (`codex/piping-numerical-integrity-20260926`) is the integration branch. Its maintained source equals main's.
- **Product PRs** are cut compactly from main. **Records** reach main only through records-only PRs.
- **All T3 component branches** are pushed, and contained in main or NUM.
