"""Pure source asset rewrite; does not evaluate JavaScript or run a test."""
from pathlib import Path
p=Path(__file__).resolve().parent/'prototype/workspace.js'
s=p.read_text()
s=s.replace("x:String((S.model.nodes.find(n=>n.id===r.end)?.p[0]||0)+1)","x:String(((S.model.nodes.find(n=>n.id===r.end)?.p[0]||0)+1)/unitFactor[r.draft.unit])")
s=s.replace("r=$('canvas').getBoundingClientRect(),sx=(e.clientX-r.left)*900/r.width,sy=(e.clientY-r.top)*560/r.height,axes=", "screenPoint=new DOMPoint(e.clientX,e.clientY).matrixTransform($('canvas').getScreenCTM().inverse()),sx=screenPoint.x,sy=screenPoint.y,axes=")
s=s.replace("(base[i]/unitFactor[d.unit]).toFixed(3)","String(Number((base[i]/unitFactor[d.unit]).toPrecision(12)))")
s=s.replace("d.placement=`Pointer ${d.plane} / ${d.axis}; entered ${d.unit}`", "d.placement=`Pointer ${d.plane} / ${d.axis}; 12 significant digits in ${d.unit}`")
# Keep strings/templates/comments byte-for-byte, expand statement and block boundaries.
parts=[];buf=[];indent=0;quote=None;escaped=False;comment=None;paren=0;i=0

def flush():
 global buf
 t=''.join(buf).strip()
 if t:parts.append('  '*max(0,indent)+t)
 buf=[]
while i<len(s):
 c=s[i];n=s[i+1] if i+1<len(s) else ''
 if comment=='line':
  buf.append(c)
  if c=='\n':flush();comment=None
 elif comment=='block':
  buf.append(c)
  if c=='*' and n=='/':buf.append(n);i+=1;comment=None;flush()
 elif quote:
  buf.append(c)
  if escaped:escaped=False
  elif c=='\\':escaped=True
  elif c==quote:quote=None
 elif c=='/' and n=='/':buf.extend([c,n]);i+=1;comment='line'
 elif c=='/' and n=='*':buf.extend([c,n]);i+=1;comment='block'
 elif c in ['"',"'",'`']:quote=c;buf.append(c)
 elif c=='(':paren+=1;buf.append(c)
 elif c==')':paren-=1;buf.append(c)
 elif c=='{':buf.append(c);flush();indent+=1
 elif c=='}':flush();indent-=1;buf.append(c)
 elif c==';' and paren==0:buf.append(c);flush()
 elif c=='\n':flush()
 else:buf.append(c)
 i+=1
flush();p.write_text('\n'.join(parts)+'\n')
