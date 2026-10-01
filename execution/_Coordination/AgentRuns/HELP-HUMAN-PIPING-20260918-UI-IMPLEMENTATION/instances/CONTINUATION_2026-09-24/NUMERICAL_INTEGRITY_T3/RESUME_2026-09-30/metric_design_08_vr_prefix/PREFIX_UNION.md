# VR-PREFIX — actual pre-cut allocation and survivor union

**Finite source union with named residuals; no complete E_max or admission
acceptance.** Frozen product40129a225d73860ac2a53da9a2fa73869df668f3.
The final independent k0_assembly_16 RETURN was read before relying on its
integration. Its checkpoint remains open. This addendum preserves VR's global
history and the original before-launch admission rule; no observed prefix,
no-op value,612 allowance or altered fixture roster is substituted.

V=VR/examples/vk_scale.rs; C=VR/src/cases.rs; Q=VR/src/scale.rs;
A=VR/src/rcm.rs. The cut is after V399 drop(source) and V400 sizes, before V401
estimate/counts output. Existing source15/serializer_15 owns output construction,
stream/format leaves; existing node work owns private request parameters.
Raw input/source/package/command provenance is under _run_records.

## Fixed inputs and reached parser

The actual binary calls load_family("RF-LARGE"), not load_all. The fixed family
has24 lines/cases;12 carry embedded models and12 name external models. The exact
family file is986182 bytes, SHA16357afa5efeaaac3ab5798bb6f632104bec1e2dfada288f5b0936bfe6188759.
All12 existing external raw files were rehashed against the sealed I23 identity
set and checked unchanged during reading. No replacement model/graph/encoding
was generated. INPUT_GRAMMAR records every document's object/array histograms,
String/key byte sums, depth, integer tokens and identities.

Metadata findings for these exact bytes: no duplicate object keys; no escaped
String tokens; no floating-number tokens; integer tokens are nonnegative and fit
u64; valid UTF-8 and shallow container nesting. Reference decimals and binary64
hex values are **JSON Strings**, not floating JSON tokens. This selects a smaller
actual parse route without excluding any of the24 cases or the other generic
lane obligations; changing input bytes invalidates this grammar binding.

Bound serde_json1.0.151 StrRead borrows the input (read.rs668-714).
Deserializer owns an initially empty scratch Vec and stack depth counter
(de.rs28-64); from_trait constructs it once, deserializes Value, checks trailing
input, then drops it (2507-2517). On these unescaped strings parse_str_bytes
returns borrowed slices without copying into scratch (read493-529). All numbers
use the u64 integer path, so the decimal/long-integer scratch path is not reached
(de462-521,933-935). Thus **successful parser scratch Vec allocation is0** for
these documents; the resulting owned Value tree is emphatically not0.

ValueVisitor strings and keys copy their logical UTF-8 bytes into owned Strings;
borrowed-str visitor forwarding and StringVisitor add no second copy. Arrays
start Vec::new and push each child (value/de.rs112-122); objects insert into the
actual Map tree after the current key/value are built (135-147). SeqAccess and
MapAccess borrow the deserializer and carry scalar state (de1919-2039).

Let N_J(k) be the existing insertion-node upper for Map<String,Value>.
For a fixed input document v define:

```
Jin(v)= sum_objects N_J(entry_count)
       +sum_arrays G(Value,element_count)
       +sum_utf8_bytes(all keys and String values)
HJin(v)=Jin(v)+epsilon*max_arrays O_G(Value,element_count)
```

Here G/P, exact E, generic A and active-old O are K0/source02 operators;
epsilon=0 ordinary requested bytes,1 growing-realloc move bytes. Complete-tree
padding counts the current unattached child/key and completed siblings once;
only one growing array's old backing is added. Unique keys prevent overwritten
values requiring extra discarded-child history. Numbers/bools/null are inline.
**Do not reuse source15's exact-length serialized-array E rule here:** the parser
uses pushed G capacities. Conversely typed Case/Model array collection and later
fresh clones use their own constructors. This source argument is tied to the
reviewed no-arbitrary_precision/no-raw_value/BTreeMap feature basis and final
build correspondence. No general malformed-JSON or floating parser bound is made.

## Launch arguments, paths and file reads

