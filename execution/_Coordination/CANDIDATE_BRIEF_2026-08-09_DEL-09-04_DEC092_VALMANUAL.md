---
doc_id: CB-2026-08-09-DEL0904-DEC092-VALMANUAL-001
doc_kind: working_items.candidate_brief
status: candidate_unadopted
prepared: 2026-08-09
package_id: PKG-09
deliverable_id: DEL-09-04
base_sha: 81c376b41a1e181d3edb0737d4f3c9e398527dbe
selection_authority: AGENT_0
posture: TERMINAL_FAN_OUT_IN
owner_adoption: none
---

# CANDIDATE Brief — DEL-09-04 DEC-092 Validation-Manual Derivative

**CandidateBriefID:** `CB-2026-08-09-DEL0904-DEC092-VALMANUAL-001`

**Status:** `CANDIDATE / UNADOPTED / NO EXECUTION AUTHORITY`

**Prepared by:** WORKING_ITEMS for exactly `PKG-09 / DEL-09-04`

**Run:** `HELP-HUMAN-PIPING-20260809-DEL0904-DEC092-VALMANUAL-CANDIDATE-R25`

**Instance:** `WI-PKG09-DEL0904-CANDIDATE-R25`

**Exact base:** `81c376b41a1e181d3edb0737d4f3c9e398527dbe` (clean detached `HEAD`)

This proposal does not adopt itself, authorize writes, execute the derivative,
change deliverable state, or imply lifecycle, release, validation acceptance,
publication, or professional reliance.

## 1. Objective

Regenerate the DEL-09-04 validation-manual derivative for implemented
D-45 O-B / `DEC-092` behavior:

- explicit user-entered temperature-point shear modulus `G` with provenance;
- exact-ID consumption of the selected point's `G`;
- strictly adjacent linear interpolation with both source-point IDs and method;
- blocking at/outside range endpoints, without extrapolation;
- blocking for missing/invalid selected point `G`, without base-`G` fallback;
- distinct base-`G` anti-fallback witness and preserved no-basis base-`G` path.

Produce one deterministic `DRAFT_EVIDENCE` mechanics case page and one index
row. Historical case-page run metadata must remain unchanged.

## 2. Accepted Live Basis and Calibration

The adopted executor resolves:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

- Latest valid receipt: `Receipt-94`, examined through
  `182610bebaed1d3c02f2fad1add59c6859fa6f16`; receipt-ledger SHA-256
  `b28e9d43e33ceb3d3d644f12d2bd100e10c818d79187a4d9e6318d1aca2ed472`.
- The committed pointer selects approved DAG-009. Pointer SHA-256:
  `04f24cd88d16b38d6da00ae2dee32f0387381ee41659de2e352cba07127736e3`;
  approval SHA-256:
  `f25526c4e0eec239f5d3464ca4d8e0ab8c9638ebd035bc9aa282def33989337b`;
  edge-ledger SHA-256:
  `4293cbe39ff794f74da7031c2f0e2706003fadb666ca4d85f0e7d3ec25baa9cc`.
- DEL-09-04 has eight ACTIVE `EXECUTION / UPSTREAM` rows and all are
  `SATISFIED`: `DAG-002-E0286..E0289` and `DAG-002-E0543..E0546`.
- DEL-09-04 is `IN_PROGRESS`. `_STATUS.md` SHA-256:
  `baa18b13c1ed9b6a639d6ee783647f8d3020f3729f208b7ee369e478147f247c`.
  Its three Remaining bullets comprise two separate owner-gated residuals and
  the selected third DEC-092 derivative-regeneration bullet.
- `ScopeOfWork.md` is `SOW_V1` (SHA-256
  `adc30cb6ef34bdda9c6e53d75e55c9c8fd44f218fb2878014f2f3f76cd9bf1be`).
  `OUT-001` / `AC-001` / `VER-001` require traceable evidence, visible gaps,
  data/provenance boundaries, and no release or professional overclaim.

Receipt and topological-wave prose are derivative maps, not substitutes for
the live status, DAG rows, implementation, tests, and evidence below.

### DEC-092 implementation/evidence basis

- Implementation commit `c394365ca72b8383c7d7203ce5be2cb9ea67d508` is an
  ancestor of the exact base; merge commit `9f66bf7c1`.
- Receipt 87 records implementation closeout and deferral of this derivative.
- Producing run record:
  `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-08-02_DEC092_TEMPERATURE_G_IMPLEMENTATION.md`
  (SHA-256 `0a644ac4f8b1ac3c843dee328fdf973a12279a956aab01ae01cfada611a50d1f`).
  Section 7 routes regeneration to DEL-09-04.
