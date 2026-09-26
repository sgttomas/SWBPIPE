# CP2_RUNTIME_TESTS / EXTENSION_1 — return

This is the TASK (Type 2) return for the session-2 load-state WORKING_ITEMS manager. ROOT resumed this TASK
for review finding N-4.
- Hashes verified before starting:
  - brief: `f453bcec…a1d`;
  - CP2_WIRE_ADDENDUM_2: `ec66628e…133`;
  - ADDENDUM_1: `c389f5e3…c760`;
  - CP2_WIRE: `81a7adba…6996`.
- Paths are WORKING_ROOT-relative.
- Machine-specific paths appear only in `_run_records/`.

## Deliverable

- **New file** `core/product_physics/tests/load_reference_state_runtime_extension.rs`, which I alone wrote.
  Its sha256 is `6fc7b27a26ccb91908c74c640d9c46ae2de4f62c229c1c8d61d3c7872ac853d9`.
- **CP2 file unchanged.** `core/product_physics/tests/load_reference_state_runtime.rs` is byte-for-byte unchanged
  (`d5be0bd8…960b`).
- **Fixtures.** I added no helper models. `reference_cases.json` (`4d7b7777…beeab`) and
  `fixtures/product_preview/load_reference_source/eigen_motion.request.json` (`58ce5b95…e8e9`) were only read. The
  test checks the witness's bytes against its sha256 at run time.
- **No other writes:** no `src/**`, no other test, fixture, schema or UI file, and no Git.

**How expectations are formed.**
- Every value check runs in SparseInteractive and DenseScrutiny, through
  `run_linear_static_preview_value_with_mode`.
- Expectations are read from the fixture at test time, or come from an elementary derivation written beside each
  assertion. No expectation was taken from observed output.
- The criterion is the protected relative 1e-9. An exact zero uses a floor of 1e-9 × the case magnitude, named at
  each call site.

**Invented inputs.**
- 2 MPa pressure;
- 1000 N tip force;
- unused base pair 170 GPa / 0.28;
- point temperatures 20 °C and 300 °C;
- warm table points at 600 K and 300 K;
- a 550 K request;
- offsets of ±10 °C;
- a 100 °C midpoint.

## Tests (9) and the mutant each targets

