# RF V4 complete-candidate review brief

Status: `SEALED_COMPLETE_SUCCESSOR_FOR_ROOT_DISPATCH`

Target `/root/friction_code_review`, independent read-only Agent 2, `gpt-5.6-sol` high, no delegation. Review only the held V4 candidate; write outputs under `{RUN_ROOT}/instances/RF/post_sweep_g0/successor_v4/**`. Do not edit source, tests, fixtures, prior evidence, or lifecycle state; do not run tests, builds, or Git acts.

The accepted V3 narrow Rust result is bound by RF REVIEW `d85c41cb75caebd3d0a61c2da3c674a8fd7e7ad1082ba1d067e5f7c904562274`, RETURN `571778794f806ad54ae039b28256fd4886880aff565c3fc83f4e0ab8bef5cc15`, MANIFEST `a3c35750ea3890aa07b7163e12d891a236a106a478a5802875147b63e336e497`, and STATUS `558a987a8286c68d43d0932efcdecb2c7f605b1e18d9bfe084fdd5b55bf687db`.

Review these V4 records:

- amendment `7682ebba8f7dc5181c05d8ec1197f038944e68f04c7a7b54a40b788e54a7bf8f`
- exact-Fraction derivation `81b0e9a9a616d65c5fbb51a8cb8949198a64bacd13af368b23a4fbafe3455cdd`
- exact-Fraction output `4863cb9b4c0f8dda61508b54455e68d7f927fe69494f14e7d0cf5f381c470bf9`
- encoded patch envelope `5103bf895c4792989d4180a2667ec06adefe45c0247abdfe13f0a95df5c49928`
- decoded patch `c31a705a39c4629aacf6a79871c1497a8b8d3976e389478f79e41f4f7752c334`
- verification program `37afc53c5117b31b8981d17b31549facbeaa84f449cbd76aacb17ce753d816cd`
- verification result `a291da477ba802a63c8617dc63ccf7b9a7d79f3ec9ceae6cd4e836d2d44a1116`

Confirm the patch applies in memory to exactly six paths and produces the six declared post-image hashes. Confirm the first four post-images are byte-identical to V3; the only V4 source-candidate additions are one `4.927109` to `4.927112` and two `3.977299` to `3.977301` replacements in `App.test.tsx`, plus one `4.927109` to `4.927112` replacement in `r2-smoke.spec.ts`. Confirm the exact-Fraction derivation admits historical-no-spring L-200 normal publication cells `{24.476359, 24.476360}`, records the observed target as `24.476359`, and retains unique friction `0.244764`. Reuse RF V3 acceptance of the Rust correction and the accepted V2 inventory/parity evidence.

The candidate changes a runtime fallback fixture and therefore requires a new bounded Owner act before application. Its eventual acceptance gates must include focused assertions, the applicable crate and frontend checks, a fresh bundle/runtime fallback check for the new fixture identity, and the clean full DEC-025 sweep. Prior native evidence covers only the unchanged successful-backend path. Return `PASS` or `CHANGES_REQUIRED`; do not grant authority or apply the candidate.
