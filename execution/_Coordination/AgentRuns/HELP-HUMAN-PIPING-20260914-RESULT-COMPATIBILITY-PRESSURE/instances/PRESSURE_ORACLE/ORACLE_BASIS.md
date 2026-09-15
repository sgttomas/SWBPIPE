# Frozen independent pressure-kernel oracle basis V1

Status: PASS for independent analytical derivation and oracle self-verification. No production function, solver, or maintained test has been read or executed. Frozen on 2026-09-14 by the root-dispatched PRESSURE_ORACLE TASK, before any implementation observation. Root owns acceptance and subsequent independent implementation refutation.

## Authority and claim boundary

This is a derivative verification package of the selected accepted pressure design supplied by root: PRIVATE_PRESSURE_KERNEL_V1.md, PRESSURE_CONTRACT_V1.md and the pressure portion of ANALYTICAL_ORACLES_V1.json from HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D. SOURCE_MANIFEST.json binds exact upstream bytes and doctrine origins. Supplied integration baseline: eff9a58dd712ff9673fa26b9fa809a2f725f6f97. Historical source-design basis: 8f27fa3d8ec5e128e61fd3ac4076e74d7955f355. Acceptance of that design is parent-supplied context; this child does not create governed acceptance.

Bounded scope: DEL-05-03, PKG-05, SOW-015, OBJ-003. Applicable contract invariants are OPS-K-MECH-1/2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-2/3 and the brief's agent instruction boundary. The deliverable _CONTEXT retains older revision 0.7 setup references; this undertaking does not adopt them over the parent-supplied formal 0.12/SCA-009/DAG-010/R5 basis. No decomposition, lifecycle, engineering acceptance threshold, runtime pressure profile or result schema is changed by these files. F-PIP-2 / DEC-081 fence remains. All examples are synthetic. No protected corpus/OCR equations, network, native/app, Git operations or product builds were used.

## Independent derivation

Write the radial and hoop stresses as a-c/r² and a+c/r². Imposing radial traction -p at ri and zero at ro solves the two equations as c=p/(1/ri²-1/ro²), a=c/ro². Their sum is 2a independently of r. Integrating uniform axial stress over the annulus gives As=π(ro²-ri²); the fluid end section has Ai=πri². Isotropic axial compliance gives εz=σz/E-nu(σr+σh)/E+thermal_strain. Hence σz=E(εz-thermal_strain)+nu(σr+σh), Nw=As σz and S=Nw-pAi. This reconstructs the contract through boundary tractions and Hooke law rather than importing a production helper.

For ri=1, ro=2, E=120, nu=1/4 and p=3: Ai=π, As=3π, P=3π, EA=360π, G=48, a=1 and c=4. The inner/outer radial stresses are -3 and 0 Pa; inner/outer hoop stresses are 5 and 2 Pa; transverse stress trace is 2 Pa.

| Case | thermal strain | axial strain | Nw/π | S/π | axial stress Pa |
|---|---:|---:|---:|---:|---:|
| P1 free transferred closures | 0 | 1/240 | 3 | 0 | 1 |
| P2 restrained transferred closures | 0 | 0 | 3/2 | -3/2 | 1/2 |
| P3 free separately supported closures | 0 | -1/240 | 0 | -3 | 0 |
| P4 free thermal + pressure | 1/1000 | 31/6000 | 3 | 0 | 1 |
| P4 restrained thermal + pressure | 1/1000 | 0 | 57/50 | -93/50 | 19/50 |

The free transferred state has Nw=P and εz=thermal+(1-2nu)P/(EAs). The free wall with separately supported closures has Nw=0 and εz=thermal-2nuP/(EAs). The constrained state has εz=0. These are prescribed algebraic states in the kernel fixture; they do not establish a successful supported solve.

## Exact signs and identities

With positive local i-to-j tension, recovered node-on-element wall actions are [-Nw,+Nw]. The end cuts [-qi,+qj] are both Nw. The pure mathematical cap pair is [-P,+P]. The external equivalent eigenload RHS is [-EAs ε0,+EAs ε0], where ε0=thermal-2nuP/(EAs). Its i entry is +3π/2 in P1/P2/P3 and +57π/50 in P4. For a prescribed uniform axial strain Kd=[-EAs εz,+EAs εz], so Kd-f_eigen equals the wall action pair exactly. Do not subtract the cap pair again. Mechanical fixed-end equivalents remain a distinct term at later recovery integration.

Terminology clarification: PRIVATE_PRESSURE_KERNEL_V1's phrase "pure local node-on-element mathematical pairs" cannot make an applied eigenload into the recovered wall action. PRESSURE_CONTRACT_V1 §4 explicitly defines f_eigen as external equivalent RHS. Pair ordering and numbers are fixed; names/documentation must preserve applied-versus-recovered meaning.

In P2 the support-on-vessel pair is [+3π/2,-3π/2], balancing wall actions and transferred caps. In restrained P4 it is [+93π/50,-93π/50]. In free P3 the pipe-support pair is zero while separate remote closures require [+3π,-3π]; that remote support is outside the pipe solve. These are independently derived balance requirements for later solver tests, not additional pure-kernel outputs.

