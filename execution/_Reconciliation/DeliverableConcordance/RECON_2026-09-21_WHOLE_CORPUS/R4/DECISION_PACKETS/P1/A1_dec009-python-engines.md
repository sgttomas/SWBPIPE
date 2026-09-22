# A1 — DEC-009 Rust core against the Python and TypeScript engines

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`. `RUN/` is the run folder `RECON_2026-09-21_WHOLE_CORPUS`. Ledger keys refer to the sealed forward ledgers under `RUN/WAVES/`.

## 1. Decision

Do Python domain engines and contract builders under `core/` (and the TypeScript packet builders that accompany them) satisfy the DEC-009 baseline, which adopts Rust core/application services? The owner chooses one project-wide answer: port, permit, or split.

**Holder: OWNER.** (Under option 3, the per-engine classification would then follow B7's product-path decision.)

## 2. Background

- **Ruling.** DEC-009 adopts "Rust core/application services, Tauri 2 desktop shell, TypeScript/React/Vite GUI, and Three.js viewport" as the implementation baseline; its status is "Accepted; exact dependency versions and component/state libraries remain TBD" (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:600`). The core language is not held open.
- **No permitting ruling located.** The Python mentions in the decomposition are dev tooling only (DEC-060, `SOFTWARE_DECOMP.md:651`). DEC-025 (`:616`) registers pytest as a gate surface, which is not permission for Python core slices (T7-C03). Under CONVENTIONS A2 a merged PR does not amend a PROJECT_BASELINE claim.
- **What the code does.** Engines and builders exist only as Python modules (existence checked at the freeze):
  - `F:core/constraints/validation/engine.py` (DEL-13-03);
  - `F:core/model_transform/physical_to_analytical/contract.py` (DEL-13-04);
  - `F:core/comparison/model_state/engine.py` (DEL-14-03);
  - `F:core/comparison/analysis_run/engine.py` (DEL-14-04);
  - `F:core/handoff/target_mapping/contract.py` (DEL-15-02);
  - PKG-17 `core/handoff/*` export builders (DEL-17-03 to 17-09; PKG-17_VERIFICATION.md W-1b per T8-K2).
- **PKG-16 Python engines not yet in the cluster.** `F:core/model_operations/validation_preview/engine.py` (DEL-16-02), `F:core/model_operations/audit_trail/engine.py` (DEL-16-03) and `F:core/model_operations/agent_rationale/engine.py` (DEL-16-04) are Python. `F:core/model_operations/operation_applier/` is a Rust crate. A script search of the four sealed PKG-16 forward ledgers finds no `DEC-009` mention (0 hits in each). T8 observation 4 also names DEL-16-02 REQ-16-02-002 and a Python contract fixture copied into the desktop for DEL-07-08 REQ-002. No verifier placed them in the cluster, and no row records the DEC-009 question for them.

## 3. Options

As they stand in the evidence (T7-C03; T8-K2):

1. **Port.** The named engines move to the Rust core per DEC-009, as the RemainingWork on DEL-13-03 and DEL-13-04 asks.
   - Deliverables: the rows stay POSSIBLE_DEFECT until ported; SOW and CONTEXT text needs no change beyond the eventual repair.
   - Code: one CODE_FIX_CANDIDATE port brief per engine (at least five engines plus the PKG-17 builders, plus the three PKG-16 engines if the owner includes them). This is the largest code consequence of any P1 packet.
   - Other packets: B7 (canonical handoff path) and B9 (PKG-13 status) interact directly, since a port presumes the engine enters the product. A2's hash-basis choice also affects the Python exporters (option (a) there moves them to the canonical_json crate).
