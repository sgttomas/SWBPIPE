#!/usr/bin/env python3
"""Original static field expectations. No product/solver imports or execution.
Reuses the checked NUM annulus, axial/torsion/EB compatibility and STRESS_REFERENCE
sign/circular-extremum derivations. New explicit E/nu fixture variant.
"""
from decimal import Decimal as D, localcontext
from pathlib import Path
import json, hashlib
ROOT=Path(__file__).parent

def atan_inverse(n):
    x=D(1)/n;term=total=x;index=1
    while True:
        term *= -x*x
        add=term/(2*index+1);previous=total;total+=add
        if previous==total:return total
        index+=1

def numbers(precision=90):
    with localcontext() as ctx:
        ctx.prec=precision
        pi=16*atan_inverse(D(5))-4*atan_inverse(D(239))
        ro,ri,L,E,nu=D('.1'),D('.09'),D(2),D('200e9'),D('.25')
        G=E/(2*(1+nu));A=pi*(ro*ro-ri*ri);I=pi*(ro**4-ri**4)/4;J=2*I;Z=I/ro
        assert G==D('80e9') and abs(A/pi-D('.0019'))<D(10)**(-precision+5)
        assert abs(I/pi-D('.0000085975'))<D(10)**(-precision+5)
        properties={'L_m':L,'ro_m':ro,'ri_m':ri,'E_Pa':E,'nu':nu,'G_Pa':G,'A_m2':A,'I_m4':I,'J_m4':J,'Z_m3':Z,'pi':pi}
        cases={}
        for case in ['axial','bending_torsion']:
            if case=='axial':Fx,Fy,Fz,T=D(1000),D(0),D(0),D(0)
            else:Fx,Fy,Fz,T=D(0),D(30),D(-40),D(20)
            Fnorm=(Fx*Fx+Fy*Fy+Fz*Fz).sqrt()
            support=[-Fx,-Fy,-Fz,-T,Fz*L,-Fy*L]
            tip=[Fx*L/(E*A),Fy*L**3/(3*E*I),Fz*L**3/(3*E*I),T*L/(G*J),-Fz*L**2/(2*E*I),Fy*L**2/(2*E*I)]
            refs=[]
            def add(id,value,unit,dimension,category,zero='derived',formula=None):
                refs.append({'assertion_id':id,'value':float(value),'expected_decimal':str(value),'unit':unit,'dimension':dimension,'category':category,'mathematically_zero':value==0,'zero_policy':'exact_prescribed' if value==0 and zero=='prescribed' else ('case_scaled_absolute' if value==0 else 'relative_1e-9_candidate'),'expression':formula})
            for node,values in [('root',[D(0)]*6),('tip',tip)]:
                for j,(dof,v) in enumerate(zip(['ux','uy','uz','rx','ry','rz'],values)):
                    add('node.'+node+'.'+dof,v*1000 if j<3 else v,'mm' if j<3 else 'rad','length' if j<3 else 'angle','nodal',zero='prescribed' if node=='root' else 'derived',formula='axial FL/EA; EB transverse FL^3/(3EI); torsion TL/GJ; signed EB rotation')
                add('node.'+node+'.magnitude',sum(v*v for v in values[:3]).sqrt()*1000,'mm','length','nodal_magnitude',zero='prescribed' if node=='root' else 'derived')
            for j,(name,v) in enumerate(zip(['Fx','Fy','Fz','Mx','My','Mz'],support)):
                add('support.'+name,v,'N' if j<3 else 'N*m','force' if j<3 else 'moment','support',formula='R_F=-F; R_M=-(T e_x + (L e_x) cross F)')
            add('support.force_magnitude',Fnorm,'N','force','support_magnitude')
            add('support.moment_magnitude',sum(v*v for v in support[3:]).sqrt(),'N*m','moment','support_magnitude')
            stations=[]
            for name,t in [('end_i',D(0)),('quarter_1',D('.25')),('midspan',D('.5')),('quarter_3',D('.75')),('end_j',D(1))]:
                x=t*L;section=[Fx,Fy,Fz,T,-Fz*(L-x),Fy*(L-x)]
                # Published endpoint actions are node-on-element. Interior rows
                # are positive-x cut actions; stress always uses cut actions.
                pub=[-v for v in section] if name=='end_i' else section
                for j,(component,v) in enumerate(zip(['N','Vy','Vz','T','My','Mz'],pub)):
                    add('section.'+name+'.'+component,v,'N' if j<3 else 'N*m','force' if j<3 else 'moment','section_action',formula='endpoint i=-q_cut(0); endpoint j=q_cut(L); interiors=q_cut(x)')
                stress=[section[0]/A,section[4]/Z,section[5]/Z,section[3]*ro/J]
                for component,v in zip(['axial','bending_y','bending_z','torsional_shear'],stress):
                    add('stress.'+name+'.'+component,v/D(1000000),'MPa','stress','stress_component',formula='N/A; My/Z; Mz/Z; T*ro/J, each converted Pa→MPa once')
                bend=(section[4]**2+section[5]**2).sqrt()/Z
                stations.append({'name':name,'fraction':str(t),'x_m':str(x),'cut_actions_decimal':[str(v) for v in section],'sigma_plus_Pa':str(stress[0]+bend),'sigma_minus_Pa':str(stress[0]-bend),'normal_abs_Pa':str(abs(stress[0])+bend),'torsional_shear_Pa':str(stress[3])})
            maxnormal=abs(Fx/A)+((Fz*L)**2+(Fy*L)**2).sqrt()/Z
            add('stress.maximum_absolute_normal',maxnormal,'Pa','stress','normal_maximum',formula='max_s |N/A|+hypot(My(s),Mz(s))/Z; torsion excluded')
            if case=='axial':
                location={'kind':'all_stations_tied','domain_fraction':['0','1'],'allowed_reported_witness':'any in-domain station; never claim unique','circumference':'all fibers tied','sigma_plus_equals_sigma_minus':True}
                scales={'length_mm':abs(tip[0])*1000,'angle_rad':abs(tip[0])/L,'force_N':abs(Fx),'moment_Nm':abs(Fx)*L,'stress_Pa':maxnormal,'stress_MPa':maxnormal/D(1000000)}
            else:
                location={'kind':'strict_endpoint','station_fraction':'0','node':'root','fiber_sigma_plus_yz_m':['-0.06','0.08'],'fiber_sigma_minus_yz_m':['0.06','-0.08'],'absolute_fiber_tie':'two opposite fibers because N=0; axial station unique','sigma_plus_equals_sigma_minus':False}
                scales={'length_mm':Fnorm*L**3/(3*E*I)*1000,'angle_rad':Fnorm*L**2/(2*E*I),'force_N':Fnorm,'moment_Nm':Fnorm*L,'stress_Pa':maxnormal,'stress_MPa':maxnormal/D(1000000)}
            # Equilibrium checks derive from the free body, no assembled K used.
            assert all(support[j]+v==0 for j,v in enumerate([Fx,Fy,Fz]))
            assert support[3]+T==0 and support[4]-Fz*L==0 and support[5]+Fy*L==0
            assert len(refs)==73 and len({v['assertion_id'] for v in refs})==73
            assert maxnormal==D(stations[0]['normal_abs_Pa'])
            cases[case]={'reference_kind':'analytical','readiness':'pending_independent_review_and_transport_admission','constitutive_variant':'original_NUM_annulus_E200GPa_nu0.25_derivedG80GPa','values':refs,'continuous_fields':{'u_x':'Fx*x/(E*A)','u_y':'Fy*x^2*(3L-x)/(6EI)','u_z':'Fz*x^2*(3L-x)/(6EI)','theta_x':'T*x/(GJ)','theta_y':'-Fz*x*(2L-x)/(2EI)','theta_z':'Fy*x*(2L-x)/(2EI)','positive_cut_actions':'[Fx,Fy,Fz,T,-Fz*(L-x),Fy*(L-x)]'},'stations':stations,'governing_normal':location,'maximum_displacement':{'value_mm':str(sum(v*v for v in tip[:3]).sqrt()*1000),'node':'tip','strictly_increasing_magnitude_on_span':True},'zero_scales_decimal':{k:str(v) for k,v in scales.items()},'criterion_note':'New per-field development comparison: nonzero relative1e-9 candidate; derived zeros absolute1e-9 times named dimensional case scale; prescribed zeros exact. Independent review required. Historical protected assertions/method1e-9 unchanged.'}
        return {'properties_decimal':{k:str(v) for k,v in properties.items()},'cases':cases}

