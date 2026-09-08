# RM independent review

Verdict: `PASS`.

Scope: fresh read-only review of the manifest-bound M9 candidate independent-reference packet at source `779dedb8670625b36af07b89fc5557470e47c50e`. This verdict covers correctness, reproducibility and claim calibration of the bounded derivative packet. It does not adopt an engineering tolerance, a physical fork, a dependency disposition, lifecycle state, or production repair.

## Findings

No actionable findings.

The disclosed native/Rust compile-slot event is a real orchestration deviation: M9 invoked the comparison before receiving F4's release notice. It is already stated consistently in `PRODUCTION_COMPARISON_EXECUTION.md`, `RUNTIME_EVENTS.jsonl`, `RUNTIME_SUMMARY.json`, `STATUS.json`, `RETURN.md` and `HANDOFF.md`. The comparison used an isolated temporary Cargo target recorded in the M9 structural execution record `PRODUCTION_COMPARISON_EXECUTION.md`; the harness path dependency resolves to this checkout's `projects/chirality-piping/core/product_physics`, the checkout remained at the declared source SHA, and no product-source path was dirty or written. The expected result and pre-normalization freeze manifest existed before the first production output. The event therefore does not compromise oracle independence, source identity or the retained output bytes. HELP_HUMAN retains governance disposition; this review does not retroactively grant the slot.

## Frozen inventory and lineage

- RM launch brief: SHA-256 `b3a33c63a705e44204b1848dcb4acd8e91c40fe1bb81aa116ea356deba3a68fb`.
- RM additive release: SHA-256 `3dcdf54a5a1471f4689f1c36e3ba892f9902d1bbaf663684d40d8a4978a5ee3e`.
- M9 manifest: SHA-256 `02131d46d094484d425825561ef0e6b1784740e6452a9d3f40b7ac9392e732b2`; 18/18 members rehashed; zero size/hash mismatches.
- DEL-09-01 manifest: SHA-256 `2eeaddfd63cf05d116b46e031844d1b1d9c028380687facf9808ce0aebc7062b`; 32/32 members rehashed; zero size/hash mismatches.
- Combined reviewed membership: 50 unique manifest rows. Both manifests have zero duplicate member paths and use the declared exclusions `MANIFEST.json` and `WRITER_FREEZE.json`.
- Fixture: SHA-256 `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`, matching the freeze and live input.
- Frozen manager expectation: SHA-256 `72a5db983ac43aebd881941161899a5e51191a417d0a86bf5215b40939787245`.

Every base64 archive strictly decoded. The decoded pre-normalization freeze-manifest hash is `26cc80861475e3004f892aa64d2f74cb4923ecd8c036d2b7b94333f0c8c76cd7`. The other decoded original hashes are `8fbb9a74ae9f58591a16a1d8c606a16af16f824b1c3d0f60ea0f7a663c6af358`, `63e627e2f45ede1cc4d221f375315be2cb3f5c0c144e080bfe6fc8a45f78743f`, `6849d256bee1f4ec26ef63928c37449b7f59c63f78194d2c18c6307d2818c2d2`, `2b6ba1e41d8189e3b50af5c23ca2f0a8ef71f37e21ef99bb01127df52958aaf8`, `4ce40b310d814929769eee58be1c0ba71ddf5f5f9621ca3545fcde9db8323288`, `c9c00623c7b2692d0a1edc12a983d405a4b344fc0e194fcaa131f19088e3e6b5`, `bbe59545fd28a2cc038dbecb572f3a2a85f7931e75079f472aec806ba049c6c8`, and `1ef244c9e255dae5fec4ea698df96c117bd796d461e392a3f8727ae87a47c4a7`. Direct decoded-to-active diffs agree with `PORTABILITY_NORMALIZATION.json`: blank EOF removal, Markdown trailing-space removal, symbolic host-path replacement, and normalized hash refresh only. The numerical expectation hash did not change.

Local filesystem chronology supports the stated ordering: the expected result was finalized at `2026-09-07T23:11:36-0600`, the original freeze manifest was born at `2026-09-07T23:15:34-0600`, and the first production output was born at `2026-09-07T23:16:56-0600`. The archived original freeze manifest binds the exact pre-normalization input, method, source basis, calculation, expected result and verifier hashes. The later active-file portability normalization is explicit and preserves those original bytes as base64.

## Independent calculation review

The manager implementation was rerun with the bundled NumPy interpreter into an isolated temporary file. The result was byte-identical to `EXPECTED_RESULTS_FROZEN.json`, including SHA-256 `72a5db983ac43aebd881941161899a5e51191a417d0a86bf5215b40939787245`. The retained verifier also returned PASS.

