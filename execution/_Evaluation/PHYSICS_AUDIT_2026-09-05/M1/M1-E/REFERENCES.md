# Independent expectations frozen before execution

Scope: project-original invented mechanics experiments; no extracted corpus equations, commercial example, or manufacturer data used. Derivations use elementary static equilibrium, linear small-displacement kinematics, and the declared Euler–Bernoulli/axial/Saint-Venant energy model. Reference calculations are written independently in Python and do not call production routines or copy their trigonometric Gram implementation. They share the declared physical theory; agreement does not validate that theory for all piping geometries.

## Frame and straight pipe
L=2 m, E=2e11 Pa, G=8e10 Pa, A=0.003 m², Iy=5e-6 m⁴, Iz=7e-6 m⁴, J=9e-6 m⁴. A clamped/free span under tip vector [120 N, 350 N, -240 N, 80 Nm, -60 Nm, 45 Nm] has u=Fx L/(EA), rx=Mx L/(GJ), v=Fy L³/(3EIz)+Mz L²/(2EIz), rz=Fy L²/(2EIz)+Mz L/(EIz), w=Fz L³/(3EIy)-My L²/(2EIy), ry=-Fz L²/(2EIy)+My L/(EIy). Root force=-F; root moment=-M-r×F. Force resultant excludes moments.

Rigid translation/infinitesimal rotation at any nodes gives u=t+omega×x, theta=omega. A freely moving frame element should have Kd=0 and strain energy=0. Reversal of an isotropic circular member and coherent DOF permutation preserves global stiffness. Work congruence is d_globalᵀKglobal d_global=(T d_global)ᵀKlocal(T d_global).

Clamped/clamped span under uniform Y load q=-100 N/m has root force -qL/2=100 N and root moment -qL²/12=+33.3333333333333 Nm. In the production straight kernel's i-side convention V(x)=100-100x and Mz(x)=100/3-100x+50x², hence midpoint Mz=-50/3 Nm. Linear interpolation of un-reoriented endpoint actions is not a section-equilibrium recovery method; it gives midpoint zero for this case. Full/partial uniform and point loads must preserve net force and moment. No distributed torsion or point moments assumed where API does not implement them.

## Curved beam
Use radius 2 m, angles 0.05,0.2,0.7,pi/2,2.5 rad, E/G/A/I/J above with I=5e-6, factors 1 and (2,3). Independently integrate the energy inner product numerically along p(theta)=(R cos theta,R sin theta,0). For tip load force F and couple C, segment actions are N=F·t, M=C+(p_tip-p)×F, torsion=M·t, radial bending=M·r, in-plane bending=Mz. Compute flexibility via Simpson integration of [N_a N_b/EA + k_in Mza Mzb/EI + k_out Mra Mrb/EI + Ta Tb/GJ] R dtheta. Refine 256→512→1024 panels and report change, not arbitrary acceptance of an unconverged reference. Uniform distributed and radial pressure loads checked by independent resultant/moment quadrature and direct segment integration.

## Criteria
Exact identities and dimensional contradictions determine qualitative findings. Report raw absolute/normalized errors for approximate identities. For matching the documented analytic-class tests, DEC-026 1e-9 relative tier is recorded only as an existing reference tier; this audit does not adopt a new engineering threshold. Use roundoff-scale comparisons diagnostically and report them, without promoting them into production policy. New generated/invented witness engineering acceptance remains unclaimed. Finite positive constructor validation implies invalid/overflow behavior should be reported explicitly; robustness witnesses distinguish successful API output containing nonfinite values from product acceptance.

## Known limitations
Euler–Bernoulli excludes shear deformation, warping, ovalization except opaque flexibility factors, geometric nonlinearity and constitutive nonlinearity. Rigid component mechanics may be disconnected; examine current contract. User-stiffness rigid-rotation expectation is a physical objectivity witness; exact connector constitutive remedy requires declared reference-frame semantics.