def main():
    a=numbers(80);b=numbers(100)
    for cid in a['cases']:
        for low,high in zip(a['cases'][cid]['values'],b['cases'][cid]['values']):
            assert low['assertion_id']==high['assertion_id']
            with localcontext() as c:
                c.prec=110
                lv,hv=D(low['expected_decimal']),D(high['expected_decimal'])
                assert lv==hv==0 or abs((lv-hv)/hv)<D('1e-70')
    for cid,data in b['cases'].items():
        data.update({'format':'openpipestress.qualification_reference_values/1','case_id':'original_static_'+cid+'_v1','basis':'independent_reference.py; checked NUMERICAL_REFERENCE and STRESS_REFERENCE mathematical subsets; no production imports','independent_review_ref':None})
        (ROOT/(cid+'.expectations.json')).write_text(json.dumps(data,indent=2)+'\n')
        carrier={k:data[k] for k in ['format','case_id','readiness','basis','reference_kind','independent_review_ref']}
        carrier['values']=[{k:v[k] for k in ['assertion_id','value','unit']} for v in data['values']]
        (ROOT/(cid+'.reference.candidate.json')).write_text(json.dumps(carrier,indent=2)+'\n')
    (ROOT/'section.reference.json').write_text(json.dumps({'arithmetic':'Decimal100; independent Machin pi, annular area integration','values':b['properties_decimal']},indent=2)+'\n')
    report={'status':'PASS reference arithmetic only','cases':2,'scalar_row_expectations_per_case':73,'total_scalar_row_expectations':146,'precision_check':'80 versus100 decimal digits; nonzero relative difference below1e-70; exact zeros retained','formula_checks':'section coefficients, complete force/moment equilibrium, station maximum','not_run':['product solve','native','external solver','thin gate'],'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    (ROOT/'_run_records/REFERENCE_ARITHMETIC.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report))
if __name__=='__main__':main()
