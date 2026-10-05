"""RV67 actual C2 points, independent prior reviewer truth; no product execution."""
import pathlib,json,hashlib,sys,math,struct,re
from fractions import Fraction as Q
B=pathlib.Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics')
R=B/'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'
prior=R/'REVIEW_RV67/prepared_producer_01/check.py'
assert hashlib.sha256(prior.read_bytes()).hexdigest()=='b926b26b02c8d4f55ad706828135e95e2073fa5c7f5aa0afc1a681533ba026cc'
text=prior.read_text(); prefix=text.split('summary=[]; allrows=[]',1)[0];assert len(prefix)<len(text)
ns={'__file__':str(prior)};exec(compile(prefix,str(prior),'exec'),ns)
# Prefix re-derives independently reviewed geometry/mechanics, integer RN64 and
# truth intervals. It executes no author checker and takes no PASS as a premise.
target,rn,bits,sqrti,allow=[ns[k] for k in ['target','rn','bits','sqrti','allow']]
add,mul,div=[ns[k] for k in ['b64add','b64mul','b64div']];new=ns['new'];unit=ns['unit']
root=B.parent/'scratch/i51_prepared_producer_implementation';log=root/'pp_c2_diagnostics_02.log';data=log.read_bytes()
assert hashlib.sha256(data).hexdigest()=='b2c09a63bb86da5c38d96fa4d6519f01ef3f95a7bcfd1408143fe3a374cafc01'
actual=[json.loads(x.split(' ',1)[1]) for x in data.decode().splitlines() if x.startswith('I51_FROZEN_REFUSAL ')];caps=[json.loads(x.split(' ',1)[1]) for x in data.decode().splitlines() if x.startswith('I50_RECORD ')];assert len(actual)==len(caps)==2
first=json.loads((root/'FIRST_C2.json').read_text());out=[];records=[]
for act,cap,firstcase in zip(actual,caps,first['cases']):
 assert act['mode']==cap['mode']==firstcase['mode'];assert cap['request']==ns['caps'][0]['request']
 assert cap['native']['precision']==128 and cap['native']['calls']==1
 assert hashlib.sha256(bytes(cap['native']['source_encoding'])).hexdigest()=='e61b9eeb14c50ca144659366ee74941310099a03df49ebd6b8f2cbd9f9c4ca43'
 for k in ['A','I','J','Z']:assert bits(cap['facts'][0][k])==bits(new[k])
 assert len(act['rows'])==len(cap['envelope']['results'])==len(act['verdicts'])==firstcase['verdict_count']
 rows=[]
 for i,(r,old,v,fv) in enumerate(zip(act['rows'],cap['envelope']['results'],act['verdicts'],firstcase['all_verdicts'])):
  assert (r['id'],r['kind'],r['unit'])==(old['id'],old['kind'],old['unit']);assert bits(r['value'])==r['bits']
  assert int(v['n'],16)==int(fv[1]) and int(v['scale'],16)==int(fv[2]) and str(v['passed']).lower()==fv[4]
  iv,kind,inp=target(old)
  if iv is None:
   assert r['bits']==bits(old['value']) and v['class']=='None';continue
  y=r['value']; n=rn(Q(y)*unit[r['unit']]);h=max(abs(Q(n)-z) for z in iv);hu=max(abs(Q(y)-z/unit[r['unit']]) for z in iv)
  rows.append(dict(i=i,id=r['id'],kind=kind,inp=inp,y=y,n=n,h=h,hu=hu,iv=iv,raw=r['bits'],v=v,entity=old['entity_ref'],component=(old.get('metadata') or {}).get('component'),unit=r['unit']))
 scales={k:max(abs(r['n']) for r in rows if r['kind']==k and not r['inp']) for k in ('translation','rotation','force','moment')}
 tr,ro,fo,mo=[scales[k] for k in ('translation','rotation','force','moment')]
 scales=dict(translation=max(tr,mul(3,ro)),rotation=max(ro,div(tr,3)),force=max(fo,div(mo,3)),moment=max(mo,mul(3,fo)))
 for kind,k in [('stress',1.),('maximum',2*struct.unpack('>d',bytes.fromhex('3ff6a09e667f3bcd'))[0])]:scales[kind]=add(div(scales['force'],new['A']),mul(k,div(scales['moment'],new['Z'])))
 counts={};failed=[]
 for r in rows:
  v=r['v'];n=r['n'];s=scales[r['kind']];assert bits(n)==v['n'] and bits(s)==v['scale']
  if r['inp']:cl='InputDerived';ok=r['h']==0
  elif s<2.**-988 or abs(n)<mul(2.**-34,s):
   cl='AbsoluteVerified';exact=Q(s)/2**64;bound=rn(exact)
   if Q(bound)<exact:bound=math.nextafter(bound,math.inf)
   ok=r['h']<=Q(bound);assert int(re.search(r'bound_bits: (\d+)',v['class']).group(1))==int(bits(bound),16)
  else:cl='RelativeVerified';ok=r['h']<=allow(n,s) and 10**9*r['h']<=abs(Q(n)) and 10**9*r['hu']<=abs(Q(r['y']))
  assert cl in v['class'] and ok,(r['id'],cl,float(r['h']))
  counts[cl]=counts.get(cl,0)+1
  if not v['passed']:failed.append(r['id']);assert cl=='AbsoluteVerified'
  records.append(dict(mode=act['mode'],id=r['id'],class_=cl,raw_bits=r['raw'],normalized_bits=bits(n),scale_bits=bits(s),point_pass=True,SI_error=str(r['h']),raw_error=str(r['hu']),certificate_pass=v['passed']))
 assert len(rows)==97 and len(failed)==7
 assert [r['i'] for r in rows if not r['v']['passed']]==firstcase['failed_rows']
 sid='rigid:N0'; components=[next(r for r in rows if r['entity']==sid and r['component']=='F'+a)['y'] for a in 'xyz']; mag=next(r for r in rows if r['entity']==sid and r['component']=='force_magnitude')['y']
 norm=sqrti(sum(Q(x)**2 for x in components)); guard=Q(64)*Q(2.**-52)*max(Q(abs(mag)),Q(2.**-1022));error_min=Q(mag)-norm[1];assert error_min>guard
 # This is an exact norm bracket of published components, not a claim to replay
 # the platform's two std::hypot implementations or their precise future bits.
 out.append(dict(mode=act['mode'],mechanical=97,counts=counts,point_passes=97,certificate_passes=90,failed_ids=failed,scales={k:bits(v) for k,v in scales.items()},rigid_force_magnitude=mag,component_norm_bounds=[float(z) for z in norm],observable_error_lower=float(error_min),observable_tolerance=float(guard),g5a_reported=act['g5a'],observable_reported=act['observables']))
 assert act['g5a']=='None' and 'support guard' in act['observables']
 def value(node,axis):return next(r['n'] for r in rows if r['id']=='result:disp:'+node+':'+axis)
 norms=[]
 for axes in [('ux','uy','uz'),('rx','ry','rz')]:
  ends=[add(add(abs(value(node,axes[0])),abs(value(node,axes[1]))),abs(value(node,axes[2]))) for node in ('N0','N1')]
  norms.append(add(*ends))
 stiff=[div(mul(200e9,new['A']),3),div(mul(80e9,new['J']),3)]
 lower=[]
 for n,k,kind in zip(norms,stiff,('translation','rotation')):
  lower.append(0. if n<=mul(2.**-59,scales[kind]) else mul(k,rn(Q(n)-Q(mul(2.**-60,scales[kind])))))
 ef,em=[struct.unpack('>d',bytes.fromhex(x))[0] for x in cap['native']['resolution'][0][1:]]
 hats=[max(ef,div(em,3)),max(em,mul(3,ef))];upper=[mul(x,1+2.**-40) for x in hats]
 assert all(upper[i]>=max(lower[i],scales[k]) for i,k in enumerate(('force','moment')))
 out[-1]['g5a_recomputed_lower']=lower;out[-1]['g5a_actual_resolution']=[ef,em];out[-1]['g5a_recomputed_upper']=upper

