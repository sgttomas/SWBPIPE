# Erratum E-19: the owner's machine names redacted from 32 records on main

**What.** 31 files on main carried the owner's machine names: the Mac's network name, its host and computer names, and a second Mac's names. A 32nd arrived with #1117 after this branch was cut. The names sat in:
- run receipts, returns and reviews;
- git's auto-derived author identities;
- junit `hostname` attributes;
- an app server's `server_name` logs.

RV76's review also carried one home-directory path.

**The owner decided** (2026-10-08): "Clean main's current files", then "Cover all 31". This includes records of other projects: app-v4 (3), runtime (1), PEC (2), and a root `execution/PKG-02` evidence folder (10 `.gz` files).

**How.**
- **Who ran it:** the auto-mode safety check refuses ROOT's rewriting of sealed records, so **the owner ran ROOT's two scripts** (`WT/tools/t3_host_cleanup.py`, then `t3_host_cleanup_2.py`, outside the repository) in this branch's worktree.
- **What they read:** the names come from the machine at run time, plus a private list beside the scripts. Neither is in the repository, and nothing printed a name.
- **What changed:**
  - every name, in any split or escaped form, became `<host>`;
  - RV76's home directory became `<home>`;
  - in the five I91 junit files, the `hostname` attribute is removed (as E-16 did), so each still parses as XML. The first pass had written `hostname="<host>"`, which is not valid XML (the reviewer's B-1);
  - `.gz` files were decompressed, edited and recompressed (level 9, mtime 0);
  - each `SHA256SUMS` line listing a changed file took its new hash;
  - each portability-policy entry for a changed file took its new hash, or was removed where no machine-absolute path remains.
- **Checks before writing:** each script checked every file, sum line and policy entry first, and wrote nothing until all had passed.

**ROOT's verification:**
- **Re-applying the redactions** to each original on main reproduces every redacted file byte for byte.
- **Text edits:** 49 spans changed in all. That is 48 names, each a whole name (with hyphens, dots, spaces or a line break between its parts), plus RV76's home path. The #1117 return adds one name.
- **Junit:** every junit file parses as XML, and no `hostname` attribute remains.
- **Sums:** every changed `SHA256SUMS` file verifies.

## Hashes cited elsewhere that now name the original bytes

The originals stay in Git history; the table below maps each old hash to its new one. Records that cite an old hash were not re-sealed:

- **T3:**
  - the chains `MANIFEST.json` → `SEAL.json` → `verification/*/VERIFY.json`, and `HANDOFF_2026-10-03/READER_STATE.json`;
  - I61's to I64's `RETURN.md`;
  - `REVIEW_RV77/coverage_producer_01/REVIEW.md`;
  - `REVIEW_RV80/reader_review_01/REVIEW.md`;
  - `REVIEW_RV96/records_01/…/policy_new_entries.tsv`.
- **Root `execution/`:**
  - `PKG-02_…/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/ARTIFACT_HASHES.csv` (10 rows);
  - `_Coordination/AgentRuns/ROOT_RUNTIME_MIGRATION_GATE5_2026-09-06/AUDIT/root_measured.json`;
  - `_Evaluation/DecompCoverage/ROOT_RUNTIME_MIGRATION_BASELINE_2026-09-05/INPUT_HASHES.csv`;
  - `_Evaluation/DecompCoverage/SCA005_MIGRATION_POST/INPUT_HASHES.csv`.
- **App-v4:**
  - `APP-V4-DEFINITION-20260926/FINAL_INTEGRATION_CHECKS.json` and `FINAL_INTEGRATION_RECHECK.md`;
  - 12-character prefixes in `APP-V4-DESIGN-PASS-2-20260930/WAVE_B/RQ.md` and `reviews/V19b.md`.
- **Runtime:** `MCP_V2_COMPATIBILITY_20260920/README.md`.
- **PEC:**
  - `P1_STORE_GUARD_04/RUN.md`;
  - `HELP-HUMAN-PEC-20260923-SCA005/returns/L2A_REVIEW_DEL-01-03.md`;
  - `_DECISIONS/D-PEC-91_…md`.

No validator or test in CI checks these chains, so nothing fails. Record-local scripts would fail if rerun against the redacted bytes. **The app-v4, runtime, PEC and root loops are notified** by this record and by the PR that carries it.

## Out of scope, by decision or judgement

- **I90's sealed screen script** (`R/I90/b1_sr_rs_01/_run_records/repair_02/scripts/write_records_r2.sh:35`) keeps its pattern line as executed (owner's S-1 decision, RR "RV119 passes #1114 with S-1; …").
- **The second Mac's default host name,** a generic factory name that many Macs share on a home network, is not treated as private. It occurs in about 80 files: about 70 receipt `"host"` fields and `Darwin … 25` uname lines in piping records and `portability_policy.json`, and 3 git identities. One occurrence was redacted here, in a review's list of host names. It stays off the private-term list, because it would trip on unrelated text. One junit file outside run records carries it (`execution/_Evaluation/SOW-STAGE2-EXEC-20260712-01/C2F-R2/reproduction/checklist-targeted.xml`).
- **The machine-local paths** that other registered receipts preserve as provenance (owner-approved at #1084) are unchanged. Only RV76's single home path was in scope.
- **Exposure that remains:** Git history, NUM's pushed branch, earlier pull requests' commits, and about 1,000 commits on main whose author or committer email is git's automatic host-derived identity. This erratum cleans main's current tree only; the private-term check (PR #1118) stops new ones.

## SHA256SUMS files updated

- `R/I91/b1_sr_py_01/SHA256SUMS`
- `R/I91/b1_sr_py_01/SHA256SUMS.repair_01`
- `R/REVIEW_RV77/coverage_producer_01/SHA256SUMS`

## Portability-policy entries (`projects/chirality-piping/validation/portability_policy.json`)

- `R/I57/summary_coverage_01/ORIGINS.json`: hash updated
- `R/I58/python_shared_01/RECEIPT.json`: hash updated
- `R/I59/rust_reader_01/RECEIPT.json`: hash updated
- `R/I60/typescript_reader_01/RECEIPT.json`: hash updated
- `R/REVIEW_RV75/ordinary_bound_01/EXECUTION.json`: hash updated
- `R/REVIEW_RV75/ordinary_bound_01/RECEIPT.json`: hash updated
- `R/REVIEW_RV76/summary_coverage_01/REVIEW.md`: removed (no machine path left)

## The 32 files (old sha256 on main, final new sha256; `R/` is T3's `RESUME_2026-09-30/`)

| File | Old | New |
|---|---|---|
| `R/I57/summary_coverage_01/ORIGINS.json` | `f1fe2c8055fb9e8201dc3ff70b086da6108147451c2ddf9471e1075e068bd03e` | `214309ab3bf8d74a808bcc7154df9bb4906b2b6caf89c329b41a951892df5306` |
| `R/I58/python_shared_01/RECEIPT.json` | `a2d4d06e2b90d87a1d92e8b0a308fa96fdb69ccef0eb53ef1abc4177d40b0db1` | `f25196ead8053c9c987942f8f444d70ac2a803fabba82e02151ee1b4e4879d19` |
| `R/I58/python_shared_01/RETURN.md` | `7f418873e6b40c4532293d92e4ce3e10c626f73d23d460f03dce8eea07c6ccb4` | `63210ba065e24404471f109dc7c219b6e2735b0b0f6f57250da6a94ac1351b20` |
| `R/I59/rust_reader_01/RECEIPT.json` | `ad71b0f3221f675cd3408cf575f531257e84ed268c62dba377e41c5e23dc2a23` | `6e71a4506ec3816bfff268a93ce8c3be5a4de6faadc2b723294f3b2af00e1421` |
| `R/I59/rust_reader_01/RETURN.md` | `d77574b804d4cfc94f8dbcac6ea91f2408e4123a5244fd80a75df2a42b9302b5` | `669bba2fdb17ea006e3aee7bc1cf58bcb838f2efbb72a61a27c6532dbf1658c2` |
| `R/I60/typescript_reader_01/RECEIPT.json` | `a127b4e145c29e77ce710de941746c11cdc946ae00c61e20b1b07e97765071ac` | `ce06f45198c5a4587cb256376c500d3bb850d23cef53a20e3ecf36a8c6e625dc` |
| `R/I60/typescript_reader_01/RETURN.md` | `7eb2ae3ff9b752ac116298210517de238358f4ee21f692835874ca07a94c84f1` | `ec64f452fd32960783410efcd5e664bbca2dd425be81faca8cbcae6cfcc27e9e` |
| `R/REVIEW_RV75/ordinary_bound_01/EXECUTION.json` | `ea750aca15b2c9cbafaf63741a590547ecf2b9a1a66c7d9a06fa2d7343827176` | `569ebd85399a07c6bb1d0e4b4f57220d10eff3d5601b668fd54418a077906c1e` |
| `R/REVIEW_RV75/ordinary_bound_01/RECEIPT.json` | `f8b45326a2856f99aea6329e8697842c435b70185cea9a68b8c725ae0bc18051` | `8f8f14ca68da9678c636574f2ba5d7a86ea034ba2fef2ac490bb832281c6373e` |
| `R/REVIEW_RV76/summary_coverage_01/REVIEW.md` | `10341323b974d7a24155f23013d16b43e803f1d3cb6638df052a6ce2f51a61f3` | `ecb7026a4bd48d42dbd8bc4f5478b7ea9d7efcb63a097cb286505f6ea727cc98` |
| `R/REVIEW_RV77/coverage_producer_01/REVIEW.md` | `3d5f773ee8d3f67b83e7d8981a06acff438b6066747a380c706ea6c90cf173ac` | `72c7250e834d1e05df039246342a3b548694d0ff30f98384575a0d20f71e1697` |
| `R/I91/b1_sr_py_01/_run_records/repair_01/suites/py_r01.xml.gz` (attribute removed) | `c6b72ca70a969dc5e11e557e512219b162af073d68a273cb73f5c6de05cc4799` | `b50d95bddbc78522d9dacb34ab828b53cb605a1f41f307aaabe3b46f3c29b564` |
| `R/I91/b1_sr_py_01/_run_records/repair_01/suites/py_r01b.xml.gz` (attribute removed) | `e4a1d29a0154548b7a16a9e38a33c6de7f6606bad9ac586953e016289fdf31fe` | `bb579f097b27a2821753b552ab617b0348fe637143036e0418ce59d71dd96ae7` |
| `R/I91/b1_sr_py_01/_run_records/suites/py_base.xml.gz` (attribute removed) | `a6c9cc17ca70038c8839bae8f0bc8460dd09d1414f892a880fab59fef3efddd0` | `9ac08445e7bb2ac42d33720f41c679219e0deda33d21684213e3aca4b5a7aaf0` |
| `R/I91/b1_sr_py_01/_run_records/suites/py_head.xml.gz` (attribute removed) | `5efaf272be005aa17ea79c32f3519336a9e2b968c2bc93bdd15572c1c1ea8f93` | `bfbd786acbc82c5461ced363385a8367cb3dcd2adb7795fd23e034a2cf66b51d` |
| `R/I91/b1_sr_py_01/_run_records/suites/py_head2.xml.gz` (attribute removed) | `8284f2f7f30c1d79b85a110a09c3a08f91600dbdc40805635a07a90f82c6e0b9` | `7c724d7048fdc50494586fadd1897cb65c2246b1e54e14d58efc365fa7f88cc5` |
| `projects/chirality-runtime/execution/_Coordination/AgentRuns/MCP_V2_COMPATIBILITY_20260920/app-server-wire.json` | `146fa6e67e1d0045151c7e0e639181d61d1b7a9b78346fd40a9d66a1071d3c16` | `0ab707c8a1aceeee479e509e0fd5d7c1eb0152fe0a4459292ebd7606f00ab35f` |
| `projects/chirality-app-v4/execution/_Coordination/AgentRuns/APP-V4-DEFINITION-20260926/RUN_BRIEF.md` | `86d3980ec2364c2289a833e21da4b0718801c342621a60e0370cfd9adc7745fb` | `c9f3048b1fa6683ca1e42930d2856f526ecbc9875200302035221efdc9474932` |
| `projects/chirality-app-v4/execution/_Coordination/AgentRuns/APP-V4-DESIGN-PASS-2-20260930/reviews/V19-B.md` | `dd467919a8442ab0c4ed3c6e656d6299df9fcfbd557cfad6b811fe15fe247269` | `3f7be5774016f7a0a95f070e9575543edb67392bd0d7ff16a5117281eb29219e` |
| `projects/pec/execution/PKG-01_Service_Core_Store/1_Working/DEL-01-03_Store_bootstrap_content_minimal_guard/_run_records/P1_STORE_GUARD_03/RUN.md` | `078914b85147417175fc8e701f80fd4799d48131e13e932d9e7bacbe38bc1b17` | `dc6f1cb8c599cab50545f9f8282d0df373e2a6cf342335406e993d4f2f15d2d0` |
| `projects/pec/execution/PKG-01_Service_Core_Store/1_Working/DEL-01-03_Store_bootstrap_content_minimal_guard/_run_records/P1_STORE_GUARD_04/RUN.md` | `d359e1f2d90fa9c627653d30e1ae0f50d6e7a7c068dfd06b401fd400f5c48a5f` | `e0978b7b482deb6c2322d129682cb87697d6f17f18208c155e5c3f737fead4d1` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/experimental.stderr.jsonl.gz` | `f106681bc53a5a55a73f1213c999f18e4c83f496771bc499feb058c3f3a985c4` | `fe61a2430d5e751fe4f150f83d9e488fc8da6b18800d6dfd0bcd9dc88736e418` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/experimental.stdout.jsonl.gz` | `5039a04c28c81183401787a68b13357c7f9e66e5cc7033447af8c58856aa4390` | `0fed4b8f019ac49b1520fcb1f0aa75a5a33016bc29347ad833d01c8a8d035e69` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/feature-page2.stderr.jsonl.gz` | `084f7ccab1b7588cd24a17a726c7eda401fe3f73041e040cd92064f7bc9252e1` | `1054b1675050c9828a578d9c504cc308147a9583c12a191ac77c3841f6d5cdaa` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/feature-page2.stdout.jsonl.gz` | `4c83835876097b7c3250194b4037a14d78a490161a8f91bd31a08561d20c28c7` | `94566a3ed276bd1fe8aad6b9c9239b5a3f94d0cbe89e4fe3d9dbaccce8db34c3` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/plugins-off.stderr.jsonl.gz` | `51d2dd073b0190057b72a17507334414dda92edf6651ac65ce809bb89da3fd0c` | `f6ce463d20b43c3f0efaaecad843b71c1378090eb42d0c3fd84246a921334292` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/plugins-off.stdout.jsonl.gz` | `b99660016f9197ec7c6ce4b44c0a811822db5ae02a7b3955f8810ada43a32b8e` | `14ea778848c9bf5ebf7ea546d0c89b21a58ca1907cabf3bd6ddbb7b966a8319c` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/precedence.stderr.jsonl.gz` | `0d4b5f0d469aaf31f3741583934460eb9594bf5ac5298f4daf39c2cb6490f3b5` | `25063ab1b3e6cab647b7c60d1138a0204cdbf9ac0cd96622bbcf77442d3373ae` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/precedence.stdout.jsonl.gz` | `6fe0bd157c7d8a35f773ec61aadcc144484e515cd009a3b36f8130859e8038df` | `c880e12a98bbe4f7c7e3bdd5347e6d712e41ba906be1859fc8d2964658779b8b` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/stable.stderr.jsonl.gz` | `a2700c185fd19e9b4457c1ceef05ddd826eea1823e3c6a67038e342824395688` | `26792fbc383f5303981be1c89cf52b36b05f21e28851f3d01bc7d69f91be52d1` |
| `execution/PKG-02_Operative_Instruction_Surface_and_Runtime_Layers/1_Working/DEL-02-08_Exact_Supply_and_Protocol_Pinning/_run_records/APP-SERVER-0.149.0-G2-CANDIDATE-2026-08-24/03_EMPIRICAL_EVIDENCE/raw/stable.stdout.jsonl.gz` | `3768745c79125f5ddc8a5ff5e4e8128ab633403baadff0149fcf618c7369f60a` | `a85b085d886ff9f3b33f4a4c1d935e1918fcfd2a468562577714622848eeeabd` |
| `projects/chirality-app-v4/execution/_Coordination/AgentRuns/APP-V4-GROUP-B-20261008/production/RETURN.md` (from #1117) | `0f9db75d7ea9f4dac31f10b8c76199d7ff518f28d6f7b9750540269efd026571` | `c82591142b51fa0660337e1ee78ef46f4ffa6ae5f7c19af324dcbd572c41923c` |
