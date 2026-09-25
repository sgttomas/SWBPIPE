"""Independent analytical references; no production imports. Python standard library only."""
from decimal import Decimal as D, localcontext
from fractions import Fraction
import json
from pathlib import Path


def pi():
    # Machin's identity, alternating series; last omitted term < 1e-105.
    def atan_inverse(n):
        x = D(1)/n; term=x; total=x; i=1
        while abs(term)>D('1e-105'):
            term *= -x*x
            total += term/(2*i+1); i+=1
        return total
    return 16*atan_inverse(5)-4*atan_inverse(239)


def residual(k,u,f,free):
    r=[sum((a*b for a,b in zip(row,u)),D(0))-load for row,load in zip(k,f)]
    den=[sum((abs(a*b) for a,b in zip(row,u)),D(0))+abs(load) for row,load in zip(k,f)]
    return {'r':r,'denominator':den,'eta':max((abs(r[i])/den[i] if den[i] else D(0) for i in free),default=D(0))}


def build():
    with localcontext() as ctx:
        ctx.prec=100
        p=pi(); ea=D(380000000)*p; ei=D(1719500)*p; gj=D(1375600)*p; a=gj/2
        out={'schema':'independent-numerical-reference-v1','arithmetic':'Decimal 100 digits; Machin pi; binary64 matrices decoded exactly',
             'section':{'E_Pa':D('2e11'),'G_Pa':D('8e10'),'OD_m':D('.2'),'ID_m':D('.18'),'A_m2':D('.0019')*p,'I_m4':D('.0000085975')*p,'J_m4':D('.000017195')*p,'EA_N':ea,'EI_Nm2':ei,'GJ_Nm2':gj},'N':{},'R':{},'NP':{}}
        n=out['N']
        n['N01']={'nodes_m':[[0,0,0],[2,0,0]],'fixed':'root all','load':{'tip_FY_N':1000},'uy_m':D(8000)/(3*ei),'rz_rad':D(2000)/ei,'root_FY_N':-1000,'root_MZ_Nm':-2000,'inertia':[6,0,0]}
        n['N02']={'nodes_m':[[0,0,0],['1.2','1.6',0]],'fixed':'both translations','axis':[D('.6'),D('.8'),D(0)],'rigid_rank':5,'inertia':[5,0,1],'rotational_eigenvalues':[D(0),gj,ei,ei,3*ei,3*ei],'torque_cases_Nm':[0,1],'null_rotations':[D('.6'),D('.8'),D(0)]*2}
        n['N03']={'basis':'N02','extra_root_RX':{'inertia':[5,0,0],'rigid_rank':6},'wrong_extra_root_RZ':{'inertia':[4,0,1],'rigid_rank':5}}
        n['N04']={'basis':'N01 plus free equal element at y=3 m','free_dimension':18,'inertia':[12,0,6]}
        for name,k,t in [('N05',D('1e-4'),D('1e-8')),('N06',D('1e-12'),D('1e-16'))]:
            s=(a/(a+k)).sqrt()
            n[name]={'nodes_m':[[0,0,0],[2,0,0]],'fixed':'root UX UY UZ RY RZ','spring_RX_Nm_per_rad':k,'tip_MX_Nm':t,'a_Nm_per_rad':a,'K_intended':[[a+k,-a],[-a,a]],'f':[D(0),t],'theta_root_rad':t/k,'theta_tip_rad':t/k+t/a,'spring_action_Nm':-t,'rcond2_equilibrated':(1-s)/(1+s),'inertia':[7,0,0]}
        n['N07']={'K':[[1,2],[2,1]],'f':[1,1],'u':[D(1)/3,D(1)/3],'negative_vector':[1,-1],'quadratic_energy':-2,'inertia':[1,1,0]}
        n['N08']={'basis':'N01 geometry/support','torque_cases':[{'T_Nm':t,'theta_x_rad':t*2/gj,'root_MX_Nm':-t} for t in [D(1),D(-1),D('.1'),D('-.1')]]}
        n['N09']={'L_m':10,'bending':{'F_N':100,'uy_m':D(100000)/(3*ei),'rz_rad':D(5000)/ei,'root_FY_N':-100,'root_MZ_Nm':-1000},'torsion':{'T_Nm':D('.1'),'theta_x_rad':1/gj,'root_MX_Nm':D('-.1')}}
        k=ea/2; exact=D(1000)/k; wrong=D('1.01')*exact
        for name in ['R01','R02','R03']:
            u=D(0) if name=='R03' else wrong
            out['R'][name]={'K_original':[[k]],'f_original':[D(1000)],'candidate':[u],'free':[0],**residual([[k]],[u],[D(1000)],[0])}
        out['R']['R02']['K_used']=[[k/D('1.01')]]
        out['R']['R03']['f_used']=[0];out['R']['R03']['work_residual']=0
        k=ea; delta=D('1e-4'); mat=[[k,-k,D(0)],[-k,2*k,-k],[D(0),-k,k]]
        out['R']['R04']={'K_original':mat,'f_original':[D(0)]*3,'prescribed':{'0':delta,'2':D(0)},'free':[1],'valid_u':[delta,delta/2,D(0)],'candidate':[delta,D(0),D(0)],'valid_reactions':[k*delta/2,D(0),-k*delta/2],**residual(mat,[delta,D(0),D(0)],[D(0)]*3,[1])}
        # Positive axial ground spring at loaded tip: original stiffness k+ks.
        ks=k/4; u=D(1000)/k
        out['R']['R05']={'K_original':[[k+ks]],'K_used':[[k]],'spring_N_per_m':ks,'f_original':[D(1000)],'candidate':[u],'valid_u':[D(1000)/(k+ks)],'free':[0],**residual([[k+ks]],[u],[D(1000)],[0])}
        out['R']['R06']={'f_original':[1,2,3,4,5,6],'prescribed':[0]*6,'free':[],'reactions':[-1,-2,-3,-4,-5,-6],'eta':0}
        out['R']['R07']={'basis':'N02 zero load','null_rotations':n['N02']['null_rotations'],'eta':0,'unique':False}
        sweep=[]
        for label,k,t in [('N05',D('1e-4'),D('1e-8')),('N06',D('1e-12'),D('1e-16'))]+[(f'ulp-{m}',D(2)**-31*D(m),D('1e-16')) for m in ['.25','.5','.75','1','1.5','2']]:
            af=float(a); kf=float(k); tf=float(t); aa=D.from_float(af); diag=D.from_float(af+kf); tt=D.from_float(tf); stored=diag-aa
            row={'id':label,'a_intended':a,'k_intended':k,'T_intended':t,'a_binary64_hex':af.hex(),'diag_binary64_hex':float(diag).hex(),'T_binary64_hex':tf.hex(),'K_stored_exact':[[diag,-aa],[-aa,aa]],'f_stored_exact':[D(0),tt],'k_stored_exact':stored,'intended_root':t/k,'intended_tip':t/k+t/a,'k_relative_loss':(stored-k)/k}
            if stored:
                root=tt/stored; tip=root+tt/aa
                row.update({'stored_exact_root':root,'stored_exact_tip':tip,'root_relative_physical_error':root/(t/k)-1,'physical_spring_action':-k*root,'stored_residual':residual(row['K_stored_exact'],[root,tip],[D(0),tt],[0,1])})
            else: row['stored_status']='singular; physical positive spring remains; numerical assembly unresolved'
            sweep.append(row)
        out['NP']['A']=sweep
        out['NP']['B']=[]
        for order in [8,32,64,80,128]:
            for coefficient in [D('.75'),D('.750000000001')]:
                c=[[D(0)]*order for _ in range(order)]
                for i in range(order):
                    c[i][i]=1
                    for j in [i-1,i-2]:
                        if j>=0:c[i][j]=coefficient
                entries=[]
                for i in range(order):
                    for j in range(max(0,i-2),min(order,i+3)):
                        entries.append([i,j,sum((c[i][h]*c[j][h] for h in range(order)),D(0))])
                z=[]
                for i in range(order):z.append(1+sum((coefficient*z[j] for j in [i-1,i-2] if j>=0),D(0)))
                out['NP']['B'].append({'n':order,'coefficient':coefficient,'K_entries':entries,'u_exact':[1]*order,'f_exact':[sum((v for i,j,v in entries if i==row),D(0)) for row in range(order)],'comparison_bound_z':z[-1],'z_squared_u':z[-1]**2*D(2)**-53,'orderings':{'natural':list(range(order)),'reverse':list(reversed(range(order))),'even_odd':list(range(0,order,2))+list(range(1,order,2))}})
        out['NP']['C']={'family':'synthetic passive connector generalized coordinates; six grounded rigid coordinates plus independent internal slip','rigid_restraint_B':[[int(i==j) for j in range(6)] for i in range(6)],'K_internal_mechanism_diagonal':[1]*6+[0],'null_vector':[0]*6+[1],'K_stabilized_diagonal':[1]*7,'near_collinear_rows':[[1,0,0,0,0,0],[1,'1e-16',0,0,0,0]],'near_collinear_exact_rank':2}
        out['NP']['D']={'skew':{'K':[[2,1],[D('1.01'),2]],'must_block':'material asymmetry'},'duplicate_cancellation':{'entries':[[0,0,'1e16'],[0,0,1],[0,0,'-1e16']],'exact_sum':1},'omitted_spring':'R05','lost_stabilizer':'NP-A/N06','prescribed':'R04','constrained':'R06','range_cases':[{'name':'overflow','operands':['1e308','1e308'],'operation':'multiply'},{'name':'underflow','operands':['0x0.0000000000001p-1022','0.5'],'operation':'multiply'},{'name':'nonfinite','values':['NaN','+Infinity','-Infinity']}],'lift_off':{'active_K_diagonal':[1,1],'inactive_K_diagonal':[1,0],'inactive_null':[0,1]}}
        return out


def encoded():
    return json.dumps(build(),default=lambda x:str(x) if isinstance(x,D) else x,indent=2,sort_keys=True)+'\n'

if __name__=='__main__':
    Path(__file__).with_name('fixtures.json').write_text(encoded())
