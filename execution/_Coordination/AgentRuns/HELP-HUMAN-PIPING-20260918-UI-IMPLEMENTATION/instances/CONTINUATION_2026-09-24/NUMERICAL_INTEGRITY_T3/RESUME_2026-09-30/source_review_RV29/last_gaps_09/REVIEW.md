# RV29 last-gap proposal and G3 baseline review

**The two scalar tests are source-ready for separate disposable compilation and exact baseline/mutant experiments. G3 may reuse the preserved full-FK baseline for its exact52-registration subset.** No test, patch or Rust command was executed or installed by this review; no runtime grant, kill or A1 acceptance follows.

## Two scalar proposals

Verified I23 last_gaps_04 seal195f46b175c063bcffd22418941597ac5229568a78672713c687d2c61e86bba5 and all nine payloads. The exact patch9e25d032d73bbeee8df2055520adfa81c84fbc6a3851f3184b602ce6c3398792 appends55 lines/two tests to factor_tests.rs, preserving every original byte as a prefix. Resulting test hash68e272332ca571fda0b0b5ed1a0ebfc0f3b9dc9049d0bb4b8913856b04d842b3. The source hash bindings and exact fault postimages for M9/M32/M11 were independently checked; each fault is disjoint from the proposed test overlay.

M9 calls the actual pivot_passes signature with d=2^-100,c=1,m=1,p128. Independent integer fractions give d*(2^128-1)-64>0, while capping precision at53 gives d*(2^53-1)-64<0. No context rounding or solver result supplies the expected sign. The p53 companion correctly remains false. This is a primitive-inequality discriminator; it does not claim that m=1 occurs in the factor loop. Only I23:K4M9_REQUESTED_P128's Ok(true) versus Ok(false) qualifies; setup, arithmetic refusal, compiler and unrelated failures do not. The later companion is not claimed reached after a mutant panic.

M32's unchanged SPRING-CARRIED model has two nodes, eleven constrained DOFs and exactly free global DOF9. Its spring provides the stored diagonal used by Structure. The existing constructed helper replaces the free/free K entries with the scalar16; it does not execute the fixture's physical solve or use its expected solution. Unlike path_source(1), this carrier actually has the required sparse entry.

The source's radix rule gives exponent4 and scale exponent-2, hence S=1/4 and Ktilde=[1]. Scalar factorization and inverse action are exactly1. Hager–Higham starts x=y=z=[1]; z*x=1 ends the first loop with estimate1. The independently rounded RN128(2/3) alternating safeguard is below1 and cannot change it. Both norm*estimate products1 and16 are below2^127; therefore condition returns exactly1 with the correct norm and exactly1/16 with the injected unscaled norm. Only I23:K4M32_EQUILIBRATED_RCOND's exact numeric mismatch qualifies. Free-DOF, diagonal, scale, factor, inverse-action assertions and any Condition/Structure/ZeroDiagonal failure are setup checks, not kills.

The old full baseline cannot cover newly appended tests: neither new filter occurs there. Each must compile and run exactly one passing test with the proposed unmutated overlay before its corresponding fault experiment. No broad suite repeat is needed to establish those two new filtered baselines.

## Exact citation correction

Fifteen source-warrant excerpts match their declared source lines exactly. SOURCE_WARRANTS.json's factor_tests_namespace entry instead cites824–827 and contains only a newline, beyond the frozen factor.rs EOF. The actual namespace is819–821:

    #[cfg(test)]
    #[path = "../../../tests/retained_k4/factor_tests.rs"]
    mod tests;

NAMESPACE_CORRECTION.json binds the actual source and excerpt hashes. ROOT directed this additive correction while preserving the proposal seal. The namespace and helper signatures were checked in the real source; readiness does not rely on the blank excerpt. This is a citation error, not a code or numerical change.

## M11 remains one existing reach experiment

No M11 test, trace, hook or overlay is proposed. The only existing filter is the_p_plus_64_residual_takes_one_correction_on_two_span_and_records_its_basis. It must reach the exact (precision,residual_basis,corrections) tuple [(128,192,1),(256,320,1)]. Selection/certificate failure, attempts.len()==2 failure, compiler failure or empty selection supplies no kill. An unreached tuple preserves the raw result and returns the observed stop; it does not automatically authorize a new diagnostic.

That original filter has one passing occurrence in the preserved full baseline and can reuse it only with the exact unoverlaid source/test/profile. This says nothing yet about mutant reachability.

## G3 metadata and baseline reuse

Verified committed metadata at8939b4495795ccbf14c09195e1d294f7928f8d8a, manifest SHA5194a1b926f01b1bc4372ec6771a76a75e2d80fbeb000a3b93855268113837f6. Every field inherited from remaining_08's52 ready entries is unchanged except the preparation status. All patch hashes, complete changed-file pre/post hashes and filters agree; corrected M37 is bound to its additive patch, not the old invalid None-floor version.

The manifest correctly contains52 physical mutants,53 mutant filter invocations and53 untouched returned-control invocations. There are33 unique initial filters. Each appears exactly once as a passing test in fk_baseline_03's preserved stdout. Its per-target counts independently total456 passed,0 failed,1 ignored. Raw/portable log hashes and all eight physical test binary hashes match; both recorded fingerprints retain empty features/rustflags.

The entire core Git tree, not only adaptive.rs, is byte-identical between baseline head9562747f6a8edc49608d9b7c64e1411ad3d5a9af and frozen40129a225d73860ac2a53da9a2fa73869df668f3. G3's required lib and S11 baseline binaries have their exact recorded hashes. All23 compile-time include_str source inputs of S11 were resolved under core and individually hashed against both revisions; G3 correctly assigns full-core source scope to K4-M31. G3_BASELINE_CHECK.json preserves these bindings.

This evidence supplies the initial baseline for that exact metadata scope without a redundant broad rerun. Before execution, the owner still must bind each disposable archive, verify containment and exact intended postimage, independently release it before compilation, pin actual mutant binary/features/environment, inspect the required semantic failure and run every untouched returned control. The S11 mutant must be freshly compiled from its fully bound core-source inputs; reusing the unmutated S11 binary as the mutant would not exercise the changed scan input.

Baseline reuse does not cover changed source, a different profile or an added overlay. The fresh M9/M32 overlay baselines above are distinct. G3 metadata creates no archive or runtime authority. G1 active results, later invalid/repaired attempts, V-K runtime, consumer gates and exact-final-candidate acceptance remain outside this check.
