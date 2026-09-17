# V59-R1 — read-only counterexample (not executed)

Bound maintained verifier: `verify-causal-presentation-extractor.mjs` SHA594f9cd24b5bf520404c00cb807baf6718200446cb0b3e7a363a6b7ec9723e11. Its `dom` fixture is a complete existing positive, including exact final epoch and valid presentation tail. Change only its style and PrePaint population as follows; leave all other events/evidence unchanged:

```javascript
const f = structuredClone(dom);
const style = f.events.find(e => e.name === "LocalFrameView::RunStyleAndLayoutLifecyclePhases");
const prepaint = f.events.find(e => e.name === "PrePaint");
style.ts = prepaint.ts = 1_020_100;
style.dur = prepaint.dur = 0;
f.events.push(structuredClone(style), structuredClone(prepaint));
```

This is an unexecuted reproduction recipe, not test evidence. The resulting local population has marker ts1020000; complete proxy [1020050,1023000]; two identical complete style spans [1020100,1020100]; two identical exact-document-A PrePaint spans [1020100,1020100]; unique complete paint [1020400,1020600]; exact-frame devtools.timeline Layerize X[1020450,1020500]; update [1021000,1021500]. All share the original renderer41/main thread7. Original exact IDs, source, stopped proof, and presentation endpoint1100000 are untouched.

At extractor lines254–266, styles.length===prepaints.length===2; all four zero durations are finite/nonnegative and all stages lie inside the proxy. Both style.end<=pairedPrepaint.start are1020100<=1020100. The interpass comparison is likewise1020100<=1020100. Terminal style is after marker, terminal prepaint ends before paint, and paint/Layerize/update checks pass. The recognizer therefore pushes this proxy and labels the second duplicate terminal. The tail does not inspect styles/PrePaint again, so the existing valid tail cannot reject these duplicates. This violates the required unique, unambiguous terminal evidence; the preimage rejected cardinality2.

The same guard cannot distinguish the tied combined orders Style,PrePaint,Style,PrePaint and PrePaint,Style,PrePaint,Style: it splits kinds into separate arrays and compares timestamps only. Array index tie-breaking in the global sort does not enforce cross-kind order. Equal reported timestamps alone do not establish two distinct ordered lifecycle passes. Reject unresolved duplicate/tied ambiguity; do not invent duration or infer a performance improvement. Add focused negative controls and preserve the established ordered-positive and first-tail-failure controls.