Let a be argc including executable, L_j the actual UTF-8 argument byte lengths,
I the selected case-id length and P the optional model-file argument length.
Unix sys/args uses Vec::with_capacity(argc), cloning each C-string into a Vec<u8>
then moving it into OsString (sys/args/unix21-61). Args wraps IntoIter
(sys/args/common6-31). env::Args::next moves the OsString into String; on the
selected Unix bytes implementation successful UTF-8 conversion keeps its Vec.
Therefore:

```
Hargv <= E(OsString,a)+sum_j L_j
ArgsRet=I+[model-file present]*P
```

The vector backing and unconsumed arguments coexist with those already moved
into Args; the total above pays each buffer once. The iterator/backing and unused
argument buffers drop at args() return. V341 then creates a separate id clone I.
The cap is installed at V332 **after** args; a later cap is not a prefix proof.
Bind actual executable/archive/cwd/argv bytes and valid UTF-8, including skipped
argv[0]; the planned relative strings are not arbitrary future absolute paths.

Family path construction is CARGO_MANIFEST_DIR -> join("cases") -> join(family
filename), C33-38/298. On Unix the relative joins clone the input bytes then
append a separator/component (path3083-3086,1414-1419; os_str/bytes120-126).
Use actual lengths L0,L1,L2, with exact initial clone and source02 growth.
A safe padded family-path retained upper is L0+A_1(L1)+A_1(L2), with only the
currently growing buffer's old request (<=max(L1,L2)) added. This deliberately
permits intermediate path owners through load_file; they are gone before tV.
The external Path::new(a.model_file) is borrowed and creates no PathBuf copy.

File open converts a borrowed path through run_path_with_cstr. On this target
L<384 uses a stack array; L>=384 calls CString::new(&[u8]), which reserves exactly
L+1, copies L, adds NUL and converts the same full Vec to Box<[u8]>
(sys/helpers/small_c_string17-59; alloc/ffi/c_str274-298,345-348).
Let Cpath(L)=0 below384, otherwise L+1. It dies when File::open returns, **before**
the file String is reserved/read. This is a source branch, not a path-length
cutoff on the domain.

For a stable regular input with F actual bytes and metadata hint F or unavailable:
std/fs383-390 starts an empty String, reserves exact hint, then calls
io::default_read_to_string. That appends into the same byte Vec (io384-398,
522-536). default_read_to_end410-519 uses a32-byte **stack** probe and reserves32
additional slots only when full. No request need exceeds F+32; a growing old
buffer has capacity<=F. Thus a conservative source request bound is:

```
ReadRet(F)=0 if F=0, otherwise max(8,2*(F+32))
HRead(F,L)=max(Cpath(L), ReadRet(F)+epsilon*F)
```

With accurate metadata F the initial exact F capacity and stack EOF probe avoid
that over-allocation; the looser formula also covers unavailable metadata.
The max-read-size heuristic does not allocate a separate read buffer. File's
read/read_buf forwards borrowed storage to libc::read; metadata uses stack stat;
File/FileDesc are scalar wrappers. No second full byte buffer is created by
String validation. This read-buffer formula assumes successful allocation;
try_reserve failure and its conversion to io::Error remain a named reached
reader-error edge for the error/format owner, not an asserted zero. Registered
read/open OS-error payloads use the previously
bound raw OS/static error representation; caller panic/Display formatting stays
an explicit FORMAT/STREAM interface. Fixed-file identity must hold throughout
the actual read: hashes recorded today do not prove a future file cannot change.
A mismatched or growing future file is not silently bounded by its old F.

## Raw-to-typed Case/Model lifetimes

Use exact public sizeof expressions and the existing source09 typed descriptors.
RModelParse comprises pushed node-name and coordinate Vecs; exact borrowed-map
member/spring/omitted/constraint/load/station Vecs; and one exact copy of each
retained typed String child (C163-220). Its construction extra is the maximum
active-old growth of the two pushed node arrays. Numeric bit conversions are
scalar; no source/model algorithm is executed to establish these metadata counts.
RModelClone uses exact logical-length Vec/String clones, not source spare capacity.

RCase(i) is its child heap: id/family/basis/units/K4SRC/optional model-hash Strings,
exact rows/controls/not-covered arrays and their String children, value-control
Vec<(String,String)> children or outcome defect Strings, optional RModelParse,
and scales/s_full BTreeMap<String,String> owners. Those maps are built by
FromIterator (C256-259), so **keep the source02 input-pair Vec, stable-sort and
bulk-tree construction alternatives**, not just retained nodes. Let HCase(i)
be that complete typed construction above a borrowed raw v; its extra terms are
those map input/sort/construction and one active pushed-node grow. Node requests
stay with the existing Case String-map specialization. String fields are fresh
copies while their original raw Value Strings still exist.

