#!/usr/bin/env python3
"""RV109 addendum 01: the mutants re-applied at 03f55e7178 (round 2's patch strings, from sp2_mutants.py),
the ordinal revert, and two test mutants of SF-1's record comparison.

Usage: a1_mutants.py <PP src dir> apply <ID>  |  a1_mutants.py list  |  a1_mutants.py check <PP src dir>
"""
import importlib.util
import sys

HERE = "WT/scratch/rv109_rvp_01"
spec = importlib.util.spec_from_file_location("sp2", HERE + "/r2/sp2_mutants.py")
sp2 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sp2)
W, F = "retained_wire.rs", "retained_facade_tests.rs"
M = {k: sp2.M[k] for k in ["M15_selected_diagnostics_reversed", "M16_frozen_attempt_ref_zero", "M17_frozen_attempt_id_zero",
                          "M19_frozen_run_owner_ordinal", "M28_headline_tie_location_only", "M32_headline_stress_only", "M11_sources_back_reversed"]}
M["O1_ordinal_is_canonical_position"] = ("the nodal-term ordinal reverted to the canonical position (`i`)",
    ((W, '"constructor_ordinal":t.original,', '"constructor_ordinal":i,'),))
M["TM1_sf1_verification_flag_not_excepted"] = ("test mutant: SF-1's comparison no longer excepts verification_shared_built_here",
    ((F, "                y.verification_shared_built_here = x.verification_shared_built_here;\n", ""),))
M["TM2_sf1_no_flags_expected"] = ("test mutant: SF-1 expects no flag difference for a later Run",
    ((F, '&[(128, "shared_built_here", false, true), (256, "shared_built_here", false, true), (256, "verification_shared_built_here", false, true)]',
         "&[]"),))


def edits_ok(src, mid):
    texts = {}
    for f, old, new in M[mid][1]:
        t = texts.get(f) or open(f"{src}/{f}", encoding="utf-8").read()
        if t.count(old) != 1:
            return False, f"{mid}: {t.count(old)} hits in {f}"
        texts[f] = t.replace(old, new)
    return True, texts


def main():
    if sys.argv[1] == "list":
        for k, (why, _) in M.items():
            print(f"{k}\t{why}")
        return
    if sys.argv[1] == "check":
        bad = [edits_ok(sys.argv[2], k)[1] for k in M if not edits_ok(sys.argv[2], k)[0]]
        print("\n".join(bad))
        print(f"checked {len(M)}, bad {len(bad)}")
        return
    src, _, mid = sys.argv[1:4]
    ok, texts = edits_ok(src, mid)
    if not ok:
        sys.exit(texts)
    for f, t in texts.items():
        open(f"{src}/{f}", "w", encoding="utf-8").write(t)
    print(f"applied {mid}")


if __name__ == "__main__":
    main()
