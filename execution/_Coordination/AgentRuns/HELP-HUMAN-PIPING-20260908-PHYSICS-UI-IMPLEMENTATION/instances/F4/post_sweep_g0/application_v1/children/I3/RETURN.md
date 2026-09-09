# F4-I3 application return

Status: `BLOCKED` pending manager validation. The exact reviewed six-path patch was applied once and all six post-images match the sealed values, but the prescribed focused browser validation failed in both configured viewport projects. No acceptance claim is made.

## Exact application

- Decoded/applied scoped diff SHA-256: `c31a705a39c4629aacf6a79871c1497a8b8d3976e389478f79e41f4f7752c334`.
- Exact application proof: reverse reconstruction from each live post-image reproduces all six sealed preimage hashes; `APPLIED_PATCH_BINDING.json` SHA-256 `085154f02663b3bda2330aaf2143190d2fedcfa875013daaf676b7c9d4981f9d`.
- Child changed-path proof: the decoded patch contains exactly the six sealed path headers and was the sole repository mutation command. All other child writes are authorized evidence; the private Cargo target was removed.

| Applied path | Post-image SHA-256 |
| --- | --- |
| `projects/chirality-piping/core/product_physics/src/lib.rs` | `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5` |
| `projects/chirality-piping/fixtures/product_preview/invented_mechanics_result.json` | `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13` |
| `projects/chirality-piping/tests/product_preview/test_product_preview_service.py` | `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735` |
| `projects/chirality-piping/apps/desktop/src/services/previewService.test.ts` | `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f` |
| `projects/chirality-piping/apps/desktop/src/App.test.tsx` | `5fa1e1e0439690db4692033eda0ae66c98c79fb050ce2cd1214fb74c417d855c` |
| `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` | `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17` |

## Validation

- Exact Rust witness: PASS, 1 passed / 0 failed.
- Complete `open_pipe_stress_product_physics` crate: PASS, 138 passed / 0 failed; doc-tests 0.
- Focused Python preview service: PASS, 9 passed / 0 failed.
- Focused desktop preview service: PASS, 16 passed / 0 failed.
- Named App test for queued editor intents: PASS, 1 passed / 161 skipped.
- Named App test for computed mechanics diagnostics: PASS, 1 passed / 161 skipped.
- Registered generator producer parity: PASS. Disposable output SHA-256 `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`; byte equality with the live fixture passed.
- Focused browser test: FAIL, 2 failed / 0 passed. Both `chromium-desktop` and `chromium-compact` timed out after 10,000 ms at `r2-smoke.spec.ts:552` because `queue-explicit-node-intent` remained disabled. The failure was preserved without retry, assertion changes, or weakening.

Browser log: `{DEL_RUN}/post_sweep_g0/application_v1/child_I3/VALIDATE_BROWSER_SMOKE.txt`. Error contexts and traces are under `{DEL_RUN}/post_sweep_g0/application_v1/child_I3/E2E_OUTPUT/`; the evidence manifest binds their exact paths and hashes.

## Frozen evidence

- Complete 17-entry source/test/fixture/generator binding: `COMPLETE_BINDING.json` SHA-256 `032d50ea69013be10f87257d8e30c9fc676704a0abfa77a4aabe7f1856555e63`.
- Aggregate binding serialization SHA-256: `2eb48c1af8d0d6d13e64204fb0fefd70da6ed3c89493ee206d3ae2fb97c5af8e`.
- Unchanged nonlinear solver producer SHA-256: `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46`.
- Fixture generator source SHA-256: `6cb68ea97330ee541d80318a5bcd69469957bfd39edcbf71d4313faae85e8241`.
- Generator registration `projects/chirality-piping/package.json` SHA-256: `7e719791e3ffdc7b57eddb2bb32d682705bf945af5b7207ffc699a2a45648656`.
- Validation summary SHA-256: `fbb95a0b85a924af566e734c01b4c3615cf8cbca238a6589790ac3b7cd7c1523`.
- Lossless text evidence bundle SHA-256: `ee122428143ee4de4935716ec8da72c8c98a931a565ae0abd6f19ed9f73696aa`.
- Evidence manifest SHA-256: `9a56cd2a9d1d447565ec6ac5bf90bb7676f0b459c49327468db089c45b91a82f`; aggregate member serialization SHA-256 `545fd7685d3fc0365235f2a4d278579dc542711f190602e28aa0d388d5ae5753`.

Residual risks: the browser acceptance surface is failing, the prior native bundle does not cover the changed runtime fixture, and the deliberately excluded full DEC-025 sweep, practitioner suite, and native build have not been run. The applied candidate must not advance to acceptance until the browser failure is resolved through a new authorized step and fresh review.

Exclusive six-path application ownership is released with this terminal return.
