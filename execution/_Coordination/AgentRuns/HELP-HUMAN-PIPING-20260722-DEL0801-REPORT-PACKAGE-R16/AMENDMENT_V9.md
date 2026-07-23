# Candidate amendment v9 — spring-hanger authority ratification

**Authority:** exact owner act in `OWNER_ADOPTION.md`  
**Product/source/schema/decomposition change:** none  
**Executor:** N4, verification only

V9 updates only `WRITE_MATRIX.csv` row 6 to ratify the existing
`renderableReportInput.ts` behavior:

1. select force with precedence
   `constant_load → hot_load → cold_load → installed_load`;
2. emit `{magnitude, unit, dimension: "force"}`;
3. set `required_for` to reporting and human review only;
4. set missing-data finding when no force quantity exists.

No implementation edit is authorized. N4 repeats affected/full/native gates
against the unchanged product, seals records, and runs exactly one v9
replacement DEC-025 sweep. No edit follows sweep invocation. Fresh N5 is
read-only/terminal-only. V6–v8 sweeps remain immutable superseded evidence; W3
remains gated on N5 `COMMIT-SAFE`.
