# FORMAT/STREAM17 — bounded leaf return

TASK I21; actual start2026-10-01 18:54:39 UTC; deadline19:14:39 UTC. Frozen40129, source15/serializer15/K0 and reviewed H source facts remain the basis. Only named leaves are assigned; I23 owns consumer-node layouts and HELPS_HUMANS owns the pre-cut union.

Current concrete distinctions:

- RowClass contains only fixed variants and one u64 bound_bits field. Its general Option Debug is finite; the actual class-mismatch branch excludes AbsoluteVerified and Unpublishable before formatting, making that reached Option smaller. It introduces no floating formatter leaf.
- sha256_hex returns a String collected from exactly eight formatted u32 hexadecimal chunks. Its message Vec stays alive during collection; the collect result and current chunk must be counted separately until the first chunk moves into the result or later chunks drop.
- itoa1.0.18's reached u64 Buffer is fixed stack storage plus a static digit table; its exact source/lock/cache authentication is being bound. No output String is allocated by Buffer::format itself.
- Wrapped writer failure is not the already closed raw-byte result: WriterFormatter creates io::Error::new(Other,"fmt error"), which owns a9-byte String plus a private StringError box and private io::Custom box. serde_json::Error::io then adds its private ErrorImpl box. These are moved owners, not recursively cloned per JSON depth. Exact private request sizes remain explicit until bound; no guessed24/40-byte substitution.

The table below keeps these request symbols, the actual error/normal branches and argument/outer-format lifetimes distinct. No whole-line serialized String, successful-I/O-only assumption, panic/library expansion or acceptance is implied.

## Scope and operators

This is an additive source result for the exact reached40129 paths, subject to independent review/final build correspondence. F(D)=max(8,2D) is the existing nonempty fixed-template formatted-String request upper; a currently growing String adds at most D old bytes in the move counter. Fresh str/String clones request logical length. No whole-line JSON buffer is introduced. Original caller/kernel/Value-tree owners remain external to these additional leaf terms.

All primitive/default non-pretty formatter results are reused under their original width/precision and built-in-type premises. Borrowed str/enum/integer/default Debug output writes into the supplied destination; its scratch is stack/static. The only new specified width is the hash's fixed u32 LowerHex width8, bound directly below. Fatal allocator failure, arbitrary custom formatting, general panic/unwind and unexpected backend failures are not declared allocation-free.

## 1. Dynamic diagnostics and Option<RowClass>

K/adaptive.rs:388-398 defines RelativeVerified, AbsoluteVerified{bound_bits:u64}, InputDerived and Unpublishable. General default Option<&RowClass> length<=59 (Some(AbsoluteVerified { bound_bits: <20digits> })); no f64 field exists. At the actual L286-303 mismatch arm, AbsoluteVerified and Unpublishable have already matched their earlier arms. The formatted other can only be None, Some(RelativeVerified), or Some(InputDerived) when not restrained. Thus the reached maximum is22 bytes. Borrowed reference formatting does not add an ampersand or clone the class.

Let I=case-id byte length, K=row-key length, E=expected-text length, Cid=control-id length. The reviewed QuantityId bound is61. Exact reached length uppers:
- class mismatch L299: I+K+61+22+9 = I+K+92;
- input-derived-not-covered key L296: I+K+1;
- control diagnostic name L67/355+: I+Cid+1;
- value_of overflow / missing L85-86:61+11=72 /61+14=75;
- unresolved key cases.rs400: K+18 (literal arithmetic is recorded separately);
- why from judge is bounded by max(9,40,75,K+18): predicate, structural-zero mismatch, unavailable, unresolved-key route.
- final selected-row failure L305: I+K+E+6+D_why.

F of each length bounds its destination and D bounds its active-old grow. Existing input strings are borrowed, not cloned merely by formatting. SourceError<=75 and the source15 finite outcome/geometry grammar remain reused. Other finite failure templates are recorded in ARITHMETIC.json as exact literal-plus-I/K/E/D_outcome/20-digit formulas: source refusal, hash mismatch, exhausted meter, expected-outcome mismatch, mechanism row count, multibody and terminal not-selected. No failure-count-times-arbitrary-text allowance is substituted.

L207's temporary Verdict::Fail source-refusal String (length16+75=91) dies at that statement before the next stored failure String. L317's case-label tally String is likewise temporary per row, not accumulated as an extra diagnostic list.

The magnitude observation tuple evaluates both value_of calls before matching. Two raw unavailable Strings can therefore overlap: additional requested<=2F(75)=300, move<=300+75=375. After match, only one moves into Observed::Unavailable. On a covered-row judge failure, its clone may coexist with that source: F(75)+75<=225. The observe temporary then drops at the let-v statement; later outer failure formatting holds the Verdict reason plus the new failure String. Use the maximum of these phases, not a triple-copy history. On !covered, holds returns None/false for Unavailable and no cloned Fail reason is made. Source11's retained maps, vectors, E/S and earlier diagnostics are unchanged.

