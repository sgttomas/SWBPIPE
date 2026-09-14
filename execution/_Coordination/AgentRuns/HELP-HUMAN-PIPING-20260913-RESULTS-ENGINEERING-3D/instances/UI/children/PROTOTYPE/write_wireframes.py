"""Pure editable SVG asset writer. No browser, app, renderer, test or package runtime."""
from pathlib import Path
from math import cos,sin
from html import escape
out=Path(__file__).resolve().parent/'wireframes'
out.mkdir(exist_ok=True)
for W,H in [(1024,768),(1280,800),(1440,920)]:
 for density in ['comfortable','compact']:
  left,right,control,dock=(232,292,32,224) if density=='comfortable' else (194,252,26,202)
  if W<=1100 and density=='comfortable':left,right,dock=194,272,214
  top=135; bottom=H-28-dock; cx=left; cw=W-left-right; ch=bottom-top
  s=[]
  def rect(x,y,w,h,fill='#fffefa',stroke='#dce2e3',r=0):s.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{r}" fill="{fill}" stroke="{stroke}"/>')
  def text(x,y,t,size=11,color='#344b5b',weight='400'):s.append(f'<text x="{x}" y="{y}" fill="{color}" font-size="{size}" font-weight="{weight}">{escape(str(t))}</text>')
  def line(x1,y1,x2,y2,stroke='#dce2e3',sw=1,dash=''):s.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{stroke}" stroke-width="{sw}" '+(f'stroke-dasharray="{dash}"' if dash else '')+'/>')
  def button(x,y,w,label,active=False,height=None):
   height=height or control;rect(x,y,w,height,'#eaf2fa' if active else '#fffefa','#bdd4e7' if active else '#cdd6da',4);text(x+8,y+height/2+4,label,10,'#2466a2' if active else '#506775')
  rect(0,0,W,H,'#eff1f0');rect(0,0,W,48,'#293542','#293542');text(18,31,'⊥',26,'#87b9df');text(51,29,'OpenPipeStress',15,'#eaf0f5','600');text(194,29,'/  Cooling water · CW-101',11,'#c7d1d9');text(W-450,29,'SIMULATED DESIGN PROTOTYPE',9,'#d4e1ea','700');text(W-161,29,'Density',10,'#c7d1d9');rect(W-112,11,98,26,'#3b4854','#596776',4);text(W-104,29,density.title(),10,'white')
  rect(0,48,W,87);groups=['Build','Supports','Properties','Loads','Edit','Select and View','Review'];x=14
  for g in groups:
   width={'Select and View':108,'Properties':81,'Supports':72}.get(g,len(g)*7+22);button(x,55,width,g,g=='Build',28);x+=width+4
  rect(W-212,55,196,27,'white','#dce2e3',4);text(W-203,73,'⌕  Find any command…  /',11,'#8a9aa3')
  x=14
  for label,width in [('＋ Node',75),('Route pipe  01',114),('Components  15',125),('Expansion joint  14',143),('Split pipe / insert node  19',187),('Cold spring  ROADMAP',176)]:button(x,92,width,label,label.startswith('Route'),28);x+=width+6
  rect(0,top,left,ch);rect(left,top,cw,ch,'#f2f5f1');rect(W-right,top,right,ch)
  text(14,top+26,'Model',12,weight='600');text(left-30,top+26,'«',12);rect(12,top+54,left-24,30,'white','#cdd6da',4);text(21,top+73,'Name, ID or type',11,'#8a9aa3');text(14,top+107,'CW-101 · INVENTED EXAMPLE',9,'#7a8b95')
  y=top+133;text(14,y,'⌄ Nodes  8',11,'#6e818d','600');y+=20
  for id,coord in [('N1','0.0, 0.0, 0.0'),('N2','2.4, 0.0, 0.0'),('N3','2.4, 0.0, 1.8'),('N4','4.8, 0.0, 1.8'),('N5','4.8, 1.8, 1.8')]:
   if y>bottom-90:break
   if id=='N5':rect(12,y-16,left-24,25,'#eaf2fa','#bdd4e7',4)
   text(20,y,'○  '+id,11);text(left-96,y,coord,8,'#8a9aa3');y+=control-3
  if y<bottom-65:text(14,y+5,'⌄ Pipe runs  6',11,'#6e818d','600');text(22,y+29,'╱  P5        N5 → N6',11);text(22,y+52,'╱  PFREE    U1 → U2',11)
  rect(left,top,cw,42,'#fbfcfa');text(left+12,top+26,'Route pipe',12,weight='600');text(left+100,top+26,'Node N5',10,'#7c8c96');x=W-right-169
  for label,width in [('Iso',35),('Front',42),('Top',35),('Fit',33)]:button(x,top+9,width,label,label=='Iso',25);x+=width+3
  text(left+18,top+64,'● Draft plane XZ · typed input remains authoritative',9,'#648494')
  points={'N1':[0,0,0],'N2':[2.4,0,0],'N3':[2.4,0,1.8],'N4':[4.8,0,1.8],'N5':[4.8,1.8,1.8],'N6':[6.2,1.8,1.8],'U1':[.4,3.1,0],'U2':[2.4,3.1,0]}
  scale=min(cw/8.5,(ch-110)/5.3)
  def project(p):
   x,y,z=[p[i]-[3.1,1.3,.9][i] for i in range(3)];a=x*cos(-.65)-y*sin(-.65);b=x*sin(-.65)+y*cos(-.65);return [left+cw*.5+a*scale,top+ch*.55+(b*sin(.65)-z*cos(.65))*scale]
  for i in range(-1,8):
   a,b=project([i,-1,0]),project([i,4,0]);line(*a,*b,'#d9e1dc',.7)
   a,b=project([-1,i,0]),project([7,i,0]);line(*a,*b,'#d9e1dc',.7)
  for id,fr,to in [('P1','N1','N2'),('P2','N2','N3'),('P3','N3','N4'),('P4','N4','N5'),('P5','N5','N6'),('PFREE','U1','U2')]:
   a,b=project(points[fr]),project(points[to]);line(*a,*b,'#637f85',9);line(*a,*b,'#91a7aa',2);text((a[0]+b[0])/2+7,(a[1]+b[1])/2-9,id,10)
  for id,p in points.items():
   a=project(p);s.append(f'<circle cx="{a[0]}" cy="{a[1]}" r="{6 if id=="N5" else 3.5}" fill="{"#e2ac4f" if id=="N5" else "#eaf0ed"}" stroke="#728d94" stroke-width="1.5"/>');text(a[0]-8,a[1]+20,id,9)
  a,b=project(points['N5']),project([6.2,1.8,3]);line(*a,*b,'#2d84c4',3,'7 5');s.append(f'<circle cx="{b[0]}" cy="{b[1]}" r="7" fill="#d5e9f7" stroke="#2d84c4" stroke-width="2"/>');text(b[0]+10,b[1]-10,'Draft N7',10,'#2466a2')
  a=project(points['N1']);s.append(f'<path d="M{a[0]} {a[1]+7}l-10 17h20z" fill="#dce6df" stroke="#6f8584"/>')
  text(left+18,bottom-48,'━━ Existing    ┄┄ Draft    ● Selected',9,'#7c8c95');text(W-right-56,bottom-46,'Z ↑',10,'#5486b6');text(W-right-57,bottom-30,'Y ↙  X ↘',9,'#648494');rect(left,bottom-29,cw,29,'#fafbf9');text(left+12,bottom-11,'Isometric · camera changes are view state',9,'#788a95');text(W-right-112,bottom-11,'Readouts: Entered',9,'#788a95')
  rect(W-right,top,right,42);text(W-right+14,top+26,'Route / straight run',12,weight='600');text(W-53,top+26,'review',10,'#2466a2')
  x=W-right+14;y=top+61;available=right-28
  text(x,y,'EXPLICIT INVENTED DEMO INPUTS',9,'#7c8c96');y+=16
  def field(label,value,width=None):
   global y
   width=width or available;text(x,y,label,10,'#6e828f');rect(x,y+6,width,control,'white','#cdd6da',4);text(x+8,y+control/2+10,value,11);y+=control+15
  field('From · existing node','N5');field('Endpoint / pipe identity','New endpoint      /      P7');field('Node identity / coordinate unit','N7                        m');field('Pointer plane / constraint','XZ                        Free');field('Explicit XYZ · m','6.200    1.800    3.000')
  if y<bottom-125:field('Outside diameter / wall · mm','114.3                    6.02')
  if y<bottom-92:field('Material / local Y','M-DEMO         0, 1, 0')
  rect(W-right,bottom-79,right,79);text(x,bottom-61,'Frozen entered values; model unchanged.',9,'#6f828e');button(x,bottom-43,72,'Cancel');button(W-139,bottom-43,125,'Add → Review',True)
  rect(0,bottom,W,dock);rect(W/2-18,bottom+2,36,2,'#bbc9cf','#bbc9cf');x=14
  for label,width in [('Review  2',86),('Results  No result',140),('History',69),('State / intent',114)]:button(x,bottom+8,width,label,label.startswith('Review'),27);x+=width+6
  x=W-430
  for label,width in [('↶ Undo',69),('↷ Redo',69),('Continue route →',121),('Apply simulated draft',151)]:button(x,bottom+8,width,label,label.startswith('Apply'),27);x+=width+6
  y=bottom+61;text(18,y,'Frozen draft · review',12,weight='600');text(W-590,y,'Basis mock:r1 · 2 members',10,'#7b8e9a');y+=21
  for textval,xpos in [('Typed target',18),('Operation reference',195),('Before → proposed',445),('Unit',W-410)]:text(xpos,y,textval,9,'#82939e');y+=0
  line(18,y+7,W-360,y+7);y+=29
  text(18,y,'Node N7',11);text(195,y,'create_node',11);text(445,y,'not present → explicit XYZ',10);text(W-410,y,'m',10);y+=25
  text(18,y,'Element P7',11);text(195,y,'connect_pipe_run',11);text(445,y,'N5 → N7 · entered section',10);text(W-410,y,'mm',10);y+=25;text(18,y,'▸ Exact source-shaped mock payload · NOT schema/Rust validated',10,'#718592')
  rect(W-336,bottom+52,318,dock-67,'#f9fbf8','#dce2e3',5);text(W-323,bottom+75,'One atomic simulated change',12,weight='600');text(W-323,bottom+96,'Frozen members, entered values, mock:r1.',10);button(W-323,bottom+110,143,'Validate simulation',True,27);text(W-323,bottom+156,'No hash / Rust proof / numerical check.',9,'#82939e');text(W-323,bottom+174,'No professional approval or acceptance.',9,'#82939e')
  rect(0,H-28,W,28,'#e8edeb');text(14,H-10,'● Mock engine: ready',10,'#667b87');text(220,H-10,'Design simulation · no Rust operation or hash proof',10,'#667b87');text(W-113,H-10,'Scenario controls',9,'#667b87')
  svg=f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}"><title>Editable engineering workspace — {W}×{H} {density}</title><desc>Design simulation. Layout constants left {left}, right {right}, control {control}, dock {dock}. All text and geometry are editable vectors. Frozen route review with entered invented inputs.</desc><g font-family="-apple-system,BlinkMacSystemFont,Segoe UI,sans-serif">'+''.join(s)+'</g></svg>'
  (out/f'workspace-{W}x{H}-{density}.svg').write_text(svg+'\n')
