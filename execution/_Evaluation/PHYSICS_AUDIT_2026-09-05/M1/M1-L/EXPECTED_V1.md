# Frozen expectation before execution
Source basis 2be412ccea62bdc4bd96deb082c46d7a792076ea. Exact production crates linked; performance source copied byte-for-byte then two private-function exposure wrappers appended; production untouched.

All arithmetic checks are numerical software invariants, not new engineering tolerances. Exact binary scalar systems k*x=f have x=f/k. For k=.5 and f=1e308, finite inputs require either reported nonfinite failure or a representable finite solution; successful infinity is not a finite solution. Dense finite symmetric [[1e308,1e308],[1e308,-1e308]], force [1,1] has exact x=[1e-308,0]; elimination can overflow and must not quietly publish NaNs.

For A=[[4,1],[1,3]], x=[1,2], f=[6,7]. Prescribing x0=1 gives reduced [3] and f=[6], hence x1=2. Full prescription gives empty reduced system and empty solution. Repeated/out-of-range constraints, ragged matrices, nonfinite input, wrong lengths and invalid sparse permutation/entry indices must return errors, not panic.

Fixed integer constructed systems use B diagonal dominant with offdiagonal small integers, A=B^T B + I, integer known x, f=A*x. All are SPD by x^T A x=||Bx||^2+||x||^2. Expected x is frozen in fixture generator before solve. Record forward errors and independent infinity-norm backward residual eta=||Ax-b||inf/(||A||inf||x||inf+||b||inf), using Python Decimal from exact float values. No pass threshold invented. Dense/sparse/direct-entry results and deterministic repeat compared.

Scalar scale witness k=f=2^-40 has exact x=1 and condition 1; existing absolute 1e-12 pivot guard predicts rejection. Record documented policy limitation, not engineering policy override.

Conditioning oracle: symmetric tridiagonal n=32, diagonal 2 and offdiagonal -1 has eigenvalues 2-2*cos(j*pi/(n+1)), j=1..n, by substituting sine eigenvectors into the recurrence and zero endpoint conditions. kappa2=(2+2cos(pi/33))/(2-2cos(pi/33)). For actual existing invented cantilever chain8 reduced matrix, independently use NumPy LAPACK eigh and assess orthonormality/eigenpair residual; report numerical discrepancy and uncertainty without asserting engineering acceptance. Production function is observation-only.

Eigenvalue-floor witness diagonal[1e-10,1] has exact condition 1e10. Discarding small eigenvalues cannot yield the true 2-norm condition. Diagonal[-2,1] has singular values2,1 and condition2 (indefinite outside expected stiffness SPD); zero eigenvalue means singular. These private-function probes are qualified as isolated numerical helper evidence, not product path evidence.
