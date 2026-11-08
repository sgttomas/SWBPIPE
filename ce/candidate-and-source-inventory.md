Local candidate `4c18bc277751e7be080ad5932fa06c5f1b49e50b`, branch `codex/rt-s3-ce-publication`, based on actual main `00832d746912c26234325e85d0dd28f1914a2eea`. No public PR or qualification queue entry exists.

RT-S3: a consumed constant-effort device must publish its positive global support-on-pipe action when certified recovery is unavailable. Main 00832d746's ordinary/no-NL reference-system overlay consumes and reports RS-CE1 +9000 N but publishes CE Fz 0 in all SUS/OPE/SAM cases, in both SparseInteractive and DenseScrutiny. Cases remain numerically sensitive and the reader accepts them; the bad zero is not a preservation baseline.

The source repair adds six production lines to PP lib.rs's fallback support-publication loop. It uses the same normalized consumption classifier as the existing case force ledger, fills only its consumed global DOF and preserves nonlinear-field precedence. Force formation, solve, tolerances, schemas, load/support states, preload, admission and certified publication are unchanged. No inference from a rounded applied-load row or an unconsumed record is used. S6 confirmed that this hunk is disjoint; its candidate and the paused hanger manager are untouched.

Maintained checks cover the naturally sensitive full reference-system ordinary/no-NL path in both modes, all-support force and moment balance using the existing independent weight record, fresh self-weight bound to the complete overlaid model, normalized kN/global-axis publication with exact zero other components, independent cantilever root balance, and the existing U4a fallback control on all axes in both modes. Existing active load-state, absent/non-consuming record, combinations, source-receipt and support-attribution tests remain. Guarded evidence: 13 CE-related unit tests, 10 CE reference tests, one source-receipt test, four support-attribution tests and the new sensitive full-reference regression pass (29 distinct tests). Baseline main was independently reproduced before production edits, including both modes and all three sensitive cases. No frozen numerical reference or raw carrier changed.

Actual-source qualification is OUTSTANDING. The pure CE pricing-source-warrant test fails in BOTH L/E variants with KD5T2: source pin changed: core/product_physics/src/lib.rs:solve_prepared_load_case. This expected old guard is left intact. The new classifier call in the support loop adds an execution/allocation site (its existing bounded translational-DOF collection); D1 excludes CE records, but that does not supply an actual-source D3 carry or registration. No rule, pin, law, coefficient, profile, cohort or registration is edited or rederived, and no AC-004/exclusive/resource witness runs. The selected modal lane alone owns the next final qualification/composition turn; this local source candidate is not published, frozen, queued or offered as combined clearance. Required exact-head hosted CI and PR/head publication consultation remain later integration conditions.

DEL-04-07 Work explicitly records the active RT-S3 defect and unqualified source repair; both affected SOWs describe the existing normalized support-action contract, and DEL-04-03/04-07 YAML reconcile paths/checks and the consumed device action. DEL-04-11's need remains unchanged: sizing already consumes ordinary published actions with this existing positive-axis sense. No sizing, preload or held hanger work is released. Independent numerical/publication source review is clear on exact local candidate 4c18bc277751e7be080ad5932fa06c5f1b49e50b with no actionable findings. The reviewer independently inspected normalization, consumed source, sign/frame/zero behavior, nonlinear precedence, regression paths and exact declaration changes; runtime passes and old-guard failures remain supplied evidence. Producer/consumer inventory review and actual-source qualification remain outstanding. No release-target status refresh, MBF/C6/App work, native launch or acceptance claim is included.


## Producer/consumer inventory — read-only preparation on stable 4c18bc277

This inventory applies only to `4c18bc277751e7be080ad5932fa06c5f1b49e50b` over actual main
`00832d746912c26234325e85d0dd28f1914a2eea`. It is proposed proof treatment for a later selected
composition, not D3, registration, queue entry, resource measurement or qualification. Source HEAD remains
unchanged. References below are repository paths at that candidate. No pricing guard or generic Value/Display
edge has been disabled or reclassified.

### 1. New edge, receivers and aliases

