VERDICT: FINDINGS

Scope: the complete delta `5438c1c98..dcbfda55b` (commit `dcbfda55b`), 9 files, all under `projects/chirality-piping/`. Findings 1–4 and 6–8 are closed. Finding 5 is only partly closed: the repair added a sentence about reproducing the ruled-on packet hash that is now false (N-1 below). Nothing else new is wrong.

**(a) Closure of findings 1–8**

1. **Closed (BLOCKING).**
   - Profile §4 layer 4 no longer requires a per-row baseline class. Recording it per row is now a candidate convention for R0.
   - §5 is retitled "Authority tiers — candidate for R0" and says it is "Not in effect until a run's owner rules it at R0".
   - Layer 5's "invariant-tier claims" is now "contract-level obligations". The piping `docs/CONTRACT.md` supports that wording (OPS-K-PRIV-1, OPS-K-PRIV-2).
2. **Closed.** Packet Item 5a now reads "kernel §7's project adoption record".
3. **Closed.** The register row's Decision cell now reads "run surfaces and the complete write and read-only boundary". The row still has 6 columns.
4. **Closed.** `OWNER_DIRECTIONS.md` line 5 now cites only the session transcript file name. The four quoted blocks are unchanged (hashes below), and `WORK_GRAPH.json` carries the file's new hash.
5. **Partly closed.**
   - Done: the ruling record's "Accepted subject" now discloses the pre-ruling profile §6 amendment and the proposal-commit profile hash `ac29229a…`, which is correct.
   - Not done: its reproduction sentence names the wrong source. See N-1.
6. **Closed.** Item 5d now names `execution/_Coordination/_TaskManagement/REGISTER.csv`, and the file exists.
7. **Closed.** `HANDOFF_STATE.md` has a "Departures from the approved plan" section. It records that the notice was not written, why, and how to reverse that.
8. **Closed.** Packet Item 3 now says R0 must reconcile the convention with the contract's "Remaining is executable truth" invariant and with validity item 12.

**(b) Finding 1's repair only narrows the profile.**
- The profile delta changes only §4 layer 4, §4 layer 5 and the §5 heading and lead paragraph. It adds no obligation, surface, phase or permission.
- No other part of the profile makes an extension that packet Item 2 leaves for R0 binding. The two that were binding (authority tiers and a per-row baseline class) are now candidates.
- §4's "Each row records which layers apply" stays. The contract's "Project divergence is preserved" invariant already requires it, so it is not an extension.
- The profile says nothing about sealed passes, cause tags or cause-clustered packets.
- The packet edits (Items 3, 5a, 5d) change wording only, not any ruled effect.

**(c) "Changes after ruling" is accurate and complete for the profile, packet and register.**
- It lists all three profile changes and all three packet wording corrections, plus the register cell change.
- The profile SHA-256 it states, `271bd0d0…da73`, matches the file at `dcbfda55b`.
- The `ac29229a…` prefix for the profile at the proposal commit `4a1b6fdd3` is correct.
- It does not list the edits to `OWNER_DIRECTIONS.md`, `HANDOFF_STATE.md` and `WORK_GRAPH.json`. Those are run-record files, not ruled instruments, so this is acceptable.

**(d) `WORK_GRAPH.json` hashes match the files**, and the file parses as valid JSON (see the hash table).
- The stored `returns/ACT-REVIEW_return.md` runs from `VERDICT: FINDINGS` to `END-OF-RETURN`, with no trailing newline. It matches my return section by section and line by line; I compared it against my own message by reading, not with a byte diff.
- `briefs/ACT-REVIEW_brief.md` at `dcbfda55b` still has the sealed hash `17a385b4…c796`.

**(e) New findings**

**N-1. The ruling record now says something false about how to reproduce the ruled-on packet hash**
- **Severity:** ACTIONABLE
- **Where:** `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-73_RULING_2026-09-21.md`, lines 18–19.
- **What is wrong:** The sentence says the hash `0241867d…` "reproduces from the merged packet by replacing the body of its §6 with the single line `*(Awaiting ruling.)*`". This same delta then edited the packet after the ruling (Items 3, 5a, 5d). Applying that procedure to the packet at `dcbfda55b` gives `8c9660d3558bc936040614bce77b65680ac09f47eeddb97ef6bf544aca12baa4`, not `0241867d…`. It still gives `0241867d…` from the packet at `5438c1c98`.
- **Smallest fix:** Replace "from the merged packet" with "from the packet as committed at `5438c1c98`".

**Hash table (at `dcbfda55b`)**

| Item | Stated | Recomputed | Match |
|---|---|---|---|
| OWNER_DIRECTIONS.md (WORK_GRAPH) | 03738706…0b6b | 03738706…0b6b (6752 B) | yes |
| briefs/ACT-REVIEW_brief.md (WORK_GRAPH) | 17a385b4…c796 | 17a385b4…c796 (4413 B) | yes |
| returns/ACT-REVIEW_return.md (WORK_GRAPH) | 23bf3049…b3f | 23bf3049…b3f (10232 B) | yes |
| PLAN.md (WORK_GRAPH) | 18d39600…6381 | 18d39600…6381 | yes |
| ENTRY_BRIEF (WORK_GRAPH) | 3767db25…7823 | 3767db25…7823 | yes |
| Profile as merged (ruling "Changes after ruling") | 271bd0d0…da73 | 271bd0d0…da73 | yes |
| Profile at proposal commit `4a1b6fdd3` | ac29229a… | ac29229a8dd6… | yes |
| Packet as ruled, from `5438c1c98` | 0241867d…a879 | 0241867d…a879 | yes |
| Packet as ruled, "from the merged packet" (`dcbfda55b`) | 0241867d…a879 | 8c9660d3…baa4 | **no (N-1)** |
| Quoted owner blocks: Directions 1–4, Acts 1–2 | ac397039 / ab4cc637 / 78cff483 / 94ebe9ac / 19d90885 / e40b902e | same, with byte counts 1825 / 765 / 110 / 82 / 844 / 39 | yes |

**Validator outputs** (HEAD `dcbfda55b`, `PYTHONDONTWRITEBYTECODE=1`)

- `python3 tools/validation/validate_claims_language.py` printed "VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied" (exit 0).
- `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .` printed "VALID …/projects/chirality-piping/loop/LOOP_RECEIPTS.md: frozen through Receipt-44; versioned receipt contract satisfied" (exit 0).
- `git status --porcelain` is empty, so this backcheck left the working tree unchanged.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

END-OF-RETURN