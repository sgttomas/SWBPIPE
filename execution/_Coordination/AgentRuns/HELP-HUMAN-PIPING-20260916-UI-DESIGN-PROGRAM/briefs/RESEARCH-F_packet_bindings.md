# Sealed brief — RESEARCH-F: what binds the nine decision-packet items

Sealed by ROOT (HELP_HUMAN) on 2026-09-18 before launch. Role: TASK, bounded research. Model requested: Claude Opus 5 (the result is an objective inventory of governed text and its anchors). Mechanism: Claude Code `Agent` tool, general-purpose type, background. Type 2 does not delegate; this child works alone.

Path placeholders: `{REPO_ROOT}` is the ROOT worktree; `{RUN}` is `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`; `{PROJ}` is `{REPO_ROOT}/projects/chirality-piping`.

## Purpose

ROOT will prepare one owner decision packet with nine items (listed below). For each item, inventory exactly what binds it today: the governed text verbatim, the registry entry, every lint anchor and validator that enforces it, every test that would fail if it changed, every place the text or its short variant appears in the product and the documents, and what kind of owner act the governance says a change requires. The packet is only as good as this inventory; nothing here is a proposal.

## The nine items

1. The maturity line "Technical preview — not a released product." (registry `BS-MATURITY`; M-01).
2. The acceptance sentence (registry `BS-ACCEPT`; M-02) and its placements.
3. The product name "OpenPipeStress" in registered sentences, the required report notice (M-03), the library notice (M-05), C-17, lint anchors, package and document titles; the rename to SWB Piping Designer / SWBPIPE.
4. Short labels for the six automatic statuses and the two evidence labels (M-08, M-09; TYPES §4; PROFESSIONAL_BOUNDARY §6), and whether "Human review required" must be shown on every surface showing solve or rule-check state.
5. Vendor hanger tables as a library class under M-05 and the hanger-selection vocabulary item (C §3 item 23).
6. The historical-run wording (M-13) as the existing component states it.
7. The Checked mark: a human-authored, content-bound tag on a row (C-22 state-labelling affordances; C-06 and C-65 acceptance-record prohibitions and binding rules).
8. Agent feedback on the Review page: the permitted agent-output classes (C-77; PROFESSIONAL_BOUNDARY §3 agent-output row; PRD §11.8 FR-AGENT-001 to 005), and whether the Review page is one surface class for the acceptance sentence.
9. The export's name before compatibility evidence exists (C-05 permitted claims; M-04 export-metadata notice) and any IP or interoperability constraint recorded in governance.

## Accepted basis

- `{RUN}/instances/RESEARCH/C_ui_constraints.md` — the extraction to start from; verify every citation at its source.
- Governance sources under `{PROJ}/docs/`: `claims_registry.md`, `PRD.md` (§5.3, §11.8, §16.3, §19.2, §19.3, §21), `PROFESSIONAL_BOUNDARY.md` (§3 to §10), `CONTRACT.md` §1, `SPEC.md` (§4.3, §12), `TYPES.md` §4, `DIRECTIVE.md` §3, `report_notice_template.md`; the decision register `{PROJ}/execution/_Coordination/_DECISIONS/_REGISTER.md` and the rulings it names for DEC-081, DEC-051, DEC-037, DEC-094, SCA-009 where they touch an item.
- Enforcement: every validator, lint and test under `{PROJ}` and `{REPO_ROOT}/tools/validation/` that anchors a registered text or a status token (start from the lint anchor names cited in C: `MISSING_PRD_NOTICE`, `MISSING_RENDERER_NOTICE`, `AD_HOC_CLAIMS_LITANY`, `BS-MATURITY`, `BS-ACCEPT`, and find the rest by searching for the registered sentences and the status tokens).
- Occurrences: the product source under `{PROJ}/apps/`, `{PROJ}/core/`, `{PROJ}/schemas/`, `{PROJ}/fixtures/`, and the documents, for each registered sentence, its listed short variants, the product name, and the status tokens.
- Context: `{RUN}/instances/ROOT/DESIGN_BRIEF_V1.md` §6, `{RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` §5, §6 and §11, `{RUN}/instances/CONCEPTS/CONCEPT_DIRECTIONS_V1.md` §6, `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` §7 and §8, `{RUN}/instances/MOCKS/MOCKS_V1.md` §4, so that you know why each item is asked; and the frames under `{RUN}/instances/MOCKS/frames/` for where each text would appear in the redesign.

## What to deliver

One file, `{RUN}/instances/RESEARCH/F_packet_bindings.md`, with one section per item, each containing:

1. **Governed text, verbatim**, with its source file and line, and the registry entry (id, canonical text, listed short variants, bound surfaces) where one exists.
2. **What enforces it**: each validator, lint rule and test, with file and line, and what exactly it checks (a sentence present in a file, a token in an enum, a footer string, a count).
3. **Where it appears today**: every occurrence in product source, templates, fixtures and documents, with file and line, distinguishing registered placements from ordinary strings and from historical records that must not change.
4. **Where the redesign puts it**: the frames and the design-system sections that place it, by file.
5. **What a change requires**: the owner act the governance names (for example a PRD-level owner act for the report notice), the decision-register mechanics, the tranche or notice obligations, and which loops or mirrors would need notice.
6. **Unknowns** as TBD with what would settle them.

Close with a summary table (item, number of enforcement points, number of occurrences, kind of owner act) and a return section: what was read, model and effort actually used, uncertainties.

## Constraints that bind

- Read only; write only the one return file. No product, test, harness or governance file is modified.
- Every statement cites file and line. Where the constraints sheet and the source differ, the source governs and the difference is noted.
- No proposals, no replacement wording, no judgement on what the owner should decide. The packet is ROOT's to write.
- No delegation.

## Acceptance

ROOT accepts when every item has its verbatim text with source, its enforcement points with what they check, its occurrences classified, its redesign placements, and the owner act a change requires; the summary table is complete; unknowns are TBD rather than filled.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
