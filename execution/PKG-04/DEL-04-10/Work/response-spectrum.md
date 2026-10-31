# F2 response-spectrum analysis: governing design

**Status.** This is DEL-04-10's governing design for F2, response-spectrum seismic analysis with uniform support motion. Scope entered by SCA-014 (owner, 2026-10-28); Agent 0 placed it in DEL-04-10 on 2026-10-31. This revision fixes the method and its boundaries so that the references and the screening class can be derived first. The product-side layout (§6) is settled when the build slice starts, after DEL-04-09's dense path, views and sparse path. Nothing is built. This design makes no acceptance, issuance or lifecycle statement (F-PIP-3). Standard claim fence applies (claims taxonomy per DEC-081).

**Labels.** **[decided]**: settled here, with the reason. **[open]**: listed in §8, with its owner.

---

## 1. Basis

- **[decided] The modal basis is DEL-04-09's.** F2 consumes a published modal result: f_i, ω_i, φ_i in its normalization case, m_i, Γ_ik, m_eff,ik, cluster ids and the result's standing. It reads DEL-04-09's mass model for the missing-mass load. It forms no mass and solves no eigenproblem of its own (AX-002).
- **[decided] The modal request for a spectrum solve** includes dynamic-only restraints (§4) and, if the model has nonlinear supports, names the solved case at which DEL-04-09 linearizes them. Both are DEL-04-09 request inputs; F2 adds no support state of its own.
- **[decided] Routes.** PF and EX both, as far as the modal basis admits them. Static recovery (element forces and moments, support loads) uses the route's own recovery from a displacement vector, with no load term.

## 2. Modal responses

For direction k (global X, Y or Z), spectrum S_k(f) and scale s_k (dimensionless):
- spectral acceleration of mode i: A_ik = s_k·S_k(f_i), interpolated by the spectrum's stated rule (§3.1);
- modal displacement: u_ik = Γ_ik·A_ik/ω_i²·φ_i. This is normalization-invariant: Γ_ik·φ_i does not depend on the scaling of φ_i, in either normalization case (DEL-04-09 Design §1.5, D-d);
- recovered quantities: nodal translations and rotations, element end forces and moments in member axes, support loads, each from u_ik by the route's static recovery with no applied load.

Every quantity is formed per mode and per direction and kept signed until combination; only combination makes it a magnitude.

## 3. Combinations and the missing-mass correction

### 3.1 Spectrum interpolation

- **[decided] Where spectra live.** Spectrum tables are top-level `spectra[]` records of model document 0.9.0 (DEL-02-01 Work `model-document-0.9.0.md` §2.4): abscissa (frequency or period), ordinate (acceleration, or a g multiple with a required g value), damping ratio, interpolation and points, entered once and referenced only by the spectrum request. Directions, scale factors and method choices stay on the request. Until F2's slice, 0.9.0 labels a displayed spectrum "not used by the solve in 0.9.0".
- **[decided]** The user chooses the interpolation per spectrum: linear or logarithmic on the abscissa, and linear or logarithmic on the ordinate. Logarithmic interpolation requires positive values, which REQ-001 already requires.
- **[decided]** No extrapolation: a mode with f_i outside the tabulated range is refused by name (`SPECTRUM_FREQUENCY_OUT_OF_RANGE`, naming the mode and the spectrum). A spectrum is interpolated on the abscissa it is entered in: a spectrum given against period is evaluated at T_i = 1/f_i. Logarithmic interpolation gives the same value on either abscissa, since log T = −log f; linear interpolation does not, so the entered abscissa governs.

### 3.2 Modal combination

