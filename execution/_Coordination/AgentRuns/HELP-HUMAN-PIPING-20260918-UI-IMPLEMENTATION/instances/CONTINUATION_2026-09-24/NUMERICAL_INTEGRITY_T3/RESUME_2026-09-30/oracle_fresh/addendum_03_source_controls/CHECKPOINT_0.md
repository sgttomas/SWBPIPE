# Addendum 03 — three independent source-control truths, frozen before outputs

Status: valid pinned source definitions; exact truth and input identities frozen
for exactly EXTRA-FM-01, EXTRA-MF-01 and EXTRA-ZR-01. **116 rows** (40, 40, 36).
No solver selection, runtime constructor execution, repair result, product/native
reachability, or design closure is asserted.

TRUTH.json SHA256:
`aac399bfb68bbb8b1cbcce08cc2901229535e88860dc5cc812cf08c9fe3ba0ee`.
Input-only A1_EXTRA_INPUTS.json SHA256:
`546b3643b496db9bb259694baf638153b535a55ff87d9d775b27cfcb61a38efe`, read from
coordination commit `331b535048daa5fbdb045ee450a242f5ae997714`.
The mathematical source basis is `d01ad98a754698631f927709d08284c272de85e8`.
SOURCE_READS.json records the consulted source hashes. This additive work leaves
all prior oracle, expectation and seal bytes unchanged.

## Constructor and identity checks

The pinned PrimitiveSource::new checks nonempty finite nodes; member IDs/nodes;
positive finite E,G,A,Iy,Iz,J; non-subnormal derived A,Iy,Iz,J; finite nonparallel
y_reference; unique spring IDs and positive finite stiffnesses; constraint DOF
uniqueness and finite values; and finite loads with nonempty source IDs. Every
input meets these conditions. The two nodes differ along +X by the positive
normal power of two L, and y_reference=(0,1,0) is nonparallel. All constraints
are exact zero. Empty directional-spring, station and support-group lists create
no additional checks. Large coordinates are not rejected by this constructor;
this does not establish acceptance on a separate product capture route.

**Three global springs sharing a DOF are valid.** source.rs sorts springs by ID
and rejects equal IDs, not equal DOFs. IDs 1,2,3 are distinct; all three stiffnesses
are +1. assemble.rs retains a separate Contribution::Spring for each ID in the
same diagonal matrix entry. recover.rs emits each separate -k*u action. No spring
is deduplicated or replaced with a synthetic combined source. Member ID 1 and
spring ID 1 belong to separate namespaces. All listed sources are valid; no input
was repaired or changed.

Each case carries both a canonical primitive-JSON SHA256 and independently
constructed canonical K4SRC/K4STF bytes plus SHA256. The encoding follows
source.rs: little-endian integers/raw binary64 bits, member and spring ID order,
constraint DOF order, and loads sorted by DOF/source-ID bytes/value bits. The
source identity includes the three separate spring IDs and the authored load
source ID. A separate cursor decoder reconstructs all primitive fields from
K4SRC bytes and matches the input JSON exactly. This is source-contract identity
reconstruction, not an executed Rust constructor witness.

| Case | Canonical K4SRC SHA256 | Free DOFs |
|---|---|---|
| EXTRA-FM-01 | c828745ec22a8e9417b43d773ba073405dbb3adb88863ed305b49ee2527646a2 | 6 |
| EXTRA-MF-01 | c04fe5b5a74554e809a97010e9795f94bee8a1a52221c2e6906488d16378b69b | 9 |
| EXTRA-ZR-01 | 4436d5bc9dfe48530ea52001a83cdd3cc09e3feaf764e6128cd7c11b202130eb | 6,9 |

Full stiffness identities and canonical JSON identities are in SUMMARY.json and
TRUTH.json. Raw JSON is preserved under inputs/ without transformation.

## Independent exact equations, signs and rows

Let h=2^-1074. Axes equal the global axes exactly, so the only free mode equations
are axial and/or torsional. D1 and source formation give a=EA/L and t=GJ/L. The
separate springs add their positive stiffnesses to the corresponding diagonal.
All bending DOFs are constrained to zero, all bending basic deformations vanish,
and no bending coupling contributes to any remaining row.

