# U7 R3 Residual Closure Map

Basis: same-RU successor backcheck review SHA-256 `54dd95c5f4410974060430221c16bdc3bd83bbb39964ea0622fc64fdd03f8f18`; R3 hashes are frozen in `R3_SUCCESSOR_MANIFEST_V3.json`. Closure remains pending same-RU backcheck.

| Finding | R3 closure | Exact repository evidence |
| --- | --- | --- |
| `RU-BC-F1` (`RU-F2`) | The binder now distinguishes the engine-created initial hash (`payload_ref=model:local_batch_input`) from the exact echoed frozen basis hash. It also validates the producer's `0.1.0` schema, deliverables, complete audit/professional boundaries, and complete info diagnostics before any publication. | `routeDraft.ts:235-404`; producer-derived mutation tests in `routeDraft.test.ts:268-311`; App mutations at `App.test.tsx:15923-16003` independently reject malformed info and both payload refs with no model, receipt, retained context, or checkpoint. |
| `RU-BC-F2` (`RU-F3`) | Continuation is authorized only by the accepted direct-Apply review ID carried as the model commit token. Content/ID/hash coincidence is insufficient; all other commits, including open and undo/redo, clear continuation. | `App.tsx:558,1109,1154,1307,1330,1423,1479`; `PipeViewport.tsx:302-315,635`; own-commit/undo/redo test at `App.test.tsx:15854-15883`; delayed coincident external-open test at `App.test.tsx:16007-16043`. |
| `RU-F1` | Carried closed: busy controls and App-owned invalidation still prevent stale publication. | `PipeViewport.tsx` flight-control disablement; delayed valid-response invalidation test at `App.test.tsx:15886-15917`. |
| `RU-F4` | Carried closed: only complete runtime target discriminators reserve IDs and intra-batch node/pipe identity collisions are rejected. | R2 reservation/builder code and `routeDraft.test.ts` regressions remain present and green. |
| `RU-F5` | Carried closed: authoring provenance starts blank and pointer placement cannot synthesize it. | R2 blank-provenance code and App/route regressions remain present and green. |

Exact final-cut checks: route **17/17**, App **162/162**, desktop `tsc -b && vite build` **PASS** (1,698 modules), Chromium desktop **1/1 PASS** at 1024×768. Packaged native evidence remains a separate root gate.
