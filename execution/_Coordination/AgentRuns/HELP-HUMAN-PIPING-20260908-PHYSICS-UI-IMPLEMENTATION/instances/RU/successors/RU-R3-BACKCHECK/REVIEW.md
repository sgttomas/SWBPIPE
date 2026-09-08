# RU R3 residual backcheck

**Verdict: PASS.**

The launch brief matches SHA-256 `d0927807457cf59d1c0a417e89c28d0751d691064a0fb77460937f0fe9d0febf`. I reviewed all 478 lines and five paths of the R2-to-R3 delta, traced the affected live source and producer contracts, rehashed all 27 manifest entries, and ran focused final-byte tests. The R3 manifest remains `bd5cdbdb12a71179df68305ef375071743607e91bea0f9d3fac074240247ca30`; the full base diff remains `6e013cf88417d02c1f1f75d48fac124ba635f8e39b974c6d2cda2477b01b6b10`; the R2-to-R3 delta remains `a0ae4a385ed9b76d8fa10974df218525b555732e333d4c8964ac76714b3a21fb`.

## Finding closure

| Finding | Result | Exact behavior |
|---|---|---|
| `RU-BC-F1` / `RU-F2` | **Closed** | `routeDraft.ts:235-409` accepts actual producer outcomes, requires single schema/deliverables/model basis and complete receipt boundaries, requires batch `initial_model_hash.payload_ref` to equal the producer-owned `model:local_batch_input`, and compares `submitted_initial_model_hash` structurally with the frozen basis hash. It validates complete info diagnostics and rejects warning, malformed, blocked, identity, step, diff, acceptance, model, and hash mutations before publication. The two hash references are checked against their distinct origins; they are not incorrectly required to equal each other. |
| `RU-BC-F2` / `RU-F3` | **Closed** | `App.tsx:558-568,1109,1154` assigns the accepted review ID only to the direct Apply commit. `PipeViewport.tsx:304-319,632-658` preserves continuation only when that token matches the pending review. External open/create, undo/redo, other commits, and Cancel cannot create that ownership token; coincident IDs/content/hash are insufficient. Own publication preserves the requested endpoint and fields, while external replacement invalidates the callback and clears continuation. |
| `RU-F1` | **Carried closed** | Busy controls remain disabled and permitted selection/model invalidation still prevents stale Add/Apply callbacks from publishing any model or checkpoint. |
| `RU-F4` | **Carried closed** | Reservation still requires a valid operation kind, supported target object type, and nonempty reference; malformed saved members remain unchanged for authoritative fail-closed validation; intra-route node/pipe ID collision is rejected. |
| `RU-F5` | **Carried closed** | Node, endpoint, pipe, and pointer provenance still start empty, and Add remains unavailable until provenance is explicitly entered. |

The private affirmative tests use actual prebuilt-WASM operation outcomes. They confirm a valid producer batch is accepted, both distinct payload-reference mutations and malformed info are rejected, the matching accepted token preserves continuation, and a coincident external replacement without the token clears it. Repository tests independently confirm zero model, receipt, retained-context, or checkpoint publication for adversarial responses.

Structural equality remains object-key-insensitive and array-order-sensitive. Malformed-member preservation, exact reservation, explicit provenance, undo/redo clearing, and all seven consumer behaviors remain supported by source and focused tests. `SEVEN_ROW_CONSUMER_PROOF_V3.md` is consistent with the reviewed implementation; it remains consumer evidence rather than formal dependency-row acceptance.

The final Chromium run passes at exactly 1024×768 with zero body horizontal overflow, an in-bounds authoring card, persistent canvas larger than 300×250, reachable Add/Apply controls, route creation, undo/redo, section assignment, and save/open. It exercised the existing prebuilt WASM service. Packaged/native GUI behavior remains a separate held gate and is not claimed here.

No production defect or acceptance-relevant evidence gap remains in the sealed R3 scope. The author-reported 162-test full App run and build are exact-cut evidence in the manifest; RU did not repeat them because this brief prohibits the full suite and builds. RU independently reran the affected 38 App cases, route, inspector, private adversaries, and browser journey on the final bytes.
