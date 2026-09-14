from pathlib import Path
import argparse,json,sqlite3,hashlib
p=argparse.ArgumentParser();p.add_argument('--output',required=True);a=p.parse_args()
src=Path('/Users/ryan/Library/Application Support/org.openpipestress.technical-preview.results-engineering-3d-20260913/openpipestress-projects.sqlite3');out=Path(a.output).resolve();assert 'instances/NATIVE_VERIFY/_run_records' in str(out);assert not out.exists()
sidecars=[x.name for x in src.parent.iterdir() if x.name!=src.name];assert not sidecars
b=src.read_bytes();out.write_bytes(b);assert src.read_bytes()==b
c=sqlite3.connect('file:'+str(out)+'?mode=ro&immutable=1',uri=True);c.row_factory=sqlite3.Row;assert c.execute('pragma integrity_check').fetchone()[0]=='ok'
r=dict(c.execute('select * from local_projects where project_id=?',('project:blank-local-20260914t051915z',)).fetchone());c.close();out.with_suffix('.json').write_text(json.dumps(r,indent=2)+'\n')
out.with_suffix('.capture.json').write_text(json.dumps({'source':str(src),'method':'quiescent completed Save exact standalone SQLite byte read; offline-copy immutable read','sidecars':sidecars,'double_read_equal':True,'source_sha256':hashlib.sha256(b).hexdigest(),'copy_integrity':'ok','project_id':r['project_id']},indent=2)+'\n')
