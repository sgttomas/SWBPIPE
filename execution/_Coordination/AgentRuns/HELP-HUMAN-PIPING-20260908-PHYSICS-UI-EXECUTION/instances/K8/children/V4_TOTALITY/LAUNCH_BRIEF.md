# K8 V4 totality Agent 2 sealed brief

Status: `SEALED_PRELAUNCH`. Run: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`. Parent: `/root/canonical_design`, WORKING_ITEMS Agent 1. Execution form: fresh delegated-harness-native ephemeral Agent 2. Required runtime: `gpt-5.6-sol`, reasoning `high`, `fork_turns=none`. The child must not delegate or message siblings.

## Objective

Resolve only RK-V3-001 from `instances/RK/BACKCHECK_RETURN_V3.md` SHA-256 `50ad5912f72acfca1f06e2868a448e8b60c51d8f817b1629390fe88d6bff5aee`. Produce a V4 semantic validator and systematic adversarial test that establish total writer/reader validation over the full malformed-value class. Preserve V1/V2/V3 bytes, stable well-typed error codes, the already-closed API/legacy-reader design, and the already-closed pressure sequencing.

## Read scope

- Root and project `AGENTS.md`, `agents/AGENT_WORKING_ITEMS.md` only as needed for the bounded role.
- `instances/RK/BACKCHECK_RETURN_V3.md`.
- K8 frozen `MANIFEST_V3.json` and its V3 validator, builder, schema, mapping and adversarial test.
- Current `jsonschema` behavior/documentation available locally; no network research.

## Method and acceptance

Before any `.get`, hash lookup, string-key method or semantic relation, establish:

1. the input is representable as strict JSON at the host boundary, including string-only object keys and finite JSON number types; and
2. the complete structural/type schema phase has passed for every consumed field.

Only then perform semantic/version/kind dispatch. Use a structural JSON-Schema/type phase rather than another list of individual container guards. Keep `RESULT_EXPORT_SCHEMA_INVALID` for malformed types, `RESULT_EXPORT_NON_FINITE_NUMBER` for non-finite host values, and existing specific relation/unknown-kind/owner codes only for well-typed inputs. Do not add a catch-all exception handler that hides programming defects.

Tests must cover both writer and reader for systematic `null`, boolean, number, string, array and object substitutions over every consumed relation field. They must explicitly include `source_record.kind=[]`, `source_record.unit={}`, numeric kind, and a writer-only non-string object key. Prove valid 830-row round trip, existing well-typed duplicate/mirror/reference/owner/unknown-pair behavior, strict finite behavior, and zero uncaught exceptions across the substitution matrix.

## Write scope and output

The child may create exactly these two files and no others:

- `instances/K8/evidence/semantic_validator_v4.py`
- `instances/K8/evidence/adversarial_tests_v4.py`

The child must not run the V4 builder, because the manager owns that file and generated outputs. It may import the frozen V3 builder/schema evidence for tests. Return exact file hashes, commands and observed results in the child final response. No production source/schema/adapter, deliverable run-record, manifest, status, register, DAG, lifecycle, public compatibility or physics write is permitted.
