# Source11 finite VR caller formulas and remaining cells

Status: proposed source accounting, subject to independent review. This supplements VR_ENVELOPE.md; neither file closes global E_max. Frozen product source is 40129a225d73860ac2a53da9a2fa73869df668f3. Start 2026-10-01 14:34:09 UTC; boundary 15:04:09 UTC. P = projects/chirality-piping; VR = P/validation/benchmarks/numerical_robustness. V = VR/examples/vk_scale.rs, L = VR/src/lane.rs, RD = VR/src/records.rs, SD = P/core/solver/sparse_direct.

## Operators and identity composition

Use source02 exact-copy E_s(k)=s*k, retained growth G_s(k), and one active old allocation per growth edge. Checked multiply/add and capacity/layout ceiling checks are prerequisites, not silent saturation. Arrays collected through Result retain the source06 fallible-capacity rule. Never replace capacity by length for those arrays. Kernel terms are source05 corrected by source06; source07 queue and reviewed layout08 facts apply only on their authenticated source/target basis.

For a phase phi:
  Live(phi) = C_pre_surviving(phi) + sum(distinct caller owners alive in phi) + K_phi.
K_phi is present only while the kernel outcome/computation is alive. A moved record, vector or selected result is one allocation identity. A deep clone is another. The phase maximum also includes source02 move-request growth, separately from retained heap. Pre-cut owners dropped during a phase may conservatively remain prepaid, explicitly as padding; no owner is silently subtracted from an observed counter.

The source09 descriptors/census remain fixed: 213 cases comprise 193 factored, 8 refused, 12 external; vk_scale's embedded branch loads RF-LARGE and selects one of its actual 24 cases. No family/model/shape/spring/body/cancellation/geometry exclusions are proposed. Arbitrary external populations are not substituted for these inputs.

## Immediate serializer contract, not a universal serde allowance

Locked serde_json 1.0.151, serde_core 1.0.229; existing layout04 production compiler evidence has serde_json features default,float_roundtrip,std, without preserve_order/arbitrary_precision/raw_value. VR manifest and lock are unchanged from that source to 40129. Local immediate source hashes are recorded in BINDING.json. Neither local package has .cargo-checksum.json: local-page hashes do not independently authenticate correspondence to the locked registry package. All serializer-dependent clone/schema/capacity and sample-prefix claims below remain conditional on that source binding. No recovery/fetch or dependency audit was attempted.

serde_json/src/macros.rs:136-164,254-282 constructs object fields in source order and inserts keys/values; expression conversion at279 is to_value(&$other). Thus json!(record), json!(run.field), and json!(not_covered_full) borrow and serialize; they do not consume those input owners. Value derives Clone (value/mod.rs:115); Map clone delegates its backing BTreeMap (map.rs:395 onward). Default Map is BTreeMap (map.rs:34); its with_capacity ignores the hint (49-55). Value serialization recursively copies its strings, arrays and objects (value/ser.rs:12-36), serialize_str owns a copy (168), sequences use Vec::with_capacity(hint) (233), and each element is pushed after conversion (330 onward). serde_core Vec serialization uses collect_seq (ser/impls.rs:188-207,233-236); collect_seq uses the iterator length hint (ser/mod.rs:1296-1304), which returns Some(k) only when lower==upper (2002-2009). Actual Vec/slice record arrays therefore allocate exact 32*k slots on the bound actual Value layout. No such exact-capacity claim is made for arbitrary unknown iterators.

Define J for these finite, successful built-in conversions:
- null/bool/numeric scalar child: zero heap;
- serialized String of byte length L: E_1(L);
- known-length array k: E_32(k) + sum J(children);
- object k: N_JSON(k) + sum key byte lengths + sum J(value children).

N_JSON is the actual requested-node envelope of BTreeMap<String,serde_json::Value>, still unbound. Root Value is a stack/local value; array/node slots own inline Value, not an extra Box. This shape grammar is useful without pretending that unknown private node requests are known. Generic Map clone tree topology and allocator failure paths also remain within that named dependency.

