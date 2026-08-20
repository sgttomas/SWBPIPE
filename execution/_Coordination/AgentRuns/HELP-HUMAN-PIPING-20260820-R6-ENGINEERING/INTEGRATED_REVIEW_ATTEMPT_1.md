# Integrated review attempt 1

Frozen range: `357a58b56726feba49507534159c3fbc4656b818..a6a1efb5bca14d9882def42d4c502939c3f4b1b4`
Verdict: `FAIL`

The fresh reviewer covered all 21 committed paths (2,871 insertions, 24
deletions), with scope validation PASS and no N2/N3 findings. It found one N1
blocking defect: the public adapter/plugin composition path emitted a top-level
result envelope with hard-coded public-reviewed privacy and accepted invented
fixture provenance, even when manifest, quantity, or adapter evidence required
private/protected/review-required handling.

Disposition: closed by N1 Amendment 1 and proof-loop commit
`37c3b9bf1cb2ff3ab4d1bc761a305da00f8faad4`. The amended N1 suite passed 107
tests; canonical boundary-envelope schema assertions, containment, and diff
checks passed; fresh N1 V16 review passed with zero findings over nine paths and
3,930 frozen lines. A replacement integrated review is required over the new
immutable HEAD.
