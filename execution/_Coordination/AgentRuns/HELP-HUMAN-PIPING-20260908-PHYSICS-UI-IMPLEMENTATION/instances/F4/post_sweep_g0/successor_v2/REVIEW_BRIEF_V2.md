# RF post-sweep G0 successor review brief

Status: `SEALED_FOR_ROOT_DISPATCH`

Target `/root/friction_code_review` as an independent read-only Agent 2 using `gpt-5.6-sol` at high effort. Delegation, sibling messaging, source/fixture/test/build/Git writes, lifecycle acts, and release acts are prohibited. Write only under `{RUN_ROOT}/instances/RF/post_sweep_g0/successor_v2/**` and return through `/root`.

Review these exact successor inputs:

| Input | SHA-256 |
| --- | --- |
| `{DEL0404_V2}/DIAGNOSIS_V2.md` | `9c7814a14198b7840aa228abd03c701d1de542ffdcb28ff3c0da0480cf1d3bb3` |
| `{DEL0404_V2}/AFFINE_BOUNDS_V2.json` | `4172c62c88ec1eece8fa1784d24ad1b0bd362c20a54ace3fbae84cdc67f66e8a` |
| `{DEL0404_V2}/AFFINE_BOUND_DERIVATION_V2.py` | `ea6af7da522b0e92c3a9103a00975d89ceba01335f08b90b5d3f63b1cad60342` |
| `{DEL0404_V2}/PUBLIC_API_AFFINE_HARNESS_V2.rs` | `b61560b184e39e6915ae0fd9749c0371336a0fe3364a00715cfdc8a49f1e56b0` |
| `{DEL0404_V2}/PUBLIC_API_MODEL_VARIANTS_GENERATOR_V2.py` | `00f04bb9679f0cfe19b2d95d4fd12861ac15376c7ef2e454868dfa6241e3c233` |
| `{DEL0404_V2}/PUBLIC_API_AFFINE_OUTPUT_V2.jsonl` | `c556a9db4ab4f15e2603ce3c50793b96dc5a7aaa8f2a08f3386707757e6ff351` |
| `{DEL0404_V2}/GENERATED_FIXTURE_CHANGE_INVENTORY_V2.json` | `d04ce5db7266969e0ab46ef7ceabdd43e32d8a3928e13cd172a26c45b6938d87` |
| `{DEL0404_V2}/PROPOSED_ACTIVE_ORACLE_UPDATE_V2.patch.b64.json` | `faa878e6278c5f5aac405817dca56655b1b9a07e231ee8324852e939e82595eb` |
| decoded exact four-path patch | `9210169b6968d7f319506cfb53b8d6bc71132b713fba8fa52736b2ee4bc8e8d7` |
| RF V1 review | `17dde8ec770db7af923511a369d7ae167dad5cbbeef56b5779b791d4dd0a4e39` |
| RF V1 return | `eb3b764fa6c587dc30b8f1e5a667dd652e2d27966ad1903a06b7624ce6a20df8` |

`{RUN_ROOT}` is `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION`. `{DEL0404_V2}` is the DEL-04-04 `_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/post_sweep_g0/successor_v2` directory.

Independently verify the fixed-point derivation and its explicit retained L-200 last-digit calibration; Dense/Sparse parity and stable branch/state conditions; the registered generator identity; same 830 row IDs and 31 diagnostics; exactly two summary and 338 result-value changes with no omitted changed field; and every decoded patch hunk. Confirm the only mutable paths are the Rust test block, generated runtime fixture, Python test, and desktop test; solver and public API bytes are unchanged. Verify the runtime/browser/native-failure fallback impact and the proposed focused, frontend-build, full-product, fresh-diff, CHANGE, clean-sweep, and native/bundle gates.

Return `PASS` or `CHANGES_REQUIRED` with exact hashes. A pass is a technical review only; it does not authorize applying the candidate. State that a new bounded Owner act is required for all four paths and classify the fixture as runtime reference data rather than test-only data.
