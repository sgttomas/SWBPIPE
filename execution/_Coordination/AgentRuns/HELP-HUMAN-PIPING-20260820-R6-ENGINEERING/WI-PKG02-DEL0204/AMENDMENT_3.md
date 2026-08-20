# N1 integrated-review Amendment 3

Trigger: integrated review v3 after Amendment 2 / V18 fan-in.

Blocking finding: adapter capability validation converted arbitrary iterables to
a set. Strings, tuples, and nonempty lists containing arbitrary hashable values
could therefore be treated as accepted declarations despite the canonical
`AdapterCapability` array and enum contract.

Bounded remediation: validate the already-loaded declaration in memory as an
exact JSON-array-shaped list of canonical string enum values, require at least
one of the operational capabilities named by the schema's `contains` rule, and
reject malformed or unhashable entries without throwing. Duplicate canonical
values remain accepted because the canonical schema does not require unique
items. Protected/quarantined provenance is still evaluated independently and
retains outcome precedence.

Checks before freeze: full focused/existing N1 suite `126 passed in 0.49s`;
composed envelope schema assertions included; runtime dispatch remains blocked.
Manager/handoff/shared fan-in updates remain deferred until fresh full amended-
diff review passes.

V19 found one hostile string-subclass exception path. Exact plain-string item
validation plus direct and composed protected/quarantined regressions close it;
the complete post-remediation suite is `129 passed in 0.50s`. V20 fresh review
is required before fan-in.
