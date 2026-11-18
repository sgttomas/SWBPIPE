# A run recorded under a revision of its table this version does not hold (reopen witness)

DEL-05-02 and DEL-14-02: a stored result is bound to the semantic table that gives its rows their
meaning by the table's identifier and SHA-256 digest, carried in its record
(`reproducibility.semantic_contract` and each `result_refs[].semantic_contract`). A table revised
under the same identifier gets a new digest, and a record that binds a digest this version does not
hold is refused at every reader. Reopened, its rows are withheld. This directory holds one such record,
made by main's own record builder. It is the witness for that withholding
(`reopenTableBinding.test.tsx`, `stored_custody_tests.rs`, and the parity cases
`saved-revised-table:recorded_table_revised` and `revised-table:recorded_table_revised`).

| File | Origin |
|---|---|
| `pressure_connected_mechanics_sparse.json` | Main at e0e3e3c7a, verbatim (`fixtures/results/pressure_connected_mechanics_sparse.json`) |
| `exact_pressure_connected_request.json` | Main at e0e3e3c7a, verbatim (`core/product_physics/tests/fixtures/exact_pressure_connected_request.json`) |
| `pressure_plain_connected_sparse.analysis_run.json` | Built by main's record builder from that statement, in a temporary checkout of main whose pressure-3 table differs by one byte (below); binds `pressure-3` with digest `3471a7035b5b25240b99018edc13b248395252bffd1ed1c3866ba4677a14a812` |

**How the record was made.** It was made in a temporary checkout of main at e0e3e3c7a. One
newline was appended to `fixtures/results/semantic_contract_v0_3_pressure_3.json`, so no member's
value changed, only the file's bytes and so its SHA-256. Every pin was then moved by the
repository's own tool (`python3 tools/serialization/semantic_contract_tables.py repin --from-rev
HEAD`; that also moves physics-retained-1's inherited digest). The checkout's own
`openpipestress-runner` and checked-JSON adapter were built from it, and main's product record
builder ran on main's verbatim statement:

```
build_analysis_run(source, input_manifest_ref={"object_type": "InputManifest",
                   "ref": "input-manifest:recorded-table-revised"}, input_manifest_hash="1" * 64)
validate_analysis_run_v0_3(record, source, expected_basis_refs=record["analysis_run"]["load_basis_refs"])
json.dumps(record, ensure_ascii=False, indent=1) + "\n"
```

So the record's own hashes hold: it is not a record edited after the fact. It is a transient record
(`created_at` null), as main's builder writes before Record Run. The reopen cases make the recorded
revision of it as they do for `recorded_model_*`. This version holds no table with its digest, and
the digest is not among the known former digests, so the refusal uses the unrecognised-revision words.