**EXTRA-FM-01.** L=2^100, A=2^100, E=G=1, J=1. Thus a=1 and t=2^-100. The sole
free axial equation is (1+1+1+1)*u_6=5h; hence u_6=5h/4 and N=a*u_6=5h/4.
Nonzero truths are:

- D:6 and M:1: +5h/4.
- E:1:I:0 and R:0: -5h/4; E:1:J:0: +5h/4.
- S:1:0, S:2:0, S:3:0: each -5h/4.

All other rows are exact zero. Springs act on the model with negative sign;
local member end actions are node-on-member, so their signs are -N at I and +N
at J. External balance is 5h-3*(5h/4)-5h/4=0.

**EXTRA-MF-01.** L=2^-100, J=2^-100, E=G=1, A=1. Thus t=1 and a=2^100. The sole
free torsional equation is (1+1+1+1)*u_9=5h; hence u_9=5h/4 and T=t*u_9=5h/4.
Nonzero truths are:

- D:9: +5h/4.
- E:1:I:3 and R:3: -5h/4; E:1:J:3: +5h/4.
- S:1:3, S:2:3, S:3:3: each -5h/4.

All other rows are exact zero. Both node displacement magnitudes are zero;
those magnitudes include translations only, not rotation. External torque
balance is again 5h-3*(5h/4)-5h/4=0.

**EXTRA-ZR-01.** L=2^100 and A=J=2^102, so a=t=4. There are no springs. Free
DOFs are 6 and 9; their uncoupled equations are 4*u_6=5h and 4*u_9=0. Thus
u_6=5h/4 and u_9=0. Nonzero truths are:

- D:6 and M:1: +5h/4.
- E:1:I:0 and R:0: -5h; E:1:J:0: +5h.

All other rows are exact zero. In particular D:9 is a **free**, non-input-derived
Rotation row with exact zero truth and positive torsional stiffness. It is not a
prescription or a torsional mechanism.

All cases contain 12 displacement/rotation rows, 2 translation magnitudes and
12 member-end rows. FM/MF add 3 spring rows and 11 constrained reactions, giving
40 rows each. ZR adds no springs and 10 reactions, giving 36 rows. Every key,
kind, body 0, InputDerived membership, sign and exact truth is listed in TRUTH.
No constrained reaction exists at a free DOF. Each spring produces its own key.

## Range, direct rounding, and intended coverage

Every nonzero truth is representable as a binary64 value after rounding; none
underflows to zero or overflows. The values +/-5h/4 round to +/-h; +/-5h are
already exact. Exact zero rounds to +0. The independent integer-bit bisection
rounder provides each row's exact rounding-cell endpoints and tie inclusion.
All 116 direct roundings also agree with separate CPython Fraction-to-float
conversion. O9 excludes InputDerived rows only here; no direct-truth row is
Unpublishable. Direct bits are baselines, not predictions of solver output.

Using the pinned scale definition with direct-rounded rows and no selected-512
floor:

| Case | Intended kind | Exact truth-coupled scale | Direct-rounded coupled scale | Loss |
|---|---|---|---|---|
| EXTRA-FM-01 | Moment, from L*force | (5/4)*2^-974 | 2^-974 | 1/5 relative |
| EXTRA-MF-01 | Force, from moment/L | (5/4)*2^-974 | 2^-974 | 1/5 relative |
| EXTRA-ZR-01 | Rotation, from translation/L | 5*2^-1176 | 0 | complete rounding to zero |

FM's primitive nodal load is 5h, but it is not itself a recovered output row:
member, reaction and separate spring force rows each have magnitude 5h/4. Thus
the intended force operand of the coupling has the subnormal rounding gap.
The same reasoning holds for MF's recovered moments. ZR supplies a genuine
free-zero-rotation recipient for the zero published rotation scale; its D:9
truth is exactly zero, so the zero-bound predicate is honest on exact output.
This coverage does not predict selection, a mutant kill, or an unmutated failure.

## Comparison interface and limits

