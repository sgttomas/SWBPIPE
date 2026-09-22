# DEL-12-05 notes — Security threat model (W3, PKG-12, worker G2)

Forward ledger sealed: `DEL-12-05_forward.csv`, SHA-256
`fc28af0d272e443dc798f6dae53dee17e9d4ab9dc08f8c0cce595b65f762dd57`
(158 rows: 91 required keys, optional blocks CLM-003, CLM-011, CLM-012,
CLM-020, CLM-026, CLM-028, CLM-033 and CLM-037 split all-or-none, and
sub-claims `SOW.s01`, `CLM-027.s01/.s02` and
`CONTEXT#architecture-basis-injection.s01/.s02`). All evidence was read from
the freeze checkout at `00115c719`. Every value here is an agent judgment, not
an owner ruling.

## Path aliases

- The product artifact is `projects/chirality-piping/docs/security/threat_model.md`
  (created 2026-05-02, b97121d; last changed 2026-06-06, d2defca8a). Its
  focused text guard is `tests/test_security_threat_model.py`.
- The desktop threat packet is
  `apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx`
  (`deliverable_id: "DEL-12-05"`). `App.test.tsx` asserts its
  `runtime_control_evidence` fields.
- SOW threat IDs `STM-T-001..010` map to doc IDs `STM-001..010`, except that
  STM-T-008 (supply chain) appears in the doc as `STM-011`. The doc adds
  STM-008 (public examples), STM-012 (authority overclaim) and STM-013..019
  (SCA-004 export).
- The `INIT.md` references in CLM-012.r01 and CLM-026.r02 point to a file
  removed on 2026-07-04 (9c4caf8fd).

## Judgment calls

- **Threat-model content rows** (STM-REQ-001..012, STM-T-001..010, CLM-006,
  CLM-009, CLM-010, CLM-035, and the themes and principles except explicit
  disclosure) are `ALIGNED`, because `threat_model.md` records each element.
  They are claims about threat-model content, not about implementing the
  controls. STM-REQ-009 and STM-REQ-010 are MEDIUM confidence: the warning
  classes are named fully only in the desktop packet, and the three-state
  authority distinction appears as mechanics/rule-check separation plus human
  acceptance boundaries.
- **Setup-era "do not write the product artifact" text (FG-DEL-12-05-01):**
  CLM-008, CLM-016, CLM-021, CLM-025, CLM-026.r04 and CLM-028.r07 are
  `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE`. Each was first present at
  `7bee9ae41` (`git log -S`). The CP-03 declarations CLM-013, CLM-022 and
  CLM-042 are their own `ALIGNED` rows and do not change sibling rows.
- **Explicit disclosure vs DEC-051 (FG-DEL-12-05-02):** CLM-011.r02 and
  CLM-033.r02 are `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING ·
  PROJECT_BASELINE · SECURITY;RECORD · OWNER`, MEDIUM confidence.
  - DEC-051 (D-T0-04 open residency, register D-24) allows owner-configured
    model-provider egress of private data with no app-side guard, opt-in gate
    or indicator. It amended OPS-K-PRIV-1 and added IP_AND_DATA_BOUNDARY s6.1.
  - The explicit-disclosure theme and principle require warning or review
    before private data leaves local control. `threat_model.md` records no
    model-provider trust boundary.
  - DEC-051 held its conforming follow-on and did not list the threat model.
    No live agent binding exists at the freeze.
  - The owner decides at R4 whether this is a real divergence.
- **Update triggers (FG-DEL-12-05-03):** CLM-036 is `PARTIALLY_IMPLEMENTED ·
  DOC_BEHIND_CODE · LOCAL_DESIGN · RECORD`, MEDIUM confidence.
  - The trigger "report/export behaviour changes" fired twice after the doc's
    last change: 9457565c2 (2026-07-22, redaction enforced across export
    surfaces) and 51e7f1e54 (2026-08-21, local-first export routes).
  - The doc still lists the redaction workflow as TBD.
  - The SCA-010 rename, which is not a listed trigger, also left the doc
    unchanged.
- **SEMANTIC_READY declarations (FG-DEL-12-05-04):** CLM-003.r11, CLM-020.r09,
  CLM-026.r06, CLM-027.s01 and CLM-028.r06 are `STALE_REVIEW_OR_EVIDENCE ·
  RECORD_DRIFT`, following F3's rule for review states. `_STATUS.md` says
  IN_PROGRESS. This run makes no lifecycle change.
- **Settled TBD (FG-DEL-12-05-05):** CLM-037.r05 (package/container format)
  and the Still TBD element of the architecture-basis injection (`.s02`) were
  settled by SCA-003. `threat_model.md` itself already records SCA-003.
- **CLM-040** ("no active source conflict in this setup run") is `ALIGNED`,
  because the statement is bound to the setup run.

## Canonical departures

None. CS-01, CS-02, CS-04, CS-06 (OK) and CS-07 are inherited as assigned.
The CP patterns used are CP-01, CP-02, CP-03, CP-04, CP-05 and CP-09.

## Convention friction

- The threat model's substance lives in the product doc, not the SOW. Rows
  that assess threat-model *content* can be `ALIGNED`, while the doc's
  *currency* is judged once, on CLM-036. This keeps F1 intact: no aligned row
  records a gap in its own claim.
- `STATUS#remaining` is pre-typed NON_NORMATIVE and empty, so it is
  `NOT_ASSESSED`.

## Smallest checks for UNKNOWN rows

None. The ledger has no UNKNOWN rows.

## Did the reverse pass change my view of anything sealed?

No change to any disposition.

- The routing file confirms that the threat doc (RC-12-0014) and the desktop
  packet (RC-12-0308) are DEL-12-05's.
- Only the packet's PDU-027 slice is keyed explicitly (CLM-002, CLM-015,
  CLM-024). The rest of the panel follows the packet identity.
- Rename residue in active identifiers that the SOW does not cite:
  `openpipestress.technical_preview.security_threat_model_review` and the
  `openpipestress-preview-security-threat-model-*.json` download name. The
  product doc's body also still says OpenPipeStress. All of these are noted on
  the SOW SURFACE CP-04 row.

## Batch consistency

`validate_ledger_v2.py --batch` over the DEL-12-04 and DEL-12-05 forward
ledgers returned **PASS, 0 consistency findings**. The shared bodies are the
CONTEXT CS rows and the architecture-basis injection `.s01/.s02` sub-claims,
which are judged identically in both ledgers.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

## Items for the verifier or owner

- Protected-check, INVARIANT or ISSUED rows: none.
- Owner items:
  - the CP-04 surface row, including the product doc and active identifiers;
  - FG-DEL-12-05-02, whether DEC-051's model-provider channel requires the
    threat model and the explicit-disclosure principle to change;
  - FG-DEL-12-05-03, a threat-model review against the July and August export
    changes.
- Authority conflict: FG-DEL-12-05-02 is recorded as a ruling-redirected
  record, not an `AUTHORITY_CONFLICT`, because DEC-051 is a single ruling that
  governs.
- Possible defect: none.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