When diagnostic text calls records::outcome_text as an argument, retain the full nested phase sequence:
  H_outcome_refused=max(H_refusal_text,
                        F_refusal+H_outer_outcome_format,
                        F_outer+D_outer) for a later to_value copy;
  H_diagnostic=max(H_outcome_argument,
                   F_outcome+H_diagnostic_format).
The copied JSON String alternative belongs only to the to_value caller; an ordinary diagnostic formatter borrows the outcome String instead. No premature argument drop is assumed.

## 2. Hash output collection

VR/src/sha256.rs:17-68 holds padded message while returning
h.iter().map(|x|format!("{x:08x}")).collect(). h has exactly8 u32 entries.
Core/fmt/num.rs:9-50,68,75 formats u32 LowerHex using a fixed digit array;
Formatter::pad_integral writes sign/prefix/padding into the existing String.
For format08x, no alternate prefix/sign and exactly8 lowercase ASCII bytes
result. The digit/padding helpers introduce0 registered scratch/old requests.
Use the existing F(8)=16 destination upper; do not multiply8 chunks as live.

Installed alloc/string.rs:2409-2423 FromIterator<String> takes the first String
as the result and extends it; :2566-2569 calls push_str on each later temporary.
Thus first-chunk storage MOVES, while one later chunk can coexist with result
growth. There is no Vec<String> of all8 chunks and no copy of the first buffer
into a separate initially empty result. With initial capacity<=16 and final
length64, source02's arbitrary-c0 growth bound gives result capacity<=128.
One current chunk adds<=16. Result growth has active old<=64; chunk growth's
old<=8 is a separate allocator event:
  returned hash request<=128;
  H_join_R<=144;
  H_join_M<=208.
These are conservative source bounds, not observed capacities or an exact64
capacity claim.

For input length L, padded message length D=64*ceil((L+9)/64). Preserve source11
message capacity M<=max(8,2D) and one message-growth old<=D. Then, excluding
the caller's borrowed input/encoding owner:
  H_hash_R<=M+144;
  H_hash_M<=max(M+D,M+208).
Message growth and joined-output growth are sequential; no simultaneous old
message and old result surcharge. Message drops on return; only the hash String
survives. The caller's encoding temporary/retained vector is still separate.
All length/rounding arithmetic in a future estimate must be checked.

## 3. Reached itoa u64 helper

The existing itoa1.0.18 .crate SHA256 matches the exact frozen lock checksum
8f42a60cbdf9a97f5d2305f08a87dc4e09308d1276d28c869c684d7777685682.
Only the cited Cargo.toml/src/lib.rs members were compared with installed bytes.
No archive was extracted to source and no other package was audited. Existing
layout04 compiler line13 binds release/aarch64 and no enabled no-panic feature;
that remains historical build scope.

lib.rs:71-121 defines Buffer as a fixed MaybeUninit byte array and format as a
borrow into it. u64's sealed implementation :143-166 and macro instantiation:194
uses its20-byte destination slice; Unsigned::fmt:264-327 and u64 instantiation:332
use scalar division, the static200-byte digit-pair table and that supplied buffer.
No String/Vec/Box, allocator, callback or u128 helper is reached. Constants999
and10000 fit u64; those conversion expects are not runtime error branches for
this type. The result is at most20 decimal bytes and remains borrowed while
writer.write_all runs.
  T_itoa=0; O_itoa=0 registered bytes.
The stack buffer may overlap writer error handling, but it contributes no
registered heap. No assertion about unrelated integer/custom types or future
package/build correspondence is inferred.

## 4. Wrapped stream failure: concrete owners, precise private requests

The actual chain is V::emit's writeln!(StdoutLock,"{line}") ->
Write::write_fmt/default_write_fmt -> Value::Display ->
serde_json::to_writer -> WriterFormatter -> outer fmt Adapter -> initialized
StdoutLock::write_all. StdoutLock has no write_fmt override (stdio.rs846-866),
so io/mod.rs1973-1978 selects default_write_fmt for the dynamic Value argument.

The normal branch streams borrowed fragments/numbers through the already
reviewed initialized byte-write path. Compact JSON states/escape bytes and
itoa are stack/static; no full serialized String and no new successful-write
helper heap exists here. Persistent stdout/runtime owners are paid elsewhere.

The handled error branch is not silently excluded:
1. io/mod.rs609-618 stores the underlying raw OS/static WriteZero error inline
   in its Adapter.error and returns fmt::Error. Source12's direct raw error
   representation has no registered child on this actual path.
2. WriterFormatter (value/mod.rs228-244) maps that fmt error to
   io::Error::new(Other,"fmt error").
3. alloc/boxed/convert.rs638-640 first copies the literal to a9-byte String;
   :577-596 moves it into Box<StringError>, a private local type.
4. std/io/error.rs255-261,292-294 moves that boxed error into Box<Custom>;
   Custom's definition:193-201 has repr(align(4)), not a proved public request.
   The tagged io::Error wrapper adds no separate allocation.
5. serde_json error.rs326-335 moves the io::Error into Box<ErrorImpl>.
   ErrorImpl:230-234 contains ErrorCode,line,column; only ErrorCode::Io is
   created here. Message(Box<str>) is a different, unreached variant.
