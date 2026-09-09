# RF staged post-G0 applied backcheck

Stable-five verdict: `PASS`

Whole-tranche verdict: `PENDING`

No actionable finding was found in the five stable applied paths or their frozen non-browser validation evidence.

The following live files match their approved V4 postimages exactly:

- `core/product_physics/src/lib.rs`: `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5`
- `fixtures/product_preview/invented_mechanics_result.json`: `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`
- `tests/product_preview/test_product_preview_service.py`: `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735`
- `apps/desktop/src/services/previewService.test.ts`: `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f`
- `apps/desktop/src/App.test.tsx`: `5fa1e1e0439690db4692033eda0ae66c98c79fb050ce2cd1214fb74c417d855c`

The frozen 17-entry binding `032d50ea69013be10f87257d8e30c9fc676704a0abfa77a4aabe7f1856555e63` and aggregate `2eb48c1af8d0d6d13e64204fb0fefd70da6ed3c89493ee206d3ae2fb97c5af8e` serialize consistently. Sixteen current members outside the mutable `r2-smoke.spec.ts` match their bound hashes and byte counts. All ten original RI source/test members remain bound: nine retain their original hashes, while `App.test.tsx` has the exact approved V4 postimage. All production logic and source portions remain unchanged. Independently reversing the V4 Rust test hunks reconstructs preimage `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903`, and the complete product-physics prefix before `#[cfg(test)]` is byte-identical.

The fixture generator source remains `6cb68ea97330ee541d80318a5bcd69469957bfd39edcbf71d4313faae85e8241`; package registration remains `7e719791e3ffdc7b57eddb2bb32d682705bf945af5b7207ffc699a2a45648656`. Frozen generated output is byte-identical to the live runtime fixture at `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`, preserving the actual resource serialization.

The frozen logs substantiate the reported non-browser results: Rust witness 1/1, full product crate 138/138 with zero doc-tests, Python 9/9, preview service 16/16, and two named App tests 1/1 each with 161 skipped. Generator execution exited zero and byte equality passed. These results are evidence for the stable applied state; no check was rerun in this backcheck.

The lossless archive `ee122428143ee4de4935716ec8da72c8c98a931a565ae0abd6f19ed9f73696aa` has 17 self-consistent members matching the frozen child evidence manifest. The three publication-corrected pointer paths match manifest `7b482b068f2416fe4adb4ae959745b4b40a939c5c03da2e400261935c4e3e5d0`; each archived original hash and active successor hash verifies, and each pointer preserves the failure, line 552, disabled-control, and no-retry facts.

The original browser result remains `FAIL 0/2`. The separately attempted RU V2 application is frozen by manifest `9dd2fd40570c534c8bfdbb83ad1e476634d3e61050f015dd58e0544a419637ec` and remains `FAIL 0/4`: both main journeys passed the changed numeric fixture assertion before failing at the later queue/review boundary, and both from-blank journeys failed at their later review-state expectation. Its historical postimage is `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248`; its 17-member binding `94b19000230b24001d98101916c55e9e891634cf1d90e927bf6309881f05530b` changes only that test member from application V1, with aggregate `847866e6be422966e285968bd280b1bfe5415d084419f263b64936c977a93801`. No failure is converted into a pass. Current `r2-smoke.spec.ts` was not compared to either historical binding. Its final browser repair, updated 17-member binding, whole-tranche diff, native work, full DEC-025, practitioner checks, Git acts, lifecycle, release, and acceptance are outside this staged verdict.

Bindings: F4 application manifest `473d3fb987d0aaf0c776b9c34f0534afb264e406410bd8e0824b429d4c6c2b8b`; manager return `1f3124c4fafe9d90a4d1c01c5e01e54cf4d1c02d28385ecb337d22f38da27c45`; manager validation `e4e784552221baad8c193dad1c26c91f810bc924897f8f460ed0d38148250908`; applied patch binding `085154f02663b3bda2330aaf2143190d2fedcfa875013daaf676b7c9d4981f9d`; complete binding `032d50ea69013be10f87257d8e30c9fc676704a0abfa77a4aabe7f1856555e63`; original browser validation summary `fbb95a0b85a924af566e734c01b4c3615cf8cbca238a6589790ac3b7cd7c1523`; successor application manifest `9dd2fd40570c534c8bfdbb83ad1e476634d3e61050f015dd58e0544a419637ec`; successor manager return `0ea81314242f04867a54c5b7b56c578241534db9ee916fc235f97d1207a92963`; successor manager status `50807226ad180a0c851e2fadcb21d6ffbff2b81c822782492bc5ed4326fb3ac3`; successor `0/4` manager validation `60a9469fe07070c25451549b9e53dfdabef061a9549486f5bdbce47c26479c24`.
