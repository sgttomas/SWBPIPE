# B3 preserved Support family — short fidelity backcheck

**PASS for the assigned actual-UI case on `bf15aed87571df3601b99c350fc5296fb337cd91`. No actionable finding in this bounded check.**

Independent TASK Type 2, parent ROOT; configured Astra/high per sealed assignment, with no separate model-serving telemetry. Original role/authority/frame basis retained. Verified candidate HEAD, brief SHA-256 `8dbc7bfc81caa3165ebff9de5e90f15771ace3eafadf582f3a33923522997ccc`, input-manifest SHA-256 `571bb4bc3ddecf348f71e1200fc58789dfc4541d25fe65250f71548b0d955d24`, and all 21 dist-file hashes (`verification.json`).

Used File → New Blank Project, opened Inspector → New support configuration, then performed exactly these real UI actions:

| Action | Expected | Observed/evidence |
|---|---|---|
| Open Support family, then Tab without option navigation | Preserve missing family; close popup | `Not provided (preserved)`, `data-value=""`, expanded false; focus moved to Restrain UX. `02-passive-tab.json/png/txt`. |
| Open Support family, then click outside Queue support creation without choosing an option | Preserve missing family; invalid input stays blocked | Same label and empty data-value; popup closed. Queue produced `Support ID is required.` The model tree remained 2 of 2 entities (blank project and diagnostic), with no added support. `03-passive-outside-queue.json/png/txt`. The validation message is retained in text/JSON below the screenshot's visible scroll extent. |
| Open Support family and explicitly click Anchor | Deliberate choice changes the draft | Label Anchor, `data-value="anchor"`, popup closed. Undo model edit stayed disabled. No support operation was applied. `04-explicit-anchor.json/png/txt`. |

`01-missing-family.png/txt` records the initial blank-project state. `actions.jsonl` and `probe.mjs` retain the actual actions and bounded runner. Read-only DOM reads captured values/state; no fixture, model, DOM, storage or option-array injection was used. Screenshots were freshly captured and visually inspected.

Environment: headless Chromium 148.0.7778.96, Node v24.18.0, macOS arm64, fresh ephemeral context at 1440×920, Light theme, existing rebuilt dist served on 5183 under the shared run lock. No native/CUA/foreground access, source/test/Git mutation, rebuild, broad matrix or delegation. Only fidelity-value-backcheck evidence was written; original returns/manifests remain unchanged.

This result covers preserved missing Support family and explicit Anchor choice through this browser UI only. Other null/unsupported values, option-list changes/races, event-time callbacks, native behavior and native unit choices remain with their independent code/unit/native witnesses. Earlier F1/geometry/Escape findings remain bound to their previously reviewed candidates; they were not repeated here. No complete compact-control, B3 merge-suitability, usability, performance, Runtime-adoption or product-acceptance claim follows.

Browser/server exited successfully; lock released, ports 5183/5184 clear, candidate and dist unchanged (`cleanup.json`). ROOT was promptly notified that the manager's full-lane slot was available. `SHA256.json` hashes all retained artifacts, including four screenshots and this return.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
