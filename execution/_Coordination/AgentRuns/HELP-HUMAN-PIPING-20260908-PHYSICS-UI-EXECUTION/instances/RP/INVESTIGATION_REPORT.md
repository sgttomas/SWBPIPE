# RP independent pressure-packet review

Verdict: **PASS**.

No actionable mathematical error, source-to-claim failure, unsupported adoption claim, or unsupported schema claim was found in the frozen P5 pressure-reference packet when `INVESTIGATION_REPORT.md` is read together with controlling `SIGN_CONVENTION_CLARIFICATION_V2.md`. This verdict establishes independent review adequacy for root fan-in only. It does not adopt the candidate pressure model, approve engineering use, accept a public schema or migration, close pressure physics, rule a dependency, or make a lifecycle/DAG decision.

## Frozen inventory

- P5 instance manifest: `172c96a6af9fbeaec30c4c974ed0614cbdfc0632061dcc127efea104cd90e52f`.
- DEL-05-03 run-record manifest: `07692034d597768ea79864540a08aa3f3d959036ddff7bbc26466643fe2c75f3`.
- Transitive reviewed inventory: 17 files total — both manifests plus their 15 declared members.
- Canonical inventory file: `REVIEWED_INVENTORY.sha256`, with lines sorted by repository-relative path and formatted as `<sha256><two spaces><path><LF>`.
- Full reviewed-inventory SHA-256: `6676c17a5b8cea47e09a1bcedd13261f5ad9fa45ac608c117aa966c048c5579d`.

Every declared member matched both its recorded byte count and SHA-256. All packet JSON parsed, packet files used LF, HEAD and the bound base were both `779dedb8670625b36af07b89fc5557470e47c50e`, and the three inspected production files had no diff from that base.

## Technical result

The Lamé fields and generalized-plane-strain constitutive reduction are correct under the stated homogeneous, isotropic, linear-elastic, axisymmetric straight-cylinder assumptions:

`A=(pi ri^2-pe ro^2)/(ro^2-ri^2)=Pc/As`, `sigma_r=A-B/r^2`, `sigma_theta=A+B/r^2`, `sigma_r+sigma_theta=2A`, `Nw=E As(epsilon_z-alpha DeltaT)+2nu Pc`, and `S=Nw-Pc=Nw+peAe-piAi`.

The four required free bodies close consistently:

- free closed: `Nw=P`, `S=0`;
- restrained closed: `Nw=2nuP-EAs alpha DeltaT`, `S=-(1-2nu)P-EAs alpha DeltaT`;
- pressurized free barrel with remote closures or open-end compensation: `Nw=0`, `S=-P`, with each remote closure support carrying `P`;
- mixed thermal/pressure: the same linear relations with free thermal strain or the restraint term `-EAs alpha DeltaT`.

The remote-closure case correctly changes wall-force transfer without deleting the `-piAi` term from effective force at a still-pressurized cut. The physical vent-to-ambient limit is separately stated as local `p,P,S -> 0`.

The additive sign clarification is coherent with the source and removes the report's ambiguous labels. For local `x:i->j` and tensile `N>0`, `[-N,+N]` is the node-on-element, stiffness-conjugate vector; element-on-node action is `[+N,-N]`; and the tension-positive cuts are `Ncut_i=-Fi_x`, `Ncut_j=+Fj_x`. Current `Kd-f_equiv` recovery and the fixed/fixed `[+P,-P]` reported pair therefore map to `S=-P` as stated.

Exact annulus and thin-wall quantities are kept distinct. The packet uses physical bore thrust `P=p pi ri^2` and wall area `As=pi(ro^2-ri^2)`; it identifies the mean-radius cap proxy as an approximation and does not reuse expansion-joint effective area as wall membrane area.

The material claim is calibrated: current inputs provide `E`, `G`, and optional `alpha` at base and temperature points, but no `nu` or isotropy/precedence contract. The conditional identity `nu=E/(2G)-1` is therefore presented only as a possible homogeneous-isotropic derivation, while the PRD's Poisson-ratio requirement is correctly described as an unwired implementation/schema gap rather than an accepted new field design.

Static source observations were confirmed: ordinary straight pressure thrust uses `pAi`; mapped expansion joints may use a separately authored effective area; the assembled straight pair is `[-P,+P]`; recovery subtracts thermal and pressure equivalent loads; straight station recovery transforms the i-side action to the j-side cut; pressure-longitudinal rows are suppressed while pressure thrust is active; the generic stress crate uses the mean-radius thin-wall formulas; pressure loads lack closure topology; and curved-bend treatment remains a separate limited model. These are observations of current source, not production changes.

## Reproducibility

- `python3 calculate_reference.py | cmp - REFERENCE_RESULTS.json`: PASS byte-for-byte.
- Independent JavaScript arithmetic, without importing the packet calculator or production code, reproduced `ri=0.077 m`, `As=0.003540574920595698 m^2`, `P=18626.5028431339 N`, `P/As=5.26086956521739 MPa`, Lamé hoop stresses `11.5217391304348/10.5217391304348 MPa`, free-closed displacement `0.0210434782608696 mm`, remote-closure/free-barrel displacement `-0.0315652173913043 mm`, restrained `Nw=11.1759017058803 kN`, thermal restraint `637.303485707226 kN`, and mixed restrained `Nw=-626.127584001345 kN`, `S=-644.754086844479 kN`.
- Independent boundary checks reproduced `sigma_r(ri)=-p`, `sigma_r(ro)=0`, and `sigma_r+sigma_theta=2A` at both wall surfaces.
- The calculator imports only Python standard-library modules `json` and `math` (plus `__future__` annotations); it imports no production source.

No full harness, DEC-025 sweep, native build, or production solver was run.

## Source calibration and limitations

The MIT primary publications directly support closed-end/open-end equilibrium, Poisson coupling, exact annulus axial stress, generalized Hooke/Lamé relations, and the stated thin/thick-cylinder limitations. Current SIMULIA documentation directly supports closed-end pressure loading, internal cancellation, explicit open-end compensation, and `ESF1=SF1+peAe-piAi`. These publications support the candidate derivation; they do not make it accepted Chirality engineering authority.

Nonblocking currency note: the exact EXT-04 Abaqus 2023 beam-library URL in `SOURCES.json` returned HTTP 404 during RP review. The identical ESF1 definition was directly verified on the publisher's current 2025 Beam Element Library page reached from the live EXT-03 Distributed Loads page. The claim remains supported, so this does not require packet churn; a future source refresh can update the stale year-specific URL.

The numerical oracle is limited to the explicit straight, homogeneous, isotropic, small-strain linear-elastic and uniform-temperature assumptions. It does not establish material validity at the worked stress, approximation thresholds, accepted tolerances, bend/EJ extension, public compatibility, or default boundary topology. Those choices remain Owner-gated exactly as the packet states.

## Execution-evidence limitation

The RP durable launch mirror is retrospective. A first persistence attempt targeted the sibling checkout because the patch tool used the caller's default working directory; the two accidental files were immediately removed and no persistent sibling change remains. The correct-lane `LAUNCH_BRIEF.md` was persisted after substantive review had begun. It records the parent-supplied scope actually followed, but it is neither pre-launch nor pre-review durable evidence. RP made no subject-matter edit and wrote only `instances/RP/**` in the required execution lane after cleanup.
