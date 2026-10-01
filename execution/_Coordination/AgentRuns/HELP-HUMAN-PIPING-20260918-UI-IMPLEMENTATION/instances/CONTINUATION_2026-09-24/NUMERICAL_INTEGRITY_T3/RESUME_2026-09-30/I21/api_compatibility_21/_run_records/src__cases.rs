1: //! The committed case files (`cases/*.jsonl`, written by `gen_vk_cases.py`
2: //! from R1's frozen references), their kernel models and R1's key map.
3: //!
4: //! - A model is V-K's adapted kernel input (plan §4.1): binary64 bits in R1's
5: //!   node and member order, springs as global-axis or directional, rigid DOFs,
6: //!   loads (RF-CANCEL's contributions one each) and one station per member at
7: //!   0.5. `Model::source_parts` builds K4's `SourceParts` from it.
8: //! - `Model::resolve` maps an R1 key to the kernel quantity it is compared with
9: //!   (plan §4.2).
10: //! - Every expected value, scale and control value stays R1's decimal string.
11: use open_pipe_stress_frame_kernel::structural::retained_api::{
12:     Component, Constraint, DirectionalSpring, Dof, NodalLoad, SourceParts, Spring, SpringKind,
13:     Station, StraightMember,
14: };
15: use serde_json::Value;
16: use std::collections::BTreeMap;
17: use std::path::{Path, PathBuf};
18: 
19: /// The ten family files, in R1's family order.
20: pub const FAMILY_FILES: [(&str, &str); 10] = [
21:     ("RF-CHAIN", "rf_chain.jsonl"),
22:     ("RF-SKEW", "rf_skew.jsonl"),
23:     ("RF-WEAK", "rf_weak.jsonl"),
24:     ("RF-LARGE", "rf_large.jsonl"),
25:     ("RF-INVARIANCE", "rf_invariance.jsonl"),
26:     ("RF-RANGE", "rf_range.jsonl"),
27:     ("RF-ZERO", "rf_zero.jsonl"),
28:     ("RF-FINITE", "rf_finite.jsonl"),
29:     ("RF-MECH", "rf_mech.jsonl"),
30:     ("RF-CANCEL", "rf_cancel.jsonl"),
31: ];
32: 
33: pub fn crate_dir() -> PathBuf {
34:     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
35: }
36: 
37: pub fn cases_dir() -> PathBuf {
38:     crate_dir().join("cases")
39: }
40: 
41: #[derive(Clone, Debug)]
42: pub struct Row {
43:     pub key: String,
44:     pub expected: String,
45:     pub class: String,
46:     /// RF-CANCEL's recommended (binding, net-governed) scale; None elsewhere.
47:     pub scale: Option<String>,
48: }
49: 
50: impl Row {
51:     /// The class without RF-WEAK's `@region` suffix.
52:     pub fn kind(&self) -> &str {
53:         self.class.split('@').next().unwrap()
54:     }
55: }
56: 
57: #[derive(Clone, Debug)]
58: pub enum ControlKind {
59:     /// R1's violating values of a value control, by key.
60:     Value(Vec<(String, String)>),
61:     /// An outcome control: the defect it describes.
62:     Outcome(String),
63: }
64: 
65: #[derive(Clone, Debug)]
66: pub struct Control {
67:     pub id: String,
68:     pub discriminates: bool,
69:     pub kind: ControlKind,
70: }
71: 
72: #[derive(Clone, Debug)]
73: pub struct Member {
74:     pub name: String,
75:     pub id: u32,
76:     pub node_i: u32,
77:     pub node_j: u32,
78:     pub elastic_modulus: f64,
79:     pub shear_modulus: f64,
80:     pub area: f64,
81:     pub second_moment_y: f64,
82:     pub second_moment_z: f64,
83:     pub torsion_constant: f64,
84:     pub y_reference: [f64; 3],
85: }
86: 
87: #[derive(Clone, Debug)]
88: pub struct SpringSpec {
89:     /// R1's `S.<node>.<index>`.
90:     pub key: String,
91:     pub id: u32,
92:     pub node: u32,
93:     pub translation: bool,
94:     /// The global component (0–5) of a global-axis spring; None for a
95:     /// directional one.
96:     pub axis: Option<usize>,
97:     pub direction: [f64; 3],
98:     pub stiffness: f64,
99: }
100: 
101: #[derive(Clone, Debug)]
102: pub struct Model {
103:     pub node_names: Vec<String>,
104:     pub nodes: Vec<[f64; 3]>,
105:     pub members: Vec<Member>,
106:     pub springs: Vec<SpringSpec>,
107:     /// R1 springs with k = 0, omitted by the adapter (RF-MECH-K0; plan §4.1).
108:     pub omitted_springs: Vec<String>,
109:     pub constraints: Vec<(u32, usize)>,
110:     pub loads: Vec<(u32, usize, f64, String)>,
111:     pub stations: Vec<(u32, u32, f64)>,
112: }
113: 
114: #[derive(Clone, Debug)]
115: pub struct Case {
116:     pub id: String,
117:     pub family: String,
118:     pub basis: String,
119:     pub units: String,
120:     pub needs_directional_spring: bool,
121:     pub refuse: bool,
122:     pub k4src_sha256: String,
123:     /// None for RF-LARGE at 1,000 and 10,000 members (generated on demand).
124:     pub model: Option<Model>,
125:     pub model_sha256: Option<String>,
126:     pub scales: BTreeMap<String, String>,
127:     pub rows: Vec<Row>,
128:     pub controls: Vec<Control>,
129:     /// The committed not-covered keys of this case (DESIGN.md §4.10's list).
130:     pub not_covered: Vec<String>,
131:     /// S(kind) of the complete reference solution (n ≥ 1,000 only).
132:     pub s_full: Option<BTreeMap<String, String>>,
133: }
134: 
135: impl Case {
136:     /// The comparison scale string of a row: its own (RF-CANCEL) or its class's.
137:     pub fn scale_of<'a>(&'a self, row: &'a Row) -> &'a str {
138:         match &row.scale {
139:             Some(s) => s,
140:             None => &self.scales[&row.class],
141:         }
142:     }
143: 
144:     /// RF-LARGE at 1,000 or 10,000 members: an example, never a CI test.
145:     pub fn is_large(&self) -> bool {
146:         self.model.is_none()
147:     }
148: }
149: 
150: fn hex(v: &Value) -> f64 {
151:     let s = v.as_str().expect("hex string");
152:     f64::from_bits(u64::from_str_radix(s, 16).expect("16 hex digits"))
153: }
154: 
155: fn text(v: &Value) -> String {
156:     v.as_str().expect("string").to_string()
157: }
158: 
159: fn uint(v: &Value) -> u32 {
160:     u32::try_from(v.as_u64().expect("integer")).expect("u32")
161: }
162: 
163: pub fn parse_model(m: &Value) -> Model {
164:     let arr = |k: &str| m[k].as_array().unwrap_or_else(|| panic!("model.{k}"));
165:     let mut node_names = Vec::new();
166:     let mut nodes = Vec::new();
167:     for n in arr("nodes") {
168:         node_names.push(text(&n[0]));
169:         nodes.push([hex(&n[1]), hex(&n[2]), hex(&n[3])]);
170:     }
171:     let members = arr("members")
172:         .iter()
173:         .map(|x| Member {
174:             name: text(&x[0]),
175:             id: uint(&x[1]),
176:             node_i: uint(&x[2]),
177:             node_j: uint(&x[3]),
178:             elastic_modulus: hex(&x[4]),
179:             shear_modulus: hex(&x[5]),
180:             area: hex(&x[6]),
181:             second_moment_y: hex(&x[7]),
182:             second_moment_z: hex(&x[8]),
183:             torsion_constant: hex(&x[9]),
184:             y_reference: [hex(&x[10]), hex(&x[11]), hex(&x[12])],
185:         })
186:         .collect();
187:     let springs = arr("springs")
188:         .iter()
189:         .map(|s| {
190:             let d = s[5].as_array().expect("direction");
191:             SpringSpec {
192:                 key: text(&s[0]),
193:                 id: uint(&s[1]),
194:                 node: uint(&s[2]),
195:                 translation: s[3].as_str() == Some("t"),
196:                 axis: s[4].as_u64().map(|a| a as usize),
197:                 direction: [hex(&d[0]), hex(&d[1]), hex(&d[2])],
198:                 stiffness: hex(&s[6]),
199:             }
200:         })
201:         .collect();
202:     Model {
203:         node_names,
204:         nodes,
205:         members,
206:         springs,
207:         omitted_springs: arr("omitted_springs").iter().map(text).collect(),
208:         constraints: arr("constraints")
209:             .iter()
210:             .map(|c| (uint(&c[0]), uint(&c[1]) as usize))
211:             .collect(),
212:         loads: arr("loads")
213:             .iter()
214:             .map(|l| (uint(&l[0]), uint(&l[1]) as usize, hex(&l[2]), text(&l[3])))
215:             .collect(),
216:         stations: arr("stations")
217:             .iter()
218:             .map(|s| (uint(&s[0]), uint(&s[1]), hex(&s[2])))
219:             .collect(),
220:     }
221: }
222: 
223: pub fn parse_case(line: &str) -> Case {
224:     let v: Value = serde_json::from_str(line).expect("case JSON");
225:     let rows = v["rows"]
226:         .as_array()
227:         .expect("rows")
228:         .iter()
229:         .map(|r| Row {
230:             key: text(&r[0]),
231:             expected: text(&r[1]),
232:             class: text(&r[2]),
233:             scale: r[3].as_str().map(str::to_string),
234:         })
235:         .collect();
236:     let controls = v["controls"]
237:         .as_array()
238:         .expect("controls")
239:         .iter()
240:         .map(|c| Control {
241:             id: text(&c[0]),
242:             discriminates: c[1].as_bool().expect("bool"),
243:             kind: if c[2].as_str() == Some("value") {
244:                 ControlKind::Value(
245:                     c[3].as_object()
246:                         .expect("values")
247:                         .iter()
248:                         .map(|(k, v)| (k.clone(), text(v)))
249:                         .collect(),
250:                 )
251:             } else {
252:                 ControlKind::Outcome(text(&c[3]["defect"]))
253:             },
254:         })
255:         .collect();
256:     let strings = |x: &Value| -> BTreeMap<String, String> {
257:         x.as_object()
258:             .map(|o| o.iter().map(|(k, v)| (k.clone(), text(v))).collect())
259:             .unwrap_or_default()
260:     };
261:     Case {
262:         id: text(&v["id"]),
263:         family: text(&v["family"]),
264:         basis: text(&v["basis"]),
265:         units: text(&v["units"]),
266:         needs_directional_spring: v["needs_directional_spring"].as_bool().unwrap(),
267:         refuse: v["refuse"].as_bool().unwrap(),
268:         k4src_sha256: text(&v["k4src_sha256"]),
269:         model: (!v["model"].is_null()).then(|| parse_model(&v["model"])),
270:         model_sha256: v["model_sha256"].as_str().map(str::to_string),
271:         scales: strings(&v["scales"]),
272:         rows,
273:         controls,
274:         not_covered: v["not_covered"]
275:             .as_array()
276:             .unwrap()
277:             .iter()
278:             .map(text)
279:             .collect(),
280:         s_full: v.get("s_full").map(strings),
281:     }
282: }
283: 
284: pub fn load_file(path: &Path) -> Vec<Case> {
285:     std::fs::read_to_string(path)
286:         .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
287:         .lines()
288:         .filter(|l| !l.is_empty())
289:         .map(parse_case)
290:         .collect()
291: }
292: 
293: pub fn load_family(family: &str) -> Vec<Case> {
294:     let (_, file) = FAMILY_FILES
295:         .iter()
296:         .find(|(f, _)| *f == family)
297:         .unwrap_or_else(|| panic!("{family}"));
298:     load_file(&cases_dir().join(file))
299: }
300: 
301: pub fn load_all() -> Vec<Case> {
302:     FAMILY_FILES
303:         .iter()
304:         .flat_map(|(_, f)| load_file(&cases_dir().join(f)))
305:         .collect()
306: }
307: 
308: /// The committed expected-unresolved list (`cases/expected_unresolved.json`;
309: /// ROOT's ruling on I17's A1 stop): the cases W1a is expected to leave
310: /// honestly unresolved, with no rows.
311: pub fn expected_unresolved() -> &'static [String] {
312:     static LIST: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
313:     LIST.get_or_init(|| {
314:         let text = std::fs::read_to_string(cases_dir().join("expected_unresolved.json"))
315:             .expect("expected_unresolved.json");
316:         let v: Value = serde_json::from_str(&text).expect("JSON");
317:         v["entries"]
318:             .as_array()
319:             .expect("entries")
320:             .iter()
321:             .map(|e| text_of(&e["case"]))
322:             .collect()
323:     })
324: }
325: 
326: fn text_of(v: &Value) -> String {
327:     text(v)
328: }
329: 
330: /// A large model written by `gen_vk_cases.py --large <dir>` (examples only).
331: pub fn load_large_model(path: &Path) -> (Model, String) {
332:     let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
333:     let v: Value = serde_json::from_str(&text).expect("model JSON");
334:     (parse_model(&v), crate::sha256::sha256_hex(text.as_bytes()))
335: }
336: 
337: /// The kernel quantity an R1 key is compared with (plan §4.2).
338: #[derive(Clone, Copy, Debug, PartialEq, Eq)]
339: pub enum Target {
340:     Displacement(Dof),
341:     Reaction(Dof),
342:     Spring {
343:         id: u32,
344:         component: Component,
345:     },
346:     DirectionalSpring {
347:         id: u32,
348:         component: Component,
349:     },
350:     /// A component a global-axis spring (or a spring of the other kind) cannot
351:     /// have: no published row; R1's value must be exactly 0.
352:     StructuralZero,
353:     Axial(u32),
354:     Torque(u32),
355:     BendingEnd {
356:         member: u32,
357:         j_end: bool,
358:     },
359:     BendingStation(u32),
360:     Twist(u32),
361:     Extension(u32),
362: }
363: 
364: fn axis_offset(letter: &str) -> Option<usize> {
365:     match letter {
366:         "X" => Some(0),
367:         "Y" => Some(1),
368:         "Z" => Some(2),
369:         _ => None,
370:     }
371: }
372: 
373: impl Model {
374:     pub fn node_index(&self, name: &str) -> Option<u32> {
375:         self.node_names
376:             .iter()
377:             .position(|n| n == name)
378:             .map(|k| k as u32)
379:     }
380: 
381:     pub fn member(&self, name: &str) -> Option<&Member> {
382:         self.members.iter().find(|m| m.name == name)
383:     }
384: 
385:     pub fn resolve(&self, key: &str) -> Result<Target, String> {
386:         let bad = || format!("unresolved R1 key {key}");
387:         let parts: Vec<&str> = key.split('.').collect();
388:         let node = |name: &str| self.node_index(name).ok_or_else(bad);
389:         let member = |name: &str| self.member(name).map(|m| m.id).ok_or_else(bad);
390:         let dof = |n: u32, c: usize| Dof {
391:             node: n,
392:             component: Component::from_index(c),
393:         };
394:         match parts.as_slice() {
395:             ["u", n, c] => {
396:                 let a = c.strip_prefix('U').and_then(axis_offset).ok_or_else(bad)?;
397:                 Ok(Target::Displacement(dof(node(n)?, a)))
398:             }
399:             ["th", n, c] => {
400:                 let a = c.strip_prefix('R').and_then(axis_offset).ok_or_else(bad)?;
401:                 Ok(Target::Displacement(dof(node(n)?, 3 + a)))
402:             }
403:             ["R", n, d] => {
404:                 let (kind, axis) = d.split_at(1);
405:                 let off = match kind {
406:                     "U" => 0,
407:                     "R" => 3,
408:                     _ => return Err(bad()),
409:                 };
410:                 Ok(Target::Reaction(dof(
411:                     node(n)?,
412:                     off + axis_offset(axis).ok_or_else(bad)?,
413:                 )))
414:             }
415:             ["S", n, i, c] => {
416:                 let spec_key = format!("S.{n}.{i}");
417:                 let s = self
418:                     .springs
419:                     .iter()
420:                     .find(|s| s.key == spec_key)
421:                     .ok_or_else(bad)?;
422:                 let (kind, axis) = c.split_at(1);
423:                 let force = match kind {
424:                     "F" => true,
425:                     "M" => false,
426:                     _ => return Err(bad()),
427:                 };
428:                 let component = if force { 0 } else { 3 } + axis_offset(axis).ok_or_else(bad)?;
429:                 if force != s.translation {
430:                     return Ok(Target::StructuralZero);
431:                 }
432:                 match s.axis {
433:                     Some(a) if a == component => Ok(Target::Spring {
434:                         id: s.id,
435:                         component: Component::from_index(component),
436:                     }),
437:                     Some(_) => Ok(Target::StructuralZero),
438:                     None => Ok(Target::DirectionalSpring {
439:                         id: s.id,
440:                         component: Component::from_index(component),
441:                     }),
442:                 }
443:             }
444:             ["N", m] => Ok(Target::Axial(member(m)?)),
445:             ["T", m] => Ok(Target::Torque(member(m)?)),
446:             ["Mb", m, "i"] => Ok(Target::BendingEnd {
447:                 member: member(m)?,
448:                 j_end: false,
449:             }),
450:             ["Mb", m, "j"] => Ok(Target::BendingEnd {
451:                 member: member(m)?,
452:                 j_end: true,
453:             }),
454:             ["Mb", m, "mid"] => {
455:                 let id = member(m)?;
456:                 let station = self
457:                     .stations
458:                     .iter()
459:                     .find(|s| s.1 == id && s.2 == 0.5)
460:                     .ok_or_else(bad)?;
461:                 Ok(Target::BendingStation(station.0))
462:             }
463:             ["tw", m] => Ok(Target::Twist(member(m)?)),
464:             ["ext", m] => Ok(Target::Extension(member(m)?)),
465:             _ => Err(bad()),
466:         }
467:     }
468: 
469:     /// K4's `SourceParts` for this model (the adapter's Rust half).
470:     pub fn source_parts(&self) -> SourceParts {
471:         let dof = |node: u32, c: usize| Dof {
472:             node,
473:             component: Component::from_index(c),
474:         };
475:         let mut parts = SourceParts {
476:             nodes: self.nodes.clone(),
477:             ..Default::default()
478:         };
479:         for m in &self.members {
480:             parts.members.push(StraightMember {
481:                 id: m.id,
482:                 node_i: m.node_i,
483:                 node_j: m.node_j,
484:                 elastic_modulus: m.elastic_modulus,
485:                 shear_modulus: m.shear_modulus,
486:                 area: m.area,
487:                 second_moment_y: m.second_moment_y,
488:                 second_moment_z: m.second_moment_z,
489:                 torsion_constant: m.torsion_constant,
490:                 y_reference: m.y_reference,
491:             });
492:         }
493:         for s in &self.springs {
494:             match s.axis {
495:                 Some(a) => parts.springs.push(Spring {
496:                     id: s.id,
497:                     dof: dof(s.node, a),
498:                     stiffness: s.stiffness,
499:                 }),
500:                 None => parts.directional_springs.push(DirectionalSpring {
501:                     id: s.id,
502:                     node: s.node,
503:                     kind: if s.translation {
504:                         SpringKind::Translation
505:                     } else {
506:                         SpringKind::Rotation
507:                     },
508:                     direction: s.direction,
509:                     stiffness: s.stiffness,
510:                 }),
511:             }
512:         }
513:         for &(n, c) in &self.constraints {
514:             parts.constraints.push(Constraint {
515:                 dof: dof(n, c),
516:                 value: 0.0,
517:             });
518:         }
519:         for (n, c, v, src) in &self.loads {
520:             parts.loads.push(NodalLoad {
521:                 dof: dof(*n, *c),
522:                 value: *v,
523:                 source_id: src.clone(),
524:             });
525:         }
526:         for &(id, member, fraction) in &self.stations {
527:             parts.stations.push(Station {
528:                 id,
529:                 member,
530:                 fraction,
531:             });
532:         }
533:         parts
534:     }
535: 
536:     /// The connected components of the member graph, as a body per node.
537:     pub fn bodies(&self) -> Vec<u32> {
538:         let n = self.nodes.len();
539:         let mut parent: Vec<usize> = (0..n).collect();
540:         fn root(p: &mut [usize], mut x: usize) -> usize {
541:             while p[x] != x {
542:                 p[x] = p[p[x]];
543:                 x = p[x];
544:             }
545:             x
546:         }
547:         for m in &self.members {
548:             let (a, b) = (
549:                 root(&mut parent, m.node_i as usize),
550:                 root(&mut parent, m.node_j as usize),
551:             );
552:             if a != b {
553:                 parent[a.max(b)] = a.min(b);
554:             }
555:         }
556:         let mut label = vec![u32::MAX; n];
557:         let mut next = 0;
558:         let mut out = vec![0; n];
559:         for k in 0..n {
560:             let r = root(&mut parent, k);
561:             if label[r] == u32::MAX {
562:                 label[r] = next;
563:                 next += 1;
564:             }
565:             out[k] = label[r];
566:         }
567:         out
568:     }
569: }