load_file reads the **whole** family String, then parses one borrowed line at a
time. During case i, previous typed cases remain in the growing result Vec,
raw v_i remains throughout typed construction, and the input family String
remains through collect. A precise finite form (M=24) is:

```
HFamily = Paths + max(HRead(Ffamily,L2),
  ReadRet(Ffamily)+G(Case,M)
   +max_i( sum_{j<i} RCase(j)
            +max(HJin(v_i), Jin(v_i)+HCase(i),
                 RCase(i)+epsilon*O_G(Case,M)) ))
FamilyRet=G(Case,M)+sum_i RCase(i)
```

The final term includes the current completed case beside an outer Vec growth.
FamilyRet itself is covered at the last iteration. After collection the file
String drops; IntoIter::find then moves the selected Case, dropping other cases
and the family vector backing. The selected Case's children survive; its old
family container does not. This earlier whole-family peak cannot be recovered
merely by summing cut survivors.

For an embedded case, a separate exact-length Model clone is made at V351 while
Case.model retains its original parse-built Model. For an external case,
load_large_model's text and raw v remain alive while `parse_model(&v)` completes
and while the **second tuple operand** hashes the text (C331-334). Define HHash(F)
from the existing SHA source: a byte clone grows to D=64*ceil((F+9)/64), with
message retained<=max(F,8,2D), active old<=D; h/w arrays are stack. Keep the
separately owned I21 hex-collection/format peak HHex and returned RHex explicit:

```
HHash(F)=max(Msg(F)+epsilon*D, Msg(F)+HHex)
HExternal=max(HRead(F,P), ReadRet(F)+HJin(v),
              ReadRet(F)+Jin(v)+HModelParse,
              ReadRet(F)+Jin(v)+RModelParse+HHash(F))
ExternalRet=RModelParse+RHex
```

This retains text, raw Value and typed Model together during hashing. After
return only typed Model and hash String survive; hash's logical64 characters
do not license an exact64-byte retained capacity without the separate HHex rule.

## Initial output and counts helper union

Let BaseBeforeStart=ArgsRet+I+RCase(selected)+RModelStandalone+optional RHex_model.
Add the declared runtime survivors separately, once. The initial start payload
copies borrowed id/hash Strings through json!, then phase.fields and merge/emit
run at V371-379. Use source15's construction/merge/stream envelopes with these
actual field values, including first stdout initialization and the Phase-prefix
sample boundaries. The emitted trees drop before counts; retained stdout/lazy
owners, if any, remain. This does not turn H's raw-byte writer proof into a
complete wrapped serde/error proof; I21 owns those outstanding leaves.

Counts begins with model.source_parts consumed by PrimitiveSource::new. Reuse
K0's **SRC0 constructor/adapter** capacities and helpers only; no PREP/GROUP,
solve cache, certificate or W1 outcome is created. At V387 a first encoded-source
Vec survives SHA's copy/padding/hex output; it drops after the hash call. The
k4src String survives. No encoding buffer is retained merely because its digest
is retained.

Q136 computes profile(&free_adjacency(model)) before the Counts struct:

- A13-51 owns restrained bool[n], position usize[n], exact outer f Set headers,
  each BTreeSet<usize> adjacency and one current12-DOF (or3-DOF) temporary Vec.
  During set-to-Vec conversion retain input nodes/backing beside output headers/
  child buffers. Allow inherited outer allocation bytes for owned-map collection,
  using max(sizeof(BTreeSet<usize>),sizeof(Vec<usize>))*f for returned headers
  until final type/reuse facts tighten it. This reuses the existing adjacency
  node parameter, not a new node probe.
- The borrowed adjacency result stays through profile. profile uses **K4 RCM**,
  whose reviewed eccentricity has4 buffers, not SD's6. After RCM returns,
  order, rank and block each have f usize slots, plus a push/pop DFS stack bounded
  by f (mark before enqueue). Let HK4RCM be the existing reviewed helper above
  borrowed adjacency; then HProfileCount=max(HK4RCM,24f+G(usize,f)+epsilon*O_G(usize,f)).
  These are count arrays/scalars; no profile-value matrix is allocated here.

Let RAdj/HAdj be that existing construction/returned adjacency union. The first
counts subphase is max(HAdj,RAdj+HProfileCount), then its temporary adjacency and
all profile-local Vecs drop before pattern_entries.