- Passing commit-bound sweep:
  `validation/evidence/sweeps/SWEEP_20260803T194132Z_c394365ca72b.json`
  (SHA-256 `7c15d42cd369c24f883a32192b069458da5eecbaba8c97d87a65735b3daee97b`).
- Independent note:
  `validation/hand_calcs/mechanics/tp_dec092_temperature_indexed_shear_modulus_torsion.md`
  (SHA-256 `14ab694d160b03e137e3bbd3d5fd229af4e669044bbdacaa3c5c627d5a576d26`).
- Mechanics fixture/oracle source:
  `validation/benchmarks/mechanics/src/lib.rs` (SHA-256
  `8f015dfeeb1fd670065f467335419ab0da8b0c9a4e6d6b3fb565ceea2d6f6a26`).
- Product resolution/tests: `core/product_physics/src/lib.rs` (SHA-256
  `53f9db1d947908790d565dc13a51642ae8ffd5d7b4415304ec9db279b8151251`).

These prove implemented development behavior, not public tolerances,
`MAINTAINER_REVIEWED` status, GUI validation, release, or reliance.

### Current derivative facts

- Generator SHA-256:
  `d2025ad7bf3dae3dd52bf96ca21da28eff3aacc501789dc4ab28f6d39fb15063`.
  It registers 63 cases, applies global 2026-07-10 evidence metadata, rewrites
  every page, and has no DEC-092 case.
- Index SHA-256:
  `98501e65212f5f1d78ec8bdbd0a67d57ea6a2b0ce92fa049638dc8c4fc86c989`;
  no DEC-092 row exists.
- Older index authority/inventory prose is a pre-existing derivative map.
  Preserve it; stop if truthful addition mechanically requires unrelated repair.

## 3. Scope and Exclusions

After adoption only, extend the generator minimally for case-specific evidence
metadata and exact-fixture generation/check rendering; register/generate one
DEC-092 mechanics page; add one index row; and, after checks pass, record
DEL-09-04 closeout and remove only its third Remaining bullet.

Explicitly excluded: threshold/DEC-046 promotion; `MAINTAINER_REVIEWED`
promotion; GUI validation; clean-checkout reproduction; implementation,
schema, fixture, hand-calc, benchmark, test, or evidence edits; existing case
page changes; first/second Remaining-item changes; lifecycle/stage/release/
publication/reliance effects; DAG/dependency/decision/decomposition/register/
PRD/receipt/workplan/foreign-deliverable writes; unrelated residual repair;
network/install; and Git/PR/merge/external acts by the execution node.

## 4. Future Work Graph and Sequence

Plan `v1`; selection `AGENT_0`; posture `TERMINAL_FAN_OUT_IN`.

1. **N0 — adoption/branch gate.** Require explicit owner `ADOPT` or `AMEND`.
   Then route CHANGE to create/switch to a `codex/` branch from the accepted
   live base before any persistent execution write. Re-read every live basis;
   stop on drift, overlap, detached HEAD, or direct-main execution.
2. **N1 — one bounded Agent 2.** Freeze hashes and the first two Remaining
   bullets. Add case-specific metadata/reproduction fields plus exact-fixture
   generation/check mode while keeping defaults byte-stable. Register and
   generate `MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION`; do not
   inherit stale 2026-07-10 or payload-stub text. Add one index row. Run §7 and
   return changed paths, commands/exits, parity, claim matrix, hashes, blockers,
   and containment. Do not edit package state or delegate.
3. **N2 — WORKING_ITEMS fan-in.** Accept only complete N1 evidence. On PASS,
   add one History and one newest-first MEMORY entry, keep `IN_PROGRESS`,
   remove only the exact third Remaining bullet, and write one run record.
   Return to HELP_HUMAN; receipt and Git closeout remain separate acts.

## 5. Acceptance Criteria

1. New deterministic page ID is
   `OPS-VALIDATION-MANUAL-CASE-MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION`,
   status `draft_evidence`, related to DEL-09-04 and DEL-09-01.
2. It cites the exact hand calc, mechanics fixture/test, product tests,
   implementation commit, producing record, and passing sweep; no material
   claim rests only on receipt/topology prose.
3. It accurately covers explicit point `G`, exact ID, strict adjacent linear
   interpolation, both source IDs/method, no extrapolation, missing/invalid
   blocking, no base fallback, and distinct no-basis base behavior.
4. Numeric values match the note/oracle, including selected `G`, rotations,
   and 37.5%/40.625% anti-fallback misses. Existing analytic-tier evidence is
   reported without promotion/generalization.
