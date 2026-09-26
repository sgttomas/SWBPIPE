"""Read-only private-source applicability checks. Never execute acquired decks."""
from pathlib import Path
from html.parser import HTMLParser
from fractions import Fraction as F
from collections import Counter
import ast, json, hashlib, subprocess, datetime, re
ROOT=Path.cwd()
D=ROOT/'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/VALIDATION_FOUNDATION/STATIC_REFERENCE_BASIS'
OUT=D/'INDEPENDENT_REVIEW'
PRIVATE=Path('/private/tmp/piping-static-reference-acquisition-20260925')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.loads(p.read_text())
acq=load(D/'ACQUISITION.json');pin=acq['source_commit'];assert pin=='7558f1d06088bd93d8324e314912bf0e6df7801f'
assert load(PRIVATE/'src_main.json')['id']==pin
retrieved=[]
for row in acq['retrievals']:
 if row.get('status')!=200:continue
 p=PRIVATE/row['private_inspection_filename'];assert sha(p)==row['sha256'];assert p.stat().st_size==row['bytes']
 if '/-/raw/' in row['url']:assert '/-/raw/'+pin+'/' in row['url']
 retrieved.append({'origin':str(p),'url':row['url'],'sha256':sha(p),'bytes':p.stat().st_size})
deps=[]
for row in acq['declared_test_dependencies']:
 p=PRIVATE/(row['test_id']+'.export');assert sha(p)==row['export_sha256']
 found=[line.split()[1:] for line in p.read_text().splitlines() if line.split()[:1]==['F']]
 expected=[ [d['kind'],d['name'],d['mode'],str(d['logical_unit'])] for d in row['dependencies']]
 assert found==expected
 for d in row['dependencies']:
  path=PRIVATE/d['name'];assert sha(path)==d['sha256'] and path.stat().st_size==d['bytes']
  if d['kind']=='mmed':assert path.read_bytes().startswith(b'\x89HDF\r\n\x1a\n')
 deps.append({'test':row['test_id'],'F_input_count':len(found),'all_declared_bytes_match':True})
assert len(deps)==10
lineage=[];external_calls=[]
for row in load(D/'ASSERTION_LINEAGE.json'):
 p=PRIVATE/row['file'];assert sha(p)==row['sha256'];tree=ast.parse(p.read_text());counts=Counter()
 for call in (n for n in ast.walk(tree) if isinstance(n,ast.Call)):
  kw={x.arg:x.value for x in call.keywords}
  if 'VALE_CALC' in kw or 'VALE_REFE' in kw:
   ref=ast.literal_eval(kw['REFERENCE']) if 'REFERENCE' in kw and isinstance(kw['REFERENCE'],ast.Constant) else 'not_explicitly_external_or_analytical';counts[ref]+=1
  if isinstance(call.func,ast.Name) and (call.func.id.startswith('LIRE_') or call.func.id in ['INCLUDE','DEFI_FICHIER','PRE_GIBI','PRE_IDEAS','open','exec','eval']):
   external_calls.append({'file':p.name,'line':call.lineno,'function':call.func.id,'units':{k:ast.literal_eval(v) for k,v in kw.items() if k in ['UNITE','FICHIER','FORMAT'] and isinstance(v,ast.Constant)}})
 assert dict(counts)==row['assertion_blocks_by_explicit_reference']
 for loc in row['axial_assertion_locators']:
  call=next(n for n in ast.walk(tree) if isinstance(n,ast.Call) and n.lineno==loc['line']);kw={x.arg:x.value for x in call.keywords};assert call.end_lineno==loc['end_line'];assert ast.get_source_segment(p.read_text(),kw['VALE_REFE'])==loc['reference_expression']
 lineage.append({'file':p.name,'counts':dict(counts),'matches_packet':True})
class Article(HTMLParser):
 def __init__(self):super().__init__();self.inside=False;self.parts=[]
 def handle_starttag(self,tag,attrs):
  if tag=='article':self.inside=True
 def handle_endtag(self,tag):
  if tag=='article':self.inside=False
 def handle_data(self,data):
  if self.inside:self.parts.append(data)
def article(name):
 a=Article();a.feed((PRIVATE/name).read_text());return ' '.join(''.join(a.parts).split())
versions=[]
for r in load(D/'_run_records/MANUAL_VERSION_CHECK.json'):
 a=article(r['v17_file']);b=article(r['v18_file']);assert a==b
 versions.append({'v17':r['v17_file'],'v18':r['v18_file'],'article_text_equal':True})