Record grammar from RD:
- stages (21-31):19 keys.
- attempt (34-65):18 keys, including two stages objects, storage3, verification5.
- report tally (67-72):7 keys.
- case_record common root7; selected adds7; attempts/report add2: selected16, other9 keys.
- source_refused (123-129):7 keys.
- Attempts array has at most4 entries; when moved as Value::Array at118, its backing allocation is not copied.
- json!(stages(...)) first constructs a temporary stage Value then serializes its borrowed contents. Current temporary and destination stage subtree overlap. The same distinction applies to the optional verification Value and format! temporary strings.
- No published-row/radius/cache arrays are embedded in the record: only their selected metadata/counts and geometry text.
- family_file/to_string_pretty (131 onward) is outside actual vk_scale.

Let J_rec and T_recbuild be this finite record grammar and its currently active expression temporaries; name all unknown geometry/refusal text byte lengths explicitly. Around V437: run.record1 + record2 coexist. At V483: run.record1 + record2 + wrapper record3 coexist. After V485 only record2 persists. These are deep identities, not three shallow aliases.

## Source-backed caller phase roster

| Phase/source | Caller owners beyond surviving pre-cut C_pre | Kernel overlap / next edge |
|---|---|---|
| Counts V402-417 | finite counts JSON plus Phase.fields map and transient emit scratch | no current kernel; source made at383 was dropped399 |
| Counts-only V418-423 | summary partial object and emit scratch | terminal branch, no lane/RCM/parity |
| Backstop V425-430 | refusal format/JSON/emit scratch | exits3, no normal final summary |
| L182-197 controls | SourceParts already built; borrowed row lookup tree; control output strings/vectors; one exact/parse/control temporary chain | no kernel yet; row lookup tree drops on value_controls return |
| L199-237 source/hash | PrimitiveSource; encoding Vec; hash-message/output temporaries; controls | encoding persists across core; hash message drops before core |
| L237-309 selected comparison | encoding; selected map of PublishedRow clones; body output/scratch; floor map; diagnostics/control lists; one exact-number/observation/judge chain; expected-list first-use if needed | full Outcome remains alive; do not count kernel selected publication Vec as caller map |
| L324 record construction | encoding; Outcome; selected map; diagnostics; J_rec + current T_recbuild; CaseRun strings | body/scales from selected branch ended309; Outcome/encoding drop on run_parts return338 |
| V436-442 | CaseRun and record1; Phase.fields; deep record2; W1 JSON | no Outcome/radius/retained kernel after return; wrapperCaseRun is stack |
| V458-482 full floor/report | CaseRun/record1 + record2; temporary floor map then retained not_covered_full Vec<String>; report deep JSON + failures-head reference Vec | full floor Scales drops in map closure, returned NC-full Vec stays |
| V483 record emit | CaseRun/record1 + record2 + wrapperrecord3 + emit scratch + NC-full | wrapperrecord3 drops after emit; run drops485 |
| V489-493 RCM | record2 + NC-full + adjacency construction/tree/output arrays + both returned order arrays | adjacency/orders explicitly dropped492, then small RCM output JSON |
| V500-507 parity | record2 + NC-full + finite sparse parity envelope below; returned classString then output JSON | dense=false; all sparse helpers dropped before parity result returns |
| V509-512 final summary | record2 + NC-full + expected-list OnceLock + progressively constructed summary object | two peak reads occur before emit schema/write/flush |

Saved diagnostics bounds: not_covered <= R_ref; class_mismatches <= 2R_ref; input_derived_not_covered <=2R_ref; failure entries <=R_ref+4 safely cover fixed non-row branches. These are lengths; each Vec uses its actual growth capacity and each String its own byte-length descriptor. Controls have two Vec<String>, one current formatted name and exact/parse chain; no per-control all-at-once scratch. Tally is scalar-only. Observed::Unavailable(String) and Verdict::Fail(String) may overlap their cloned reason, then the retained formatted failure. These small counts do not bound string bytes.