Applied to each response quantity R separately, with R_i the signed modal value of mode i in one direction:
- **SRSS:** R = √(Σ R_i²).
- **CQC:** R = √(Σ_i Σ_j ρ_ij R_i R_j), with the Der Kiureghian (1981) correlation for equal modal damping ζ, r = ω_j/ω_i: ρ_ij = 8ζ²(1 + r)r^{3/2} / ((1 − r²)² + 4ζ²r(1 + r)²). ζ is the user's modal damping ratio, required for CQC.
- **Grouping of closely spaced modes:** modes are grouped by the ten-percent rule of US NRC Regulatory Guide 1.92, Rev. 1 (a public US-government document): consecutive modes whose frequencies differ from the group's lowest by at most 10 % form a group, and R = √(Σ R_i² + 2Σ_{i<j, same group} |R_i R_j|). The rule and its 10 % are stated in the result.
- **[decided] Why these three.** They are the established rules for piping spectrum analysis, and the vendor's public manual lists mode combination among its options (SCA-014's citations). Other rules (for example RG 1.92 Rev. 3's Lindley–Yow rigid-response split) are later options, [open] O-1.

### 3.3 Missing-mass correction

- **[decided]** For direction k, the residual inertia load is F_res,k = a_ZPA,k · M (r_k − Σ_i Γ_ik φ_i), summed over the extracted modes, with a_ZPA,k the user's rigid acceleration and M the full mass on the free DOFs. It is solved statically on the same K_ff: u_res,k = K_ff⁻¹ F_res,k, and recovered like a modal response.
- In exact arithmetic, M(r_k − Σ Γ_ik φ_i) is M-orthogonal to every extracted mode, so the correction carries only the mass the modes leave out. Its effective mass is r_kᵀM r_k − Σ_i m_eff,ik, published as the missing-mass fraction.
- It is combined with the modal part of the same direction by the user's rule, SRSS or absolute sum, before directional combination.
- **Check:** on a model whose modes all lie above the spectrum's rigid range, with no modes extracted below it, the corrected response equals the static response to a_ZPA,k·M r_k. This is a closed-form reference (§5).

### 3.4 Directional combination

- **SRSS:** R = √(R_X² + R_Y² + R_Z²) over the directions requested.
- **100-40-40:** R = max over the three permutations of (R_a + 0.4 R_b + 0.4 R_c), with R_a, R_b, R_c the directional magnitudes. With two directions, max(R_a + 0.4 R_b, R_b + 0.4 R_a).

### 3.5 Combination with static cases

- **[decided]** F2 does not combine with static cases. A spectrum result is offered to DEL-05-02's load-case algebra as an unsigned operand, so that, for example, sustained plus seismic by absolute sum or SRSS is a load-case combination with DEL-05-02's rules. The interface (an unsigned-operand marker on the result) is agreed with the load-case lane when F2's results land (O-2).

## 4. Snubbers

- **[decided] Document shape** (agreed with the load-case manager, who owns the 0.9.0 member list, 2026-10-31; DEL-02-01 Work `model-document-0.9.0.md` §2.4): `SupportFamily` gains `snubber`, reusing `Support`'s members; restraints are translational DOFs only (`SNUBBER_ROTATIONAL_RESTRAINT_UNSUPPORTED`); an optional `stiffness` gives one finite dynamic stiffness on exactly its one restrained DOF (`SNUBBER_STIFFNESS_INVALID`); `hanger` and a non-null `nonlinear` are refused on a snubber (`SNUBBER_MEMBER_CONFLICT`); `supports[].direction` is admitted on a snubber with exactly one translational restraint, so it acts along that skew direction as a skewed line stop does.
- **Interim** (0.9.0's first slice): every static solve on both routes refuses a model containing a snubber, by name (`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`). F2's slice replaces that refusal with the meaning below.
- **[decided] Dynamic meaning.** In a modal request that includes dynamic-only restraints, and so in every spectrum solve, a snubber is a rigid restraint of its DOFs, or a diagonal spring of its stiffness. Static solves on both routes ignore it, with the disclosure "dynamic-only restraint, not active in static cases".

## 5. References and the screening class

- **[decided]** `validation/references/f2_spectrum/`: a standard-library Python generator, exact rationals of the binary64 inputs and `decimal` for roots, invented data only, frozen JSON and `SHA256SUMS`, written by a TASK that reads no product code and refuted by a second TASK before any comparison. Cases:
  - a single degree of freedom under a stated spectrum, each interpolation rule;
  - a two- or three-degree-of-freedom chain with closed-form modes, under SRSS, CQC (ζ series) and grouping, including a closely spaced pair;
  - a uniform cantilever with distributed mass under a flat spectrum, using its closed-form participation (the effective-mass series of the clamped beam), with the missing-mass correction for 1, 2 and 3 retained modes converging to the all-mode value;
  - a stiff model whose corrected response equals its static rigid-acceleration response;
  - directional combinations, SRSS and 100-40-40, with two and three directions;
  - a snubber model: the same frame with the snubber as a rigid restraint in the dynamic basis and absent in statics.
- **[decided] The spectrum screening class** for `docs/RELEASE_TARGET_v1.0.md` §3 is derived from these references before any comparison run. The method: from the generator's sensitivity of each combined response to a relative perturbation of the frequencies at the modal class's 5e-3, propagated through interpolation and combination, rounded up to a stated value. It is reported to Agent 0 for the owner with its derivation and date, and only tightened.

## 6. Product layout (settled at build start)

Planned, as in `deliverable.yaml`: `core/product_physics/src/spectrum.rs` with an entry beside DEL-04-09's modal entry, the `spectrum-1` table and profile, the Rust and Python readers, a headless command, and a Tauri command, editor, results panel and report section. Priced-source and reviewed-input changes re-register D3 in their change.

## 7. Risks

- **R1, unsigned results misused.** Combined magnitudes look like static results. Every row and surface labels them unsigned, and no signed recovery, deformed shape or stress is made from them (AX-003).
- **R2, truncation.** Few modes with a high-frequency spectrum lose mass. The missing-mass correction and its published fraction address it; the correction is the user's choice, and its absence is disclosed with the cumulative effective-mass fraction.
- **R3, closely spaced modes.** SRSS underestimates correlated close modes. CQC and grouping are offered; the choice is the user's.
- **R4, nonlinear supports.** The spectrum result holds only at the linearized state the modal basis names, which the result states.
- **R5, stresses.** Users will want seismic occasional stresses. They need per-mode stress recovery combined by the same rules, not recovery from combined forces; outside these commitments (O-3).

## 8. Open items

| # | Item | Owner | When |
|---|---|---|---|
| O-1 | Further modal-combination rules (RG 1.92 Rev. 3 rigid-response methods) | Later slice | After F2's core |
| O-2 | The unsigned-operand interface with DEL-05-02's load-case algebra | Dynamics and load-case lanes | When F2's results land |
| O-3 | Stresses from spectrum responses, by per-mode recovery and combination | HELP_HUMAN, for scope | After F2's core |
| O-4 | Independent support motion (REQ-011, Should): group spectra and group combination | F2 | After the uniform-motion core |
| O-5 | The spectrum screening class: derived from the references and reported | F2 | Before any comparison run |