# Scalar/source-formula checks only: exact rational geometry/datum/coefficients.
a=F(4,100);b=F(36,1000);radius=(a+b)/2;E=F(200_000_000_000);nu=F(3,10);L=F(5);alpha=F(1,100000);pressure=F(10_000_000)
thermals={}
for delta in [100,200]:
 u=L*alpha*delta;thermals[str(delta)]={'local_axial_m':str(u),'global_x_m':str(u*F(4,5)),'global_y_m':str(u*F(3,5)),'thermal_strain':str(alpha*delta)}
assert thermals['200']['global_x_m']=='1/125' and thermals['200']['global_y_m']=='3/500'
assert thermals['100']['global_x_m']=='1/250' and thermals['100']['global_y_m']=='3/1000'
manual_radial=pressure*b*b*radius/(E*(b*b-a*a))*((1-nu)+(1+nu)*a*a/(radius*radius))
command_radial=pressure*b*b*radius/(E*(a*a-b*b))*((1-nu)+(1+nu)*a*a/(radius*radius))
assert manual_radial==-command_radial and command_radial>0
# I/A=(a²+b²)/4. Timoshenko extra tip force displacement / EB displacement.
shear_ratio=3*(2*(1+nu))*(a*a+b*b)/(4*L*L);assert shear_ratio>F(1,10**9)
# ex,ey,ez inferred from declared global elementary load vectors.
ex=[F(4,5),F(3,5),F(0)];ey=[-F(3,5),F(4,5),F(0)];ez=[F(0),F(0),F(1)]
assert sum(x*x for x in ex)==1 and sum(x*y for x,y in zip(ex,ey))==0
subset_checks={'axial_projected_load_N':[str(500*x) for x in ex],'torsion_projected_moment_Nm':[str(500*x) for x in ex],'my_projected_moment_Nm':[str(500*x) for x in ey],'mz_projected_moment_Nm':[str(500*x) for x in ez],'axial_tip_expression':'u=(F L/(E A))*ex','torsion_tip_expression':'theta=(T L/(G J))*ex','my_tip_expressions':'u=-(My L²/(2 E I))*ez; theta=(My L/(E I))*ey','mz_tip_expressions':'u=(Mz L²/(2 E I))*ey; theta=(Mz L/(E I))*ez','reason_no_timoshenko_delta_in_pure_moments':'zero transverse shear force; no shear contribution to these displacement fields','no_target_values_adopted':True}
# Check exact engine commit blobs without writing product source copies.
engine=[]
for record in load(D/'BASIS.json')['source_basis']:
 if not record['origin'].startswith('git:22452'):continue
 _,revision,path=record['origin'].split(':',2);blob=subprocess.check_output(['git','show',revision+':'+path],cwd=ROOT);h=hashlib.sha256(blob).hexdigest();assert h==record['sha256'];engine.append({'origin':record['origin'],'sha256':h})
results={'status':'PASS static source checks','source_pin':pin,'retrieved_hashes_checked':len(retrieved),'declared_dependencies':deps,'assertion_lineage':lineage,'input_calls_static_only':external_calls,'manual_versions':versions,'thermal_datum_check':{'alpha_per_C':str(alpha),'pinned_temperature':200,'pinned_VALE_REF':0,'manual_problem_temperature':100,'results':thermals,'conclusion':'Manual problem100 cannot yield its0.008/0.006 table under shown alpha*T formula; pinned200/ref0 can. No differing reference datum is supplied to reconcile manual100. Source mismatch retained; no target selected.'},'pressure_sign_check':{'manual_denominator':'b²-a² with a outer,b inner','command_denominator':'a²-b²','manual_ur_m':str(manual_radial),'command_ur_m':str(command_radial),'magnitudes_equal_opposite_sign':True,'conclusion':'Printed equations differ algebraically under the supplied definitions; radial sign/operator convention still requires explicit target mapping. No repaired pressure target adopted.'},'eb_shear_check':{'force_tip_shear_correction_relative_to_EB':str(shear_ratio),'decimal':float(shear_ratio),'protected_criterion':1e-9,'comparison_claim':'Confirms distinct model; not a product run or target tolerance adoption.'},'four_subsets':subset_checks,'fixed_engine_hashes':engine,'limits':['No acquired command/deck imported or executed','No mesh topology/group decode','No installed external runtime qualification','No campaign target/tolerance adoption','No product/solver/native/build execution']}
(OUT/'CHECKS.json').write_text(json.dumps(results,indent=2)+'\n')
(OUT/'PRIVATE_SOURCE_HASHES.json').write_text(json.dumps(retrieved,indent=2)+'\n')
print(json.dumps({'status':results['status'],'retrieved_hashes_checked':len(retrieved),'export_dependencies':len(deps),'lineage_files':len(lineage),'manual_pairs':len(versions),'engine_blobs':len(engine),'shear_fraction':str(shear_ratio),'private_source_bytes_copied':False},indent=2))
