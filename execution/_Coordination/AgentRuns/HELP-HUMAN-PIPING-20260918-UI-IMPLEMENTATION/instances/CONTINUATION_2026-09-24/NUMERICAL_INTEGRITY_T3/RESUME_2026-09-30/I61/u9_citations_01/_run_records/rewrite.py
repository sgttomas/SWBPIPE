"""Plan (and with --apply, perform) the symbol rewording of bare code line citations, comment-only and line-neutral."""
import re,subprocess,os,json,sys
env=dict(os.environ,GIT_OPTIONAL_LOCKS="0")
def g(*a): return subprocess.run(["git",*a],capture_output=True,text=True,env=env).stdout
P="projects/chirality-piping/"; HEAD="e543c3d8f3"; READER_CODE="fc23cff95f"; apply="--apply" in sys.argv
S=[n for n in g("diff","--name-only","381be775ae",HEAD,"--",".",":!"+P+"execution",":!execution").split("\n") if n]
d=g("diff","-U0","381be775ae",HEAD,"--",*S)
NUMS=r"\d+(?:\s*[–-]\s*\d+)?(?:(?:,\s*|/)\d+(?:\s*[–-]\s*\d+)?(?![\w.]))*"
rx=re.compile(r"(?<![\w/])(?:(PP|FC|FK|SR)(?:/((?:[\w.]+/)*[\w.]+\.(?:rs|py|ts)))?|((?:[\w-]+/)*[\w-]+\.(?:rs|py|ts|tsx))):(" + NUMS + ")")
added={}  # (file, line) -> True
f=None;ln=0
for line in d.split("\n"):
    if line.startswith("+++ "): f=line[6:]; continue
    m=re.match(r"@@ -\S+ \+(\d+)(?:,\d+)? @@",line)
    if m: ln=int(m.group(1)); continue
    if line.startswith("+"):
        if rx.search(line[1:]): added[(f,ln)]=line[1:]
        ln+=1
trees={}
def files(c):
    if c not in trees: trees[c]=[x for x in g("ls-tree","-r","--name-only",c,"--",P+"core",P+"apps/desktop/src").split("\n") if x]
    return trees[c]
FKD=P+"core/solver/frame_kernel/src/structural/"
def cands(c,pre,sub,gen):
    fs=files(c)
    if pre=="PP" and not sub: return [P+"core/product_physics/src/retained_product.rs"]
    if pre=="FC" and not sub: return [x for x in fs if x.endswith("/final_case.rs")]
    base=(sub or gen)
    if pre=="PP": base=base.replace("core/product_physics/src/","")
    m=[x for x in fs if x.endswith("/"+base) or x==base]
    if pre=="PP": m=[x for x in m if "/product_physics/" in x]
    if pre=="FK": m=[x for x in m if "/frame_kernel/" in x]
    if len(m)>1:
        r=[x for x in m if x.startswith(FKD+"retained/")]; m=r or m
    if len(m)>1:
        r=[x for x in m if "/product_physics/src/" in x]; m=r or m
    return m
FN=re.compile(r"^(\s*)(?:pub(?:\([\w ]+\))?\s+)?(?:const\s+|async\s+|unsafe\s+)*(?:fn|const|static|struct|enum|trait|mod|def|class|function|export function|export const|const)\s+([A-Za-z_]\w*)")
IMPL=re.compile(r"^impl(?:<[^>]*>)?\s+(?:[\w:<>, ]+\s+for\s+)?([A-Za-z_]\w*)")
def symbol(L,n):
    for i in range(min(n,len(L))-1,-1,-1):
        m=FN.match(L[i])
        if m:
            name=m.group(2); ind=len(m.group(1))
            if ind>0:
                for j in range(i-1,-1,-1):
                    mi=IMPL.match(L[j])
                    if mi: return f"{mi.group(1)}::{name}"
                    if L[j] and not L[j][0].isspace() and not L[j].startswith(("}","#","//","///")): break
            return name
    return None
