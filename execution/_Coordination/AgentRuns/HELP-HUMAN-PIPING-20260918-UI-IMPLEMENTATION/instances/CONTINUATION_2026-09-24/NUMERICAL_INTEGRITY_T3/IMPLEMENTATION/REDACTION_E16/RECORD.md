# Erratum E-16: the junit host attribute redacted from RV113's SR-PY review evidence

**What.** 42 gzipped pytest junit files under `R/REVIEW_RV113/rvr_sr_py_01/evidence/` (committed on NUM before `d1d6517455`) carried the junit `hostname` attribute, with the machine's network name. ROOT's commit-time screen looked for the machine's other host-name form only, so it missed them. None of them reached main.

**Ruled by the owner (2026-10-08): redact in place on NUM**, as E-10's redaction was. The original bytes stay only in NUM's history, which is never merged into main.

**How.**
- The auto-mode safety check refused ROOT's in-place rewrite of the reviewer's files and their sums.
- The owner ran ROOT's script in RV113's folder instead. For each affected file, it:
  - removes the single ` hostname="…"` attribute from the decompressed XML;
  - recompresses at level 9 with mtime 0;
  - replaces that file's line in `SHA256SUMS`.
- The script checked every file before writing any.

**ROOT's verification** against the committed originals at NUM `c8e54918cd`:
- 43 files changed: the 42 `.gz` files and `SHA256SUMS`.
- Each redacted file decompresses to its original with exactly that attribute removed, and nothing else differs.
- `SHA256SUMS` changed in exactly 42 lines, each to the redacted file's sha256. It is 245 of 245 OK on disk.
- `REVIEW.md` is unchanged (`d8611e59…`).
- No `.gz` file under `R/REVIEW_RV113/` still carries a host name or a `hostname` attribute.

**The screen** gains the machine's network name and any `MacBook` form, case-insensitive, beside the earlier host-name form. Every new brief carries it (RR "RV118 confirms B2-C revision 01; …").

**The redacted originals' list gains these 42 files,** at their pre-redaction commit. The next records PR's reviewer checks that none of the original bytes is present.

**Main's earlier records.** Main already carries this machine's network name in 11 T3 records from 2026-10-03, and one home-relative path in RV76's REVIEW.md. They are outside this erratum. The owner has been told, and any cleanup of main's tree is the owner's choice.

## The 42 files (path under `rvr_sr_py_01/`, old sha256, new sha256)

