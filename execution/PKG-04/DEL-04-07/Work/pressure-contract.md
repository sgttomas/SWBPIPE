# Pressure contract and T4 integration

## Current pressure boundary

The owner retired `1.0.0/legacy_pressure_v1` product-wide in #1168. It is not accepted even for zero-pressure documents. The flawed computation, its historical bypass and dependent pressure oracles are removed. A pressure-free model is a document without a pressure contract. On 2026-10-10 the owner decided that each format keeps only its current version ("as you recommended, no-compatibility principle carries"), reversing the 2026-10-08 choice of model formats 0.1.0/0.2.0 as the pressure-free namespace: B7 defines one model format, 0.5.0 (designed in DEL-04-08 Design and accepted by HELP_HUMAN on 2026-10-10), that expresses models with or without pressure, and refuses every older version by name ([DEL-04-08 Design, b7-plan.md](../../DEL-04-08/Work/b7-plan.md)). B7 reached main in #1387: 0.5.0 became the only model version, a document without `pressure_contract` is pressure-free, and 0.1.0 to 0.4.0 are refused by name (`MODEL_SCHEMA_VERSION_RETIRED`, PP `model_format.rs`). Since T4-U4b/U5b, 0.6.0 is the only model version and 0.5.0 is refused by name too ([u4b-u5b-implementation.md](u4b-u5b-implementation.md) §1.1). Any pressure primitive in a non-exact document, including zero, requires re-authoring (`PRESSURE_MODEL_REAUTHOR_REQUIRED`); authoring uses `OP-PRESSURE-PRIMITIVE-RETIRED`.

The current exact route admits straight pipe members (Euler–Bernoulli, or Timoshenko since T4-U8), realized bends carrying the user's k (T4-U2), objective connectors, unpressurized or as constant-effective-area joints (T4-U3, T4-U5a), untied or with explicit elastic tie rods (T4-U5b), and linear restraints or springs. W-16 slice c admits a case's DEC-068 `equivalent_static` generation on documents without the load/reference-state group: seismic loads use the route's D-5 mass basis, and generation is refused by name on load/reference-state documents (`EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED`) and for wind on connector-replaced spans (`EQUIVALENT_STATIC_WIND_SPAN_REPLACED`), and partial-span wind on Timoshenko members takes the exact Timoshenko fixed-end vector (DEL-05-01 Design `exact-equivalent-static.md`). W-16 slice b admits model-authored load combinations (all three bases) over the primitive cases, formed from their published rows; a mechanics combination is withheld by name across mixed modulus bases or resolved stiffness, or when an installed connector or tie-rod reference meets factors that fail the unit factor-sum rule, and a document with a combination attempts no retained-source recovery (DEL-05-02 Design `exact-combinations.md`). The retained exact route still refuses combinations. RT-A3 increment 1 admits the branch connection (tee) and recognises pressure junctions at nodes where regions end ("Tees and pressure junctions", below). Mapped rigid-family components retain W-15's equivalent-member scaling; reducer increment 2a admits the bounded concentric step described below. Families outside the current admission seam remain refused by name (`EXACT_PRESSURE_FAMILY_NOT_ADMITTED`, PP `exact_admission.rs`). T4 extends that route; it does not revive the legacy computation. Existing PRD/INTENT pressure requirements remain desired scope, not claims of present capability. T4 rebuilds the curved-pressure and membrane validation removed with the flawed computation.

## Accepted T4 direction