plan=[]; edits={}
for (f,ln) in sorted(added):
    if f.endswith(".json"): continue
    if f.endswith("retained_memory.rs") and 1051<=ln<=2300: continue
    text=added[(f,ln)]
    bl=g("blame","-L",f"{ln},{ln}","--porcelain",HEAD,"--",f).split("\n")[0].split(" ")[0]
    def sub_(m):
        pre,sub,gen,nums=m.group(1),m.group(2),m.group(3),m.group(4)
        reader=("/analysis_runs/" in f or "/result_export/" in f or "/features/results/" in f or f.startswith(P+"tests/"))
        rev=READER_CODE if reader else bl
        cs=cands(rev,pre,sub,gen)
        if not cs: rev=bl; cs=cands(rev,pre,sub,gen)
        if not cs: rev=HEAD; cs=cands(rev,pre,sub,gen)
        if len(cs)!=1: plan.append((f,ln,m.group(0),"UNRESOLVED file",cs)); return m.group(0)
        L=g("show",f"{rev}:{cs[0]}").split("\n")
        syms=[]
        for lo,hi in [(int(x),int(y or x)) for x,y in re.findall(r"(\d+)(?:\s*[–-]\s*(\d+))?",nums)]:
            lo_line=L[lo-1].strip() if lo<=len(L) else ""
            inner=[i for i in range(lo,min(hi,len(L))+1) if FN.match(L[i-1])]
            if not hi or hi==lo:   # single line: a closing brace, attribute, doc or blank looks forward
                if not lo_line or lo_line=="}" or lo_line.startswith(("#[","///","//")):
                    inner=[i for i in range(lo,min(lo+6,len(L))+1) if FN.match(L[i-1])][:1]
            names=[symbol(L,i) for i in inner]
            boundary=(not lo_line) or lo_line=="}" or lo_line.startswith(("#[","///","//")) or FN.match(L[lo-1])
            if not boundary or not names:
                names=[symbol(L,lo)]+names
            names=[n for n in names if n]
            if len(names)>2: names=[names[0], "@through@"+names[-1]]
            for s in names:
                if s not in syms: syms.append(s)
        if not syms: plan.append((f,ln,m.group(0),"UNRESOLVED symbol",cs)); return m.group(0)
        disp={"PP":"retained_product.rs","FC":"final_case.rs"}.get(pre) if not (sub or gen) else (sub or gen).split("/")[-1]
        cur=g("show",f"{HEAD}:{cs[0]}")
        missing=[s for s in syms if not re.search(r"\b(fn|const|static|struct|enum|def|function|class)\s+"+re.escape(s.replace("@through@","").split("::")[-1])+r"\b",cur)]
        parts=[]
        for s in syms:
            parts.append(("through `"+s[9:]+"`") if s.startswith("@through@") else f"`{s}`")
        rep=disp+" "+", ".join(parts).replace(", through"," through")
        plan.append((f,ln,m.group(0),rep,("MISSING in HEAD: "+",".join(missing)) if missing else rev[:10]))
        return rep
    new=rx.sub(sub_,text)
    if new!=text: edits[(f,ln)]=(text,new)
json.dump([list(x) for x in plan],open(sys.argv[1],"w"),indent=1)
json.dump({f"{k[0]}:{k[1]}":v for k,v in edits.items()},open(sys.argv[2],"w"),indent=1,ensure_ascii=False)
print("occurrences",len(plan),"lines",len(edits),"unresolved",sum(1 for p in plan if str(p[3]).startswith("UNRESOLVED")),"missing",sum(1 for p in plan if str(p[4]).startswith("MISSING")))
if apply:
    byf={}
    for (f,ln),(old,new) in edits.items(): byf.setdefault(f,[]).append((ln,old,new))
    for f,lst in byf.items():
        data=open(f,encoding="utf-8").read().split("\n")
        for ln,old,new in lst:
            assert data[ln-1]==old,(f,ln)
            data[ln-1]=new
        open(f,"w",encoding="utf-8").write("\n".join(data))
    print("applied to",len(byf),"files")
