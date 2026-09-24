# Independent M24 integration review

**Suitable for manager fan-in. No unresolved actionable findings.** This return covers the exact source union at `beaf4869bee2ae8c866e7e62eaa6a8f369610d7e`, the added maintained authored-unit integration test and its Cargo dependency/lock delta, the frozen 24-record portability package, and the specifically added unactivated numerical preparation artifacts. Source identities and instruction origins are in [ORIGINS.json](ORIGINS.json); reproducible mechanical checks are in [audit.py](audit.py) and [STATIC_AUDIT.json](STATIC_AUDIT.json).

TASK `/root/solver_manager/m24_review` is a delegated-harness-native fresh reviewer returning to WORKING_ITEMS `/root/solver_manager`. The reviewer authored no production code or tests and launched no descendants. Parent-supplied model configuration is gpt-6-astra xhigh; this is not an independently introspected model or an OS/model-diversity claim. Only this review evidence directory was intentionally written. No Cargo, npm, product solver, browser or native application workload was executed by the reviewer.

## Source integration and contract trace

The nine incoming authoring source/test/document hashes match `AUTHORING_MANAGER/EXPANDED_CANDIDATE.json` and original selected commit `ff2cb76ddfda43f6bcb4202b80b7fd199eb3c44e`. Both product source hashes match solver commit `c90089c3279ee1213501835eaef08938ed800897` and its independent repaired-candidate backcheck. Existing independent reviews of the complete source tranches remain applicable; this review traces their connected interfaces and the new frozen delta rather than claiming a new broad UI review. Scope validation covers the union and pending owned files, including the untracked maintained test; [SCOPE_CHECK.json](SCOPE_CHECK.json) passes. The selected diff whitespace check passes.

The maintained test calls the real `apply_operation` with a structured primitive-create intent. The creation gate validates the entered unit against its explicitly required dimension even when that token equals the project preference. The catalog accepts legacy `C` only under temperature/temperature-interval semantics. Operation application retains the input model and original stored unit/provenance text. JSON serialization/deserialization preserves that authored carrier before DTO construction.

The request construction is the same `PreviewModel` deserialize plus `LinearStaticPreviewRequest { model, materials: vec![] }` used by native `solve_preview_mechanics_with_mode`. Product validation requires nonblank local source references without requiring public-clearance vocabulary. Normalization uses the shared dimension-aware unit catalog: temperature intervals normalize without an absolute offset; GPa moduli normalize as stress; alpha normalizes as inverse temperature. The product then assembles the thermal action from alpha, temperature increment, E and area. Nothing in the incoming unit work changes support/contact classification, typed moment axes, hydrotest containment, protected numerical policy or pressure theory.

The three positive cases exercise canonical degC, unchanged persisted C, and 180 degF with GPa moduli in both sparse and dense modes. Each requires an applied model and solved mechanics, a unique tip x-displacement row in mm equal to 7.2 within its stated comparison precision, and zero anchor resultant in N within its stated numerical allowance. The two negative cases reject an unknown matching project token and a length unit masquerading as thermal input before an applied model exists. Their otherwise identical positive fixture and direct gate trace support the intended rejection reason.

The GPa case establishes that the engineering token reaches a supported product solve. Free thermal extension is independent of E and does not establish modulus-sensitive numerical conversion by itself; the imported independent numeric catalog tests supply that narrower scale evidence. JSON carrier equality is not native file/history/result persistence or canonical-hash evidence. Those limits are stated by the test/reference and remain outstanding programme witnesses.

## Independent thermal reference

For a uniform prismatic axial member with one anchored end and the other traction-free, axial equilibrium gives zero axial force and stress. The small-strain relation then gives epsilon = alpha times Delta T. Integrating constant strain over 6 m gives 0.000012/K times 100 K times 6 m = 0.0072 m = 7.2 mm, positive along root-to-tip x; anchor force is zero. The section and E influence the intermediate thermal equivalent force/stiffness but cancel in this free-extension response.