def endpoint(txt):
 if txt in ('Z+','Z-'):return Q(0)
 m=re.fullmatch(r'([+-])([0-9a-f]+)p(-?\d+)',txt);assert m,txt
 return (-1 if m[1]=='-' else 1)*Q(int(m[2],16))*ns['power'](int(m[3])-1023)
lines=[x for x in data.decode().splitlines() if x.startswith('I51_DUAL_LANES ')]
firstlines=[x for x in (root/'pp_c2_candidate_01.log').read_text().splitlines() if x.startswith('I51_DUAL_LANES ')]
assert lines==firstlines and len(lines)==2
width_results=[]
for case_index,line in enumerate(lines):
 groups=re.findall(r'LaneReadouts \{ rows: \[(.*?)\], data:',line);assert len(groups)==2
 lanes=[[tuple(endpoint(x) for x in pair) for pair in re.findall(r'Enclosure \{ lo: ([^,]+), hi: ([^}]+) \}',g)] for g in groups]
 assert len(lanes[0])==len(lanes[1])==58
 for ordinal in [47,48,49,14,20,26,32,38,50]:
  ki,si=lanes[0][ordinal],lanes[1][ordinal];assert ki[0]<=0<=ki[1] and si[0]<=0<=si[1]
  kid=caps[case_index]['native']['rows'][ordinal]['id']
  if ordinal in (47,48,49):
   rid='result:support-action:4:case:8:rigid:N0:F'+'xyz'[ordinal-47]
  elif ordinal==50:rid='result:support-action:4:case:8:rigid:N0:force_magnitude'
  else:rid={14:'result:force:M1:axial',20:'result:force:M1:axial:end-j',26:'result:force:M1:quarter-1:axial',32:'result:force:M1:midspan:axial',38:'result:force:M1:quarter-3:axial'}[ordinal]
  row=next(z for z in actual[case_index]['rows'] if z['id']==rid);n=Q(row['value']);h=max(abs(n-q) for q in (*ki,*si));scale=struct.unpack('>d',bytes.fromhex(out[case_index]['scales']['force']))[0];bound=Q(scale)/2**64
  expected=ordinal not in [48,49,14,20,26,32,38];assert (h<=bound)==expected
  w={'mode':actual[case_index]['mode'],'native_ordinal':ordinal,'native_id':kid,'row':rid,'K_width':str(ki[1]-ki[0]),'K_halfwidth':str((ki[1]-ki[0])/2),'source_width':str(si[1]-si[0]),'source_halfwidth':str((si[1]-si[0])/2),'actual_center_error_to_enclosure':str(h),'absolute_bound':str(bound),'certificate_pass_rederived':expected}
  width_results.append(w)
 # The proposed hypot center alone cannot pass this unchanged broad norm interval.
 normrow=lanes[1][50];new_center_upper=Q(out[case_index]['component_norm_bounds'][1])*Q(1+2.**-50)
 assert normrow[1]-new_center_upper>Q(scale)/2**64
result={'width_records':width_results,'unchanged_lane_debug_first_vs_diagnostic':True,'support_hypot_alone_fails_current_norm_enclosure':True,'scope':'independent exact actual-point arithmetic on same checked dual-truth basis; no author PASS used as an oracle','prior_reviewer_script':str(prior),'prior_sha256':hashlib.sha256(prior.read_bytes()).hexdigest(),'diagnostic_log_sha256':hashlib.sha256(data).hexdigest(),'cases':out,'rows':records}
pathlib.Path(sys.argv[1]).write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(out,indent=2))