For every accepted scalar input, Nw=As σz; S=Nw-P; eigen_i+eigen_j=0; cap_i+cap_j=0; σr+σh=2P/As. Pressure and strains superpose linearly at fixed geometry/material. Pressure reversal changes all pressure contributions' signs. At p=0 the pressure terms vanish while thermal force remains. At nu=0 the Poisson eigenload vanishes while cap force and Lamé stress remain. Auxetic -1<nu<0 is allowed. Derived G is always E/[2(1+nu)]. The independent midpoint example E=180, nu=7/24 requires G=2160/31; 69 is the forbidden independently interpolated G.

## Frozen floating-point comparison policy

Binary64 inputs are promoted exactly with Decimal.from_float. Random cases use deterministic dyadic inputs (seed 20260914) and 110-digit independent boundary reconstruction, never values obtained from production. A 110-digit π literal supplies mathematical area/force expectations. Rational/pi cases are also checked exactly using Fraction; converting 1/240 and 1/1000 into f64 inputs introduces only their normal input rounding, which the declared arithmetic scale covers.

Every finite scalar comparison uses abs(actual-expected_decimal) <= 128*2^-52*declared_scale + 8*2^-1074. The scale is frozen per scalar in FROZEN_EXPECTATIONS.json: areas and G use their magnitude; force/stress sums use the sum of magnitudes of the independently derived physical terms, including EAs εz, EAs thermal and 2nuP, with |P| added for S. This handles cancellation without an arbitrary absolute 1-N or 1-Pa floor. NaN/infinity always fail. Missing scalar/case data fail. No relative error division by zero is used.

Radial traction boundaries deliberately use |p| as scale, not the potentially huge Lamé terms. Therefore an adjacent-radius annulus returning radial stress zero at the inner wall fails for p=3 even if its hoop stress is enormous. Endpoint radial stress must satisfy the boundary traction at pressure scale. Interior radial/hoop sums use their term scale because their independent terms may cancel. The multiplier 128 is a conservative bounded-operation floating arithmetic allowance, not an engineering fit, convergence, material, geometry or acceptance tolerance. The design's separate 64*epsilon geometry guard belongs to future topology conversion; it is not a minimum wall thickness and is not substituted into these scalar checks.

Correctly rounded subnormal arithmetic is accepted within the explicit 8-subnormal-ULP floor. Geometry areas and G are still required to be strictly positive; an underflowed zero fails these constructor contracts. No new universal rejection of nonzero underflow in signed force/stress is imposed: signed outputs use the same bounded subnormal comparison, while nonfinite derived outputs must return errors. This distinguishes representability/rounding from an invented physical cutoff.

## Coverage and failure criteria

The executable oracle has 114 valid input cases: five exact states covering the four design cases; thirteen named reductions/numeric cases; 96 deterministic varied cases. Self-verification has 6,592 sign/scaling scalar checks and 1,648 independent load/strain superposition checks. Mutation witnesses reject omitted/doubled/flipped Poisson contribution, cap subtraction from wall force, mean-radius fluid area, double longitudinal stress, omitted fluid effective term, doubled cap, reversed eigen pair, independent G interpolation and lost thin-wall inner traction. They are analytical mutation witnesses, not claims that actual product mutations were run.

INVALID_INPUT_EXPECTATIONS.json freezes required constructor/function errors for invalid radii, nonfinite values, E/nu boundaries, areas/G underflow or overflow, outside-wall radius and true derived load/stress overflow. Err is required; a finite-looking fallback, default, clamp or saturated infinity substitute fails. Exact error enum names are implementation detail unless separately required by root.

A representable arbitrarily thin annulus is allowed: ADJACENT_RADII uses ri=1, ro=nextafter(1,+inf), p=3 and finite output stresses. As must avoid subtracting separately rounded squares; the design prescribes (ro-ri)*(ro+ri). No broad conditioning wall-thickness threshold is authorized. SMALL_GEOMETRY and LARGE_GEOMETRY span binary exponents ±200 without mathematical overflow, so ratios/load terms must not spuriously lose the specified outputs. Finite signed pressure, including zero and negative values, is mandatory.

## Implementer handoff and deferred work

Root may copy/consume FROZEN_EXPECTATIONS.json and the independent script into maintained verification. A product adapter may read inputs and map outputs into {case_id:{quantity:value}}; running independent_oracle.py --check-actuals <file> performs comparison without importing product code. Each radius_witnesses member is an additional lame_at_radius call. Constructor rejects are separate cases. Do not replace expected values with product output or enlarge tolerances after seeing failures. Return discrepancies to root against this frozen basis.

The requested implementation stays private/dormant. Pure coverage does not validate member orientation mapping, region cap cancellation, supports or singularity, topology, mechanical distributed-load recovery, interpolation dispatch, material precedence, DTOs/defaults/versions/migrations, units at public boundaries, row identities/hashing/persistence, dense/sparse parity, solver convergence or native behavior. Those remain explicit later gates. Root owns 100% diff review, source currency, maintained build checks, native witnesses, full sweep, final receipt/publication and combined acceptance.

Closure: this independent analytical sub-scope is complete and ready for root acceptance; product refutation is pending a separately supplied source-frozen follow-up. The package is derivative, has no governing pointer update, and retains the accepted upstream design as authority. Rerun this script and check this freeze's hashes before consuming it. Reopen only on an accepted upstream change or independently demonstrated oracle defect; any correction creates a successor evidence version rather than silently changing this freeze.
