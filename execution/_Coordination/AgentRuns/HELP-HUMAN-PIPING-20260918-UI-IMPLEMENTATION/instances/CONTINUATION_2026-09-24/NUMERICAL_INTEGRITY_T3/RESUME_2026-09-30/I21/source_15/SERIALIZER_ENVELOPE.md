# Source15 — finite vk_scale serializer envelope

TASK I21; start2026-10-01 17:13:48 UTC; deadline17:33:48 UTC. Frozen40129; sealed source11/vr_caller_07 and additive serde_binding_07/serde_binding_08 identity proof. Existing layout04 activates serde_json default,float_roundtrip,std and serde_core std, with preserve_order/arbitrary_precision/raw_value disabled. This is conditional source reasoning on that recorded feature/layout basis, not a new binary equivalence or E_max acceptance.

Three identities remain separate: record1 built in lane, record2=run.record.clone(), and serialized record3 constructed inside the record output wrapper. record2 and not_covered_full survive drop(run), RCM/parity and the final peak reads. Serialization borrows; moving a completed Value into a parent does not create another tree.

The actual emit path is Value::Display -> serde_json::to_writer with a borrowed WriterFormatter -> the supplied stdout formatter. It does not call to_string or allocate a complete serialized-line String. The family's to_string_pretty helper is outside actual vk_scale. Numeric conversion helpers and formatting/IO failure terms remain explicit where not supplied; no global serializer allowance is assigned.

## Core owner operators (finite schemas below)

Let Q=size_of::<serde_json::Value>() (the existing bound target fact is32); let L_J/I_J be the still-unbound actual leaf/internal requests for BTreeMap<String,Value>. Let T_J(k) be source02's insertion-node envelope for a new k-key map:0 at k=0, otherwise (1+floor((k-1)/5))*max(L_J,I_J). This includes the new split/root nodes that survive insertion. Exact one-leaf counts may be used for k<=11. Key String bytes and value children are separate from these node slots.

For a finite Value shape v define R(v): scalar/null/bool/number0; String of length d has d; array with k known elements has Q*k+sum R(children); object with fixed keys has T_J(k)+sum key lengths+sum R(values). Root wrappers are inline, not extra Boxes. Fresh clones copy logical String/Vec lengths, not original spare capacity.

Construction is a prefix maximum: retain already completed siblings and the parent builder while constructing the current child. A formatted source temporary also remains while to_value copies it. The envelope below names these explicit expression temporaries and moves, rather than multiplying a whole record by an unexplained factor.

## Prefix construction and fresh-copy rules

Write K_o=sum lengths of object keys; SCHEMAS.json enumerates every literal key and count. All arithmetic below must be checked in any later implementation. H is additional heap above externally retained inputs; R is the returned owner. Evaluate H separately for ordinary requests and growing-realloc move requests.

For object fields in order, a safe prefix recurrence is
  H_object = max(R_object, max_i[T_J(i)+sum_{j<=i}|key_j|
                  +sum_{j<i}R(child_j)+H(expression_i)]).
T_J(i) deliberately prepays the current insertion's eventual nodes while the child is built. No finished parent copy is added again. Keys precede value expressions in json! (macros.rs:148,159); SerializeMap holds a copied next_key while serializing the child (value/ser.rs:398-431). serde_core's default serialize_entry:1855-1862 calls key then value. Known-length array construction is
  H_array = Q*k +max_i(sum_{j<i}R(child_j)+H(child_i)),
also including its returned R. Exact borrowed map/Vec/slice size hints reserve k slots; no generic P(k) is substituted for these exact sites.

For a formatted String temporary with output length<=D and returned JSON String length<=D:
  H_format_to_value = max(H_format(D), F(D)+D),
  R_string<=D.
F(D)=max(8,2D) for these nonempty fixed templates; H_format retains its explicitly named formatting helper term, and move growth adds only the current old String request<=D. The temporary and copied JSON String are distinct owners. Option<String>::Some follows the same rule; None is Null with no child.

For a temporary Value t passed as a json! expression:
  H_temp_to_value = max(H_build(t), R(t)+H_serialize_to_value(t)).
This is why stages(...) and optional verification JSON require source and destination subtrees together. Directly moving a Value/Vec into a parent has no such copy.

