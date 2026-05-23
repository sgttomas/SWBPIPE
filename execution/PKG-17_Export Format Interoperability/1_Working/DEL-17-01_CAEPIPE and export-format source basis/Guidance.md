# Guidance: DEL-17-01 CAEPIPE and export-format source basis

## Working Guidance

Use DEL-17-01 as a source authority and guardrail document. Its value is not in deciding the final exporter design; its value is preventing downstream exporter work from inventing target behavior or silently crossing IP/professional boundaries.

## How to Use the Source Basis

- Treat official/public vendor documentation as evidence for documented interfaces only.
- Treat `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` as the accepted human-authored strategy, not as a substitute for target-format details.
- Treat project governance docs as binding constraints whenever source material is silent, ambiguous, or tempting to overstate.
- Treat any unsupported, version-sensitive, undocumented, or inferred target behavior as `TBD`.

## CAEPIPE Guidance

CAEPIPE is the first CAEPIPE-focused source-basis target because the admitted references support a text model handoff and optional external execution workflow. This does not make OpenPipeStress a CAEPIPE replacement, does not bundle CAEPIPE, and does not create a claim that exported models are accepted engineering work.

For downstream work:

- prefer MBF over PCF for the first deterministic CAEPIPE exchange backbone;
- keep all CAEPIPE-specific assumptions versioned and profile-scoped;
- require loss reports for unsupported or approximate target behavior;
- keep executable paths, licenses, and execution environments user-owned;
- record parsed CSV results as regression/handoff evidence only.

## PCF Guidance

PCF should be treated as broader interoperability, not the first exchange backbone. The CAEPIPE PCF documentation indicates translator-dependent mappings and defaults. Downstream PCF work should therefore begin from a conservative subset and report limitations explicitly.

## GLB/glTF Guidance

GLB/glTF should be used for visual review and lightweight geometry inspection. It should not be described as solver input, stress-model proof, or professional acceptance evidence. Any stable ID mapping that cannot be carried directly in the geometry file should be preserved in sidecar records.

## Question-Dossier Guidance

The CAEPIPE developer-team question dossier should ask for clarification only where public documentation is insufficient for deterministic, supportable behavior. Questions should avoid requesting proprietary internals, reverse-engineered behavior, licensed examples, or protected code/compliance content.

Good questions are about:

- supported public interface behavior;
- version/profile expectations;
- allowed command-line invocation shape;
- stable CSV sections suitable for automated parsing;
- MBF field/record interpretation boundaries;
- recommended citation targets for public documentation.

Poor questions are about:

- proprietary solver algorithms;
- hidden translator internals;
- commercial example files;
- code-compliance calculations;
- protected standards content;
- license-bypass behavior.

## Review Guidance

Reviewers should reject this deliverable if it:

- overstates CAEPIPE compatibility;
- treats PCF translation as deterministic without qualification;
- implies successful export equals engineering acceptance;
- embeds proprietary examples or protected standards data;
- fails to mark unknown target behavior as `TBD`;
- lets later DEL-17 deliverables proceed without consuming this source basis.
