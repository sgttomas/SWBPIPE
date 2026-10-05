# RV78: the two carried artefacts (retained semantic table; results.v0.3 successor branch)

RV78 is a TASK (Type 2) dispatched by ROOT (HELP_HUMAN) for a scoped review of two artefacts. Both arrived with the reader drafts frozen at the handoff (READER `ae97b7d5c2`) and are now on NUM. ROOT is the return path, and RV78 did not delegate. RV78 wrote neither artefact.

- **Basis:** NUM `b819b902a2` (readers fanned in at `c15e64b756`).
- **The artefacts:**
  - `P/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json`, sha256 `c74742ce…`;
  - `P/schemas/results.v0.3.schema.yaml`, sha256 `4585a45f…`, compared with the merge base `a8bec61e8c`.
- **The inherited table:** `semantic_contract_v0_3_preview_physics_1.json`, sha256 `ae55503d…`.
- **Contract:** C1 §3 (identities, profiles, inherited hash) and §6 (C1:162, the carrier schemas); C3 §1 and its G0 row; C3 §4.
- **Run window:** 2026-10-03, 20:58:55 to about 21:06 local, inside the 45-minute box. Nothing in scope is unfinished. The memory guard (PID 5387) was running.
- **Limits held:** read-only on NUM, apart from this folder. No Git writes or installs. Probes ran from NUM with VENV.

## Verdict: PASS

**Counts:** 0 BLOCKING, 1 SHOULD-FIX, 4 NOTE.

**D36 triage:** nothing found could change eligibility or a selected case's standing, so **no finding gates acceptance**. All five go to the next reader or carrier round on NUM.

| Question | Answer |
|---|---|
| Does the table differ from the inherited one only where the successor requires? | **Yes.** There are exactly 8 differences, each grounded in C1/C3 (TABLE_DIFF.json). The rows, vocabulary, hash vectors and every inherited policy are unchanged. |
| Does the YAML admit exactly the successor shape, without loosening preview-physics? | **Yes, for the results document.** Branch 7 equals the preview-physics-1 branch plus a required `retained_precision`, through a `$ref` to the closed receipt schema. All 7 existing branches gained "`retained_precision` not present", so none is loosened. The row `recovery_method` token is a raw-producer-row field; the results 0.3 *derivative* rows are closed and correctly cannot carry it. |
| Do the existing schema tests exercise it? | **Partly.** One test checks four cases. RV78's 26 probes cover the rest, and all behave correctly (N2). |

## Findings

| ID | Severity (D36) | Where | Evidence | Remedy |
|---|---|---|---|---|
| S1 | SHOULD-FIX, **not gating** | `P/schemas/analysis_run.v0.3.schema.json`, `P/schemas/stress_neutral_export.v0.3.schema.json` | C1:162: "Schemas add explicit successor branches in results, AnalysisRun and stress-neutral carrier." Only the results schema has one. Both carrier schemas name `preview-physics-1` (4 and 7 occurrences) and nowhere mention `retained_precision` or `preview-physics-retained-1`. **This fails closed:** an AnalysisRun or export carrying a successor result cannot validate. So it cannot create eligibility, and the T6 export refusal stands. | Add the two successor branches with the carrier/serializer work, where C1:162's save/reopen and derivative-copy obligations live. |
| N1 | NOTE | The retained table | C1:88: "Policy/table hashes bind these stable contents and limits." The table binds `receipt_policy`, the definition (`product_formation_definitions`) and the facade policy (in `accuracy_classification`). It does not bind the projection policy `RP-LOGICAL-ATTEMPTS-v1`, the work policy `W1-LME-20B-60B-v1` and its 20B/60B limits, the method token, or the canonicalization profile. All three readers enforce these as G0 constants (D2), so behaviour is unaffected. | Optionally add them to the table at the next table revision, which changes its pinned hash in all three readers. |
| N2 | NOTE | `tests/test_retained_precision_schema.py::test_results_successor_branch_is_explicit_and_old_branches_reject_receipt` | **What the test checks:** case 0's successor document is valid; a missing receipt is rejected; the historical preview fixture is valid; a receipt on the preview branch is rejected. **What it doesn't:** the other 14 bases; `source_block_recovery` on the successor; profile, limitation and contract-ref mismatches; a receipt invalid through the `$ref`; the other six historical branches rejecting a receipt. RV78's probes Y0–Y11 show all of these behave correctly. | Add them as schema tests in the next round (YAML_PROBES.json has the cases). |
| N3 | NOTE | results.v0.3 successor branch | C1:162: "Canonical derivatives copy it [the receipt] and all absolute/not_covered disclosures." The branch requires the receipt but does not constrain `row_disclosures` to carry the absolute/not_covered disclosures. The derivative's value rows (`PreviewPhysicsQuantityResult`, closed) carry no method token. That is consistent with C1:162's transport rule ("executes only G0–G2… never eligible"). | Specify the derivative's disclosure carriage with the carrier work, alongside S1. |
| N4 | NOTE | results.v0.3 YAML, `PreviewPhysicsContractEvidence.description` | One literal `§` became `§`, a re-serialization artefact. The JSON value is identical. No consumer outside execution records pins either the old or the new file hash. | None needed. |