- The new direct edge is `lib.rs:solve_prepared_load_case` (~6259) →
  `classify_constant_effort_consumption` (~14491). It is synchronous and private; the new site installs no
  function pointer, trait object, callback, task, closure capture or retained owner. It runs after the outer
  `node_index(model, &support.node)` succeeds, only if no `selected_source` action matched, `nonlinear` is absent,
  and the existing `is_constant_effort_support` predicate succeeds. It runs on both ordinary and certified
  attempts: a later successful certified call overwrites the preliminary vector. “Fallback repair” does not
  mean the extra work executes only when certification is absent.
- `model: &PreviewModel` and `support: &PreviewSupport` are borrowed from the same immutable normalized model
  used by `case_force_ledger`/`add_constant_effort_support_loads`. The support loop reborrows an element of
  `model.supports`; it neither clones nor mutates that record. `hanger.as_ref()`, `constant_load.as_ref()`,
  `restraints`, each `raw: &String`, and `support.node` remain borrowed. The returned
  `ConstantEffortApplication {node_index, dof, force_newtons}` owns only scalar values; no borrowed pointer or
  heap owner escapes through it. `dof_index` consumes a Copy enum and changes one slot in the pre-existing
  `[f64;6]`; no new force vector or result buffer is allocated by the assignment.
- Leaf reach: `is_constant_effort_support` → `support_hanger_type` borrows hanger type, trims it and falls back
  to the trimmed family; `positive_finite`, `FrameDof::is_translational`, `dof_index` are scalar. `node_index`
  scans borrowed node IDs. `parse_dof(&str)` returns a scalar for exactly its explicit case spellings and an
  owned formatted error otherwise. There is no fallback to a unit, direction or force inferred from an output.
  Hanger force normalization precedes case resolution/build/publication (`normalize_model_units`,
  `hanger.constant_load` as Dimension::Force). Load-state assembly substitutes the resolved effective CASE,
  not another support-model receiver (`solve_load_case_observed`, ~5350).
- Production call census: before this repair, the classifier was called by
  `constant_effort_solve_dispositions` and `certified_publication::support`; the new site is the third direct
  caller. The dispositions wrapper feeds once-per-invocation consumption diagnostics, per-case load ledger,
  per-case applied-load rows, and PF rendering/gate facts. These existing callers still execute. No cache or
  reuse is introduced; their allocations must not be subtracted to pay for the extra call.
- Enclosing reach remains the existing typed/value ordinary entries and permitted observer path:
  `run_linear_static_preview_*` → captured/observed inner → case loop (base, resolved load-state or selected
  modulus basis) → `solve_load_case_observed` → `solve_prepared_load_case`. The existing PF held-combination
  closure can also call this wrapper, then render its private held solve. `solve_load_case` is a forwarding
  wrapper; the direct prepared caller outside the wrapper is a test. A recursive caller check also identifies
  `modal/outcome.rs::run_modal_dense_with_budget` → `modal/state.rs::solve_modal_state_case` (~317/356) →
  `solve_load_case_observed`. That caller reborrows `Prepared.model/built/materials/boundary/state_resolution`,
  uses a local nonlinear basis stiffness and scratch source budget, passes no capture/observer, and runs once
  only when built nonlinear supports exist. A distinct CE record in that model can still reach the new branch.
  This modal SAME-RUN STATIC STATE SOLVE must be included; it is not excluded by D1 or confused with the later
  eigenproblem. No modal source change or turn is authorized by identifying it.

### 2. Added live storage and resource obligations

Let S be the number of model supports reaching the loop, R_s the authored restraint-entry count of support s,
T_s the number of translational tokens visited before exit, N the model node count, and B_s the byte length of
an invalid token. These are source dimensions, not newly admitted limits.

