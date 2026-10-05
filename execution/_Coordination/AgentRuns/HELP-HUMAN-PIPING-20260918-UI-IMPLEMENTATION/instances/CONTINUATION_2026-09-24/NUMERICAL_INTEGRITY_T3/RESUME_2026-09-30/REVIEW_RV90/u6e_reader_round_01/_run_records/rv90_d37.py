"""RV90: D37 table derived by RV90 from PP source (retained_receipt.rs transitions; retained_product.rs
prepare_owned_case :3309-3433, solve_native :3449-3468, freeze_candidate :3650-3760; the facade order
lib.rs:2962-2974) and C3's PublicFailure kinds (C3_DELTA:277), compared entry by entry with corpus d37."""
import json, sys
corpus = json.load(open(sys.argv[1]))["d37"]
# Each native exit point: (stage record left, public kind). '-' not entered, C completed, F failed.
# A record is fixed by: the stages completed before the exit, the stage entered at the exit (F after fail_entered),
# or, for checked stages, the check result.
exits = [
    ("F---------", "preparation", "prepare_owned_case: any Err after enter(Preparation); fail_entered :3430"),
    ("CF--------", "native", "solve_native: nonselected outcome after o.native=Some (NativeUnavailable); serializer native{run_ref}"),
    ("CF--------", "capture", "solve_native: Err before o.native=Some (source missing, Origin errors); D38 capture{cause}"),
    ("CC--------", "capture", "freeze_candidate before enter(ProofStart): consumed, missing owner, nonselected, bind_rows, prepared_specs, MapWrite"),
    ("CCF-------", "proof", "begin_prepared_product Err -> Proof"),
    ("CCCF------", "proof", "draft.project Err -> Proof"),
    ("CCCCF-----", "abandoned", "prepared_maxima Err -> Abandoned"),
    ("CCCCCF----", "values", "complete_maxima Err -> Values"),
    ("CCCCCCF---", "abandoned", "prepared_alias Err -> Abandoned"),
    ("CCCCCCC---", "abandoned", "bind_rows_view Err after completed(Aliases), Certificate not entered; fail_entered is a no-op"),
    ("CCCCCCCF--", "proof", "certify_final Err; verdict copy failed or does not cover rows: checks not entered"),
] + [("CCCCCCCF" + o + g, "proof", "certify_final Err; Observables and G5a entered and checked") for o in "CF" for g in "CF"] + [
    ("CCCCCCCC--", "capture", "certified; matches_values false or prepared_verdict_copy Err"),
    ("CCCCCCCCFC", "observable", "!pass with observable_error"), ("CCCCCCCCFF", "observable", "!pass with observable_error"),
    ("CCCCCCCCCF", "g5a", "!pass, observables passed, g5a_error"),
    ("CCCCCCCCCC", "numeric", "!pass with both checks passed (numeric_pass false, or capture.error/adapter fault/final_calls)"),
    ("CCCCCCCCCC", "capture", "pass; commit move-plan CountRange or adapter.require Err"),
]
mine = {}
for rec, kind, _ in exits: mine.setdefault(kind, set()).add(rec)
theirs = {k: set(v) for k, v in corpus["kinds"].items()}
rec = lambda r: r.replace("-", "-")
print("kinds equal:", set(mine) == set(theirs), sorted(set(mine) ^ set(theirs)))
for k in sorted(set(mine) | set(theirs)):
    print(f"  {k:12s} rv90={sorted(mine.get(k, ()))} corpus={sorted(theirs.get(k, ()))} {'OK' if mine.get(k) == theirs.get(k) else 'DIFF'}")
# universe: records with a completed prefix of the 8 pipeline stages then one F or -, rest -, checks entered together after certificate entered
U = set()
for k in range(9):
    for t in (["F", "-"] if k < 8 else [""]):
        first8 = "C" * k + t + "-" * (8 - k - len(t))
        for o, g in [("-", "-")] + ([(a, b) for a in "CF" for b in "CF"] if first8[7] in "CF" else []):
            U.add(first8 + o + g)
print("universe", len(U), "equal to corpus records:", U == set(corpus["records"]), "all exits inside:", all(r in U for r, _, _ in exits))
print("unreachable universe records (refused for every kind):", sorted(U - {r for r, _, _ in exits}))
print("C3 kinds:", sorted(theirs) == sorted(["preparation", "native", "capture", "proof", "values", "abandoned", "numeric", "observable", "g5a"]), "unknown:", corpus["unknown_kinds"])