Fresh Value::clone derives fieldwise clones. Number's non-arbitrary-precision N is Copy/inline (number.rs:22-35); String and Vec clone logical lengths exactly. Map::clone delegates to BTreeMap::clone (map.rs:395-400). Pinned alloc/collections/btree/map.rs:226-308 clones source leaves, promotes the cloned first subtree for an internal node, then clones its key/value and next subtree before moving them into the parent. Every allocated node/child belongs to the final cloned tree, including temporarily detached subtrees. No bulk-input Vec, stable-sort scratch or clone_from reuse is involved. Empty clone returns a new empty map even if its source retained an empty leaf. Thus a fresh-copy construction envelope is its complete destination R bound, with no active-old realloc surcharge; the external source remains separately live. Serialized record3 uses insertion-built maps in sorted source iteration order, so its node shape need not equal record2's, but the same T_J(k) upper applies.

These rules concern the actual built-in successful value conversions. Numeric fields are u64/u32/u8/usize64, bool or Option thereof. StageWork fields and work-total methods are u64; elapsed time is explicitly cast to u64. The serialized verification fields are only data_blocks, shift_factorizations, uc_missing, g_max and g_violation. Its resolution/theta/bound floats and bound_refusals are NOT serialized. Therefore no f64 JSON conversion or zmij text helper is reached by this finite grammar. The u128-out-of-range to_value error is not reached, and every actual map key is String/&str. No arbitrary Serialize implementation is being accepted.

## Retained record grammar and expression-specific overlap

Let A<=4 attempts, V<=3 present verification objects, B bodies and N nodes from the existing descriptor. Define:
  S=T_J(19)+K_stage; U=T_J(3)+K_storage;
  W=T_J(5)+K_verification; P=T_J(7)+K_tally.
For attempt j:
  R_Aj=T_J(18)+K_attempt+D_role_j+D_attempt_outcome_j
       +[gate_j]D_gate_j+2S+U+[verification_j]W.
The two19-key stages are separate retained objects; their temporary construction/copy phases occur successively. Scalar/null fields add no heap child.

With I=|case.id|, Fm=|case.family|, D_o=outcome_text length, D_g=geometry Debug length:
  R_selected=T_J(16)+K_selected+|SCHEMA_record|+I+Fm+64+D_o+D_g
             +Q*A+sum_j R_Aj+P.
  R_unresolved=T_J(9)+K_other+|SCHEMA_record|+I+Fm+64+D_o
               +Q*A+sum_j R_Aj+P.
  R_refused is the latter with A=0.
  R_source_refused=T_J(7)+K_source_refused+|SCHEMA_record|+I+Fm+64+D_sr+P.
The schema literal is exactly its source byte length in SCHEMAS; no guessed fixed String allowance. Empty attempts has no allocation.