This derivation agrees with the thermal/mechanical strain split and free-expansion discussion in [MIT 16.20 Unit 9, pages 2–5](https://ocw.mit.edu/courses/16-20-structural-mechanics-fall-2002/9116903dab61fcd685f268bd3c97f733_unit9.pdf). [NIST temperature guidance](https://www.nist.gov/pml/owm/si-units-temperature) independently fixes equal Celsius/Kelvin intervals and 1.8 Fahrenheit degrees per Celsius interval. Both primary sources were retrieved during review; no prohibited OCR equation artifact was used. Applicability is uniform small-strain axial expansion with constant synthetic properties, not a material-grade, temperature-gradient, pressure, buckling or engineering-acceptance claim. A transverse shear model does not alter this zero-shear axial identity.

## Actual execution and dependency registration

Inspected parent-run `M24_INTEGRATION/RUN_BEFORE.json`, `RUN_AFTER.json`, exact execution metadata and raw log. All seven post-run source hashes match the reviewed working candidate. The raw log hash matches and contains exactly five passing named tests, zero failures, exit 0. All positive tests explicitly loop through both solver modes. These are manager-run results, not reviewer executions. Earlier source unit/applier/product/frontend evidence remains attributed to its original runs.

The initial review identified the missing lockfile registration; the authorized offline Cargo run resolved it. The final lock adds the operation-applier edge and 13 packages in its transitive dependency closure. Every existing package block remains byte-identical except the benchmark's explicit new dependency edge; no existing version/checksum changed. The preserved `Cargo.lock.before` matches the union Git source. The ordinary integration-test layout is automatically registered by Cargo, and the existing evidence-sweep manifest discovery includes this benchmark crate. No new dependency installation or network execution is inferred from these records.

## Portability custody backcheck

The frozen [custody manifest](../PORTABILITY/CUSTODY.json) has SHA-256 `caad74e0b1f9eb0958210a61ef1965042b7aa073be6f8fb837116381c8e038e0`. Independent checks established:

- All 24 original archives and current views match their declared hashes; all 23 committed originals match their named Git source bytes. The one untracked preparation capture is accurately distinguished.
- Every JSON value/type, list entry and dictionary field is retained after only the declared machine-path substitution and added `_portability` metadata. All replacement tokens have consistent original values and their declared raw JSON-pointer locations account for every substitution. Existing empirical values and hashes remain unchanged.
- The Markdown backcheck body is unchanged except the two declared snapshot-path substitutions; its custody addendum identifies the exact raw original. All raw-record references resolve and all raw archives classify as EVIDENCE under the unchanged classifier.
- Twenty-eight historical logs, patches, RETURN and REVIEW records remain byte-identical to the union commit. No classifier, harness policy or protected expectation was changed.
- The bounded scan of solver/imported-authoring records found zero active machine-path hits outside recognized EVIDENCE. This is not a broad GEN8/self-check/DEC-025 pass.

Historical hashes continue to identify original observations; the explicit original-to-raw mapping avoids silently treating a rewritten view as the original byte capture. The frozen manifest need not be rewritten to cover this later review evidence directory.

## Bounded numerical preparation addition

The three preparation files are complete and mutually linked. Their source-qualified `NUMERICAL_REFERENCE.md` resolves in the primary checkout and matches the recorded hash; the isolated manager worktree does not pretend that source came from its own Git snapshot. Artifact hashes match, and running the inspected standard-library-only Decimal script regenerates `prepared_expectations.json` byte-for-byte. No production module is imported.

Independent 85-digit Decimal evaluation using explicit annulus coefficients and a separate pi literal checks all 22 scalar values against the reference expressions; maximum relative difference is below 3e-60. N02's rigid torsional null mode, its removal by N03 RX and retention under RZ, N04's six disconnected rigid modes, and N07's eigenvalues 3 and -1 with negative quadratic form -2 agree with the recorded exact classifications. The soft-spring reciprocal-condition expression follows the two equilibrated eigenvalues; it is not a selected production threshold.

This is arithmetic/output-custody review of unactivated preparation. It does not accept a production numerical gate, certify forward accuracy, close the reference-owning loop's acceptance prerequisites, or expand the benchmark's free-thermal claim. The metadata accurately retains `policy_activation: false`.

## Remaining boundaries

No further repair is required for this bounded return. ROOT still owns native authored/edit/apply/solve/save-reopen/hash/history witnesses, the clean integrated-candidate DEC-025 sweep, required actual-head CI, final independent coverage of later deltas, and repository integration. Existing protected numerical criteria and separate numerical/pressure decisions remain intact. This fan-in recommendation establishes neither engineering acceptance nor release or lifecycle acceptance.
