"""Independent decimal analytical constants; imports no product mechanics.
Reference expressions: CORRECTNESS_DESIGN/NUMERICAL_REFERENCE.md.
Preparation only; no operational numerical-policy threshold is adopted here.
"""
from decimal import Decimal as D, localcontext
import json

def atan_inverse(n):
    x = D(1) / n
    term, total, index = x, x, 1
    while True:
        term *= -x*x
        add = term / (2*index+1)
        previous = total
        total += add
        if previous == total:
            return total
        index += 1

def oracle():
    with localcontext() as ctx:
        ctx.prec = 70
        pi = 16*atan_inverse(D(5))-4*atan_inverse(D(239))
        od, inner, e, g = D('.20'), D('.18'), D('200e9'), D('80e9')
        area = pi*(od**2-inner**2)/4
        inertia = pi*(od**4-inner**4)/64
        polar = 2*inertia
        # Independent integrated annulus relations, with symbolic pi coefficients checked.
        assert abs(area/pi-D('.0019')) < D('1e-65')
        assert abs(inertia/pi-D('.0000085975')) < D('1e-65')
        ea, ei, gj = e*area, e*inertia, g*polar
        length=D(2); force=D(1000); torque=D(1)
        a=gj/length; k=D('1e-4'); small_t=D('1e-8')
        s=(a/(a+k)).sqrt()
        rows={
          'area_m2':area,'inertia_m4':inertia,'polar_m4':polar,
          'N01_tip_y_m':force*length**3/(3*ei),
          'N01_tip_rz_rad':force*length**2/(2*ei),
          'N01_root_fy_N':-force,'N01_root_mz_Nm':-force*length,
          'N05_root_rx_rad':small_t/k,'N05_tip_rx_rad':small_t/k+small_t/a,
          'N05_spring_reaction_Nm':-small_t,'N05_scaled_rcond_2_exact':(1-s)/(1+s),
          'N08_tip_rx_rad':torque*length/gj,'N08_root_mx_Nm':-torque,
          'N09_tip_y_m':D(100)*D(10)**3/(3*ei),
          'N09_tip_rz_rad':D(100)*D(10)**2/(2*ei),
          'N09_torque_tip_rx_rad':D('.1')*10/gj,
          'R01_original_free_residual_N':D(10),'R01_eta_c':D(1)/201,
          'R03_original_free_residual_N':D(-1000),'R03_eta_c':D(1),
          'R04_middle_ux_m':D('1e-4')/2,'R04_root_fx_N':ea*D('1e-4')/2,
        }
        return {'status':'prepared_not_runtime_acceptance','arithmetic':'Decimal precision 70; Machin pi; no production import',
          'expected_decimal':{key:str(value) for key,value in rows.items()},
          'exact_classifications':{'N02':{'inertia':[5,0,1],'rigid_constraint_rank':5},'N03_RX':{'inertia':[5,0,0]},'N03_RZ':{'inertia':[4,0,1]},'N04':{'inertia':[12,0,6]},'N07':{'inertia':[1,1,0],'negative_vector':[1,-1],'quadratic_value':-2}},
          'policy':'No rank, pivot, residual or sensitivity screen constant selected by this artifact.'}

if __name__ == '__main__':
    print(json.dumps(oracle(),indent=2,sort_keys=True))
