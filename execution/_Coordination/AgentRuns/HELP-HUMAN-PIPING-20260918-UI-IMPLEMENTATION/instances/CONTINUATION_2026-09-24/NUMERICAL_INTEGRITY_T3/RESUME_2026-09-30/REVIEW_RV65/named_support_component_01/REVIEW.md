# RV65 — named support component source/design review

**FINDINGS: two implementation-plan corrections must be selected explicitly
before implementation reliance.** Both can be resolved in ROOT's same six-file
grant using the dispositions below. No fresh mathematical redesign, new support
law, seventh maintained path, public structural-zero exception or numerical
predicate change is needed. This is checkpoint-0 design review, not review of an
implemented candidate or a demonstrated publishing case.

Basis: records `f58de3f5cccc44200a2fc021a7c6d447618791e7`; final I50 packet
`848981f43349e62173b0a91850f369c3a040a1a0`; source
`d0daa18717f8243a7232e898c9ef9b4f4d18d9e4`. P means projects/chirality-piping;
PP=P/core/product_physics; FK=P/core/solver/frame_kernel. Full origins and hashes
are in ORIGINS.json. Later ROOT_CURRENT changes were not adopted.

## Required corrections for the implementation brief

1. **RV65-1 — blocking implementation-plan omission: include PP
   `ProductCapture::g5a`, not just `bind_rows`/metadata, in the
   retained_product.rs seam.** Its current loop at 1929–1946 admits only
   `ProductRecipe::Native`; changing support components to a new enum otherwise
   bypasses both the actual final force/moment scale and the
   `resolution[kind-2] == 0 && normalized_bits != +0` check. Infer kind from the
   checked component and body from the checked source group; every component,
   empty or singleton, is mechanical. In FK `run_case`, classify from the
   actual final normalized value and complete body scale, with `input=false`,
   force slot 2 or moment slot 3, existing extent coupling and p512 floors before
   the unchanged gate. In PP G5a, use those same normalized verdict bits, include
   every component in the force/moment maximum with no InputDerived exclusion,
   and apply its existing zero-bit check even to an empty law. Preserve the same extent coupling,
   uncoupled-resolution zero rule, sanity comparison, order and first failure.
   Add an isolated empty-row **-0.0** control with the corresponding zero
   resolution: its ordinary interval gate may pass but unchanged G5a must refuse.
   Also make a support component the largest final force/moment to prove both FK
   and PP scale scans include it. An empty-law nonzero final value must enter the
   unchanged numerical/G5a predicates; it must not receive a blanket exemption
   or a newly invented always-reject predicate.

2. **RV65-2 — blocking implementation-plan ambiguity: state how the old Native
   reaction recipe participates in coverage.** All
   PP support-component rows, including the existing full rigid/base and
   selected-material specimens, migrate to `SupportComponent { support,
   component }`. Only that attributed recipe fills a `(group,component)` slot;
   direct `Native(Reaction)` or `Native(SpringAction)` retains its independent
   native-coverage behavior but never fills a support-component slot. This is an
   explicit non-aliasing rule, not a blanket API rejection of direct Native rows.
   A singleton consumes exactly its native contributor, so a second Native row
   for it fails duplicate contributor coverage; replacing the attributed row
   with that Native row fails missing support coverage. No compatibility alias
   is necessary. Within admitted product capture every reaction/spring is owned
   exactly once, so no such extra direct Native row can coexist in a complete
   accepted product roster. Leave group magnitudes as their own native group quantities.
   Repository caller search finds one maintained facade-producing caller,
   PP `bind_rows`; origins.rs forwards the same API and existing FK focused
   tests construct isolated NonQuantity gate rows, not another complete legacy
   support-roster client. PP `bind_rows`, `validate_row_metadata` and `g5a` must
   handle the new recipe. FK `recipe` and all `run_case` dispatches (association,
   component/contributor coverage, scale accumulation and final gate input) must
   handle it. PP `observables` retains its six components/two native magnitudes
   checks. Preserve `observed` as the sparse wrapper, add the proposed mode-aware
   helper, and migrate all base/material test invocations through the one binder;
   no different source law or result expectation is needed. Existing FK isolated
   NonQuantity gate-helper tests remain valid unchanged. The origins.rs forwarding
   method/re-exports need no new signature or path. These changes fit the proposed two implementation and
   two test files; the fixture/test-extraction paths complete the six-file fence.

