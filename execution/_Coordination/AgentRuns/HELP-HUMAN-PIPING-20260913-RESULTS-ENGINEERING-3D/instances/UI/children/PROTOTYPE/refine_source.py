from pathlib import Path
out=Path(__file__).resolve().parent
p=out/'prototype/workspace.js';s=p.read_text()
s=s.replace("x:String(((S.model.nodes.find(n=>n.id===r.end)?.p[0]||0)+1)/unitFactor[r.draft.unit]),placement:'Continuation after own matching apply'", "x:'',y:'',z:'',placement:'Continuation after own matching apply; enter next explicit XYZ'")
s=s.replace("new_pipe_ids:[`SPLITA${S.sequence}`,`SPLITB${S.sequence}`]", "new_pipe_ids:[d.pipe,`SPLITB${S.sequence}`]")
s=s.replace('with explicit new node and two pipe identities','with explicit new node, original pipe on first half and one new pipe identity')
s=s.replace("S.phase='busy';S.engine='busy';announce('Mock operation held busy.", "S.phase='busy';S.engine='busy';S.heldGeneration=S.generation;announce('Mock operation held busy.")
s=s.replace("S.phase='busy';S.engine='busy';}if(S.operationScenario", "S.phase='busy';S.engine='busy';S.heldGeneration=S.generation;}if(S.operationScenario")
s=s.replace("$('solve-scenario').onchange=", "$('complete-held').onclick=()=>{if(S.heldGeneration!==S.generation||!S.frozen){announce('LATE-MOCK-COMPLETION-IGNORED: withdrawn generation cannot restore a review.');return;}S.engine='ready';S.phase='review';S.operationScenario='ready';$('operation-scenario').value='ready';renderAll();announce('Held mock completion matched current generation; returned to Review, no model apply.');};$('solve-scenario').onchange=")
p.write_text(s)
p=out/'prototype/index.html';s=p.read_text().replace('<button id="reset"','<button id="complete-held">Complete held mock callback</button><button id="reset"');p.write_text(s)
# A source-only formatter that preserves nested template literals and regex tokens.
s=(out/'prototype/workspace.js').read_text()
def quoted(i):
 q=s[i];j=i+1
 while j<len(s):
  if s[j]=='\\':j+=2;continue
  if q=='`' and s[j:j+2]=='${':
   j=expression(j+2);continue
  if s[j]==q:return j+1
  j+=1
 return j

def expression(i):
 depth=1;j=i
 while j<len(s):
  if s[j] in ['"',"'",'`']:j=quoted(j);continue
  if s[j]=='{':depth+=1
  if s[j]=='}':
   depth-=1
   if depth==0:return j+1
  j+=1
 return j
parts=[];buf=[];indent=0;paren=0;i=0

def flush():
 global buf
 t=''.join(buf).strip()
 if t:parts.append('  '*max(indent,0)+t)
 buf=[]
while i<len(s):
 c=s[i]
 if s[i:i+2] in ['//','/*']:
  end=s.find('\n',i) if s[i:i+2]=='//' else s.find('*/',i)+2
  if end<0:end=len(s)
  flush();parts.append('  '*max(indent,0)+s[i:end].strip());i=end;continue
 if c in ['"',"'",'`']:
  j=quoted(i);buf.append(s[i:j]);i=j;continue
 if c=='/' and (s[i+1:i+2] in ['[','^']):
  j=i+1;charclass=False
  while j<len(s):
   if s[j]=='\\':j+=2;continue
   if s[j]=='[':charclass=True
   if s[j]==']':charclass=False
   if s[j]=='/' and not charclass:
    j+=1
    while j<len(s) and s[j].isalpha():j+=1
    break
   j+=1
  buf.append(s[i:j]);i=j;continue
 if c=='(':paren+=1;buf.append(c)
 elif c==')':paren-=1;buf.append(c)
 elif c=='{':buf.append(c);flush();indent+=1
 elif c=='}':flush();indent-=1;buf.append(c)
 elif c==';' and paren==0:buf.append(c);flush()
 elif c=='\n':
  if buf and ''.join(buf).strip()=='}':flush()
  else:buf.append(' ')
 else:
  if c not in ' \t' and buf and ''.join(buf).strip()=='}' and c not in ',);].':flush()
  buf.append(c)
 i+=1
flush();(out/'prototype/workspace.js').write_text('\n'.join(parts)+'\n')
