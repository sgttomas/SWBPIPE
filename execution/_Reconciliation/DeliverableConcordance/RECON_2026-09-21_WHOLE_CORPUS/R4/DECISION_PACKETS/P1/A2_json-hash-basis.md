# A2 — JSON hash basis and the "JCS-compatible" labels

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder.

## 1. Decision

Does "canonical JSON with JCS-compatible hashing" (AB-00-04, DEC-010, DEC-017) require RFC 8785 bytes on every path, or may named paths hash sorted-compact JSON under an accurate label? The same answer settles the DEL-17-03 AC-001 wording, the MBF label, and whether the frozen-contract identifier `openpipestress_jcs_ijson_v1` needs a new contract version on content grounds.

**Holder: OWNER.** A change to a FROZEN_CONTRACT basis is an owner baseline event (T7-C02).

## 2. Background

- **Rulings.** DEC-010 adopts "canonical JSON with JCS-compatible hashing" as the schema/API/persistence baseline (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:601`). DEC-017 keeps "canonical JSON/JCS" as domain truth for local storage (`:608`). DEC-028 binds the native package to per-member JCS hashes via DEL-17-02 REQ-007 (`:619`). DEC-074 E1 (`:665`) narrowed DEL-17-03 REQ-006 on 2026-07-12 (T4B-C02). The D-41 R5 T2A relabelling is a record, not a baseline amendment (T7-C02).
- **What the code does** (read at the freeze):
  - The Python persistence service labels `SORTED_COMPACT_JSON` and says it "is not RFC 8785/JCS" (`F:core/project_persistence/service.py:7`, `:28`, `:241`).
  - The audit manifest and report package label `project_local_deterministic_json`, and the wire format accepts no RFC 8785 label (`F:core/reporting/report_package/src/lib.rs:464`; `wire.rs:251`), including where DEL-08-02 computes the hash with RFC 8785 (T7-C02).
  - The native JSON export labels `deterministic_sorted_compact_json_payload_hash` (`F:core/handoff/native_json/package.py:24`).
  - The MBF writer labels sorted-compact bytes `JCS_compatible_json_payload_hash` (`F:core/handoff/caepipe_mbf/package.py:734`, `:743`, `:790`).
  - The desktop Rust path and `F:core/serialization/canonical_json` use RFC 8785 (T7-C02; T11 R-01).
  - The frozen profile identifier is a schema const at `F:schemas/stress_neutral_export.v0.2.schema.json:388`.

## 3. Options

As they stand in the evidence (T7-C02; T11 T-01; T4B-C02 item 2):

1. **RFC 8785 everywhere.** Move the Python persistence, model-state, exporter and audit-manifest hashing, and their labels, to the shared canonical_json basis.
   - Deliverables: DEL-02-02 and DEL-08-02 SOW text is repaired after the code lands; DEL-17-03 AC-001 keeps its JCS wording and the implementation becomes the gap (T4B-C02 option (b)).
   - Code: a CODE_FIX_CANDIDATE brief over four surfaces (audit_manifest enum and report_package wire labels; exporter hash functions; persistence service; MBF label). Existing persisted hashes change, so a read-compatibility question follows.
   - Other packets: A1 (Python exporters may be ported anyway); A4 (a content change to the frozen profile would need a new contract version regardless of the rename).
2. **Amend the basis.** Amend AB-00-04 / DEC-010 / DEC-017 to permit labelled sorted-compact hashing on the named paths, and relabel accurately.
   - Deliverables: R5 repair of DEL-02-02, DEL-08-02, DEL-17-02 REQ-007 and DEL-17-09 text; DEL-17-03 AC-001 reworded to the implemented project-local basis per DEC-074 E1 (T4B-C02 option (a)).
   - Code: label-only fixes where labels are wrong (the MBF `JCS_compatible` label; the DEL-08-02 case where RFC 8785 bytes carry a project-local label).
   - Other packets: DEC-028's per-member JCS wording would also need restating.
3. **Split** (the DEL-08-02 "R4 code-change candidate"). Keep the persistence basis, but make the product audit-manifest and export labels accurate.
   - Deliverables: DEL-02-02 needs the baseline permission of option 2 for persistence; DEL-08-02 rows close after the label fix.
   - Code: label fixes on audit_manifest, report_package wire and MBF.
   - Other packets: as option 2 for DEC-028.

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| `SOFTWARE_DECOMP.md:601`, `:608`, `:619` | The ruled basis | Governing source |
| Code lines above | Four labels and two byte bases | Frozen code, lines read by this task |
| T7-C02 (`RUN/R3/TASKS/T7_CLASSES.md`) | 11 rows; three departures; three options | R3 task proposal over verifier-checked ledgers |
| T11 T-01, R-01, SS-04 (`RUN/R3/TASKS/T11_METHOD.csv`) | Six readings across 13 deliverables; "implemented once" (DEL-00-04 REQ-04-03, ALIGNED) cannot hold together with the audit-manifest finding | R3 scripted screen plus hand review |
| T4B-C02 (`RUN/R3/TASKS/T4B_CLASSES.md`) | DEL-17-03 AC-001 still asserts JCS after the DEC-074 E1 narrowing | R3 task proposal; ledger RemainingWork |
| W3 owner item (`RUN/WAVES/W3/W3_ASSESSMENT.md:70`) | "JCS-compatible" labels are not RFC 8785 (PKG-14, PKG-17) | Agent 0 assessment |
| DEL-17-02 REQ-007 Notes | Only record of the MBF mislabel; DEL-17-04 has no hash-basis row (T7 obs. 2; T11 obs. 4) | Worker note, confirmed by this task's code reading |

Verified: the labels and their line numbers. Not verified here: byte-level equivalence of any path to RFC 8785 for particular content (DEL-02-04 notes that sorted-compact coincides with JCS for its content only).

## 5. Affected claims

**This packet's portion: 12 rows.**

| Class | Class rows | Portion | Filter |
|---|---|---|---|
| T7-C02 (Authority OWNER) | 11 | 11 (whole class) | `ClassID == 'T7-C02'` |
| T4B-C02 (Authority OWNER) | 5 | 1 | `ClassID == 'T4B-C02' and DeliverableID == 'DEL-17-03'` |

Portion keys:
- T7-C02: `DEL-02-02:SOW#CLM-014/U-008`, `DEL-02-02:SOW#CLM-015.r10`, `DEL-02-02:SOW#CLM-040`, `DEL-02-02:SOW#CLM-046`, `DEL-02-02:CONTEXT#architecture-basis-injection.s03`, `DEL-08-02:SOW#CLM-004.r02`, `DEL-08-02:SOW#CLM-006.r02`, `DEL-08-02:SOW#CLM-011.r02`, `DEL-08-02:SOW#CLM-024.s01`, `DEL-17-02:SOW#CLM-018/DEL-17-02-REQ-007`, `DEL-17-09:SOW#CLM-018`.
- T4B-C02: `DEL-17-03:SOW#completion-and-reliance-basis-epistemology/AC-001`.

T4B-C02 is not listed as a split class in the topic file; its other four rows (DEL-01-01) are A6's portion. Reported to Agent 0.

**Rows on other routes that the answer settles (not claimed).**
- T6-C02 (CODE_FIX_CANDIDATE, H2) rows whose gap is the hash basis: `DEL-14-01:SOW#CLM-005`, `#CLM-011.r04`, `#CLM-012`, `#CLM-019`, `DEL-14-01:CONTEXT#architecture-basis-injection.s03`, `DEL-02-05:SOW#CLM-005.r05`, `DEL-02-05:SOW#CLM-014/REQ-02-05-005`. H2 should mark them `BlockedOnPacket = A2`.
- T5B-C10 (R5, H4): the six DEL-14-02 CONTRACT_VERSION_ADVANCED rows (T-01 reading (f)).
- ALIGNED rows that T-01 reads as resting on a label (not divergent): `DEL-13-04:SOW#CLM-031`, `DEL-02-04:SOW#CLM-005.r06`, `DEL-10-01:SOW#CLM-013/DEL-10-01-REQ-14`, `DEL-10-01:SOW#CLM-015/REQ-14`, `DEL-16-03:SOW#CLM-011`, `DEL-00-04:AB#normative-requirements/REQ-04-03`, `DEL-02-05:SOW#CLM-014/REQ-02-05-012`.
- The three FROZEN_CONTRACT rename rows for `openpipestress_jcs_ijson_v1` are A4's portion (T4B-C01). This packet owns only the contract-version question: whether the profile's content stays valid under the chosen basis.

**Rows known only from `OtherCorrections`** (not divergent; CONTESTED readings that depend on this ruling): `DEL-02-01:SOW#CLM-011/REQ-02-01-09`, `DEL-02-01:SOW#CLM-017` (Checksum.canonicalization admits non-JCS values), `DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-12`, `DEL-02-05:SOW#CLM-022.r02`, `DEL-08-01:SOW#CLM-004.r05` (report package labels model hashes project-local).

**MBF label (no row).** `DEL-17-04` owns the MBF writer and carries no hash-basis row; the mislabel is recorded only in DEL-17-02 REQ-007 Notes. Any ruling or fix should name it.

**Packages and deliverables.** Portion: PKG-02 (DEL-02-02), PKG-08 (DEL-08-02), PKG-17 (DEL-17-02, 17-03, 17-09). Affected by the answer: DEL-00-04, 02-01, 02-04, 02-05, 08-01, 10-01, 13-04, 14-01, 14-02, 16-03, 17-04.

## 6. Risks

- **Undecided.** Hash records carry labels that do not describe their bytes; in one case the label says project-local and the bytes are RFC 8785. Cross-surface hash comparison can fail or mislead. A frozen contract stays breached with nothing recorded. AC-001 keeps asserting a conformance property the code disclaims, a claims-boundary exposure on an interop contract (T4B-C02).
- **Option 1.** Changes persisted hashes; existing saved projects and packages need a compatibility path. Largest code change.
- **Option 2.** Loosens a baseline that DEC-028 also cites; two hash bases then coexist by design, and every consumer must read the label.
- **Option 3.** Leaves persistence and products on different bases; needs the same baseline permission as option 2 for persistence.

## 7. Recommended routing

No recommendation on substance; owner's call. One point the evidence supports under any option: the MBF `JCS_compatible_json_payload_hash` label and the DEL-08-02 project-local label on RFC 8785 bytes are inaccurate as labels, so a label correction is needed whichever basis is chosen.

## 8. On-ruling mechanism

- **Option 1 or 3.** An owner decision recorded in the register / SOFTWARE_DECOMP §12 authorises a CODE_FIX_CANDIDATE brief (H2) over the named surfaces, executed only under an accepted production brief. R5 then repairs the DEL-02-02, DEL-08-02, DEL-17-02, DEL-17-09 rows (and DEL-17-03 AC-001 under option 3 if the product labels change).
- **Option 2.** An architecture-decision amendment to DEC-010 / DEC-017 / AB-00-04 (and DEC-028's wording) through the baseline change path, then an R5 record-repair tranche over the 12 portion rows and the affected T6-C02 rows' text; the label fixes as a small H2 brief.
- **Frozen profile.** If the chosen basis changes what `openpipestress_jcs_ijson_v1` hashes, a new contract version goes through the baseline change path; this can be combined with A4's rename of the same identifier.

Nothing executes until the owner acts. R5 needs separate authorisation (D-73).

## 9. Dependencies

- **Depends on:** none.
- **Interacts with:** A4 (same frozen identifier; one contract-version change should carry both), A1 (Python exporters), B7 (handoff packages).
- **Blocks:** H2 hash-label and hash-basis briefs; the T6-C02 hash rows listed above; H4 repair of the 12 portion rows and the six DEL-14-02 T5B-C10 rows.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval or engineering-acceptance claim is made.
