python3 - <<'PY'
from fractions import Fraction as F
from math import pi,cos,sin
V=lambda a:list(map(F,a))
add=lambda a,b:[x+y for x,y in zip(a,b)]
sub=lambda a,b:[x-y for x,y in zip(a,b)]
mul=lambda s,a:[s*x for x in a]
dot=lambda a,b:sum(x*y for x,y in zip(a,b))
def cross(a,b):return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
z=V([0,0,0]); xi=V([1,2,3]); ai=V([F(1,5),F(2,5),-F(1,5)]); aj=V([-F(1,10),F(3,10),F(1,2)]); r=V([2,0,0]); xj=sub(add(add(xi,ai),r),aj)
def q(d):
 ui,ti,uj,tj=[d[i:i+3] for i in [0,3,6,9]]
 return sub(sub(add(uj,cross(tj,aj)),add(ui,cross(ti,ai))),cross(mul(F(1,2),add(ti,tj)),r))+sub(tj,ti)
cols=[q([F(int(i==j)) for i in range(12)]) for j in range(12)]
B=[[cols[j][i] for j in range(12)] for i in range(6)]
o=V([7,-3,5]); modes=[]
for k in range(3):
 e=V([int(k==j) for j in range(3)])
 modes += [q(e+z+e+z),q(cross(e,sub(xi,o))+e+cross(e,sub(xj,o))+e)]
assert all(v==0 for mode in modes for v in mode)
g=V([2,-3,5,7,-11,13]); f=[dot(c,g) for c in cols]; Fi,Mi,Fj,Mj=[f[i:i+3] for i in [0,3,6,9]]
assert add(Fi,Fj)==z
moment=add(add(cross(sub(xi,o),Fi),Mi),add(cross(sub(xj,o),Fj),Mj))
assert moment==z
d=V([F(i-4,17) for i in range(12)])
assert dot(f,d)==dot(g,q(d))
print('EXACT rational checks: six rigid modes, arbitrary-origin force/moment balance, virtual work PASS')
print('offset nodal internal force blocks:',f)
print('reference q:',q(d),'virtual work:',dot(g,q(d)))
kp=27500000*pi; kj=200000; Pi=125*pi; Pe=800; h=75*pi
opening=(Pe-Pi+h)/(kj+kp/2); T=kj*opening; Nw=T-Pe+Pi; S=T-Pe
print('anchored untied: q_m, T_N, Nw_N, S_N, Rleft_N:',opening,T,Nw,S,-S)
print('anchored exact tie: Ttie_N, Nw_N, S_N:',Pe-Pi+h,h,h-Pi)
print('free untied: q_m, each_stub_extension_m, total_extension_m:',Pe/kj,1/550000,Pe/kj+1/275000)
print('free tied: tie_N, total_extension_m:',Pe,1/275000)
print('free finite tie kt=2e7: q_m, Tbellows_N, Ttie_N:',Pe/(kj+2e7),kj*Pe/(kj+2e7),2e7*Pe/(kj+2e7))
qwrong=(h+Pe)/(kj+kp/2)
print('doublecount: anchored untied q_m, free untied q_m, anchored tied wrongtie_N:',qwrong,(Pe+Pi)/kj,Pe+h)
print('finite rotation L=2 phi=.1rad residual q:',2*(cos(.1)-1),2*(sin(.1)-.1))
PY