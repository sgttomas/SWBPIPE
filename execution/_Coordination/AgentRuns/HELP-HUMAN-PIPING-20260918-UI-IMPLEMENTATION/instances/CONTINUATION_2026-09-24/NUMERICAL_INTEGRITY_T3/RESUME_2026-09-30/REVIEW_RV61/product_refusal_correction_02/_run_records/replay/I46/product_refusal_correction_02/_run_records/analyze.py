from fractions import Fraction as F
from pathlib import Path
import json,struct,math,hashlib
HERE=Path(__file__).resolve().parent

def Q(x): return F.from_float(x) if isinstance(x,float) else F(x)
def bits(x): return struct.pack('>d',float(x)).hex()
def unbits(h):return Q(struct.unpack('>d',bytes.fromhex(h))[0])
def rn(x):return Q(float(x))
def add(a,b):return rn(a+b)
def mul(a,b):return rn(a*b)
def div(a,b):return rn(a/b)
def iv(x):return (Q(x),Q(x))
def plus(a,b):return (a[0]+b[0],a[1]+b[1])
def times(a,b):
 v=[x*y for x in a for y in b];return min(v),max(v)
def over(a,b):
 assert b[0]>0
 return times(a,(1/b[1],1/b[0]))
def root(a):
 assert a[0]>=0
 def down(q):return F(math.isqrt((q.numerator<<1280)//q.denominator),1<<640)
 lo,hi=down(a[0]),down(a[1]);hi=hi if hi*hi==a[1] else hi+F(1,1<<640)
 assert lo*lo<=a[0] and hi*hi>=a[1]
 return lo,hi

def atan_recip(n,N=240):
 s=sum((F((-1)**k,(2*k+1)*n**(2*k+1)) for k in range(N)),F())
 t=F((-1)**N,(2*N+1)*n**(2*N+1))
 return min(s,s+t),max(s,s+t)
a,b=atan_recip(5),atan_recip(239)
PI=(16*a[0]-4*b[1],16*a[1]-4*b[0])
assert PI[1]-PI[0] < F(1,2**1100)
# Machin identity pi=16 atan(1/5)-4 atan(1/239); alternating remainder encloses.
U=F(1,2**53);EPS=F(1,2**64)+F(1,2**85);H=F(1,2**1074)
def allowance(n,s):return max(abs(n),s)*EPS+abs(n)*U+H
def af64(n,s):
 a0=mul(F(1,2**64),max(abs(n),s));a1=mul(a0,1+F(1,2**21))
 u0=mul(U,abs(n));u1=add(u0,H);return add(a1,u1)
def norm_raw(y,unit):return div(y,1000) if unit=='mm' else mul(y,1000000) if unit=='MPa' else y
def raw_truth(v,unit):return times(v,iv(1000)) if unit=='mm' else over(v,iv(1000000)) if unit=='MPa' else v
def error_range(n,t,both=False):
 hi=max(abs(n-t[0]),abs(n-t[1]))
 lo=hi if both else min(abs(n-t[0]),abs(n-t[1])) if n<t[0] or n>t[1] else F()
 return lo,hi

def check(y,unit,s,t,both=False,input_row=False):
 n=norm_raw(y,unit);el,eh=error_range(n,t,both)
 if input_row:cl='input';limits=[F()];eranges=[(el,eh)]
 elif s==0 or abs(n)<mul(F(1,2**34),s):
  cl='absolute';b=rn(s/F(2**64));b=Q(math.nextafter(float(b),math.inf)) if b<s/F(2**64) else b
  limits=[b];eranges=[(el,eh)]
 else:
  cl='relative';limits=[allowance(n,s),af64(n,s),abs(n)/10**9,abs(y)/10**9]
  eranges=[(el,eh)]*3+[error_range(y,raw_truth(t,unit),both)]
 verdict=['pass' if hi<=lim else 'fail' if lo>lim else 'unresolved' for (lo,hi),lim in zip(eranges,limits)]
 return {'class':cl,'verdict':verdict,'error_lower':float(el),'error_upper':float(eh),'allowance':float(limits[0]),'ratio':float(el/limits[0]) if limits[0] else None,'normalized_bits':bits(n),'scale_bits':bits(s)}

records=json.loads((HERE/'captured_records.json').read_text());cases={};case=None;pending=None
for o in records:
 if o['tag']=='I45_REQUEST':pending=o['data']
 elif o['tag']=='I45_CASE':case=o['data'];cases[case]={'REQUEST':pending}
 elif case:cases[case][o['tag'].removeprefix('I45_')]=o['data']

def make_truths(p,loaded,source):
 D,t,E,G=[Q(p[k]) for k in ['D','t','E','G']]
 if source:
  A=times(PI,iv(D*t-t*t));I=times(PI,iv((D**4-(D-2*t)**4)/64));J=times(I,iv(2));Z=over(I,iv(D/2));c=iv(D/2)
 else:
  A,I,J,c=[iv(p[k]) for k in ['A','I','J','c']]
  z1,z2=Q(p['Z']),Q(p['I'])/(D/2);Z=(min(z1,z2),max(z1,z2))
 result=[iv(0) for _ in range(74)]
 if not loaded:return result,(A,I,J,Z)
 ux=over(iv(1),times(iv(E),A));uy=over(iv(1),times(iv(3*E),I));rx=over(iv(1),times(iv(G),J));rz=over(iv(1),times(iv(2*E),I))
 result[2]=root(plus(times(ux,ux),times(uy,uy)))
 for i,v in [(9,ux),(10,uy),(12,rx),(14,rz)]:result[i]=v
 for i in [15,16,18,20,23,25,29,33]:result[i]=iv(-1)
 for i in [24,26,30,35,36,38,41,42,44,47,48,50]:result[i]=iv(1)
 result[21]=result[22]=root(iv(2))
 for i,v in [(40,F(3,4)),(46,F(1,2)),(52,F(1,4))]:result[i]=iv(v)
 for start,m in [(53,1),(57,0),(61,F(3,4)),(65,F(1,2)),(69,F(1,4))]:
  result[start]=over(iv(1),A);result[start+2]=over(iv(m),Z);result[start+3]=over(c,J)
 result[73]=plus(over(iv(1),A),over(iv(1),Z))
 return result,(A,I,J,Z)
# Every direct product functional is paired to the actual captured native ordinal.
MAP={**{i+3:i for i in range(12)},1:12,2:13,
 **{15+i:44+i for i in range(6)},21:50,22:51,
 **{23+2*c+e:14+6*e+c for c in range(6) for e in range(2)},
 **{35+i:26+i for i in range(18)}}
assert len(MAP)==52 and set(MAP.values())==set(range(52))
def kind(i):
 if i in [1,2,3,4,5,9,10,11]:return 0
 if i in [6,7,8,12,13,14]:return 1
 if i in [15,16,17,21,23,24,25,26,27,28] or 35<=i<=52 and (i-35)%6<3:return 2
 if i<=52:return 3
 return 4

def scales(rows,p):
 maxima=[F()]*4
 for i,y in rows.items():
  if i in range(3,9) or i==0 or i>52:continue
  k=kind(i);maxima[k]=max(maxima[k],abs(norm_raw(y,cases['loaded']['ROWS'][i]['unit'])))
 # L_b=1 exactly from actual captured coordinates. Actual p=128: no p512 floor.
 tr,ro,fo,mo=maxima
 coupled=[max(tr,mul(1,ro)),max(ro,div(tr,1)),max(fo,div(mo,1)),max(mo,mul(1,fo))]
 def stress(k):return add(div(coupled[2],Q(p['A'])),mul(k,div(coupled[3],Q(p['Z']))))
 return coupled,stress(1),stress(unbits('4006a09e667f3bcd'))
def scale_for(i,s):return s[2] if i==73 else s[1] if i>52 else s[0][kind(i)]
def sqrt_rn64(q):
 # Exact nearest-even binary64 square-root control on an already rounded operand.
 assert q >= 0
 if q == 0: return F()
 lo,hi=0,0x7fefffffffffffff
 assert unbits(f'{hi:016x}')**2 >= q
 while hi-lo>1:
  mid=(lo+hi)//2
  if unbits(f'{mid:016x}')**2 <= q:lo=mid
  else:hi=mid
 a,b=unbits(f'{lo:016x}'),unbits(f'{hi:016x}')
 if a*a==q:return a
 m=(a+b)/2
 return a if q<m*m or q==m*m and lo%2==0 else b

def body_extent(nodes):
 # DESIGN §4.1.6.1 item5: rounded axis spans, rounded squares,
 # separately rounded left-associated sum, then RN64 sqrt.
 delta=[rn(max(Q(n[k]) for n in nodes)-min(Q(n[k]) for n in nodes)) for k in range(3)]
 squares=[mul(x,x) for x in delta]
 return sqrt_rn64(add(add(squares[0],squares[1]),squares[2]))

def couple_resolution(resolution,L):
 # R7 lines231/350: both results use the original uncoupled operands.
 E_force,E_moment=resolution
 return [E_force,E_moment] if L==0 else [max(E_force,div(E_moment,L)),max(E_moment,mul(L,E_force))]

coupling_inputs={}
all_results={}
for name,cap in cases.items():
 loaded=name=='loaded';p=cap['INPUT'];ts,props=make_truths(p,loaded,True);tk,_=make_truths(p,loaded,False)
 # All captured values normalized from their actual serialized binary64 values.
 ys={i:Q(row['value']) for i,row in enumerate(cap['ROWS'])}
 ss=scales(ys,p);ordinary=[]
 for i in range(1,74):
  row=cap['ROWS'][i];s=scale_for(i,ss)
  assert bits(s)==cap['VERDICTS'][i]['scale_bits']
  src=check(ys[i],row['unit'],s,ts[i],input_row=i in range(3,9))
  rep=check(ys[i],row['unit'],s,tk[i],both=i not in [2,21,22],input_row=i in range(3,9))
  assert 'unresolved' not in rep['verdict']
  assert cap['VERDICTS'][i]['passed'] == ('fail' not in src['verdict'] and 'fail' not in rep['verdict'])
  assert 'unresolved' not in src['verdict']
  ordinary.append({'row':i,'id':row['id'],'source':src,'represented':rep,'captured_rust_pass':cap['VERDICTS'][i]['passed']})
 # Algebraic unit conversion only, actual native publication, fixed existing row units.
 # Stress/maximum values are not invented; they do not enter these four body scales.
 ny={i:mul(unbits(cap['NATIVE_ROWS'][j]['value_bits']),1000) if cap['ROWS'][i]['unit']=='mm' else unbits(cap['NATIVE_ROWS'][j]['value_bits']) for i,j in MAP.items()}
 ns=scales(ny,p);native=[];projected=[]
 for i,j in sorted(MAP.items()):
  x=unbits(cap['NATIVE_ROWS'][j]['value_bits']);s=scale_for(i,ns)
  src=check(x,'m' if cap['ROWS'][i]['unit']=='mm' else cap['ROWS'][i]['unit'],s,ts[i],input_row=i in range(3,9))
  rep=check(x,'m' if cap['ROWS'][i]['unit']=='mm' else cap['ROWS'][i]['unit'],s,tk[i],both=i not in [2,21,22],input_row=i in range(3,9))
  native.append({'product_row':i,'native_ordinal':j,'actual_bits':bits(x),'source':src,'represented':rep})
  projected.append({'product_row':i,'native_ordinal':j,'projected_raw_bits':bits(ny[i]),'source':check(ny[i],cap['ROWS'][i]['unit'],s,ts[i],input_row=i in range(3,9)),'represented':check(ny[i],cap['ROWS'][i]['unit'],s,tk[i],both=i not in [2,21,22],input_row=i in range(3,9))})
 def fails(r,branch):return [q.get('row',q.get('product_row')) for q in r if 'fail' in q[branch]['verdict']]
 # Independently recompute G5a scalar checks from captured summary fields.
 g=cap['G5A_DATA'];resolution=[unbits(v) for v in g['resolution'][0][1:]]
 kcheck=[mul(Q(p['E']),Q(p['A'])),mul(Q(p['G']),Q(p['J']))] # L=1 exact; separate /L leaves bits unchanged.
 assert [bits(v) for v in kcheck]==[bits(g['operational'][0][key]) for key in ['ka','kt']]
 totals=[]
 for kinds in [(3,4,5,9,10,11),(6,7,8,12,13,14)]:
  a,b,c,d,e,f=[abs(norm_raw(ys[i],cap['ROWS'][i]['unit'])) for i in kinds]
  totals.append(add(add(add(a,b),c),add(add(d,e),f)))
 lower=[]
 for k,N,S in zip(kcheck,totals,ss[0][:2]):
  lower.append(F() if N<=mul(F(1,2**59),S) else mul(k,rn(N-mul(F(1,2**60),S))))
 L_body=body_extent(p['nodes']);coupled_resolution=couple_resolution(resolution,L_body)
 guard=unbits('3ff0000000001000');ceilings=[mul(e,guard) for e in coupled_resolution]
 coupling_inputs[name]={'body_extent_bits':bits(L_body),'uncoupled_resolution_bits':[bits(e) for e in resolution],'coupled_resolution_bits':[bits(e) for e in coupled_resolution],'guarded_resolution_bits':[bits(e) for e in ceilings],'zero_rule_uses':'original uncoupled resolution'}
 zero_bad=[i for i in range(1,53) if kind(i) in [2,3] and resolution[kind(i)-2]==0 and bits(cap['ROWS'][i]['value'])!='0000000000000000']
 summary_ok=all(unbits(v[-1])<=F(1,4) for v in g['estimate']) and all(unbits(v[-1])<=1 for v in g['charge']) and all(Q(v[-1])<=F(1,2) for v in g['theta']) and all(unbits(v[-1])>0 for v in g['B'])
 g5a={'first_zero_rule_failure':zero_bad[0] if zero_bad else None,'all_zero_rule_failures':zero_bad,'lower_bound_bits':[bits(v) for v in lower],'resolution_guard_bits':[bits(v) for v in ceilings],'sanity':all(a>=b for a,b in zip(ceilings,ss[0][2:])),'lower_bound':all(a>=b for a,b in zip(ceilings,lower)),'summary_scalar_bounds':summary_ok,'scope':'Arithmetic checks of captured G5a fields only; source-bound unpublished factor/estimates are supplied facts, not replayed.'}
 all_results[name]={'g5a_arithmetic':g5a,'ordinary':ordinary,'native_actual_point':native,'algebraic_direct_unit_projection':projected,'ordinary_scales':[[bits(a) for a in ss[0]],bits(ss[1]),bits(ss[2])],'projected_recomputed_scales':[[bits(a) for a in ns[0]],bits(ns[1]),bits(ns[2])],'failures':{mode:{b:fails(arr,b) for b in ['source','represented']} for mode,arr in [('ordinary',ordinary),('native_actual_point',native),('algebraic_direct_unit_projection',projected)]},'negative_zero_rows':[i for i,y in enumerate(cap['ROWS']) if bits(y['value'])=='8000000000000000']}
 if loaded:
  n=unbits(cap['NATIVE_ROWS'][9]['value_bits']);s=ns[0][1];source=ts[12];represented=tk[12][0]
  srcerr=error_range(n,source);gap=(source[0]-represented,source[1]-represented);A=allowance(n,s)
  assert srcerr[0]>A and srcerr[0]>af64(n,s)
  assert abs(n-represented)<A
  native_bits=int(bits(n),16)
  rn_lower=(unbits(f'{native_bits-1:016x}')+n)/2
  rn_upper=(unbits(f'{native_bits+1:016x}')+n)/2
  assert rn_lower < represented < rn_upper
  other_max=max(abs(norm_raw(y,cap['ROWS'][i]['unit'])) for i,y in ny.items() if i!=12 and kind(i) in [0,1])
  ymin=represented/F(1000000001,1000000000)
  assert ymin>other_max
  # Stronger conditional incompatibility: public relative bound to K implies y <= K/(1-1e-9).
  # With this fixed projected body scale S, any y satisfying both sharper bounds would need
  # gap <= 2 allowance(ymax,S). It does not. No output selection is performed.
  ymax=represented/F(999999999,1000000000);twoA=2*allowance(ymax,s)
  assert gap[0]>twoA
  exact={'native_rx_hex':bits(n),'ordinary_rx_hex':bits(cap['ROWS'][12]['value']),'D_hex':bits(p['D']),'t_hex':bits(p['t']),'G_hex':bits(p['G']),'J_K_hex':bits(p['J']),'source_J_lower':str(props[2][0]),'source_J_upper':str(props[2][1]),'qK_RN64_is_actual_native':True,'qK_RN_midpoint_lower':str(rn_lower),'qK_RN_midpoint_upper':str(rn_upper),'other_translation_rotation_max_exact':str(other_max),'nearby_y_lower_from_public_K':str(ymin),'qK_exact':str(represented),'qS_lower':str(source[0]),'qS_upper':str(source[1]),'native_n_exact':str(n),'fresh_S_exact':str(s),'A_exact':str(A),'A_f64_exact':str(af64(n,s)),'source_error_lower_exact':str(srcerr[0]),'source_error_upper_exact':str(srcerr[1]),'represented_error_exact':str(abs(n-represented)),'source_error_over_A':float(srcerr[0]/A),'J_K_exact':str(Q(p['J'])),'J_K_minus_Js_lower':str(Q(p['J'])-props[2][1]),'J_K_minus_Js_upper':str(Q(p['J'])-props[2][0]),'J_K_relative_excess_lower':str((Q(p['J'])-props[2][1])/props[2][1]),'nearby_y_upper_from_public_K':str(ymax),'gap_lower_exact':str(gap[0]),'dual_tolerance_upper_exact':str(twoA),'gap_over_dual_tolerance':float(gap[0]/twoA),'scope':'Algebraic exact control on captured operands and retained point; no producer execution. Actual recomputed projection S; incompatibility also holds when only Rx candidate changes and its body scale is recomputed from unchanged other direct rows (S=y).'}
  (HERE/'rx_counterexample.json').write_text(json.dumps(exact,indent=2)+'\n')
(HERE/'analysis_results.json').write_text(json.dumps({'method':{'pi':'Machin identity, 240 alternating terms each, exact rational remainder','root':'exact integer isqrt directed to quantum 2^-640','operations':'Fraction exact arithmetic with explicit separate binary64 RN conversions; no solver/model/product imports','projection':'direct units only; no future stress/maximum/observable evidence manufactured'},'cases':all_results},indent=2)+'\n')
print(json.dumps({name:{'failures':r['failures'],'ordinary_scales':r['ordinary_scales'],'projected_recomputed_scales':r['projected_recomputed_scales'],'negative_zero_rows':r['negative_zero_rows']} for name,r in all_results.items()},indent=2))
print('RX ratio',exact['source_error_over_A'],'dual gap ratio',exact['gap_over_dual_tolerance'])

(HERE/'g5a_coupling_inputs.json').write_text(json.dumps(coupling_inputs,indent=2)+'\n')