source_controls.py accepts only these three case IDs and the existing
`a1-public-tsv-v1` grammar. For a selected output it requires exactly one supported
SELECTED p/2p pair, the full exact ROW/LAYOUT key set, four scales, properly
shaped p=512 floors when applicable, finite canonical bits, and exact matches to
independently derived SOURCE_ENCODING, SOURCE_ENCODING_SELECTED and
STIFFNESS_ENCODING. Rejected or unresolved statuses never become accuracy passes.

The comparator checks exact truth against each **reported** row claim, using the
existing claim factors frozen in the original CHECKPOINT_0: absolute b*(1+2^-22)
or at 512 b*(1+2^-21), public relative 1e-9 with the published-value denominator,
the separately named sharper scale-relative allowance, and exact prescriptions.
It reports both exact-rational and source-binary64 allowance predicates, direct
bits, and range outcomes. It imports only our own frozen exact/rational helper;
it does not import any response oracle, fixture expectations, or implementation.
The new truth hash is checked before comparison.

**Deliberate boundary:** the existing pinned direct class/scale/bound fields in
TRUTH.json are labeled baselines. This additive checker does not certify a
repaired scale or bound algorithm by comparing it to an obsolete pre-repair
construction. A row passing its reported bound does not establish that the
construction of that bound is authorized or correct. ROOT must separately
supply the accepted repair contract and its source/design review; p=512 floor
formation, receipts and unit conversion are likewise outside this interface.
No repaired output has been received or read at freeze.

Exact CLI from the A1 checkout (R is the resumed-run repository-relative path):

```sh
<VENV>/bin/python -B "$R/oracle_fresh/addendum_03_source_controls/source_controls.py" compare <released-output.tsv> <new-owned-report.json>
```

Exit 0 means applicable truth-vs-reported-claim checks passed; 1 means a numerical
finding; 2 means malformed input, wrong source identity, or changed frozen truth.
Keep candidate/binary/guard admission and raw output provenance with each result.
A false unmutated publication requires immediate BLOCKING return under COMMON.
For reproduction in an empty owned destination:

```sh
<VENV>/bin/python -B "$R/oracle_fresh/addendum_03_source_controls/source_controls.py" freeze <empty-owned-destination>
<VENV>/bin/python -B "$R/oracle_fresh/addendum_03_source_controls/verify_packet.py" <new-empty-owned-scratch> <new-owned-verification.json>
```

## Actual checks, read order and provenance

Agent `/root/a1_oracle_fresh`, TASK, directly commissioned by ROOT. Same pinned
Root/TASK/Piping and A1_ORACLE_FRESH/COMMON instructions as original checkpoint.
No delegation, new role body, selected workflow or skill body. Interpreter was
ROOT's existing `<VENV>/bin/python -B`, Python 3.13.14; no installation.

1. 975d84: read only input JSON with `GIT_OPTIONAL_LOCKS=0 git show` at 331b535.
2. 6e607c / 258d9e: parallel read-only source.rs 280–630 and recover.rs 97–200
   (requested recover 550–680 had no content). Constructor and layout only.
3. 2a80c5 / 31f58a: source.rs 645–880 (canonical identity) and recover.rs 205–470
   (equations, signs, individual spring recovery, reactions).
4. 748a31: assemble.rs 360–435 and 550–670; verify separate spring contributions.
5. f33d8d: copy committed input bytes into owned inputs; hash consulted source
   blobs; record VENV version. No implementation or test-plan content read.
6. 7b2cf4: independently implement the three scalar-mode equations and identity
   encoding; freeze TRUTH.json, 3 cases / 116 rows.
7. 7f0c14: run verify_packet.py: identical truth regeneration in owned scratch,
   separate identity decoder, exact free/global force and torque balance,
   all 116 secondary rounding checks, and three synthetic exact-output interface
   checks. Those synthetic outputs are not solver observations or new sources.
8. 72ab2e: add the now-known frozen truth hash check to the comparator and repeat
   all three synthetic interface checks; no numerical truth changed.
9. Seal: verify all prior manifests and their files, write this checkpoint and
   new inventory. All prior byte identities are preserved.

VERIFICATION.json and SUMMARY.json retain executed check results and identities.
No TEST_AND_MUTANT_PLAN equations, new implementation/test code, or new numerical
output was read. No Rust/solver, host tooling, Git/index mutation, source edit,
or case expansion occurred. Await ROOT's output release after this freeze.
