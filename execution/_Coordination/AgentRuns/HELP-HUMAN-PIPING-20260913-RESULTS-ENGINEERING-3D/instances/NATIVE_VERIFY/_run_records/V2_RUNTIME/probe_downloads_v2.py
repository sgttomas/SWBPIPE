from pathlib import Path
import argparse,json,stat,time
p=argparse.ArgumentParser();p.add_argument('--output',required=True);a=p.parse_args()
d=Path(__file__).parent;root=Path(json.loads((d/'DOWNLOADS_RESOLVER_V2.json').read_text())['root']);base='openpipestress-preview-results-run-preview-linear-static-001';records=[]
for suffix in ['']+[f' ({i})' for i in range(1,9)]:
 f=root/(base+suffix+'.json')
 try:
  s=f.lstat();q={'path':str(f),'exists':True,'regular':stat.S_ISREG(s.st_mode),'symlink':stat.S_ISLNK(s.st_mode),'size':s.st_size,'mtime_ns':s.st_mtime_ns,'ctime_ns':s.st_ctime_ns,'inode':s.st_ino}
 except FileNotFoundError:q={'path':str(f),'exists':False}
 records.append(q)
(d/a.output).write_text(json.dumps({'probe_unix':time.time(),'root':str(root),'exact_scoped_metadata_only':True,'unrelated_enumeration':False,'records':records},indent=2)+'\n');print(json.dumps(records))
