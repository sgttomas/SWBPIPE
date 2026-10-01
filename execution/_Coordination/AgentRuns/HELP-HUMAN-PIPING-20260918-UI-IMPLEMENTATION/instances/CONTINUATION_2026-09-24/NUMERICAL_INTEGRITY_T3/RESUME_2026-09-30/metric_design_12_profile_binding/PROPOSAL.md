# Profile binding: reference arithmetic plus external executable qualification

**Recommend a named immutable reference-proof profile, with qualification of a
particular executable kept in the existing external archive/build/launch review.**
This replaces the unaccepted Verified*Profile factory sketch; it changes neither
the selected VR -> H library seam nor the ordinary CLI. ROOT's in-turn clarification
explicitly leaves a new blanket runtime refusal policy unselected. Proposal only,
for independent design review before implementation.

Paths below use P = projects/chirality-piping, H = P/core/solver/performance_harness
and VR = P/validation/benchmarks/numerical_robustness; all are repository-relative.

**Smallest carrier and precise meaning.** In the already proposed
H/src/k6/w1/envelope.rs, use distinct immutable ReferenceKernelProfile,
ReferenceHProfile and ReferenceVRProfile (names proposed), selected by named
proof-basis IDs, with private fixed facts and named constructors. They carry the
reviewed source/layout/request facts and their scope; they do not certify the
current executable. The current basis combines several separately scoped witnesses;
it is not one already qualified production artifact. No caller-chosen overhead,
unproved private mirror or generic zero is permitted. Use the same checked shared
arithmetic for reference tests and adapter composition. The returned proof_basis
identifies those mathematical premises, not a build attestation. Invalid or missing
descriptor/proof cells still return explicit errors; this choice supplies no zero
substitution, arbitrary case-domain extension or fallback admission subtotal.

Keep reference public-layout facts coherent with that named basis. Current
W1SizeFacts::of_this_build() (H/src/k6/w1/counts.rs:369) and
Sizes::of_this_build() (VR/src/scale.rs:231) establish only the public expressions
they actually evaluate. They neither qualify private requests nor authorize
mixing current-host public facts with a historical private profile under its old
identity. Tests of current public types remain distinct from reference arithmetic.

The associated qualification is an external, artifact-specific record in the
existing governed run packet/sidecar: reference IDs; exact source/archive and
lock inputs; actual compiler/std/target/features/profile/allocator/request-site
correspondence; binary hash; and exact launch/input/counts identities. The existing
planned sidecar carries this linkage without adding non-estimate fields to protected
numerical records. Embed only the small maintained reference fact set and IDs needed
by the estimator; do not make Rust source/tests read the execution tree. No build
script, environment token, runtime sidecar parser, self-hash or new host tool is
needed. A profile ID is an index into evidence, never evidence by itself.

**Where unknown builds stop.** Unknown or mismatched builds do not receive
ROOT's external qualification/reliance and measured-run authorization. They may
still execute ordinary CLI behavior and compute conditional reference-policy
numbers. Those numbers must not be described as proven heap bounds of that
executable without the external binding. This preserves the existing half-cap
predicate, caps, measurement windows, descriptors and required evidence. It does
not make qualified admission independent of the exact build/input/launch review.
Documentation at the estimator and entry points must say this explicitly.

This is procedural qualification, not automatic rejection of every unsupported
direct CLI invocation. H/runner/k6_runner.py:800–826 records supplied source IDs,
labels build_profile='release', queries a toolchain and hashes a binary; those
operations alone do not prove how that binary was built. VR/runner/vk_scale_runner.py:
269–277 reuses that metadata. The existing exact archive/build-command evidence and
ROOT's correspondence review must supply the missing connection. Do not rename
that metadata function an attestation or claim the current runners already enforce
it. Missing facts remain missing; seed/debug/M5 records do not qualify Linux or a
later ordinary-production binary.

**Protected tests and CLI.** H/Cargo.toml:12–19 deliberately builds k6_observe
without a libtest harness; H/tests/k6b_bin.rs:1–5,48–136,149–229 launches its ordinary
debug binary, requires numeric estimates, successful W1 solves/parity/prefixes and
the named half-cap refusal. A cfg(test) exception in library code does not establish
this subprocess's qualification. Keep all these assertions and ordinary argv;
introduce no bypass flag, environment secret, unknown-build early success, ignored
test or blanket platform skip. H/src/bin/k6_observe/main.rs:384–417,579–661 and
H/src/k6/counts.rs:308–324 remain the counts/backstop integration points.

H/tests/k6b_w1.rs:635–729 should test exact corrected reference equations and all33
committed count identities under the named reference profile. VR/tests/scale.rs
keeps all193 factored-case storage checks and its estimate relations; the shared
mathematics is tested against the declared reference facts. Such Linux tests prove
arithmetic/behavior under those premises, not Linux allocator/request equivalence.
Current-public-type assertions, allocator tests, overflow/missing-descriptor checks,
mutants and all existing semantic coverage remain; labeling reference arithmetic
is not permission to replace an executable-bound assertion with a weaker one.
VR/examples/vk_scale.rs:400–430 retains ordinary counts-only and half-cap behavior.

Hosted .github/workflows/piping-desktop-e2e.yml:171–195 runs ubuntu-latest with
Rust1.97.1 and excludes execution/. P/tools/ci/numerical_ci.py:26–47 retains locked
fetch and the actual offline cargo tests from P/tools/release/check_release_readiness.py:
145–160. VR/tests/feature_guard.rs keeps no-features and no-execution-source checks;
the seeded-fault suite remains fault-behavior evidence, never ordinary profile
qualification. No CI feature, compiler, test selection or success criterion changes
are proposed here.

**Remaining owner decision and verification.** ROOT should approve this explicit
reference/current-executable distinction, then have independent review check the
adapter names, field documentation and tests before implementation. Mathematical
profile materialization and the shared checked estimator can then proceed without
inventing a verified-current-build factory. Final ordinary-production transfer and
the sidecar's exact artifact/input/launch binding still precede reliance on a numeric
bound. No current compiled artifact is qualified by this proposal.

If ROOT instead requires every arbitrary direct CLI to refuse unknown builds,
that is a separate unresolved API/policy choice: the available source supplies no
trusted in-process carrier, and blanket refusal conflicts with the protected debug
subprocess semantics. The dependent step is entry-point admission integration, not
another envelope derivation. Do not implement that alternative or a workaround
under this proposal. No new proof programme or automatic follow-on is authorized.
