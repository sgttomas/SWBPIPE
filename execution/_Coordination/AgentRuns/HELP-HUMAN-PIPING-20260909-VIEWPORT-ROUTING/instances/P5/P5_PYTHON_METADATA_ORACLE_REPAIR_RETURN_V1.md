# P5 Python metadata oracle repair return V1

Status: FROZEN FOR COMBINED TEST-ONLY REVIEW

The three DEC-025 G1 failures were stale assertions, not producer regressions. Endpoint mechanical stress now truthfully binds to the repaired section cut, while pressure membrane stress remains a distinct explicit-pressure category.

Only two Python test files changed. Their assertions now require the endpoint torsional row's local-stiffness basis and j-side, element-local, section-equilibrium semantics. Endpoint and station pressure-hoop rows require the open-mechanics stress-component basis, `pipe_section` coordinates, exact explicit-pressure convention, and absence of section-action wording. Complete station component/location coverage, schema validation, mechanical distributed-load correction checks, and invalid-basis rejection remain active.

Validation with the qualified Python environment:

- Three affected node IDs: 3/3 PASS.
- Complete two-module run: 13/13 PASS.
- Scoped `git diff --check`: PASS.

Frozen file hashes:

- `tests/product_preview/test_product_preview_service.py`: `451924c6c2f9aa1a47b75f822459e35d1ea8740c6b77bbb73e4086ff77f19032`
- `tests/test_results_schema.py`: `4af1dce79e20c2c68d4f848d961ff735c7ef6bf048faf9fb76b97099d454aacb`

Product physics and the generated fixture remain unchanged at `dbfff6d5c2fa9241e972042fd2c3a6ce35c6d60f4a973dd5a083a7fe5dfb9c58` and `fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c`. The 196-error full-envelope public-contract limitation remains open and unchanged. No production, fixture, schema, numeric, tolerance, Git, or lifecycle effect occurred.
