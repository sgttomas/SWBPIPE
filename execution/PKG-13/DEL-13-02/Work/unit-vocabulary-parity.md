# Constraint quantity vocabulary: remaining parity gap

`schemas/constraint.schema.json` does not include `force_per_length` in the
Quantity dimension enum. `tests/test_constraint_schema.py` compares it with
its own fixed `ACCEPTED_PKG02_DIMENSIONS` list; that agreement does not prove
parity with the canonical unit vocabulary. The ScopeOfWork's fixed-vocabulary
qualification remains applicable.

Next actor: Agent 0 scopes a comparison with the maintained canonical units,
then reconciles the schema and meaningful parity coverage. This finding
does not authorize invented dimensions, change unit conversions or establish
runtime constraint validation.
