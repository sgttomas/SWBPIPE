# V1 — independent review of the T3 designs (D1 and D2 together)

Fresh-context reviewer TASK. Read `_COMMON.md` first. You wrote neither design and have not advised either designer.

## Purpose

Review both T3 designs as one package before ROOT selects them. Find what is wrong, missing, unsafe or inconsistent, with evidence. Do not redesign; say what must change and why. This follows the T0R review (`DEFAULT_ROUTE_DESIGN/REVIEW/RETURN.md`).

## Inputs (read-only)

- `T3/DESIGN_NUMERICS/DESIGN.md` revision 1 (sha256 `7199390f39f2c46d7e98ddb3d70cae682785830fb3470fde5d4817307ebf726b`), with its `_run_records/`.
- `T3/DESIGN_STANDING/DESIGN.md` (sha256 `185e178efe3b985594521111a7b91d88a7c98093be86eb19b6e27ff36ec2bca2`), with its `_run_records/`.
- `T3/STAGE0_MAP.md`, `T3/STAGE1_PLAN.md` (§8 holds ROOT's confirmations) and the D1 and D2 briefs.
- The basis records each design cites, product source at `c61a540ea`, and T1 at `f3270ea79` (with `git show` only).

## What to check

**Across both designs**
1. Every T3 item in `STAGE0_MAP.md` §1 and §2 has a home in one design, or an explicit remainder with a named owner. Nothing closes by containment alone; the closure rule is respected.
2. **The interface.** D1 §5 against D2 §5 (I-1 to I-6). In particular: D1 recommends retiring exact-block selection for fresh solves (its D-4 option A). What does that do to D2's joined-eligibility slice S-E (I-4), to its F1 fallback reach, and to R-B's I-3 gate? Is any slice then built for an identity that stops being fresh? Are the identity, receipt and standing assumptions compatible?
3. No protected predicate or frozen reference changes, and no new tolerance is disguised as method policy. Test especially D1's `2^-64` stop rule and D2's one-ulp bound on host-rounded `exp` fields (DD-4).
4. The file plans agree with each other and with the T1 overlap in `STAGE0_MAP.md` §3 (as corrected for `knownSemanticLimitations.ts`), and the pre-merge slices really touch only disjoint files.

**D1 (numerics)**
5. The probe reasoning in §3.1, which rejects generalized exact-block solving (including axis-aligned members with bending soft modes and N06-class skew). Rerun the standard-library probe if useful, and try to break its conclusion.
6. The stop rule and schedule (§4.1.6): can agreement at 2p accept a wrong answer? Consider the k = 1e-28 example and adversarial cases. Is mechanism handling sound at every precision?
7. The in-repo arithmetic type (D-2): is the proposed verification plan enough to trust correctly rounded operations and range handling? Weigh it against `dashu-float`.
8. The trigger (D-5): can a case pass the ordinary gate yet miss 1e-9? What does the design do then?
9. W2: is "bit-identical whenever today's evaluation succeeds" actually guaranteed by the proposed scaling?
10. W3: is dropping any automatic fallback safe, given current behaviour? Are the parity and memory protocols adequate for M32's requirement?
11. W4 and W5; the coverage phases W1a–W1c, and whether W1a alone leaves real models uncovered in a way the design discloses honestly.

**D2 (standing, envelopes, transport)**
12. F1 against T1's SF-1 (`LOAD_STATE_IMPLEMENTATION/CP4_WIRE_ADDENDUM.md` §1.2 and T1's source): is the port faithful? Are residuals R-1a and R-1b correctly described? Does any failed or Sensitive case gain standing, or any correct case lose it?
13. The J4 re-derivation: its scope, bit-exactness claims (Fraction, BigInt, fma), the DD-4 bound, and the new segment-containment check.
14. The selected-UNAVAILABLE tightening: verify the single-emitter claim and the corpus scan, and whether tightening refuses any qualified byte.
15. R-B retirement and DD-7 (historical-only all-selected source-blocks-1): what users lose, and whether it is disclosed.
16. The transport finding that every finite |x| > 2^53 − 1 is refused by the checked canonical carriers. Verify it against source; state its consequence (for example, a large but finite load becoming a finalization trigger) and whether the DD-8 split to T6 is sound.
17. The display refusal, and the standing changes S-1 to S-4 with cross-language parity.

## Probes

Read-only standard-library probes are fine now. A Rust probe runs only after the manager releases the host, in `<scratch>` with `<t3-target>`, against an exported copy (`git archive`) of `c61a540ea` or `f3270ea79`, never T1's worktree. Record probe sources with `.txt` suffixes.

## Write set

`T3/REVIEW/**` only: `RETURN.md`, and `_run_records/` with probe sources, outputs and `SHA256SUMS`.

## Return

`T3/REVIEW/RETURN.md`:
- a verdict: CLEAR, FINDINGS or BLOCKING;
- a findings table: id, severity (BLOCKING, SHOULD-FIX or NOTE), design and section, evidence with file:line, consequence, and the required change;
- what you confirmed independently;
- what you did not check.

Then send the manager a SendMessage summary.
