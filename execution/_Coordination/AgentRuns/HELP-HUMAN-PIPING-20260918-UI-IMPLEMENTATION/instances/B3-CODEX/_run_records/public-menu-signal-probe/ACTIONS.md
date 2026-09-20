# Exact CUA action sequence

Bound app by explicit isolated SWBPIPE.app path; owned PID19891. After each action call, fetched getAXState. No synthetic event scripts or input outside CUA.

1. click(3), Close devtools.
2. click(653), View menu; click(5), Both View (already Both): ordinary application menu positive action control.
3. click(167), File; click(4), New Blank Project. Result project:blank-local-20260920t041132z, blank unchanged model.
4. click(100), Node tool: Inspector exposes New node coordinate unit m. No fields entered, no Apply.
5. click(150), unit popup; pressKey('Escape'): popup closes, Inspector stays open, m focused.
6. pressKey('space'), popup visibly open with m selected; pressKey('Escape'): popup closes, Inspector stays open, m focused.
7. pressKey('space'), popup visibly open m; pressKey('Return'): popup closes, value m, Inspector stays open.
8. click(131), popup; click(2), selected m: closes unchanged.
9. click(131), popup; click(3), mm: closes with mm selected.
10. getScreenshot succeeds; observed blank tree area at [200,400]. pressKey('space'), popup opens with mm selected; click([200,400]): popup closes, mm unchanged, Inspector stays open. DOM target SECTION Modeling workspace; no model operation.

Final AX: blank project ID above, Both, Node armed, Inspector open, unit mm, pending changes0, Undo/Redo disabled, popup/devtools closed. Screenshot is in CUA transcript; no independent screenshot file claimed. All intermediate raw AX observations are in the tool transcript. Runtime log retains ordered passive native and DOM output.