| Site/owner | Added obligation and actual lifetime |
|---|---|
| Predicate | One extra no-NL/type test per support with an existing node and no source action; borrowed string trimming/comparison can scan its consulted bytes. No string owner is made by this predicate. |
| `translational: Vec<FrameDof>` in classifier | `Vec::new` starts empty; each recognized translational ENTRY pushes once. Logical len ≤ R_s; successful consumption has len exactly one, but the vector may have collected many entries before returning MultipleTranslationalRestraintDofs. No universal three-entry/six-entry bound exists at this typed PP interface: duplicates and malformed CE restraints are intentionally non-consuming warnings. Capacity/growth/reallocation envelopes need the maintained Vec rule and actual target layout, not a guessed multiplier. Rotation-only/empty restraints make no backing allocation. |
| `parse_dof` error plus `raw.clone()` | Invalid CE tokens can reach this site: CE is omitted from ordinary linear-support building, so that builder's DOF refusal is not a reason to exclude the branch. The parser formats `unsupported frame DOF {value}`, then the classifier constructs `UnparseableRestraintDof(raw.clone())`. Account the formatted message and cloned B_s-byte token, plus any previously grown translational vector. Until compiler/drop scheduling is established, include a conservative overlap; do not credit immediate deallocation of the parser error. The new caller discards Err rather than formatting `unmet_condition`. This edge is variable text, not a fixed label. |
| Result/frame | One temporary Result/application/error value plus iterator/reborrow state in the live static-solve call stack. `FrameDof`, enum padding, Vec header, Result niche/size and inlining are actual-build layout questions. No guessed stack-byte count or compiler spill claim is made. The already-existing 48-byte vector is not a second new vector. |
| Lifetime union | The per-support classifier's vector is dropped within the call; a returned error's String is dropped by the new if-let statement. Nothing is stored across support iterations. The new temporary can overlap the current case's assembled ledger/force, stiffness/built model, solved displacements/reactions, pressure/thermal/load state, optional selected nonlinear/confirmed state, member bound/certified formation source, accumulated results/recovery records/preview record and invocation/observer custody. Those owners are not new, but the new allocation must be added to their actual live union. It does not overlap the later classifier invocation inside certified publication: the first call/if-let has finished before that call starts. |
| Modal state caller's parent union | `Prepared` retains the original borrowed request plus its owned normalized model/materials/build/boundary/bound-state/resolution and diagnostics (`modal/prepare.rs:27`). The static state solve additionally owns stiffness, scratch budget, diagnostics and the returned LoadCaseSolve until it projects typed nonlinear evidence. New CE temporaries overlap that parent union. `run_modal_dense_with_budget` calls it BEFORE mass assembly, pencil formation and eigensolving (`modal/outcome.rs:277`), so those later mass/pencil/solution owners are not simultaneous at this site. The prepared owners are not zeroed. |

Per prepared case, new classifier invocations ≤ the count of qualifying CE records ≤ S, in sequence. Each scans
at most R_s restraint entries and, on its single-translational success path, scans up to N node records again;
this duplicate lookup is real work. Total new logical work is bounded in these source dimensions, not by a new
fixed input cap. Heap peak is a sequential maximum of the added per-call temporaries plus live parent owners,
not S simultaneous Vecs; a maximum-live proof still needs actual capacity/reallocation/drop and frame treatment.

**D1 warrant and its limit.** `retained_memory.rs` D1.6 (~1032) rejects every hanger record and any family outside
anchor/guide/line_stop/vertical_support/spring, as well as nonlinear/direction records; D1 also excludes authored
load states. With those exact receivers, hanger type is absent and the CE family predicate is false, so the NEW
classifier edge is unreachable in an admitted D1 call. The predicate itself still executes. Calls outside D1,
including ordinary fallbacks after refused admission, do execute the classifier; the fact D1 refuses them is NOT
a bound for their ordinary heap. It also does not show unchanged compiled frame/stack size, unchanged registered
source identity or actual-current-predecessor qualification. D1.6 is a conditional source warrant to review, not
an automatic zero-cost rule to install.

### 3. Publications and consumers: value changes versus unchanged paths

