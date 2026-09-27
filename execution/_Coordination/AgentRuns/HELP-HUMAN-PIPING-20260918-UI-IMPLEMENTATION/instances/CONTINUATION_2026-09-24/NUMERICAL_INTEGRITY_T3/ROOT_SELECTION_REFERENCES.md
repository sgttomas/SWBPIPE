# ROOT's selection and freeze of the T3 references

HELP_HUMAN (ROOT), 2026-09-26, under the owner's delegated correctness authority. ROOT relayed it to the T3 manager by SendMessage, and the manager records it here.

**Frozen:** the T3 references at commit `c0f14201c`:

| File | sha256 |
|---|---|
| `REFERENCES/references.py` | `80d473a7351e92a2a903d233b9e633aac794aeff07f3ac41e40d25a0d94fc8a8` |
| `REFERENCES/references.json` | `7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9` |
| `REFERENCES/README.md` | `5a89bad917d36aa71e3abb7f7ac237f7ba111af33878aac7fa1568a6017cc2a9` |

That is 216 cases, 27,893 expected values, and 724 negative controls, of which 550 discriminate.

## Basis

- R1's return ([REFERENCES/README.md](REFERENCES/README.md)), written product-code-blind from exact-rational closed forms.
- V2's independent refutation, using exact direct-stiffness assembly: [REFERENCE_CHECK/RETURN.md](REFERENCE_CHECK/RETURN.md), REFERENCES CONFIRMED.
- ROOT's rulings: [ROOT_RULINGS_V2.md](ROOT_RULINGS_V2.md). The RF-CANCEL net-governed "recommended" column is the binding scale. F1, F3, F4, F5 and F9 were revised before freezing.
- R1 revision 2, and V2's narrow backcheck of it: [REFERENCE_CHECK/BACKCHECK.md](REFERENCE_CHECK/BACKCHECK.md), CONFIRMED. All 27,893 values agree to within 4.98e-40 of max(|exp|, scale) on the printed inputs, nothing regressed, and the regeneration is byte for byte.

## Rules

- **No further changes.** No slice may change `references.py` or its output. A mismatch goes back to ROOT; it never changes a reference, scale or tolerance.
- The comparison is `|obs − exp| ≤ 1e-9 · max(|exp|, scale)`, with the stated scales. For RF-CANCEL, the binding scale is the recommended (net-governed) column. Expected values below the binary64 range are compared absolutely and reported as such (V2 F8). Comparisons below D1's floor are reported `not_covered`, which never counts as a pass.
- **V2's N1 is a known wording erratum.** README line 9 and the JSON `conventions.classes` sentence name only the class scale. The binding RF-CANCEL scale rule is the one in `ROOT_RULINGS_V2.md` and the 41 `recommended_scale` texts. The frozen files are not reopened for this; see [REFERENCE_CHECK/ERRATUM_N1.md](REFERENCE_CHECK/ERRATUM_N1.md).
- **P1.** P1's provisional results, taken at references `3592032fa` / references.py `7fe459b1…` (revision 1, the same values and exact inputs from `references.py`), are re-evaluated against these frozen references before any finding is final.