| File | Old | New |
|---|---|---|
| `evidence/mutants/runs/b1/NONE/pytest.xml.gz` | `6fa11a5b213d72c61661684bf9a7338ff87e1a6eaa8657c1b1e2bd2cd9e1fedb` | `a643786f5339dea1f51fdfc61ae8ddcaed459c58e99ab0da95e4ac9540a78ac6` |
| `evidence/mutants/runs/b1/Q01/pytest.xml.gz` | `9072d725cb4d7b3adaf0d1499437630e492c13441e3290be1986161b21c3cf11` | `501ed180794c7a12515b8cf645c4b71f6fab32b329e47bb2b6cc695ad935b993` |
| `evidence/mutants/runs/b1/Q02/pytest.xml.gz` | `e7dbed2bd7736f5f9e152237b2ade74d3d5488e1db2e61b2b965a6eb152b8c2b` | `ff063b0b9d6c90d0fb954742a00b8707975fa77e86d009b110f0a07e3b2a4f1f` |
| `evidence/mutants/runs/b1/Q03/pytest.xml.gz` | `0df0d7001ca3a4d2a08f5079a66892a37d5f598d07247571f349fbecc81dc1dc` | `96e73d3143e7e44d85c460046f3614913e57e9a31e271b476863330994c2dbc8` |
| `evidence/mutants/runs/b1/Q04/pytest.xml.gz` | `f207168092810433dfcc561b9a400a2276c4420482f8da0de5056b92dfcfeaaf` | `aad72219f046a88cdaff8987b6384182e80bdc23f86e1541990ab451172c769e` |
| `evidence/mutants/runs/b1/Q05/pytest.xml.gz` | `861193819c545a34b985c97d0a426855b53ff1080913b078ddce123a599fff76` | `3876323a056982392a94503b9d357774c9f86752804f8edc5bfaacd939749b1b` |
| `evidence/mutants/runs/b1/Q06/pytest.xml.gz` | `58eebdf737fc72f3d1190e397bda1fdcf924f5d4be8dac12cfef67d5940464eb` | `f26447bec21c419be12a0b5b1a2626bcc611bfd007cddee735c171741a664763` |
| `evidence/mutants/runs/b1/Q07/pytest.xml.gz` | `96d476f885017e916d52782af8f38547cb813990bfa4cc2a2d16d0f35f599a18` | `dc5377cc9f7a8ea72827afade3f9048b31e59a6ed9ba9b47ff363d53d317b712` |
| `evidence/mutants/runs/b1/Q08/pytest.xml.gz` | `c0a500fbd488ee999c8cc096d83d6ca2dfdc77404db1e8bc2763f93788194d96` | `344330b98241d47cdff1f59076bccdad205405f3e4eff1ebf128ded116278a7c` |
| `evidence/mutants/runs/b1/Q09/pytest.xml.gz` | `248a6f45f409b0c16c6e1d6c3ca44ba11d301380211a285f7926bfbd5355f288` | `457b485a1c17218c5d328c4bdbbe42bae976ef391870fcc315a94c574a992e3c` |
| `evidence/mutants/runs/b1/Q10/pytest.xml.gz` | `1a823a1ac2e7ec91c964279ee833aa060051d3d8369037443d4f183aa1e99807` | `7a291904870b9f9584a2d84abf056554600e8df00a6e99a99f55d13c60c817cb` |
| `evidence/mutants/runs/b1/Q11/pytest.xml.gz` | `5ca9ed97a6410ec9904636ae85215640c0e073be1d00564fb4a672455b981f74` | `24565d51dd7345861cb785def1b1c96ee58efbe1b6672d56bfa26577a8050e4d` |
| `evidence/mutants/runs/b1/Q12/pytest.xml.gz` | `6163d0cf59ff20530292a686de2f108c8970358ace827fdbea7742a9354f34fb` | `50138d7d3d21b52daacdaca4546c9b1d76919292d466fad6d916f1a84057cd2e` |
| `evidence/mutants/runs/b1/Q13/pytest.xml.gz` | `9df8966206f283dcba39b38525841c60c342fbb922f10295af6e54ff892b70cd` | `2fee3e5fd3e6a07358594d3c15136a875cc032353d251ab0549458d302724e8b` |
| `evidence/mutants/runs/b1/Q14/pytest.xml.gz` | `078d1fc7a55f042b70942639681c761701be39f4bb4dd079826b2554c5401f63` | `b5c709013ffac62a0473bb21c992c20c7a43365b326c9aa4cfe2625d5e42df7a` |
| `evidence/mutants/runs/b1/Q15/pytest.xml.gz` | `dcdfc521166ea06d53c113bcb8561e5df8e8e44d22ff47fec09a48579372b10f` | `c04298dd71cd42ed9977b25587078d8f9d9f09639df3ec4594705173b7bd6985` |
| `evidence/mutants/runs/b1/Q16/pytest.xml.gz` | `8ca33fdb59cfa876d19adb355626fd6a3eaff601ec95bd7a68ba9e17c2796744` | `79a38afd803d157730c4a71c9dea40cf1ba4f53a70c6f0e374c4eeb271422451` |
| `evidence/mutants/runs/b1/Q17/pytest.xml.gz` | `22b18c687d5b6a84356cc66ced614b27ed7177d87afac2616c6804ebdaf472fc` | `8a86585ab34d85f607f570e34b4b648bc6705163bf033b216ce6e66fe8916b0f` |
| `evidence/mutants/runs/b1/Q18/pytest.xml.gz` | `f82bbc435f751414147430654a82ef258840c3e727218aa020a8abf33d1ded88` | `d559633eafe10fe84d1795de5c4e0907d2ccb746b983c0898e23eeedfe2120f3` |
| `evidence/mutants/runs/b1/Q19/pytest.xml.gz` | `1474e0633ed94850babf6a82db6f4fc039561408adaca1b91017f9d98b87515d` | `19688f0a7b3eb5c6972e12c862e1633a499eee4d2ce14c4c16fd5327e42ad8b5` |
| `evidence/mutants/runs/b1/Q20/pytest.xml.gz` | `f0c4d28850a22c115276404820ea83b726ab0189bfe45ad903115a032da74c0a` | `c41d5eadc083b1c75034568f1ec9f518e021d5b6061af0639b44d4b26f6d06f2` |
| `evidence/mutants/runs/b1/Q21/pytest.xml.gz` | `609cdd747c7f6ea2f0e1fe3594c9933d333908696169e40676064ea7dbc2d4f0` | `14cdc11fc46e76707deffdb9c6bfc4e28f2894c7017226a186292e8ecddda5a3` |
| `evidence/mutants/runs/b1/Q22/pytest.xml.gz` | `2b58a66339c1d504e3e5c081d50deb45efc05b7b4d064dff02073025a5687641` | `ce17a5dcd80016b71d9278379ea10e9bf85877ef46b0cb4c129406fa0d5dd17d` |
| `evidence/mutants/runs/b1/Q23/pytest.xml.gz` | `3275a2321534c98d0fb3b4c8125c74292ff0e72ddd9d0aac66262c576bb19d86` | `d61a941e4a4f11e208c99e58cbe19721713eb5ce9e975b4185b1929960c9ef31` |
| `evidence/mutants/runs/b1/Q24/pytest.xml.gz` | `5b775d4e251856c9047592d58405891064f21cd436100c90c8852dd5e7e624c7` | `b2e7430032a4728fdc89a1fa9627d85027f954e08f3757c77554a698ae0f1dd4` |
| `evidence/mutants/runs/b1/Q25/pytest.xml.gz` | `81a7878725d56a723635026490703eba6731c4ac61b013383d5b1da7e6a6f994` | `b83bf0f966ae80418d9455e2c37d187c92e837fb052d0cc32ae2889cbc514205` |
| `evidence/mutants/runs/b1/Q26/pytest.xml.gz` | `61be03ab9a919145a0ee2587405f067558b5dbb8b3816d3f5fafd24b6f63657d` | `ccd6e9533184018ff965083aef168a77b4cd0e1d60117689a2ac42d2f5103161` |
| `evidence/mutants/runs/b1/Q27/pytest.xml.gz` | `1db64df8064727b7a27f869806a264ac3272e66ca32ccd2159b3b43d7ce6bbaf` | `a8a495266b3ad72238cc40c926693e48637465c21a160d68c84f875870b5502e` |
| `evidence/mutants/runs/b1/Q28/pytest.xml.gz` | `9715bb120d234738d73803e18d9a8d46202f3a1ac4d8c9163c618ba67fd09446` | `d242325547687c8703915ab071a5758ce4938adf059ea6704f205f8d8008aa7e` |
| `evidence/mutants/runs/b1/Q29/pytest.xml.gz` | `97831189127a51dc895dc6547b60b464f43a8a2a84f91aa898f78ecaaa6a9205` | `89a696e290cb3986b3fe0c3ed073e0a8e3a704c69571aefbd8a1f43b132c4ecc` |
| `evidence/mutants/runs/b1/Q30/pytest.xml.gz` | `0e66951639004fd0da8ba3374d87f345070214d4d260d5d8b7b56cd98b3584c1` | `ee43c934295c20cd4b96367304344cead9234b91dca8c474e31dad406d604fa4` |
| `evidence/mutants/runs/b1/Q31/pytest.xml.gz` | `a3808c71c7c8c3e1c020ecd1587842024c2d590f7cc5e61397b13627508a9d47` | `926c257928b666ba275b3edd8c15dc36deba06f09eb56a9f5d2be61d7705bf89` |
| `evidence/mutants/runs/b1/Q32/pytest.xml.gz` | `e6bb838fe3666e298d960ecd22324b3160b4be80c87f28d580996875189a9303` | `c128d0ab8984820fdc381750fdb86b16bfe168cf54e628c94955e8959181d9c2` |
| `evidence/mutants/runs/b1/Q33/pytest.xml.gz` | `011f36dc45df839643ce68ba67318d0165a8b1726fe7fd8c8bac23c73a11b816` | `19f91bf353cd4f018b10d9129583f1e14f594bd0b5012d03b4ff5fc8c5178db8` |
| `evidence/mutants/runs/b1/Q34/pytest.xml.gz` | `38fdbc682b3d3a76d7003e83eecdaaa21b7d2e13741845937800c5fd5c79f48c` | `efa6a5846de33093891a689b7f34a5abaaf763a084fa90e7c3f4ad3516083899` |
| `evidence/mutants/runs/b1/Q35/pytest.xml.gz` | `648f273b9ed37d10bf3d258de494139e9b501fc03dd14963bf8da425acf50d56` | `5cc3afcd3d1e01cc689025c0e115f3d15f67ce79169e177bccc276b8327cbab6` |
| `evidence/mutants/runs/full/NONE/pytest.xml.gz` | `267f3fcbd3047a2a6360581c6a18fc4d3cdbaa8699eb2942f2addc7fd402e646` | `af44c10dd4b6ac4a7bf7657e208d3d40c2f0c0f4c7d3259157e2ec724c64d059` |
| `evidence/mutants/runs/full/Q17/pytest.xml.gz` | `cc0d4426788556d68de3b1813a19a41b8dad45e89df15c85911613087659d780` | `2c90c16d66937e349e17848a132870db2f0d322ca5cacc7daf9dad9dd277bc15` |
| `evidence/suites/suite_head.xml.gz` | `f2180e5c894c92cec5adc1e6b1448493ea4c0c3cc8764335518b56fd7d4c85e8` | `c34e9441e1ce720c5c37a61218884e0cbaebac2b879296e35dc18bf53854e615` |
| `evidence/suites/suite_head_handoff.xml.gz` | `3c49b5dab66cc6fafdc2ebd8e38f7be8c9dd3c517fda7f71cfe5bdaa9b52a85a` | `28a55592ecc6672876d43ce3778e9be8abb8f4110da2e832cb504cbfece52f1a` |
| `evidence/suites/suite_i1.xml.gz` | `1ca43b2b8598b9fa7e7c410884e1d33060f99dc0c8b8e6157206331c8bb3d57e` | `b737ea32ff0c45655af49a963b1fc2737064f16a6d138b07798e2a7a9637530c` |
| `evidence/suites/suite_i1_handoff.xml.gz` | `6e2926d60358c7c1a68ccd11511d518248589bdf5b35ea51b62b613acf6fb425` | `59a32e744a53b6e833d525b831f5c6fc024376630494c6c799adf383080559ab` |