Construction sites:
- RD35-65: role/outcome/gate formatted temporaries coexist with their copied String; each stages(...) temporarily holds S beside the copied S; optional verification Some temporarily holds W beside copied W. Nested storage literal is built once and moved, not serialized from a temporary Value.
- RD91-117: root common7 (selected14 after its extra fields) remains while the exact A-slot attempts Vec and completed attempts coexist with current H_Aj. Its parent key/node may be prepaid as above, but no second root copy is created.
- RD118 moves that Vec through Value::Array into root. RD119 directly moves report(t), so only one tally subtree P is needed for that current child.
- RD123-129 source_refused_record uses report(t) INSIDE json!, so its expression instead has temporary P plus copied P. Its input error String is a caller owner (the lane's format argument), separate from the newly formatted SourceRefused String and its serialized copy.
- outcome_text's Refused branch first creates refusal_text; this temporary remains during outer formatting, then drops before the completed outcome String is borrowed by to_value. Use max(F(D_refusal_text)+H_format(D_o), F(D_o)+D_o), not an unexplained full-record multiplier.

At A=4,V=3, a selected record has at most21 objects,274 key/value entries and one4-element Value array. No published rows, certificates, radius vectors, body arrays or saved verification float arrays are embedded. These kernel/caller owners remain external in source11's phase union.

## Finite text grammar and explicit formatting cells

Default non-pretty derived schemas give the following byte uppers (SCHEMAS.json preserves the literal/integer arithmetic):
- AttemptRole25 (VerificationThenCandidate).
- QuantityId61, using the reviewed Dof39, u32 ten digits, usize twenty digits and two-character Component names.
- AttemptStop<=88: the PublicationCertificate variant is the same bounded index/issue grammar already reviewed in source10; every other listed Stop variant is shorter.
- AttemptReason<=161 (PublicationEnclosure with QuantityId/body/Kind/PublicationPredicate); AttemptOutcome<=171 including Rejected(...).
- GateTest<=38 (two u8 fields with three digits); selected outcome_text<=16 (Selected at1024).
- BodyGeometry is Restrained, NumericallyUnresolved, or NotAssessed(Vec<(u32,SpringKind)>). Each tuple<=25 bytes, plus at most2 separator bytes. There is at most one entry per node and each of Translation/Rotation (factor.rs:145-196), hence total tuple count<=min(d,2N), where d is the existing directional-spring input count; every such group must contain at least one directional spring. Outer geometry list length is bounded by D_g<=2+23B+27*min(d,2N).
- refusal_text strips the six-float mechanism witness to its body. RV30 N1/source10's other refusal grammar gives D_refusal_text<=89. Thus Refused text<=98+D_g and Unresolved text<=100+D_g; use100+D_g across non-selected outcomes. No six-float witness is serialized here.
- SourceRefused output<=14+D_SourceError, with the previously reviewed D_SourceError<=75 (13 letters plus one space).

These bound output LENGTHS. Formatting helper scratch/active-old terms remain separately named where the existing primitive/derived formatter warrant has not been carried through; no unproved zero is inserted into H_format. Original geometry/attempt input Vecs remain kernel-owned while formatted text is produced.

Report input String lengths are tied to actual finite producers:
- row keys, expected decimal text, committed not-covered keys, case/control IDs and hash64 are prelaunch input descriptors.
- control-list strings have length I+1+|control.id|; their two lists together contain<=C entries.
- selected row failure text has length I+|row.key|+|row.expected|+6+D_why, where D_why is max of literal predicate/structural-zero messages, QuantityId61 plus literal overflowed/not-published suffix, and unresolved-key prefix plus actual key length.
- source/meter/hash/expected-outcome/multibody/terminal failures use the actual finite lane templates and the same SourceError/outcome/ID/count bounds. Preserve the maximum over those named templates, rather than inferring text size from failure count.
- class mismatch length is I+|row.key|+literal punctuation+61+D_OptionRowClass. Its exact reached RowClass Debug helper/length substitution remains a V-FMT leaf.
- parity success class/error text remains its named source11 V-SPARSE/V-FMT String bound. No sparse-helper audit occurs here.

## Actual outer payload cardinalities and builders

SCHEMAS.json records key names/key-byte sums. The base counts below precede merge's4 Phase fields and emit's1 schema field.

| Payload | Base object keys | Added array/string ownership |
|---|---:|---|
| start |8| kind5,case I,optional two hash64 copies |
| counts |23| kind6,case I,k4src hash64 |
| w1 |10| kind2,case I,copied record2 outcome text; other extracted fields scalar/null |
| report |21| kind6,case I and the arrays listed next |
| record wrapper |3| kind6,case I,serialized record3 |
| rcm |3| kind3,case I |
| binary64 success/error |3| kind8,case I,copied class/error String |
| summary |4| kind7,case I; peak values scalar |
| counts-only summary |5| same plus bool |
| half-cap refusal |5| kind7,case I,reason literal |
| no-op start/summary |3 each| finite literals; separate no-comparison branch |
| fail |2| supplied kind/reason String; pre-cut/error length remains source-named |

Report arrays are known-length borrowed serialization, so each output backing is Q*k with copied String bytes:
  failures_head h=min(50,F), F<=R+4;
  not_covered_rows <=R;
  not_covered_committed = actual input Nc;
  not_covered_rows_s_full = None/Null or <=R;
  class_mismatches <=2R;
  controls_undiscriminated + controls_unexpectedly_failing <=C.
Their retained String sums use each actual bounded producer above; no arbitrary per-entry allowance. The temporary failures_head Vec<&String> costs h*size_of::<&String>() and survives while its Q*h output Vec/String children are constructed. That pointer Vec is not a String clone. The source diagnostic/control lists remain in CaseRun, and not_covered_full remains an independent outer local. The input_derived_not_covered list remains a caller owner but is not serialized in this report.

merge(a,b) moves the four Phase entries into a. Map::extend -> BTreeMap::extend is a for_each(insert), not FromIterator bulk collection (std map.rs:2558-2564). A safe merge peak is:
  T_J(k+4)+T_J(4)+K_a+K_phase+children(a).
This retains b's source nodes throughout consumption and completed destination nodes together, but each moved key/value child is counted only once. No Vec/sort term is introduced for this merge. Conversely the separate source11 control/floor/Case bulk-map constructors keep their inherited TB/Vec/sort terms; this serializer proof does not erase them.

emit adds schema via mutable indexing (value/index.rs:96-108). For each actual outer payload the root has no schema yet; RHS json!(SCHEMA) creates the11-byte value, then indexing creates the6-byte key and may grow the map by insertion. The finished augmented object/prefix recurrence covers both. Record1's schema is nested, not a conflicting outer key. Read-only record indexing in w1 uses static Null for absent fields (index.rs:212-215), with no allocation or missing-index panic.

## Clone/serialization phases and unchanged sample edges

1. Lane constructs record1 while Outcome/encoding/caller diagnostics remain. Use H_record from the exact field/array recurrences, not just R_record.
2. V436 builds Phase fields, then V437 clones record2. record1 + completed Phase map + H_clone(record2) coexist. Both records persist after that output.
3. V458-482 builds not_covered_full, then report JSON from borrowed lists. Its new arrays/strings are distinct; caller lists are not moved.
4. V483 constructs the record wrapper and serialized record3 from borrowed record2. record1 + record2 + wrapper(record3) coexist. record3 is a Value tree, not a serialized-byte String. The wrapper drops after emit returns.
5. V485 drops run/record1 and its lists/map. record2 and not_covered_full persist through RCM, sparse parity and final summary. No kernel Outcome survives lane return merely because record2 does.
6. Final summary's peak and peak_move calls occur while the object is partially built. Preserve source11's one-leaf prefixes: first Leaf_J+32+I, second Leaf_J+54+I. Counts-only adds its11-byte key. These are distinct sampling instants. Final emit/schema/stringification occurs later; earlier emissions remain in the original global history.

The separate no-op summary omits case: its corresponding prefixes are Leaf_J+28 and Leaf_J+50. This finite syntax branch does not replace the normal RF-LARGE comparison window or supply a measured baseline.

No max over these phases is replaced by adding three complete historical records after their drops. The H staged/outer-prefix metric and VR global metric remain separate.

## Actual stringify route and residuals

Value Display (value/mod.rs:197-256) creates a stack WriterFormatter and calls ordinary to_writer; ser.rs:17-35/2177-2185 holds borrowed writer and CompactFormatter. Recursive Compound map/array states are stack/borrowed state, not a second Value tree. format_escaped_str:2069-2132 scans borrowed UTF-8 slices; escapes are fixed stack bytes (1757-1796). A string of d input bytes emits at most2+6d bytes; fixed ASCII keys emit exactly2+key length. Scalar JSON text has length<=20 for reached unsigned numbers,<=5 bool and4 null. Hence finite output byte lengths follow:
  Text(array)=2+sum Text(child)+max(0,k-1),
  Text(object)=2+sum(2+keylen+1+Text(value))+max(0,k-1).
These lengths do NOT imply an allocated byte buffer.

The reached integer leaf is itoa::Buffer::new/format in ser.rs:1655-1663. Its final local source/registered-scratch contract remains T_itoa/O_itoa unless separately bound; no other package audit was started. zmij/f64, pretty-printing, arbitrary precision and raw-value routes are not reached in this finite grammar. Successful stringify adds no complete output String; retain the exact unproved integer/writer/IO helper terms rather than guessing0.

WriterFormatter's fmt-error conversion calls io::Error::new, and serializer writes map errors through Error::io. H's separately reviewed dropped-byte-write result does not automatically close this wrapped formatting-error route. Parsing, error/panic/runtime behavior and prior source11 global-prefix obligations remain their named cells.

For insertion into the source11 union, use H_emit(v)=max(H_schema_add(v), R_augmented(v)+H_stream). H_stream is the maximum over the reached numeric-fragment, literal/string-fragment and wrapped-error phases; numeric writes can retain their itoa helper while the writer/error helper runs. Keep T_itoa plus the current writer/error owner terms at that overlap, with their active-old requests separately. These unclosed helper terms are precise leaf obligations, not freely chosen estimator constants. Persistent stdout/runtime owners are paid by source11 outside this additional term.

Seven private consumer specializations remain symbolic: JSON map<String,Value>; borrowed controls map<&str,&Row>; publication map<QuantityId,PublishedRow>; floor map<String,(f64,f64)>; adjacency BTreeSet<usize> with its actual private marker; parity map<(usize,usize),(f64,usize)>; Case map<String,String>. Only the first appears in the new J formulas. No kernel-node size, mirror, sampled peak,612 allowance or safety factor substitutes for any of them.

Remaining minimum V-JBUILD integration cells are the symbolic private JSON requests, named dynamic formatting leaves/helper terms, reached itoa/stream-error terms, checked substitution of actual input lengths/counts and final source/features/target correspondence. The finite builder/clone/temporary identity recurrences above are proposed for RV30 review; no full E_max, admission, W1/F2a or implementation acceptance follows. No additional tranche is implied.

An optional coarse retained selected-record substitution from SCHEMAS is
  R_selected <=60*M_J+4*size_of::<Value>()+I+Fm+D_g+3808.
The constant is the actual fixed key-byte sum plus bounded role/outcome/gate/schema/hash/selected-outcome String children; it is not an allocator allowance. M_J remains private/unbound. Construction temporaries still require the prefix equations and are not absorbed by this retained formula.
