RV12's run records for the independent review of slice K3 (PR #1041, head b7e93650e).
Review: T3/REVIEW/K3_REVIEW.md. Everything here was built from `git archive` copies of the
candidate in <wt>/scratch/rv12; the implementer's worktree was read only; no Git writes.

checks/       merge identity, FK/Cargo.toml, commit-by-commit stats, the wide.rs/mod.rs diff
              against eb52114e9, the records' SHA256SUMS and hygiene, GEN-8, hosted CI status
head_tests/   FK's full suite at opt-level 0 on the head archive (227 passed), the non-test
              build (no warnings), and both generators' --check (K3a 6/6 OK, K3 8/8 OK)
oracle/       RV12's own oracle (rv12_oracle.py, dyadic integers and Fraction, written without
              reference to K3's generator), the probe module (rv12_probe.rs, added only to a
              scratch copy of FK as `#[cfg(test)] #[path = "rv12_probe.rs"] mod rv12_probe;`
              at the end of multi.rs; it calls K3's pub(crate) API), and the results:
              check_all.txt (1,023,720 operations, 0 mismatches), check_ops3.txt (mul_pow2,
              cmp_value, fits_precision), check_tail.txt (far subtractions whose (1 - f) tail
              decides a tie), check_wide_operand.txt (TwoSum/TwoProduct operand refusal),
              boundary_report.txt (named binary64 boundary cases at L = 2, 4, 8, 16),
              class_stats_all.txt (the rounding classes exercised), fold_demo (finding S1).
              The operation files are regenerated with:
                rv12_oracle.py gen 20260928 ops1.txt; gen <20260928+s> for s = 1..30;
                gen2 <s> for s = 7, 11..15; boundary_report.py ops ops_boundary.txt;
                concatenated in that order and renumbered (ops_all.txt);
                gen3 99 ops3.txt; tail_decides.py 5 ops_tail.txt; ops_wide_operand.txt from
                wide_operand.py (seed 4).
              Their sha256 values are in ops_and_outputs_sha256.txt.
mutations/    rv12_mutate.py (driver), per-mutant .json and .log, the two batch logs, and
              oracle_sensitivity (the same mutants through the probe against the oracle).
              first_attempt_fk_only_archive/: the first full-suite attempt, which did not
              compile because the archive held FK alone (s11_site_table.rs reads sibling
              crates); rerun with the whole core/ tree.
t9/           the T9 spot-check at the final head (Mac-only): S11-K's harness built --release
              against a git archive of b7e93650e (core, fixtures, validation); 112 outputs,
              compared with ROOT's Mac main hashes and I11's candidate list.
build_records.py.txt assembles this folder; SHA256SUMS covers every file except itself.
