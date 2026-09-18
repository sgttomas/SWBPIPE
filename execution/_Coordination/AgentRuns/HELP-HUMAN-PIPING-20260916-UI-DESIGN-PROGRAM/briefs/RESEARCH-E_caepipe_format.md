# Sealed brief — RESEARCH-E: CAEPIPE exchange format

Sealed by ROOT (HELP_HUMAN) on 2026-09-17 before launch. Role: TASK, bounded research. Model requested: Claude Opus 5 (the result is objective and quantitative: what a documented file format contains). Mechanism: Claude Code `Agent` tool, general-purpose type, background. Type 2 does not delegate; this child works alone.

## Purpose

The owner has chosen the CAEPIPE layout grammar for the SWB Piping Designer tables (one row per node carrying the element that arrives at it, From explicit, branches as rows whose From names an earlier node, element type as a column) and wants to export files compatible with CAEPIPE. Characterise, from public documentation only, what CAEPIPE's exchange formats contain, so that the design program can confirm the row grammar and field names map onto them and can name the gaps.

## Accepted basis

- `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/DIRECTION_DECISION_2026-09-17.md` §2 item 1: the grammar to map onto.
- `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/RESEARCH/D_domain_ux_research.md`: what was already found about CAEPIPE's layout window; do not repeat it, extend it.
- Public sources: the CAEPIPE product pages and manuals published by SST Systems, and any public documentation of its import and export formats. Record every source with its URL and the retrieval time.

## What to deliver

One file, `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/RESEARCH/E_caepipe_format.md`, with these sections:

1. **Sources.** Each public source read, with URL, title, publisher, retrieval time (UTC) and what it covers. Sources you could not reach are listed as unreachable, not inferred.
2. **Formats CAEPIPE reads and writes.** Every model exchange format the public documentation names (for example a batch or text model file, neutral formats, exports to other tools), with what each is for and whether it is documented well enough to write to.
3. **Layout row fields.** For the documented text or batch model format, a field-by-field table of the layout data: node identifiers, element types and their type-specific data, lengths or offsets, section, material, temperatures and pressures, and how From, To and branches are expressed. Give units conventions.
4. **Attachment and auxiliary data.** How restraints, anchors, guides, limit stops, hangers and spring data, loads, SIFs and other node data are expressed, with fields.
5. **Load cases and analysis options.** How load cases and combinations are expressed, if the format carries them.
6. **Mapping.** A table from the chosen grammar's tables and columns to the documented format's fields, marking each as direct, transformable (say how), or missing on either side.
7. **Open questions.** Everything not determinable from public documentation, stated as TBD with what would resolve it.
8. **Return.** What was read, model and effort actually used, and anything uncertain.

## Constraints that bind

- Public documentation only. Do not download or install software, do not reverse-engineer binaries, do not use sample files of unknown licence. Reading web pages and published manuals online is in scope.
- Every statement about the format cites a source. Nothing is inferred from memory of the product; if a fact comes only from prior knowledge, mark it so and list it under open questions.
- No claim of compatibility. The return describes what the documented format contains; whether an export is compatible is for later evidence.
- Write only the one return file. Modify no product source, test, harness or governance file.
- No delegation.

## Acceptance

ROOT accepts when: the sources section is complete and every format statement is cited; the layout fields, attachment data and load cases are tabulated to the level the documentation supports; the mapping table names every gap; unknowns are TBD rather than filled.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