| Path and receiver | Impact / warrant |
|---|---|
| Case force / applied-load row | `case_force_ledger` and `append_constant_effort_support_results` use the same unchanged normalized classifier and input. The physical force and applied-load row stay identical; there is no second assembled load. Missing/nonpositive/ambiguous/invalid/unknown-node records remain unconsumed. |
| Exact ordinary fallback | `append_signed_support_results` (~13592) receives the corrected vector: six components plus force/moment magnitudes, same IDs, metadata, ordering and eight-row count. Only consumed CE axis value and its force norm change from the erroneous zero to F. No recovery-record/certificate object is added by this branch. Non-CE support paths, no-record cases and nonlinear-field precedence retain their original branches. |
| Certified path | `certified_publication::support` (~320/376) still creates its own exact CertifiedRow from the classifier and returns its existing rows; caller still overwrites vector, creates the same `support_record` groups (support_force, support_moment and two norms), and computes certified norms. Correctly certified CE row bytes and member-relative standing machinery are unchanged. Only the earlier extra temporary/call changes resource work. |
| PF ordinary renderer | `preview_physics::CaseRecord.support_vectors` now stores the corrected preliminary CE vector, with the same existing ID clone/tuple/slot. However `support_dispositions` (~221) already independently classifies CE and returns its positive-force vector or withholds a non-consuming record before consulting that tuple. `render` (~589) constructs the final eight rows from that disposition. Therefore PF CE wire output was already correct; the new PF all-axis demotion test is a compatibility control, not independent evidence that the new fallback branch caused its output. The naturally sensitive EX test distinguishes the repair. |
| Combination producer/certificate | `exact_combinations::build` (~812) gathers whole six-component support groups, evaluates each using the existing expression, clones row templates and derives the norms (or range values); corrected fallback operands therefore yield corrected combined CE actions and changed numeric receipts. Fixed-load κ gates, unsigned/range rules and state/operand standing are unchanged. A difference of equal CE forces remains zero. `combination_certificate::operand` is still created only when member_bound exists (~7376); the repair creates no certificate for Sensitive/no-bound fallback. PF gates/rendering already consume the correctly restated PF actions. |
| Case state / receipt | `case_state::resolve::validate_case` still requires every support's explicit active_model_device state; inactive/locked states and omission are refused. No new “unused support” interpretation is introduced. Stored ordinary load-source selection remains independent from CE device force. `qualify_source_case_rows` clones existing result records/IDs and ordinary source finalization can bind their corrected values; source-selected CE recovery remains excluded by `source_recovery::prepare_sources` and reader `source_blocks::solver_built_support`. A changed ordinary receipt/hash is a consequence of corrected values, not permission to alter unrelated carriers. |
| Maintained readers | `reader_verdict::pressure_outcome` dispatches unchanged pressure evidence/model/combination/nonlinear/certificate checks; `physics_evidence` checks support slots/coverage and nonnegative norms; `physics_model_checks` binds the support ID. `exact_combination_evidence::model_checks` independently reconstructs consumed CE and its entire APPLIED row, while combination derivation checks downstream rows. None of those base EX checks is an independent law proof that a fallback CE action equals its consumed force: main's bad-zero reproduction passed the model-bound reader. This is an explicit existing assurance gap, covered here by producer equilibrium/axis regressions, not silently repaired or advertised as reader protection. PF readers additionally check attributed/withheld row partitions and norms. Reader source is unchanged; no new exception. |
| Reports and stress-neutral export | TS `reportResultRows` maps each source row's value/unit into display text; Rust report-package `expected_report_result_rows` (~145) derives the same Value records and binds them. Stress-neutral `normalizeStressNeutralInputs` (~227) sorts the source rows and calls `precisionStressRow`/CSV validation; `declaredSourceResultDimension` maps Fx/Fy/Fz to force and moments separately. Corrected fallback values/norms therefore flow into output text/CSV/checksums. No extra row, alternate CE source, unit conversion or report-only force is introduced. Existing Value trees, row/template clones, formatted/display strings, CSV and serialized/hash buffers remain live consumers: their byte contents/lengths may change from zero to F. They are not zero-heap or excluded because their implementations are unchanged. No whole-App bound follows from this inventory. |
| Modal state consumer/resource path | `solve_modal_state_case` (~380) reads only the returned typed nonlinear_state and static diagnostic codes/severities for admission, then linearizes the final states; it does not consume CE support ResultItems as eigen stiffness/mass or final modal result rows. The static solve still constructs those ResultItems, so their allocations cannot be omitted. Force formation and nonlinear state are unchanged, hence no new modal mathematical effect is inferred; actual composed-source resource proof must cover the extra classifier in this same-run T1 state-work path. Current P0's existing unbounded-term/weight-or-refuse policy is not a licence to classify CE work as zero or to change a count/weight. This CE preparation runs no modal witness and edits no modal proof. |
| Hanger sizing dependency | Existing `derived_document` (~303) clones a document, replaces each candidate with its rigid/spring role or removes it in favour of an explicit force, then its single `solve` (~366) uses the ordinary PP entry. `reaction` (~387) reads the named candidate's component and converts its unit; verification reads displacements/states. A replaced candidate is not this CE record. Other CE devices retained in that document can execute the new publication path, without changing assembled physics or sizing selection logic. DEL-04-11's ordinary-consumer dependency remains; no exact-sizing/preload/recording work or new sizing claim is released. |

