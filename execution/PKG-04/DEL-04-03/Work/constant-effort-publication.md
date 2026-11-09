# Constant-effort fallback publication

RT-S3 and RT-A3 require a consumed DEC-049 constant-effort device to publish
its normalized positive global support-on-pipe force. On source
`00832d746912c26234325e85d0dd28f1914a2eea`, the ordinary/no-nonlinear reference
system consumes RS-CE1 +9,000 N but publishes zero SUS/OPE/SAM support actions
in both modes. An applied-load row or accepted reader verdict does not make
that zero correct.

The current preparation composes the repair from immutable recovery head
`4c18bc277751e7be080ad5932fa06c5f1b49e50b` onto provisional S6 candidate
`aea0887d5c51495f1b32bdea7f66863aa556fc87` (#166), incorporating its two
test-tool repairs by an ordinary merge after the original `7c70f52fe` basis.
The six production lines
apply cleanly; the remaining changes are tests and DEL-04-03/DEL-04-07
reconciliation. The final predecessor is the actual integrated S6 plus current
main. Merge it normally, assess the delta and rederive affected consumers
before qualification. Preparation on this candidate supplies no final-source
qualification or acceptance. S6's source, observations, registration and
pressure-contract status retain their own identities.

## Publication and regression scope

The support loop uses `classify_constant_effort_consumption` on the same
normalized model as the force ledger, after selected-source precedence and
only without a nonlinear field. A successful application fills its global
translation; the remaining components retain exact +0. The later certified
publication still overrides that preliminary vector. Force formation, support
laws, admission, schemas, load states, tolerances and numerical standing do
not change. The pressure reader currently binds the applied-load record but
does not independently establish the fallback CE support action; producer
checks must establish that action and equilibrium.

The maintained packet is:

- `core/product_physics/src/lib.rs`: normalized 7.25 kN on UX/UY/UZ, certified
  and forced-unavailable-bound fallback, both modes; fallback non-consumption
  for empty, rotational, multiple and repeated translational entries and a
  long invalid token after a valid entry; existing absent/unknown-node,
  missing/nonpositive-force, nonlinear precedence and every-case controls.
  Long-list controls establish behavior, not an allocation or input cap.
- `core/product_physics/tests/rt_a3_constant_effort_references.rs`: frozen
  CE1/CE1n/CE2/CE3 references, normalized axes and independent root balance,
  active load/reference states and combination refusal/reader controls;
  inactive and locked CE states must remain refused before solve rows.
- `core/product_physics/tests/rt_a3_reducer_2b_reference_system.rs`: the
  naturally sensitive ordinary/no-NL RS-R1/RS-CE1 reproduction, SUS/OPE/SAM,
  both modes, all-support force and moment balance against the independent
  frozen weight record. Its self-weight is derived on the complete overlaid
  model; original geometry, materials, configurations and combinations stay.
- Existing support-attribution, source-receipt, S6 LS-1/C-1 and modal state
  custody checks remain relevant. An unavailable certification bound is
  deliberately covered; a separately forced per-support certified-recovery
  error is not claimed as executed evidence.

Run targeted lib `constant_effort` tests, both reference-test targets above,
`support_reactions_runtime`, the source-receipt controls and the affected S6
and modal-state tests from the final selected source under the maintained
route. New tests are prepared source, not current runtime passes. Original
29-test evidence applies only to the recovery candidate.

## Changed proof and resource obligations

This assessment concerns the CE delta on the provisional composition, not a
replacement for S6's §8.60 observations. The classifier body, its normalized
receivers and the solve body's literal inventory are unchanged from the
provisional predecessor. The extra call is still real work before a later
certified override; existing classifier calls are not replaced or discounted.

| Site | Required treatment on the actual composition |
| --- | --- |
| Predicate and receiver | Reborrow each normalized support with a known node, no selected source action and no nonlinear field. Hanger/family trimming borrows strings. No new retained owner or force assembly occurs. |
| Authored restraints | Each recognized translational entry pushes one `FrameDof` into the classifier's `Vec`. Length is bounded by authored entry count, including duplicates, not by three axes or six DOFs. Use the maintained Vec growth/reallocation rule and actual layout; rotations need no backing. |
| Invalid token | `parse_dof` formats an error and the classifier clones the invalid token into its returned error. Include variable token bytes and a conservative overlap with earlier Vec backing until drop scheduling is established. The caller drops the error; it does not format `unmet_condition`. No fixed-label classification can discharge this path. |
| Repeated classification | At most one added call per eligible support per prepared case, sequentially; each scans authored restraints and successful consumption repeats the node search. Heap peak is the per-call maximum plus its parent union, not all support Vecs simultaneously. The later certified call and existing ledger/diagnostic/applied-row calls remain separate. |
| Static custody | Temporaries overlap the case's ledger, stiffness/build, solve arrays, selected nonlinear/confirmed state, certified formation source, accumulated result/recovery records and observer custody as actually live. No new result field or carrier is introduced. |
| Modal same-run state | `modal/state.rs::solve_modal_state_case` reaches this static producer for nonlinear state work; a separate CE record can reach the new call. Include Prepared normalized model/build/boundary/resolution, stiffness, scratch budget, diagnostics and returned state solve. Later mass/pencil/eigen owners are disjoint at this call. Do not classify the state solve as absent merely because the final consumer reads typed nonlinear evidence rather than CE rows. |
| Publication consumers | Changed action and norm values flow through preview/result records, exact combinations, readers, report packages and stress-neutral exports; their text/serialization lengths can change even with unchanged row count. No whole-app resource bound follows from unchanged consumer code. |

D1.6 refuses every hanger and every family outside its listed linear families;
D1 also excludes load states and nonlinear supports. With those exact receiver
warrants, the new classifier branch is unreachable on admitted D1 models.
That is a proposed retained-domain proof premise, not a global zero-cost
claim, stack-neutrality proof or automatic registration carry. The ordinary
outside-D1 allocation/live-union and modal-state residuals remain explicit.
Existing modal resource policy and unbounded terms are unchanged.

The current maintained guards stop on the changed
`solve_prepared_load_case`: KD5T2 protects the producer/consumer literal and
owned-slot inventory; B5O1 protects carrier children and staging. The same body
is also pinned by first-error and other owner models, while full-file and
routing digests protect broader source identities. Historical L/E failures
remain in the recovery evidence. No guard, pin, source assertion, profile,
coefficient, threshold, law or native-smoke expected value is changed here.
A proposed guard update needs source-grounded treatment and independent
scrutiny; a requirement/proof conflict goes to Agent 0 before an affected
change, with unaffected preparation continuing.

The maintained guard obligations for the selected source are specific:

| Guard/model | Source basis needed before replacement |
| --- | --- |
| KD5T2, `kd5_text_model.py` | Removing exactly the six reviewed lines recovers S6's solve body. Its literal inventory and the classifier/ledger helpers are unchanged. Carry fixed-label/slot facts only after treating the new edge and its variable error allocation separately; an unchanged literal list alone does not price that edge. |
| B5O1, `batch5_model.py` | No `LoadCaseSolve`/ResultItem field, carrier constructor, existing staging site or support row count changes. New classifier temporaries still coexist with the live static parent; use the existing carrier/child staging basis plus the added work, not a zero-temporary assertion. |
| O4, `o4_owners_model.py` | Of 30 owner-body warrants, only `solve_prepared_load_case` differs; sidecar, reduced-set and case-preparation spans still match. Reconcile O4-SR's second composite maximum and O4-SP's payload keys/per-station components against the six-line delta. Preserve the positive owner/copy, push-growth/previous-backing and phase/lifetime charges, including distinct/reduced preparations, receipt and reader copies and kernel staging. No new CE temporary is paid for by subtracting an unchanged O4 owner or assuming reuse. Native strides and the actual live union still need final-source checks. |
| KD5C6, `kd5_first_error_model.py` | The new if-let discards the consumption error and has no `?`, early return or call to the private fallback-error roots. Existing fallback attempts, first-return alternatives and outer clones stay. Check the exact helper/root/site inventory before carrying those first-return limits. |
| B7a domain and KD5C4 primitive-owner source bindings | Raw lib.rs changes include both production and test additions. D1.6 rejects the only receivers that can enter the new CE edge; verify the admission receiver and exact hanger/family predicate and preserve the primitive-owner census, formulation/text and late-admission warrants. |
| W1 reduction source/lifetime binding | FormationSource and its Cow reduction, guarded combination index/shadow/insert/call and the reduction owners remain unchanged. The extra static call does not transfer ownership to a W1 reduction. Check those exact warrants and parent lifetimes before carrying this full-file binding. |

The final tool treatment should source-guard the added edge's D1 exclusion and
its exact receiver, not remove a broad helper/Display edge or weaken the D1
admission domain. Which rule keys, coefficients, registration inputs and native
atoms move must come from the maintained full derivation. No final rule or
replacement pin is selected in this preparation.

The routing selector also binds runtime readers to their calling crate's whole
Rust tree. `source_block_recovery.rs` is unchanged and matches its declared
hash, but the CE delta changes that crate tree. A projected in-memory derivation
using maintained `numerical_ci.rust_tree_digest` and
`source_read_dependencies` needs exactly the product-physics tree digest and
`rt_a3_constant_effort_references.rs` runtime-read SHA. The latter reader's own
source changed; the reducer reference test adds no runtime-read declaration.
With those two projected values, all 47 manifests, 45 tree bindings and 1,808
resources validate on the selected source. Inputs, source scans, callers and all
other declaration fields remain unchanged. This is preparation only: the
committed routing file is untouched, the count is an actual current-source
observation rather than S6's historical count, and final values must be
rederived after S6 lands. Preserve S6's historical derivation and proof.
S6's existing `check_routing_inventory` already derives its tree and reader
bindings from actual current source. Assess the final two-binding CE registration
against that maintained check before proposing any guard change. The three
historical malformed-owner materialization failures are a distinct S6 source
repair in `aea0887d5`; incorporating that repair is not a passing test result.

## Actual-source D3 and AC-004 inputs still needed

1. Agent 0 integrates S6. The CE manager merges that actual predecessor plus
   current main without rewriting history and assesses changed source,
   dependencies and consumer writers. S6's pending native summary/detail/reopen
   and PARITY outputs must be derived by maintained writers on the final CE
   source, never hand-merged or treated as provisional hash carry.
2. Review the changed edge, receivers, authored-entry and error allocations,
   repeated classification and modal/static parent unions. Apply maintained
   pricing rules to that exact source, including any approved warrant change.
   Compare all four points against S6's §8.60 registered tree using its complete
   tool cohort; inspect pruning, unclassified sites and line-map movement.
   Establish `a_le_b` or derive a proposal with the maintained generator.
3. Bind actual native compiler/profile/lock, source-selected inputs and layouts,
   run profile laws and all 80 budget evaluations. Reconcile registered input,
   generated block/point atoms, retained carrier writers and runtime-read graph
   only where actual source derivation requires it. No preselected coefficient
   change or registration disposition is implied by this packet.
4. Prepare immutable final lib-test/challenge binaries, hashes, dep-info source
   map, harness/supervisor, exact command and fresh empty output destinations.
   Changed solve code compiles into witnessed binaries; equal core portions or
   coefficients cannot establish stack neutrality. The scoped witness set is
   W1 headroom sparse/dense, W2 deep milestone sparse/dense, W_C2 publication
   sparse/dense and `retained_direct_peak_is_within_the_profiles_bounds`.
   Ask Agent 0 for the separate owner AC-004 decision on that concrete packet.
   Old attempts are consumed; no fresh AC-004 is authorized here.
5. Once Agent 0 supplies a verified usable route and lane-specific resource
   window, run ordinary guarded regressions/builds with explicit source/input
   provenance. AC-004 additionally requires separate owner authorization and an
   exclusive no-build interval. Exact-head hosted checks and Agent 0's existing
   publication consultation precede its integration decision.

The maintained host route is currently refused and the guard is not running;
no local build, pytest, native launch, benchmark or capture follows from this
preparation. Independent source scrutiny and lightweight checks can proceed.
Next actor: CE manager owns source/proof preparation and ordinary repairs;
Agent 0 owns final S6 integration, usable-route scheduling and the scoped
owner AC-004 request. The CE draft remains unqualified.

## Recovery and boundaries

The immutable CE branch and original baseline/repair logs, failing guards and
source inventory remain at
[recovery commit dc037b5495ccd6fa35fb04758edbcdb019021bba, ce/](https://github.com/sgttomas/SWBPIPE/tree/dc037b5495ccd6fa35fb04758edbcdb019021bba/ce).
Historical evidence is not regenerated. Library consultation uses 3.3.0 commit
`ec3ee2d1b398eba1fb3227ccf93e2569df95bacd`.

Publication order remains S6, then verified CE correction, then further modal
composition. The separate [hanger hold](../../DEL-04-11/Work/hanger-sizing.md)
and H1 reservation remain. No sizing/preload, frozen-reference, native-smoke
expected-data, release or acceptance authority is exercised. Public recovery
refs and raw host paths remain within the owner's D-20/DEC-058 scan scope;
publication supplies no scan signature. Preserve these refs while their named
evidence consumers or owner retention need them.
