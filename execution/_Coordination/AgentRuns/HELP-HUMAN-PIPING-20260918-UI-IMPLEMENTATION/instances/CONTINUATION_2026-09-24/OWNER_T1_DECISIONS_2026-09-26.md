# Owner decisions on the T1 plan (2026-09-26)

HELP_HUMAN (ROOT) asked three questions from `LOAD_STATE_IMPLEMENTATION/T1_PLAN.md` §4 on branch `codex/piping-load-states-20260925`. The owner answered:

1. **D1 packager edit.** Question: may the stress-neutral export packager (`core/handoff/stress_neutral/package_v0_3.py`) be edited to accept `load-reference-source-1`? Answer: "Approve the edit (Recommended)".
2. **D2 authoring surface.** Question: plain input fields, or headless only? Answer: "Plain input fields (Recommended)". That means typed operations, plain fields in the existing Load Case Manager and Materials panels, and a read-only resolved-state block, with no layout or visual work.
3. **D3 pre-0.4 models and the legacy imposed displacement.** Answer: "We don't need any backwards compatibility if that's what this is about."

## ROOT application

- **D1.** T1 activates `load-reference-source-1` in its PR once the joined readers pass independent review. The packager edit is approved by the owner. If the host still refuses the write, ROOT takes that to the owner and does not route around it.
- **D2.** T1 includes the minimum native input and the read-only resolved-state block, and nothing more. The native witnesses run on the owner's Mac.
- **D3.**
  - T1 builds no upgrade operation and no automatic mapping of the legacy `imposed_displacement`. The 0.4.0 load-state format is for new models.
  - Historical records and bytes stay preserved, as governance requires.
  - The owner's answer removes the compatibility requirement. It does not permit current behaviour to be silently wrong. Fresh desktop models are still authored at schema 0.2.0, so what today's default route does with an imposed-displacement load is current behaviour, not compatibility. If it silently drops the load, that is a default-route defect for T0R.
- **D4 (ROOT, write scope).** T1 may write in the paths of T1_PLAN §5. Work that shares write sets with T0R lands after T0R: WP2 (desktop types and persistence), WP3's native fields and WP4 (the headless adapter). Work that doesn't can proceed now:
  - WP1: joined readers and the packager;
  - WP3's typed operations in `operation_applier`;
  - WP5: the harness adapter;
  - WP6: the VP-STATIC cases.
  Mechanical identity-enumeration conflicts are resolved at each main merge.