5. Product behavior cites product tests; mechanics-only oracle evidence is not
   misrepresented as proving product selection.
6. DEC-092-specific current evidence metadata is used. The new page makes no
   false 2026-07-10 run or obsolete runner-stub claim.
7. Exactly one index row is added; `DRAFT_EVIDENCE` and open gates remain.
8. Targeted generation reproduces the new page; full check-render proves
   63/63 old pages byte-identical and the new page current.
9. Only invented public-original data is emitted; no protected/proprietary or
   private-project value appears.
10. DEL-09-04 stays `IN_PROGRESS`; first two Remaining bullets are byte-identical;
    the third is removed only after accepted execution/checks.
11. Every required check passes and final containment has zero violations.

## 6. Minimal Exact Future AllowedWriteTargets

Held until explicit adoption and branch-first completion:

1. `docs/validation_manual/cases/generate_validation_case_pages.py`
2. `docs/validation_manual/cases/mechanics/mech-tp-dec092-temperature-indexed-shear-modulus-torsion.md` (new)
3. `docs/validation_manual/index.md`
4. `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md`
5. `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/MEMORY.md`
6. `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-08-09_DEL0904_DEC092_VALMANUAL.md` (new)

Existing pages are read-only parity subjects. Use targeted generation and a
task-specific temporary directory for full check rendering.

## 7. Declared Reads, Tools, and Checks

Reads: governing agent/project instructions; DEL-09-04 intake/state; DAG-009
pointer/node/edges; latest/Receipt-87 records; DEC-077/DEC-092 sources;
producing candidate/run evidence; exact hand calc, mechanics/product source and
tests, passing sweep; current generator/index/pages; software workflow contract
and profile.

Tools: repository read/search/edit; Python deterministic generation and checks;
existing locked/offline Cargo; registered check selection/execution and scope
validation; read-only Git/diff. No network, install, destructive action,
dependency update, release/evidence-sweep tool, Git mutation, or Agent 2
delegation.

Checks, in order:

1. Reverify clean branch basis, implementation ancestry, receipt/DAG/status,
   source hashes, and selected bullet.
2. Targeted generate plus temporary full check-render: 63 old pages identical,
   new page exact.
3. Offline mechanics test:
   `cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml temperature_indexed_shear_modulus_torsion_matches_independent_oracle`.
4. Focused product tests for DEC-092 provenance/carry-through,
   base-independence/sensitivity, missing/invalid blocking, adjacency/duplicates,
   endpoint/outside blocking, selector exclusion, and no-basis base behavior.
5. Validate frontmatter, links, source/test anchors, numeric values, required
   behavior, unique ID/row, and absence of stale run/stub claims.
6. Deterministically select registered checks from changed paths. At preparation,
   `docs/**`/`execution/**` select `harness-pytest`; `harness-self-check` is
   always selected. Re-resolve live profile; never report unavailable as passed.
7. Run claims-language, path-anchor, candidate-whitespace, `git diff --check`,
   and `validate_change_scope.py` against the adopted base and six targets.
8. Verify final status/bullet invariants and absence of every excluded delta.

No new sweep, GUI test, desktop build, clean reproduction, or release gate is
required unless live registered selection mechanically requires it; if so,
stop rather than widen silently.

## 8. Stop Conditions and Handoff

Stop on missing adoption/branch gate; material basis drift; need for any
source/test/fixture/hand-calc/evidence/existing-page/unrelated-index repair;
contradicted DEC-092 behavior; newly required tolerance/promotion/GUI/
reproduction/lifecycle/release/reliance decision; unrepairable check failure
inside the fence; first/second Remaining change; or containment violation.
A failed/blocked run leaves the third Remaining bullet open.

Derivative status: `STALE / REGENERATION_PENDING / UNADOPTED`. This candidate
is coordination evidence, not decomposition truth. Manager-node verdict:
`CANDIDATE_PREPARED_ONLY`; next owner is the human owner. After adoption,
HELP_HUMAN routes WORKING_ITEMS and the separate branch-first CHANGE gate.

## 9. Owner Adoption — None

```text
CandidateBriefID: CB-2026-08-09-DEL0904-DEC092-VALMANUAL-001
OwnerAdoption: NONE
AdoptionEvidence: NONE
EffectStatus: HELD
ExecutionAuthority: NONE
OwnerChoices: ADOPT | AMEND | DEFER | DECLINE
Verdict: OWNER_ADOPTION_REQUIRED
```

No standing approval is invoked or inferred. Until explicit owner adoption,
no future target is writable and the DEC-092 Remaining bullet stays open.
