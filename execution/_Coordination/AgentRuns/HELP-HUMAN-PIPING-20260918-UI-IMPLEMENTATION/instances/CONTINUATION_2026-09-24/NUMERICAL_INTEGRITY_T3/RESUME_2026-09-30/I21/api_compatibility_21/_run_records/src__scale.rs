1: //! V-K's scale runs (checkpoint B; plan §13): W1's deterministic counts of a
2: //! model, in O(nnz) and without a solve, and the admission estimate.
3: //!
4: //! - **The counts** follow K4's own rules. The structural pattern is the
5: //!   members' node blocks plus the springs at nodes no member touches (K4R
6: //!   `assemble.rs` `Structure`). The profile is K4's skyline rule
7: //!   (`K4R/factor.rs` `order_free`: first = the least rank over a row's
8: //!   neighbours and itself) over the exported `reverse_cuthill_mckee`, on the
9: //!   free–free adjacency (`rcm::free_adjacency`). `tests/scale.rs` checks both
10: //!   against K4's `StorageCounts` in every committed record.
11: //! - **The estimate** is K6b's E_max (`performance_harness/src/k6/w1/counts.rs`,
12: //!   `estimate`, at K6b's `082990c8d`, recomputed there on KF1's bounded
13: //!   trackers), ported term for term. Its constants are the sizes of FK's
14: //!   crate-private types, derived from their definitions, with K6b's citations.
15: //!   Only the model term is V-K's own (`ModelSizes`). K6's admission rule
16: //!   calibrates the estimate with the measured ratio ρ. Nothing asserts a
17: //!   memory bound. Once K6b's crate is on main, the two copies can be
18: //!   deduplicated in one follow-up.
19: use crate::cases::{Member, Model, SpringSpec};
20: use crate::rcm::free_adjacency;
21: use open_pipe_stress_frame_kernel::structural::retained_api::{
22:     layout, reverse_cuthill_mckee, AttemptRecord, Constraint, NodalLoad, PrimitiveSource,
23:     PublishedRow, QuantityId, QuantityMeta, Station, StraightMember,
24: };
25: use std::collections::BTreeSet;
26: 
27: const DOF: usize = 6;
28: 
29: /// W1's deterministic counts of one model (K6b's `W1Counts`, the same fields).
30: #[derive(Debug, Clone, PartialEq, Eq)]
31: pub struct Counts {
32:     pub nodes: usize,
33:     pub members: usize,
34:     pub springs: usize,
35:     pub stations: usize,
36:     pub constraints: usize,
37:     pub loads: usize,
38:     pub dofs: usize,
39:     pub free_dofs: usize,
40:     pub bodies: usize,
41:     /// Stored entries of K4's structural pattern (both triangles).
42:     pub pattern_entries: usize,
43:     /// K4's skyline of the RCM-ordered free–free pattern.
44:     pub profile_entries: usize,
45:     pub half_bandwidth: usize,
46:     /// Connected components of the free–free adjacency (K4's blocks).
47:     pub blocks: usize,
48:     /// The published layout's length (`layout(&source).len()`).
49:     pub rows: usize,
50:     /// K4SRC's length (`PrimitiveSource::encoding`).
51:     pub source_encoding_len: usize,
52: }
53: 
54: /// K4's structural pattern entries: 36 per node a member touches, 36 per
55: /// ordered pair of member-adjacent nodes, and the spring entries at nodes no
56: /// member touches (a global-axis spring's diagonal; a directional spring's
57: /// 3×3 block), each entry once.
58: pub fn pattern_entries(model: &Model) -> usize {
59:     let n = model.nodes.len();
60:     let mut touched = vec![false; n];
61:     let mut pairs: BTreeSet<(u32, u32)> = BTreeSet::new();
62:     for m in &model.members {
63:         touched[m.node_i as usize] = true;
64:         touched[m.node_j as usize] = true;
65:         if m.node_i != m.node_j {
66:             pairs.insert((m.node_i, m.node_j));
67:             pairs.insert((m.node_j, m.node_i));
68:         }
69:     }
70:     let mut spring_entries: BTreeSet<(u32, usize, usize)> = BTreeSet::new();
71:     for s in &model.springs {
72:         if touched[s.node as usize] {
73:             continue;
74:         }
75:         match s.axis {
76:             Some(a) => {
77:                 spring_entries.insert((s.node, a, a));
78:             }
79:             None => {
80:                 let off = if s.translation { 0 } else { 3 };
81:                 for a in 0..3 {
82:                     for b in 0..3 {
83:                         spring_entries.insert((s.node, off + a, off + b));
84:                     }
85:                 }
86:             }
87:         }
88:     }
89:     let touched = touched.iter().filter(|&&t| t).count();
90:     DOF * DOF * (touched + pairs.len()) + spring_entries.len()
91: }
92: 
93: /// K4's skyline on the free–free adjacency: (profile entries, half-bandwidth,
94: /// blocks).
95: pub fn profile(adjacency: &[Vec<usize>]) -> (usize, usize, usize) {
96:     let order = reverse_cuthill_mckee(adjacency);
97:     let mut rank = vec![0usize; adjacency.len()];
98:     for (k, &a) in order.iter().enumerate() {
99:         rank[a] = k;
100:     }
101:     let (mut entries, mut bandwidth) = (0usize, 0usize);
102:     for (i, &a) in order.iter().enumerate() {
103:         let first = adjacency[a]
104:             .iter()
105:             .map(|&b| rank[b])
106:             .chain(std::iter::once(i))
107:             .min()
108:             .unwrap_or(i);
109:         entries += i - first + 1;
110:         bandwidth = bandwidth.max(i - first);
111:     }
112:     let mut block = vec![usize::MAX; adjacency.len()];
113:     let mut blocks = 0;
114:     let mut stack = Vec::new();
115:     for seed in 0..adjacency.len() {
116:         if block[seed] != usize::MAX {
117:             continue;
118:         }
119:         block[seed] = blocks;
120:         stack.push(seed);
121:         while let Some(a) = stack.pop() {
122:             for &b in &adjacency[a] {
123:                 if block[b] == usize::MAX {
124:                     block[b] = blocks;
125:                     stack.push(b);
126:                 }
127:             }
128:         }
129:         blocks += 1;
130:     }
131:     (entries, bandwidth, blocks)
132: }
133: 
134: /// The counts of `model` and its source, with no solve.
135: pub fn counts(model: &Model, source: &PrimitiveSource) -> Counts {
136:     let (profile_entries, half_bandwidth, blocks) = profile(&free_adjacency(model));
137:     Counts {
138:         nodes: source.node_count(),
139:         members: source.members().len(),
140:         springs: model.springs.len(),
141:         stations: source.stations().len(),
142:         constraints: source.constraints().len(),
143:         loads: source.loads().len(),
144:         dofs: source.dof_count(),
145:         free_dofs: source.free_dofs().len(),
146:         bodies: source.body_count() as usize,
147:         pattern_entries: pattern_entries(model),
148:         profile_entries,
149:         half_bandwidth,
150:         blocks,
151:         rows: layout(source).len(),
152:         source_encoding_len: source.encoding().len(),
153:     }
154: }
155: 
156: // ------------------------------------------------------------------ estimate (K6b's, ported)
157: 
158: /// The bytes of one `Wide<L>` (`K4R/wide.rs`): 8L + 16 with padding.
159: pub const fn wide_bytes(limbs: u128) -> u128 {
160:     8 * limbs + 16
161: }
162: 
163: /// Each attempt's shared build: (p, L, R) (`K4R/adaptive.rs`).
164: pub const ATTEMPT_WIDTHS: [(u32, u128, u128); 4] =
165:     [(128, 4, 4), (256, 4, 8), (512, 8, 16), (1024, 16, 16)];
166: /// Each verification's shared data: (P, L_P, W).
167: pub const VERIFY_WIDTHS: [(u32, u128, u128); 3] = [(256, 4, 8), (512, 8, 16), (1024, 16, 16)];
168: /// `MemberOperators<L>` (`K4R/assemble.rs`): 164 values and three u32.
169: pub const MEMBER_OPERATOR_WIDES: u128 = 164;
170: pub const MEMBER_OPERATOR_EXTRA: u128 = 16;
171: /// `BoundedCoefficients<L>`: five values and a u32.
172: pub const BOUNDED_WIDES: u128 = 5;
173: pub const BOUNDED_EXTRA: u128 = 8;
174: /// A member's 12×12 block (`bounded_block`; K_e at q_W).
175: pub const BLOCK_WIDES: u128 = 144;
176: /// Per free row of `RetainedFactor<L>`: two values and 56 bytes.
177: pub const FACTOR_ROW_WIDES: u128 = 2;
178: pub const FACTOR_ROW_EXTRA: u128 = 56;
179: /// `BlockBound<L>`: three values and an option.
180: pub const BLOCK_BOUND_WIDES: u128 = 4;
181: pub const BLOCK_BOUND_EXTRA: u128 = 8;
182: /// `Structure`: 78 upper contributions per member, 8 bytes each, 16 while
183: /// tagged during the build.
184: pub const CONTRIBUTIONS_PER_MEMBER: u128 = 78;
185: pub const CONTRIBUTION_BYTES: u128 = 8;
186: pub const TAGGED_CONTRIBUTION_BYTES: u128 = 16;
187: /// The verification report's per-row option vectors.
188: pub const REPORT_ROW_VECTORS: u128 = 5;
189: pub const OPTION_EXTRA: u128 = 8;
190: /// A solve's working vectors at the residual width.
191: pub const SOLVE_VECTORS: u128 = 4;
192: /// `ExactWideSum` (`K4R/wide_sum.rs`): 2,144 bytes with alignment.
193: pub const EXACT_WIDE_SUM_BYTES: u128 = 2144;
194: /// An unevaluated tracker row, or a fallback row: 4,304 bytes (KF1).
195: pub const TRACKER_ENTRY_BYTES: u128 = 2 * EXACT_WIDE_SUM_BYTES + 16;
196: /// A tracker table entry: 40 bytes (KF1).
197: pub const TRACKER_TABLE_ENTRY_BYTES: u128 = 40;
198: /// KF1's bounds on unevaluated rows at T = 512, G = 8T, each at its peak.
199: pub const STOP_RULE_PEAK_ROWS: u128 = 4096 + 512;
200: pub const PIVOT_TRACKER_PEAK_ROWS: u128 = 768;
201: pub const SOLVE_TRACKER_PEAK_ROWS: u128 = 2816;
202: pub const DECIDE_TRACKER_SETS: u128 = 3;
203: pub const VEC_SLACK: u128 = 2;
204: pub const SOLVE_TABLE_ROWS_PER_FREE_DOF: u128 = 5;
205: pub const RESIDUAL_ROW_EXTRA: u128 = 16;
206: pub const LEDGER_ENTRY_BYTES: u128 = 64;
207: pub const PRESCRIBED_ENTRY_BYTES: u128 = 48;
208: pub const SOURCE_ID_BYTES: u128 = 16;
209: /// A heap string's allocation for a short label (node, member or source id).
210: pub const LABEL_BYTES: u128 = 16;
211: /// A `BTreeMap` entry's overhead per key and value (node share, amortized).
212: pub const MAP_ENTRY_EXTRA: u128 = 32;
213: 
214: /// Sizes of the exported types and of V-K's model types, from this build.
215: #[derive(Debug, Clone, Copy, PartialEq, Eq)]
216: pub struct Sizes {
217:     pub straight_member: usize,
218:     pub constraint: usize,
219:     pub nodal_load: usize,
220:     pub station: usize,
221:     pub quantity_meta: usize,
222:     pub quantity_id: usize,
223:     pub published_row: usize,
224:     pub attempt_record: usize,
225:     pub vk_member: usize,
226:     pub vk_spring: usize,
227:     pub vk_load: usize,
228: }
229: 
230: impl Sizes {
231:     pub fn of_this_build() -> Self {
232:         Self {
233:             straight_member: std::mem::size_of::<StraightMember>(),
234:             constraint: std::mem::size_of::<Constraint>(),
235:             nodal_load: std::mem::size_of::<NodalLoad>(),
236:             station: std::mem::size_of::<Station>(),
237:             quantity_meta: std::mem::size_of::<QuantityMeta>(),
238:             quantity_id: std::mem::size_of::<QuantityId>(),
239:             published_row: std::mem::size_of::<PublishedRow>(),
240:             attempt_record: std::mem::size_of::<AttemptRecord>(),
241:             vk_member: std::mem::size_of::<Member>(),
242:             vk_spring: std::mem::size_of::<SpringSpec>(),
243:             vk_load: std::mem::size_of::<(u32, usize, f64, String)>(),
244:         }
245:     }
246: }
247: 
248: /// The estimate and its terms (bytes).
249: #[derive(Debug, Clone, Copy, PartialEq, Eq)]
250: pub struct Estimate {
251:     /// V-K's model and its lane state (the published map), alive throughout.
252:     pub model: u128,
253:     /// The stop rule's tracker bound (a transient at each decision).
254:     pub decide: u128,
255:     /// Alive through the call: the model, the source twice, the case, the group.
256:     pub fixed: u128,
257:     /// E_max: the peak over the whole schedule (128 … v1024).
258:     pub max: u128,
259:     /// E_sel128: the peak of the path selected at 128 (128, 256, v256).
260:     pub sel128: u128,
261: }
262: 
263: /// V-K's model term: `cases::Model` (node labels and coordinates, members,
264: /// springs, constraints, loads with their source ids, stations) and the
265: /// lane's published map (`lane::CaseRun::published`).
266: pub fn model_bytes(c: &Counts, s: &Sizes) -> u128 {
267:     let u = |x: usize| x as u128;
268:     u(c.nodes) * (24 + 24 + LABEL_BYTES)
269:         + u(c.members) * (u(s.vk_member) + LABEL_BYTES)
270:         + u(c.springs) * (u(s.vk_spring) + LABEL_BYTES)
271:         + u(c.constraints) * 16
272:         + u(c.loads) * (u(s.vk_load) + LABEL_BYTES)
273:         + u(c.stations) * 16
274:         + u(c.rows) * (u(s.quantity_id) + u(s.published_row) + MAP_ENTRY_EXTRA)
275: }
276: 
277: /// K6b's W1 estimate, ported; build transients are added on top of each
278: /// build's kept bytes, so it bounds rather than tracks the peak.
279: pub fn estimate(c: &Counts, s: &Sizes) -> Estimate {
280:     let u = |x: usize| x as u128;
281:     let (m, n, nf, nnz) = (
282:         u(c.members),
283:         u(c.dofs),
284:         u(c.free_dofs),
285:         u(c.pattern_entries),
286:     );
287:     let (p_entries, blocks, rows) = (u(c.profile_entries), u(c.blocks.max(1)), u(c.rows));
288:     let (nodes, r, loads, stations) = (u(c.nodes), u(c.constraints), u(c.loads), u(c.stations));
289:     let enc = u(c.source_encoding_len);
290:     let w = wide_bytes;
291: 
292:     let model = model_bytes(c, s);
293:     let source = 24 * nodes
294:         + m * u(s.straight_member)
295:         + r * u(s.constraint)
296:         + loads * (u(s.nodal_load) + SOURCE_ID_BYTES)
297:         + stations * u(s.station)
298:         + 16 * n
299:         + 4 * nodes;
300:     let case = loads * LEDGER_ENTRY_BYTES
301:         + r * PRESCRIBED_ENTRY_BYTES
302:         + enc
303:         + rows * u(s.quantity_meta)
304:         + 8 * u(c.bodies);
305:     let group = 8 * (n + 1)
306:         + 16 * nnz
307:         + 8 * (nnz + 1)
308:         + m * CONTRIBUTIONS_PER_MEMBER * CONTRIBUTION_BYTES
309:         + 32 * nf
310:         + 8 * n
311:         + 12 * nf
312:         + 24 * blocks;
313:     let group_build = m * CONTRIBUTIONS_PER_MEMBER * (TAGGED_CONTRIBUTION_BYTES + 16);
314:     let fixed = model + 2 * source + case + group;
315: 
316:     let shared = ATTEMPT_WIDTHS.map(|(_, l, r)| {
317:         m * (MEMBER_OPERATOR_WIDES * w(l) + MEMBER_OPERATOR_EXTRA)
318:             + nnz * (w(l) + w(r))
319:             + m * (BOUNDED_WIDES * w(r) + BOUNDED_EXTRA)
320:             + p_entries * w(l)
321:             + nf * (FACTOR_ROW_WIDES * w(l) + FACTOR_ROW_EXTRA)
322:             + blocks * w(l)
323:     });
324:     let shared_build = ATTEMPT_WIDTHS.map(|(p, _, r)| {
325:         if p == 1024 {
326:             0
327:         } else {
328:             m * (MEMBER_OPERATOR_WIDES * w(r) + MEMBER_OPERATOR_EXTRA)
329:         }
330:     });
331:     let state = ATTEMPT_WIDTHS.map(|(_, l, _)| (n + 6 * m + rows) * w(l));
332:     let solve_trackers = SOLVE_TRACKER_PEAK_ROWS * TRACKER_ENTRY_BYTES
333:         + VEC_SLACK * SOLVE_TABLE_ROWS_PER_FREE_DOF * nf * TRACKER_TABLE_ENTRY_BYTES
334:         + VEC_SLACK * nf * TRACKER_ENTRY_BYTES;
335:     let solve = ATTEMPT_WIDTHS.map(|(_, l, r)| {
336:         SOLVE_VECTORS * n * w(r) + nf * (RESIDUAL_ROW_EXTRA + w(l)) + solve_trackers
337:     });
338:     let verify = VERIFY_WIDTHS.map(|(_, l, ww)| {
339:         nnz * w(l)
340:             + m * BLOCK_WIDES * w(ww)
341:             + blocks * (BLOCK_BOUND_WIDES * w(l) + BLOCK_BOUND_EXTRA)
342:     });
343:     let verify_build = VERIFY_WIDTHS.map(|(p, l, ww)| {
344:         m * (BOUNDED_WIDES * w(l) + BOUNDED_EXTRA)
345:             + m * BLOCK_WIDES * w(l)
346:             + if p == 1024 {
347:                 0
348:             } else {
349:                 m * (MEMBER_OPERATOR_WIDES * w(ww) + MEMBER_OPERATOR_EXTRA)
350:             }
351:     });
352:     let report = VERIFY_WIDTHS
353:         .map(|(_, l, _)| REPORT_ROW_VECTORS * rows * (w(l) + OPTION_EXTRA) + 2 * nf * w(l));
354:     let pass = VERIFY_WIDTHS.map(|(_, l, _)| p_entries * w(l));
355:     let decide = STOP_RULE_PEAK_ROWS * TRACKER_ENTRY_BYTES
356:         + DECIDE_TRACKER_SETS * VEC_SLACK * rows * TRACKER_TABLE_ENTRY_BYTES;
357:     let pivot =
358:         PIVOT_TRACKER_PEAK_ROWS * TRACKER_ENTRY_BYTES + VEC_SLACK * nf * TRACKER_TABLE_ENTRY_BYTES;
359:     let end = rows * (u(s.published_row) + 40)
360:         + enc
361:         + (n + 6 * m) * (9 + 8 * 16)
362:         + 8 * u(s.attempt_record);
363: 
364:     let mut kept = fixed;
365:     let mut peak = fixed + group_build;
366:     let mut sel128 = 0;
367:     for (k, &(p, _, _)) in ATTEMPT_WIDTHS.iter().enumerate() {
368:         peak = peak.max(kept + shared[k] + shared_build[k] + pivot);
369:         kept += shared[k];
370:         peak = peak.max(kept + state[k] + solve[k]);
371:         kept += state[k];
372:         if let Some(v) = VERIFY_WIDTHS.iter().position(|&(vp, _, _)| vp == p) {
373:             peak = peak.max(kept + verify[v] + verify_build[v]);
374:             kept += verify[v];
375:             peak = peak.max(kept + report[v] + pass[v]);
376:             peak = peak.max(kept + report[v] + decide);
377:             if p == 256 {
378:                 sel128 = peak.max(kept + report[v] + end);
379:             }
380:         }
381:     }
382:     let max = peak.max(kept + report[2] + end);
383:     Estimate {
384:         model,
385:         decide,
386:         fixed,
387:         max,
388:         sel128,
389:     }
390: }