Publication map key is QuantityId, value PublishedRow. layout04 measured row stride64 and QuantityId12; cloned row is child-free. Its BTree nodes remain a separate private consumer specialization, not a layout08 kernel-tree substitution. Floor map key String,value(f64,f64), one cloned member-name child per entry. Borrowed control lookup keys/values have no cloned String/Row children, but still allocate tree nodes.

## Hash, expected-list and formatting terms

SHA implementation VR/src/sha256.rs:17-68 copies message L at22, pads to D=64*ceil((L+9)/64), appends length8. Its h/w arrays are stack. A conservative retained byte-vector bound follows pinned growth, e.g. max(8,2D), with a separate active-old growth term; checked L+9 and rounding required. The 64-character output is collected from eight formatted 8-hex chunks while the message remains alive. Do not claim output capacity64 from length alone: retain H_hex_collect plus the currently active hex-format chunk as exact source-contract cells unless their String-collect implementation is bound. Encoding source identity and owned lane encoding are distinct; a kernel-preparation identity copy is counted in K_phi only.

Expected-list VR/src/cases.rs:311-323 uses a static OnceLock<Vec<String>>. First successful initialization reads text, parses raw JSON and constructs the typed list while those earlier temporaries can overlap. The existing file is1357 bytes and holds two 15-byte case names: exact final typed children are E_24(2)+30=78 bytes. The static wrapper is not heap. File byte length does not bound parser/read capacity or parser scratch. Its initialized78-byte owner persists into every later phase; initialization source/parse requests remain V-PARSE.

Value::Display (serde_json/src/value/mod.rs:197-256) adapts io::Write into the caller formatter and calls to_writer on the ordinary path. It does not imply an extra full serialized String. This only narrows the owner grammar. Serializer scratch, stdout/flush/error/lazy initialization requests remain V-IO; an error path can construct io::Error at WriterFormatter. No1024 stdout-buffer fact from H is imported as a VR-runtime bound.

## RCM and parity: finite supplemental obligations

VR/src/rcm.rs:13-51 builds free flags/positions, a Vec of BTreeSet<usize> adjacency sets, bounded current element-dof Vec, then output Vec<Vec<usize>>. Use actual public sizeof expressions for these headers, pinned growth for pushed arrays, exact child cardinalities; each set's requested nodes remain N_SET(d_a). A conservative transition may retain all set nodes/backing plus all completed output adjacency, explicitly as padding. This is independent of numerical-kernel RCM adjacency.

both_orders:55-59 keeps K4 order while SD computes its order. SD/lib.rs:541-549,580-593 marks before enqueue; the reviewed source07 VecDeque population/capacity recurrence applies to the fresh SD queue. SD eccentricity:598-621 also clones next_level into last_level at618 before moving next_level into current at619. During a later call, outer component and outer last-level plus inner current, old last, next and the new clone can coexist: use six G_8(c) buffers conservatively for component size c, not the K4 four-buffer envelope. Include one active-old growth term where applicable; full neighbor arrays/degrees/order/visited remain distinct. The source01 source-bound SD roster remains conditional on its helper/private capacity facts.

VR/src/parity.rs:
- binary64_model:45-102: fallible borrowed nodes G_32(N), frames G_136(m), pushed axis springs G_16(s), force8n, restraints8r, free G_8(f), prescribed16r plus sort scratch. Directional springs return at78 before later force/free construction; do not exclude the case.
- retained B64 adapter = G_136(m)+G_16(s)+8n+G_8(f)+16r; constructor adds nodes/restraints and its current scratch.
- contributions106-128: G_24(144m+s).
- allowances134-164: actual map<(usize,usize),(f64,usize)> temporary magnitudes and result transition; preserve their distinct tree identities.
- sparse assembly204-212 and round/count arrays G_8(z64) each215-224.
- solve_sparse_structural238 invokes preparation/audit/order/factor/condition/refinement/refusal helpers already named in source01. Their remaining storage facts stay V-SPARSE, rather than being assigned an arbitrary allowance.
- class String241; dense=false returns246-247 before dense assembly/factorization.
Sparse fill h_s is a descriptor/structural upper, not copied numerical-kernel h. A conservative combinatorial bound f(f+1)/2 needs checked arithmetic and source-domain validation.