6. That one serde error moves up the recursive serializer returns. There is no
   newly allocated ErrorImpl per JSON nesting level. Value::Display's
   map_err(|_|fmt::Error) drops the error and its nested owners before returning.
7. default_write_fmt:624-627 returns the already stored raw error. V drops the
   writeln Result and then its raw flush Result. A failure of the final newline
   or flush has only the direct raw-error route, not the Value wrapper chain.

Name actual requested bytes S_StringError, S_IOCustom and S_JSONErrorImpl.
The concrete additional wrapped-error envelope is
  E_wrap =9+S_StringError+S_IOCustom+S_JSONErrorImpl.
All four owners coexist when the outer Box is built; ownership is nested/moved,
not cloned. These fresh exact requests do not grow:
  O_wrap=0; H_stream_R<=E_wrap; H_stream_M<=E_wrap
above the unchanged Value tree and persistent writer/runtime owners, for this
normal-or-handled-I/O route and successful construction of its error owners.
The numeric formatter's stack buffer overlaps but adds0 registered bytes.
Subsequent emissions take a maximum; prior dropped error chains are not summed.

The three private request sizes remain UNBOUND. No sizeof(public Error),
fat-pointer width, field-size sum or mirrored private struct is substituted.
They are separate from I23's seven BTree specializations; this return does not
dispatch them to I23 or commission a witness.

default_write_fmt:628-633 has a panic if a formatter fails with no underlying
I/O error. For the actual built-in source15 Value grammar, semantic number/key
errors are excluded and writes are its only fallible operation; therefore the
traced handled error implies Adapter.error is Err. This statement is not a
general guarantee for arbitrary custom Serialize/Display implementations.
Allocator failure during error-box creation, unexpected backend fatal panic
or violation of the pinned formatter/source premises stops at that interface;
general panic/unwind/abort allocation remains outside this tranche. No
successful-I/O assumption is used to erase E_wrap.

## 5. Parity lengths/helper interface and finite residuals

parity.rs168-180 class() does not Debug-print entire StructuralError:
- Ok quality is Passed/Sensitive, length<=9;
- Asymmetric,NegativeEnergy,Mechanism are fixed10/14/9-byte labels;
- InvalidInput(w) length14+|w|; Range(w) length7+|w|;
- NumericallyUnresolved(reason) length23+|reason|.
The returned formatted String uses F(D), active old<=D; fixed .into labels
clone literal length exactly. direction Vecs and energy/skew floats stay in
the caller outcome; class does not stringify or clone them. Primitive/borrowed
string/enum helpers add0 registered scratch under the reused default proofs.

Early binary64_model/assembly errors (parity.rs45-80,204-212) Debug-format
FrameKernelError. Its enum (FK/lib.rs319-377) has bounded usize/f64 fields and
static name/detail strings, never an owned direction Vec. Default f64 Debug
can reuse the reviewed stack-only helper and finite32788-byte conservative
bound; NonFinite's NaN/inf branches are<=4 and covered by that same maximum.
For a static name/detail payload of byte length M, quoted Debug length is
bounded by2+6M. Thus its finite type-grammar envelope is the max of the actual
variant-name/field punctuation, bounded20-digit usize fields,32788 float and
quoted static payload. The exact formula/variant terms are in ARITHMETIC.

Two precise length-premise gaps remain: P_STATIC_STRUCTURAL is a bound on the
actual &'static str w/reason payloads returned by the selected
prepare/factor/finish_sparse_structural chain; P_STATIC_FRAME is the bound on
name/detail literals reaching FrameNode/Section/Element and sparse assembly.
Some directly read examples are bound literals, but this packet does not
pretend that observing those examples proves the complete routed set. Static
lifetime alone does not impose a byte-length ceiling. A complete finite
message-pool/call-route binding remains necessary before numeric substitution.
No unrelated function tree or general error/panic audit was started to fill it.

At the consumer edge W+Solution/error remains while class formatting runs.
At parity return those solver owners drop; only the resulting class/early-error
String survives into its JSON copy. Preserve source15's old String plus new
logical-byte JSON child at that conversion. Do not retain W throughout later
emit or omit the source class String while it is copied.

## Disposition

Closed here within stated source/build premises: reached diagnostic/RowClass
lengths and their primitive helpers; hash collection capacity/overlap; itoa u64
heap0; normal streamed helper heap0; exact wrapped-error owner topology and its
move behavior. Numeric wrapped-error bytes remain conditional on three exact
private requests. Parity message-pool lengths remain two named source-routing
premises. Fatal/allocation-failure/general panic behavior stops at the explicit
interface. These gaps are not freely chosen overhead parameters.

Keep refusal argument construction and every original measurement/sample edge.
No work duplicates seven-node binding or pre-cut union, alters source14/R7
numeric consequences, or changes admission/records/API. No Rust/runtime/probe/
model/test/solver, library/dependency/tool change, maintained/Git/index write or
delegation occurred. No experiment is running; independent review and full
E_max/acceptance remain separate.
