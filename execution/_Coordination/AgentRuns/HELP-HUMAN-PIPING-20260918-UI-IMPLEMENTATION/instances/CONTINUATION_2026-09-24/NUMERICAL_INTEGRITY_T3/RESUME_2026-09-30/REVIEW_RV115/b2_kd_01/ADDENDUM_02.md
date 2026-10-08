# RV115 (RV-K) ADDENDUM_02: DEF-C's numerical content (documents only)

TASK (Type 2), RV115, holding RV-K, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. This addendum answers the coordinator's request under RR "I97's B2-C returned and verified; RV118 (RV-C) reviews it; RV115 checks DEF-C". It covers REVISION_01 §1.1's RV-K share: "B2-KD supplies the numerical content; RV-C and RV-K review it RV69-style". RV118 holds RV-C for the contract as a whole.

**The basis** is my B2-KD review (REVIEW.md) and its rulings in RR "RV115 (RV-K) accepts B2-KD with amendments; R-1 to R-11 ruled". REVIEW.md, ADDENDUM_01 and their sums are untouched.

**The subject:**
- `R/I97/b2_c_01/CONTRACT.md`, sha256 `165cd4b1c0b5ed3f83f9230435c0a3ab2284211c90908daa647610bba81d9d28`, verified; SHA256SUMS 13 of 13 OK. I read §0, §2.5, §3, §4, §10.1 (G8) and §12 (C-12, C-15).
- `statics/retained_precision_prepared_combination_v1.json` (DEF-C), raw sha256 `03d40598be82a5df8867bb9a279b212b3acd53ba5f51ff85b9635e6cefaa69cc`.

**Method.**
- Documents and code reading at NUM: `P/core`, `P/fixtures` and `P/schemas` equal main `2007709549`, checked by `git diff --quiet`.
- One standard-library Python script of my own, run once with VENV and `PYTHONDONTWRITEBYTECODE=1` in `WT/scratch/rv115_rvk/` (`addendum_02/`). It re-implements the canonical form; it does not call the checked-JSON executable or import repository code.
- No cargo, no native job, no install, no Git write. The record has no symlink.

Placeholders are as in REVIEW.md.

## Verdict

**ACCEPT.** 0 BLOCKING, 0 SHOULD-FIX, 5 NOTE.

DEF-C states the certificate my B2-KD review found sound. Every member the proof relies on is byte-identical to DEF-O's. R-7, R-8 and R-10 hold, and H rebuilds independently. Nothing in DEF-C asks of a combination owner what the kernel design cannot do, or would let a published combination row lie outside its enclosure.

The notes are wording tightenings and a witness to carry. None blocks J1.

## Findings

