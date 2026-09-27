# ROOT's selection of the S11 containment

HELP_HUMAN (ROOT), 2026-09-26, under the owner's delegated correctness authority. ROOT relayed it to the T3 manager by SendMessage, and the manager records it here.

**Selected:** [DESIGN_NUMERICS/S11_CONTAINMENT.md](DESIGN_NUMERICS/S11_CONTAINMENT.md) **revision 3** (`d6575c25e`, sha256 `561c7200…`) as the S11 containment method, together with the R3 fixes below. It is selected ahead of the full T3 selection package, because S11 is top priority. The basis is V1's check ([REVIEW/S11_CHECK.md](REVIEW/S11_CHECK.md)), its backchecks ([S11_BACKCHECK.md](REVIEW/S11_BACKCHECK.md) and [S11_BACKCHECK_R3.md](REVIEW/S11_BACKCHECK_R3.md), which is FINDINGS with both blockers resolved), and ROOT's rulings in [ROOT_RULINGS_V1.md](ROOT_RULINGS_V1.md).

## Conditions (the R3 fixes)

1. **R3-1.** T1's committed fixtures with nonzero support motion are pre-registered in the fixture stop rule as expected diffs: `load_reference/connected*`, `load_reference_source/eigen_motion*`, and their derived results documents. The "bit-identical" claim is scoped to all-zero prescribed values. Every such diff is still reported to ROOT, with the size of each change, before regeneration.
2. **R3-2.** F8 uses (G, 0.3, −G), is added to the precondition list, and gets a thermal case with cancelling strains. K4 gets an axial-effect case, thermal or thrust (G, n, −G). M1h–M1k and M1b must then be killed.
3. **R3-3.** The constant-table approach: the §2.5 table becomes the site test's constant, and every floating-point compound assignment or sum/fold in the named modules must appear in it. The `nonlinear_integration:1333` unit-force influence solves are allow-listed as T5's.
4. **Notes.** KS1 and KS3 scale before rounding, or the design proves that double rounding cannot occur. The M1 label count is fixed.

D1 carries these fixes in a narrow S11 revision 4. V1 checks revision 4 in its combined pass with DESIGN revision 3 and D2 revision 3.

## Implementation

- **S11-K** may be built now, T1-disjoint, as its own full-gate PR: independent review, hosted CI including the surface-4 dual-viewport dispatch, a clean DEC-025 sweep, and the fixture stop rule.
- **It lands after T1 merges** (T1's PR963), and takes the pre-registered T1 fixture diffs, reported to ROOT with their sizes before regeneration. It is built either on main after T1 merges, or now on a base that already includes T1's head `e43412a9b`, so its fixture diffs are known during development.
- The S11-K TASK builds to revision 3 plus the R3 items above. It does not wait for V1's combined pass. Anything that pass finds in revision 4 is applied to S11-K before the S11-K PR merges.
- The S11-K implementation review verifies each R3 item on the actual code, and verifies the required kill set (G = 1e8 and 1e80, one mutant per E site).
