# V58 — filter presentation diagnosis

The recorded ~535/537 ms intervals are explained by an overly restrictive DOM-lifecycle recognizer selecting a late eligible frame. They do not establish a 500 ms product filter computation or prove that the 500 ms driver wait caused the presentation. A prospective verifier-only correction is justified; product, marker timing, driver waits, workload, targets and clock accounting should remain unchanged.

## Bound evidence

`INPUTS.json` binds the exact two small raw traces, stopped records, canonical derived/results, inspected method/product source and pinned Chromium source. Five method files match V20T_R2 manifest `0dfc5eb61d22d9bac9575278f2fcc5af2211a92fffaf4ba08ef6d577ff10d418`. Primary Chromium revision is `507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c`. This is read-only source/raw analysis with independent Python integer-preserving JSON and lineage calculations, not an executed test or fresh method qualification.

| Observation | Filter sample 1 | Focused empty sample 0 |
|---|---:|---:|
| Product publication minus page action-listener time | about 2.2 ms | about 1.4 ms |
| Final epoch-2 feedback minus trace action marker | 6.922 ms | 4.060 ms |
| Canonically selected proxy minus trace action marker | 512.310 ms | 511.199 ms |
| Canonically selected presentation minus trace action marker | 535.278 ms | 537.432 ms |
| Preserved canonical interval | [535.0779985427845, 535.4560023307812] ms | [537.271998310087, 537.6320019722006] ms |

ROOT `ModelTree.tsx:50–116,145–150` computes filtered state directly and publishes from its effect. The raw records show two early DOM epochs, exact final query/count/rows and stopped reconciliation; no late epoch reset explains the result. The mutation journal records counts rather than targets, so the individual transient DOM edits are unknown. Final-epoch selection must stay intact.

## Exact structural cause and earlier lineage

The extractor `domLifecycleProxies` at lines 234–254 requires exactly one complete-X style, PrePaint, paint, Layerize and update. It therefore excludes:

- Empty sample proxy **1528**, main frame **3776186964249928200**, starting +4.091 ms after action and after final feedback. It has two sequential complete style/PrePaint cycles, followed by one paint, Layerize and update. Its source-backed causal chain reaches one clean ALL reporter, origin/current BeginFrame **1032**, surface **5375170299773704241**, reporter indexes **4259/4375**, submit-child **4285/4374**, endpoint **2275670490898** (+20.784 ms trace delta).
- Sample 1 proxy **1450**, main frame **3776186964249927813**, starting +7.911 ms after action and after final feedback. Its one Layerize at index **1463**, timestamp **2275669214101**, is `ph:I,s:t` with no `dur`, enclosed within the complete paint at index **1461**. Other lifecycle stages are complete and ordered. Its chain reaches one clean ALL reporter, origin/current **881**, surface **5375170299773704361**, reporter **4046/4125**, child **4073/4124**, endpoint **2275669232577** (+26.963 ms trace delta).
- Empty sample's next proxy **1776** similarly contains instant Layerize **1789** and is excluded. It has an ALL reporter endpoint +29.117 ms. It must not replace the earlier qualifying proxy 1528 merely because another endpoint is convenient.

These are diagnostic trace facts, not alternative accepted V57 scores. Sample 1 proxy **1214** really paints before the final epoch-2 marker: it remains ineligible regardless of multiple-pass support. No earlier epoch or nearest-time pairing is permitted.

Pinned `local_frame_view.cc:2474–2671` explicitly repeats style/layout and prepaint for observer and related lifecycle updates, then calls the final paint once. Multiple sequential passes are consequently supported behavior, not intrinsically duplicate evidence. `RunPaintLifecyclePhase:2958–2983` calls `PaintTree` then `PushPaintArtifactToCompositor` synchronously. The frame-bound Layerize trace is inside Push at **3291**, before its possible fast-path return. `inspector_trace_events.cc:1147–1167,1511–1528` gives Layerize, PrePaint and TimeStamp the same native FrameId namespace.