These corrections make I50 invariants 4–6 explicit at their actual consumers.
ROOT can resolve both by selecting this exact disposition in the same grant;
neither requires another source-law design packet. The original seam table alone
is not unconditional implementation clearance. No other blocking defect was
found in the bounded zero-or-one proposal. The corrections remain mandatory
checks for the later frozen diff; current source does not implement them.

## Independent warrant and source checks

- B1 SOURCE_OPERANDS' support row requires Reaction, SpringAction or the exact
  identified support-law sum, with matching support id/node/list/ambiguity.
  B1 RETURN 151–156 gives the signed finite-sum enclosure. ROOT_RULINGS_V1
  5055–5067 retains that finite algebra as a conditional basis; 5183–5202 selects
  I33's source-action bridge; 5227–5239 selects I36's ordinary reconciliation;
  5792–5813 selects the private ordinary dual-readout interface. I33 §4 requires
  actual `-k*u`, the same source-selected component/group map, and complete
  action-functional change. Its original conditional header is not independently
  presented as an adoption decision. I36 supplies explicit base E/G here; no
  E/nu material interpretation may be substituted.
- FK recover.rs 442–475 constructs each group's components from its exact lists,
  beginning at zero; source_residual.rs 1043–1086 constructs the corresponding
  source intervals from those same reactions/springs. An empty list is exactly
  zero by this existing attributed law. A singleton uses the same represented
  and source native enclosures and their unchanged hull; it is not recomputed
  from the ordinary rounded displacement. Directional or multiple-contributor
  components refuse within this bounded implementation. They do not motivate a
  general support-sum engine here.
- PP lib.rs 6662–6720/6766–6790 chooses Guide for the three-axis unnamed-family
  rigid support and Spring for each explicit spring. Linear-support boundary
  preparation 384–431 and 537–590 creates three constrained translational DOFs
  plus three scalar rotational SpringEntries. A spring's `restrained_dofs`
  stores its affected axis. PP recovery 4099–4133 initializes a zero vector,
  copies configured residual slots and then overwrites the spring slot with
  that identified spring's action. I50's phrase “copy only rigid residual
  entries” abbreviates this sequence; it is not literally the loop's family
  test, but the final scalar-spring meaning is correct.
- Capture currently refuses nonempty springs at retained_product.rs 474. The
  existing support loop 735–751 treats every affected axis as restrained and
  leaves membership empty. Removing only the guard cannot work. The proposed
  actual boundary ordinal/id map must be checked against the full authored and
  built support identity, node, family, affected axis, stiffness dimension and
  bits, with every built and boundary entry consumed exactly once. Never infer
  ownership from equal k or same-node values. Source construction canonicalizes
  child lists with dedup at source.rs 637–644, so duplicates must be rejected
  before entry. Check canonical output identity afterward. Preserve
  NonPositiveSpring for zero k: the ordinary boundary permits zero, the retained
  primitive does not. This proposal adds no exception.
- Rigid constraint ownership must be unique at the node-DOF, not merely the
  node. Preview ambiguity at preview_physics.rs 202–232/528–568 and capture of
  attributed/withheld evidence remain binding. Distinct RX/RY/RZ spring groups
  on N0 are valid; duplicate residual ownership cannot be divided or duplicated.
- FK native layout provides Reaction only for actual constrained DOFs, one
  action per scalar spring, and two magnitudes per support. New independent
  `6*g` component coverage is essential: native coverage alone cannot detect
  missing or duplicated empty rows. Group id, component, body, force/moment unit,
  and final PP entity/id/sign metadata must all agree before truth or coverage
  use. Wrong groups with equal values are association failures, not numerical
  comparisons. Full native/21*m derivative and mode/selected-basis coverage stay
  required. Existing magnitude and PP observable norm checks stay independent.

