"""I98 B2-W: the native Run each successor or precommit dump records (read-only; VENV).

Usage: python runs.py <file>...   (a successor document, or a precommit dump {"source", "invocation"})
Per file: the case status, the Run's kernel terminal, its attempts (precision, outcome,
verification phase and precision), the work charged (invocation_after), and the product
attempt's stages.
"""
import json
import os
import sys


def main():
    for path in sys.argv[1:]:
        with open(path, "rb") as f:
            doc = json.loads(f.read())
        source = doc.get("source", doc)
        body = source["retained_precision"]["body"]
        for c in body["cases"]:
            run = c.get("run") or {}
            attempts = [(a.get("precision"), (a.get("outcome") or {}).get("kind"), (a.get("verification") or {}).get("phase"),
                         (a.get("verification") or {}).get("precision")) for a in run.get("attempts", [])]
            stages = []
            if c.get("product_attempt_ref") is not None:
                pa = body["product_attempts"][int(c["product_attempt_ref"])]
                st = pa.get("stages") or {}
                stages = sorted({v for v in st.values()}) if isinstance(st, dict) else st
                stages = {"stages": len(st), "values": stages, "result": pa.get("result") if not isinstance(pa.get("result"), dict) else pa["result"].get("kind", pa["result"])}
            print(f"RUN {os.path.basename(path)} case={c['basis_ref']['ref_id']} status={c.get('status')} terminal={run.get('kernel_terminal')} "
                  f"attempts={attempts} invocation_after={run.get('invocation_after')} product_stages={stages}")


if __name__ == "__main__":
    main()
