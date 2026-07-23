# Candidate amendment v7 — accepted run-local remediation

**Basis:** `R16-SOFTWARE-DECOMP-REMEDIATION-PLAN-001`  
**Authority:** owner Gate-7 acceptance recorded in `OWNER_ADOPTION.md`  
**Canonical decomposition effect:** none  
**Implementation owner:** N4, sole serialized owner

Fresh N5 blocked v6 on four semantic defects: false input-manifest evidence,
misdimensioned stiffness results, invented-public classifications copied into
current-private records, and malformed SHA-256 acceptance. The owner accepted
the final run-local derived remediation basis and the expanded exact write
matrix.

## Serialized slices

1. `R16-SLICE-01` / DEL-08-02 — deterministic current-session input manifest,
   stable ref, exact SHA-256, mutation/hash/blocking tests.
2. `R16-SLICE-02` / DEL-14-02 — bind manifest ref/hash and explicit source
   result dimensions through analysis-run schema, Python, TypeScript, and tests.
3. `R16-SLICE-03` / DEL-08-04 — add only `linear_stiffness` and
   `rotational_stiffness` to the result schema and Rust exporter with tests.
4. `R16-SLICE-04` / DEL-08-01 — consume the actual manifest and declared
   dimensions, project copied records as current-private, validate exact
   lowercase 64-hex SHA-256 at TypeScript and Rust boundaries, and add negative
   tests.
5. `R16-SLICE-05` / DEL-08-01 — focused/full gates and replacement
   packaged-native cancel/new-save/replace proof.

## Binding constraints

- Option A: no new package member and no portable-replay claim.
- Result-envelope hashes are never relabeled as input-manifest evidence.
- Consumers do not infer stiffness dimension from unit text.
- Package-owned current-session copies are
  `private_project_data`/`private_only`/pending/user-local without mutating
  source records.
- Every wire value declaring SHA-256 matches bare lowercase
  `^[0-9a-f]{64}$`.
- PKG-02 is read-only semantic authority.
- Product physics, report-package member vocabulary, runner/DEL-10-05,
  predecessor status/memory, lifecycle, release, DAG, and every path absent
  from `WRITE_MATRIX.csv` remain excluded.

After slices 01–05 and all gates/native proof pass, N4 seals records and runs
exactly one replacement DEC-025 sweep. The v6 sweep remains immutable
superseded evidence. No edit follows the replacement sweep. Fresh N5 is then
read-only; W3 requires N5 `COMMIT-SAFE`.
