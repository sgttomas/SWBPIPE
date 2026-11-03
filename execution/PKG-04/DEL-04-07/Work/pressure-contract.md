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
runs are preserved verbatim. The new stage excludes model/request/input-manifest
hashes and input-scoped digest records from its hash allowance and preserves
the digest encoding. Independent carrier scrutiny also checks source-input
hashes across the generated outputs. RV120's `input_sha256` is explicitly a
digest of its materialized result statement and invocation, not an authored
model hash: that fixture's exact shape field is re-derived and checked at both
comparison endpoints, with the authored invocations required unchanged.

**Next boundary.** Agent 0's adopted sequence is concentric 2a, bounded
eccentric 2b, then constant effort. RS-R1's +25 mm offset, 270/220 mm sizes,
scale 2 and RG-R region are preserved. Tee/arc/joint-adjacent targets remain
beyond the plain-run slice. The bounded eccentric step and its 3-D verification
are in "Eccentric reducer pressure step (RT-A3 increment 2b)" below; no extra
`p*A*e` term is formed. No nonlinear or reference-system readiness follows
from 2a.

## Eccentric reducer pressure step (RT-A3 increment 2b)

Candidate (RT-A3 reducer 2b, on 2a's merge `393a11ee7`). Independent numerical
and literal old/new commitment review checked `0ee639782` with no blocking
numerical finding. The semantic tables and ordinary carriers are regenerated
(identity and binding hashes only, `tests/test_rt_a3_reducer_2b_text_only_envelope_diff.py`);
the retained registration (DEL-04-08 `retained-pricing-rules.md` §8.41, owner
disposition reserved), the retained carriers it gates and Agent 0 integration
remain pending; this is not adoption or release.

**Admission.** The step topology is 2a's and is checked first, unit-free:
the body is the single member of its own region; at each body end exactly one
other pipe meets it, a straight member of another region of the case whose
region has a terminal there; every terminal at both nodes is
`transfers_to_wall`; no tee record, bend, expansion-joint or connector-replaced
neighbour. A body that meets the topology is then admitted by either of two
branches, each judged whole on the normalized geometry:
- **Legacy 2a concentric step, unchanged:** no authored nonzero
  `eccentric_offset` (a marker's offset counts) and, at both ends, the body and
  neighbour directions opposite within the `64*f64::EPSILON` representation
  guard. This branch is not W-15c run evidence: it still admits back-to-back
  absent-offset bodies, whose neighbours W-15c never accepts as run evidence.
- **2b W-15c branch:** W-15c's whole method (DEL-03-05
  `eccentric-reducer-offset.md` §3.2–§3.3, `rigid_component.rs`) accepts the
  body. That means the mapping, ambiguous, unsupported and unresolvable
  evidence, θ ≤ θ_base + β_N + β_F with θ_base = 1e-3 rad (β_arc is part
  of the method, though the topology already excludes arcs), J2's
  better-resolved end, J3's split tie, D·t > 0 and, for a read offset, the
  precision, transversality and rule-B placement allowances, unchanged and
  read through the same functions as the pressure-free check. The run must
  also pass through the body: at each end the neighbour leaves the shared node
  on the far side of the body along W-15c's governing axis t, oriented so that
  D·t > 0. This is a strict sign test with no tolerance. It keeps 2a's
  opposite-direction continuation, because W-15c's undirected axis would
  otherwise accept a folded-back neighbour.

Offset states keep their W-15c meaning, and none is a shortcut:
- **Read offset** (present, not on a `not_solver_consumed` marker): admitted
  only when W-15c realizes it. A failed read offset is refused by W-15c's own
  blocking diagnostic, before any case is assembled. A zero offset is judged
  as zero, so on eccentric geometry it is a mismatch, not a concentric
  declaration.
- **Absent offset or an unread marker offset:** declares nothing. No offset is
  compared and no concentricity is inferred, but W-15c's run evidence and
  D·t > 0 are still required whole.
- **No inclination limit:** none is added.
- **Topology failures:** a failed W-15c branch never falls back to the legacy
  branch, which needs its own collinear geometry. A body with an authored
  nonzero offset beside another reducer's body is refused on topology
  (`EXACT_PRESSURE_REDUCER_IN_REGION`), because only W-15c could admit it and
  W-15c never accepts that neighbour. Other topology failures stay refused as
  in 2a, before W-15c is read.

**Old and new contract excerpts.** The complete paired passages belong to the
2b PR review record, including every changed governing document; these
excerpts explain the compatibility rationale and do not replace those literal
quotations. The independent review above examined the complete Git diff.
- Refusal, old: "…is not the admitted concentric pressure-bore step; …no
  nonzero eccentric_offset is authored (an absent offset declares nothing),
  …continuing collinear in the opposite direction within the 64*epsilon
  representation guard…; a multi-member region, an eccentric or slanted body,
  … is refused". New: "…is not the admitted concentric pressure-bore step or
  an eccentric or slanted pressure step accepted by W-15c; …it is then
  admitted either as the concentric step, when no nonzero eccentric_offset is
  authored … within the 64*epsilon representation guard, or by W-15c's run
  evidence, when the two neighbours' run directions agree within 1e-3 rad plus
  their coordinate-rounding uncertainties, the body has positive length along
  the governing run axis, each neighbour runs on away from the body along that
  axis … and a read eccentric_offset passes W-15c's consistency check…". The
  2a prefix is retained.
- Formulation basis, old: "reducers when their mapped member lies outside
  every pressure region or forms a concentric pressure-bore step, the only
  member of its own region, collinear between single straight members of other
  regions with transfers_to_wall at both ends, whose junction caps carry the
  step's pressure load". New: "…or forms a pressure step, the only member of
  its own region between single straight members of other regions with
  transfers_to_wall at both ends, either concentric and collinear or eccentric
  or slanted with W-15c's run evidence and any read offset accepted, whose
  junction caps at the actual nodes along each member's own axis carry the
  step's pressure force and moment".
- This section, old (2a): "Authored nonzero eccentric offsets remain refused
  in pressure in 2a". New: the admission rules above.
- Compatibility: every 2a admission is preserved, because the legacy branch's
  conditions and guard are unchanged in code and every historical 2a positive
  still binds its frozen reference. 2b only adds admissions. A read offset
  that W-15c refuses keeps a blocking refusal, now W-15c's own code, where 2a
  named `EXACT_PRESSURE_REDUCER_IN_REGION` at profile validation.

**Mechanics and theory.** Nothing new is formed. Each region places its caps,
`-p*A_i*s`, at its actual terminal nodes along each member's own direction, as
before. For an eccentric or slanted body, their sum is the step's pressure
force and their positions supply its moment. No averaged axis is used, no cap
is projected to a declared offset, and no `p*A*e` term is added. The existing
caps are the whole eccentric wrench, and a second term would count it twice.
The other rules carry over from 2a:
- the Poisson pair `2*nu*p*A_i` is unscaled, and
  `s*E*A_n*epsilon = N_w - 2*nu*p*A_i`;
- `S = N_w - p*A_i`;
- the reduced bore is used once;
- each basis divides N_w by its own metal area and has its own Lamé annulus;
- body rows are reference-section rows (`component_body`); their per-member
  maxima remain, and case maxima and headlines exclude them (DEL-03-05 §2.4).
They are not cone, taper or body stresses. D-1 is unchanged: unequal
pressures give the closure warning and the caps' actual resultant, not a
hydrostatic gradient. A compensated shared terminal stays refused.

**Verification.** Candidate references are in
`validation/references/rt_a3_reducer_2b/` (generator `--check` is
standard-library only). They are 27 exact frame cases on the actual RS-R1
neighbourhood, each solved two ways: by K stiffness and by an FM
force/unit-load method. Rigorous 70- to 560-digit enclosures decide each binary64 value.
`core/product_physics/tests/rt_a3_reducer_2b_references.rs` binds every case
through the public entry in both solver modes: Euler-Bernoulli and Timoshenko
at s = 1, 2 and 4, determinate free and compensated cases, the reversed body,
a general mechanical load, the tilted actual nbrN, D-1 unequal pressures, a
rational rotation with far translation, and the C3 held-junction control. The
bound rows are displacements, rotations, reactions, global end wrenches,
N_w, S, wall endpoint actions, station section actions, element stress rows
on both bases (MPa), membrane, Lamé and member-maximum rows (Pa), and the body
record. The criterion is 1e-9·max(|expected|, zero scale), with the packet's
own zero scales. There are 25,968 comparisons; the largest uses 0.043 of its
tolerance. The packet's mutants sit far outside that bound: an extra `p*A*e`
by 3.8e9 tolerances, averaged-axis caps by 3.0e9, a cap projected to the
declared offset by 4.2e8. The same file also runs these checks:
- the 24 W-15c geometry controls, on the pressure-free route and in pressure;
- accepted and just-refused agreement bounds for declared, absent and marker
  offsets;
- bypass mutants: absent, zero and marker offsets with failed evidence, and
  the D·t = 0 body;
- the folded-back neighbour;
- agreement where both branches apply;
- the legacy guard's 64ε boundary on back-to-back bodies.

`rt_a3_reducer_2b_pf_controls.rs` digests 23 pressure-free and outside-region
W-15c controls, compared with their outcomes before the helper extraction.

**Full reference system (candidate).** Independent numerical/commitment delta
review checked `d1ef2eccf` against `ee833b0c5` and found no correction needed
within this scope; it supplies no adoption, accuracy or release authority.
`rt_a3_reducer_2b_reference_system.rs` overlays RS-R1 on the maintained
fixture `fixtures/reference_system/reference_system.json` in the test only.
RS-R1 is entered as DEL-09-01 §5 specifies, and no node, region, support or
case state moves. W is regenerated by the product's own `self_weight`
generator, bound to the overlaid input's hash; the fixture's captured W is
refused as stale (`SELF_WEIGHT_INPUTS_STALE`). The test solves RS-SUS,
RS-OPE and RS-SAM, with RS-EXP and RS-dSAM, through the public entry. It
runs Euler-Bernoulli and Timoshenko, each in both solver modes. In every
run RS-R1 is realized and scaled by 2. Against the unmodified fixture solved
alike, the diagnostics differ only by those two records and RS-P150's
dropped weight-mapping warnings. The same row identities and solve-quality
screens are published, and the reader's verdict is clean. The test derives
loads and constitutive parameters from inputs. Local recovery uses published
motions, and section checks use actions; these are consistency relations,
not independent global displacements or reactions:
- **Weight.** EQ-1, EQ-2 and EQ-5 use the frozen
  `validation/references/reference_system/` totals, less RS-P150's weight,
  plus 300 N at the body midpoint (1e-9).
- **Local formation and recovery.** On the body and both neighbours, the
  published end wrenches equal an independently assembled member stiffness
  times the product's displacements, less the thermal (case modulus), Poisson
  and neighbour weight actions.
- **Equilibrium and sections.** Nodal equilibrium at RS-150 and RS-160 uses
  the caps at the actual nodes plus RS-R1's weight wrench and no other
  moment. The body's stations, stresses and Lamé rows are checked on both
  bases (1e-12 of the summed terms).

Mutants of those checks must miss by over 10³ allowances, and the smallest
observed miss is 1.3·10⁶: s = 1, a scaled
Poisson pair, the reference modulus or no eigenstrain in RS-OPE, a
pipe-weighted body, a `p*A*e` moment, and a run-axis cap. A `rigid`
surrogate with the same body, scale and weight gives bit-identical rows. It
is a product consistency control, not a reference. The test adds no
independent global solution of the 27-node system: the 27 cases above remain
the closed numerical basis, and the full system adds the connecting path.

Row mapping: the element-local force and moment rows at `end_i`/`end_j` are
end-DOF force vectors, the node's action on the member as each row's sign
convention states. They bind to the packet's `end_wrench_on_member_local`.
Interior stations and every stress row use the j-side section action. The
packet README and generated row semantics distinguish these from the
endpoint force vectors; this mapping clarification changes no reference
number.

**Limits.** The determinate cases show force independence of s; an
indeterminate system's response is not generally affine in 1/s, and no
extrapolation follows. The following remain refused or future work:
- tee-, arc- and joint-adjacent steps, reducers in series beyond 2a's
  back-to-back legacy form, multi-member regions, and dry or free ends;
- RS-R1 in the maintained fixture, its W capture and the DEL-09-01 §10
  ledger, which the full-system test above neither edits nor adopts;
- an independent global reference for the full system;
- the retained profile registration (§8.41 candidate) and the retained
  carriers and successor pins, which only a registered build regenerates;
- final integration checks after carrier and registration reconciliation.
The RS-R1 ledger row stays pending until that adoption. The full-system
cases keep the unmodified fixture's `sensitive` integrity screen, and no
accuracy is claimed. No reference-system, nonlinear or release readiness follows.

## Constant-effort supports (RT-A3 increment 3)

**Admission.** `exact_admission::admission` admits the constant-effort
support family on `3.0.0/exact_pressure_v3`, on plain and on
load/reference-state documents ([route-unification-r4.md](route-unification-r4.md)
§3.2, S2). A record carrying a `nonlinear` field stays in the nonlinear
family and keeps its refusal; the record's existing validation still
requires its travel metadata (`CONSTANT_EFFORT_TRAVEL_MISSING`).

**Semantics (DEC-049; `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`).**
The ideal constant-effort element is the one the pressure-free route already
assembles, through the same functions:
- A record is consumed when it declares exactly one translational restraint
  DOF (rotations beside it do not count; every entry must be a frame DOF) and
  a finite positive `hanger.constant_load`. Nothing is defaulted, inferred or
  read from a catalog.
- A consumed record applies its load as a constant nodal force along the
  **positive** global axis of that DOF in every solved case: one plain input
  term in the shared `case_force_ledger` (`add_constant_effort_support_loads`),
  so the dense, sparse and active-set paths consume it identically and K-D5
  closes it with radius 0. It has zero stiffness and adds no restraint row.
- Each solved case publishes one `constant_effort_support_applied_load` row
  per consumed record (`append_constant_effort_support_results`, with the
  user-limit comparison warning), and the record's support-action rows state
  the same force on its axis (support on pipe) and zero elsewhere.
- A record that is not consumed applies no force and no restraint, publishes
  no applied-load row (its support actions are zero) and carries one
  `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning naming the unmet condition
  (`append_constant_effort_consumption_diagnostics`). Its review rows remain.

**D-2: load/reference-state documents (Agent 0, under its engineering-judgment
delegation).** On a load/reference-state document the constant force is a
device action of the active support in every state, not an ordinary primitive
of the case's declared source ledger. The record's support state is
`active_model_device`, the only participation the resolver admits (inactive
and locked states are refused by `LOAD_STATE_SUPPORT_PARTICIPATION_UNSUPPORTED`,
and boundary motion or a device reference on it by the existing state rules),
and the shared ledger applies the force in each state's case. Its force
therefore cancels exactly in O − S (R-4 §3.2, "Interactions"). Held or locked
devices belong to hydrotest (R-4 §4.9, `locked_equivalent_support`) in the
load-case lane and are not built here.

**Combination gate.** A `mechanics` combination over a model with a consumed
record counts the force Σf times. EX now withholds it, as PF does, when the
factors fail the unit factor-sum rule ("Constant-effort combinations: the
unit factor-sum rule" above), with PF's reason
`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` and PF's message
(`exact_combinations::CONSTANT_EFFORT`). Subtractions and range envelopes are
not gated.
- **Order: gate 4, after `COMBINATION_MODULUS_BASIS_MIXED`,
  `COMBINATION_RESOLVED_STIFFNESS_MIXED` and
  `COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE`.** The two stiffness gates
  decide whether the operands are states of one system at all, which no
  choice of factors repairs; the two factor-sum gates presuppose one system
  and decide whether a fixed load is counted once. The constant-effort gate
  shares the installed-reference gate's predicate and follows it, so every
  reason published before this increment is unchanged and the pressure
  table's gate list is extended by appending. PF has no stiffness or
  connector gate and orders this reason before its modulus gate; where both
  hold, the two routes name different reasons for the same withholding.
- **Readers.** The pressure-2 combination pass recomputes the gate
  (`exact_combination_evidence.rs` `gate`, and the Python mirror
  `tests/verifiers/exact_combinations.py`). On the received document the
  gate's input is whether a primitive case publishes an applied-load row.
  With the captured model, `model_checks` decides consumption from the model
  by the DEC-049 conditions (`consuming_constant_effort`, reusing
  `source_blocks::solver_constant_effort_support` for the hanger-type reading)
  and requires every solved case to publish exactly those records' rows, in
  model order and nothing else of that kind
  (`SOURCE_PHYSICS_COMBINATION_MODEL_CONSTANT_EFFORT`). Each consuming record
  is resolved as the producer resolves it, keeping its model node and the
  normalized name of its one translational DOF (`UX`, `UY` or `UZ`, whatever
  accepted spelling the model uses), and each row must equal, field for field
  (`bit_equal`, no field missing or added), the row
  `append_constant_effort_support_results` writes:
  - `id`: `result:constant-effort-support:{support}:applied-load` in the
    first solved case of `exact_cases`, and under
    `result:loadcase:{case}:` in every later case
    (`qualified_load_case_result_id`; `:` written `-` in both parts);
  - `kind` and `metadata.component`: `constant_effort_support_applied_load`;
  - `value`: the normalized constant load in newtons, bit for bit; `unit`:
    `N`;
  - `entity_ref`: the support id; `basis_ref`: the case;
  - `metadata.coordinate_system`: `global`;
  - `metadata.location`: `{node}:{DOF} applied load`;
  - `metadata.basis`: the producer's string, with `consumed_dof={DOF}` and
    the hanger's `source_reference`, `manufacturer_reference` and
    `load_side_review_reference` (entered text verbatim; missing or blank
    stated as `…_missing`);
  - `metadata.sign_convention`: the producer's text
    (`CONSTANT_EFFORT_APPLIED_SIGN_CONVENTION`), whole.

  A row dropped from a case or state, a changed force, a forged consumption,
  a row whose node, DOF or any other field is not the model's, or a model
  whose support acts along another axis or at another node is refused, so the
  gate and the applied loads are decided by the model. On a full document a
  changed `unit`, `metadata.component` or `source_result_refs`, or a duplicated
  id, is refused first by the direct-case pass; the binding refuses each too.
- **Review finding (deafe7153) and repair.** The first binding compared only
  (support id, force bits), so two forgeries passed both readers: (a) CE1-EB
  solved with its UZ support, checked against the model with that support
  changed to UX; (b) the applied-load row's location changed to
  `node:A:UX applied load`. Now the resolved node and DOF are retained and the
  whole row is bound, as above, in the Rust reader and its Python mirror. Both
  forgeries are refused with `SOURCE_PHYSICS_COMBINATION_MODEL_CONSTANT_EFFORT`
  (`rt_a3_constant_effort_references.rs`
  `applied_load_rows_are_bound_to_the_models_node_and_dof` in both solver
  modes; the reader's unit test
  `the_model_binding_binds_every_field_of_the_applied_load_rows`; and the
  Python `test_the_model_binding_binds_every_field_of_the_applied_load_rows`),
  with one control for each bound field. The same review found that CE-2's
  combined applied-load assertion passed vacuously when no row was found. It
  now asserts the row count and the support and location first.

**Retained source.** The retained-source replay forms the case force from the
nodal loads plus eigen loads only (`source_receipt.rs`, the load-state replay
included), so source recovery declines any model holding a constant-effort
record (`source_recovery.rs` `prepare_sources`) and the reader refuses a plan
naming one (`source_blocks.rs`, `CONSTANT_EFFORT_SUPPORT`). Where the ordinary
checks pass, the ordinary route publishes. Where the case needed the retained
response, the method is declined by name (`SOURCE_BLOCK_RECOVERY_UNAVAILABLE`;
on a load-state document `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED`,
`retained_source_attempt=unavailable`) and the ordinary route's own finding
stands; there is never a pressure-source-1 publication with a mismatched
replay (`tests/rt_a3_constant_effort_source_receipt.rs`).

**References and binding.** The frozen independent references are
`validation/references/rt_a3_constant_effort/` (CE-1, CE-1n, CE-2, CE-3 and
their mutation controls; two exact formulations). The product binding,
`core/product_physics/tests/rt_a3_constant_effort_references.rs`, authors
each case from its stated inputs and solves it in both solver modes at 1e-9
relative with per-family zero floors. It checks:
- motions, reactions, global end actions, N_w and S, axial stress per basis,
  the bending resultant `hypot(σ_y, σ_z)` and torsion per basis and end, the
  elastic maximum per basis and the Lamé rows;
- the applied-load rows (+F per case) and the record's support actions;
- CE-1's with − without = F-only, under both shear models, unpressurized and
  capped; CE-1n's non-consumption (no force, the warning, every mechanics row
  bit-identical to the model without the record);
- CE-2's five admitted combinations (exact factor sums of the case values,
  one combined applied-load row per consuming support, its support and
  location asserted, at Σf·F) and three withheld ones, and the 2⁵³ pair
  (refused by the value entry, withheld at the gate on the typed entry);
- CE-3's states S and O on load/reference-state documents with absolute
  temperatures (293.15 K installation, 393.15 K operating), O − S against the
  thermal-only solution, with − without = F-only in both states, and O − S
  independent of the force; per-state pressure regions are admitted, so
  CE3-EB-P and CE3-T-P are exercised.

Every envelope passes the pressure reader with and without its model. The
sign, double-count and O-omission mutants are distinguished, and forged
reasons, forged factor sums, dropped terms and applied-load rows not bound to
the model's node and DOF are refused.

**Pending (held).** The v3 formulation identity text still says every other
support family is refused, and the load-state limitation sentences do not
yet state D-2. Those texts, their copies (`exactReportLimitations.ts`,
`schemas/results.v0.3.schema.yaml`, the source-receipt test copy) and the
pressure-2 table's `combination_policy.mechanics_gates` (which lists gates
1–3) are held until another change integrates. They are updated together
afterwards.

## Nonlinear supports on EX (RT-A3 increment 4)

**Basis.** R-4 stage S5 ([route-unification-r4.md](route-unification-r4.md)
§3.5, with §00 and §2.1–§2.4); DEL-04-01 `kd5-nonlinear-active-set.md`.
This increment is stacked on the reviewed constant-effort candidate
`9282fc52f` (increment 3, above) and binds the frozen references
`validation/references/rt_a3_nonlinear/` committed at `324462348`.

**Admitted.** On a plain `3.0.0/exact_pressure_v3` document, gap, one-way,
lift-off and friction supports (explicit normal, or a normal derived from a
linear restraint) are admitted (`exact_admission::admission`, family
`NonlinearSupport`). They solve through PF's active-set loop and K-D5
confirmation, called from the shared `solve_prepared_load_case`. The loop's
fixed force is the case ledger's `AssembledForce`, which already carries the
pressure groups: Poisson pairs, terminal caps and each realized arc's
K_b·u_free − c_b (§2.1). Confirmation receives the same force and manifest
(§2.2). No change to the shared loop, `LoadCaseSolve` or
`nonlinear_integration` was needed. The EX publication path (§2.4) publishes
the loop's existing rows unchanged, and the wall force, membrane stress and
Lamé rows are recovered from the final displacement, as in a linear case.
The references exercise straight Euler–Bernoulli and Timoshenko members and
one realized bend (O-1).

**Still refused.**
- *By name, with the family's code and text unchanged*
  (`EXACT_PRESSURE_FAMILY_NOT_ADMITTED`; `exact_admission::nonlinear_support_admission`,
  called from `pressure_runtime::validate_profile`):
  - a nonlinear support on a load/reference-state document. Any authored group
    key counts, the `case_state::carries_fields` detector that
    equivalent-static generation also uses.
  - a nonlinear support in a model with an objective connector. Both belong to
    S6 (§3.7).
- *By the shared routes' existing refusals:*
  - a nonlinear support as a friction normal source (S7);
  - rotational gaps and rotational friction;
  - a linear restraint or spring on a nonlinear DOF;
  - a repeated active boundary DOF;
  - the loop's own connector refusal.
- *The retained sub-route:* source recovery stays ineligible for nonlinear
  cases (§2.4, D1 unchanged).
- *`difference_basis` on EX:* stays refused. Its S5 lift belongs to the
  combination seam (below).

**Standings.** Each is published identically in both solver modes.
- Gap-bearing cases take R1's unqualified branch:
  `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`, with no confirmation until
  R-3.
- A derived-normal friction support that ends Inactive (the p = 0 F-1
  controls) fails closed. It is published `NUMERICAL_INTEGRITY_SENSITIVE` with
  "support support:FRICTION: friction normal not established" (kd5 §2.4
  item 7). Its rows are the loop's binary64 values: the dense-mode inactive
  friction row is −2.9e-11 N, not R4's exact +0, which applies only to a
  confirmed state.
- Every other reference case is confirmed and published
  `NUMERICAL_INTEGRITY_CHECKS_PASSED` with the certified rows. These are the
  F-1 slide/stick (k = 1 Stage A), F1E, F1-T (Timoshenko), O-1 and their
  p = 0 twins. In those cases the inactive rows are exact +0.

**Binding.** `core/product_physics/tests/rt_a3_nonlinear_references.rs`
authors each case as a plain v3 document from its stated inputs and solves it
through the public entry in both solver modes. Twenty of the packet's 21
cases bind at 1e-9 relative with per-family floors. The comparisons cover:
- every displacement and rotation;
- every reaction, including the nonlinear and linear supports;
- all six global wall end actions at both ends of every member, the arc
  included;
- N_w and S;
- the axial wall stress on both bases;
- the Lamé rows;
- each support's state, deciding displacement and reaction, its separation
  and contact, and for friction N, μN, the source reaction and the stick
  margin;
- the slip sense and the derived-normal sign β.

The DEC-067 laws and strict complementarity are checked on the published rows
independently of the references. The binding refuses each of these forged
controls:
- a halved contact;
- a closed gap stated inactive;
- a friction force at 1.1·μN;
- a gap case relabelled Passed;
- a derived inactive case relabelled Passed.

**Reader binding (§2.4).** The pressure reader now binds the nonlinear support
results: RE `nonlinear_support_evidence.rs`, called from
`physics_evidence::validate_direct_pressure_evidence` (without the model) and
from `reader_verdict::validate("pressure", …)` (with it), and its Python mirror
`tests/verifiers/nonlinear_support_evidence.py`, called from `pressure_2.py`
and `reader_verdict.py`. Both refuse with the same `SOURCE_PHYSICS_NONLINEAR_*`
codes.

The rows and their producer are PF's, but PF's readers bind only the rows'
attribution (`preview_physics_evidence`) and the operand records
(`preview_combination_operands`). There were no PF state, law or standing
checks to reuse. The binding therefore reuses what exists: PF's support-magnitude
rule (`guarded` over the correctly rounded `norm3`; Python `_consistent_norm`),
the operand reader's state-code tokens and the verifiers' unit catalog. The
following checks are new.

*Without the model*, per load case with nonlinear rows:
- the loop rows: one iteration count (an integer ≥ 1) naming the support
  classes, a zero final residual count and a converged flag of 1;
- per support: one state row (code 0–3), one final displacement and one final
  reaction row. Each is on the state row's DOF, in that DOF's units, and
  states the same `final_state`. The support's
  `NONLINEAR_SUPPORT_STATE_REVIEW` diagnostic names the same state;
- no relative-displacement row (`RELATIVE_DISPLACEMENT`). S5 admits nonlinear
  supports on plain v3 documents only, and EX refuses a support-targeted
  primitive by name, so no case moves a nonlinear support's ground. W-16
  R-b's row is PF's. A later lift of ground motion on EX must bind the row's
  presence and value to the resolved ground and the absolute displacement;
- the final displacement equals, bit for bit, some node's published
  displacement or rotation row at the support DOF in the row's unit
  (`DISPLACEMENT`). The producer's `restate_nonlinear_support_displacements`
  restates the support node's published SI value, ×1000 for mm and unscaled
  for rad. Without the model this is association with some published node
  row, not proof that the row is the support node's: a value equal to another
  node's row is admitted. The model names the node (below);
- the final reaction is the support's published action component, bit for
  bit, and the support's magnitudes follow its components;
- a derived friction normal is the absolute value of its named source's
  published component, bit for bit;
- the classes name the support kinds: the friction supports are exactly those
  with a normal row;
- the DEC-067 laws that need no inputs:
  - a sliding support slips (δ ≠ 0) against its force (R·δ ≤ 0);
  - an inactive friction support has N ≤ 0 (derived: N = 0);
- the standing:
  - one integrity diagnostic, with a code the producer publishes. Its
    `numerical_quality` record carries exactly the four tokens the producer's
    `assessed_numerical_quality` assigns to that code (structural status,
    solve quality, matrix fidelity, accuracy evidence) and names the
    diagnostic in `evidence_refs` (`STANDING`). An unknown code is refused
    (`STANDING_CODE`). Before the RT-A3 increment 4 review this was only
    "Passed iff checks_passed";
  - a gap-bearing case is `RECOVERY_BASIS_UNQUALIFIED` and unresolved (R1).
    This gap precedence is checked after the mapping;
  - a Passed case carries kd5 §3.3's evidence line;
  - a Passed case has no derived-normal friction support inactive. That state
    fails closed and is never Passed (kd5 §2.2; the case publishes
    `NUMERICAL_INTEGRITY_SENSITIVE`);
  - a Passed case has no friction support in state active;
  - a Passed case publishes each inactive support's reaction as exact +0
    (kd5 R4);
  - a non-Passed case keeps whichever earlier reason the producer stated
    (kd5 §2.4: the confirmation did not run, or it failed), so any mapped
    non-Passed code is admitted outside the gap and derived-inactive rules;
- an inactive support in a non-Passed case keeps the loop's reported
  reaction. That is the binary64 residual of its free row (kd5 §0, §1.2):
  inspection evidence, with no claim that it is zero. The reader requires it
  finite (`INACTIVE_REACTION`, a backstop behind the outer number rules and
  `SLOT`/`MAGNITUDE`) in the support's family (`UNIT`). The case must
  also publish the producer's own original-equation evaluation of the final
  linearized solve's free rows (`append_nonlinear_residual_observation_results`
  over `product_equilibrium::evaluate`) once in the support's family:
  `nonlinear_support_observed_free_dof_force_residual` (N) or
  `…_moment_residual` (N·m), on the loop, finite and ≥ 0, with
  `residual_basis=free_dof_force_moment_equilibrium` and
  `observed_compliance=true` (`RESIDUAL`). The verdict states that every free
  row met 64·γ(n) of its own denominator Σ|K_ij·u_j| + |f_i|.

  That record is required evidence, not a rounding allowance for the
  reaction, and the reader imposes no magnitude relation between them.
  Recovery forms the reaction as a row dot product and then subtracts the load
  (`nonlinear_integration::solve_linearized_system_evidence`). The residual
  evaluation starts from the negative load and accumulates normalized products
  (`frame_kernel` `structural::evaluate_original_residual_bound`). These are
  different evaluations, and the envelope publishes neither row's denominator,
  so no published bound relates the two values. The first review's B1 showed
  the 1e-9 rule refusing untouched dense output. The second review's B1 showed
  the |R| ≤ record rule refusing untouched sparse output: a loaded inactive gap
  with R = −3.24e-13 N against a compliant record of 2.10e-13 N
  (`core/product_physics/tests/fixtures/rt_a3_nonlinear/b1_loaded_inactive_reaction.json`).

*With the model:*
- every solved case publishes exactly the model's nonlinear supports, with
  their DOFs, kinds and normals (an explicit N bit for bit, a derived source by
  support and DOF);
- each support's final displacement is its own node's published row at the
  support DOF, bit for bit (`MODEL_DISPLACEMENT`; the model input keeps
  `support.node`). This, not the model-free rule, binds the row to the
  support's node;
- the DEC-067 laws that need μ, the clearance g or the sense σ, on that
  deciding δ:
  - gap: s = g − σδ, c = −σR; one-way and lift-off: s = σδ, c = σR;
  - s ≥ 0. An active support is closed (s = 0) and pushes (c ≥ 0). An inactive
    support's force is the model-free rule above, a residual whose sign and
    magnitude state nothing, so s·c is not checked for it;
  - friction, unless inactive: |R| ≤ μN. Sticking has δ = 0. Sliding has
    |R| = μN, opposite to a nonzero slip when μN > 0;
  - a Passed explicit-normal slide is −sign(δ)·fl(μN), bit for bit (kd5 §3.1).
- Each "= 0" and "≥ 0" holds within 1e-9 of the case's scale in the
  family (kd5's criterion), except the exact rules named.

*Results.* Both readers admit all 40 reference envelopes with and without
their model. They also admit the 34 K-1 twin envelopes (below). Each forged
control above is refused by both, without the model, by the code shown:

| Forgery | Code |
|---|---|
| Halved contact | `MAGNITUDE` |
| Closed gap stated inactive | `STATE_REVIEW` |
| Friction at 1.1·μN | `MAGNITUDE` |
| Gap case relabelled Passed | `STANDING` |
| Derived inactive case relabelled Passed | `STANDING` |

Deeper variants keep the cheaper evidence consistent and reach the later rules:

| Variant | Without the model | With the model |
|---|---|---|
| Stated inactive, with its state diagnostic (limit) | admitted | admitted |
| G2's open gap stated active | admitted | `MODEL_COMPLEMENTARITY` |
| 1.1·μN with its magnitude | admitted | `MODEL_FRICTION_CAPACITY` |
| Force turned along the slip | `SLIP` | `SLIP` |
| Doubled derived normal | `NORMAL` | `NORMAL` |
| Relabelled Passed with quality and evidence line, gap case | `GAP_STANDING` | `GAP_STANDING` |
| Relabelled Passed with quality and evidence line, F1-P0 | `FRICTION_NORMAL_STANDING` | `FRICTION_NORMAL_STANDING` |

The RT-A3 increment 4 review's three blocking reader findings (B1–B3) are
repaired with these regressions and controls
(`reader_binds_inactive_residuals_support_nodes_and_standings`;
`test_inactive_residuals_support_nodes_and_standings_are_bound_by_both_readers`):

| Case | Without the model | With the model |
|---|---|---|
| B1: the review's inclined gap, untouched, both modes; dense R = −3.3e-15 N against 1e-6 N | admitted | admitted |
| B1 (second review): the loaded inactive gap fixture, untouched sparse output, R = −3.24e-13 N above its 2.10e-13 N record; and its model solved now | admitted | admitted |
| The inclined gap's inactive force doubled, slot and magnitude consistent (limit) | admitted | admitted |
| Its inactive reaction stated in N·m | `UNIT` | `UNIT` |
| Its free-row evaluation stated non-compliant, absent, negative or duplicated | `RESIDUAL` | `RESIDUAL` |
| Its free-row force record stated in N·m | refused before this module | refused before this module |
| O1-LIFT (Passed) inactive reaction published −0 | `PASSED_ZERO` | `PASSED_ZERO` |
| B2: G2 stated active at a 1 mm (clearance) displacement | `DISPLACEMENT` | `DISPLACEMENT` |
| G2 stated active with an invented ground row at the clearance | `RELATIVE_DISPLACEMENT` | `RELATIVE_DISPLACEMENT` |
| G2's displacement replaced by the anchor's published +0 | admitted | `MODEL_DISPLACEMENT` |
| F1-SLIDE (Passed) slip ×1000 | `DISPLACEMENT` | `DISPLACEMENT` |
| B3: F1-P0 code Sensitive → unqualified, quality left sensitive | `STANDING` | `STANDING` |
| F1-P0 code Sensitive → failed, quality left sensitive | `STANDING` | `STANDING` |
| F1-P0 accuracy evidence alone stated unresolved | `STANDING` | `STANDING` |
| F1-P0 quality record naming no diagnostic | `STANDING` | `STANDING` |
| F1-P0 code unknown to the producer | `STANDING_CODE` | `STANDING_CODE` |
| G1 code unqualified → Sensitive, quality left unresolved | `STANDING` | `STANDING` |
| G1 consistently relabelled Sensitive (code and quality) | `GAP_STANDING` | `GAP_STANDING` |
| F1-P0 consistently relabelled `UNRESOLVED` (limit) | admitted | admitted |

These are the negative controls of
`rt_a3_nonlinear_references.rs` (`forged_contacts_states_friction_and_standings_are_refused`
and the test above)
and of `tests/test_rt_a3_nonlinear_reader.py`. The Python test authors the same
documents, solves them with the runner and compares Python's verdict with the
runner's `read pressure` on every envelope.

*Limit, pinned:* a contact scaled consistently, with its magnitude, is lawful
and self-consistent. Both readers admit it, and only the references refuse it.
Refusing it needs a nodal-equilibrium binding of support actions against
member end actions and the pressure ledger, which no reader has on either
route.

*Further limits:*
- *A non-Passed inactive reaction's magnitude.* Since the second review's B1
  repair, nothing in either reader bounds it. A forged magnitude with
  consistent slot and magnitude rows is admitted, including a closed gap's
  contact force restated inactive with its state diagnostic (the table's
  limit rows). What still refuses a forged inactive force: a non-finite or
  non-numeric value (the outer number rules first, Rust
  `SOURCE_PHYSICS_NUMBER_INVALID`, Python `SOURCE_PHYSICS_EVIDENCE_INVALID`;
  within this module `SLOT` or `MAGNITUDE` precede the backstop
  `INACTIVE_REACTION`, which no current envelope reaches),
  the wrong family (`UNIT`), a missing or non-compliant evaluation record
  (`RESIDUAL`), a magnitude row not following the components (`MAGNITUDE`),
  and in a Passed case anything but exact +0 (`PASSED_ZERO`). Only the
  references, or an equilibrium binding no reader has, refuse the magnitude
  itself.
- *A consistent non-Passed relabel.* A non-gap case relabelled to another
  mapped non-Passed code with its quality tokens is admitted. kd5 §2.4
  preserves earlier non-Passed reasons, and the reader cannot tell which
  reason applied.
- *The model-free displacement rule.* It requires some node's published row
  to carry the same bits. That is association with some published node row,
  not proof of ownership: a fabricated value equal to another node's row
  passes without the model (G2's gap displacement set to the anchor's +0).
  Only the model, or an independently bound support-to-node map, names the
  node.
- *Parity of outer refusals.* Malformed numbers that the outer validators
  refuse before this module are refused by both readers, with different
  codes: Rust gives `SOURCE_PHYSICS_NUMBER_INVALID` (`physics_evidence.rs`)
  and Python `SOURCE_PHYSICS_EVIDENCE_INVALID` (`pressure_2.py` wraps the
  physics-evidence errors). Normalizing them would change shared readers
  outside this increment, so the parity claim is acceptance and
  `SOURCE_PHYSICS_NONLINEAR_*` codes only. The tests assert identical codes
  for every nonlinear refusal.

G1-P0 is not expressible as the packet states it. It has no load of any kind,
and the product refuses a document without a primitive load
(`LOAD_INPUT_MISSING`). The binding pins that refusal and invents no zero
load.

**K-1 controls (§3.5).** `core/product_physics/tests/rt_a3_nonlinear_k1_twins.rs`
solves every PF K-D5 nonlinear control on its exact twin in both solver modes.
The controls are the seven `validation/references/kd5_nonlinear/` requests with
the cases `kd5_nonlinear_tests.rs` solves, and the U-1 gap containment control:
17 cases, 34 case-modes. A twin changes the PF request only where the exact
contract asks:
- `pressure_contract` 3.0.0/exact_pressure_v3, Euler–Bernoulli;
- each material as E and ν = E/(2G) − 1 (ν = 0.25), asserted to reproduce
  PF's G bit for bit;
- an explicit zero corrosion allowance;
- `pressure_regions: []` in each case.

*What holds, asserted bit for bit in both modes:*
- every support's state code;
- the loop's iteration count, final residual count and converged flag;
- the standing: the integrity code, the `numerical_quality` record, and kd5's
  evidence line or the named §2.4 reason. This is Passed for 13 cases;
  Sensitive with "stick capacity not established" (F-1 BOUNDARY) and
  "separation not established" (U-1 IN-PLANE); and the gap branch for the 2 gap
  cases;
- the reader admits every twin with and without its model.

The rotational-friction control is refused by name on the twin as on the
original.

*Finding: the loop results are not bit-identical, and the cause precedes the
loop.* EX forms each member's section constants by D-5's sequence
(`pressure_exact::SourceAnnulus`, published as `pipe_sections`). PF forms them
by its own, bound by the kd5 oracles' `normalized_sections`. For the controls'
0.168 × 0.007 m section, A differs by 3 units in the last place and I and J by
1 (A = 0.003540574920595697 m² on EX, 0.003540574920595698 m² on PF). The
same twin without its nonlinear supports, a linear case, differs in its nodal
rows too.

The other loop rows differ by at most 4.5e-15 of their family's case scale.
The exception is U-1 IN-PLANE dense, whose moment family is wholly binary64
noise of order 1e-16 N·m and changes sign. The loop's residual observations
differ as noise of the final iterate. The cause is pinned
(`kd5_twin_section_constants_differ_from_pf_in_the_last_place`). K-1's first
wording (`every_kd5_control_twin_is_bit_identical`) stays held as superseded.

*The K-1 amendment (option (a)).* Agent 0 chose to keep each route's
established section formation. The amended K-1 (R-4 §3.5, wording, bound and
basis in [k1-twin-bound.md](k1-twin-bound.md)) asserts on the named corpus:
- the states, the loop's counts, convergence and the standing, exactly;
- every value row, finite on both routes, within |x_EX − x_PF| ≤ a_f + r_f·S_f
  per family. S_f comes from the case's inputs and a_f = 0, so a zero scale
  admits equality only. r_f ranges from 2^-40 (work) through 2^-32
  (translation and translation deltas) to 2^-26 (force).

The constants come from independent exact-rational forward-error enclosures
([k1_twin_bound.py](k1_twin_bound.py), on kd5's reference formation). They
enclose the exact section-difference response and each route's
residual-certified solve. The recovery cancellation and its row's formation,
an applied force's own error, the coupled friction denominator 1 + c·g and the
gap's prescribed terms are included. A delta row's preceding iterate is derived
from the authored seeds and the first iterate's strictly decided signs, under
the published iteration counts and the absence of the seed recovery, which the
test pins; the product publishes no per-iteration state. The constants are not
set from the observed differences, which are at most 1.5e-3 of their
enclosures.

The assertion (`every_kd5_control_twin_is_within_the_k1_bound`) passes all 34
case-modes when run explicitly, and is held `#[ignore]`d until the amendment
is independently reviewed. Its controls run un-held
(`the_k1_bound_refuses_its_controls`). They are:
- the U-1 lifted twin without its parallel spring;
- a 1e-6 EX-only section perturbation (U-1 LIFTED and F-2 SLIDE), and a 1e-6
  response perturbation (every twin);
- a one-ulp offset at zero scale;
- NaN and ±infinity.

The bound's resolution limits are recorded in the note: a 1e-6 section
perturbation is invisible on U-1 PRESSED, the delta families hold only for the
derived matched history, and the two weakly decided cases' equal final states
are observations.

*Not formable:* kd5's eight committed case-modes solve the two committed demo
models (`fixtures/product_preview/invented_demo_model.json` and
`core/product_physics/tests/fixtures/preview_physics_invented_model.json`). Their
twins are refused by name (`EXACT_PRESSURE_FAMILY_NOT_ADMITTED`) on the
geometry-only bend `component:C-110` (D-2; S6's inventory) and the terminal
`component:C-140`. This is pinned, not adapted.

**Pending.**
- *The v3 identity and limitation texts* (held for sequenced integration):
  - the "Admitted families" sentence;
  - the support-attribution sentence ("linear restraints, springs and
    nonlinear supports");
  - the gap-standing sentence.

  Their copies and the pressure-2 table move with them.
- *Combinations, OQ-1 and κ:* the load-case lane's RT-A4 implements EX's
  `NONLINEAR_COMBINATION_REQUIRES_SOLVE` gate and OQ-1 with both readers. On
  this base EX still forms every flat mechanics combination of a nonlinear
  model, singleton included (`withheld: false`). This increment must
  therefore not integrate ahead of that gate. The binding holds the
  withholding assertion `#[ignore]`d until RT-A4 lands.
- *Shared-loop hunks:* none were needed. No `LoadCaseSolve` construction or
  early return was added.
- *Pricing* of the changed static chain (`pressure_runtime.rs`,
  `exact_admission.rs`), held in the serial pricing order.
- *Performance harness:* the rack's "RT-A3 increment 4" ledger entries and
  its interim linear forms (`rack_model.py`, `release_runner.py`).
- *K-1* (above): the option-(a) amendment awaits independent review; its
  assertion is held until then.
- *The reader's equilibrium limit* and the further limits (above).
- *§2.2's explicit member-bound integration test.* The confirmed cases'
  certified rows match the references, but the wall-bound derivation is not
  separately pinned.
