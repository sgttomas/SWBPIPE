**PASS — complete 3-file backcheck** of `87faf4b240d1be8b197843cd869b22408e85be3b..840a21647983948c2f9930dd2fc56afb9e5d3eff`. No new actionable findings.

The repair closes all three P2s:

- Editing-boundary Tab uses a persistent destination after successful/no-op Apply; invalid or rejected edits remain available.
- Filtering or deleting remembered focus leaves a live coordinate Tab entry without moving external focus or publishing selection.
- Deferred navigation respects newer selection, including observed A→C→A, through a selection revision guard.

Existing editor ownership, generation reset, file-order Enter, virtualization and original test assertions remain intact. The browser change adds coverage without weakening an oracle.

Verified retained evidence:

- **23 core tests passed**; TypeScript exit 0 is recorded with empty output.
- **4/4 new browser cases passed, 11.4 s**.
- **6/6 legacy browser cases passed, 14.8 s**.
- Both browser records name `840a2164`, exit 0, pinned Chromium 148.0.7778.96 and one worker. Their raw-output hashes match.
- All **20 freeze04 file hashes** match committed bytes; the repair delta contains exactly the three authorized files.

Suitable for source-review fan-in. Native WebKit/text-undo verification, final sweep and combined-candidate gates remain pending. Earlier 130+17 results were not rerun collectively on this revision. I performed no tests, UI, builds or mutations. Changes beyond `840a2164` need separate coverage.

Context continuity: retained independent TASK `/root/b4_1_code_review`, Codex/Astra xhigh, same-model review; prior instruction/context hashes remain as returned. Newly consulted hashes:

| Record | SHA256 |
|---|---|
| ROOT `B4_1_BACKCHECK_840_BRIEF.md` | `aa82e434aea6c0b2735f3cd4425e25637a4b5f7fc2da31b38580664cfdf774d5` |
| wt3 `SOURCE_FREEZE_04.json` | `5d0719aae57de2cc332434d768b4567e86608bc597b792c51f964b9169d8cf7d` |
| wt3 `WORKER_RETURN_FREEZE04.md` | `f3c76f086930d20ed83e7d0c42c2837127cf5b85c98c3179b18598bbd7462e2b` |
| wt3 `SOURCE_BROWSER_RETURN.md` | `7918dee58153668ed7d31643bb1120bc3e20cdec185f48bc83564d8a4e01437c` |
| Core repair log | `dfe58510920c1c266f28647439eea7cf0295b41ae442b195f1b0986676c1970c` |
| Browser `new-02/stdout.txt` | `66d636f00baa884e199dbc439a331933b1c37223ed068734e0ee91165e55ccc0` |
| Browser `legacy-02/stdout.txt` | `2b77fbe3da5f51528b621f1796cecb32d5ca042981c710d94233980536b18c9c` |
