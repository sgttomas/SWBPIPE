"""Audit-only references. Standard library; imports no product/benchmark implementation.
Arc oracle integrates vector equilibrium + strain energy numerically, unlike
production closed forms. Float refinement reports error evidence, not tolerance approval.
"""
import math,json
from fractions import Fraction as Q
from pathlib import Path
ROOT=Path(__file__).parent

def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
def add(a,b):return [x+y for x,y in zip(a,b)]
def solve(a,b):
    a=[list(row)+[y] for row,y in zip(a,b)]; n=len(b)
    for k in range(n):
        p=max(range(k,n),key=lambda i:abs(a[i][k]));a[k],a[p]=a[p],a[k]
        d=a[k][k];a[k]=[x/d for x in a[k]]
        for i in range(n):
            if i!=k:
                d=a[i][k];a[i]=[x-d*y for x,y in zip(a[i],a[k])]
    return [row[-1] for row in a]

def arc(n,k,q):
    R=1.2; phi=math.pi/2; D=.1683;t=.0071;E=200e9;G=80e9
    area=math.pi/4*(D*D-(D-2*t)**2);I=math.pi/64*(D**4-(D-2*t)**4);J=2*I
    flex=[[0.]*6 for _ in range(6)]; free=[0.]*6
    B=[R*math.cos(phi),R*math.sin(phi),0.]
    for s in range(n+1):
        th=phi*s/n;P=[R*math.cos(th),R*math.sin(th),0.];tang=[-math.sin(th),math.cos(th),0.]
        arm=[x-y for x,y in zip(B,P)];ds=R*phi/n/3*(1 if s in (0,n) else 4 if s%2 else 2)
        fields=[]
        for j in range(6):
            f=[float(j==a) for a in range(3)];m=add(cross(arm,f),[float(j==a+3) for a in range(3)])
            fields.append((dot(f,tang),m,dot(m,tang)))
        fq=[R*(phi-th)*x for x in q]
        first=[R*R*(math.sin(phi)-math.sin(th)-(phi-th)*math.cos(th)),R*R*(math.cos(th)-math.cos(phi)-(phi-th)*math.sin(th)),0.]
        mq=cross(first,q);qfield=(dot(fq,tang),mq,dot(mq,tang))
        def energy(a,b):return a[0]*b[0]/(E*area)+k*(dot(a[1],b[1])-a[2]*b[2])/(E*I)+a[2]*b[2]/(G*J)
        for i in range(6):
            free[i]+=energy(fields[i],qfield)*ds
            for j in range(6):flex[i][j]+=energy(fields[i],fields[j])*ds
    reaction=solve(flex,[-x for x in free])
    stations=[]
    for frac in (.25,.5,.75):
        th=phi*frac; P=[R*math.cos(th),R*math.sin(th),0.]; tang=[-math.sin(th),math.cos(th),0.]; inward=[-math.cos(th),-math.sin(th),0.]
        arm=[x-y for x,y in zip(B,P)];first=[R*R*(math.sin(phi)-math.sin(th)-(phi-th)*math.cos(th)),R*R*(math.cos(th)-math.cos(phi)-(phi-th)*math.sin(th)),0.]
        m=add(add(reaction[3:],cross(arm,reaction[:3])),cross(first,q))
        stations.append({'fraction':frac,'torsion':dot(m,tang),'bending_y':dot(m,inward),'bending_z':m[2]})
    return {'free_tip':free,'clamped_B':reaction,'stations':stations}

simple={
'cantilever_tip_m':Q(6)*10**3/(3*1200*4),
'cantilever_moment_Nm':Q(6)*10,
'branch_junction_m':Q(90)/(Q(12*1200*4,2**3)+Q(12*1200*4,3**3)),
'branch_tip_m':Q(90)/(Q(12*1200*4,2**3)+Q(12*1200*4,3**3))+Q(90,600),
'canonical_tip_m':Q(-2)*4**4/(8*2000)+Q(-4)*2**2*(3*4-2)/(6*2000),
'canonical_rotation_rad':Q(-2)*4**3/(6*2000)+Q(-4)*2**2/(2*2000),
'canonical_station_shear_N':Q(12)-2*2-4,
'canonical_station_moment_Nm':Q(24)-12*2+Q(2*2**2,2),
'constant_effort_tip_m':Q(3)*10**3/(3*1200*4),
'bounded_friction_u_mm':Q(10-6,100),
'parallel_spring_u_m':Q(10,100+50),
'gap_u_m':min(Q(10,100+50),Q(1,20)),
'gap_reaction_N':max(Q(10)-(100+50)*Q(1,20),Q(0)),
'fixed_fixed_end_magnitude_Nm':Q(12)*4**2/12,
'fixed_fixed_mid_magnitude_Nm':Q(12)*4**2/24,
'axial_stress_Pa':Q(120,12),
'bending_y_Pa':Q(50,25),
'bending_z_Pa':Q(-30,15),
'torsion_shear_Pa':Q(40)*2/80,
'pressure_hoop_Pa':Q(100)*3/Q(1,2),
'wind_intensity_N_per_m':Q(480)*Q(7,10)*Q(1,4),
'wind_partial_end_i_N':Q(84)*Q(1,2)*3*Q(55,100),
'wind_partial_end_j_N':Q(84)*Q(1,2)*3*Q(45,100),
'hot_thermal_force_N':Q(180000000000)*Q(4,1000)*Q(13,1000000)*10,
'hot_thermal_stress_Pa':Q(180000000000)*Q(13,1000000)*10,
}
results={'method':'independent exact-rational elementary cases and vector-energy Simpson arc quadrature; no production import','simple':{k:{'exact':str(v),'decimal':float(v)} for k,v in simple.items()},'arc':{}}
for k in (1,2):
 for name,q in [('in_plane',[0.,-1500.,0.]),('out_of_plane',[0.,0.,-800.])]:
    a,b=arc(1024,k,q),arc(2048,k,q)
    results['arc'][f'k{k}_{name}']={'n1024':a,'n2048':b,'max_B_refinement_delta':max(abs(x-y) for x,y in zip(a['clamped_B'],b['clamped_B']))}
(ROOT/'INDEPENDENT_ORACLE_RESULTS.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps({'simple_cases':len(simple),'arc_cases':len(results['arc']),'max_B_refinement_delta':max(v['max_B_refinement_delta'] for v in results['arc'].values())}))
