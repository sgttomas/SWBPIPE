# DAG-009 candidate evaluation — validation

## Verdict

PASS — VALIDATED PROPOSAL READY FOR BYTE-COPY. This verdict authorizes only
the N4 CHANGE handoff described in `APPLICATION_SPEC.csv`; it is not live DAG
activation or owner acceptance.

## Deterministic checks

| Check | Result | Evidence |
|---|---|---|
| Candidate schema | PASS | 31 columns, 1,480 rows |
| Five changed local consumers | PASS | canonical 29-column schemas; row counts 13, 20, 17, 14, 14 |
| Exact semantic delta | PASS | 13 dependency IDs; only `SatisfactionStatus`, `LastSeen`, and `Notes` differ |
| Intended closures and anchor | PASS | 12 status closures plus one anchor normalization |
| Preserved unresolved state | PASS | 12 failed candidates and five holds unchanged |
| Strict canonical DAG audit | PASS | 972 machine edges; zero SCCs, duplicate edges, bidirectional pairs, or endpoint failures |
| Node identity | PASS | DAG-008 and candidate node bytes match; SHA-256 `6e5050c4e578f6ff9819ee7a11dbb395b3f0a163b4fb0c48e88c3d084d9b0732` |
| JSON parsing | PASS | candidate DAG JSON, nodes JSON, edges JSON, and portability policy parse |
| Bundle role inventory | PASS | 16 required roles present |
| Bundle manifest | PASS | all 15 listed payloads reproduce; manifest SHA-256 `a8a1e3cbfecefea9114ca22ad396d4fabf0bfe1cf69ae76a0e8ff10a95f2d4fe` |
| Claims-language validator | PASS | 268 files scanned |
| Path-anchor validator | PASS | zero findings across 1,207 live path-anchor surfaces |
| Portability-policy tests | PASS | 51 tests passed |
| Write containment | PASS | changes confined to nine intended consumer files, the authorized policy file, the R23 run root, and the R23 evaluation root |
| Diff whitespace check | PASS | `git diff --check` |

## Portability amendment

Owner amendment `R23-SCOPE-004` is reproduced exactly. The prior policy SHA-256
was `f5a7d67feb57e74b2bc0763768a9ab145c92a6dd8c931eda22375b9b294b274a`;
the amended policy SHA-256 is
`f0dacfee06bdd4aac177a962f4710edc99fda0e073e53e965c88fb4d238fade4`.
All existing entries remain unchanged and ordered. Exactly five historical
`EVIDENCE` role overrides and one `CONTROL` path exception were appended, each
bound to its target SHA-256. No classified source file was edited and no
filename-wide exemption was added.

## Input drift reconciliation

The pre-amendment intake inventory contains 125 files. Of those, 121 remain
byte-identical. Four authorized changes are recorded: the portability-policy
basis transition and the manager-owned `RUNTIME_EVENTS.jsonl`,
`RUNTIME_SUMMARY.json`, and `WORK_GRAPH.json` bookkeeping required to record
the amendment and completed N3 transition. `INPUT_DRIFT_RECONCILIATION.csv`
and `UPSTREAM_INPUT_HASHES_POST_AMENDMENT.csv` carry the exact hashes.

## Independent fan-in

- N3A edge/evidence crosscheck: PASS, all 30 reviewed IDs and all required
  two-part evidence fields.
- N3B authority/schema crosscheck: PASS, all 16 roles plus manifest, schema,
  JSON, strict-DAG, topology, and authority checks.

The preserved bundle manifest is unchanged from the independently reviewed
bundle, so both returns remain valid.

## Boundary confirmation

No live `execution/_DAG/DAG-009` materialization, pointer update, receipt,
product or lifecycle status, memory update, Git action, or acceptance action
was performed. N4 CHANGE is READY; later owner gates remain in force.
