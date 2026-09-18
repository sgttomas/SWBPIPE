# Activation — SWBPIPE interface implementation

Status: ROOT record (HELP_HUMAN, Agent 0), 2026-09-18. Base: `origin/main` at `04aefa9803a6a9d26dae39ad6bbc06c470963fad`. This run implements the design handed over by `../HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md`. The design run's activation limited that run to design; this record is the owner's authorization for implementation, in a separate run.

Custody: the owner's messages below are the bytes the host stored in the session transcript, with their SHA-256 over UTF-8. This is stored-transcript custody, not transport bytes.

## 1. The owner's messages

2026-09-18T19:03:58.811Z, 103 bytes, SHA-256 `c81eb7c5b0aedf7c49bb599c1e529aa2fd632de0e577255e055988ebf596ca3c`:

> I actually want you to do the implementation, not hand it off.  Thoughts?  Don't proceed, just discuss.

ROOT answered with a discussion and three questions: whether implementation by this program is authorized, whether the piping loop stays on hold while ROOT works in its source, and whether SCA-010 stands as narrowed.

2026-09-18T19:07:52.185Z, 226 bytes, SHA-256 `50d2fd176f7b9602365559c165d1a60e34905da512bfb7ac5e4bd3a269e636bf`:

> I authorize implementation; piping loop stays on hold; SCA-010 stands.  Plan out implementation from the perspective of Agent 0 and consider an effective delegation and model selection strategy for Type 1 and Type 2 instances.

## 2. The owner's three rulings in planning

ROOT put three questions through the host's question form. The host stored the owner's selections as one result, 2026-09-18T19:33:29.650Z, 1139 bytes, SHA-256 `96a3d2bb1d6f5cd6c09bf9f996172effe416bd27a26efa23aa00a17b7f06357c`. Each question's wording and each option's label were ROOT's; "(Recommended)" marks ROOT's recommendation. The owner selected:

| Question (ROOT's short name) | The owner's selection | Effect |
|---|---|---|
| Rename order | "Visible, shell, identity (Recommended)" | Tranche A1 (the visible rename and the rulings), then Tranche B (shell and canvas), then Tranche A2 (the identity layer of `DEC-101` (iv)) |
| Hard identifiers | "Keep all four as they are (Recommended)" | `openpipestress_jcs_ijson_v1`, `openpipestress_result_semantics_v0_2`, the `.opsproj` extension and the store filename `openpipestress-projects.sqlite3` do not rename. This is an exception to `DEC-101` (iv), ruled by the owner; ROOT proposes its text for the decision record with Tranche A2 |
| Root lint | "Yes, edit the root lint in Tranche A1 (Recommended)" | The owner directs the edit of `{REPO_ROOT}/tools/validation/validate_claims_language.py` and its test, in the same tranche as the registry and product text, under a tranche manifest and independent review |

The stored result, verbatim:

> Your questions have been answered: "DEC-101's rename has two layers. The visible layer (name, removed sentences, labels, registry, lint, packaging, PRD) is ~150 files. The identity layer (schema namespace, 65 document kinds, crate and binary names, 95 hash-bearing fixtures) is ~450 more and touches the same source the new shell rewrites. In what order?"="Visible, shell, identity (Recommended)", "Four identifiers are not user-visible and renaming them has a cost: the hashed canonicalization profile id and result-semantics id (renaming invalidates recorded hashes), the .opsproj file extension, and the local store filename (renaming orphans existing stores without a fallback). What should happen to them?"="Keep all four as they are (Recommended)", "The claims lint at the repository root embeds the maturity sentence and the content-boundary text with the old name. DEC-101 and DEC-105 say it must change in the same tranche as the product text, and that a root tool edit needs your explicit direction. Do you direct that edit?"="Yes, edit the root lint in Tranche A1 (Recommended)". You can now continue with these answers in mind.

The owner then approved ROOT's plan through the host's plan-approval step. The approved plan is reproduced as `../ORCHESTRATION_PLAN.md`.

## 3. What this authorizes and what it does not

Authorized: implementation of the accepted design in `{WORKING_ROOT}` product source, tests, schemas and documents, in the tranche order above, under the handoff's ten constraints and its verification, with Git operations under the standing authorization of 2026-09-12. The piping loop stays on hold; this run is the only writer in `apps/desktop` and `core` for this work.

Not authorized by these messages, and returning to the owner: the new Apple App ID (the owner's act); typed-interface and engine gaps, per gap; any change to a ruling; product release; lifting the usability holds PDU-045 and PDU-046. Tolerances, oracles and limits are never changed to obtain a benchmark result.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
