# Additional frozen expectations before V2
Symmetric A=[[1e308,1e308],[1e308,-1e308]], f=[1e308,-1e308] has exactly representable finite x=[0,1] by row substitution. Overflow during elimination must not be returned as successful NaN solution. Unlike scalar .5*x=1e308, the exact solution is representable.
For the existing chain1 fixture with finite lateral load1e308, expose reduced-system dense/sparse vectors and independently recompute every residual component, retaining nonfinite flags. A NaN residual must not be reduced to 0. No engineering limit is introduced.