This map covers the changed CE support-value flow; it does not claim that unchanged generic readers or report
builders bound arbitrary Value/Display inputs. No model ID/text bound is inferred from a sample or an unchanged
function. Native/UI plumbing, persistence and MBF routes are not altered or newly exercised.

### 4. Existing proof, material gaps and next proposed scope

- Read-only stdlib comparison establishes that the changed `solve_prepared_load_case` body differs by exactly the
  six reviewed production lines and keeps the same literal/template list; 14 classifier/ledger/caller/row helpers
  and 13 traced consumer/state/record/sizing modules are byte-identical to main00832. No constructor label,
  ResultItem/LoadCaseSolve/CaseRecord field, schema, signature or admission clause changed. This supports a
  narrow KD5T2 producer/consumer literal-inventory carry for later review, not a pin replacement now.
- Existing 29 passing focused Rust tests establish consumed force, units, sign/global axes, zero remaining
  components, both modes, natural EX sensitive/no-NL publication and global force/moment balance, preserved
  certified/PF/state/non-consumption/combination/receipt behaviour. They are not allocator/stack observations.
  Invalid short token and multiple-translational non-consumption controls exist; repeated long lists, long invalid
  tokens, allocation-capacity peaks and drop overlaps are not resource-tested. CE-plus-modal-state-call resource
  composition is an identified proof consumer, not covered by these 29 CE tests. No additional build/test run was
  needed for this source inventory. Existing report projection parity tests and stress-neutral row-preservation
  tests provide generic consumer coverage; no CE-repair-specific report/export App walkthrough is claimed.
- The two L/E CE source-warrant checks remain exactly failed at
  `KD5T2: source pin changed: core/product_physics/src/lib.rs:solve_prepared_load_case`
  (raw `/private/tmp/swbpipe-rt-s3-ce-old-guard-check.log`). No guard/pin/rule/law/profile/cohort change was made.
  D1's exclusion does not discharge the new call's ordinary/error/live-frame obligations. The variable-length
  parser-error/clone edge and authored-length Vec are explicit, not folded into fixed-label TEXT or silently
  excluded as existing helper work.
- **Next bounded proposal after Agent 0 selects a concrete composition/predecessor:** review this exact changed-edge
  reach/receiver/owner/lifetime inventory and its D1.6 warrant; map the new call and predicate to the maintained
  source/cap/Vec/string/stack rules, including invalid-record paths, actual parent live union and modal T1's
  same-run static-state caller (Prepared custody, with later eigen owners disjoint at this site). Determine whether
  the selected D1 registered domain needs only a reviewed unreachable-classifier carry plus predicate/frame/source
  identity treatment, and separately state the ordinary outside-D1 resource residual. Carry the unchanged fixed
  label/record/route-string facts only where those exact producers/consumers were checked. Any generic Value/Display
  edge without a governed bound stays an explicit residual; extending a domain, imposing a new cap, changing a law
  or selecting an exclusion requires a concrete separate decision, not an automatic zero rule. Preserve runtime
  consumption semantics; no classifier refactor or input restriction is proposed in this preparation.
- Remaining inputs are the selected actual predecessor and composed source, current rule/registration basis at that
  turn, actual target/toolchain/layout/stack evidence and governed rule for allocation growth/text, and the separately
  authorized D3/AC-004/CI/publication sequence. Modal retains the selected turn. This candidate is not frozen,
  repinned, rebased, queued, registered or published, and no witness/resource/measurement run is requested by this
  inventory. Routine AppAgent0 reservation handshakes are no longer a preparation dependency under the updated
  owner policy; wrappers, allocation and qualification rules remain unchanged.