The frame stiffness, right-handed local-axis transformation, consistent line-load vector, thermal initial-strain vector, retained zero-reference ground spring, support elimination, element-end recovery and station-cut equations have consistent signs and units. Independent sums from the frozen signed values reproduce the physical-frame force residual within `2.61e-10 N` and moment residual within `1.98e-9 N*m`. All four physical/literal frictionless/mixed result sets satisfy element force balance, element moment balance and station endpoint identities at raw double-precision residual scale.

The independently computed physical mixed state is mechanically self-consistent under its stated branch: N-130 UZ `-0.0066439267973924755 m`; S-130 UY action `-52.37328198732987 N`; current normal `52.37328198732987 N`; friction action `+0.5237328198732987 N`; NL-140 UY action `-297.62671801267015 N`; SH-140 UZ action `+279.0085184271103 N`. Friction opposes negative UZ slip and equals `mu*N` from the same state. The active unilateral branch satisfies `g=-u=0`, `lambda=-R>0`, and complementarity; the open solution violates the explicitly stated block-positive-UY gap interpretation.

The manager/child derivations are distinct in structure and ownership. Across the stated 198 corresponding L-100 nodal, element-end and station components, the independently recomputed maximum manager/child difference is `3.7624658943968825e-10`. Current-normal difference is `7.815970093361102e-14 N`; friction-action difference is `7.771561172376096e-16 N`. Independence remains instruction/config asserted, as the packet says; it is not claimed as mechanism-proven.

## Product comparison and case calibration

The source fixture, selected pressure-free L-100 seed, connector-excluded physical controls and literal C-150 adapter diagnostics are separated without relabelling a transformed model as the unaltered full fixture. The physical case retains the four pipe members and SH-140/NL-140/NL-130-FRIC while excluding C-150. The literal case adds the exact authored six C-150 relative stiffness values in parallel with unchanged P-130 and is correctly limited to a current-adapter mapping diagnostic.

The eight production envelopes parse, report `MECHANICS_SOLVED`, contain unique result IDs, and bind every compared row to `load:L-100`. Every one of the 636 JSON comparison rows has an identical CSV row, a present production identity and a matching unit. Dense and sparse values match for all compared rows. Independent counts reproduce 157/157 exact-at-emitted-precision rows for each frictionless case, 89 exact and 72 differing rows for physical mixed, and 79 exact and 82 differing rows for literal mixed.

The direct same-return-state checks reproduce:

- physical: `0.524314 - 0.01*52.373217 = +0.0005818300000000054 N`;
- literal: `0.409403 - 0.01*40.90935 = +0.0003094999999999626 N`.

Six-decimal emission cannot explain either sign-positive residual: the maximum combined rounding uncertainty in `R_f - mu*N` is `0.000000505 N`. The packet correctly reports these as exact algebraic observations and does not turn them into an adopted engineering threshold.

The literal C-150 connector end forces independently sum to zero while their global moment about the origin is `[0, +658.4367607916422, 0] N*m`. The literal whole-frame moment residual independently recomputes to approximately the same `+658.436760790536 N*m`. Classification as D02 diagnostic evidence rather than a physical/full-fixture oracle is correct.

## Sources, units and retained forks

The cited CALFEM, Abaqus, OpenSees and COMSOL pages are official project/vendor documentation, and the Zimmerman/Ateshian item is the primary research article. Their stated uses are limited to formulation context, axis/load meaning and history-versus-static branch framing. The numerical oracle is derived in the packet. No `domains/piping-design` OCR/extracted equation is used.

The input freeze matches the fixture's five nodes, four members, annular section, E/G/alpha, restraints, spring stiffness, friction coefficient and three retained non-pressure L-100 primitives. Its unit boundary is explicit fixture-local SI: m, N, N/m, N*m, Pa, rad and temperature interval. It expressly does not satisfy canonical project conversion acceptance.

The packet retains C-150 topology/objectivity, SH-140 force-free/preload reference, friction history/committed slip/rollback, one-way allowed-displacement side, comparison thresholds and canonical unit/conversion acceptance. It does not adopt or discard those forks. The pressure-free control excludes pressure and leaves the pressure matrix to P5. `DAG-002-E0532`, `DAG-002-E0533` and `TP-DAG-004-DEL-09-01-E001` remain unchanged and unsatisfied by this derivative evidence.

## Review limits

Per the sealed brief, RM ran no Cargo command, native build, product test or full harness. The review used read-only source inspection, manifest/hash validation, JSON/CSV/syntax checks and isolated NumPy recomputation. No product, deliverable, governance, DAG, decision, lifecycle or Git state was changed.
