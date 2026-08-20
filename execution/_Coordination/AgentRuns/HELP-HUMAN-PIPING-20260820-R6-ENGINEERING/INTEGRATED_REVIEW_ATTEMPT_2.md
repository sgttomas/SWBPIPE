# Integrated review attempt 2

Frozen range: `357a58b56726feba49507534159c3fbc4656b818..37c3b9bf1cb2ff3ab4d1bc761a305da00f8faad4`
Verdict: `FAIL`

The fresh reviewer inspected all 22 changed files (3,415 insertions, 32
deletions), with ancestry, scope, and whitespace checks passing and no N2/N3
finding. It found that `_verify_manifest_schema` accepted marker-preserving
weakened lookalike schemas, allowing canonical checksum or professional-boundary
rules to be removed while verification still passed.

Disposition: closed by N1 Amendment 2 and proof-loop commit
`d699413d122df744b7801bdccb6f8d0058cc5280`. The final implementation binds
one exact serialization to the canonical SHA-256, creates one plain JSON
snapshot from those bytes, and uses the same snapshot for identity and all
evaluation. Weakened-schema and hostile custom-mapping regressions pass. The
N1 suite passed 112 tests and fresh V18 review passed with zero findings over
nine paths and 4,038 lines. Replacement integrated review is required.