The instant event does **not** prove zero elapsed time, a particular exporter quantization branch, or that the fast path ran. None is needed for the narrow correction below: the complete enclosing paint bounds synchronous Push completion. No global missing-duration normalization is justified.

## Permitted prospective correction

Change only `apps/desktop/e2e/ui-foundation/causal-presentation-extractor.mjs` and its maintained control file `verify-causal-presentation-extractor.mjs`, plus run-owned declarations/source bindings. Preserve the public API; no declaration-file change is needed unless the writer intentionally exposes additional diagnostics.

1. Recognize ordered sequential style/PrePaint passes within one complete proxy instead of demanding cardinality one. Require a unique terminal eligible style/PrePaint chain after the exact final feedback, preceding the unique complete final paint and update. Account for prior passes explicitly; reject overlapping, duplicated or ambiguous terminal evidence. Do not merely choose a convenient last timestamp or ignore duplicate records. A bounded implementation supporting the observed ordered cycles is sufficient; unrecognized shapes remain unavailable.
2. Allow a unique exact-frame/thread/category Layerize checkpoint either as the existing complete X span or the observed thread-scoped `I` event with absent duration. Require it inside the unique **complete-X** `RunPaintLifecyclePhase`, itself inside the complete proxy, with completed paint preceding the exact DoUpdateLayers. For I, record/use checkpoint semantics; do not manufacture `dur:0`, call it a complete slice, normalize arbitrary instants, or infer omitted duration for parents. Complete parent closure is the completion proof.
3. Keep the earliest eligible lifecycle, all stopped-content/model/document/final-epoch guards, same PID/TID/frame identity, exact M→commit→activation→current draw/surface/origin reporter namespace and fully paired presentation tail, first-tail-failure behavior, lossless identifiers, clocks and conservative upper bounds. Never repair a failed first eligible causal tail by falling through to a later presentation.

## Focused acceptance and controls

Retain every existing negative, especially duplicated PrePaint, wrong document, content after the only style pass, missing Layerize/commit, stale final epoch and first eligible frame with missing origin followed by a valid later frame. Add independent positives for two sequential style/PrePaint passes ending in one paint and for frame-bound instant Layerize inside complete paint; both must use the same exact existing tail rules. Add negative cases for duplicate terminal passes or mixed I+X duplicate Layerize, overlap/reversed ordering, wrong frame/PID/TID or instant scope/category, absent or malformed parent duration, Layerize outside paint, missing terminal PrePaint, a final marker after the terminal style, and missing/ambiguous/partial/aborted earlier tail with a later valid candidate.

After a bounded independent source backcheck and controls, run a separately labelled archived regression: sample 1 should identify proxy1450/endpoint2275669232577; empty0 should identify proxy1528/endpoint2275670490898. Other action classes and unrelated derived evidence must remain byte/semantically equivalent as appropriate. Archived results are diagnostic-only and contribute zero cohort samples. Then rerun fresh focused qualification under the amended frozen method before full-cohort admission. The unchanged p95<=200 ms target still requires the real full population; two focused actions cannot establish it.

## Limits and handoff

At about +506 ms, both traces show a SetNeedsCommit and then the selected lifecycle. Stopped reconciliation is about +506.8 to +507.3 ms in page time, and the controller subsequently requests a screenshot. No captured stack or named caret event proves the late commit's trigger. The delay's similarity to the settle wait is not a causal identification. There is no evidence here justifying product optimization, changing caret behavior, shortening waits, adding RAFs or subtracting observer cost.

V57 remains instrument/identity/foreground/restoration PASS with its original ~535/537 ms intervals and zero full-cohort contribution. This packet is a derivative diagnosis, not replacement measurement authority. Product/native/resource evidence is unaffected by the proposed two verifier files. Remaining work is prospective implementation, independent review, focused controls/regression and fresh qualification. Full cohort remains held by ROOT. Hardware scan-out, exact uninstrumented latency and true observer cost are not newly established.
