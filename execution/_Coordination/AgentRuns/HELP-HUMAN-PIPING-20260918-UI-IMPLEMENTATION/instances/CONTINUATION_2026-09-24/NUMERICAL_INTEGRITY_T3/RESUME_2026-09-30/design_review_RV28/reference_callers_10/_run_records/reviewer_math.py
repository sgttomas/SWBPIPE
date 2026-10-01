import re
def minimum(s):return 8 if s==1 else (4 if s<=1024 else 1)
def pushed(s,k):return 0 if k==0 else s*max(minimum(s),1<<(k-1).bit_length())
def old(s,k):return 0 if k<=minimum(s) else pushed(s,k)//2
def arbitrary(s,k):return 0 if not k else s*max(minimum(s),2*k)
def sortmem(s,k):return s*max(48,k) if k>=2 else 0
def fmt(k):return max(8,2*k) if k else 0
node={'json':(632,728),'case':(544,640),'borrowed':(280,376),'floor':(456,552),'published':(848,944),'allow':(368,464),'adj':(104,200),'pair':(104,200)}
def insert(kind,k):
 l,i=node[kind];return 0 if not k else l if k<12 else (1+(k-1)//5)*max(l,i)
def bulk(kind,k):
 if not k:return 0
 l,i=node[kind];z=l*(1+k//12);power=12
 while power<=k:z+=i*(1+k//(12*power));power*=12
 return z
def jtree(x):
 if isinstance(x,str):return len(x.encode()),0
 if isinstance(x,list):
  sub=[jtree(v) for v in x];return pushed(32,len(x))+sum(v[0] for v in sub),max([old(32,len(x))]+[v[1] for v in sub])
 if isinstance(x,dict):
  sub=[jtree(v) for v in x.values()];return insert('json',len(x))+sum(len(k.encode()) for k in x)+sum(v[0] for v in sub),max([0]+[v[1] for v in sub])
 return 0,0
def model(m,clone=False):
 n=len(m['nodes']);children=sum(len(x[0].encode()) for k in ['nodes','members','springs'] for x in m[k])+sum(len(x.encode()) for x in m['omitted_springs'])+sum(len(x[3].encode()) for x in m['loads'])
 mem=(48*n if clone else 2*pushed(24,n))+112*len(m['members'])+88*len(m['springs'])+24*len(m['omitted_springs'])+16*len(m['constraints'])+48*len(m['loads'])+16*len(m['stations'])+children
 return mem,0 if clone else old(24,n)
def caseheap(c):
 r=sum(len(c.get(k,'').encode()) for k in ['id','family','basis','units','k4src_sha256'])+len(c.get('model_sha256') or '')
 r+=96*len(c['rows'])+sum(len(v.encode()) for row in c['rows'] for v in row if isinstance(v,str))
 r+=64*len(c['controls'])
 for ident,_,kind,payload in c['controls']:
  r+=len(ident.encode())
  r+=48*len(payload)+sum(len(k.encode())+len(v.encode()) for k,v in payload.items()) if kind=='value' else len(payload['defect'].encode())
 r+=24*len(c['not_covered'])+sum(len(v.encode()) for v in c['not_covered']);extra=0;move=0
 for name in ['scales','s_full']:
  obj=c.get(name) or {};k=len(obj);tree=bulk('case',k);r+=tree+sum(len(a.encode())+len(b.encode()) for a,b in obj.items());extra=max(extra,48*k+max(sortmem(48,k),tree)-tree)
 if c['model'] is not None:
  h,o=model(c['model']);r+=h;move=o
 return r,r+extra,r+max(extra,move)
def encode(x,e):return arbitrary(1,x)+e*x
def hashing(x,e):
 d=((x+72)//64)*64;msg=max(8,2*d);return msg+144 if not e else msg+max(d,208)
def readfile(x,e):return (max(8,2*(x+32)) if x else 0)+e*x
def obj(name,child=0):
 s=schema[name];return insert('json',s['count'])+s['key_bytes']+child
def output(name,child=0,with_phase=True,extra=0):
 s=schema[name];n=4 if with_phase else 0
 return insert('json',s['count']+n+1)+s['key_bytes']+(45 if with_phase else 0)+17+child+(insert('json',4) if with_phase else 0)+max(extra,97)
def recbound(c,b,e):
 I=len(c['id']);family=len(c['family']);geometry=2+23*b+27*min(sum(v[4] is None for v in c['model']['springs']),2*len(c['model']['nodes']));S=obj('stage');W=obj('verification');Tally=obj('tally');attempt=obj('attempt',25+171+38+2*S+obj('storage'))
 shared=17+I+family+64+128+4*attempt+3*W+Tally
 ret=max(obj('selected_record_root',shared+16+geometry),obj('other_record_root',shared+100+geometry),obj('source_refused',17+I+family+64+89+Tally))
 dout=100+geometry;formatpeak=max(fmt(89)+e*89,fmt(89)+fmt(dout)+e*dout,fmt(dout)+dout,fmt(geometry)+geometry,fmt(171)+171)
 return ret,ret+max(S,W,Tally,formatpeak),dout
def rcm(f,edges,e,levels):
 neighbors=56*f+32*edges;base=neighbors+17*f
 qr=pushed(8,max(1,f-1)) if f else 0;qo=old(8,max(1,f-1)) if f else 0;reachold=0 if f<=1 else 8 if f<=4 else old(8,f)
 return base+max(e*old(8,2*max(0,f-1)),sortmem(8,max(0,f-1)),f+pushed(8,f)+qr+e*max(qo,reachold),f+levels*pushed(8,f)+e*old(8,f),qr+e*qo)
def adjacency(n,f,edges,e):
 tree=(f+edges//5)*200 if edges else 0;ret=80*f+16*edges
 return ret,9*n+24*f+tree+ret+arbitrary(8,12)+e*max(96,8*max(0,f-1))
def sparse(d,e):
 N,m,n,f,r=[d[k] for k in ['N','m','n','f','r']];spring=d['s'];C=144*m+spring;z=min(n*n,C);zf=min(f*f,z);hs=f*(f+1)//2;k=min(r,n)
 if d['d']:
  prefix=pushed(32,N)+pushed(136,m);growing=max(old(32,N),old(136,m));direct=prefix+max(e*growing,41);prior=prefix+66144+e*max(growing,33072)
  return {'branch':'directional_early_return_or_prior_constructor_error','adapter_prefix':prefix,'directional_string_bytes':41,'directional_error':direct,'frame_error':prior,'max':max(direct,prior),'full_sparse_helpers':'not reached because a directional input remains in this exact adapter'}
 B=pushed(136,m)+pushed(16,spring)+8*n+pushed(8,f)+16*r
 HB=B+pushed(32,N)+8*r+max(sortmem(16,r),e*old(32,N),e*old(136,m),e*old(16,spring),e*old(8,f))
 Con=pushed(24,C);I=insert('allow',z);Neighbors=56*N+64*m;Rows=56*n+16*z;Pat=8*(n+1)+arbitrary(8,z)+8*z
 HP=Pat+Neighbors+n+Rows+e*max(32*m,8*n,8*z);Kg=Pat+8*z;HA=1184*m+8*spring+max(HP,Kg)
 W=B+Con+I+Kg+2*pushed(8,z)+8*n
 front=max(HB,B+Con+e*old(24,C),B+Con+2*I,B+Con+I+HA,W+e*old(8,z))
 Sums=64*z+16*C;Diff=80*z+16*C;HSums=Sums+pushed(8,m+spring)+e*old(8,m+spring);HDiff=Diff+pushed(8,m+spring+1)+e*old(8,m+spring+1)
 Round=pushed(96,z)+8*(2*C+z);RoundCopy=96*z+8*(2*C+z)
 Audit=max(HSums,Sums+HDiff,Sums+Diff+Round+e*old(96,z),Sums+Diff+Round+pushed(16,k)+e*old(16,k))
 textlen=len("V-K parity: the structural adapter's formation allowances")
 Core=4*f+8*(f+1)+3*pushed(8,zf)+8*zf+16*f+textlen;Prep=Core+Round
 Row=max(pushed(16,min(f,n))+e*max(old(16,min(f,n)),old(8,zf)),pushed(24,k)+e*old(24,k))
 Prepare=max(n,4*f+8*n,Core+8*n+max(Row,Audit))
 Entries=arbitrary(24,zf);Adj=56*f+16*zf;Profile=max(f,24*f+8*(f+1)+arbitrary(8,hs)+e*8*hs)
 Order=max(Entries+e*24*zf,Entries+Adj+e*old(8,max(0,f-1)),Entries+Adj+rcm(f,zf,e,6),Entries+Adj+8*f+Profile)
 Fct=40*f+8*hs+pushed(48,f);Pivot=48*f;Factor=max(n+16*f,Fct+41*f+e*old(48,f))
 RR=pushed(104,f);Residual=max(n,RR+e*old(104,f));kr=1+2*(12*m+spring)
 Intended=max(HSums,Sums+RR+max(2*pushed(8,kr)+e*old(8,kr),pushed(8,kr)+e*old(104,f)))
 Finish=max(n,40*f,16*f,8*f+8*n+Residual,8*f+8*n+RR+pushed(8,f)+e*old(8,f),24*f+8*n+RR+pushed(8,f),8*f+8*n+RR+Intended,8*f+8*n+2*RR+4*f+RoundCopy+textlen)
 Solver=max(Prepare,Prep+Order,Prep+Factor,Prep+Fct+Pivot+Finish,Prep+max(n,8*f+8*n))
 Solution=8*n+4*f+Pivot+2*RR+RoundCopy+textlen
 total=max(front,W+Solver,W+max(Solution,8*n)+200+e*100,front+66144+e*33072)
 return dict(front=front,W=W,prep=Prep,prepare=Prepare,order=Order,factor=Factor,finish=Finish,solver=Solver,solution=Solution,max=total,hs_upper=hs,factor_values=8*hs,z64_upper=z,contribution_count=C,tau_upper=m+spring,row_terms_upper=12*m+spring,branch='sparse_success_or_returned_error')

# Exact envelope tuples: bits, length, capacity, exponent2 interval, exponent10 interval.
def S(bits,a,A,b,B,cap=None):
 l=(bits+31)//32;return (bits,l,l if cap is None else cap,a,A,b,B)
def mem(x):return 4*x[2]
def copy(x,da=0,db=0):return S(x[0],x[3]+da,x[4]+da,x[5]+db,x[6]+db)
def decimal(text):
 m=re.fullmatch(r'([+-]?)([0-9]*)(?:\.([0-9]*))?(?:[eE]([+-]?[0-9]+))?',text);assert m
 ds=m[2]+(m[3] or '');j=int(m[4] or 0)-len(m[3] or '');bits=0 if set(ds)<=set('0') else 4*len(ds)
 return S(bits,0,0,j,j,pushed(4,(bits+31)//32)//4)
def parseheap(text,e):
 m=re.fullmatch(r'([+-]?)([0-9]*)(?:\.([0-9]*))?(?:[eE]([+-]?[0-9]+))?',text);d=len(m[2]+(m[3] or ''));chunks=(d+8)//9;l=(4*d+31)//32
 return max(fmt(d)+e*d,fmt(d)+arbitrary(16,chunks)+e*16*chunks,fmt(d)+arbitrary(16,chunks)+pushed(4,l)+e*4*l)
def lift(x,k,j,e):
 if not x[0]:return S(0,0,0,0,0),0
 bits=x[0]+k;l=(bits+31)//32;w=k//32;needed=w+x[1]+bool(k);c=max(w,4,2*needed)
 cb=l if j==0 else max(l,4,2*((bits+4*j+31)//32));hb=4*cb+e*(4*((bits+4*j+31)//32) if j else 0)
 return S(bits+4*j,0,0,0,0,cb),max(4*c+e*4*needed,4*c+hb)
def align(x,y,e):
 a=min(x[3],y[3]);b=min(x[5],y[5]);xx,hx=lift(x,max(0,x[4]-a),max(0,x[6]-b),e);yy,hy=lift(y,max(0,y[4]-a),max(0,y[6]-b),e)
 return xx,yy,max(hx,mem(xx)+hy),(a,max(x[4],y[4]),b,max(x[6],y[6]))
def compare(x,y,e):return align(x,y,e)[2]
def plus(x,y,e):
 a,b,h,exps=align(x,y,e);c=max(a[1],b[1])+1;return S(max(a[0],b[0])+1,*exps,c),max(h,mem(a)+mem(b)+4*c)
def minus(x,y,e):
 ny=copy(y);s,h=plus(x,ny,e);return s,mem(ny)+h
def times(x,y,e):
 exps=(x[3]+y[3],x[4]+y[4],x[5]+y[5],x[6]+y[6])
 if x[0]==0 or y[0]==0:return S(0,*exps,0),0
 q=x[1]+y[1]+1;c=max(2*q,4);return S(x[0]+y[0],*exps,c),8*q+4*c+e*4*q
def mx(x,y,e):
 z=S(max(x[0],y[0]),min(x[3],y[3]),max(x[4],y[4]),min(x[5],y[5]),max(x[6],y[6]),max(x[1],y[1]));return z,max(compare(x,y,e),mem(z))
def tolerance(x,y,e):
 a=copy(x);m,h=mx(a,y,e);t=copy(m,db=-9);return t,max(mem(a)+h,mem(a)+mem(m)+mem(t))
F64=S(53,-1074,971,0,0,2)
def predicate(o,x,y,e):
 d,hd=minus(o,x,e);da=copy(d);t,ht=tolerance(x,y,e)
 return max(hd,mem(d)+mem(da),mem(d)+mem(da)+ht,mem(d)+mem(da)+mem(t)+compare(da,t,e))
def magnitude(x,y,e):
 p,hp=times(F64,F64,e);q,hq=times(F64,F64,e);v,hv=plus(p,q,e);t,ht=tolerance(x,y,e);u,hu=plus(x,t,e);u2,hu2=times(u,u,e);l,hl=minus(x,t,e);l2,hl2=times(l,l,e)
 return 16+max(hp,mem(p)+hq,mem(p)+mem(q)+hv,mem(v)+ht,mem(v)+mem(t)+hu,mem(v)+mem(t)+mem(u)+max(hu2,mem(u2)+compare(v,u2,e)),mem(v)+mem(t)+mem(u)+hl,mem(v)+mem(t)+mem(u)+mem(l)+max(hl2,mem(l2)+compare(v,l2,e)))
def floorhelp(x,y,e):
 a=copy(x);c,h=mx(a,y,e);f=copy(F64,da=-34);return max(mem(a)+h,mem(c)+8+mem(f),mem(c)+mem(f)+compare(c,f,e))
def rangehelp(x,e):
 a=copy(x);tiny=S(1,-1075,-1075,0,0,2);big=S(1,1024,1024,0,0,2);v=S(1,970,970,0,0,2);huge,h=minus(big,v,e)
 return max(mem(a),mem(a)+24+h,mem(a)+8+mem(huge)+max(compare(a,tiny,e),compare(a,huge,e)))
def exact_helpers(c,e):
 rowmax=ctrlmax=held=0;rows={r[0]:r for r in c['rows']}
 def operands(r):
  a=r[1];b=r[3] if isinstance(r[3],str) else c['scales'][r[2]];return a,b,decimal(a),decimal(b)
 for r in c['rows']:
  a,b,x,y=operands(r);live=mem(x)+mem(y);held=max(held,live)
  rowmax=max(rowmax,parseheap(a,e),mem(x)+parseheap(b,e),live+floorhelp(x,y,e),live+8+predicate(F64,x,y,e),live+magnitude(x,y,e),live+rangehelp(x,e))
 for ctl in c['controls']:
  if ctl[2]!='value':continue
  for key,o in ctl[3].items():
   a,b,x,y=operands(rows[key]);z=decimal(o);ctrlmax=max(ctrlmax,parseheap(a,e),mem(x)+parseheap(b,e),mem(x)+mem(y)+parseheap(o,e),mem(x)+mem(y)+mem(z)+predicate(z,x,y,e))
 return rowmax,ctrlmax,held