| ID | Severity | Where | Finding | Suggested change |
|---|---|---|---|---|
| NB-1 | NOTE | DEF-C `scope.operand_equality` (R-8) | "Material basis" equality gives bit-equal material operands (E, Ĝ, or the interpolation tuple t_lo … g_hat) only by implication. It goes through T0R's one-basis gate (`COMBINATION_MODULUS_BASIS_MIXED`, PP/src/preview_physics.rs:775–792) and G8's per-case rederivation from one model. The G-lane law depends on the whole tuple. | Optional: also say "the selected material operands of every member, bit for bit" |
| NB-2 | NOTE | CONTRACT §0 item 10; DEF-C `rows.coverage` (R-7) | I97's R-7 witness has components with stress modifiers (4) and a constant-effort hanger, so it is outside W1a. Its summary says `support_count` 2, while g = 1 counts only the support with reaction rows ("attributed"). It confirms PP's shape-generic publication code, not a W1a model. | Carry to B2-W: W-CB1 and W-CB2 confirm 7n + 50m + 8g on a W1a model (S-14) |
| NB-3 | NOTE | DEF-C `lanes.loads` | "Never an operand's individual terms" concerns the load **values**. The data-flag predicate must stay per individual product (c_i ≠ 0 and v ≠ 0; FKR/ledger.rs:225), never per net, so cancelled nets stay data (RV56). The text is silent on it. | Optional: add "data flags from individual products, never from nets" |
| NB-4 | NOTE | DEF-C `scope.prescriptions` | DEF-C says `exact_positive_zero`, but B2-K's planned check compares by value (`== 0.0`), which my review's N-3 left optional. It has no soundness effect, since G8 checks each operand source's prescriptions. | Adopt N-3 (compare bits with +0.0) in B2-K, so the text and the predicate agree literally |
| NB-5 | NOTE | C-12; DEF-C `trust.G8` | Readers recompute K4CMB but not K4LED, consistent with cases (readers recompute K4SRC and K4STF and attest `ledger_sha256`, `retained_precision.py:312`). The kernel binds K4LED to its own evidence (the view's `encoding_matches`), and B2-K's oracle checks K4LED independently. My REVIEW evidence gives independent K4LED sha256 values for C1–C6. | None |

## 1. R-8: operand equality

**What fixes each law, member by member:**
- **K law:** E, G, A, Iy, Iz, J, the frame (nodes, y-reference), springs and the constrained set. All are K4STF bytes (FKR/source.rs:812–870), and the kernel checks them across operands with layout, stations and supports (`validate_operands`).
- **G law:** D, t_eff and the material operands (`build_member`; `material()`), plus the same frame and supports.
- **K-lane stress recipe:** A, Ẑ, J and c from `ProductMemberFacts` (final_case.rs:74–84, :748–758).

**DEF-C binds the rest:**
- the K-law terms above, through the kernel's checks;
- material basis, normalized D, effective wall and the prepared A, I, J, Z and c "across all operands and the combination".

**So every certificate bounds the same law pair.** Every operand certificate and the combination's bound the same K and the same G, which is my review's §5.5 item 4. **Nothing is missing,** given NB-1's implication.

**What the kernel cannot check is assigned correctly.** Cross-operand D, t_eff, Z and material equality is G8's (CONTRACT §10.1: "R-8's operand equality (§2.5)", plus `material_basis_ref` equal to every operand case's). The combination's own facts are the representative CaseSource's by construction (C2 §3), and the kernel checks those against S's members for the combination itself.

## 2. R-7: the coverage rule

**DEF-C's `rows.coverage`** is the kernel rule:
- 6n + n + 12m + 18m + 20m + 6g + 2g;
- slots 0–19 required and slot 20 empty;
- no record.

**I recounted independently** (evidence §3), taking n, m and g from distinct entity references rather than from case-row arithmetic. On `preview_physics_unicode_ids_sparse.json` (sha256 `a5f6d688…`), the combination has **129 rows**, with n = 3, m = 2 and g = 1. The families are exactly 18 / 3 / 60 / 40 / 6 / 2:
- each member has 10 rows at each of `end_i`, `end_j`, `quarter_1`, `midspan` and `quarter_3`;
- row ids and (kind, entity, location, component) keys are unique;
- there is no maximum, intensified, mode, parity or modulus row.

**Each case has 134 rows:** 7n + 51m + 8g = 131, plus 2 intensified rows and 1 mode record. I97's 129 = 7·3 + 50·2 + 8·1 is confirmed (NB-2).

## 3. R-10: the hash domain, rebuilt

H(domain, d) = sha256(JCS({"domain": domain, "payload": d})), as PY's `_hash` defines it (`retained_precision.py:186`). I re-implemented the canonical form, valid here because the values hold no floats and only ASCII keys (asserted). The results (evidence §1):

| Object | Hash |
|---|---|
| **Control:** DEF-O under `retained_precision_formation_v1` | `a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349` (= PY's `DEFINITION_HASH`) |
| DEF-C under `retained_precision_formation_v1` | **`9adf5178c7c1d5b81de340c2f0f605396e717a9264b59430df8d2a1306f21731`**, as I97 and PTABLE's revised `product_formation_definitions` state |
| DEF-C under the alternative domain | `562cbe14…`, as I97 shows |

**Canonical form.** Both raw files equal their canonical form, and DEF-C is ASCII only.

**The operand link.** `inherits.operand_definition.sha256` equals H(DEF-O). That is the same H-in-a-`sha256`-field convention PTABLE already uses for DEF-O.

**Reuse is sound.** The two ids differ, so the two H values cannot collide by construction, and no numerical content depends on the domain's name.

## 4. The proof statement and the stages

**Byte-identical to DEF-O** (evidence §2, every leaf):
- `precision`; `projection`;
- `acceptance.final`, `native`, `standing` and `warrant`;
- `lanes.order`, `per_lane`, `private_only`, `reuse`, `annular_seed` and `annular_source`;
- `rows.component`, `component_stress`, `displacement_magnitude` and `support_magnitude`;
- the definition, publication, receipt and source hash domains;
- `scope.materials`, `supports`, `source`, `entry`, `requires` and `scope_limit`;
- `trust.G7`, `attestation` and `validation`;
- every inherited policy.

DEF-O's final predicates, classes, projection and lanes are therefore unchanged.

**Changed, and each matches my review's findings:**
- **Loads.** `scope.loads` is `combined_exact_ledger`. `lanes.loads` encloses each net N_g outward at 1024 bits, in free residual rows and constrained reaction offsets (both sites, SF-2), and forms no binary64 product or rounded net. That is E5.
- **Prescriptions** are an exact +0 for every operand term, which is E6 and the kernel's P2.
- **`lanes.admitted_k`** reads the representative's members (common K4STF).
- **`lanes.owner`** is the recorded selected Run with its K4CMB source, its ledger, the imports plus its own builds, and one anchor.
- **The rows.** `rows.maximum` is not published and `ancillary` is none. `rows.coverage` is as in §2.
- **The excludes** add subtraction, range, nested, repeated, no-selected, maxima, intensified rows and nonzero prescriptions, and drop only `prepared_combinations`.
- **`acceptance.composition`** is my review's §5.5 items 1–3 and 5.

**Stages.**
- `preparation` is `not_entered` both in `stages` and in `preparation.combination`.
- There is no preparation hash domain.
- `maxima` completes empty, matching FK's `complete_maxima(&[])`.
- `CombinationAttempt` has no preparation member (CONTRACT §4).

**C-12.** Readers recompute K4CMB as `"K4CMB\x01"`, u32le(h), then per operand u64le(factor bits), u32le(len) and K4SRC. That is FKR/adaptive.rs:1001–1006 exactly. K4LED stays attestation (NB-5).

## 5. Could a combination owner fail DEF-C, or a row escape its enclosure? No

**Every clause is something the kernel design does:**
- the trigger: the first selected operand's group, and operand 0 as representative;
- operand sources: rebuild and K4SRC check, with no imports from prepared operands;
- P2;
- the net enclosure at both sites;
- `product_owner` with the combination kind;
- the coverage branch;
- an empty maxima completion;
- its own Run under its own 20B case limit.

**Every soundness premise is bound or refused:**
- nonzero prescriptions are excluded and refused by the kernel view;
- a missing, duplicated or extra row fails the whole proof;
- no operand quantity enters (`acceptance.composition`).

**The rows that bypass the proof are owner-generic.** Support and displacement magnitudes, and InputDerived rows, use DEF-O's unchanged recipes on the combination's own enclosures.

## 6. Inputs, execution and limits

**Read** (sha256 prefixes):

| Input | sha256 |
|---|---|
| CONTRACT.md | `165cd4b1c0b5ed3f` |
| DEF-C | `03d40598be82a5df` |
| I97's `b2c_checks.py` and its output (the R-7 method) | read only |
| PTABLE now; PTABLE revised | read: their `product_formation_definitions` |
| DEF-O | `3e0779a45a74cf0b` |
| The R-7 fixture | `a5f6d688c8eae9a2` |
| PY `retained_precision.py` | read: `_hash`, G0, K4SRC |
| `canonical_json/adapter.py` | read: the H definition |
| PP `preview_physics.rs` | read: `gate_reason` |
| FKR `final_case.rs`, `ledger.rs`, `adaptive.rs` | as in REVIEW.md |

**Executed.** `addendum_02/rv115_addendum02_checks.py`, run once with VENV's Python 3.13.14. It takes DEF-O, DEF-C and the fixture as arguments, reads only those, writes only stdout, and exited with status 0. Its output is `addendum_02/rv115_addendum02_checks.out.json`, and `addendum_02/RUN_ADDENDUM_02.md` has the command with placeholders.

**Limits:**
- **Numerics only.** This is DEF-C's numerical content. The contract as a whole is RV118's.
- **No code was compiled or run.** The kernel's behaviour is B2-K's to establish, with its oracle and RV-K's code round.
