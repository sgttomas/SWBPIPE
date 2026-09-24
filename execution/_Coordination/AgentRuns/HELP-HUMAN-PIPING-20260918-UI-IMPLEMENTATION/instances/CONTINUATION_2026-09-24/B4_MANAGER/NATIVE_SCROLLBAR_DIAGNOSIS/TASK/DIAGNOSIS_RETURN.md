# Native scrollbar diagnosis return

Status: bounded static diagnosis complete; no repair, test, build, process control, or live UI operation performed.

## Basis and attribution

Candidate HEAD verified as `5cfd2257cafca0fb141fc7dd5085c627d8caae9e`. Executing child `/root/b4_manager/native_scrollbar_diagnosis`, parent `/root/b4_manager`; mechanism delegated-harness-native descendant, TASK Type 2, no descendants. Parent supplied ROOT native observations: PID 9925, SystemDark/Comfortable, Model/Grid/Inspector docked, minimum native window, requested 180 drawer; full Node row and fixed chrome visible; wheel changes N100 to N130. Those are ROOT observations, not my reproduction. Read full Root, TASK and Piping instructions and applied project software-defect-diagnosis skill. No other role instructions consulted. Write boundary is this TASK directory only; host permissions do not expand assignment.

## Confirmed source mechanism

`apps/desktop/src/features/workspace/table/EngineeringTable.tsx:291–307` constructs one fractional grid template and applies it separately to header and each body row. The header is a direct grid child; the rows are inside VirtualList inside the body slot. Identical templates do not imply identical track widths when their containing widths differ.

`apps/desktop/src/features/workspace/VirtualList.tsx:111–144` gives only the list host vertical overflow auto when row content exceeds its height. Its inner content and absolute row wrappers use that host's content width. `apps/desktop/src/styles.css:3991–3994,4036–4042` allows outer horizontal scrolling, sets minimum widths on both list and rows, and does not compensate header for the list's vertical gutter. Classic vertical scrollbar therefore subtracts width only from the body row containing block. Header fractional tracks resolve against a wider box. The cumulative rightward header-boundary drift in viewed ROOT screenshot 10 matches this mechanism. Confidence high. Exact scrollbar width (~17 CSS px) remains ROOT's estimate; this task did not measure live DOM dimensions or infer precise CSS coordinates from the resized image.

Contributing condition: classic native scrollbar allocation versus overlay-scrollbar browser checks. Sorting, column order, virtual-window indexing, or model data mutation are not needed to produce the defect. Width divergence occurs before per-row interaction. All EngineeringTable consumers sharing these styles can be affected when vertically scrolling; the 5-row Nodes example is nonvirtual (default VirtualList threshold 100), so changing virtualization is unjustified.

## Horizontal evidence and limits

The launch brief described horizontal consequences as unobserved. On reading the already-existing ROOT screenshot `13-material-minimum-native.jpg`, I observed a native horizontal bar under Materials and visibly insufficient body height / clipped row content. This is an image observation, not a new native exercise or a measured clientHeight claim. The screenshot also shows long Material headings overflowing the constrained header region. It does not establish every wide family's behavior, exact pixel loss, or whether every cell remains reachable.

Source predicts the height pressure: outer grid owns `overflow-x:auto` (styles3991); bounded grid hides vertical overflow and flexes its body slot (4037–4040). A classic outer horizontal bar consumes grid client height. The measured body slot height is passed faithfully through `useTableBodyHeight` (EngineeringTable345–363) to VirtualList (307). At an allocation already budgeted to one 36px Comfortable row, a positive horizontal-bar footprint leaves less than a full row. This is different from the confirmed vertical-gutter track mismatch. The horizontal risk should not be reported as a new VirtualList arithmetic failure. An inner horizontal bar is additionally possible when row min-width exceeds a gutter-reduced rowgroup clientWidth; verify rather than assume its occurrence.

## Minimal repair scope and validation

Keep table measurement/style ownership inside EngineeringTable and table CSS. Establish one shared content-track width for header and body, accounting for the actual list scrollbar gutter (zero under overlay settings). Preserve the column minimum sum, and give the body container room for its gutter at minimum width so the fix does not introduce an inner horizontal scrollbar. A narrow measured gutter/header-width adjustment is preferable to rewriting VirtualList or hardcoding 17px. Observe relevant resize, family, row-count/filter, density and active/hidden transitions; avoid measurement feedback loops. Merely adding stable gutter to the body leaves the same mismatch. Any CSS-only matched-gutter option must prove matching used widths under native behavior and at the minimum-width threshold.

Treat horizontal footprint as a separate bounded table-layout repair. Under the preserved 127px host and existing row/header/footer geometry, a classic extra horizontal bar cannot consume row height while preserving one complete Comfortable row. The implementation must arrange a zero-additional-height horizontal affordance or otherwise place the horizontal affordance within existing chrome while retaining reachable wide columns. Do not silently enlarge the protected budget, shrink row geometry, clip wide columns, or remove horizontal access. The integration owner should choose the smallest accessible option supported by the current host; no authority to alter the budget is created here.

Meaningful validation on the actual repaired candidate: native classic-scrollbar Nodes at minimum width/height, then Materials or another genuinely wide family at same dimensions; compare all header/body boundary x-coordinates within device-rounding tolerance, assert full 36px Comfortable row and 30px Compact row fit, exercise both axes through last row/last column, verify fixed toolbar/footer, and verify editor anchor remains aligned after horizontal scrolling. Include empty/fitting versus overflowing row counts, width crossing table minimum, resize and hidden/reveal, and an over-threshold virtual list. Browser geometry checks must deliberately use a nonzero gutter and horizontal footprint; overlay-only passes cannot close native behavior. Use focused EngineeringTable geometry regression coverage and actual native screenshot/input evidence; existing interaction checks protect edit/focus behavior. No checks executed by this task.

## Evidence hashes

Paths below are repository-relative. SHA-256 covers bytes read; JPEG originals were neither renamed nor modified.

- `AGENTS.md` — `c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd`
- `agents/AGENT_TASK.md` — `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7`
- `projects/chirality-piping/AGENTS.md` — `d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792`
- `.agents/skills/software-defect-diagnosis/SKILL.md` — `f281558734732ab0a97cf1407ba80a53c25e8c2b9dd96069bec589fd521a7a23`
- `projects/chirality-piping/apps/desktop/src/features/workspace/table/EngineeringTable.tsx` — `2c4bedcacb145287c132069ab50bb05d74ad74518ea45a35571b5d825a590326`
- `projects/chirality-piping/apps/desktop/src/features/workspace/VirtualList.tsx` — `0ef58bdc7c98188be7b1a828125ac056de25bb7646ca38bff76f3ad11d49c92c`
- `projects/chirality-piping/apps/desktop/src/styles.css` — `4096d44dad9b84a0a7edff534290cbd4001a81cc477b8783a0a5a16d733df3b0`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/_run_records/ROOT_NATIVE/10-model-comfortable-minimum.jpg` — `f8e9bb0ccb54716427a03f6484a09f9282ff96128c5a331eea87e8fba2c0852a`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/_run_records/ROOT_NATIVE/13-material-minimum-native.jpg` — `5eda5c076dec807444fca95118fd3801cfb35709ff58e6c0dde2d8fc6c595f20`
