**PASS for the page-close repair. Original finding 1 is fully closed; no additional actionable finding in this three-file diff.**

Reviewed `1658a2511e4e7527997bd3c0b0d3047f7287e3a4` against `99c0bf0e404982c7de358af56d8292317af730ad`. The clean wt2 checkout matches the candidate, and desktop bytes match manager checkpoint `74e69a269`.

The repair restores focus after the closing page has unmounted and the stage’s inert state has cleared. It retains an existing deliberate focus destination, otherwise restores a usable opener or falls back to the current stage/Select control. Disconnected, hidden, inert, and disabled destinations are rejected. Page-to-page navigation preserves the original return target. Canvas, inspector, and draft mounting remain unchanged.

The new regressions use real Tab/Enter to activate Close, with **no subsequent manual focus call**. Six retained witnesses establish:

- Libraries returns to its rail opener.
- Project and Analyze return to the Model-stage fallback after their menu command unmounts.
- Every destination is visible, non-inert, unobscured, and a nonignored accessibility button.
- Canvas, inspector, draft identity, and entered text remain intact.

The supplied final log reports **18/18 passing**, including the previous 12 accessibility cases. Both supplied hashes match:

```text
close-repaired.log
3452bf38b5194cdf4da40127c4f83dd2432a61d92f7206d1c741e0840e7deccf
RETURN_CLOSE_FOLLOWUP.md
fc5f1a76f99ef9aa197c466a7e74958fd1c892e9732b205806a02e7a5b15a71d
```

All four tested-product/spec hashes and all 32 follow-up evidence entries match. Manifest hashes:

```text
close-candidate-sha256.txt
db373ec945ae56276e1cf3949ca7a722c2b6603f672361db5d83ae4eccd1e9ec
CLOSE_SHA256SUMS.txt
4d99354720bcdb9bbf14d62170fa4a51d0057c9a91551684c705d50
```

Correction to the abbreviated second manifest hash above: its full SHA-256 is `4d99354720bcdb9bbf14d62170fa4a51d0057c9a9155166ab93517684c705d50`.

`git diff --check` passes. I performed no writes, additional test execution, UI activity, or delegation. The prior context manifest and Astra/xhigh allocation remain applicable.

**This is not final native or merge suitability.** The separately reproduced native failures remain unresolved: text Undo does not undo text, and the first Escape from a real select popup also closes the inspector. Their repairs require independent review and rebuilt native backchecks. Final full lanes, native witness, fidelity review, ROOT inspection, clean-candidate sweep, and actual-candidate CI remain owed.

Standard F-PIP-2 / DEC-081 claim fence applies.
