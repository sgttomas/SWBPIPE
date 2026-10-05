# RV68 initial frozen-source assessment

Status: source and exact arithmetic assessed; **not clear for component acceptance**.
This is an unsealed working assessment of `430bc4f798` plus FIRST_AMENDED patch
`dd6c65aafbf7804425935fdfaae43d5b09a735b39039d9e8028eb83772716925`.
It makes no claim about I51's later moving source or completed controls/accounting.

## Actionable findings sent to ROOT

### RV68-1 — P2, blocking: consume the one-attempt permission on every path

In PP retained_product.rs:3346–3419, `project_candidate` takes PreparedCase by
value but returns it through `PreparedCandidateRefusal.prepared`, restores its
native owner and sets no spent-attempt state. On Proof, Observable or G5a failure
the capture error may remain clear. Calling the method again with the returned
prepared state and ordinary envelope therefore starts another pair of residual
lanes and can discard the first proof's owned work. A consuming signature alone
does not prevent re-entry when the consumed state is returned. API §3 and the
C2 continuation require one actual attempt, including refusal/re-entry.

Repair with a closed sticky attempted state or a state transition that never
returns draft-start capability after entry. Charge/check it before any proof
construction, preserve first entered work/cause, and demonstrate refusal re-entry
without a second lane/native call. No invocation-wide public debit is requested.

### RV68-2 — P2, blocking: retain custody of the actual ordinary envelope

PP `prepare_case` at 3113 borrows an ordinary envelope, while
`project_candidate` at 3346 later accepts an arbitrary owned MechanicsEnvelope.
No ordinary-owner anchor or closed owning state ties those arguments to the
completed `finish` observation. `finish` checks semantic identity, solved status
and no exact selection, but `project_candidate` only binds row/observation
metadata and checks numerical/observable evidence. Passing an otherwise identical
substituted envelope with changed source_block_recovery, semantic contract,
mechanics status or run/model identity can preserve every checked row and reach
private commit. The FK projection anchor binds its native proof and values; it
does not authenticate this separately supplied ordinary envelope.

API §3 requires one OrdinaryOwner and disallows arbitrary replacement. Consume
the actual ordinary owner into the prepared state/driver and prevent caller
substitution, with controls spanning finish, preparation, attempt and commit.
Repeating a subset of header predicates does not by itself establish same-owner
custody. This is a source-confirmed path; no runtime reproduction was authorized.

### RV68-3 — P2 local prerequisite: retain projection conversion evidence

FK final_case.rs:1534 calls `raw.to_binary64()` without a separate entered
conversion counter or retained conversion outcome. Underflow is collapsed to
0.0. The unchanged final distance check still includes the physical nonzero error,
so this finding does not assert a false numerical PASS. Selected DESIGN §1–2
requires the actual conversion outcome and separately counted conversions,
including failed prefixes. WideContext's add/mul/div record is not that record.
Complete this local ledger/outcome custody and normal/subnormal/underflow/failure
controls before local acceptance; library-internal integer work may stay named
auxiliary work.

## Concrete accounting observation awaiting I51's C4 return

`row_scales` runs at draft start and final certification. Its line 971
`spent.verdicts = reserve(...)` allocates the replacement before dropping the
first verdict vector; that first capacity stays live during both lanes. The
capacity slot is overwritten, so the concrete local schedule must preserve both
allocation entries, failure prefixes and their overlap (or remove the redundant
allocation). This is additional diagnostic/local work, not a public native-meter
debit demand. The existing C4 return explicitly leaves local work open.

## Checks already completed

- Reconstructed all eleven core changed files from immutable Git WIP and applied
  the exact first amendment in RV68 scratch. Both amended hashes match ROOT's
  frozen manifest. Full maintained diff against accepted c79 was inspected.
  The standard scope helper passes the exact selected fence. Adaptive changes
  only add the consuming data move; lib changes only the granted five-line hook.
- Traced genuine prepared source formation from normalized D/effective wall and
  independently selected E/G; no E/nu substitution or repeated wall subtraction.
  New K/facts/operational operands are produced from prepared values.
- Traced recorded native owner/run/source/cache, both closed laws, same-draft K
  center seed, exact prescribed/no-data handling, one correction per lane, fresh
  residual and recovery, complete stored lanes and final-check reuse.
- Checked both physical norm proofs remain; support-only actual two-call hypot
  precedes final scale/class recomputation. Component identities/units are checked.
  Ancillary values follow the captured observation metadata and presence path.
- Checked final frozen values feed row binding, certification, observable guards,
  G5a, coefficient maxima and headlines. The post-gate transfer has precharged
  finite moves and no Result/allocation path after first row mutation.
- Independently reconstructed both modes' actual source encoding and re-derived
  all 194 mechanical point predicates and actual dual-certificate predicate sets;
  all 232 K/Source native enclosure truth checks pass. Replayed 176 fixed1024
  projections, 16 two-call host C hypot sequences, two maximum midpoints, two
  headlines per mode and both G5a calculations. Same classes each mode: 69
  Absolute, 25 Relative, 3 InputDerived. Native p128 is observed, never inferred
  from proof precision1024. G5a lowers are 53050.36890729408 N and
  96.11431291987076 N*m; uppers 325026.4895405444 N and 975079.4686216331 N*m.
- Independent old-J nonoverlap and old binary64 stress recipe refusal controls
  remain true. Existing sharper/raw/SI decimal inequalities are unchanged.
- Both S11 inventories and formation runtime source are unchanged. Historical
  C0 premature-run qualification, first C1 failures, seven-row C2 refusals and
  source-recovery-after-run qualification remain explicit in the read evidence.

## Limits and pending basis

No Cargo, model, solver or native lane was used. Exact arithmetic uses the
inspected, hash-pinned RV67 analytical derivation and RV68's independent actual
row/lane mapping, interval recipes, bit rounding and comparison checker. Author
PASS flags are checked against re-derived results, not used as premises.

Final sealed source delta, all new controls, actual local owner/capacity prefix
accounting and same-reviewer repair confirmation remain pending. ROOT may release
focused runtime after I51 reaps its sole lane. Public route/receipt/readers,
all-in resource/caller/RSS qualification, native Current and PR gates are outside
this private review. ROOT owns acceptance and integration.