## Original global sampling boundary

The ordinary final summary constructs fields in order: kind, case, repeats_heap_peak, repeats_heap_peak_move (V509-512). The map has at most4 keys, fitting one BTree leaf. At the first peak read, the allocated object prefix contains that leaf, kind key4 + value7, case key4 + copied id bytes, and current peak key17: Leaf_JSON+32+|id|. At the second read, the fourth key22 has also been created: Leaf_JSON+54+|id|. Values returned by the peak calls are inline numeric Values. These are partial-construction owners, not complete post-emit objects. Counts-only adds its11-byte key and remains one leaf. The first and second reads are distinct instants; the global counters preserve all earlier allocations/freed peaks. Final schema insertion/serialization in emit occurs after both reads and is outside those sampled instants; prior emissions are inside the global history.

The arithmetic above is conditional on the immediate macro/evaluation/layout source binding and is offered for independent checking. It does not assert node byte sizes or cover pre-cut parsing/runtime. A conservative later-phase extension may include more owners but must be labeled and cannot silently redefine the recorded metric.

## Exact residual proof cells

| Cell | Missing premise; smallest completion |
|---|---|
| V-JNODE | Actual requested leaf/internal bytes for JSON Map<String,Value>, borrowed controls map<&str,&Row>, map<QuantityId,PublishedRow>, floor map<String,(f64,f64)>, adjacency BTreeSet<usize>, parity map<(usize,usize),(f64,usize)>, and pre-cut Case maps. These are not the three measured kernel specializations. No private alignment inferred. |
| V-JBUILD | Substitute actual finite field/array/name/geometry/refusal descriptors into J and T_recbuild, including temporary-to_value overlap and every actual wrapper; private node dependency stays named. No generic future serde identities. |
| V-LIB | Local serde_json/serde_core immediate pages have exact hashes/version-path and existing feature evidence, but absent local registry checksum manifests prevent independent package-source authentication in this tranche. All dependent clone/schema/prefix claims are conditional. |
| V-EXACT | Checked decimal input digit/exponent descriptors and the finite comparison Exact arithmetic live-DAG bound. Source01 absolute observation-derived allowances are not proofs. |
| V-FMT | Actual byte maxima/capacity contracts for resolver/QuantityId/RowClass/outcome/geometry/FrameKernelError formatting and immediate float parsing; hash output String-collect/hex chunk contracts remain explicit. |
| V-PARSE | First expected-list read/parse scratch after cut; family/external-model deserialization and pre-cut peak are separate prefix obligations.1357 bytes proves only source length. |
| V-SPARSE | Source01 finite SD preparation/audit/ordering/factor/condition/refinement/intended/refusal helper private strides, capacities and coexistence. Six-level-buffer correction is a caller-source fact, not complete sparse-helper closure. |
| V-IO/RUNTIME | Immediate successful/error serializer/stdout/flush and late runtime/lazy growth within original global window. Static state alone is not a process-heap envelope; no baseline or safety multiplier used. |
| V-PRE | Actual argv/external12 identities, prelaunch model/input descriptors and overflow checks; all pre-cut owners/peaks/runtime. Conditional tV boundary cannot replace the original source-derived admission/global obligation. |

No runtime job was launched. No API, estimator, record, fixture, admission, model, source, Git/index, observer, allocator, library, toolchain or H packet was changed. All open cells remain open.
