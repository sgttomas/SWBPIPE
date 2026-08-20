# N1 integrated-review Amendment 5

Trigger: integrated review v5 after Amendment 4 / V26 fan-in.

Blocking findings: public composed verification still traversed raw caller
adapter, unit-catalog, and unit-evidence mappings/lists outside complete
exception containment; malformed manifest structure also discarded safely
readable protected/quarantined provenance markers.

Bounded remediation: every caller adapter, catalog, and unit-evidence input is
converted through bounded exact-JSON normalization into a detached plain
snapshot before validation, lookup, diagnostics, provenance/privacy ranking,
or envelope construction. Hostile/custom accessors, subclasses, cycles, depth/
node/byte limits, nonfinite values, and serialization failures produce
structured conservative malformed contexts without mutation or exception
escape. Safely observable exact protected/quarantined markers retain quarantine
and envelope precedence even when unrelated structure is malformed.

Direct and composed regressions cover adapter, catalog, and evidence mappings/
lists/accessors plus cyclic, deep, nonfinite, and protected-marker combinations;
results remain schema-valid and runtime never dispatches.

Checks before freeze: complete N1 suite `219 passed in 0.56s`; canonical
composed-result schema, explicit five-path containment, and diff check PASS.
Manager/status/shared fan-in updates remain deferred until fresh full amended-
diff review passes.

V27 found malformed manifest marker fallback gaps, missing node/text/byte bounds,
and a truncated unit-evidence marker scan. Remediation preserves safely
observable provenance/metadata quarantine markers, enforces deterministic
10,000-node, 1 MiB UTF-8 text, depth-512, and 1 MiB serialized manifest limits,
and scans the complete bounded unit-evidence list. Thirty-four direct/composed
regressions include markers at indexes 1,024 and 1,500. Complete suite: `253
passed in 0.64s`; V28 fresh review required.

V28 reviewed all 17 files/6,964 lines and failed on three fallback defects:
hostile colliding dictionary keys could escape marker observation, unit
fallback width was not bounded after snapshot failure, and malformed manifest
fallback reused looser adapter bounds/copied raw identifiers. Exact remediation
is frozen under `TASK-AMENDMENT-5/REMEDIATION_V28.md`; fan-in remains unchanged.

Remediation uses bounded exact-key scans, caps unit fallback at 2,048 entries
and 64 keys/object with marker-only detached evidence, and applies manifest
limits plus a canonical 256-byte ceiling before using fallback plugin IDs.
Twenty-one exact regressions pass. Complete suite: `274 passed in 0.67s`;
containment/diff PASS; V29 fresh full amended-diff review required.

V29 verified the V28 fixes but found two residual fallback gaps: raw adapter
capabilities could be scanned without a width bound after snapshot failure, and
over-limit adapter/unit strings could re-enter diagnostics. Exact remediation
is frozen in `TASK-AMENDMENT-5/REMEDIATION_V29.md`; fan-in remains unchanged.

Remediation bounds raw capability fallback and applies exact canonical 256-byte
limits to adapter and unit diagnostic references, otherwise using deterministic
fallback identifiers. Nine adversarial regressions pass. Complete suite: `283
passed in 0.78s`; containment/diff PASS; V30 review required.

V30 found raw invalid-path segments could enter diagnostic references and
caller plugin-schema normalization lacked full bounded exception containment.
Exact remediation is frozen in `TASK-AMENDMENT-5/REMEDIATION_V30.md`; fan-in
remains unchanged.

Remediation canonicalizes every preflight diagnostic path and normalizes caller
plugin schemas with bounded exact-JSON containment before hashing/evaluation.
Twenty-four focused regressions pass; complete suite `306 passed in 0.77s`;
containment/diff PASS; V31 fresh full amended-diff review required.

V31 matched every one of the 20 frozen hashes/line counts, reviewed the full
8,082-line frozen set and original-basis amended product/test diff, and returned
`PASS` with zero findings. Amendment 5 is complete and valid for fan-in.
