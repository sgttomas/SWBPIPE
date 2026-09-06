# Pre-comparison independent V2 expectations
Dimensioned invented scalar cases; no new acceptance tolerance or production policy.

Two axial bars in series, each EA/L=100 N/m, node0 fixed, node1/2 axial unknown. K=[[200,-100],[-100,100]]. Upper gaps g1=.04m,g2=.1m and F=[0,10]N: u=[.04,.1]m, R=K*u-F=[-2,-4]N, both admissibly active. F=[0,-10] instead gives free u=[-.1,-.2],R0 on bothgaps. All four active/inactive seeds should find same unique friction-free equilibrium where intermediate solves remain stable. This is coupled multisupport interaction, not two independent scalar bodies.

Parallel user-stiffness element axial200N/m beside axialbar100: effective300. F10N uppergap.04m yields u=1/30m,Rgap0 (inactive); F20 gives u=.04,Rgap=-8. User macro slot must retain contribution in nonlinear assembly. This does NOT substitute for product ground-spring transfer testing.

Torsional straight element L1m,G40Pa,J1m4 gives ktheta40N m/rad. Upper rotational gap.05rad underT4N m gives theta.05,R=-2N m. Mirrored T/gap gives opposite signs. Rotational quantities must remain separately dimensioned; no torque+force norm used.

Coupled friction observation uses USER stiffness rotated45deg: local axial100,lateral200 gives global translational Kxx=150,Kxy=-50. yfixed; Fx10N,Fy=-10N. Derived normal Ry=10-50u. For positive sliding, simultaneous Coulomb bound Rf=-.3*(10-50u) and equilibrium150u=10+Rf imply u=7/135=.05185185185, normal200/27=7.4074074074N, friction=-20/9=-2.2222222222N. This analytically represents the declared bounded static relation, not a new friction path-history model. Test reports discrepancies as existing prior-iterate/normal-update policy hold; do not choose repair policy. At any returned iterate, compare applied friction magnitude against .3*current derived normal as a dimensioned observation.

No production results used to choose these values; equations derive from serial/parallel equilibrium and orthogonal rotation of a diagonal stiffness. Tests are verification of these invented cases, not external engineering validation.
