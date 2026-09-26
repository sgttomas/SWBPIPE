# ROOT's selection and freeze of the RF-ELOAD references

HELP_HUMAN (ROOT), 2026-09-26, under the owner's delegated correctness authority. ROOT relayed it to the T3 manager by SendMessage, and the manager records it here.

**Frozen:** the RF-ELOAD references, revision 1, at commit `b6927f783`:

| File | sha256 |
|---|---|
| `REFERENCES_ELOAD/references_eload.json` | `8240765095481e08ca3153eb89ef7b5828018615f3dd774ff3eed5d7af4549c9` |
| `REFERENCES_ELOAD/references_eload.py` | `3601db34fd17df3d2397bb8e1152faac90d04c58913b5fc54517017f36581f87` |
| `REFERENCES_ELOAD/README.md` | `7a0c1aadde41f41222d1d50055e48d481067108c46f7f31a1505203fbb162c83` |
| `REFERENCES_ELOAD/_run_records/SHA256SUMS` | `cc16cea55a9deeec8b26c034d1844442b255050ce447f9babe43cbd68f12c319` |

That is 49 cases, 2142 expected values and 216 negative controls, of which 194 discriminate and 22 are labelled. The represented basis applies to RF-ELOAD-CANCEL-SEIS-G1e7, -G1e8 and -G1e8-R. Verify the checksums from the `REFERENCES_ELOAD/` root.

## Basis

- **The brief:** `TASK_BRIEFS/R1_ADDENDUM_ELOAD.md` (D-11, D-14), and the manager's definition answers in `MANAGER_NOTES/RF_ELOAD_DEFINITIONS.md`, accepted by ROOT with its self-weight ruling.
- **The author's return:** [REFERENCES_ELOAD/README.md](REFERENCES_ELOAD/README.md), written product-code-blind, by two routes (A: tree integration and the force method; B: exact direct stiffness).
- **V3's independent refutation** by a third route ([REFERENCE_CHECK_ELOAD/RETURN.md](REFERENCE_CHECK_ELOAD/RETURN.md), `3bb46bc62`). Its verdict was FINDINGS with no value wrong: 2111/2111 values and 382/382 represented values agree, and all 213 controls were rebuilt.
- **ROOT's rulings on it** (`ROOT_RULINGS_V1.md`):
  - the D-14 clarification: exact from the binary64 inputs as the document stores them;
  - the variant G1e8-R added, with G1e8 left byte-identical;
  - the NOTEs applied in revision 1.
- **V3's delta check of revision 1** (RETURN.md addendum, `ab47f6f3`; SHA256SUMS `d0868038`; commit `1daa512d4`), CONFIRMED:
  - G1e8-R re-derived by V3's own route: 31/31 values, and all 4 controls discriminate;
  - all 48 pre-existing cases (2111 values, 382 represented rows) string-identical to revision 0, by V3's own script;
  - 1 control retired, 16 reworded, 22 labelled, and no result changed.

## Rules

- **No further changes.** No slice may change `references_eload.py` or its output. A mismatch goes back to ROOT; it never changes a reference, scale or tolerance. Any later change needs a new revision and an independent refutation.
- **The comparison** is `|obs − exp| ≤ 1e-9 · max(|exp|, scale)`, with the stated scales. For the cancellation cases, the binding scale is the recommended (net-governed) column. The CANCEL-FEM gross column is review-only (`gross_scale_status`). Comparisons below D1's floor are reported `not_covered`, which never counts as a pass.
- **Use.** RF-ELOAD gates F3: W1b's element, eigen, support-effort and prescribed loads, and D-14's generated loads. It is used together with the **required** kernel-level D-14 test in F3's brief: each generated load enters the ledger as the exact product of its binary64 inputs at working precision, with no binary64 intermediate.

## Errata (recorded, not applied)

The frozen files are not reopened for non-binding text.

- **N1.** The review-only `gross_scale_status` says span-B rows show "about one tenth of |expected|". The actual ratio is 0.1 to 2.0 (2× at Mb.M2.q1). The column is review-only, so nothing binding is affected.
- **N2.** README §4's both-zero bullet names "prescribed-free cases", but none of the three cases it names has prescribed motion.
- **ROOT's ruling text.** COMB-DIFF's NC-MAG-SUM is |Mb_A − Mb_B|, as computed and as the text has said since revision 0. V3 withdrew its F6. `ROOT_RULINGS_V1.md` carries the dated correction (`457c63106`).
