"""Independent reference audit: no production or submitted-generator imports.
Run from repository root. Writes stdout only. Fraction represents exact arithmetic;
140-digit Decimal and Gauss-Legendre pi check submitted 100-digit constants.
Internal comparison budgets check reference arithmetic, not product tolerances.
"""
import hashlib,json,math,platform,sys
from decimal import Decimal as D,localcontext
from fractions import Fraction as F
from pathlib import Path
# Resolve by the fixture's ancestor rather than assuming an installed tool package.
for parent in Path(__file__).resolve().parents:
    if (parent/'projects/chirality-piping/validation/benchmarks/numerical_integrity/fixtures.json').is_file():
        FIX=parent/'projects/chirality-piping/validation/benchmarks/numerical_integrity/fixtures.json';break
X=json.loads(FIX.read_text())
if len(sys.argv)>1:
    mutation=sys.argv[1]
    if mutation=='annulus':X['section']['I_m4']=str(D(X['section']['I_m4'])*2)
    elif mutation=='reaction':X['N']['N08']['torque_cases'][0]['root_MX_Nm']=1
    elif mutation=='original_residual':X['R']['R03']['f_original']=[0]
    elif mutation=='stored_spring':X['NP']['A'][0]['k_stored_exact']='1e-4'
    elif mutation=='band_entry':X['NP']['B'][0]['K_entries'][0][2]='2'
    elif mutation=='internal_mode':X['NP']['C']['K_internal_mechanism_diagonal'][-1]=1
    else:raise ValueError(mutation)
report={'python':sys.version,'platform':platform.platform(),'fixture_sha256':hashlib.sha256(FIX.read_bytes()).hexdigest(),'checks':{},'observations':{}}

def d(x):return D(str(x))
def frac(x):return F(str(x))
def close(got,want,rel=D('1e-96'),absolute=D('1e-110')):
    assert abs(d(got)-want)<=max(abs(want)*rel,absolute),(got,str(want))
def rank(rows):
    a=[[frac(x) for x in row] for row in rows];r=0
    for j in range(len(a[0])):
        p=next((i for i in range(r,len(a)) if a[i][j]),None)
        if p is None:continue
        a[r],a[p]=a[p],a[r];v=a[r][j];a[r]=[x/v for x in a[r]]
        for i in range(len(a)):
            if i!=r:
                v=a[i][j];a[i]=[x-v*y for x,y in zip(a[i],a[r])]
        r+=1
        if r==len(a):break
    return r

def residual(k,u,f,free):
    k=[[frac(x) for x in row] for row in k];u=list(map(frac,u));f=list(map(frac,f))
    r=[sum((a*b for a,b in zip(row,u)),F(0))-b for row,b in zip(k,f)]
    den=[sum((abs(a*b) for a,b in zip(row,u)),F(0))+abs(b) for row,b in zip(k,f)]
    return r,den,max((abs(r[i])/den[i] if den[i] else F(0) for i in free),default=F(0))