| # | Test | Target | What it checks |
|---|---|---|---|
| 1 | `pressure_two_case_per_member_pairs_drive_wall_action_and_evidence` | M06, M19, M23 | Closed region at 2 MPa with both closures transferring to the wall. One material has an unused base pair and two exact points (the fixture serial-companion pairs). Case A is point A, unchanged; case B is point B with interval strain 0.001. Fixed and free variants. Checks wall N at 5 stations, S = Nw − p·Ai, the wall end actions (−Nw at end i, +Nw at end j), root and far Fx, and the tip UX. Asserts `pressure[].materials` and `exact_cases.pipe_materials` E/ν/G, `material_selection_kind` and `resolved_eigenstrain`. Rejects wall N computed from the base pair or from the other case's pair, and the base-ν Poisson tip. |
| 2 | `temperature_identity_groups_select_same_point_across_units` | M22 (and M14 side) | For every ordered unit pair of every fixture identity group (−50 °C ≡ 223.15 K; 242 °C ≡ 467.6 °F ≡ 515.15 K; 20 °C ≡ 527.67 °R) there are two routes. (a) An exact point, with the actual operating T authored in the other unit: no override. (b) Interpolation to the same-class endpoint. Checks: exactly one consumed point (`point:cold`); E and ν bitwise equal to the point's; the K evidence equals the fixture kelvin; tip UX = F·L/(E·As), bitwise identical across all units, groups and routes. |
| 3 | `temperature_identity_non_equal_control_is_not_snapped` | M24 | Interpolation at −49.999999 °C against a 223.15 K point: two consumed points, a fraction in (0, 1), and E strictly between the two points' E. An exact point at that operating T gives `LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED`. |
| 4 | `verification_two_point_secant_table_needs_only_install_to_operating_coverage` | M16 | The fixture's `verification_two_point` (VERIFICATION control-4 two-point table, datum 20 °C outside it). Fixed: N and Fx. Free: tip UX = 43/25009 and N = 0. Evidence checks cover the strain, T_install/T/datum in K, λ(T_install) = 1 + 9/25000 (the fixture's minimum stretch), λ(T), and consumed point indices [0, 1]. Operating at 160 °C and installation at 40 °C each block with `LOAD_STATE_STRAIN_UNRESOLVED`. |
| 5 | `temperature_identity_binary64_order_inversion_blocks` | M15 | Material points at 467.6 °F and at the fixture's 515.1500000000001 K (plus 600 K), requested at 550 K. Blocks with `LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED` and publishes no results. |
| 6 | `duplicate_class_material_points_are_refused` | M14 | Two point pairs of the same exact class in different units: −50 °C with 223.15 K, and 467.6 °F with 515.15 K. Each is requested at the duplicate class and at an interior T. Blocks with `LOAD_STATE_MATERIAL_TEMPERATURE_AMBIGUOUS` (or `_POINT_AMBIGUOUS`). |
| 7 | `dilation_datum_zero_is_consulted_not_consumed` | M17 | An `engineering_dilation` law with points (20 °C, 0), (50 °C, 9/25000) and (150 °C, 13/6250), which are the fixture dilations, and datum 20 °C. The exact-sample variant expects 43/25009; the interior variant at 100 °C expects the derived value. Checks N and the evidence strain; consumed indices == [1, 2]; consulted ∋ 0; every consumed segment is `interpolation_sample`; the interior variant has an interpolation_sample 1-2. |
| 8 | `joined_eigen_motion_closed_form_and_source_publication` | item 7 | Closed-form values are listed below the table. Publication: `load-reference-source-1`/`resolved_straight_load_state_source_v1`; receipt `LOAD-REFERENCE-SOURCE-1` with status `qualified`; `source_recovery {selected, retained_source_blocks_exact_v1}`; `solve.recovery_method`; info `SOURCE_BLOCK_RECOVERY_SELECTED`; no NOT_JOINED. |
| 9 | `joined_eigen_motion_with_pressure_stays_ordinary_not_joined` | item 7 companion | The witness plus a 2 MPa closed region. It stays load-reference-1 and `not_joined`, with exactly one NOT_JOINED and an info `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`. The closed form Nw = E·A(δ/L − ε\*) + 2ν·p·Ai gives S, the end actions, anchor Fx = p·Ai − Nw and stop Fx = Nw − p·Ai. The Poisson term, about 2.5e4 N, is far above the roughly 6e-5 N floor; the test states this. |

Test 8 closed form, derived independently:
- tip UX = δ;
- tip UY = UY_root + L·θ, and tip RZ = θ;
- root RX = T/k, and tip RX = T/k + T·L/(G·J);
- N = E·A(δ/L − ε\*) with ε\* = (1 + 6e-5)(1 + 4e-5) − 1;
- anchor Fx = −N and stop Fx = +N;
- spring Mx = −T;
- zero anchor Fy/Fz/My/Mz, zero shear and bending at the end and midspan stations, and zero tip UZ/RY.

The inputs are read from the witness and checked against the brief's stated values. The annulus As and J come from
my own formulas and are cross-checked against the fixture geometry.

**Wall-action relation (test 1 and test 9).** It comes from DESIGN.md §4 (Cold spring and installation) at
`9e8a55d`: "Nw = E(T) A [(u_j−u_i)/L − epsilon_star] + pressure_Poisson_term".
- The Lamé state of a straight annulus under internal p gives σr + σθ = 2p·ri²/(ro² − ri²), so the Poisson term is
  2ν·p·Ai.
- With closures transferring to the wall, nodal equilibrium gives root Fx = p·Ai − Nw and far Fx = Nw − p·Ai.

## Commands and results

Raw logs are in `_run_records/`:
- `COMMANDS.md`;
- `ext_run_1.log`, sha256 `86250bbf…bd27`;
- `ext_run_2.log`, sha256 `e833764e…65dd`;
- `cp2_baseline_run.log`, sha256 `42a82e92…8bb6`.

The command was the brief's: `cargo +1.97.1 test --locked --offline -j 1 --test <binary> -- --nocapture`, run in the
ROOT-given target directory and only after the manager said the tree compiles.

Candidate: HEAD `d92ca3b7c` plus the uncommitted implementer working tree. The src tree hash was `8d197045…b221`, identical before and after every run. This is not a committed candidate.

