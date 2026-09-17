Extract, as an objective reference sheet, every governance rule that constrains what the OpenPipeStress GUI may show, say, or do. This sheet will be used by a UI design effort so that new designs do not violate accepted project invariants. Extraction only — no design opinions. READ-ONLY: do not modify any repository file.

Repository root (git worktree, run everything from here, do not cd elsewhere):
{REPO_ROOT}
Project root: projects/chirality-piping

Write your output ONLY to:
{SCRATCHPAD}/research/C_ui_constraints.md
(create the directory if needed).

Sources to read (project-root relative):
- docs/CONTRACT.md (all invariants; pay attention to IDs like OPS-K-DATA-*, UNIT-*, IP-*, PRIV-*, AUTH-*, AGENT-*, and anything about GUI, warnings, reports, claims)
- docs/DIRECTIVE.md, docs/PROFESSIONAL_BOUNDARY.md, docs/claims_registry.md
- docs/PRD.md sections 5.3, 5.5, 5.6, 5.7, 5.9, 9, 14 (all of 14), 15, 21 (especially 21.2 prohibited statuses/claims)
- docs/SPEC.md section 8 (GUI requirements) and the warning-class table
- docs/INTENT.md "Graphical user interface intent"
- execution/_Coordination/_DECISIONS/_REGISTER.md and the decision files for anything touching GUI/UX/claims: search for DEC-081 (claims-language taxonomy), DEC-018 (dual-unit display), DEC-037 (structured composer), DEC-041/DEC-042 (agent panel seam / harness-independent redesign), DEC-020 (wasm engine), D-58 (live agent provider), D-21, and any decision mentioning "GUI", "UI", "usability", "viewport", "inspector", "palette", "toolkit". Use grep -rl across execution/_Coordination/_DECISIONS and execution/_Decomposition/SOFTWARE_DECOMP.md §12.
- grep for "F-PIP-2" and "claim fence" to find the standard claim fence definition; grep for "H4" UI evidence posture (plans/DRAFT_2026-06-11_H4_coordination_evidence_posture.md if present) and summarize its testing expectations for UI changes.
- execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md — list the 24 normative interactive-operation vocabulary rows (name + one-line meaning) and the 3 roadmap rows.
- execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_STATUS.md and _CONTEXT.md — extract the PDU-045/PDU-046 usability holds and any stated viewport minimums (1024x768 etc.).

Output format:
1. Constraints table: ID (your own C-01..), source (file + section/ID), rule (one sentence, quote key phrase), UI implication (one sentence: what a screen must/must not do).
   Group by: Claims & professional boundary; Missing data & no silent defaults; Units; Provenance & IP boundary; Warning/blocking taxonomy; Model states/runs/results (Current vs Historical, immutability); Agent proposals & review gates; Privacy/telemetry/local-first; Accessibility/usability holds; Testing/evidence expectations for UI changes.
2. Accepted GUI-related decisions: table of DEC/D IDs with date and one-line summary and whether they bind future UI design.
3. The 24+3 vocabulary rows.
4. A short list of "mandatory disclosures" — text or state that must be visible somewhere in the product (e.g., the technical-preview / professional-judgment notice), with source.
Be precise and cite. Do not editorialize.