def dec(x):return D(x.numerator)/D(x.denominator)
with localcontext() as ctx:
    ctx.prec=140
    # Quadratically convergent AGM construction, independent of Machin series.
    a=D(1);b=1/D(2).sqrt();t=D(1)/4;p=D(1)
    for unused in range(12):
        new=(a+b)/2;b=(a*b).sqrt();t-=p*(a-new)**2;a=new;p*=2
    pi=(a+b)**2/(4*t)
    R=D('.1');r=D('.09');area=pi*(R*R-r*r);I=pi*(R**4-r**4)/4;J=pi*(R**4-r**4)/2
    E=D('2e11');G=D('8e10');EA=E*area;EI=E*I;GJ=G*J
    for key,val in [('A_m2',area),('I_m4',I),('J_m4',J),('EA_N',EA),('EI_Nm2',EI),('GJ_Nm2',GJ)]:close(X['section'][key],val)
    report['checks']['annular_polar_integration_and_independent_pi']='pass'
    for name,L,force in [('N01',D(2),D(1000)),('N09',D(10),D(100))]:
        row=X['N'][name] if name=='N01' else X['N'][name]['bending']
        close(row['uy_m'],force*L**3/(3*EI));close(row['rz_rad'],force*L**2/(2*EI));assert d(row['root_FY_N'])==-force;assert d(row['root_MZ_Nm'])==-force*L
    for row in X['N']['N08']['torque_cases']:
        close(row['theta_x_rad'],d(row['T_Nm'])*2/GJ);assert d(row['root_MX_Nm'])==-d(row['T_Nm']);assert float(row['theta_x_rad'])!=0 and round(float(row['theta_x_rad']),6)==0
    close(X['N']['N09']['torsion']['theta_x_rad'],D(1)/GJ)
    assert d(X['N']['N09']['torsion']['root_MX_Nm'])==D('-.1')
    for name,k,T in [('N05',D('1e-4'),D('1e-8')),('N06',D('1e-12'),D('1e-16'))]:
        row=X['N'][name];at=GJ/2;close(row['a_Nm_per_rad'],at);close(row['theta_root_rad'],T/k);close(row['theta_tip_rad'],T/k+T/at)
        assert d(row['spring_action_Nm'])==-T and row['inertia']==[7,0,0]
        s=(at/(at+k)).sqrt();rc=(k/(at+k))/(1+s)**2
        # Original (1-s) expression loses up to 22 decimal digits, still far
        # more accurate than binary64 or the protected product comparison.
        close(row['rcond2_equilibrated'],rc,rel=D('1e-75'))
        for i in range(2):
            for j in range(2):close(row['K_intended'][i][j],[[at+k,-at],[-at,at]][i][j])
    report['checks']['N01_N05_N06_N08_N09_displacements_and_reactions']='pass'
    e=[F(3,5),F(4,5),F(0)];P=[[x*y for y in e] for x in e];Q=[[F(int(i==j))-P[i][j] for j in range(3)] for i in range(3)]
    # pi-factored exact rotational matrix, directly from torsion/bending energy.
    aa=F(687800);bb=F(859750)
    K=[[((aa if i//3==j//3 else -aa)*P[i%3][j%3]+(4*bb if i//3==j//3 else 2*bb)*Q[i%3][j%3]) for j in range(6)] for i in range(6)]
    assert rank(K)==5 and all(sum(K[i][j]*(e+e)[j] for j in range(6))==0 for i in range(6))
    for fixed,want in [(0,5),(2,4)]:
        keep=[i for i in range(6) if i!=fixed];assert rank([[K[i][j] for j in keep] for i in keep])==want
    # Translation rows evaluated on six infinitesimal rigid-body coordinates.
    B=[[1,0,0,0,0,0],[0,1,0,0,0,0],[0,0,1,0,0,0],[1,0,0,0,0,F(-8,5)],[0,1,0,0,0,F(6,5)],[0,0,1,F(8,5),F(-6,5),0]]
    assert rank(B)==5 and rank(B+[[0,0,0,1,0,0]])==6 and rank(B+[[0,0,0,0,0,1]])==5
    n=X['N'];assert n['N02']['inertia']==[5,0,1];assert n['N03']['extra_root_RX']['inertia']==[5,0,0];assert n['N03']['wrong_extra_root_RZ']['inertia']==[4,0,1];assert n['N04']['free_dimension']==18 and n['N04']['inertia']==[12,0,6]
    for x,y in zip(n['N02']['rotational_eigenvalues'],[D(0),GJ,EI,EI,3*EI,3*EI]):close(x,y)
    assert sum(n['N07']['negative_vector'][i]*n['N07']['K'][i][j]*n['N07']['negative_vector'][j] for i in range(2) for j in range(2))==-2
    report['checks']['N02_N03_exact_rank_nullspace_N04_dimension_N07_energy']='pass'
    controls={}
    for name,target in [('R01',F(1,201)),('R02',F(1,201)),('R03',F(1)),('R04',F(1)),('R05',F(1,9))]:
        row=X['R'][name];rr,dd,eta=residual(row['K_original'],row['candidate'],row['f_original'],row['free'])
        close(dec(eta),dec(target));close(row['eta'],dec(target));assert eta>F(1,10**9)
        for got,want in zip(row['r'],rr):close(got,dec(want))
        for got,want in zip(row['denominator'],dd):close(got,dec(want))
        controls[name]={'residual':[str(dec(v)) for v in rr],'eta':str(dec(eta))}
    row=X['R']['R02'];assert abs(dec(residual(row['K_used'],row['candidate'],row['f_original'],row['free'])[2]))<D('1e-97')
    row=X['R']['R04'];rr,dd,eta=residual(row['K_original'],row['valid_u'],row['f_original'],row['free']);assert eta==0
    for got,want in zip(row['valid_reactions'],rr):close(got,dec(want))
    row=X['R']['R05'];close(row['spring_N_per_m'],EA/4);close(row['valid_u'][0],D(1000)/(EA*D('1.25')))
    assert X['R']['R03']['work_residual']==0;assert X['R']['R06']['free']==[] and X['R']['R06']['reactions']==[-1,-2,-3,-4,-5,-6] and X['R']['R06']['eta']==0
    assert X['R']['R07']['eta']==0 and X['R']['R07']['unique'] is False
    report['checks']['R01_R07_independent_fraction_substitutions']='pass';report['observations']['residual_controls']=controls
    ulps=[]
    for row in X['NP']['A']:
        af=float.fromhex(row['a_binary64_hex']);diag=float.fromhex(row['diag_binary64_hex']);tf=float.fromhex(row['T_binary64_hex']);aa=F.from_float(af);dd=F.from_float(diag);tt=F.from_float(tf);ks=dd-aa
        assert af==float(GJ/2) and diag==af+float(row['k_intended']) and tf==float(row['T_intended'])
        assert [[frac(v) for v in rr] for rr in row['K_stored_exact']]==[[dd,-aa],[-aa,aa]] and list(map(frac,row['f_stored_exact']))==[0,tt] and frac(row['k_stored_exact'])==ks
        if ks:
            root=tt/ks;tip=root+tt/aa;close(row['stored_exact_root'],dec(root));close(row['stored_exact_tip'],dec(tip));assert residual([[dd,-aa],[-aa,aa]],[root,tip],[0,tt],[0,1])[2]==0
            close(row['root_relative_physical_error'],dec(root)/d(row['intended_root'])-1,rel=D('1e-77'))
        else:assert 'singular' in row['stored_status'] and d(row['k_intended'])>0
        ulps.append({'id':row['id'],'stored_increment_ulps':str(ks/F(1,2**31))})
    assert F(X['NP']['A'][0]['k_stored_exact'])==F(214748,2**31) and abs(d(X['NP']['A'][0]['root_relative_physical_error']))>D('1e-9')
    report['checks']['NP_A_exact_hex_fraction_matrix_solutions_ULP_ties']='pass';report['observations']['ulp_sweep']=ulps
    M=1/((1-D(3).sqrt()/2)*(D(13).sqrt()/4));cond=(D('2.5')*M)**2;assert cond<429
    # Perturbation extension to the non-dyadic companion via Neumann series.
    ep=D('1e-12');perturbed_cond=((D('2.5')+2*ep)*M/(1-2*ep*M))**2;assert perturbed_cond<429
    band=[]
    for row in X['NP']['B']:
        N=row['n'];c=frac(row['coefficient']);entries={(i,j):frac(v) for i,j,v in row['K_entries']}
        assert len(entries)==len(row['K_entries'])
        for i in range(N):
            for j in range(N):
                hi=max(i,j);dist=abs(i-j)
                expected=1+c*c*(int(i>=1)+int(i>=2)) if dist==0 else c+(c*c if hi>=2 else 0) if dist==1 else c if dist==2 else F(0)
                assert entries.get((i,j),F(0))==expected
        assert list(map(frac,row['f_exact']))==[sum(entries.get((i,j),F(0)) for j in range(N)) for i in range(N)] and row['u_exact']==[1]*N
        assert all(sorted(order)==list(range(N)) for order in row['orderings'].values())
        z0=F(0);z1=F(1)
        for i in range(1,N):z0,z1=z1,1+c*(z1+z0)
        close(row['comparison_bound_z'],dec(z1));close(row['z_squared_u'],dec(z1*z1/F(2**53)))
        if N>=64:assert z1*z1>2**53
        coeff=[F(1),-c]
        for j in range(2,N):coeff.append(-c*(coeff[-1]+coeff[-2]))
        assert sum(abs(v) for v in coeff)<F(str(M))
        band.append({'n':N,'c':str(c),'comparison_squared_times_u':str(dec(z1*z1/F(2**53)))})
    report['checks']['NP_B_exact_banded_coefficients_loads_permutations_bounded_condition']='pass';report['observations']['NP_B']={'uniform_condition_bound':str(cond),'perturbation_extended_bound':str(perturbed_cond),'recurrence':band}
    c=X['NP']['C'];assert rank(c['rigid_restraint_B'])==6 and c['K_internal_mechanism_diagonal']==[1]*6+[0] and c['K_stabilized_diagonal']==[1]*7 and rank(c['near_collinear_rows'])==2
    dd=X['NP']['D'];assert frac(dd['skew']['K'][1][0])-frac(dd['skew']['K'][0][1])==F(1,100);assert sum(frac(row[2]) for row in dd['duplicate_cancellation']['entries'])==1
    assert (1e16+1)-1e16==0 and float('1e308')*float('1e308')==math.inf and float.fromhex('0x0.0000000000001p-1022')*.5==0
    assert dd['lift_off']['active_K_diagonal']==[1,1] and dd['lift_off']['inactive_K_diagonal']==[1,0]
    report['checks']['NP_C_NP_D_internal_rank_skew_range_duplicate_controls']='pass'
report['result']='PASS independent reference checks; no production execution or acceptance'
print(json.dumps(report,indent=2))
