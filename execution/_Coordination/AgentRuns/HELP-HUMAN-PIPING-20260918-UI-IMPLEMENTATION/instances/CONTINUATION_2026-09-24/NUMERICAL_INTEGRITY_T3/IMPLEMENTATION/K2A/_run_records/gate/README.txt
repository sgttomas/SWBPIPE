K2a (I6) both-entry no-Passed-breach gate, two parts (ROOT's K-D5 gate ruling, applied to K2a by the brief).
Tools, unchanged: P1's probe (DETECTION/probe/main.rs.txt, 8dc727f4...) built --release --offline against the
candidate (probe_Cargo.toml.txt); P1's run.py (8d9a32a3...) and compare.py (1c91b6c6...); P1's gen.py (c515ec63...)
re-run on references.py/json c0f14201c: 252 cases, 222 authorable; K-D5's combined gate_run_parts.py (6e9a8b50...)
and gate_check.py (8ad89fca...), copied unchanged (see KD5/_run_records/combined/gate/ and KD5/_run_records/gate/).
Part 1: every (case, mode, entry) except the 4 known dense timeouts (884 runs). Part 2: those 4, on a quiet host
(manager's confirmation; load average ~1.0 recorded per run in gate_part2.log).
Lists: main's GATE/S11_EXCEPTIONS.json (138515b3...) and GATE/FORMATION_EXCEPTIONS.json (0e110b4b...), both empty.
Result (final_check.stdout, final_result.json; the union of both parts, one binary): 888 runs, 768 on frozen
references, 328 trusted, 0 trusted breach triples, PASS.
Standing against main (vs_main.py.txt -> final_vs_main.txt): K-D5's combined-tree final gate result, whose product
source equals main 5ae22926e (only test files differ): 0 changes over all 768 frozen-reference runs.
Standing against P1 (standing_vs_p1.py from K-D5's records): the same 26 changes as K-D5's combined final
(K-D5's 122, and S11-F/S11-G moves already on main); none from K2a.
