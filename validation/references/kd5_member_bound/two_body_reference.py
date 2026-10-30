"""Maintained exact RN128 reference for the invented two-body notice control.

Each disconnected body has twelve free DOFs. The tiny nodal load is resolved,
although every group can meet the cancellation-scale noise condition. Product
code never imports this independent test reference.
"""
import argparse, json, hashlib, math
from fractions import Fraction as F
from pathlib import Path
here=Path(__file__).resolve().parent
helper=here/'notice_reference.py'
helper_bytes=helper.read_bytes(); ns={}
exec(compile(helper_bytes.decode().split('# Invented control;')[0],str(helper),'exec'),ns)
request_path=here/'two_body_request.json'
request_bytes=request_path.read_bytes(); wire=json.loads(request_bytes); model=wire['model']
normalized={'As_m2':.005969026041820607,'I_m4':2.7009842839238257e-5,'J_m4':5.4019685678476514e-5}
sections={p['id']:dict(normalized) for p in model['pipe_segments']}
nodes={n['id']:n for n in model['nodes']}; pipes={p['id']:p for p in model['pipe_segments']}
assert len(nodes)==6 and len(pipes)==4 and len(model['supports'])==2
assert model['components']==[] and model['combinations']==[]
material=model['materials'][0]
assert len(model['materials'])==1 and material['id']=='material'
assert material['elastic_modulus']=={'value':200e9,'unit':'Pa'} and material['poisson_ratio']=={'value':.25,'unit':'1'}
E=F.from_float(material['elastic_modulus']['value']);G=F.from_float(80e9)
case=model['load_cases'][0];state=case['analysis_state']; assert len(model['load_cases'])==1
assert case['pressure_regions']==[] and len(case['primitive_loads'])==1
load=case['primitive_loads'][0]
assert load['id']=='tiny-resolved' and load['direction']=='global_y' and load['magnitude']=={'value':1e-7,'unit':'N'}
assert load['target']=={'type':'node','node':'free:b'} and load['dimension']=='force'
assert state['load_sources']==[{'source_ref':'tiny-resolved','factor':1.0}]
intended_nodal=ns['mul'](F.from_float(load['magnitude']['value']),F(1))
elements={e['pipe_ref']:e for e in state['element_states']};supports={s['id']:s for s in model['supports']};support_states={s['support_ref']:s for s in state['support_states']}
reference=model['reference_configurations'][0];refs={r['pipe_ref']:r for r in reference['member_references']}
assert len(elements)==4 and len(refs)==4
summaries={}
for suffix in ('',':b'):
 node_ids=[n+suffix for n in ('root','tip','free')]
 positions=[[F.from_float(float(nodes[n]['position'][a])) for a in ('x','y','z')] for n in node_ids]
 shift=100 if suffix else 0
 assert [[float(x) for x in p] for p in positions]==[[5e6+shift,7.3e6,1.2e6],[5e6+3+shift,7.3e6+4,1.2e6+5],[5e6-1+shift,7.3e6+7,1.2e6+5]]
 support=supports['anchor'+suffix];ss=support_states['anchor'+suffix]
 assert support['family']=='anchor' and support['node']==node_ids[0] and set(support['restraints'])=={'UX','UY','UZ','RX','RY','RZ'}
 assert ss['participation']=={'kind':'active_model_device'}
 phi=[F.from_float(q['value']['value']) for q in ss['boundary_motion']]
 assert [q['dof'] for q in ss['boundary_motion']]==['RX','RY','RZ'] and list(map(float,phi))==[.01,-.005,.007]
 K=[[F(0) for _ in range(18)] for _ in range(18)];f=[F(0)]*18;members=[]
 for ni,nj,name in ((0,1,'member'+suffix),(1,2,'member2'+suffix)):
  pipe=pipes[name];section=sections[name];element=elements[name]
  assert pipe['from']==node_ids[ni] and pipe['to']==node_ids[nj] and pipe['material']=='material'
  assert pipe['section']=={'outside_diameter':{'value':.2,'unit':'m'},'wall_thickness':{'value':.01,'unit':'m'}}
  assert pipe['y_reference']=={'x':.3,'y':.1,'z':1.}
  assert {k:section[k] for k in ('As_m2','I_m4','J_m4')}=={'As_m2':.005969026041820607,'I_m4':2.7009842839238257e-5,'J_m4':5.4019685678476514e-5}
  assert element['material_selection']['kind']=='explicit_base_properties' and element['material_selection']['material_ref']=='material'
  assert element['thermal_state']['kind']=='explicit_interval_strain' and element['thermal_state']['strain']=={'value':.00012,'unit':'1'}
  assert refs[name]['fit']=={'kind':'none'} and refs[name]['basis']=={'kind':'direct_strain_reference'}
  eps=F.from_float(element['thermal_state']['strain']['value']);A,I,J=[F.from_float(section[k]) for k in ('As_m2','I_m4','J_m4')]
  yr=[F.from_float(pipe['y_reference'][a]) for a in ('x','y','z')];d=[b-a for a,b in zip(positions[ni],positions[nj])]
  ke,fe,R,L=ns['frame'](d,yr,A,I,J,E,G,eps);mp=list(range(6*ni,6*ni+6))+list(range(6*nj,6*nj+6))
  for i,gi in enumerate(mp):
   f[gi]+=fe[i]
   for j,gj in enumerate(mp):K[gi][gj]+=ke[i][j]
  members.append((name,ke,fe,R,L,mp))
 if suffix:f[13]+=intended_nodal
 uc=[F(0)]*3+phi;free=list(range(6,18));rhs=[f[i]-sum(K[i][j]*uc[j] for j in range(6)) for i in free]
 U=uc+ns['solve']([[K[i][j] for j in free] for i in free],rhs)
 assert all(sum(K[i][j]*U[j] for j in range(18))==f[i] for i in free)
 reaction=[sum(K[i][j]*U[j] for j in range(18))-f[i] for i in range(6)]
 expected_force=[F(0),-intended_nodal if suffix else F(0),F(0)]
 assert reaction[:3]==expected_force
 force_sq=[];moment_sq=[]
 def groups(v):
  a,s,t,m=ns['sections_rows'](v);force_sq.extend([sum(x*x for x in a),sum(x*x for x in s)]);moment_sq.extend([sum(x*x for x in t),sum(x*x for x in m)])
 for name,ke,fe,R,L,mp in members:
  vg=[sum(ke[r][c]*U[mp[c]] for c in range(12))-fe[r] for r in range(12)]
  vl=[sum(R[r%3][c]*vg[3*(r//3)+c] for c in range(3)) for r in range(12)]
  groups(vl[:6]);groups(vl[6:])
  for frac in (F(0),F(1,4),F(1,2),F(3,4),F(1)):
   x=L*frac;v=[-z for z in vl[:6]];v[4]-=x*vl[2];v[5]+=x*vl[1];groups(v)
 force_sq.append(sum(x*x for x in reaction[:3]));moment_sq.append(sum(x*x for x in reaction[3:]))
 summaries['body_b' if suffix else 'body_a']={'free_equations_exact':True,'force_group_count':len(force_sq),'moment_group_count':len(moment_sq),'reaction_force_exact':[str(v) for v in reaction[:3]],'reaction_moment_display_nm':list(map(float,reaction[3:])),'max_force_group_display_n':float(ns['display_norm_squared'](max(force_sq))),'max_moment_group_display_nm':float(ns['display_norm_squared'](max(moment_sq))),'force_norm_upper_n':ns['dyadic_norm_upper'](max(force_sq)),'moment_norm_upper_nm':ns['dyadic_norm_upper'](max(moment_sq)),'force_squared_max':str(max(force_sq)),'moment_squared_max':str(max(moment_sq))}
 if suffix: loaded_reaction=reaction
result={'control_request_sha256':hashlib.sha256(request_bytes).hexdigest(),
        'basis':'exact rational solve of independently formed RN128 stiffness and loads',
        'normalized_sections':sections,'elastic_modulus_pa':float(E),
        'shear_modulus_pa':float(G),'bodies':summaries}
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--write-oracle',action='store_true',help='Replace the oracle after independent review of the complete invented control.')
args=parser.parse_args()
if args.write_oracle:
 (here/'two_body_oracle.json').write_text(json.dumps(result,indent=2)+'\n')
else:
 assert json.loads((here/'two_body_oracle.json').read_text())==result,'maintained two-body oracle differs from exact reference'
print('control_sha256='+result['control_request_sha256']+'; both exact twelve-free-DOF body references verified')
