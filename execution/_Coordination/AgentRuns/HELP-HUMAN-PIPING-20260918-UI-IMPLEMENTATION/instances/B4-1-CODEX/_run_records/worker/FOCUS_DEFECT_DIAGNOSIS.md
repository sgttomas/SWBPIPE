# First-click focus-entry regression

Observed under one-worker Vitest/jsdom; exact failed run focused-test-06.txt, EngineeringTable interaction ownership / publishes keyboard focus entry without turning the first pointer click into editing. Expected: keyboard focus entry selects its row; first pointer click on another cell selects without editing. Actual: n:1 X input exists after first click.

Deterministic reproduction: focus cell A; pointerDown B; focus B (causes blur A then focus B); click B. High-confidence causal chain from handlers: pointerDown set global boolean true; blur A reset it false; focus B therefore publishes focused B; click B observes B already focused and starts edit. Downstream Apply semantics are unaffected. Native event ordering remains to be checked in browser/WebKit.

Bounded repair: tie pending pointer focus to CellAddress; blur A clears only an A marker, preserving B marker until click B. This preserves keyboard focus publication and prevents first-click editing. Existing worker authority covers core repair. Regression is the unchanged failing test plus source browser first/second-click journey.

Skill consulted at assigned-root .agents/skills/software-defect-diagnosis/SKILL.md; SHA256 f281558734732ab0a97cf1407ba80a53c25e8c2b9dd96069bec589fd521a7a23