- **Extension (final file `6fc7b27a…`): 8 passed, 1 failed.** The failure is the same in both modes and in both
  runs. Original failure text:
  ```
  assertion `left == right` failed: sparse_interactive exact_samples: consumed_law_segments use label {"end_k":423.15,"lower_index":1,"start_k":323.15,"upper_index":2,"use":"integration_interval"}
    left: String("integration_interval")
   right: "interpolation_sample"
  ```
  Observed law evidence (printed before the assertions):
  - Dilation, exact samples:
    - `consumed_points=[1,2]`;
    - `consumed_segments=[{integration_interval, 1-2, 323.15→423.15 K}]`;
    - `consulted_points=[0,1,2]`;
    - `consulted_segments=[]`.
  - Dilation, interior (100 °C):
    - `consumed_segments=[{interpolation_sample, 1-2, 373.15→373.15}, {integration_interval, 1-2, 323.15→373.15}]`;
    - `consulted_segments=[{interpolation_sample, 1-2, 373.15→373.15}]`.
  - Secant (verification_two_point), for comparison: `consumed_points=[0,1]`, `consumed_segments=[]`,
    `consulted_points=[0,1]`, `consulted_segments=[]`.

  **Expected:** every consumed segment of a dilation law is an `interpolation_sample`.

  **Derivation.** An `engineering_dilation` law gives λ(T) = 1 + d(T), with d linearly interpolated, and
  ε_th = λ(T)/λ(T_install) − 1. Only the samples d(T_install) and d(T) enter the value; no coefficient is integrated
  over [T_install, T]. ADDENDUM_1 §3 says consumed data "entered the value", while consulted data "only established
  coverage or positivity admissibility". An interval over [T_install, T] can therefore at most be consulted.
  The point indices ([1, 2] consumed, 0 consulted and not consumed) and the mechanics all pass. The failure concerns
  only the consumed segment label. I have reported it to the manager and did not adjust the expectation.
- **CP2 binary (unchanged `d5be0bd8…`): 20 passed.**

**Test edit between runs.** It added only evidence prints. It also narrowed the consulted-segment label check to
"one of the two wire labels" before any consulted value had been observed, because the wire assigns no label to a
pure admissibility interval. The consumed-segment assertion was not changed.

Observed blocking codes. All were `MODEL_INCOMPLETE` in both modes, with no results published:
- `LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED` for the non-equal control;
- `LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED` ("distinct exact temperatures do not keep a strictly increasing binary64 representation");
- `LOAD_STATE_MATERIAL_TEMPERATURE_AMBIGUOUS` for all four duplicate-class requests;
- `LOAD_STATE_STRAIN_UNRESOLVED`/`OutsideTableCoverage` for both out-of-interval secant variants.

## Wire ambiguities and limits

1. **Segment `use` semantics.** The wire defines the two labels only by name. For sampled definitions (secant,
   dilation), the product's dilation evidence places an `integration_interval` over [T_install, T] in consumed data,
   while its secant evidence places none. Also, in the interior dilation case the same interpolation sample appears
   in both the consumed and the consulted lists. The wire should state which segments each definition consumes, and
   whether consumed and consulted may overlap.
2. **An interpolation sample's segment** is published with start_k = end_k = the sampled T. The wire does not
   describe this shape; I assert only `use`, `lower_index` and `upper_index`.
3. **Kill targets are argued, not demonstrated.** No mutant was run: the brief allows no `src/` writes. Each target
   follows from the mutant's described effect, as the review's `review2_mutations.py` gives it.
4. **Test 8's tip RX** includes a member torsion term T·L/(G·J) of about 4.6e-15 rad, which is below the 1e-9 check
   on 1e-4 rad. That check therefore pins the spring path, not GJ.
5. **Not exercised:**
   - duplicate-class refusal through an exact-point selection (only interpolation selections are asserted);
   - `LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED`, `LOAD_STATE_MEMBER_SECTION_MISSING` and ADDENDUM_2's closed-quantity or
     empty-variant rejections. These are outside this brief.

The target directory was deleted after the runs, as the brief directs (recorded in `_run_records/COMMANDS.md`).

## Note added after the return

The manager clarified the method after my return had been delivered to ROOT. The clarification is recorded for the CP3 review in CHECKPOINT_3.