- **D-1:** the L-line (T4-U2) is the first usable path; the corrected joint (T4-U3) proceeds in parallel.
- **D-2:** only realized bends carrying the user's k carry pressure. Geometry-only chord bends are refused on the exact route with a realization message and remain pressure-free.
- **D-3:** steady-flow/transient loads, Bourdon opening, pressure stiffening of k/SIF and ovalization are excluded and disclosed on bend results and the v3 approximation. T4-U9 requires a later owner choice. The owner chose on 2026-10-28 ("As recommended"): the Bourdon effect is a Should option for v1.0, off by default and clearly labelled; pressure stiffening of k/SIF stays out of scope, and users enter their own k and SIF (`docs/RELEASE_TARGET_v1.0.md` RT-L2).
- **H-1:** all new authoring writes `3.0.0/exact_pressure_v3` with reserved `pressure-1` semantics, straight-only documents included (HELP_HUMAN, 2026-10-10, revising the earlier "straight-only documents continue to be authored as v2"). T3's captured v3 join publishes a selected supported retained-source case under `pressure-source-1`; identity, policy, profile and case commitments are chosen from the captured contract. The temporary `EXACT_PRESSURE_V3_RETAINED_SOURCE_NOT_JOINED` diagnostic and capture suppression lapse with this join. Families outside the source method remain declined by name (D7 below). HELP_HUMAN's ruling D4 (a) (2026-10-10) rebased W1's exact route E onto `pressure-1` in B7b, and v3 is the only exact contract (DEL-04-08 Design, `b7-plan.md`). The applier and desktop write v3 since T4-U2's authoring step (below).
- **SP-1 (lapsed with v2):** while v2 existed, a straight-only v3 case was bit-equal to its v2 twin in its ordinary-route publication (and, from the join, in supported retained-source publications outside D5's closed exclusions). B7b's v3 twins of the v2 fixtures were checked against the v2 originals once, with every physical number bit-identical; the twin tests themselves went with v2.
- **v2 retirement (B7b, as built):** `2.0.0/exact_straight_pressure_v2` is deleted with its contract, admission branch, the readers' v2 acceptance, its four result identities and tables (physics-1, physics-source-1, load-reference-1, load-reference-source-1), fixtures and the SP-1 twin tests. A 0.5.0 document declaring it is refused by name (`PRESSURE_MODEL_REAUTHOR_REQUIRED`), a stored result naming a retired identity is refused at header dispatch (`SOURCE_EXACT_PRESSURE_V2_RETIRED`), and no migration code exists (DEL-04-08 Design, `b7-plan.md`).
- **U4b/U5b formats and identities.** T4-U4b and T4-U5b landed together on model 0.6.0 ([u4b-u5b-implementation.md](u4b-u5b-implementation.md)), which retires and re-authors 0.5.0 (HELP_HUMAN, 2026-10-13; DEL-05-03 Design `section-and-shear-decisions.md`). Carried in the owner's 2026-10-14 resumption steer: the raw/component result format stays 0.2 and the canonical result format stays 0.3; the ordinary semantic identities became `pressure-2` and `preview-physics-2`, and `pressure-1` and `preview-physics-1` are refused by name (`SOURCE_PRESSURE_1_RETIRED`, `SOURCE_PREVIEW_PHYSICS_1_RETIRED`).
- **H-2:** retain the pre-cancelled ledger and add member-owned bend `K_b·u_free(ε_p) − c_b` and joint `p(Ae−Ai)` terms. One implementation owns the assembly. T4-U5 must independently compare representation (A), including JR's tie-force double-count control. A connector-replaced span has no owner for transferred eigenloads: the existing named unsupported-transfer refusal applies unless its raw intended eigenstrain midpoint and enclosure radius are both exactly zero (K-D5 source-closure ruling, 2026-10-11). A binary64 total rounded to zero is insufficient. Connector q_ref remains author-supplied. T4-U5a's joint term, its representation-(A) comparison and the untied J4 control are designed in [joint-pressure-thrust.md](joint-pressure-thrust.md); elastic tie rods follow in T4-U5b (SCA-012).

**Batch-4 M3 dependency.** The seven RV31/RV32 repairs and 135 independent checks bind U2 source `98dba87714262722a78a534ad5921a188e5ff44f`. Existing FK primitive-law controls establish their re-formed operand and matrix laws; the M3 numbers in [DEL-04-01 Design §4.3](../../DEL-04-01/Work/kd5-member-relative-bound.md) are emulation evidence. These warrants do not establish the final integrated producer. After the K-D5 source seal, batch 4 must bind every grouped Poisson, terminal-cap, bend-cap and kink constituent to independently enumerated curved/tangent closure manifests, and `PressureFreeStrainCurved` to the same `K_int,arc` and raw `(1−2ν)pAi/(EA)` law. Final repaired section cuts and wall publication must preserve `Nw=N_el+pAi`, `S=N_el`, including end-tangent actions, stations and the separate wall bound. `ShortBend` removal remains conditional on actual final-producer closure and M3/reference controls in both modes, including drop-one-of-many checks; `bend_neighbour_ratios` remains for pivot refusal. On the integrated batch-4 branch each factor-group summand now binds its own `PressureOperand` (`pressure_runtime::closure_source`: chord direction on a straight member, the end tangent at the group's node on a realized arc), each arc pushes its `PressureFreeStrainCurved` per K_int,arc column beside its ledger product, and certified arc publication keeps N_w = N_el + pAi along the end tangents (`certified_publication::arc_pressure_wall`). The family bindings, the drop-one-of-many, duplicate, extra, wrong-pipe and wrong-binding mutants on the ordinary and W2 routes, and the arc wall law are pinned in PP `kd5_pressure_closure_tests.rs`. M3 on this producer holds in both modes, including the wTp (pressure) models RV30 and the realistic set now admit, with 0 judged member, section, support or stress breaches on Passed cases and 0 strict coverage failures ([DEL-04-01 Design §4.3](../../DEL-04-01/Work/kd5-member-relative-bound.md)); `ShortBend` is removed on that evidence.

## T3 coexistence and B7

**The v3 retained-source join (D7, HELP_HUMAN 2026-10-10).** Every v3 family or basis a T4 unit admits (realized bends in T4-U2, dual-basis sections in T4-U6, Timoshenko in T4-U8) is declined by name in `prepare_sources` unless that unit extends the source method; otherwise a selected case's composite evidence fails finalization and loses that selected response. `prepare_sources` declines a realized curved bend ("realized curved bend (curved_bend_macro_element) is not a straight source family") and T4-U3's objective connector ("objective connector is not a straight source family") by name before its dimension closure, which a realized arc or a connector-replaced span (no straight frame) previously failed as `SourceMismatch("actual model/source dimensions")`; a legacy curved model that reaches source recovery takes the named decline too (no compatibility path). `source_recovery`'s D7 test pins both. A selected source that cannot finalize republishes the invocation on its ordinary route, with the failure stated, on both the 0.4 and pre-0.4 paths (DEL-04-08 Design `pre04-failure-unification.md`, 2026-10-22, owner decision 17; it supersedes this paragraph's earlier "the pre-0.4 path retains its named refusal"). The join itself, `pressure-source-1`, is governed in DEL-04-08 Design: direct received v3 evidence, no v2 projection/reseal, explicitly empty pressure inventory for selected source cases, and the existing 0.4 T1 ineligibility boundary. D6's export refusal lapsed with T4-U2's export (#1314): a `pressure-source-1` envelope exports as the result document, which binds its received v3 evidence and receipt (b7-plan MJ8); since epoch 8 the desktop's result JSON export, stress-neutral export and rule check admit it at the Rust reader's numerically eligible standing, and every other desktop output surface refuses it by name (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; [desktop-exact-outputs.md](desktop-exact-outputs.md)).

**The interim after O4 (owner, 2026-10-12: "Scope extension after B7b").** T4 does not extend the source method for T4-U6 or T4-U8. Named declines are the interim, as built in T4-U6/U8 ([section-bases-and-timoshenko.md](../../../PKG-05/DEL-05-03/Work/section-bases-and-timoshenko.md) §5): `prepare_sources` declines a Timoshenko document and one with nonzero mill tolerance or corrosion allowance by name, after the D7 declines, and W1-E declines the same documents at its G-A admission (`ShearModel`, `SectionAllowance`). Such a document publishes on the ordinary route without retained-source or W1-E publication. Nonzero mill tolerance, admitted on both retained routes before D-5 with one effective wall, is declined since. The pressure-source-1 and physics-retained-1 readers require zero mill tolerance and zero allowance with `euler_bernoulli`. Since T4-U8 made Timoshenko the authoring default, default-authored documents take that decline. A source-method/W1 extension that restores retained-source and W1-E publication for default-authored documents is T3 work after B7b ([DEL-05-03 Design](../../../PKG-05/DEL-05-03/Work/section-and-shear-decisions.md)). T4-U5a's pressurized joint keeps the existing connector decline.

**As built (O4, epoch 5).** O4 retired that interim: both retained routes now admit Timoshenko and nonzero-allowance documents on straight members under the D-5 bases, and the pressure-source-1 and physics-retained-1 readers rederive both bases and the prepared shear-area leaf instead of requiring zero allowance and `euler_bernoulli` (DEL-04-08 Design [o4-source-extension.md](../../DEL-04-08/Work/o4-source-extension.md)). The D7 bend and connector declines are unchanged.

Keep PP's Cargo.lock and priced layouts unchanged (`MemberRecord`, `RecoveryRecord`, Preview inputs); connector fields and bend E/ν belong on unpriced component types. Reconcile real shared-file diffs with the preserved B2 work before T4-U1/U3 land, including `s11f_site_test.rs`. T4-U1 must explicitly account for the changed ordinary envelope on a refused retained call and whether curved cases reach the source attempt. The curved numerical requirements are in DEL-04-01's Design.

**H-4:** the summary-key rename, removal (not renaming) of the historical joint row kind, deferred preview-physics wording and any LIMITATIONS change occur together in **B7**, not PR-B2. B7a carried that wave (C1–C6) to main in #1387: the summary count is `curved_bend_macro_element_count` and the historical joint row kind is gone. LIMITATIONS strings are reviewed semantic inputs, so changing them requires re-registration. T4-U4a's preview-physics-1 arc-frame unification landed in the same wave (C5) with the LIMITATIONS sentence and table delta it specifies ([arc-frames-and-signs.md](arc-frames-and-signs.md) §8); its pressure-1 stage landed before it (#1333), and T4-U4b does not wait on it. No product change is authorized merely by migrating these requirements.

**O-10 (HELP_HUMAN, 2026-10-10):**
- TEXT is priced, not zeroed.
- Rule R1 (`edge_zero for_source → validate_pressure_evidence`) is accepted, and the v3 identity text (+23,920 B) is priced.
- The batch-3 rule whose callee `UserStiffnessElement::orientation` is deleted is dropped.
- T3's manager adjudicates L1–L3 and the connector text-argument classification, with an independent check.
- T3 re-runs SQ2 and the TEXT chain on T4's batch heads. The owner clarified on 2026-10-11 that qualification may use up to 12 GiB on the 16 GiB base machine; 10.5 GiB was the earlier selected profile. Re-registration retains the 0.9 M margin and 5 % text-budget rule. The owner-adopted larger-machine scaling and safe parallel reservations are defined in [the memory-budget design](memory-budget-decision.md); the separate 64 GiB development-host allowance does not select a product profile.

Sources: [NUM frozen rulings: pressure retirement, U3 and U5](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md); [T4 frozen rulings: D-1–D-4, H-1–H-4 and SP-1](https://github.com/sgttomas/chirality/blob/f2211ecef6/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/PRESSURE_STRESS_T4/T4_RULINGS.md).

## T4-U1b and T4-U2a as implemented

- **T4-U1b (certified arc loads, R-1):** S11-G certifies consistent load vectors on realized arcs with the proven midpoint–radius enclosure of T4-I9 revision 01 (directed binary64 radii; `Exact { scale: 1.0, scaled_intended: split(m) }`; no FK type, priced-layout or PINNED_RECORD change). Every failed precondition stays CannotBound. The arc load vector uses exact-rational series below the s < ½ switch. T15c's natural refusal stays at guard level: no model in the k window gives an ordinary Passed report, so the end-to-end CannotBound sentence is not shown (R-2's narrowing, ruled by HELP_HUMAN; DEL-04-01 Design).
- **T4-U2a (seam and v3 identity):** `PP/src/exact_admission.rs` classifies every family per contract; v2 keeps its codes and texts; under `3.0.0/exact_pressure_v3` every family not yet admitted is refused with `EXACT_PRESSURE_FAMILY_NOT_ADMITTED`. v3 publishes under `pressure-1` with its own limitations (D-3 exclusions; collapse not assessed) and admits signed pressure. The three readers dispatch on the contract and refuse cross-reads by name. Pressure-1 export was refused by name until T4-U2 (it exports since; see "Export chain" below). The `pressure-1` table is not a reviewed input.
- **SP-1's twin clause** holds on the ordinary route and now on supported captured retained-source twins under D5. Selected v3 cases publish `pressure-source-1` with the captured v3 receipt/evidence context; ordinary v3 cases remain `pressure-1`. Numerical references and tolerances remain unchanged.
- **Captured retained-source join:** the former v3 gate and missing-join flag/diagnostic are removed. Supported cases that need retained-source recovery use the same capture/selection path under their actual contract; selected v3 publication carries the joined limitations. The typed entry still attempts no retained-source recovery under either contract. Failed source admission or finalization never yields an inferred valid or eligible source publication: the case keeps its ordinary route, and a selected source that cannot finalize republishes the invocation ordinarily (DEL-04-08 Design `pre04-failure-unification.md`).

## T4-U2 as implemented

This records what the code does at T4-U2 source `98dba87714262722a78a534ad5921a188e5ff44f` (T4-U2 with T4-U3 and O-13). Final integration with K-D5 remains subject to the batch-4 M3 dependency above; this is not an acceptance claim.

- **Realized bends under v3 (D-2).** `3.0.0/exact_pressure_v3` admits realized curved bends carrying the user's k; a region span over a geometry-only (chord) bend is refused with a realization message and the bend stays pressure-free. v2 is unchanged.
- **Ledger (H-2).** Straight members keep the pre-cancelled ledger (SP-1). Each arc owns K_b·u_free(ε_p) − c_b, with ε_p = (1−2ν)pAi/(E·As) formed in Scaled arithmetic and c_b its end caps along the end tangents. A region changes direction only at realized bends: a bend-end direction change θ ≤ α_tan = 1e-3 rad is carried exactly by the kink remainder pAi(t_in − t_out); a larger θ is a mitre, refused (`PRESSURE_REGION_MITRE_UNSUPPORTED`) until T4-U7, and a straight–straight kink is `PRESSURE_REGION_NONCOLLINEAR`.
- **Arc recovery.** At arc ends and stations N_w = N_el + pAi and S = N_el; shear and moments are elastic. The `element_local_*` end rows stay in the chord frame and the wall end action is published along the arc's end tangents; T4-U4a moves the end rows to the tangent frame at each end ([arc-frames-and-signs.md](arc-frames-and-signs.md)). The straight Lamé hoop and radial rows and the straight-statics maximum were withheld on arcs, with the reason named in `withheld_on_arcs` and the arc listed as unavailable, until T4-U4b (HELP_HUMAN split T4-U4 into U4a and U4b on 2026-10-12). T4-U4b removed the withholding: pressurized arcs with R > r_o publish the toroidal membrane hoop and radial rows, and arcs publish the certified normal-stress maximum, or name it unavailable ([u4b-u5b-implementation.md](u4b-u5b-implementation.md) §§2.3 and 2.5). O-13's arc recovery guard (DEL-04-01 Design, `arc-recovery-guard.md`) demoted arc end groups with formation noise to Sensitive and a bend shorter than Rφ/L = 1.6e-2 to Sensitive; on the integrated batch-4 branch K-D5's unified member predicate on certified rows replaces both.
- **Reading pressure-2.** The Rust reader (RE), the exact identity's only product reader, and the test-only Python verifier (`tests/verifiers/pressure_2.py`, which shares no code with the producer) read pressure-2 (pressure-1 until T4-U4b/U5b, which refuses it by name), including arc evidence, the v3 RHS terms (`bend_cap_removed`, `kink_remainder`), model-scoped review rows and coverage by pipes or connectors; v2 refuses arc evidence. Their shared corpora (`fixtures/results/pressure_v3_{straight,arc,connector}_reader_corpus.json`) are regenerated only by their PP generator tests. The desktop does not read pressure-1: such a result is refused as `PRESSURE_1_RESULT_NOT_READABLE_IN_DESKTOP` on the solve job, results panel, standing, notices, rule checks and every output surface, until T3's reader consolidation. Python product paths (the analysis-run record, the stress-neutral package and the product preview) refuse it as `PRESSURE_1_RESULT_NOT_READABLE_IN_PYTHON_PRODUCT_PATH` until T3's unit R2 gives them the Rust reader's verdict (owner, 2026-10-10: "move to one product reader in Rust"; DEL-04-08 Design, `b7-plan.md`). On the integrated batch-4 branch R2 is present, and `reader_verdict` has the exact identity's route, `pressure` (manager ruling R1 under the owner's T3/T4 delegation): its header requires the evidence object, its table binding is the exact identity's table (pressure-2's since T4-U4b/U5b), `read pressure` runs `validate_pressure_evidence` with R2's model checks, and its standing is that verdict plus the ordinary `numerical_quality` checks. The desktop and the Python product paths take this one verdict and standing; both interim names and the TypeScript header check are retired, and the test-only verifier stays (it reads the route independently in `tests/verifiers/reader_verdict.py`). The exact identity has no transport (metadata-only) reader: a transported statement is refused by name (`SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED`). Since B7b, the exact identity (pressure-2 since T4-U4b/U5b) and pressure-source-1 take over the retired v2 identities' Python capabilities: the 0.3 AnalysisRun builder and validator admit both (`CURRENT_RECORD_CONTRACT_IDS`), and the stress-neutral package carries both, an exact package being read through its source because the exact identity has no transport reader. The desktop builds Python's pressure-2 record after the route's reader passes, so an exact session solve completes and its result is viewable; holding a record grants no output by itself. Since epoch 8 the desktop's result JSON export, stress-neutral export and rule check admit pressure-2 and pressure-source-1 at the Rust reader's numerically eligible standing, and every other desktop output surface refuses them by name (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`); the owner's "Merge with the gap" (2026-10-15) and the epoch 8 placement (2026-10-20) are recorded in [desktop-exact-outputs.md](desktop-exact-outputs.md).
- **Authoring writes v3 only.** `pressure_profile` writes `3.0.0/exact_pressure_v3` (0.3.0, or the contract only on 0.4.0) and refuses a v2 write (`OP-PRESSURE-PROFILE-SUCCESSOR-REQUIRED`); a v2 document stays solvable and is upgraded only by selecting the profile again. Blank 0.4.0 documents declare v3. Since B7b, `pressure_profile` is authored only on model 0.5.0 (`OP-PRESSURE-PROFILE-SCHEMA-VERSION-LOCKED`), writes the closed contract with an explicit `shear_model` (`timoshenko_annulus_energy_v1` by default since T4-U8) and requires an explicitly entered `corrosion_allowance` on every pipe section (`EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED`); no v2, 0.3.0 or 0.4.0 document remains solvable. Regions take signed pressure under v3, refuse a geometry-only bend's span ("realize this bend") and check each realized-bend end against its region neighbour with CB's `kink` and α_tan, as the solve does.
- **Bends (T4-I11 D-A, D-B, D-G).** `insert_bend_at_corner` places the tangent points exactly (L_t = R·tan(Φ/2)), shortens both straights, adds the arc span with y_reference = u_in and a realized bend carrying the user's k, or converts a marker at the corner in place. `bend_realization` writes the solver consumption, k and its source on a span bend whose arc forms, refusing corner markers, an inconsistent angle and an end kink above α_tan at a single-neighbour end. An agent never writes k. Bend creation no longer takes `bend_plane_orientation`.
- **Desktop authoring.** The inspector realizes bends and inserts them at corners; one objective-connector form, offered on v3 documents only, replaces the legacy four-rate joint forms. Connectors refuse arc spans.
- **Export chain.** The exact identity (pressure-2 since T4-U4b/U5b) has results, stress-neutral and analysis-run schema branches (placed before the retained successors; its results branch and definitions are held in `schemas/results_pressure_2.v0.3.schema.json`, which `results.v0.3` references, so the Rust reader's embedded `results.v0.3` stays at the other semantics' size), a fresh identity and standing in the Rust reader, whose `pressure` verdict and standing the Python product paths and the desktop take (Reading pressure-2, above); `contract_evidence` is copied whole; connector rows carry one stable basis and export as quantities. HR exports v3 documents, and names an ordinary source the reader refuses.
- **First usable path.** The L line authored from its corner through the panels equals the applier's document (`fixtures/model_operations/pressure_v3_l_line_authoring.json`); it solves under pressure-1 in both modes in PP and HR and agrees with T4-I7's hand-placed document.
- **VP-STATIC `exact_pressure_1`.** The transport `exact_pressure_1_cli_1.0_raw0.2` (`tools/validation/qualification_exact_pressure.py`) scores the package `validation/qualification/fixtures/exact_pressure/` (168 cases from the frozen T4-I7 and T4-I8 references, both modes). Its owned-reader check takes the Rust reader's verdict, `openpipestress-runner read pressure` (binding `openpipestress.exact_pressure_consistency_binding/2`, which selects `physics_evidence.rs` with the table and unit authority), through the current pressure route, as the physics and load-reference qualification adapters do (b7-plan R2); no module outside `tests/` imports a verifier. The independent review T4-RV28 admitted it (`ADMISSION.json`; admission changes statuses only). The CLI gate run is not required: DEC-025 no longer binds.

## Tees and pressure junctions (RT-A3 increment 1)

**Decision D-1 (Agent 0, 2026-10-30, under the owner's OC-2(a) ruling that EX is extended).** A pressure junction is recognised implicitly: it is a node where terminals of two or more regions of a case meet, each `transfers_to_wall`. The model format does not change, and the R-4 design's explicit `closure_transfer: "junction"` token is not adopted ([route-unification-r4.md](route-unification-r4.md) §3.1, §6.3 D-1). The reason is that the two representations are numerically identical. Each `transfers_to_wall` cap is p·A_i along its member's outward axis −s_k, so the caps at the node sum to the fluid force on the fitting, −Σ p·A_i,k·s_k, whatever the fitting's angles or bores. An explicit token would only label the node; it would not change a load. The implicit form needs no 0.9.0 format step and no authoring change. The regions' terminals are already explicit.

**Rules, as built.**
- A region is one chain between its two terminals. An internal node may not touch an outside pipe (`PRESSURE_REGION_INTERNAL_BRANCH`). So a pressurized tee is authored as one region per run, each ending at the tee node.
- A node where two or more regions end, with every terminal there `transfers_to_wall`, is a junction. Its caps enter the ledger as the existing `terminal_cap` summands with their K-D5 manifests. The region evidence records each terminal's cap at the node (`closure_pressure_load_global_n`, `pipe_cap_transfer_global_n`), so the junction resultant is the sum of the node's terminal entries.
- A `separately_supported_or_compensated` terminal at such a node is refused by name (`EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED`, `pressure_runtime.rs`; §8.30 correction 1). This is because it would omit its member's share of the junction load. The refusal names the case, the node and the compensated regions. It also applies where the omitted term or the resultant is zero or cancels, such as a zero-pressure compensated branch or equal opposed compensated caps. It is no longer temporary: under D-1 a compensated terminal cannot be part of a junction.
- Unequal pressures at the node, or an incident pipe outside every region, are admitted as a closure at the fitting: a blind or check valve between two pressures, or a closed branch. The caps then give that closure's fitting load exactly, by the same sum. Because either is more often a modelling slip (a wrong region assignment or a forgotten branch) than an intended closure, each carries its own named warning (Agent 0, 2026-10-30), emitted once per solve entry beside `validate_profile` (`pressure_runtime::junction_closure_warnings`). The warning goes into the result diagnostics and from there into the report's diagnostics section (`renderableReportInput.ts` `diagnosticsForSections`). The person confirms the closure is intended; the numbers are those of the closure.
  - `EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER` (warning). Affected refs: the case, the node and every region ending there. The message lists each region's pressure as authored. Pressures compare in canonical Pa, so 10 MPa and 1e7 Pa are equal.
  - `EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS` (warning), once per such pipe. Affected refs: the case, the node and the pipe. The message names the regions that end at the node. Region membership counts member pipes and connector-replaced spans. A single region's closure beside unpressurized pipes is not a junction and carries no warning.

  This departs from R-4 §3.1's proposed `PRESSURE_JUNCTION_PRESSURE_DIFFERS` and `PRESSURE_JUNCTION_MEMBER_UNREGIONED` refusals, which would have refused those closures under an explicit token. `rt_a3_tee_references.rs` checks that each warning appears with its refs, and that the numbers are the closure's (T1's legs by linearity in p: the branch elongation halves at half its pressure and is 0 when dry; the anchor carries 0).
- The node's angles are not restricted. A two-member node of straight pipes in two regions has exactly the caps' resultant, as on main before this increment. A realized bend's ends inside one region keep their tangency rule (`PRESSURE_REGION_MITRE_UNSUPPORTED`), and a straight–straight kink inside one region stays `PRESSURE_REGION_NONCOLLINEAR`. No mitre flexibility or SIF is modelled.

**Release limitation LIM-002 (exit criterion 3).** The compensated-terminal refusal above is a standing refusal in a Must row. It is listed as [LIM-002 in `docs/RELEASE_LIMITATIONS_v1.0.md`](../../../../docs/RELEASE_LIMITATIONS_v1.0.md), with the reasoning here. The owner's acceptance is pending.

**The tee on EX.** `exact_admission::admission` admits the branch family (`branch`, `tee`, `branch_connection`) under `3.0.0/exact_pressure_v3`. As on the pressure-free route, the tee is a record at its node: its members meet as a rigid joint, there is no element, and branch flexibility is not assembled (RT-K2's Should).
- **Intensified rows.** The user's header and branch SIFs give `branch_intensified_bending_stress_v1` rows, one per section basis, nominal then reduced (`u4b_rows::append_branch_intensified`). They go at the end of the named header pipe and the named branch pipe at the tee's node, by the pressure-free route's association rule (`preview_physics::associations`).
- **Value.** RN(1e6·`norm2`(RN(i·σ_z), RN(i·σ_y)))) Pa, formed from that end's and basis's published bending stress rows σ_y and σ_z (MPa), which is i·hypot(M_y, M_z)/Z_basis. A reader recomputes it bit for bit.
- **Record.** Each row's `intensified_measures` record carries `pipe_id`, `result_id`, `location`, `component_id`, `factor_role` (`branch_header` or `branch_branch`), `sif`, `sif_source_reference` and `section_basis`.
- **Not a code stress.** The rows are never combined (`COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE` on a combination) and are not code stresses.
- **Diagnostics.** As on the pressure-free route, the model-level diagnostics are `COMPONENT_INTENSIFICATION_SIF_NOT_SUPPLIED` (info), for a named side without a positive SIF, and `COMPONENT_INTENSIFIED_COVERAGE_INCOMPLETE` (warning), for a straight pipe ending at the tee that is neither its header nor its branch reference. A realized arc named by a tee carries no row, as on the pressure-free route.
- **Retained routes.** pressure-source-1 and W1-E decline any document with a component, unchanged.

**Readers.** The pressure-2 readers (Rust `u4b_evidence::branch_records` and `branch_model_checks`, and the Python mirror in `tests/verifiers/`) check the following:
- each branch record binds one row of its member, end, basis and case;
- the value is recomputed bit for bit;
- every model tee's associated ends have both bases' rows in each solved case, with the model's factor;
- no other branch row exists.

The pressure-2 table carries the two signatures (`apply-branch` in `tools/serialization/semantic_contract_tables.py`), and its admitted-families limitation states the tee and the junction. The v3 formulation identity text states them too (`exact_admission.rs`).

**References and evidence.** The frozen references are `validation/references/rt_a3_tee/`: eleven cases, two independent exact formulations, and R-4's T-1, T-2 and T-4 values reproduced. The product binding, `core/product_physics/tests/rt_a3_tee_references.rs`, solves the ten admitted cases in both solver modes at 1e-9 relative. It covers nodal motions, reactions, member end actions, the wall, effective and membrane axial values, and the tee rows on both bases with nonzero mill tolerance and corrosion allowance. It also covers the dropped-cap control and the compensated-terminal refusal. The step reaction depends on the far ends: it is zero with free capped ends (R-4 C-3), −(1−2ν)p·ΔA_i with anchored ends, and −p·ΔA_i with compensated ends.

## Direction magnitudes of arc tangents

**Defect (Agent 0, 2026-10-31).** The RT-U4 linear-ex rack has 82 nodes (#31's generator, `--profile linear-ex --nodes 82`). Its 90-degree bend RK-H0022 has chord (0.405, 0.405, 0) m, R = 405 mm and y reference +y. In every case it published `direction_component_magnitude` = 1 + 2^-52 in the pressure groups at the bend's end. Those groups are the `kink_remainder` and `bend_cap_removed` terms on the arc. The Rust and Python pressure-2 readers required the magnitude to be at most 1 and refused the envelope (`SOURCE_PHYSICS_RHS_DIRECTION`). The desktop then reported `SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED`, and Record Run was unavailable. Valid results were refused, so this was a wrong output.

**Where the value comes from.** A group's magnitude is the absolute value of one global component of the direction the producer's pressure term uses (`pressure_runtime::add_factor_term`).
- **Straight member:** the direction is the member's frame axis: the chord scaled by a rounded reciprocal of a norm that dominates every component. Each component is therefore at most 1. The independent review's 400,000-trial search found no exception.
- **Realized arc:** the direction is an end tangent from `curved_bend::arc_geometry`, formed as t = c·d̂ ± s·n̂ and not renormalized. A component can round above 1. The rack bend shows the mechanism exactly: at 45°, c, s, d̂_x and n̂_x each round near 1/√2, and the x component rounds to 1 + 2u (u = 2^-53).
  - At ordinary scales, the review's search over 330,000 trials found at most 2 ulps.
  - No small universal ceiling holds for `arc_geometry` alone. A subnormal y reference, which the frame element's absolute axis guard rejects before an arc is formed, can give components far above 1 through the separately rounded Gram–Schmidt normal.
- **Joint-area group:** the direction is an authored connector axis, kept unchanged. The frame kernel's orthonormality guard admits a component of 1 + EPSILON.

**Decision: the reader binds the value exactly; the producer is unchanged.**
- **Why not the producer.** Renormalizing or clamping a tangent would change the operands the arc's pressure terms use, or else make the published value disagree with them. No bit-preserving correction exists.
- **Why not a bound.** The first draft admitted arc groups up to 1 + 16u. The independent review (codex xhigh) showed that this ceiling was not rigorous for the construction in general.
- **What the readers check instead**, Rust `physics_evidence.rs` and Python `pressure_2.py` alike:
  - **Arc terms:** a term on a realized arc of its region binds the group's magnitude exactly, bit for bit, to the group-axis component of one of that arc's published end tangents (`arc_end_tangents_global`, the operands the producer used). The arc set comes from the region's geometry entries marked `realized_arc`.
  - **Straight groups:** a group with no arc term keeps the bound 1.
  - **Joint-area groups:** the bound 1 is dropped. They were already bound exactly to their connector axis (`RHS_JOINT_DIRECTION`), so an authored axis component just above 1 is no longer refused.
  - **Composite:** the pressure-source-1 reading never carries an arc or a connector and keeps the bound 1.

**Regression.**
- `core/product_physics/tests/arc_tangent_direction_magnitude.rs` rebuilds the rack bend between two straights in one pressurized region. In both solver modes it asserts:
  - the 1 + 2^-52 magnitude, on the arc's groups only;
  - that the Rust reader and the model-bound verdict accept the envelope (on the previous reader it fails with `SOURCE_PHYSICS_RHS_DIRECTION`);
  - that a magnitude one ulp off the arc's tangent component, and 1 + 2^-52 on a straight-only group, are refused.
- The Python verifier tests are `tests/test_pressure_arc_readers.py` (`test_arc_tangent_magnitude_binds_to_the_published_tangent`, `test_a_straight_group_above_one_is_refused`, and `test_an_arc_tangent_component_above_one_is_admitted`). The last reads the maintained sparse envelope `fixtures/results/pressure_arc_tangent_above_one_sparse.json`, which the Rust regression writes with `ARC_TANGENT_WRITE_FIXTURE=1` and otherwise checks for currency.
- The joint tests are `a_joint_axis_component_just_above_one_binds_exactly`, in Rust `connector_contract.rs` and in Python `test_pressure_connector_readers.py`.

**Review.** Two independent reviews (codex, xhigh) were held. The first refused the 1 + 16u bound (above) and the joint bound of 1. The second found no blocking defect and asked for a positive Python case above 1, which the maintained fixture supplies. Retained pricing at the PR head is coefficient-equal to the registered profile (1,586 equal, prune and resolutions unchanged), so nothing is re-registered.

## Load cases that apply nothing

A load case whose authored loads are not applied is refused by name; it is
never solved and published as an all-zero result as if its loads had been
solved (Agent 0, 2026-10-23: a silent wrong result is the worst failure class
for this product). At main `9ad5d79fa5` the first and third cases below
published all-zero rows marked `NUMERICAL_INTEGRITY_CHECKS_PASSED`.

**An action** is an applied primitive load, an equivalent-static load, a
non-empty pressure-region list, a boundary motion, a member state of nonzero
eigenstrain, or a model-level preload. **A model-level preload** loads every
case: a constant-effort support, a tie rod with a nonzero reference extension,
or an objective connector with K q_ref ≠ 0, decided exactly as the kernel
decides a stress-free connector. A connector whose `q_ref` lies in the null
space of its stiffness (K may be positive semidefinite), zero `q_ref`
included, loads nothing, whatever its declared reference state.

- **Load-state case whose stored primitives are all unlisted (EX):** a stored
  primitive is applied only when the case's `analysis_state.load_sources`
  lists it. A case that stores primitives, lists none of them and applies no
  other action is refused with `LOAD_STATE_CASE_APPLIES_NO_ACTION`, which
  names the unlisted primitives. A case already refused by another finding is
  not judged. Leaving a primitive unlisted beside an applied action remains
  legal (the contract's "omit the source to exclude it") and is reported with
  the warning `LOAD_STATE_SOURCE_EXCLUDED`, naming up to six primitives.
- **Reference states (EX load-state):** a case that stores nothing and applies
  nothing is a reference state, for example the cold state of a
  cold/hot/return sequence. Its zero result is correct beside cases that
  apply actions, and it is neither refused nor reported. A model whose every
  case is a reference state publishes only zeros and is refused with
  `LOAD_INPUT_MISSING`.
- **Empty case among loaded cases (PF and EX):** a case with no primitive
  load, no equivalent-static load and no non-empty pressure-region list
  publishes an all-zero result. It may be deliberate (a cold operand of a
  range) and is visible where it is authored, so it is reported with the
  warning `LOAD_CASE_APPLIES_NO_ACTION`, not refused.
- **Every case empty:** an explicit `pressure_regions: []` means unpressurized
  and applies nothing, so an exact model whose cases are all empty is refused
  with `LOAD_INPUT_MISSING`, as a pressure-free one already was.
- **Models with a model-level preload:** the per-case warning does not apply.
  `LOAD_INPUT_MISSING` keeps its earlier rule there: some case must carry a
  primitive load, an equivalent-static load or a pressure-region list. So a
  pressure-free model loaded only by a constant-effort support is still
  refused, as it was before this change (independent review, round 2: a
  pre-existing gap, recorded for follow-up, not changed here).

Evidence: `core/product_physics/tests/silent_zero_guard_runtime.rs`, in both
solver modes. Its refusal and warning tests fail at `9ad5d79fa5`. Its controls
pass: the loaded case alone, a boundary-motion-only load-state case, a
reference state beside a loaded case, a load-state case whose preload acts
with its primitive unlisted, and a preloaded connector exempting an otherwise
empty model. A connector with a null-space `q_ref` exempts nothing: the
exact all-empty model, the load-state case with its primitive unlisted and
the reference-states-only model are each refused. `stress_maximum_coverage.rs`'s exact-zero tie tests now load the
anchored node, whose load goes wholly to the reaction, instead of solving an
unloaded model.

## Pressure-free input closure

Authored input on the pressure-free route's model records is never dropped
silently either (Agent 0, 2026-10-23, as above). Every model record is closed,
and so is every material record of the request; the combination reader names
every row it does not publish.

- **Closed records:** these deny unknown fields:
  - the model root (`PreviewModelWire`);
  - a load case, with its `modulus_basis_temperature` and `equivalent_static`
    inputs (seismic, wind and its exposed spans);
  - a primitive load, with its target and magnitude;
  - a combination and a combination term;
  - a node, with its position;
  - a pipe segment, with its section, the section's quantities and its
    `y_reference`;
  - a section of the library, with each property's quantity;
  - a support, with its stiffness and value, its hanger (with the hanger's
    stiffness and quantities) and its nonlinear input (with its quantities
    and `normal_reaction_source`);
  - a component, with its geometry (quantities and `center_of_gravity`), its
    modifiers' quantities, its mechanics interface and the objective
    connector's own members;
  - a material of the model or of the request, with its quantities and its
    temperature points and their quantities.

  The value entry scans the actual request first, before any route, and
  refuses an undeclared key with `UNKNOWN_MODEL_FIELD`. It names each key with
  its record (by id, or by index without one) and path, for example
  ``unknown field `mill_tolerence` in the section of pipe segment `pipe:P-110`
  (model.pipe_segments[1].section)``. The scan keeps at most eight findings,
  forming each only while it is kept, and counts the rest (`; and N more`);
  nothing is solved. Every typed reader refuses the same keys. Quantities and
  vectors are closed where these records carry them; the pressure, case-state
  and settings records keep their own readers. The objective connector stays
  an authored value read by key. Its own members are checked against its
  record's key list; its member records are the connector decoder's, on the
  exact route.
- **Records that are not objects:** serde's derives also read a record written
  as an array of its members in order, whose keys cannot be checked. Before
  the key scan, the value entry refuses any element of a model record
  collection (load cases, combinations, nodes, pipe segments, sections,
  supports, components, materials) or of the request's materials that is not
  an object, with `MODEL_RECORD_NOT_OBJECT` and its path, for example
  `model.nodes[0] is an array (a positional record)`, at most eight named and
  the rest counted. Main read and solved a positional node of two or three
  members (id, position, provenance) and refused a longer one, a null or a
  scalar in the typed parse. Only those readable positional forms, which are
  outside the schema, change outcome.
  Records inside these (loads, terms, temperature points, member records) are
  left to their typed readers.
- **Material records:** the material record keeps its flattened wire, so every
  outcome but an unknown key's is unchanged. A duplicate `label`, `tags`,
  `notes` or `provenance_record` is accepted as before. An annotation value the
  wire refused, such as `"notes": 1e400`, is still refused. A duplicate known
  key is still refused at its position. The record's other keys are collected,
  and any but the four annotations is refused as an unknown field, in the text
  reader and the value entry alike. `mod material_wire` in
  `src/silent_drop_unit_tests.rs` keeps a copy of main's wire and compares
  outcomes from the text at model and request level. It runs regression cases
  (duplicate annotations, an overflowing annotation, a duplicate known key,
  field order, null against absent, positional forms) and, with
  `MATERIAL_WIRE_CORPUS` set, every committed material record: 807 records in
  332 files, 694 read the same and 113 are refused by both, with no record
  refused for an unknown key only.
- **Self-weight evidence:** a generated self-weight record's `mass_inputs`
  rebuild a closed pipe section, and its `normalized_dependencies` declare one
  member beside the section's own, `section_ref`, removed before the section is
  read. A member either does not declare, such as a misspelled input or an
  extra key in a quantity, makes the evidence Invalid rather than Fresh.
- **Declared and ignored:** members a document may carry that the solve does
  not read are declared on the closed records and accepted unread: on the
  root `data_boundary`, `diagnostics` and model document 0.7.0's `lines`,
  `unresolved_assumptions`, `constraints`, `equipment_interfaces` and
  `rule_pack_refs`; on a load case `kind`, `label`, `status`, `tags` and
  `notes`; on a combination `tags` and `notes`; on a node and a pipe segment
  `label`, `tags` and `notes`; on a section `tags`, `notes` and
  `provenance_record`; on a support `label`, `boundary_association`,
  `properties`, `tags`, `notes` and `provenance_record`; on a component
  `completeness`, `tags`, `notes` and `provenance_record`; on a material
  `label`, `tags`, `notes` and `provenance_record`. `model_settings` is a typed
  member. Each closed record's accepted key set is exactly its record's key
  list in `schemas/model.schema.yaml` (0.7.0), bound by `model_fields.rs`'s
  key tests, which also bind the scan's quantity and vector members to the
  schema's.
- **Combinations:** a `mechanics` term naming a load case twice, or a
  `range_envelope` naming an operand twice, is refused with the blocking
  `LOAD_COMBINATION_DUPLICATE_TERM`, once per repeated case at its second
  occurrence, so each diagnostic id is unique however often a case repeats. A row missing in the
  first operand is reported as `LOAD_COMBINATION_SOURCE_RESULT_MISSING`, as
  for any other operand. A row the load-case algebra refuses or that has no
  algebra dimension, and a support action group that cannot be formed
  whole, are withheld under one `LOAD_COMBINATION_ROW_WITHHELD` warning per
  combination, naming up to eight; a support's group publishes all its rows
  or none.
- **Outside this closure:** the request's own members besides its materials,
  and the project record. The records of the pressure contract, the
  load/reference-state group (a material's `expansion_laws` among them) and
  `model_settings` keep their own readers; the scan does not name their keys. The objective connector's member records (its
  ends, stiffness, calibration and the rest) are read by key on the exact
  route; an unknown key there is not checked.

Evidence: `core/product_physics/tests/silent_drop_guard_runtime.rs` and
`src/silent_drop_unit_tests.rs`, both solver modes; the annotated document
(`tests/annotated_model_document_solve.rs`) solves byte-identically
with and without its declared members, and the demo does with its nested
records' declared members varied or removed. The desktop refuses a hand-edited
nested key at solve with that reason. Priced changes: DEL-04-08
[retained-pricing-rules.md](../../DEL-04-08/Work/retained-pricing-rules.md)
§8.19, §8.25 for the nested records (the tree is coefficient-identical to
§8.24's), and §8.26 for model document 0.8.0 and the reducer's closed
`eccentric_offset` (coefficient-identical to §8.25's).

## Constant-effort combinations: the unit factor-sum rule

A `mechanics` combination superposes its case states. A consuming
constant-effort support's fixed load is in every case state, so the combined
state counts it Σf times. The combined state is a state of the original system
only when Σf = 1. Where the model has such a support,
`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` withholds every mechanics
combination whose factors do not satisfy the unit factor-sum rule (option C,
Agent 0, 2026-10-25):

- **Rule:** Σf is the exact dyadic sum of the binary64 factors. The combination
  is admitted iff |Σf − 1| ≤ 64ε = 2⁻⁴⁶ (about 1.42e-14), compared exactly. There
  is no Σ|f| scaling. A non-finite factor is refused by name: validation's blocking
  `LOAD_COMBINATION_FACTOR_INVALID` refuses it first, and the gate also withholds
  it under the same reason code. An accumulator error is withheld the same way.
- **Why:** the rule it replaces admitted |fl(Σf) − 1| ≤ 64ε·max(1, fl(Σ|f|)), and
  the hole was the relative scaling, not the tolerance. 2⁵² and −2⁵² sum exactly
  to 0, but the bound scaled to 128, so the combination was published with the
  constant effort counted zero times. A fold that overflowed to infinity was also
  admitted, because inf > inf is false. With an exact sum and an absolute bound:
  - 2⁵³/−2⁵³ gives |0 − 1| = 1 and is refused, and an exact sum cannot overflow;
  - 0.3 + 0.7, 1.2 − 0.2 and fl(1/3) + fl(2/3) each sum exactly to 1 − 2⁻⁵⁴ and
    are admitted, as before;
  - no decimal formatting is involved, so there is no Rust/TS formatting-parity
    risk;
  - what it admits scales the fixed loads by at most 1 ± 1.4e-14, below solve
    precision.
- **Rejected:** (A) exact Σf = 1, which withholds 0.3 + 0.7 and 1.2 − 0.2. (B) an
  exact rational sum of each factor's shortest round-trip decimal = 1, which
  refuses fl(1/3) + fl(2/3) (its decimals sum to 0.9999999999999999) and needs a
  formatting step.
- **Where:** the one implementation is DEL-05-02's
  `open_pipe_stress_load_case_algebra::factor_sum_within_unit_tolerance`, so other
  gates that need Σf = 1 can reuse it. `preview_physics.rs` `gate_reason` calls
  it. The test-only Python verifier mirrors it with exact rationals
  (`tests/verifiers/preview_physics_evidence.py`). The preview contract tables'
  `combination_policy` states the same condition.

Evidence: the helper's tests (both bounds closed; 1 ± (2⁻⁴⁶ + 2⁻¹⁰⁷⁴) refused),
`src/preview_physics.rs` `constant_effort_gate_withholds_a_factor_sum_it_cannot_evaluate`,
and `tests/preview_physics_runtime.rs`
`constant_effort_gate_uses_the_exact_unit_factor_sum`, which covers both solver
modes. Factors of magnitude 2⁵³ or more are integral, so the value entry refuses
them (`CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`), and only the typed entry reaches the
gate with them.

## Imposed displacement on the pressure-free route

Owner, 2026-10-23, verbatim: "Row 2 I approve option A". Imposed
displacement (support and anchor movement, including nozzle movements) is
built on the pressure-free route in two stages; no "the exact route is the
only route" statement is made.
- **Stage 1:** a support target for the primitive load, mapped into the
  partitioned reduction of prescribed values the exact route already uses
  (`boundary_motion`). It is refused on a non-rigid DOF or a spring, and,
  until stage 2's R-a, refused by name when the model also has nonlinear
  supports. PF benchmarks:
  the W-9 L-frame settlement against its closed form and a nozzle movement.
  The desktop `imposed_displacement` primitive solves where supported and is
  refused with a named, readable reason elsewhere. Priced change: D3
  re-registration with independent review.
- **Stage 2:** imposed motion in a model with nonlinear supports, in
  separable parts. (R-a) Rigid anchors and restraints carry stage-1 primitives
  as nonzero base prescribed values in the nonlinear active-set loop and its
  K-D5 confirmation; every nonlinear support's ground stays fixed. (R-b) A
  primitive on a nonlinear support's own DOF moves that support's ground: gap
  closure, lift-off and friction slip are measured on the displacement relative
  to the moved ground, and an active or sticking state prescribes the moved
  ground (plus or minus the clearance for a gap). (R-c) A primitive on a
  linear spring's DOF moves its base; the case gains k·g and the spring
  publishes −k·(u−g). (R-d) A primitive on a mixed nonlinear
  record's rigid restraint prescribes that restraint's base value. Design:
  DEL-04-03 `Work/pf-nonlinear-imposed-motion.md`,
  with dual numerical review and its own benchmarks; R-b may land as a later
  slice.
Stage 1 is built (W-16 slice a; DEL-04-03 Design `pf-imposed-displacement.md`).
On the pressure-free route a support-targeted `imposed_displacement` primitive
is one prescribed value on its rigid support's global DOF, solved by the
shared partitioned reduction in both solver modes; a case without one keeps
its unchanged path. Each misuse is refused by name before generic validation
(`IMPOSED_DISPLACEMENT_*`), and any refusal withholds the whole document. The exact route keeps
`boundary_motion` as its form and refuses a support-targeted primitive by name
(`EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED`); the retained sub-route
declines it at D1.7. `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL` is
retired. Evidence: `core/product_physics/tests/w16_imposed_motion.rs` (the
nozzle movement against `validation/references/w16_imposed_motion/`, parity
with EX) and W9-C on both routes (`tests/w9_solver_benchmarks.rs`).
Stage 2 is built. In a model with nonlinear supports, a stage-1 primitive on a
rigid anchor or restraint is a base prescribed value of the nonlinear loop and
its K-D5 confirmation (R-a), and a primitive on a nonlinear support's own DOF
moves that support's ground (R-b): its state is measured on u_d − g_s, an
active or sticking state prescribes the moved ground, and the support publishes
a `nonlinear_support_final_relative_displacement` observation row. The
confirmation checks the loop's prescribed map against the case's motion bit
for bit. `IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS` is retired; the R-b
refusals are `IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH`,
`IMPOSED_DISPLACEMENT_GAP_CLOSURE_INEXACT` and the extended
`IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT`. A primitive on a mixed nonlinear
record's rigid restraint prescribes that restraint's base value (R-d). Still
refused (`IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID`): a constant-effort record,
a nonlinear field included. A primitive on a linear spring's DOF moves its
base: the case gains the formed load k·g and the spring publishes
−k·(u − g) (R-c). Evidence:
`core/product_physics/src/w16_nonlinear_motion_tests.rs` against
`validation/references/w16_nonlinear_motion/` (A-1 to A-3, B-1 to B-3, C-1 to
C-5, D-1 to D-3 and the rigid-translation property, both modes).

## Exact temperature identity on the load/reference-state route

The resolver decides each member's temperature equality and order (material
point selection, bracketing, the installation temperature and expansion-law
tables) on exact values, not on binary64 affine conversions
(`case_state/temperature.rs`). Each authored absolute temperature becomes the
exact kelvin value N / (9·10^24) of its canonical decimal under the unit's
exact affine definition (K, degC, degF, degR). The canonical decimal and the
integer form are DEL-02-01 Design `model-document-0.9.0.md` §1.5.1's: the
shortest decimal that parses back to the authored binary64, the one closer to
it when two of that length do, and the even one when they are equally close
(the rule of Python's `repr` and ECMAScript's `Number::toString`; Rust's
`{:e}` takes the other candidate on such 16- or 17-digit ties). It is computed
on integers, never by a float formatter, in
`case_state/temperature/canonical_decimal.rs`, the implementation C1's future
lowering shall reuse.
- **Domain:** |value| ≤ 10^6 in the authored unit, with a canonical decimal
  exponent e ≥ −24 (§1.5.1's admitted operand range). Normalization runs
  first: a consumed quantity it cannot convert, a non-finite one or one below
  zero kelvin is refused with `LOAD_STATE_QUANTITY_INVALID`, whatever its
  magnitude. An otherwise valid consumed absolute temperature outside the
  domain is refused with its own blocking code,
  `LOAD_STATE_TEMPERATURE_RANGE_UNSUPPORTED`. Called directly, `exact_kelvin`
  maps a non-finite value or a negative exact kelvin to
  `LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED`. Inside the domain |N| < 10^31, so
  equality and order are integer comparisons that cannot overflow.
- **Representatives:** exactly equal temperatures share the binary64 kelvin of
  the first one registered. Distinct ones stay distinct, and a member whose
  representatives do not strictly increase with the exact order is refused
  (`LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED`); there is no tolerance
  snapping.
Evidence: the unit tests of both files, including the tie regressions and a
corpus of 4549 binaries against Python's `repr`
(`tools/fixtures/temperature_canonical_decimal_corpus.py`), and the solve-level
refusal in `tests/load_reference_state_runtime_extension.rs`.

## Concentric reducer pressure-bore step (RT-A3 increment 2a)

The exact route admits a mapped reducer in pressure only in this initial
representation: its body is the single member of its own region, and each
body end joins exactly one straight member belonging to another region of
the same case. Every region terminal at either shared node transfers to the
wall. A tee record, bend/expansion-joint neighbour, connector-replaced span,
branched end or dry/free body end lies outside this increment. A multi-member
body region remains `EXACT_PRESSURE_REDUCER_IN_REGION`, including an equal-bore
region. A reducer outside every pressure region keeps its existing admission.

**Entering the step.** Map the reducer to its existing body pipe, which must
end at the component's node. Give that body its own region and split the
adjacent regions at its ends; retain their explicit `transfers_to_wall`
terminals. Actual pipe sections supply the bores. End-size metadata supplies
no bore, taper law or mechanical load. W-15 eligibility supplies the user's
stiffness scale; a marker or incomplete scaling input keeps the existing
unscaled behavior and warnings. No model version, closure token or authoring
operation is added.

**Concentricity and remaining domain.** The normalized body and neighbour
directions leaving each shared node must be opposite (negative dot product) with
cross-product norm at most `64*f64::EPSILON`. This reuses the representation
guard, a numerical agreement rule rather than a geometry fit allowance.
The guard is an absolute bound on the normalized chord directions. Binary64
coordinate rounding can exceed it for an intended collinear skewed run with a
short body far from the origin: the review found a 0.5 m body around 500 m,
and a 0.1 m body around 100 m, can be refused. Millimetre input normalization
can expose the same effect. Axis-aligned coordinates need not exhibit it.
Such a refusal reports representation disagreement; it does not establish
physical eccentricity. This initial slice does not widen that bound or claim
translation-invariant skewed-run admission. A coordinate-aware agreement
design needs separate review, and no release limitation is accepted here.

Authored nonzero eccentric offsets remain refused in pressure in 2a; absence
of the slot declares nothing and is admitted only when the actual geometry
passes. Explicit zero retains W-15c's separate offset consistency checks.
The entry checks topology; case assembly checks normalized geometry before
forming pressure loads. Passing profile validation alone is not a solve.

D-1's general closure/junction rules remain in force. Unequal pressures at an
otherwise admitted step produce the existing closure warning and the caps'
actual resultant. A compensated shared terminal remains refused. The initial
step guards do not alter junction-angle or dry-branch behavior elsewhere.
None of these remaining refusals is declared an acceptable release limitation.

**Mechanics and theory.** Each region already places its cap at a shared node
as `-p*A_i*s`, where `s` points into that member. Their sum is the fitting load;
for a reducing collinear step it is `p*(A_i,left-A_i,right)` along the run. No
second thrust term is added. Pressure areas use the reduced bore once. For
a scaled equivalent member, `s_scale*E*A_nominal*epsilon = N_wall-2*nu*p*A_i`:
W-15 scales stiffness, while the Poisson pair has no E and is unchanged. Both
published section bases divide that wall force by their own metal area, and
each Lamé result uses its own annulus. Body rows retain `component_body`
reference-section attribution and stay out of the headline; these are not
taper/body stresses.

The frozen independent references in `validation/references/rt_a3_reducer/`
use exact axial compatibility and an independently assembled rational
stiffness equilibrium. With free capped far ends a junction anchor carries
exactly zero; holding all nodes is a different boundary condition. Far-anchor,
scale-2, signed-pressure, orientation and dual-basis cases check the other
meanings. Product tests additionally rotate the run, check meaningful refusals
and validate a complete connector model before asserting its reducer refusal.

Current producer fixtures and their carrier copies are regenerated through
their maintained writers. `tests/test_rt_a3_reducer_text_only_envelope_diff.py`
binds the regeneration to integrated main `2656dcb33`: exact number tokens,
row/member order, diagnostics and selection remain unchanged; only the first
formulation-basis limitation and binding hashes move. The earlier O4/W-15/tee
checks keep their historical endpoints, with this separate stage checking
the current files. Frozen numerical references and recorded 0.7/0.8 model
runs are preserved verbatim.

**Next boundary.** Agent 0's adopted sequence is concentric 2a, bounded
eccentric 2b, then constant effort. RS-R1 remains eccentric and pending 2b;
its +25 mm offset, 270/220 mm sizes, scale 2 and RG-R region are preserved.
Tee/arc/joint-adjacent targets remain beyond the plain-run slice. The existing
caps' total eccentric wrench, its formation and recovered reference-section
meaning need independent 3-D verification in 2b; an extra `p*A*e` term must
not be presumed missing or added twice. No nonlinear or reference-system
readiness follows from 2a.
