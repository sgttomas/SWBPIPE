# B4 CI consumer-map policy check

ROOT authoring/integration, candidateab3b057e4f5a5b01fdbfa1b8e50381522cb1b369, following combined sourcece625f5c8f9a3ff6466c0b49eb4389489ee7fc0d. Only e2e_plan.py and test_ci_e2e_plan.py changed. New b4-table-editing.spec.ts coverage follows existing authoring/layout production-only changes; shared/new table paths still select full coverage.

Actual command in wt2, using configured Python3.13: `python3 -m unittest discover -s projects/chirality-piping/tests -p test_ci_e2e_plan.py -v`. Tool execution session92191 exited0. Supplied final output: `Ran 31 tests in 3.793s` then `OK`. All31 policy cases passed, including both new production-only B4 cases. No product browser, source collection or hosted validation is claimed by this command. Existing oracles remain intact. Final candidate collection and review remain.