## Fixture, census and future oracle

The independent CHECK re-extracts the unique literal from the owning formation
test and compares its bytes with the delivered request: **2,747 bytes**, SHA-256
`2aa51bee095186b95463567f190198f82a69557a569bf8a6a7b318aad37ab0b1`.
The shared fixture must contain those exact bytes, including no added newline;
include_str! changes only literal storage. The existing both-entry/both-mode
Sensitive expectations at formation_check_runtime.rs 65–97 remain protected.
They are not a fresh run or a no-source-block-selection witness. The observed
helper must check actual ordinary producer/capture and source_block_recovery
absence, both modes and unchanged None-path serialized output.

From owning `recover::layout`, `source_counts`, nodal/support/force/stress producer
loops and the maximum/mode rows, independently confirmed n2,m1,t3,s3,k3,g4 and
Q=7n+12m+6t+s+k+2g=58. Final roster:
14 nodal +30 end/station +24 support components +8 magnitudes +20 stress
+1 maximum +1 mode =98; 97 are quantity/mechanical rows. Component partition is
3 Reaction, 3 SpringAction and 18 empty. Native coverage is 44 other primitives
+6 contributors +8 magnitudes=58. These remain static expectations, not observed
counts. The original base Q52/final74 and material Q52/final75 caller assertions
remain valid: a full six-DOF rigid group still has six singleton components.
Keep base W0's existing G5a signed-zero failure and all prior numerical outcomes.

The input's binary64 moment terms are exactly alpha*(1,2,2), and exact chord
length is 3. The proposed pure-torsion oracle premise is sound for the identical
linear frame/load/support law with arbitrary positive torsional coefficient:
root rotation is M_i/k_i, root rigid rotation gives tip translation
theta_root cross chord, and the relative twist is T*L/(GJ) along the chord.
E and bending stiffness do not supply a hidden cancellation-dependent load in
that ideal solution. The later checker must independently prove the actual exact
frame and both endpoint/station signs, include all rows/magnitudes/maxima/zeros,
represented K and source-annulus/base E/G plus both represented-Z meanings, and
bracket pi/norms. Rounded ordinary nonzero leakage on ideal-zero components is
still measured against unchanged predicates. No oracle was executed here, and
this premise does not establish that the unchanged output will publish.

## Checked work, storage and return boundary

The existing AdapterWork reserve/copy/entered-event machinery and FK fallible
reserve/WorkTotal machinery can accommodate this seam without a new resource
policy. Before allocations/casts/index products, check s, g, 6g, 6n, 3m and Q,
including actual layouts and capacity-byte conversion. Charge every entered new
scan/comparison/map write/copy/library call before entry; keep ownership bitmap,
child lists, retained support/spring identity maps and certificate coverage
capacities visible. Record capacities of successfully allocated temporaries on
later failure as applicable, not only successful final state. The old four-slot
FK capacities array and PP summary that omits supports are not by themselves a
complete new allocation record; extend the scoped record within the same files.

Seed every newly entered adapter/certificate counter, stop before the failing
mutation, preserve the exact successful prefix and first produced error, and
test duplicate/missing/canonicalization/storage/count failures at their intended
boundary. Call-entry counters that are historically advanced before an existing
error remain separately described. Existing numerical-error collection precedence
must not be replaced by a later accounting fault. No whole-caller cost, allocator
internals, RSS ceiling or complete memory/resource profile is established.

CHECK independently verifies all delivered base/addendum seal entries and all
nine final subject files at the preserved subject revision. Thirteen inspected
source files match the full source pin and NUM. The overwritten preliminary I50
seal/payloads remain expressly not preserved; their hashes are history only.
This review relies on the independently checked final bytes, not that draft.

No maintained edits, compiler/model/solver/native runs, Git/index/API writes,
host-tool changes or descendants occurred. Only the new RV65 record directory
was written; no external bulk was created. ROOT owns scope selection, later
implementation/runtime authorization, correction backcheck and integration.