2. **Permit.** Record a decision that amends DEC-009 to permit Python core slices.
   - Deliverables: the rows become stale records (SCOPE_REDIRECTED_BY_RULING in T8's reading) repaired at R5; the CONTEXT architecture-basis rows cite the amendment.
   - Code: none.
   - Other packets: B7 still decides whether the engines are wired into the product; A2 is unaffected.
3. **Split.** Permit Python as a contract, reference or test layer (DEL-15-02's framing), but require product runtime services to be Rust. Each engine is classified by whether it has a product caller (T12).
   - Deliverables: mixed. Contract/reference engines take option 2's record repair; product-runtime engines take option 1's port brief.
   - Code: port briefs only for the engines classed as product runtime.
   - Other packets: depends on B7 and B9 for the product-caller facts; the PKG-16 engines would be classed the same way.
4. **Per deliverable** (T7-C03 option (c)). The owner rules case by case. T7-C03 records that the R2 notes on DEL-15-02 and DEL-14-04 both say one project-wide answer settles these rows, so this option is listed only because the task output names it.

## 4. Evidence and reliability

| Source | What it shows | Reliability |
|---|---|---|
| `F:execution/_Decomposition/SOFTWARE_DECOMP.md:600` (DEC-009) | The ruled baseline | Governing source (authority) |
| Python module paths above | Engines exist only in Python | Frozen code; existence checked by this task. Whether each is "core/application services" is a reading, not verified |
| T7-C03 (`RUN/R3/TASKS/T7_CLASSES.md`) | 7 rows, all CONTESTED; cause split 4 POSSIBLE_DEFECT / 2 AUTHORITY_UNCLEAR / 1 UNKNOWN | R3 task proposal over sealed, verifier-checked ledgers |
| T8-K2 (`RUN/R3/TASKS/T8_CLUSTERS.md` §T8-K2) | Proposed reading IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · BASELINE · OWNER; 20 rows | R3 task proposal; the PKG-13 and PKG-14 verifiers (§5 C1, §4.2) both doubt AUTHORITY_UNCLEAR because DEC-009 is not silent |
| WEAK resolutions on DEL-17-03 to 17-09 ABI rows (`RUN/WAVES/W3/RESOLUTIONS.csv`) | Direct a DEC-009 `.sNN` that was never minted | Adopted resolutions; the finding survives only in `OtherCorrections` |
| T11 S-05 (`RUN/R3/TASKS/T11_METHOD.csv`) | 9 resolved-baseline sub-claims with 6 signatures; PKG-17 parents unsplit | R3 scripted screen plus hand review |
| PKG-16 observation above | Three Python engines with no DEC-009 row | Code reading and ledger search by this task only; no worker or verifier assessed it |

Verified: the ruling text, the existence of the Python modules, the absence of a permitting ruling in SOFTWARE_DECOMP §12 (T7/T8 search; not re-run exhaustively here), and the absence of DEC-009 mentions in the PKG-16 ledgers. Not verified: whether any engine has a product caller (T12/B7), and whether a contract layer is "core/application services".

## 5. Affected claims

**This packet's portion: 14 rows.**

| Class | Class rows | Portion | Filter |
|---|---|---|---|
| T7-C03 (Authority OWNER) | 7 | 7 (whole class) | `CLASS_ASSIGNMENTS.csv` `ClassID == 'T7-C03'` |
| T4A-C06 (Authority OWNER; split class) | 14 | 7 | `ClassID == 'T4A-C06'` and key ends `CONTEXT#architecture-basis-injection` |

Portion keys:
- T7-C03: `DEL-13-03:SOW#CLM-004.r05`, `DEL-13-03:CONTEXT#architecture-basis-injection.s03`, `DEL-13-04:SOW#CLM-014.s01`, `DEL-13-04:CONTEXT#architecture-basis-injection.s03`, `DEL-14-04:SOW#CLM-004.r07`, `DEL-14-04:CONTEXT#architecture-basis-injection.s02`, `DEL-15-02:CONTEXT#architecture-basis-injection.s03`.
- T4A-C06 (DEC-009 group (i)): `DEL-17-03:CONTEXT#architecture-basis-injection`, `DEL-17-04:CONTEXT#architecture-basis-injection`, `DEL-17-05:CONTEXT#architecture-basis-injection`, `DEL-17-06:CONTEXT#architecture-basis-injection`, `DEL-17-07:CONTEXT#architecture-basis-injection`, `DEL-17-08:CONTEXT#architecture-basis-injection`, `DEL-17-09:CONTEXT#architecture-basis-injection`.

The other T4A-C06 rows belong to A7 (5, export plan) and A10 (2, release-label floor).

**Rows known only from `OtherCorrections`.** The seven T4A-C06 keys carry their effective CP-02 pointer values. The DEC-009 finding (IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE · RECORD;BASELINE · OWNER, cause per the corpus-wide settlement) exists only in the WEAK resolutions' `OtherCorrections` as an unminted `.sNN`. For DEL-17-07/08/09 the WEAK resolution moved the sealed IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR finding off the effective row. R4 carries it here so it is not lost (T8 observation 2).

**T8 rows on other routes (both views shown; not claimed here).**

| Key | Class (route) | T8 route | Note |
|---|---|---|---|
| `DEL-14-01:SOW#CLM-005` | T6-C02 (CODE_FIX_CANDIDATE) | OWNER_DECISION | Member by Notes; disposition carried by the JCS gap (A2) |
| `DEL-14-03:SOW#CLM-004` | T6-C02 (CODE_FIX_CANDIDATE) | OWNER_DECISION | PARTIAL_SLICE / NO understates the owner question (T8-K2) |
| `DEL-14-03:SOW#CLM-006` | T6-C02 (CODE_FIX_CANDIDATE) | OWNER_DECISION | same |
| `DEL-14-03:CONTEXT#architecture-basis-injection.s03` | T6-C02 (CODE_FIX_CANDIDATE) | OWNER_DECISION | same |
| `DEL-17-01:CONTEXT#architecture-basis-injection` | T4A-C03 (R5_RECORD_REPAIR) | NO_ACTION | Documentation-only; Rust clause does not diverge (RESOLVED_PAIR) |
| `DEL-17-02:CONTEXT#architecture-basis-injection` | T4A-C03 (R5_RECORD_REPAIR) | NO_ACTION | Contract-only (DEC-076); RESOLVED_PAIR |

Source: `RUN/R3/T8_ROUTE_DISAGREEMENTS.csv` (cluster DEC-009, 6 rows). H2 carries the four T6-C02 rows and H4 the two T4A-C03 rows; both should mark them `BlockedOnPacket = A1`.

**Packages and deliverables.** PKG-13 (DEL-13-03, 13-04), PKG-14 (DEL-14-04; DEL-14-01, 14-03 via T8), PKG-15 (DEL-15-02), PKG-17 (DEL-17-03 to 17-09). PKG-16 (DEL-16-02, 16-03, 16-04) and DEL-07-08 are affected by the answer but carry no row.

## 6. Risks

- **Undecided.** A ruled baseline is contradicted in at least 15 deliverables with no ruling (T8-K2). It stays unclear whether new core work may be written in Python. In PKG-17 the finding lives only in `OtherCorrections`, so it can drop out of any later tranche.
- **Option 1.** Large code cost and sequencing risk; ports of engines that no product path calls (T12) may spend effort before B7 settles whether they enter the product.
- **Option 2.** Amends a project baseline after the fact. The permission then needs clear bounds, or it also covers future product-runtime code by default.
- **Option 3.** Needs a product-caller fact per engine that T12 and B7 own; if those are unsettled the split cannot be applied, and the rows stay open longer.
- **Option 4.** Seven or more separate rulings on one question, with drift between them.

## 7. Recommended routing

No recommendation on substance; owner's call. The evidence supports one procedural point only: whatever the answer, it should be a single decision record that names the PKG-16 engines and the PKG-17 builders explicitly, because neither currently carries a DEC-009 row.

## 8. On-ruling mechanism

- **Option 1 (port).** An owner decision record confirming DEC-009 for these engines. It authorises CODE_FIX_CANDIDATE port briefs, one per engine, drafted under H2 and executed only under an accepted production brief. The rows stay open until each port lands; R5 then repairs the CONTEXT/SOW text.
- **Option 2 (permit).** An architecture-decision amendment to DEC-009 in `SOFTWARE_DECOMP.md` §12 through the governing decision path (a DEC entry; a scope-change handoff if the owner treats it as changing the architecture scope). It authorises an R5 record-repair tranche over the 14 portion rows and the six T8 rows above; the unminted PKG-17 `.sNN` is recorded as closed by the amendment.
- **Option 3 (split).** The same DEC-009 amendment, bounded to contract, reference and test layers, plus per-engine classification after B7. Port briefs for runtime engines (as option 1) and R5 repair for the rest (as option 2).
- **Option 4.** One DEC entry per deliverable, then as options 1 or 2 per row.

Nothing executes until the owner acts. R5 needs separate authorisation (D-73).

## 9. Dependencies

- **Depends on:** none for the ruling itself.
- **Interacts with:** B7 (canonical handoff path and prover packet), B9 (PKG-13 status), and T12 product-caller facts, which option 3 needs. A2 (the exporters' hash basis is on the same Python modules).
- **Blocks:** H2 port briefs for these engines; H4 repair of the 14 portion rows, the two DEL-17-01/02 rows and any DEC-009 text on DEL-13-03/04, 14-03/04, 15-02 and 17-03 to 17-09 CONTEXT rows; the C03-style amendment-list repoint on the seven PKG-17 ABI rows (T4A-C06 mechanism).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This packet makes no certification, code-compliance, professional-approval or engineering-acceptance claim.
