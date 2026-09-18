# SCA-010 Amendment Preview

The amendment is one replacement applied uniformly. This preview shows the header, the required notice, and the rule for every other occurrence, instead of reproducing the 1,787-line PRD.

## Header, before

```text
# Product Requirements Document: OpenPipeStress

**Working title:** OpenPipeStress
**Document version:** 0.3 Draft
**Amended:** 2026-07-16
**Status:** Adopted PRD authority per D-21 / DEC-056 / SCA-005, as amended by D-47 / DEC-080 / SCA-007 (2026-07-16)
```

## Header, after

```text
# Product Requirements Document: SWBPIPE

**Product name:** SWBPIPE
**Formerly:** OpenPipeStress (renamed by D-71 item 3 / DEC-101 / SCA-010, 2026-09-18)
**Document version:** 0.4 Draft
**Amended:** 2026-07-16; 2026-09-18
**Status:** Adopted PRD authority per D-21 / DEC-056 / SCA-005, as amended by D-47 / DEC-080 / SCA-007 (2026-07-16) and by D-71 / DEC-101 / SCA-010 (2026-09-18, product name only)
```

## §19.3 required notice, after (and its appendix copy, identically)

```text
SWBPIPE is decision-support software for piping design, flexibility, and stress-analysis workflows. It computes mechanical results from recorded user inputs and may evaluate user-supplied rule packs. It does not certify, seal, approve, authenticate, or determine code compliance for professional reliance. Code-specific and project-specific data are supplied by the user or user-controlled private sources. Competent human review and, where required, validation in accepted professional tools remain the responsibility of the user and project authority.
```

Only the first word differs from the adopted text.

## Every other occurrence

Each of the remaining body occurrences of "OpenPipeStress" becomes "SWBPIPE" with no other change to its sentence, for example the boundary statement at the head of the PRD: "SWBPIPE computes and helps design. The external prover tool validates for reliance. The responsible engineer accepts."

## `docs/report_notice_template.md`

Line 16 "…notice language for SWBPIPE-generated reports"; line 41, the notice as above; line 68 "SWBPIPE version or commit basis used to generate the report."

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
