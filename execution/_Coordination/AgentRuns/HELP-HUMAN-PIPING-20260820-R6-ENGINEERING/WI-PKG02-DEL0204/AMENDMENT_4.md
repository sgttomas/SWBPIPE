# N1 integrated-review Amendment 4

Trigger: integrated review v4 after Amendment 3 / V20 fan-in.

Blocking finding: adapter declaration and operation-result provenance accepted
complete canonical but non-cleared status combinations, and
`protected_suspected` privacy was treated as an ordinary classification. The
public composed verifier could therefore report verification passed with a
blocked-runtime outcome for evidence that must reject or quarantine.

Bounded remediation: complete provenance now requires cleared redistribution
(`public_permissive` or `private_only`) plus `review_status=accepted`; canonical
unresolved or rejected review postures emit a blocking
`ADAPTER_PROVENANCE_NOT_CLEARED` finding. Protected privacy emits quarantine
`ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED`. Direct and composed tests cover
declaration and operation-result paths, diagnostic routing, malformed-
capability coexistence, protected top-level dominance, schema conformance, and
runtime non-dispatch.

Checks before freeze: complete focused/existing N1 suite `146 passed in 0.51s`;
composed schema assertions included; explicit four-path containment and diff
check PASS. Manager/status/shared fan-in updates remain deferred until a fresh
full amended-diff review passes.

V21 found truthy non-string provenance and hostile unhashable status values
could bypass or throw before quarantine evaluation. Remediation requires exact
nonblank plain-string provenance, retains protected/quarantined markers first,
and hardens diagnostic normalization plus composed canonicalization/boundary
ranking without mutating caller evidence. Direct/composed declaration/result
regressions include protected-privacy and malformed-capability coexistence.
Complete suite: `163 passed in 0.53s`; V22 fresh review required.

V22 found the corresponding hostile-status path in plugin-manifest provenance.
The plugin verifier now applies exception-contained reads, exact plain-string
marker-first handling, and a canonical snapshot before membership/clearance.
Direct manifest and composed protected-privacy regressions cover both status
fields and declaration/result surfaces without mutation or invention. Complete
suite: `169 passed in 0.54s`; V23 fresh review required.

V23 found raw schema enum evaluation could invoke caller-overloaded equality
before provenance validation. Manifest schema evaluation now operates on a
once-serialized, `allow_nan=False` detached plain-JSON snapshot; raw evidence is
not mutated and still fails exact provenance shape structurally. Direct and
composed raising-comparator regressions cover both status fields and protected
declaration/result privacy. Complete suite: `175 passed in 0.55s`; V24 fresh
review required.

V24 found later semantic checks still operated on raw caller values. A recursive
exception-contained exact raw-JSON preflight now rejects subclasses, nonfinite
numbers, cycles, and hostile containers before overloaded access. One detached
strict snapshot drives schema, all manifest semantics, composed diagnostic
context, and envelope boundaries while raw provenance retains its specific
structural finding. Complete suite: `179 passed in 0.82s`; V25 fresh review
required.

V25 found recursive preflight could overflow before structured rejection. The
raw-shape walk is now iterative, active-ancestor cycle-aware, exact-type, and
exception-contained, with deterministic maximum JSON depth 512 before strict
serialization. Direct/composed deep, cyclic, nonfinite, and hostile-container
regressions preserve protected quarantine and non-dispatch. Complete suite:
`190 passed in 0.52s`; V26 fresh review required.
