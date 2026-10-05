# C3 corrective addendum — RV69-C3-F1

Proposed exact correction to sealed I52/prepared_public_contract_02/C3_DELTA.md.
Both earlier seals remain unchanged. Same I52 TASK under ROOT; no source/code,
runtime, Git/index/API mutation or public selection. Source is immutable CODE
`922db9dce3`, PP = projects/chirality-piping/core/product_physics/src/retained_product.rs.

**F1 is accepted.** The original clause improperly grouped old and new operational
records under equality to the new C2 section terms. PP:3154 retains old_operational;
PP:3204–3205 evaluates the new A/J. Old torsional stiffness must not be required to
equal new torsional stiffness when J changes. The following text replaces those
clauses; it does not replace the complete C1/C2/C3 corpus.

## Exact replacements

**R1 — replace the ProductAttempt operational member type in C3 §2:**

```text
operational:{old_coverage:"captured_prefix"|"complete",
             old:[MemberOperational],new:[MemberOperational]}
```

The added old_coverage is an actual capture-state observation. It becomes complete
only after the owning old-source/facts/operational cardinality and association
prelude is established; it never follows from array length alone. The current
private source checks cardinality at PP:3138–3142 and performs each old-to-old
section check before that member's helper. Accordingly, complete means a complete
old operational inventory with source/member ordering established, **not** that
every subsequent per-member old-to-old check or operational result has passed.
The latter checks retain their actual failures. Public integration must preserve
that distinction in its typed capture. Before this point the state is
captured_prefix. The flag does not confer source admission or numerical eligibility.

**R2 — replace C3 §2's paragraph beginning “Required equalities” through the
sentence asserting equality to C2 length/axial_stiffness/torsional_stiffness:**

> For every entered PreparedMember, old_source is [Eo,Go,Ao,Iyo,Izo,Jo]
> and old_facts is [D,t,Af,If,Jf,Zf,cf]. Require Ao=Af, Iyo=Izo=If,
> Jo=Jf bit for bit and D/t equal the actual normalized preparation inputs.
> The selected material and unchanged source maps bind Eo/Go and node/frame/load/
> support identities. A prepared result [An,In,Jn,Zn,cn] replaces only the section
> operands: the new native member has [A,Iy,Iz,J]=[An,In,In,Jn], and its product
> facts/C2 operational section data have [A,I,J,Z,c]=[An,In,Jn,Zn,cn].
>
> operational.old records the **retained earlier evaluation**. Its inputs are
> [xi.xyz,xj.xyz,Eo,Go,Ao,Jo]; for each member with a PreparedMember entry, these
> E/G/A/J bits equal old_source indices [0,1,2,5]. Its result and work equal that
> same member's actual saved old OperationalSpent, including an actual error.
> Do not re-evaluate old records while serializing and do not replace them with
> new values. Old axial/torsional stiffness is not required to equal new C2.
>
> operational.new contains only actual newly entered evaluations. Its inputs are
> [xi.xyz,xj.xyz,Eo,Go,An,Jn], using that member's successful preparation result.
> Its result/work equal the returned new OperationalSpent. **Only this new
> evaluation's successful length/EA-over-L/GJ-over-L results match the new C2
> length/axial_stiffness/torsional_stiffness.** Retain an actual failed result;
> never fill a missing new field from old data or publish a fabricated successful
> C2 section term. Existing finalization/encoding failure rules govern an
> unrepresentable source map. Where no new CaseSource exists yet, compare the
> entered new record with its PreparedMember and retained new evaluation; do not
> invent a C2 source reference merely to satisfy this equality.

Unchanged-node coordinates and E/G may match between old and new; that does not
license equality of A/J, section-derived outputs or work. A representative valid
old J is `3f0c52664442210e`, versus prepared J `3f0c52664442210a`; unequal old/new
GJ/L is a valid consequence, not a section-mismatch failure.

An old OperationalSpent.result=Err is preserved and **does not itself forbid a
Ready new product**. There is no requirement that operational.old entries have
result.kind=ready. evaluate_operational returns its result plus work rather than
throwing (PP:2410–2470); the later G5a reads the actual new operational result at
PP:2664–2675. New readiness is established there and by the unchanged final gates.
Independent work/status requirements still apply; this does not erase an actual
accounting fault merely because a numerical result is available.

**R3 — replace the operational-array coverage implication with this complete rule:**

Let M be the authenticated old member inventory, in the producer's native member
order; C2's member map preserves its actual model/native correspondence. For an
earlier incomplete capture, use the actual captured member ordinals/maps in their
construction order; do not infer an admitted PrimitiveSource. Every serialized
MemberOperational carries its actual member id. Arrays never skip an entered
evaluation or sort a failed prefix by value. No helper call occurs before the old
inventory prelude has completed.

