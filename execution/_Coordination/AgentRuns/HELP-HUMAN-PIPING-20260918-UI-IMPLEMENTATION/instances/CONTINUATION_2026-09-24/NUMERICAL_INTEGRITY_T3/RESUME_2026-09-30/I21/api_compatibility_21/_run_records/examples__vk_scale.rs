1: //! `vk_scale`: one of V-K's scale runs per process (checkpoint B; plan §13;
2: //! brief Scope 12). `runner/vk_scale_runner.py` runs it in release, under K6's
3: //! runner, which provides the `/usr/bin/time` wrapper, the RSS watchdog and
4: //! the admission rule. It is observation only: it prints JSONL on stdout and
5: //! asserts no time or memory bound.
6: //!
7: //! Each phase prints one line:
8: //! - `start`: the case, the model file's sha256 against the committed one,
9: //!   and the heap cap;
10: //! - `counts`: W1's counts in O(nnz) with no solve, and the admission estimate
11: //!   (`scale.rs`: K6b's E_max, ported). **The backstop:** an estimate above
12: //!   half the heap cap is refused (exit 3), independently of the runner;
13: //! - `w1`: the case through `lane::run_case_with`, the same code the CI lane
14: //!   judges with, with `CaseLimit` and `InvocationMeter` at `u64::MAX`;
15: //! - `report`: the report's counts, the failures, the not-covered set against
16: //!   the committed list, and C9's second floor set, from S of the complete
17: //!   solution (`s_full`);
18: //! - `record`: the per-case record (`vk-case-record-v1`);
19: //! - `rcm`: K4's RCM against SD's on the model's free–free adjacency;
20: //! - `binary64`: the binary64 sparse gate's outcome class, sparse only (C4);
21: //! - `summary`: the heap peaks.
22: //!
23: //! Every phase line carries its elapsed time and its heap peaks (the in-place
24: //! and move models of K6's allocator). K6's runner records the process's load.
25: //!
26: //! Usage:
27: //! `vk_scale --case <id> [--model-file <path>] --heap-cap-bytes <n> [--counts-only]`,
28: //! or `vk_scale --noop --heap-cap-bytes <n>` for the no-op baseline run.
29: //!
30: //! Exit codes: 0 completed (an unresolved or refused outcome is an outcome,
31: //! not an error); 2 usage; 3 refused by the binary; 4 a check failed before
32: //! the solve (unknown case, model sha256, K4SRC, source refusal). A heap-cap
33: //! refusal aborts after the allocator's marker.
34: use open_pipe_stress_frame_kernel::structural::retained_api::PrimitiveSource;
35: use piping_numerical_robustness::cases::{load_family, load_large_model, Case, Model};
36: use piping_numerical_robustness::{floor, lane, parity, rcm, scale, sha256};
37: use serde_json::{json, Value};
38: use std::io::Write;
39: use std::time::Instant;
40: 
41: mod alloc {
42:     //! K6's counting, capped global allocator (`H/src/bin/k6_observe/alloc.rs`),
43:     //! as V-K uses it: `CURRENT` is the bytes requested and alive now. `PEAK`
44:     //! counts a growing `realloc` as its difference (the in-place model), and
45:     //! `PEAK_MOVE` counts the old and the new block together (the move model).
46:     //! The stage peaks restart at `stage_reset`. A request that would take
47:     //! `CURRENT` above `CAP` is refused: a fixed marker goes to fd 2 without
48:     //! allocating, and a null return makes Rust abort (SIGABRT).
49:     use std::alloc::{GlobalAlloc, Layout, System};
50:     use std::sync::atomic::{AtomicUsize, Ordering};
51: 
52:     static CURRENT: AtomicUsize = AtomicUsize::new(0);
53:     static PEAK: AtomicUsize = AtomicUsize::new(0);
54:     static PEAK_MOVE: AtomicUsize = AtomicUsize::new(0);
55:     static STAGE_PEAK: AtomicUsize = AtomicUsize::new(0);
56:     static STAGE_PEAK_MOVE: AtomicUsize = AtomicUsize::new(0);
57:     static CAP: AtomicUsize = AtomicUsize::new(usize::MAX);
58: 
59:     /// The marker `vk_scale_runner.py` recognizes as a heap-cap abort.
60:     pub const MARKER: &[u8] = b"vk_scale: heap cap refused ";
61: 
62:     pub struct VkAlloc;
63: 
64:     pub fn set_cap(bytes: usize) {
65:         CAP.store(bytes, Ordering::SeqCst);
66:     }
67:     pub fn cap() -> usize {
68:         CAP.load(Ordering::SeqCst)
69:     }
70:     pub fn current() -> usize {
71:         CURRENT.load(Ordering::SeqCst)
72:     }
73:     pub fn peak() -> usize {
74:         PEAK.load(Ordering::SeqCst)
75:     }
76:     pub fn peak_move() -> usize {
77:         PEAK_MOVE.load(Ordering::SeqCst)
78:     }
79:     pub fn stage_peak() -> usize {
80:         STAGE_PEAK.load(Ordering::SeqCst)
81:     }
82:     pub fn stage_peak_move() -> usize {
83:         STAGE_PEAK_MOVE.load(Ordering::SeqCst)
84:     }
85:     pub fn stage_reset() {
86:         let now = current();
87:         STAGE_PEAK.store(now, Ordering::SeqCst);
88:         STAGE_PEAK_MOVE.store(now, Ordering::SeqCst);
89:     }
90: 
91:     fn note_peak(level: usize) {
92:         PEAK.fetch_max(level, Ordering::SeqCst);
93:         STAGE_PEAK.fetch_max(level, Ordering::SeqCst);
94:         note_move(level);
95:     }
96: 
97:     fn note_move(level: usize) {
98:         PEAK_MOVE.fetch_max(level, Ordering::SeqCst);
99:         STAGE_PEAK_MOVE.fetch_max(level, Ordering::SeqCst);
100:     }
101: 
102:     fn reserve(bytes: usize) -> Option<usize> {
103:         let cap = cap();
104:         let mut now = CURRENT.load(Ordering::SeqCst);
105:         loop {
106:             let next = now.checked_add(bytes)?;
107:             if next > cap {
108:                 return None;
109:             }
110:             match CURRENT.compare_exchange_weak(now, next, Ordering::SeqCst, Ordering::SeqCst) {
111:                 Ok(_) => return Some(next),
112:                 Err(seen) => now = seen,
113:             }
114:         }
115:     }
116: 
117:     fn release(bytes: usize) {
118:         CURRENT.fetch_sub(bytes, Ordering::SeqCst);
119:     }
120: 
121:     fn put_usize(buf: &mut [u8], mut at: usize, value: usize) -> usize {
122:         let mut digits = [0u8; 20];
123:         let (mut n, mut len) = (value, 0);
124:         loop {
125:             digits[len] = b'0' + (n % 10) as u8;
126:             len += 1;
127:             n /= 10;
128:             if n == 0 {
129:                 break;
130:             }
131:         }
132:         for k in (0..len).rev() {
133:             if at < buf.len() {
134:                 buf[at] = digits[k];
135:                 at += 1;
136:             }
137:         }
138:         at
139:     }
140: 
141:     fn put_bytes(buf: &mut [u8], mut at: usize, bytes: &[u8]) -> usize {
142:         for &b in bytes {
143:             if at < buf.len() {
144:                 buf[at] = b;
145:                 at += 1;
146:             }
147:         }
148:         at
149:     }
150: 
151:     /// `vk_scale: heap cap refused <size> bytes (current <c>, cap <cap>)` to
152:     /// fd 2 from a stack buffer: no allocation.
153:     fn refuse(size: usize) {
154:         let mut buf = [0u8; 160];
155:         let mut at = put_bytes(&mut buf, 0, MARKER);
156:         at = put_usize(&mut buf, at, size);
157:         at = put_bytes(&mut buf, at, b" bytes (current ");
158:         at = put_usize(&mut buf, at, current());
159:         at = put_bytes(&mut buf, at, b", cap ");
160:         at = put_usize(&mut buf, at, cap());
161:         at = put_bytes(&mut buf, at, b")\n");
162:         write_stderr(&buf[..at]);
163:     }
164: 
165:     #[cfg(unix)]
166:     fn write_stderr(bytes: &[u8]) {
167:         use std::io::Write;
168:         use std::os::unix::io::FromRawFd;
169:         // SAFETY: fd 2 is the process's stderr; the `File` is forgotten, never
170:         // closed, and writing a byte slice does not allocate.
171:         let mut file = unsafe { std::fs::File::from_raw_fd(2) };
172:         let _ = file.write_all(bytes);
173:         std::mem::forget(file);
174:     }
175: 
176:     #[cfg(not(unix))]
177:     fn write_stderr(_bytes: &[u8]) {}
178: 
179:     unsafe impl GlobalAlloc for VkAlloc {
180:         unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
181:             let size = layout.size();
182:             let Some(level) = reserve(size) else {
183:                 refuse(size);
184:                 return std::ptr::null_mut();
185:             };
186:             let ptr = System.alloc(layout);
187:             if ptr.is_null() {
188:                 release(size);
189:             } else {
190:                 note_peak(level);
191:             }
192:             ptr
193:         }
194: 
195:         unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
196:             let size = layout.size();
197:             let Some(level) = reserve(size) else {
198:                 refuse(size);
199:                 return std::ptr::null_mut();
200:             };
201:             let ptr = System.alloc_zeroed(layout);
202:             if ptr.is_null() {
203:                 release(size);
204:             } else {
205:                 note_peak(level);
206:             }
207:             ptr
208:         }
209: 
210:         unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
211:             System.dealloc(ptr, layout);
212:             release(layout.size());
213:         }
214: 
215:         unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
216:             let old = layout.size();
217:             if new_size > old {
218:                 let Some(level) = reserve(new_size - old) else {
219:                     refuse(new_size - old);
220:                     return std::ptr::null_mut();
221:                 };
222:                 let moved = System.realloc(ptr, layout, new_size);
223:                 if moved.is_null() {
224:                     release(new_size - old);
225:                 } else {
226:                     note_peak(level);
227:                     note_move(level + old);
228:                 }
229:                 moved
230:             } else {
231:                 let moved = System.realloc(ptr, layout, new_size);
232:                 if !moved.is_null() {
233:                     release(old - new_size);
234:                 }
235:                 moved
236:             }
237:         }
238:     }
239: }
240: 
241: #[global_allocator]
242: static ALLOCATOR: alloc::VkAlloc = alloc::VkAlloc;
243: 
244: const SCHEMA: &str = "vk-scale-v1";
245: 
246: fn emit(mut line: Value) {
247:     line["schema"] = json!(SCHEMA);
248:     let mut out = std::io::stdout().lock();
249:     let _ = writeln!(out, "{line}");
250:     let _ = out.flush();
251: }
252: 
253: /// A phase's elapsed time and heap peaks, then a fresh stage.
254: struct Phase(Instant);
255: 
256: impl Phase {
257:     fn start() -> Self {
258:         alloc::stage_reset();
259:         Phase(Instant::now())
260:     }
261:     fn fields(&self) -> Value {
262:         json!({
263:             "elapsed_ns": self.0.elapsed().as_nanos() as u64,
264:             "heap_peak": alloc::stage_peak(),
265:             "heap_peak_move": alloc::stage_peak_move(),
266:             "heap_current": alloc::current(),
267:         })
268:     }
269: }
270: 
271: fn merge(mut a: Value, b: Value) -> Value {
272:     if let (Some(a), Value::Object(b)) = (a.as_object_mut(), b) {
273:         a.extend(b);
274:     }
275:     a
276: }
277: 
278: fn fail(code: i32, kind: &str, why: String) -> ! {
279:     emit(json!({ "kind": kind, "reason": why }));
280:     std::process::exit(code);
281: }
282: 
283: struct Args {
284:     case: Option<String>,
285:     model_file: Option<String>,
286:     heap_cap: Option<usize>,
287:     counts_only: bool,
288:     noop: bool,
289: }
290: 
291: fn args() -> Args {
292:     let mut a = Args {
293:         case: None,
294:         model_file: None,
295:         heap_cap: None,
296:         counts_only: false,
297:         noop: false,
298:     };
299:     let mut it = std::env::args().skip(1);
300:     while let Some(arg) = it.next() {
301:         match arg.as_str() {
302:             "--case" => a.case = it.next(),
303:             "--model-file" => a.model_file = it.next(),
304:             "--heap-cap-bytes" => a.heap_cap = it.next().and_then(|v| v.parse().ok()),
305:             "--counts-only" => a.counts_only = true,
306:             "--noop" => a.noop = true,
307:             other => fail(2, "usage", format!("unknown argument {other:?}")),
308:         }
309:     }
310:     a
311: }
312: 
313: /// S(kind) of the complete reference solution, in `floor::maxima`'s order.
314: fn full_maxima(case: &Case) -> Option<[f64; 4]> {
315:     let s = case.s_full.as_ref()?;
316:     let mut out = [0.0f64; 4];
317:     for (k, kind) in ["translation", "rotation", "force", "moment"]
318:         .iter()
319:         .enumerate()
320:     {
321:         let v: f64 = s.get(*kind)?.parse().ok()?;
322:         out[k] = v.abs();
323:     }
324:     Some(out)
325: }
326: 
327: fn main() {
328:     let a = args();
329:     let Some(cap) = a.heap_cap else {
330:         fail(2, "usage", "--heap-cap-bytes is required".into());
331:     };
332:     alloc::set_cap(cap);
333:     if a.noop {
334:         emit(json!({ "kind": "start", "noop": true, "heap_cap_bytes": cap }));
335:         emit(json!({
336:             "kind": "summary", "repeats_heap_peak": alloc::peak(),
337:             "repeats_heap_peak_move": alloc::peak_move(),
338:         }));
339:         return;
340:     }
341:     let Some(id) = a.case.clone() else {
342:         fail(2, "usage", "--case is required".into());
343:     };
344: 
345:     // ------------------------------------------------------------ load
346:     let phase = Phase::start();
347:     let Some(case) = load_family("RF-LARGE").into_iter().find(|c| c.id == id) else {
348:         fail(4, "error", format!("unknown case {id}"));
349:     };
350:     let (model, model_sha256): (Model, Option<String>) = match (&case.model, &a.model_file) {
351:         (Some(m), None) => (m.clone(), None),
352:         (None, Some(path)) => {
353:             let (m, sha) = load_large_model(std::path::Path::new(path));
354:             (m, Some(sha))
355:         }
356:         _ => fail(
357:             2,
358:             "usage",
359:             format!("{id}: --model-file is needed exactly when the case has no committed model"),
360:         ),
361:     };
362:     if let (Some(got), Some(want)) = (&model_sha256, &case.model_sha256) {
363:         if got != want {
364:             fail(
365:                 4,
366:                 "error",
367:                 format!("{id}: model file sha256 {got}, committed {want}"),
368:             );
369:         }
370:     }
371:     emit(merge(
372:         json!({
373:             "kind": "start", "case": id, "members": model.members.len(),
374:             "heap_cap_bytes": cap, "model_sha256": model_sha256,
375:             "committed_model_sha256": case.model_sha256,
376:             "case_limit": u64::MAX, "invocation_limit": u64::MAX,
377:         }),
378:         phase.fields(),
379:     ));
380: 
381:     // ------------------------------------------------------------ counts
382:     let phase = Phase::start();
383:     let source = match PrimitiveSource::new(model.source_parts()) {
384:         Ok(s) => s,
385:         Err(e) => fail(4, "error", format!("{id}: source refused: {e:?}")),
386:     };
387:     let k4src = sha256::sha256_hex(&source.encoding());
388:     if k4src != case.k4src_sha256 {
389:         fail(
390:             4,
391:             "error",
392:             format!(
393:                 "{id}: K4SRC sha256 {k4src}, the generator's {}",
394:                 case.k4src_sha256
395:             ),
396:         );
397:     }
398:     let counts = scale::counts(&model, &source);
399:     drop(source);
400:     let sizes = scale::Sizes::of_this_build();
401:     let est = scale::estimate(&counts, &sizes);
402:     emit(merge(
403:         json!({
404:             "kind": "counts", "case": id, "k4src_sha256": k4src,
405:             "nodes": counts.nodes, "members": counts.members, "springs": counts.springs,
406:             "stations": counts.stations, "constraints": counts.constraints,
407:             "loads": counts.loads, "dofs": counts.dofs, "free_dofs": counts.free_dofs,
408:             "bodies": counts.bodies, "pattern_entries": counts.pattern_entries,
409:             "profile_entries": counts.profile_entries, "half_bandwidth": counts.half_bandwidth,
410:             "blocks": counts.blocks, "rows": counts.rows,
411:             "source_encoding_len": counts.source_encoding_len,
412:             "estimate_max_bytes": est.max as u64, "estimate_sel128_bytes": est.sel128 as u64,
413:             "estimate_fixed_bytes": est.fixed as u64, "estimate_model_bytes": est.model as u64,
414:             "estimate_decide_bytes": est.decide as u64,
415:         }),
416:         phase.fields(),
417:     ));
418:     if a.counts_only {
419:         emit(json!({
420:             "kind": "summary", "case": id, "counts_only": true,
421:             "repeats_heap_peak": alloc::peak(), "repeats_heap_peak_move": alloc::peak_move(),
422:         }));
423:         return;
424:     }
425:     if est.max > (cap / 2) as u128 {
426:         emit(json!({
427:             "kind": "refusal", "case": id, "reason": "estimate_exceeds_half_cap",
428:             "estimate_max_bytes": est.max as u64, "half_cap_bytes": cap / 2,
429:         }));
430:         std::process::exit(3);
431:     }
432: 
433:     // ------------------------------------------------------------ W1
434:     let phase = Phase::start();
435:     let mut run = lane::run_case_with(&case, &model);
436:     let w1 = phase.fields();
437:     let record = run.record.clone();
438:     let storage = &record["attempts"][0]["storage"];
439:     let counts_match_storage = storage["pattern_entries"].as_u64()
440:         == Some(counts.pattern_entries as u64)
441:         && storage["profile_entries"].as_u64() == Some(counts.profile_entries as u64);
442:     emit(merge(
443:         json!({
444:             "kind": "w1", "case": id, "outcome": record["outcome"],
445:             "selected_precision": record["selected_precision"],
446:             "verification_precision": record["verification_precision"],
447:             "attempts": record["attempts"].as_array().map(Vec::len),
448:             "corrections": record["corrections"],
449:             "invocation_charged": record["invocation_charged"],
450:             "published_rows": record["published_rows"],
451:             "counts_match_storage": counts_match_storage,
452:         }),
453:         w1,
454:     ));
455: 
456:     // ------------------------------------------------------------ report
457:     let phase = Phase::start();
458:     let not_covered_full = full_maxima(&case).map(|m| {
459:         let scales = floor::scales_from(m, &model);
460:         floor::not_covered(&case, &model, &scales)
461:     });
462:     let t = &run.tally;
463:     emit(merge(
464:         json!({
465:             "kind": "report", "case": id, "rows": t.rows, "pass": t.pass,
466:             "pass_absolute_range": t.pass_absolute_range, "not_covered": t.not_covered,
467:             "structural_zero": t.structural_zero, "expected_unresolved": t.expected_unresolved,
468:             "fail": t.fail, "accounted": t.accounted(),
469:             "failures": run.failures.len(),
470:             "failures_head": run.failures.iter().take(50).collect::<Vec<_>>(),
471:             "not_covered_rows": run.not_covered,
472:             "not_covered_committed": case.not_covered,
473:             "not_covered_equal": run.not_covered == case.not_covered,
474:             "not_covered_rows_s_full": not_covered_full,
475:             "class_mismatches": run.class_mismatches,
476:             "controls_discriminated": run.controls.discriminated,
477:             "controls_non_discriminating": run.controls.non_discriminating,
478:             "controls_undiscriminated": run.controls.undiscriminated,
479:             "controls_unexpectedly_failing": run.controls.unexpectedly_failing,
480:         }),
481:         phase.fields(),
482:     ));
483:     emit(json!({ "kind": "record", "case": id, "record": record }));
484:     run.published.clear();
485:     drop(run);
486: 
487:     // ------------------------------------------------------------ RCM
488:     let phase = Phase::start();
489:     let adjacency = rcm::free_adjacency(&model);
490:     let (k4, sd) = rcm::both_orders(&adjacency);
491:     let equal = k4 == sd;
492:     drop((adjacency, k4, sd));
493:     emit(merge(
494:         json!({ "kind": "rcm", "case": id, "equal": equal }),
495:         phase.fields(),
496:     ));
497: 
498:     // ------------------------------------------------------------ binary64
499:     let phase = Phase::start();
500:     let b64 = parity::parity(&model, false);
501:     emit(merge(
502:         match b64 {
503:             Ok(p) => json!({ "kind": "binary64", "case": id, "sparse": p.sparse }),
504:             Err(e) => json!({ "kind": "binary64", "case": id, "error": e }),
505:         },
506:         phase.fields(),
507:     ));
508: 
509:     emit(json!({
510:         "kind": "summary", "case": id,
511:         "repeats_heap_peak": alloc::peak(), "repeats_heap_peak_move": alloc::peak_move(),
512:     }));
513: }
