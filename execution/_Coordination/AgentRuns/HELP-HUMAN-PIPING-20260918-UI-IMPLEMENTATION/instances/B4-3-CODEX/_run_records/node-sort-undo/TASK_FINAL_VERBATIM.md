Confirmed: sorted Node review loses browser-native text Undo after its active row moves, despite retaining input identity, focus, and caret. Model/hash/history remained unchanged.

No production repair made. Recommend opting **Node review only** into the existing persistent editor; deferred sorting is a broader alternative.

Evidence: `node-sort-undo/WORKER_RETURN.md`
SHA256: `66f7120fcd40f29f9bcef376a9cd056605f912ac14f1fe5e745ccdc96400c49a`

Browser resources released; diagnostic test remains intentionally failing pending repair disposition.