| Actual point of refusal/success | operational.old | preparation.members | operational.new |
|---|---|---|---|
| Earlier capture or old-inventory prelude fails | old_coverage=captured_prefix; preserve all actually retained earlier OperationalSpent entries, which may be empty or a full-length yet unqualified prefix | Empty: no helper entered | Empty: no new evaluator entered |
| Prelude passed; reserve or common map/load copy fails before first helper | old_coverage=complete; exactly M old entries | Empty | Empty |
| Failure before the next helper, after k fully processed members | Complete M | The k actual helper entries | The k actual new evaluations |
| Next helper returns SectionPreparationError | Complete M | Successful prefix then the one actual refused helper entry | Prior completed new-evaluation prefix; no entry for the refused helper |
| Helper returns prepared data; later association/copy/accounting failure precedes its new evaluator | Complete M | Include that actual successful helper result/work as well | Prior prefix; no current-member new entry |
| New evaluator returns, then a later map-write/preparations.push check fails | Complete M | Include the successful helper | Include the actual new result and work, even if the result is an OperationalError |
| All helpers/evaluators finished, but new Source construction or support validation fails | Complete M | Complete M | Complete M; no invented successful new source |
| Prepared source ready or later native/proof failure/success | Complete M | Complete M | Complete M |

The captured_prefix case remains unavailable with source_ref/run_ref null and no
preparation digest. Its member ids must resolve through the actual captured input
map; if that association cannot be encoded, use existing receipt-encoding fallback
and retain the private prefix. Do not manufacture a native source or empty ledger.
For old_coverage=complete, old ids exactly equal the full member inventory. Public
readers check old-input equality to PreparedMember.old_source wherever such an
entry exists. Old entries for not-yet-entered helpers retain the earlier captured
evaluation; no dummy PreparedMember is created to bind them. Those unattached old
operands/results remain producer attestations under the existing failed-prefix
trust boundary, not claimed independent geometric re-derivation or eligible rows.

Helper and new-evaluation arrays are entered prefixes of the same member order.
Each new record requires the corresponding successful PreparedMember; a refused
helper cannot own a new record. An OperationalError is a returned evaluator result,
not absence of entry: the actual loop stores it and can continue. The final source
may check it later through G5a. A Ready attempt requires all its existing success
gates; preserving this error does not grant selection.

Do not derive any prefix from preparations.len() or associations.len(). At
PP:3204 the new OperationalSpent is pushed before the fallible accounting entry
and preparations.push at PP:3206. Similarly, successful helper data can exist
before the fallible association capture. The proposed C3 typed capture must retain
the actual helper result/work at return, before a subsequent failure can discard
it. This completes the already identified capture gap; it does not claim the
current private vectors already provide every requested public event.

**R4 — owning-vector and reader clarification:** before PP:3154's replacement,
the earlier old list lives in capture.operational and the new list is empty.
After the replacement, old_operational owns the old list and capture.operational
owns only the new-evaluation prefix. Public capture uses the actual transition
state, never “old_operational is empty” as a proxy. Early refusal must not relabel
the still-old capture.operational vector as new or lose its cost.

G3 checks the declared complete/prefix inventory and ordered helper/new overlap.
G5 checks actual stage/work/error consistency. G8 checks the old tuple association
on its available overlap and the **new-only** C2 operand/result equality. The old
evaluation's actual result remains digest-bound provenance, checked against the
saved old source/evaluation by producer review and independent Rust validation;
this correction does not quietly add three-language replay of old geometry or
claim that hashes prove an entered calculation.

## Definition, source and control effects

DEFINITION.json is unchanged: its preparation.operational clause already calls
for re-evaluation on prepared inputs, and it does not equate old and new values.
Registered-definition H remains `a7ed7ca0bf`; full raw/domain hashes are in the
manifest. The pending C3 closed shape gains only old_coverage and the normative
binding/coverage rules above. No identity/domain/policy name, numerical guarantee,
tariff, source interpretation or owner decision changes. Sealed packets remain
historical inputs; a later reviewed consolidation must apply this addendum.

The implementation seam remains PP retained_product.rs and its already proposed
C3 receipt capture: retain the prelude/swap state, the actual old inventory and
entered helper/evaluator returns. No source edit is made or newly authorized here.
All public finalization, resource/caller, three-reader, native and protected gates
remain open under their owning instruments.

Decisive controls: (1) accept an actual successful old-J/new-J case with each
operational record tied to its own inputs; (2) reject copied new A/J in old inputs
when PreparedMember.old_source remains old, after independent rehash; (3) preserve
complete old/empty helper/empty new at a post-prelude pre-helper reserve failure;
(4) preserve captured_prefix on earlier capture failure; (5) helper-success failure
before new evaluation keeps unequal prefix lengths; (6) failure after evaluation
but before preparations.push keeps the entered new result/work; (7) source
construction failure keeps complete arrays but no invented source. Shared readers
check shape/order/associations; producer/Rust replay checks actual retained old/new
results and failure capture. No runtime or parity controls were run by I52.
