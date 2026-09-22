# B1 — Owner of the product solve path (`core/product_physics`, the DEC-044 loop, PHYS-007)

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. It changes no row, deliverable,
code, lifecycle state, DAG or instruction.

Path conventions: `RUN` is the run folder
(`execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`).
Freeze paths are relative to `projects/chirality-piping/` at `00115c719`. `SD`
is `execution/_Decomposition/SOFTWARE_DECOMP.md` at the freeze.

## 1. Decision

Which deliverable owns the product solve path? That path is the product
adapter `core/product_physics`, the assembled nonlinear loop in
`core/solver/nonlinear_integration` that DEC-044 gave to an integration
tranche which was never made a deliverable, and the product section and mass
routine (CAP-PHYS-007, contested between DEL-03-08 and the product-physics
owner).

**Holder: OWNER.** A WORKING_ITEMS (workflow: scope-change) run follows the
ruling. It creates or amends deliverables and issues keys.

## 2. Background

**Earlier decisions.**
- **DEC-044** (`SD:635`; register row D-16, RULED 2026-06-20). The assembled
  nonlinear loop is owned by "a new PKG-04 integration tranche bridging
  DEL-04-04 and DEL-04-01". DEL-04-04 stays the per-iteration classifier.
  DEL-04-01 stays the frame assembly and linear-solve provider. The ruling
  also says DEL-04-04's anticipated-loop wording must be re-pointed once the
  integration tranche opens. No tranche deliverable exists at the freeze.
  DEL-04-04:SOW#CLM-009 (ALIGNED) still says the loop is owned by the
  integration tranche (T1 F1, citing `ScopeOfWork.md:110-122`).
- **DEC-074 O3** (`SD:665`). This names `core/product_preview` (not
  `core/product_physics`) and three tools as shared governed infrastructure.
- **DEC-046 / D-19.** These govern the loop's convergence tolerance and
  iteration cap. They do not assign an owner.
- **SOW-051 / DEL-03-08.** The section-property deliverable. Its own forward
  rows DEL-03-08:SOW#CLM-003.r06 and SOW#CLM-011/DEL-03-08-RQ-007 (both
  ALIGNED) assign the product routine to "other deliverables". Its reverse
  answer RC-03-0091 nonetheless answers UNKEYED for
  `core/product_physics::derive_pipe_section`
  (`WAVES/W2/PKG-03/PKG-03_VERIFICATION.md` §7, lines 310-317).
- **SCA-009 annex row 22** (`execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md:126`,
  the DEC-094 coverage contract; freeze). The row covers automatic
  self-weight case generation.
  - It names the existing mass-per-length computation
    `compute_pipe_mass_per_length` in `core/product_physics/src/lib.rs`.
  - Its landing column refers to "mass properties from DEL-03-08".
  - This governing text bears on sub-question (iii). It is presented as
    evidence only, not as a ruling on ownership.

**What the code does now (freeze, evidence).**
- `core/product_physics/src/lib.rs` (17,873 lines) is the linear and
  nonlinear solve path. Both the desktop and the headless runner reach it
  (T1 F1).
- `derive_pipe_section` is at `core/product_physics/src/lib.rs:6321`. It is
  the product's section and mass routine. The Python calculator
  `core/section_properties/calculator.py:63` has no product caller
  (T12-C06).
- `core/solver/nonlinear_integration/` exists as a crate beside
  `nonlinear_supports`, `frame_kernel` and `curved_bend`.
- The PKG-03 verifier found that the product routine omits the corrosion
  allowance that the Python calculator applies
  (`PKG-03_VERIFICATION.md` §8 item 3, lines 337-339). No claim row carries
  this, because no deliverable owns the routine.

## 3. Options

These are the options as they stand in T1 F1, T3-G2, T3-G3 and T3-G6. The two
sub-questions can be ruled together or separately.

**Sub-question (i): product adapter `core/product_physics`**

| Option | What it means | Consequences |
|---|---|---|
| (a) CREATE | A PKG-04 product-physics integration deliverable. It also instantiates the DEC-044 integration owner (merge with (ii-a)). | Scope change: new SOW, DAG node and keys. 13 T1 and 9 T3-G2 capabilities get one owner. DEL-04-04 CLM-009 is re-pointed as DEC-044 requires. PKG-04/PKG-05 lifecycle closure then covers the code the product runs. |
| (b) ASSIGN to an existing member | DEL-10-05 (the runner; it calls the service "without owning it", RC-10-0109), or DEL-04-01 (it hosted the tranches, MEMORY.md:305 and :358-367 per T1). | SOW amendment for the chosen member. Its verification and Remaining lists grow to the adapter. The member's existing NOT_MINE answers are then R5 record repairs. |
| (c) Per-function keys | Keep the split. Key each adapter function into the physics deliverable that supplies the primitive (DEL-04-01, 04-03, 04-06, 05-01, 05-02, 03-02, 02-02). | Many small SOW amendments. The adapter's own glue (normalization, result rows, mode switch, identity passthrough) still has no single owner. T1 notes the PHYS area was never routed to PKG-02 or PKG-03 (O-4), so some owners never answered. |
| (d) Shared governed infrastructure | Rule it like `core/product_preview` under DEC-074 O3. | No scope change. T1 notes this leaves the product solve path with no deliverable-level verification owner. |

