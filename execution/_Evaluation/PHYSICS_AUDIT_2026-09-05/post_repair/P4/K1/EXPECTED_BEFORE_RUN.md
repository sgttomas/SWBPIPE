# Independent expectations frozen before repair comparisons
Accepted baseline M1 packets supply original observations; no baseline packet edited.
* Dense/sparse .5 x = 1e308 must return an error because x=2e308 cannot be represented. Finite 2x2 [[1e308,1e308],[1e308,-1e308]], rhs [1e308,-1e308] has exact [0,1]; structured rejection of intermediate overflow is acceptable.
* Overflowing positive finite E/G=1e308 and section properties 10 must not publish nonfinite stiffness.
* A spring affects exactly one DOF, finite nonnegative stiffness; zero remains accepted. Point load fraction outside [0,1] is an error even when beyond the queried station.
* Symmetric spectral condition diag[1e-10,1]=1e10, diag[0,1] unavailable, diag[-2,1]=2. Scaling matrix leaves condition unchanged. Tridiagonal n=32 with diagonal2/offdiagonal-1 has condition (2+2cos(pi/33))/(2-2cos(pi/33)) = approximately440.68856. This analytic spectrum is independent of implementation.
* NaN or infinite residual/delta cannot be reduced to zero; helpers must retain an explicit nonfinite signal. These checks are diagnostic arithmetic, not new production convergence criteria.
