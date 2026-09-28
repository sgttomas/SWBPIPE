T9 (I6): S11-K's harness (fixdiff_main.rs, sha256 ec089c1d..., unchanged) built twice, --release --offline:
against main 5ae22926e (git archive of core, fixtures, validation, schemas; binary sha256 e2b9dae7...) and against
the candidate working tree (binary sha256 63e9d8ef...). Every committed JSON request or model under P/fixtures,
P/validation and P/core through run_linear_static_preview_value_with_mode in both modes.
Result: 112 of 112 outputs byte-identical (fixtures 72, validation 30, core 10); 6 outputs are ERR on both trees.
Main's hashes also equal K-D5's recorded combined-candidate hashes (KD5/_run_records/combined/fixture_diff/
output_sha256_candidate.txt), all 112. No committed byte changes; the stop rule did not trigger.