- For `engineering_dilation`, the strain is Δd/λ_install.
- Δd is formed as a sum of per-segment increments over [T_install, T] (the CP1-reviewed kernel's `dilation_difference`), not as the difference of two samples.
- That interval data is therefore consumed, with use `integration_interval`.

This is algebraically the same value as my (1 + d(T))/(1 + d(T_install)) − 1. It differs only in how the value is formed. The wire (ADDENDUM_1 §3) does not fix the method, so the failing label assertion encodes my assumption about the method, not a physics or index defect. The M17 point split passes.

Because the owner directed a pause and the rerun budget is one, I did not change or rerun the test. Test 7 stays recorded as failing on this wire ambiguity.

Resolution belongs to the wire. If CHECKPOINT_3 adopts the clarification into the wire, the consumed-segment label assertion can be revised to the stated method, with a single rerun. Until then the test stays unchanged.

## Final step: test 7 revised to the adopted CP3 wire, and rerun

ROOT adopted the producer's dilation method into the wire: `CP3_WIRE_ADDENDUM.md`, sha256
`f69043b682d027cc7f15e2173b932f016cd76f3a64d76b76d974cd0f0105f7d4`, verified.

**Revision.** Only test 7's consumed/consulted segment assertions changed. Every other assertion, the M17 point
split (consumed == [1, 2], consumed ∌ 0, consulted ∋ 0) and every tolerance is unchanged.
- Test file sha256 before: `6fc7b27a26ccb91908c74c640d9c46ae2de4f62c229c1c8d61d3c7872ac853d9`.
- Test file sha256 after (final): `5febacba957bc9aed01c8010ea3496616d05021290c2b15d0e08cbabf4558a58`.
- The diff is `_run_records/test7_revision.diff`.

**Removed:**
- "every consumed segment is `interpolation_sample`";
- "each consulted segment's `use` is one of the two labels";
- "the interior variant has a consumed interpolation_sample 1-2".

**Added:**

1. **Shape checks** on every segment entry, via a new helper `segments_match` (addendum §1):
   - `upper_index == lower_index + 1`;
   - `T_lower ≤ start_k ≤ end_k ≤ T_upper`;
   - an `interpolation_sample` has `start_k == end_k` bitwise;
   - an `integration_interval` has `start_k < end_k`;
   - no unknown label.

2. **Exact set equality** (order-free, one-to-one) between the observed list and the expected entries. The expected entries are derived from addendum §2 (engineering_dilation) and the fixture inputs:
   - the table is 0 = T_m 20 °C, 1 = T_i 50 °C, 2 = 150 °C;
   - the Kelvin values are the fixture's `installation_temperature_K` and `operating_temperature_K`, and their mean for the 100 °C midpoint;
   - the Kelvin values are compared with the protected 1e-9 relative criterion.

   | Variant | consumed segments | consulted points | consulted segments |
   |---|---|---|---|
   | T = 150 °C (exact samples) | `{integration_interval, 1–2, T_i_K → T_K}` | [0, 1, 2]: datum and positivity samples | none, since dilation never consults an integration_interval |
   | T = 100 °C (interior) | `{interpolation_sample, 1–2, T_K = T_K}` and `{integration_interval, 1–2, T_i_K → T_K}` | [0, 1, 2]: datum 0 and T_i 1 as points; 2 as an endpoint of the consulted sample | `{interpolation_sample, 1–2, T_K}` (an overlap the addendum permits) |

3. **Consulted points exact.** `consulted_law_point_indices == [0, 1, 2]` in both variants, derived as above. The earlier "consulted ∋ 0" check is kept.

**Rerun on a fresh rebuild.** Cargo +1.97.1 --locked --offline -j 1, ROOT-given target dir, each binary once. The
source tree hash was `8d197045…b221`, identical before and after, and the same candidate as the earlier runs: HEAD
`d92ca3b7c` plus the uncommitted working tree. It is not a committed candidate.
- **Extension** (`5febacba…`): **9 passed, 0 failed**, including `dilation_datum_zero_is_consulted_not_consumed` in
  both modes. The log is `_run_records/final_load_reference_state_runtime_extension.log`.
- **CP2** (`d5be0bd8…`, unchanged): **20 passed**. The log is `_run_records/final_load_reference_state_runtime.log`.
- The target directory was deleted afterwards.

With this revision, `CP3_WIRE_ADDENDUM.md` resolves two of the recorded ambiguities:
- limit 1 (segment use semantics, and the overlap between consumed and consulted), including the earlier post-return note;
- limit 2 (the shape of a sample entry, start_k == end_k), which the test now asserts.

Limits 3 to 5 are unchanged.