## 1. The retained semantic table

**The diff against the inherited table** (RV78's own structural diff, TABLE_DIFF.json): exactly 8 differences.

| Path | Change | Contract basis |
|---|---|---|
| `semantic_contract_id` | preview-physics-1 → **preview-physics-retained-1** | C1 §3 |
| `formulation_profile_id` | product_preview_mechanics_v1 → **product_preview_retained_w1a_v2** | C1 §3 |
| `inherited_semantic_contract_sha256` | `d75aacee…` → **`ae55503d…`** | C1 §3, "inherits exact preview base table hash". It is the raw-file sha256 of the inherited table, the same convention the base uses (its own value is precision_1's raw sha). |
| `receipt_policy` (added) | `M03-INTEGRITY-MP-v2` | C1 §3 |
| `receipt_schema` (added) | `retained_precision_mp_v2.schema.json` | the closed receipt schema; no dated path (C1 §3) |
| `product_formation_definitions` (added) | `[{id: RP-PREPARED-ORDINARY-DUAL-v1, sha256: a7ed7ca0…}]` | C3:21–25 (the closed `[{id,sha256}]` member) |
| `accuracy_classification` (added) | `RP-FACADE-SI-v2`; small-scale threshold 2^-988; absolute bound `RU64(2^-64 S)` with the small-S form; the five classes; standing before S-I; ordinary-prepared scope | C1:158–160 and C1 G5c. The relative threshold 2^-34 equals the readers' `floor_ratio` and TypeScript's class constant (basis D2/R7; not rederived here). |
| `formation_warrant` (added) | D2 §4.9.10, §4.11.2 and §5 I-9 replaced for this definition only; no S-I activation | C3 §4 cross-reference (C3:321–334) |

**Unchanged:** all 73 rows, the metadata vocabulary, the hash vectors, the source producer and schema versions (0.2.0), the signature and kind counts, the metadata, combination and contract-evidence policies, the reserved successors, the retired kinds and the supported limitations.

**G0 binding (C3's G0 row),** read in all three readers:
- Python (retained_precision.py:1596–1601), Rust (retained_precision.rs:488–535) and TypeScript (retainedPrecision.ts:187–189) all check the table bytes' sha256 against the pinned `c74742ce…`.
- All three check the inherited table bytes against the table's own `inherited_semantic_contract_sha256`.
- All three check the definition's H against `a7ed7ca0…`, failing with FORMATION_MISMATCH.
- TypeScript also cross-checks the table's id, profile and `product_formation_definitions` entry; the pinned table hash makes those values fixed in Python and Rust as well.

## 2. The results.v0.3 successor branch (149 changed lines against `a8bec61e8c`)

**What changed:**
- **Envelope-level enumerations** gain `preview-physics-retained-1` (producer id, contract ref) and `product_preview_retained_w1a_v2` (profile). Each `oneOf` branch still fixes its own values by `const`, so nothing is loosened.
- **`ResultEnvelope.properties` gains `retained_precision`** (a `$ref` to `retained_precision_mp_v2.schema.json`). Every one of the 7 existing branches gains "`retained_precision` not present".
- **New branch 7** for `preview-physics-retained-1` is structurally identical to branch 4 (preview-physics-1): the same `contract_evidence`, `result_sets`, and "no `source_block_recovery`". It differs only in:
  - the contract id and ref;
  - the profile;
  - the limitations constant, which equals the retained table's `supported_profile_limitations` exactly;
  - the required `retained_precision`.
  
  Branches 4 and 7 are mutually exclusive, because one forbids the receipt and the other requires it.

**Probes** (YAML_PROBES.json; the repository's own `validate_instance`, which resolves the local `$ref`):
- the successor documents for **all 15** corpus bases validate;
- a receipt with policy v1, an unknown member or an empty body is rejected, which shows the `$ref` is enforced;
- the successor branch is rejected with `source_block_recovery`, with the preview profile, with changed limitations, with a contract-ref mismatch, or without `contract_evidence`;
- the preview branch carrying a receipt is rejected;
- the historical preview fixture still validates;
- a derivative value row carrying a method token is rejected.

**The repository's tests:** `test_retained_precision_schema.py` passes, 12 passed.

## 3. Method and evidence

**Evidence in this folder:**
- **TABLE_DIFF.json:** the 8 differences, with their contract basis.
- **YAML_PROBES.json:** 26 probes, all matching their expected outcome.
- **SHA256SUMS.**

**Scripts and logs** are in `WT/scratch/rv78_carried_artefacts/`: `rv78_yaml_probes.py` and the YAML diff.

**Commands:**
- `VENV/bin/python -m pytest -q -p no:cacheprovider tests/test_retained_precision_schema.py`, from `NUM/P`;
- `VENV/bin/python rv78_yaml_probes.py NUM/P`.

## 4. For ROOT

Nothing gates acceptance. Route S1 and N3 to the carrier/serializer work, and N1 and N2 to the next reader round.