**Sub-question (ii): the DEC-044 loop (`nonlinear_integration`)**

| Option | Consequences |
|---|---|
| (a) CREATE the PKG-04 integration deliverable DEC-044 already ruled | Carries out DEC-044 as written. DEL-04-04's loop wording is re-pointed. CAP-PHYS-009/010, CAP-SOLVER-036..041 get an owner. |
| (b) Key the loop into DEL-04-04 | Contradicts the DEC-044 text ("does not assemble or solve the global nonlinear system"), so it needs a new ruling that amends DEC-044. |
| (c) Record the loop as explicitly unowned | Re-express DEL-04-04's PARTIAL answers as COVERS (the PKG-04 verifier's strict reading, which the verifier itself calls "weak", `PKG-04_VERIFICATION.md:357-358`) and leave the gap visible. No owner for convergence policy, friction or recovery rules (T3-G3). |

**Sub-question (iii): product section and mass routine (CAP-PHYS-007)**

| Option | Consequences |
|---|---|
| (a) DEL-03-08 owns the product routine (SOW-051 realization) | DEL-03-08's CLM-003.r06 and RQ-007 need revision (R5 record repair after scope change). DEL-03-08's verification then reaches the Rust routine, and the corrosion-allowance omission gets an owner. |
| (b) The product-physics owner from (i) owns it | DEL-03-08 stays the Python reference. Its relation to the Rust routine becomes a parity question for T12-C06 (ENGINEERING; see H3). |

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:635` DEC-044; `SD:665` DEC-074; register row D-16 | GOVERNING | Read at the freeze and in the working tree |
| `core/product_physics/src/lib.rs:6321`; `core/solver/` crate list | EVIDENCE (freeze) | File and line checked by this writer; no build or test run |
| Reverse answers RC-04-0061, RC-10-0256, RC-10-0109, RC-03-0091 | R2 sealed ledgers | Cited through T1/T3; the verifier samples covered the PKG-03 and PKG-04 answers |
| `PKG-04_VERIFICATION.md:350-362` (strict COVERS reading), `PKG-03_VERIFICATION.md:310-339` | R2 verification reports | Verifier findings; the corrosion omission is "confirmed in the frozen code" per §8 |
| T1 F1, T3-G2, T3-G3, T3-G6 | R3 PROPOSAL | Classifications only |
| DEL-04-01 MEMORY lines (tranche hosting) | CONTEXT | Worker reading via T1; not rechecked here |
| SCA-009 annex row 22 (`Vocabulary_Annex.md:126`): `compute_pipe_mass_per_length`, "mass properties from DEL-03-08" | GOVERNING (DEC-094 coverage contract) | Read at the freeze; bears on (iii) |

Verified: the file and function locations above, and the DEC-044 text. Known
only from worker notes or code reading: the corrosion-allowance omission
(verifier code reading, no test), and the call graph "desktop and runner reach
it" (T1 reading of surfaces).

## 5. Affected claims

No class in `CLASS_INDEX.csv` is wholly or partly in B1. The decision acts at
capability level, so the portion is given as capability IDs.

- **T1_UNMAPPED.csv (13), T1 F1, explicit IDs:** CAP-PHYS-001, 002, 003,
  005, 006, 009, 010, 017, 018, 022, 027 and CAP-SOLVER-037
  (PRODUCT_UNOWNED), plus CAP-PHYS-021 (ROUTING_GAP, DEL-03-06).
- **T3_OWNERSHIP.csv (15).**
  - T3-G2 (9): CAP-PHYS-004, 008, 011, 013, 014, 015, 023, 024, 026.
  - T3-G3 (5): CAP-SOLVER-036, 038, 039, 040, 041.
  - T3-G6 exception (1): CAP-PHYS-007 (OWNER_DECISION, LOW).
- **Packages and deliverables touched.** PKG-04 (DEL-04-01, 04-03, 04-04,
  04-06), PKG-05 (DEL-05-01, 05-02, 05-04), PKG-10 (DEL-10-05), PKG-03
  (DEL-03-02, 03-06, 03-08), PKG-02 (DEL-02-02).
- **Claim rows that a ruling would reopen** (not in my portion; each sits in
  another class or is ALIGNED):
  - DEL-04-04:SOW#CLM-009 (ALIGNED; re-pointed under DEC-044 if (ii-a)).
  - DEL-03-08:SOW#CLM-003.r06 and SOW#CLM-011/DEL-03-08-RQ-007 (ALIGNED;
    revised if (iii-a)).
  - DEL-04-04:STATUS#remaining/R07 is in T6-C04 D13 (B12), not here.
- **Rows known only from OtherCorrections.** None. The OBSERVED resolution on
  DEL-04-03 CLM-010.r12 (F7 library reading), named on CAP-PHYS-008, stays
  visible.
- **Related scope-change items.** H1 carries these capabilities on its route
  with BlockedOnPacket B1. The curved-bend crate (CAP-SOLVER-024..027,
  UNKEYED to DEL-04-01) is an H1/R5 keying item. W2 lists it with the
  ownership gaps, but T3 finds DEL-04-01 already its owner of record.
- **Related item, not in the portion.** CAP-SOLVER-070 (T3-G9,
  PARTIAL_UNOWNED_REMAINDER). Its unowned part, the `nonlinear_integration`
  README, is assigned to the "T3-G3 owner", so it follows sub-question (ii).
  It is an H1 item with BlockedOnPacket B1.

## 6. Risks

- **Undecided.** Changes to the only solve path users reach fall outside every
  deliverable's verification and Remaining lists. PKG-04/PKG-05 lifecycle
  closure would not cover the code the product runs. Recorded divergences go
  to no one: PHYS-008 departs from `apply_linear_supports`, PHYS-014 uses its
  own thrust path, and PHYS-007 omits the corrosion allowance (T3-G2). DEC-044
  stays unexecuted.
- **(i-a)/(ii-a) CREATE.** A new node needs DAG edges and keys. There is a risk
  of double ownership with DEL-04-01's hosted tranche history until the R5
  repair lands.
- **(i-b) ASSIGN.** The assigned member takes a very large surface. Its SOW
  envelope may be exceeded (the context envelope has to be re-checked).
- **(i-c) Per-function keys.** Glue code stays unowned. Several owners never
  saw the capabilities (routing blind spot O-4).
- **(i-d) Shared infrastructure.** No verification owner for the product
  solve path (T1).
- **(ii-b).** Needs an amendment of DEC-044 itself.
- **(iii-a) vs (iii-b).** Either leaves a Python and Rust twin. The parity
  question goes to engineering (T12-C06, H3).

## 7. Recommended routing

For sub-question (ii) the evidence supports one reading: DEC-044 already rules
option (a), and what is missing is only the deliverable that the ruling
anticipated. The owner may confirm and have DEC-044 executed through
scope-change, or amend it. For sub-questions (i) and (iii): no recommendation;
owner's call. T1 states that option (i-d) would leave no deliverable-level
verification owner. That is a stated consequence, not a recommendation.

## 8. On-ruling mechanism

- **(i-a)/(ii-a).** A scope-change handoff (WORKING_ITEMS, workflow:
  scope-change). It adds the deliverable to `SOFTWARE_DECOMP.md` and the DAG,
  issues keys, and re-points DEL-04-04 CLM-009 as DEC-044 requires. The run is
  re-routed for these capabilities only. R5 record repairs of the NOT_MINE and
  PARTIAL reverse answers follow and need separate R5 authorization.
- **(i-b)/(iii-a).** A scope-change handoff amending the chosen SOW(s). Then R5
  record repair.
- **(i-c).** A scope-change handoff issuing keys in each named SOW. R5 repair of
  the reverse answers.
- **(i-d).** A register/decision-log entry by owner ruling (DEC-074 O3 style).
  R5 records the shared-infrastructure status in the reverse answers. No scope
  change.
- **(ii-b).** A new DEC amending DEC-044, then a scope-change handoff for
  DEL-04-04.
- **(ii-c).** R5 record repair only (DEL-04-04 answers re-expressed).
- The corrosion-allowance observation becomes a CODE_FIX_CANDIDATE only once
  an owner exists (H2, BlockedOnPacket B1). Nothing executes until the owner
  acts.

## 9. Dependencies

- **Blocks.** H1 items for the listed capabilities, and any H2 brief on
  `core/product_physics` (for example the corrosion allowance), and T12-C06
  parity work for DEL-03-08 (H3).
- **Related.**
  - B12 D13 (mechanics residuals, DEL-04-04 R07 result vocabulary).
  - A1 (DEC-009 engine placement; the Python section calculator is a
    twin of the Rust routine).
  - B4 (CAP-FEATC-031 self-weight GUI; its engine CAP-PHYS-028 is DEL-05-01
    UNKEYED).
- **Depends on.** None.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