**Additional node-interface fact:** Q61 builds a nonempty
BTreeSet<(u32,u32)> of ordered member-node pairs, up to2m entries, beside
bool[N]. This is not BTreeSet<usize>, nor one of K0's named seven specializations.
Use a distinct symbolic I_pair32(2m). Q70 also declares
BTreeSet<(u32,usize,usize)> for spring-only entries. Exact RF-LARGE24 inputs
(including the12 external files) all have no springs, so that tree stays empty
on this actual prefix. Preserve its separate I_spring3(s+9d) term for a wider
counts API; do not narrow that API or claim type-layout equivalence.

```
HPatternCount=N+I_pair32(2m)+I_spring3(s+9d)
```

Then Q151 `layout(source).len()` creates G(QuantityMeta,q), and Q152
`source.encoding().len()` creates a second encoding Vec. Conservatively retain
both receiver temporaries until the complete Counts initializer ends; there is
no explicit intervening drop. The returned Counts contains only scalars:

```
HCounts=max(HAdj,RAdj+HProfileCount,HPatternCount,
            G(QuantityMeta,q)+HEncoding(X))
```

SRC0 and k4src remain outside HCounts. V399 finally drops SRC0; sizes has no heap.
The proposed/final estimator must be added if its implementation allocates after
or before this cut; today's arithmetic-only estimate does not grant future zero.

## Prefix maximum and exact cut survivors

Take maxima over the actual ordered phases, with their explicitly retained
prefix: ordinary runtime entry; Hargv; HFamily; embedded clone or HExternal;
start construction/emit; SRC0 construction; first encoding+HHash; and HCounts.
The surrounding Args/id/selected Case/model/hash owners are added at each edge,
not every already-freed phase summed together. Failure/usage/noop branches keep
their reached prefix plus the separately owned fail/format/emit/panic envelope;
noop has no family/source/count cut. Global counters retain all earlier peaks.

At tV the source-derived survivor union is exactly:

```
C_tV=RuntimeRet_after_start + ArgsRet + I + RCase(selected)
     +RModelStandalone + optional RHex_model + RHex_k4src
```

There are three distinct case-id strings (Args.case,id,Case.id); Case.id is already
inside RCase. Embedded Case.model plus standalone clone are distinct. No family
text/vector/discarded Case, raw Value, temporary path, source/encoding/adjacency/
layout/count-helper Vec survives. Counts/Sizes/Phase wrappers are inline.
Do not add the old VR fixed/model estimate on top of this identity union.

The original global composition remains
`max(PrefixPeak, max_later(C_tV surviving identities + later owner union))`
for both counters. Neither PrefixPeak nor C_tV is an observed input in this
proposal. This cannot be replaced with the current heap at the cut.

## Exact remaining cells and transfer boundary

1. Bind I_pair32 for the newly identified reached counts set, alongside the
   existing JSON/Case/adjacency node pairs. ROOT was notified; no new layout work
   or assumed mirror/equivalence was performed. The extra spring triple set is
   proven empty only for the actual RF-LARGE prefix.
2. Complete the runtime **global-prefix** premise before args and any newly
   reached lazy initialization. Existing H direct-main survivor/stdio identities
   remain usable within their reviewed source scope and final-build transfer;
   H completed-stage evidence alone is not proof about earlier freed global
   peaks. This packet does not invent a value for any unsupplied startup peak
   or reopen the reviewed64/544 requests. Match the actual final VR std/source/
   entry/allocator/cfg/artifact and explicitly account any extra prefix/error
   event, or retain that precise gap. No general runtime audit is commissioned.
3. I21's HHex, initial JSON stream/format/IO/error/panic terms and any retained
   helper effects must be composed at the named edges. The successful fixed
   parser proof does not close failed file/usage/SourceError/hash-check routes
   or formatted failures. Their existing owners are exposed, not zeroed.
4. Freeze actual CARGO_MANIFEST_DIR/argv/cwd/path bytes and stable input identity,
   final serde/std features/type sizes/source correspondence, and checked integer
   operations. Fixed JSON metadata does not execute/accept typed constructors,
   canonical-source validation, graph counts or a solver. Subject these finite
   new parser/read/count unions to independent review before K0 substitution.

This closes/narrows the named source ownership edges without a baseline,
measurement, metric/admission change, new model/graph, allocator change or full
checkpoint acceptance. It ends at the stated residuals rather than expanding.
