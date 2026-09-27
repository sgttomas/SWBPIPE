# Erratum N1 to the frozen T3 references

Recorded by the T3 manager on ROOT's direction ([ROOT_SELECTION_REFERENCES.md](../ROOT_SELECTION_REFERENCES.md)). This note sits beside the frozen files; they are not reopened.

Two general sentences in the frozen references name only the class scale as the comparison scale:

- `REFERENCES/README.md` line 9: "The `scale` is the published class scale of each value".
- `REFERENCES/references.json`, the `conventions.classes` sentence.

**The binding rule for RF-CANCEL is different.** Its comparison scale is the recommended (net-governed) column, as stated in the 41 RF-CANCEL `recommended_scale` texts and in the `criterion` header, and as ruled in `ROOT_RULINGS_V2.md` §1. That column never exceeds the class scale. Where it falls below |exp| (84 rows, all `mixed`), the comparison is exactly relative. For every other family, the class scale is the comparison scale, as the two sentences say.

Source: V2's backcheck, [BACKCHECK.md](BACKCHECK.md), note N1.
