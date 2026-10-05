"""RV62 fresh oracle: exact Fraction arithmetic, Machin 16 atan(1/5)-4 atan(1/239),
integer-square-root enclosures, fixed axial x cantilever statics from captured loads.
No product or author-oracle import. Only the actual log is an input.
"""
from pathlib import Path
from fractions import Fraction as F
import math,struct,json,sys,hashlib
H=lambda x:struct.pack('>d',x).hex()
D=lambda x:struct.unpack('>d',bytes.fromhex(x))[0]
def point(a):return (F(a),F(a))
def plus(a,b):return(a[0]+b[0],a[1]+b[1])
def times(a,b):
 v=[x*y for x in a for y in b];return(min(v),max(v))
def over(a,b):
 assert b[0]>0;return times(a,(1/b[1],1/b[0]))
def join(a,b):return(min(a[0],b[0]),max(a[1],b[1]))
def sqroot1(x):
 assert x>=0;q=1<<512;z=math.isqrt(x.numerator*q*q//x.denominator);lo=F(z,q)
 return (lo,lo) if lo*lo==x else(lo,F(z+1,q))
def sqroot(a):return(sqroot1(a[0])[0],sqroot1(a[1])[1])
def hypot2(a,b):return sqroot(plus(times(a,a),times(b,b)))
def atan(q):
 s=sum((F((-1)**i,(2*i+1)*q**(2*i+1)) for i in range(160)),F(0));return(s,s+F(1,321*q**321))
PI=plus(times(point(16),atan(5)),times(point(-4),atan(239)))
def err(y,bounds):
 y=F(y);return(max(F(0),bounds[0]-y,y-bounds[1]),max(abs(y-bounds[0]),abs(y-bounds[1])))
def up(x):
 n=float(x);return math.nextafter(n,math.inf) if F(n)<x else n
def si(r):return r['value']/1000 if r['unit']=='mm' else r['value']*1e6 if r['unit']=='MPa' else r['value']
def raw(x,r):return times(x,point(1000)) if r['unit']=='mm' else over(x,point(1000000)) if r['unit']=='MPa' else x
def family(r):
 k=r['kind']
 if k.startswith('global_nodal_displacement_') or k=='displacement_magnitude':return 0
 if k.startswith('global_nodal_rotation_'):return 1
 if k.startswith('element_local_') and 'force' in k:return 2
 if k.startswith('element_local_') and 'moment' in k:return 3
 if k=='support_reaction_component_v2':return 2 if r['metadata']['component'][0]=='F' else 3
 if k=='support_reaction_force_magnitude_v2':return 2
 if k=='support_reaction_moment_magnitude_v2':return 3
 return None

def inputs(rec):
 req=rec['REQUEST'];m=req['model'];lc=m['load_cases'];assert len(lc)==1;case=lc[0];nodes=m['nodes'];pipes=m['pipe_segments'];assert len(nodes)==2 and len(pipes)==1
 pos=[[n['position'][a] for a in 'xyz'] for n in nodes];assert pos==rec['INPUT']['nodes']==[[0,0,0],[1,0,0]]
 assert m['project']['units']['length']=='m';p=pipes[0];assert p['from']==nodes[0]['id'] and p['to']==nodes[1]['id']
 assert p['y_reference']=={'x':0.,'y':1.,'z':0.}
 supports=m['supports'];assert len(supports)==1 and supports[0]['node']==nodes[0]['id'] and set(supports[0]['restraints'])=={'UX','UY','UZ','RX','RY','RZ'}
 loads=case['primitive_loads'];assert len(loads)==3
 values={}
 for x in loads:
  direction=x['direction'];assert direction not in values and x['target']=={'node':nodes[1]['id'],'type':'node'}
  assert x['magnitude']['unit']==('N*m' if direction=='RX' else 'N');values[direction]=F(x['magnitude']['value'])
 assert set(values)=={'UX','UY','RX'}
 sec=p['section'];assert sec['outside_diameter']['unit']==sec['wall_thickness']['unit']=='m'
 inp=rec['INPUT'];assert (H(inp['D']),H(inp['t']))==(H(sec['outside_diameter']['value']),H(sec['wall_thickness']['value']))
 mats=req['materials'];sel=rec['SELECTION'];assert len(mats)==1 and p['material']==mats[0]['id']==sel['material'];mat=mats[0]
 assert sel['material_ordinal']==0 and sel['case']==case['id'];pts=mat['temperature_points'];v=sel['values']
 base=sel['base'];assert len(base)==1 and base[0]['id']==mat['id']
 assert(base[0]['e'],base[0]['g'])==(H(mat['elastic_modulus']['value']),H(mat['shear_modulus']['value']))
 source=[];represented=[];details={}
 if 'modulus_basis_ref' in case:
  ordinal=next(i for i,x in enumerate(pts) if x['id']==case['modulus_basis_ref']);chosen=pts[ordinal]
  assert v['kind']=='point' and v['ordinal']==ordinal and sel['point_ids']==[chosen['id'],None]
  for prop,key in [('elastic_modulus','e'),('shear_modulus','g')]:
   x=chosen[prop];assert x['unit']=='Pa';assert H(x['value'])==v[key];source.append(F(x['value']));represented.append(x['value'])
  assert sel['source_alpha']==[H(chosen['thermal_expansion_coefficient']['value']),None] and sel['alpha']==sel['source_alpha'][0]
 else:
  t=F(case['modulus_basis_temperature']['value']);assert case['modulus_basis_temperature']['unit']=='K'
  eligible=[(i,x) for i,x in enumerate(pts) if 'temperature' in x]
  lo=max(((i,x) for i,x in eligible if F(x['temperature']['value'])<t),key=lambda z:z[1]['temperature']['value'])
  hi=min(((i,x) for i,x in eligible if F(x['temperature']['value'])>t),key=lambda z:z[1]['temperature']['value'])
  il,l=lo;ih,h=hi;tl=F(l['temperature']['value']);th=F(h['temperature']['value']);span=th-tl
  assert v['kind']=='interpolated' and(v['lower'],v['upper'])==(il,ih) and sel['point_ids']==[l['id'],h['id']]
  assert [v[k] for k in ['t_lo','t','t_hi']]==[H(float(z)) for z in [tl,t,th]]
  w=float(F(float(t-tl))/F(float(span)))
  for prop,key in [('elastic_modulus','e'),('shear_modulus','g')]:
   a=F(l[prop]['value']);b=F(h[prop]['value']);assert [v[key+'_lo'],v[key+'_hi']]==[H(float(a)),H(float(b))]
   # Independently form the exact four-product numerator of the selected law.
   products=[th*a,-t*a,t*b,-tl*b];q=sum(products)/span
   represented_value=float(a+F(float(F(w)*F(float(b-a)))))
   assert H(represented_value)==v[key+'_hat'];source.append(q);represented.append(represented_value)
   details[key]={'products':list(map(str,products)),'denominator':str(span),'source':str(q),'represented':H(represented_value),'difference':str(q-F(represented_value))}
  aa=[z['thermal_expansion_coefficient']['value'] for z in [l,h]];assert sel['source_alpha']==list(map(H,aa))
  assert sel['alpha']==H(float(F(aa[0])+F(float(F(w)*F(float(F(aa[1])-F(aa[0])))))))
 assert tuple(map(H,represented))==(H(inp['E']),H(inp['G'])) and all(x>0 for x in source)
 assert H(inp['E'])!=base[0]['e'] and H(inp['G'])!=base[0]['g']
 return {'case':case,'nodes':nodes,'pipe':p,'support':supports[0],'loads':values,'source':source,'represented':represented,'details':details}

def law(rec,basis,r,source):
 inp=rec['INPUT'];d=F(inp['D']);t=F(inp['t']);L=F(1);fx,fy,tx=[basis['loads'][k] for k in ['UX','UY','RX']]
 if source:
  e,g=basis['source'];a=times(PI,point((d*d-(d-2*t)**2)/4));i=times(PI,point((d**4-(d-2*t)**4)/64));j=times(point(2),i);z=over(i,point(d/2));c=point(d/2)
 else:
  e,g=map(F,basis['represented']);a,i,j,c=[point(inp[x]) for x in ['A','I','J','c']];z=join(point(inp['Z']),over(i,point(d/2)))
 motions=[over(point(fx*L),times(point(e),a)),over(point(fy*L**3),times(point(3*e),i)),point(0),over(point(tx*L),times(point(g),j)),point(0),over(point(fy*L**2),times(point(2*e),i))]
 kind=r['kind'];node=[n['id'] for n in basis['nodes']];loc=r.get('metadata',{}).get('location');component=r.get('metadata',{}).get('component')
 if kind.startswith('global_nodal_'):
  idx=('xyz'.index(kind[-1]))+(3 if 'rotation' in kind else 0);return point(0) if r['entity_ref']==node[0] else motions[idx]
 if kind=='displacement_magnitude':return point(0) if r['entity_ref']==node[0] else hypot2(motions[0],motions[1])
 reactions={'Fx':-fx,'Fy':-fy,'Fz':F(0),'Mx':-tx,'My':F(0),'Mz':-fy*L}
 if kind=='support_reaction_component_v2':return point(reactions[component])
 if kind=='support_reaction_force_magnitude_v2':return sqroot(point(fx*fx+fy*fy))
 if kind=='support_reaction_moment_magnitude_v2':return sqroot(point(tx*tx+(fy*L)**2))
 station={'end_i':F(0),'end_j':L,'quarter_1':L/4,'midspan':L/2,'quarter_3':3*L/4}
 if kind=='pipe_elastic_normal_stress_maximum_v2':return plus(over(point(abs(fx)),a),over(point(abs(fy*L)),z))
 assert loc in station;arm=L-station[loc]
 actions={'element_local_axial_force':fx,'element_local_shear_force_y':fy,'element_local_shear_force_z':F(0),'element_local_torsional_moment':tx,'element_local_bending_moment_y':F(0),'element_local_bending_moment_z':fy*arm}
 if kind in actions:return point(-actions[kind] if loc=='end_i' else actions[kind])
 if kind=='element_local_axial_normal_stress':return over(point(fx),a)
 if kind=='element_local_bending_normal_stress_y':return point(0)
 if kind=='element_local_bending_normal_stress_z':return over(point(fy*arm),z)
 if kind=='element_local_torsional_shear_stress':return over(times(point(tx),c),j)
 raise AssertionError(kind)

def verify(rec):
 b=inputs(rec);inp=rec['INPUT'];rows=rec['ROWS'];vs=rec['VERDICTS'];assert len(rows)==len(vs)==75
 assert len({r['id'] for r in rows})==len(rows)
 assert rec['BOUNDARY']['calls']==[1,1,1] and rec['BOUNDARY']['error']=='None'
 assert len(rec['NATIVE_ROWS'])==52 and len({r['quantity'] for r in rec['NATIVE_ROWS']})==52
 assert all(r['ordinal']==j and r['body']==0 for j,r in enumerate(rec['NATIVE_ROWS']))
 assert rec['NATIVE_IDENTITY']['calls']==1
 ancillary={'linear_solver_mode_basis','modulus_basis_record'};assert sorted(r['kind'] for r in rows if r['kind'] in ancillary)==sorted(ancillary)
 norms=[0.0]*4
 for r in rows:
  f=family(r)
  if f is not None and not(r['kind'].startswith('global_nodal_') and r['entity_ref']==b['nodes'][0]['id']):norms[f]=max(norms[f],abs(si(r)))
 scale=[max(norms[:2])]*2+[max(norms[2:])]*2
 outcomes=[];proved_passes=0;truth_misses=0;conservative_rows=0;conservative_predicates=0
 for n,(r,v) in enumerate(zip(rows,vs)):
  assert v['row']==n and v['normalized_bits']==H(si(r))
  assert r['basis_ref']=={'ref_type':'load_case','ref_id':b['case']['id']}
  if r['kind'] in ancillary:
   assert v['class']=='nonquantity' and v['scale_bits']=='0000000000000000' and v['predicates']==[None]*4 and v['passed'] and v['failed'] is None;continue
  f=family(r);y=si(r)
  s=scale[f] if f is not None else scale[2]/inp['A']+(D('4006a09e667f3bcd') if r['kind']=='pipe_elastic_normal_stress_maximum_v2' else 1.0)*(scale[3]/inp['Z'])
  assert v['scale_bits']==H(s)
  input_row=r['kind'].startswith('global_nodal_') and r['entity_ref']==b['nodes'][0]['id']
  cls='input' if input_row else 'absolute' if s<2.0**-988 or abs(y)<s*2.0**-34 else 'relative';assert v['class']==cls
  S=law(rec,b,r,True);K=law(rec,b,r,False);si_errors=[err(y,z) for z in [S,K]]
  low=max(e[0] for e in si_errors);high=max(e[1] for e in si_errors)
  if cls=='input':tests=[('InputDerived',low,high,F(0))]
  elif cls=='absolute':
   bound=up(F(s)*F(2)**-64) if s else 0.
   if 0<s<2.0**-988:bound=up(F(bound)+F(up(F(2)**-53*abs(F(y))))+F(2)**-1074)
   assert v['bound_bits']==H(bound);tests=[('Absolute',low,high,F(bound))]
  else:
   exact=F(2)**-64*max(abs(F(y)),F(s))*(1+F(2)**-21)+F(2)**-53*abs(F(y))+F(2)**-1074
   rounded=((2.0**-64)*max(abs(y),s))*(1+2.0**-21)+((2.0**-53)*abs(y)+2.0**-1074)
   raw_errors=[err(r['value'],raw(z,r)) for z in [S,K]]
   tests=[('SharperExact',low,high,exact),('SharperBinary64',low,high,F(rounded)),('DecimalSi',low,high,abs(F(y))/10**9),('DecimalRaw',max(e[0] for e in raw_errors),max(e[1] for e in raw_errors),abs(F(r['value']))/10**9)]
  actual=[x for x in v['predicates'] if x is not None];assert len(actual)==len(tests)
  assert v['passed']==all(actual) and v['failed']==next((t[0] for t,p in zip(tests,actual) if not p),None)
  proofs=[]
  for (name,lo,hi,allow),candidate in zip(tests,actual):
   assert hi<=allow or lo>allow,(n,name,'unresolved truth interval')
   if candidate:assert hi<=allow,(n,name,'uncertified candidate PASS');proved_passes+=1
   elif hi<=allow:conservative_predicates+=1
   proofs.append({'name':name,'lower':str(lo),'upper':str(hi),'allowance':str(allow),'candidate':candidate,'truth_pass':hi<=allow})
  miss=any(t[1]>t[3] for t in tests);truth_misses+=miss;conservative_rows+=not miss and not v['passed']
  outcomes.append({'row':n,'id':r['id'],'source':list(map(str,S)),'represented':list(map(str,K)),'class':cls,'scale':H(s),'passed':v['passed'],'proofs':proofs})
 # Independently derive G5a first-zero ordinal from rows and uncoupled actual E.
 g=rec['G5A_DATA'];E=list(map(D,g['resolution'][0][1:]));assert g['precision']==128
 loaded=any(b['loads'].values());assert g['coverage']==[{'body':0,'charge':[loaded]*2,'data':loaded,'estimate':[loaded]*2,'stop':[loaded]*4}]
 for key,limit,kinds in [('stop',2.0**-64,range(4)),('estimate',.25,range(2,4)),('charge',1.,range(2,4))]:
  assert sorted((x[0],x[1]) for x in g[key])==([(0,k) for k in kinds] if loaded else [])
  assert all(0<=D(x[2])<=limit for x in g[key])
 assert len(g['theta'])==1 and g['theta'][0][0]==0 and 0<=g['theta'][0][1]<=.5
 assert len(g['B'])==int(loaded)
 if loaded:assert g['B'][0][0]==0 and D(g['B'][0][1])>0
 op=g['operational'][0];ka=float(F(float(F(inp['E'])*F(inp['A']))));kt=float(F(float(F(inp['G'])*F(inp['J']))))
 assert op=={'L':1.0,'ka':ka,'kt':kt,'operations':23,'checks':39,'lost':False}
 first=next((i for i,r in enumerate(rows) if family(r) in [2,3] and E[family(r)-2]==0 and H(si(r))!='0000000000000000'),None)
 if first is not None:assert g['failure']=='Zero { row: '+str(first)+' }' and H(rows[first]['value'])=='8000000000000000'
 else:
  assert loaded
  U=float(F(max(E))*F(D('3ff0000000001000')));assert U>=max(scale[2:])
  for k,stiffness in enumerate([ka,kt]):
   sums=[]
   for node in b['nodes']:
    prefix='global_nodal_displacement_' if k==0 else 'global_nodal_rotation_'
    comps=[abs(si(next(r for r in rows if r['entity_ref']==node['id'] and r['kind']==prefix+a))) for a in 'xyz'];sums.append((comps[0]+comps[1])+comps[2])
   norm=sums[0]+sums[1];threshold=2.0**-59*scale[k];lower=0.0 if norm<=threshold else stiffness*(norm-2.0**-60*scale[k]);assert U>=lower
  assert g['failure'] is None
 assert g['full_case'] is False and rec['BOUNDARY']['full_case'] is False
 # Closed observable shape, midpoint operation sequence, norm guard, headlines, actual text.
 env=rec['ANCILLARY'];assert env['results']==rows;ev=env['contract_evidence'];assert set(ev)=={'preview_cases','combination_gates'} and ev['combination_gates']==[]
 c=ev['preview_cases'][0];assert len(ev['preview_cases'])==1 and set(c)=={'load_case_id','pipe_stress_extrema','stress_maximum_coverage','support_attribution','intensified_measures'}
 assert c['load_case_id']==b['case']['id'] and c['intensified_measures']==[]
 assert c['stress_maximum_coverage']=={'complete':True,'unavailable_pipe_ids':[],'outside_domain_pipe_ids':[]}
 assert c['support_attribution']=={'attributed_support_ids':[b['support']['id']],'withheld':[]}
 for x in c['pipe_stress_extrema']:
  assert set(x)=={'pipe_id','result_id','station_fraction','span_index','local_fraction','value_lower_pa','value_upper_pa','global_upper_bound_pa','certified_gap_pa','subdivisions','approximation','coefficient_basis','enclosure_scope'}
  row=next(r for r in rows if r['id']==x['result_id']);lo,hi=x['value_lower_pa'],x['value_upper_pa'];assert H(lo+.5*(hi-lo))==H(row['value']) and lo<=row['value']<=hi
  assert row['entity_ref']==b['pipe']['id']==x['pipe_id'] and row['kind']=='pipe_elastic_normal_stress_maximum_v2'
 for kind,letter in [('support_reaction_force_magnitude_v2','F'),('support_reaction_moment_magnitude_v2','M')]:
  values=[r['value'] for r in rows if r['kind']=='support_reaction_component_v2' and r['metadata']['component'].startswith(letter)]
  assert len(values)==3;mathematical=sqroot1(sum((F(v)**2 for v in values),F(0)));row=next(r for r in rows if r['kind']==kind)
  assert err(row['value'],mathematical)[1]<=64*F(2)**-52*max(abs(F(row['value'])),F(2)**-1022)
 for key,kind in [('max_displacement','displacement_magnitude'),('max_open_formula_stress','pipe_elastic_normal_stress_maximum_v2')]:
  row=max(sorted((r for r in rows if r['kind']==kind),key=lambda r:r['entity_ref']),key=lambda r:r['value']);h=env['summary'][key]
  assert(h['result_ref'],h['location_ref'],h['unit'],H(h['value']))==(row['id'],row['entity_ref'],row['unit'],H(row['value']))
 capture=rec['BASIS_CAPTURE'];mr=next(r for r in rows if r['kind']=='modulus_basis_record');case=b['case']['id']
 assert capture['calls']==1 and capture['expected'] and capture['case']==case
 assert mr['id']=='result:modulus-basis:'+case.replace(':','-') and mr['entity_ref']==case and H(mr['value'])==H(1.0) and mr['unit']=='record' and mr.get('source_result_refs',[])==[]
 assert mr['metadata']=={'component':'material_modulus_basis','coordinate_system':'not_applicable','location':case,'basis':capture['text'],'sign_convention':'presence record; value 1.0 means the load case solved with the recorded user-entered property basis'}
 assert rec['BOUNDARY']['observable_error']=='None'
 return {'case':rec['label'],'source_E':str(b['source'][0]),'source_G':str(b['source'][1]),'selection':b['details'],'rows':outcomes,'mechanical_rows':len(outcomes),'numeric_pass_rows':sum(o['passed'] for o in outcomes),'truth_miss_rows':truth_misses,'conservative_rows':conservative_rows,'conservative_predicates':conservative_predicates,'certified_candidate_pass_predicates':proved_passes,'g5a_first_zero':first,'g5a_failure':g['failure'],'full_case':False}

if __name__=='__main__':
 log=Path(sys.argv[1]);records=[]
 keys={'REQUEST','INPUT','ROWS','VERDICTS','G5A_DATA','SELECTION','ANCILLARY','NATIVE_ROWS','NATIVE_IDENTITY','BOUNDARY','BASIS_CAPTURE'}
 for line in log.read_text().splitlines():
  if line.startswith('I47_CASE '):records.append({'label':line.split(' ',1)[1]})
  elif line.startswith('I47_'):
   k,_,v=line.partition(' ');k=k[4:]
   if k in keys:records[-1][k]=json.loads(v)
 assert len(records)==4
 result={'oracle':'independently authored RV62; no author-oracle or product import','input_sha256':hashlib.sha256(log.read_bytes()).hexdigest(),'pi_formula':'16 atan(1/5)-4 atan(1/239), alternating 160 terms each','sqrt_fraction_bits':512,'cases':[verify(r) for r in records]}
 Path(sys.argv[2]).write_text(json.dumps(result,indent=2)+'\n')
 print(json.dumps([{k:v for k,v in c.items() if k not in ['rows','selection']} for c in result['cases']],indent=2))
